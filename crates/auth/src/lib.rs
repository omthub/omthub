//! This crate implements [`axum_login`] for picturepro types, using a SurrealDB
//! backend.

use axum_login::{
  AuthManagerLayer, AuthManagerLayerBuilder, AuthnBackend, UserId,
};
use core_types::ssr::CoreId;
use eyre::{eyre, Context, OptionExt, Result};
use serde::{Deserialize, Serialize};
use tower_sessions::ExpiredDeletion;
use tracing::instrument;

/// The credentials type for the authentication layer.
///
/// This type will be transformed into an enum when we implement additional
/// authentication methods.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Credentials {
  /// The email address of the user.
  pub email:    String,
  /// The password of the user.
  pub password: String,
  /// The remember-me flag.
  pub remember: bool,
}

/// Takes in a password and produces a PHC string ($argon2id$v=19$...)
fn hash_password(password: &str) -> Result<String> {
  use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
  };

  let salt = SaltString::generate(&mut OsRng);
  let argon2 = Argon2::default();

  let password_hash = argon2
    .hash_password(password.as_bytes(), &salt)
    .map_err(|e| eyre::eyre!(e))
    .wrap_err("failed to hash password")?
    .to_string();

  Ok(password_hash)
}

fn verify_password(pw_hash: &str, password: &str) -> Result<bool> {
  use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
  };

  let parsed_hash = PasswordHash::new(pw_hash)
    .map_err(|e| eyre::eyre!(e))
    .wrap_err("failed to parse password hash")?;

  Ok(
    Argon2::default()
      .verify_password(password.as_bytes(), &parsed_hash)
      .is_ok(),
  )
}

/// The backend type for the authentication layer.
///
/// This type implements the [`AuthnBackend`] trait for the picturepro types,
/// and has a [`signup`](Backend::signup) method for creating new users.
#[derive(Clone, Debug)]
pub struct Backend {
  db: db::DbConnection,
}

impl Backend {
  /// Create a new backend instance.
  pub async fn new() -> eyre::Result<Self> {
    Ok(Self {
      db: db::DbConnection::new().await?,
    })
  }

  /// Create a new user.
  ///
  /// This method has checks to ensure that a user with the given email does
  /// not already exist.
  #[instrument(skip(password))]
  pub async fn signup(
    &self,
    name: String,
    email: String,
    password: String,
  ) -> Result<core_types::User> {
    let existing_users = self
      .db
      .select_all_users_matching_email(&email)
      .await
      .map_err(|e| eyre!("surrealdb error: {e}"))?;

    if !existing_users.is_empty() {
      return Err(eyre!("User with email {} already exists", email));
    }

    let user_to_create: core_types::User = core_types::User {
      id: core_types::UserRecordId::new(),
      name,
      email,
      pw_hash: hash_password(&password)?,
      meta: core_types::Meta::new(),
      is_active: true,
    };

    let user: Option<core_types::User> = self
      .db
      .insert_user(user_to_create)
      .await
      .map_err(|e| eyre!("surrealdb error: {e}"))?;

    user.ok_or_eyre("Failed to create user")
  }
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
  #[error("failed: {0}")]
  Surreal(db::SurrealError),
  #[error("duplicate users exist: {0:?}")]
  Duplicates(Vec<core_types::UserRecordId>),
}

#[async_trait::async_trait]
impl AuthnBackend for Backend {
  type User = core_types::User;
  type Credentials = Credentials;
  type Error = AuthError;

  #[instrument(skip(self, credentials))]
  async fn authenticate(
    &self,
    credentials: Self::Credentials,
  ) -> Result<Option<Self::User>, Self::Error> {
    let users = self
      .db
      .select_all_users_matching_email(&credentials.email)
      .await
      .map_err(AuthError::Surreal)?;
    tracing::info!("got {} users", users.len());

    let users = users
      .into_iter()
      .filter(|u| {
        verify_password(&u.pw_hash, &credentials.password).is_ok_and(|v| v)
      })
      .collect::<Vec<_>>();

    match users.len() {
      0 => Ok(None),
      1 => Ok(Some(users.first().unwrap().clone())),
      _ => Err(AuthError::Duplicates(
        users.into_iter().map(|u| u.id).collect(),
      )),
    }
  }

  #[instrument(skip(self))]
  async fn get_user(
    &self,
    user_id: &UserId<Self>,
  ) -> Result<Option<Self::User>, Self::Error> {
    Ok(
      self
        .db
        .select_user(core_types::UserRecordId(*user_id))
        .await
        .map_err(AuthError::Surreal)?,
    )
  }
}

/// The authentication session type.
///
/// This is an alias for the [`axum_login::AuthSession`] type with our backend
/// type. We can pull this type out of the axum router after we've added the
/// auth layer, and it's generally all we need to read at runtime for auth
/// state.
pub type AuthSession = axum_login::AuthSession<Backend>;

/// Builds an authentication layer for use with an Axum router.
pub async fn build_auth_layer() -> Result<
  AuthManagerLayer<
    Backend,
    tower_sessions::CachingSessionStore<
      tower_sessions::MemoryStore,
      tower_sessions_surrealdb_store::SurrealSessionStore<db::WsClient>,
    >,
  >,
> {
  let surreal_client = db::DbConnection::new().await?.into_inner().await?;

  tracing::debug!("connected to db with sqlx");
  let session_store = tower_sessions_surrealdb_store::SurrealSessionStore::new(
    surreal_client,
    "sessions".to_string(),
  );

  tokio::task::spawn(
    session_store
      .clone()
      .continuously_delete_expired(tokio::time::Duration::from_secs(300)),
  );

  let memory_store = tower_sessions::MemoryStore::default();
  let session_store =
    tower_sessions::CachingSessionStore::new(memory_store, session_store);

  // session_store
  //   .migrate()
  //   .await
  //   .wrap_err("failed to perform db migration for auth backend")?;

  let session_manager_layer =
    tower_sessions::SessionManagerLayer::new(session_store).with_expiry(
      tower_sessions::Expiry::OnInactivity(time::Duration::days(30)),
    );

  Ok(
    AuthManagerLayerBuilder::new(Backend::new().await?, session_manager_layer)
      .build(),
  )
}
