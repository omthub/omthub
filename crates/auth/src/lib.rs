//! This crate implements [`axum_login`] for picturepro types, using a SurrealDB
//! backend.

use axum_login::{
  AuthManagerLayer, AuthManagerLayerBuilder, AuthnBackend, UserId,
};
use clients::diesel::prelude::*;
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

/// The backend type for the authentication layer.
///
/// This type implements the [`AuthnBackend`] trait for the picturepro types,
/// and has a [`signup`](Backend::signup) method for creating new users.
#[derive(Clone, Debug)]
pub struct Backend {
  db: clients::DbConnection,
}

impl Backend {
  /// Create a new backend instance.
  pub async fn new() -> eyre::Result<Self> {
    Ok(Self {
      db: clients::DbConnection::new().await?,
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
    use core_types::schema::users;

    let conn = &mut self.db.get().wrap_err("failed to get db from pool")?;

    let user: Option<core_types::User> = users::table
      .filter(users::email.eq(email.clone()))
      .limit(1)
      .select(core_types::User::as_select())
      .load(conn)
      .wrap_err("failed to select users from db")?
      .first()
      .cloned();

    if user.is_some() {
      return Err(eyre!("User with email {} already exists", email));
    }

    let user_to_create: core_types::User = core_types::User {
      id: core_types::Ulid::new(),
      name,
      email,
      pw_hash: password,
      meta: core_types::Meta::new(),
      is_active: true,
    };

    let user: Option<core_types::User> = user_to_create
      .insert_into(users::table)
      .load(conn)
      .wrap_err("failed to insert user into db")?
      .first()
      .cloned();

    user.ok_or_eyre("Failed to create user")
  }
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
  #[error("failed: {0}")]
  Eyre(eyre::Report),
  #[error("duplicate users exist: {0:?}")]
  Duplicates(Vec<core_types::Ulid>),
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
    // CAUTION: we don't do any argon comparison yet, and that's dangerous

    use core_types::schema::users;

    let conn = &mut self
      .db
      .get()
      .wrap_err("failed to get db from pool")
      .map_err(AuthError::Eyre)?;

    let users: Vec<core_types::User> = users::table
      .filter(users::email.eq(credentials.email.clone()))
      .limit(1)
      .select(core_types::User::as_select())
      .load(conn)
      .wrap_err("failed to select users from db")
      .map_err(AuthError::Eyre)?;

    let users = users
      .into_iter()
      .filter(|u| u.pw_hash == credentials.password)
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
    use core_types::schema::users;

    let conn = &mut self
      .db
      .get()
      .wrap_err("failed to get db from pool")
      .map_err(AuthError::Eyre)?;

    Ok(
      users::table
        .filter(users::id.eq(user_id.to_string()))
        .limit(1)
        .select(core_types::User::as_select())
        .load(conn)
        .wrap_err("failed to select users from db")
        .map_err(AuthError::Eyre)?
        .first()
        .cloned(),
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
pub async fn build_auth_layer(
) -> Result<AuthManagerLayer<Backend, tower_sessions_sqlx_store::PostgresStore>>
{
  let pool =
    tower_sessions_sqlx_store::sqlx::PgPool::connect(&clients::db_url()?)
      .await
      .wrap_err("failed to connect to db")?;
  let session_store = tower_sessions_sqlx_store::PostgresStore::new(pool);

  tokio::task::spawn(
    session_store
      .clone()
      .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
  );

  let session_manager_layer =
    tower_sessions::SessionManagerLayer::new(session_store).with_expiry(
      tower_sessions::Expiry::OnInactivity(time::Duration::days(30)),
    );

  Ok(
    AuthManagerLayerBuilder::new(Backend::new().await?, session_manager_layer)
      .build(),
  )
}
