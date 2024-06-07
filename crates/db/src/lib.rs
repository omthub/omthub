use std::sync::Arc;

use core_types::{MOTHER_TONGUE_TABLE, USER_TABLE};
use eyre::{Context, Result};
pub use surrealdb::{
  engine::remote::ws::Client as WsClient, Error as SurrealError,
  Result as SurrealResult,
};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

#[derive(Clone, Debug)]
pub struct DbConnection(Arc<Surreal<WsClient>>);

impl DbConnection {
  pub async fn new() -> Result<Self> {
    let client = Surreal::new::<Ws>(
      std::env::var("SURREAL_WS_URL")
        .wrap_err("could not find env var \"SURREAL_WS_URL\"")?,
    )
    .await
    .wrap_err_with(|| {
      format!(
        "Could not connect to SurrealDB endpoint: `{}`\n\tNB: don't include \
         the ws:// or wss:// prefix, e.g. `example.com:8080` instead of \
         `wss://example.com:8080`",
        std::env::var("SURREAL_WS_URL").unwrap()
      )
    })?;

    client
      .signin(Root {
        username: &std::env::var("SURREAL_USER")
          .wrap_err("could not find env var \"SURREAL_USER\"")?,
        password: &std::env::var("SURREAL_PASS")
          .wrap_err("could not find env var \"SURREAL_PASS\"")?,
      })
      .await
      .wrap_err("failed to sign in to SurrealDB as root")?;

    Ok(Self(Arc::new(client)))
  }

  pub async fn into_inner(self) -> SurrealResult<Surreal<WsClient>> {
    let client = self.use_main().await?;
    Ok(Arc::unwrap_or_clone(client.clone()))
  }

  async fn use_main(&self) -> SurrealResult<&Arc<Surreal<WsClient>>> {
    self.0.use_ns("main").use_db("main").await?;
    Ok(&self.0)
  }

  pub async fn select_user(
    &self,
    user_id: core_types::UserRecordId,
  ) -> SurrealResult<Option<core_types::User>> {
    self.use_main().await?.select(user_id).await
  }

  pub async fn select_all_users_matching_email(
    &self,
    email: &str,
  ) -> SurrealResult<Vec<core_types::User>> {
    self
      .use_main()
      .await?
      .query(format!("SELECT * FROM {USER_TABLE} WHERE email = $email"))
      .bind(("email", email))
      .await?
      .take(0)
  }

  pub async fn insert_user(
    &self,
    user: core_types::User,
  ) -> SurrealResult<Option<core_types::User>> {
    self.use_main().await?.insert(user.id).content(user).await
  }

  #[tracing::instrument(skip(self))]
  pub async fn select_mother_tongues(
    &self,
    offset: u32,
    count: u32,
  ) -> SurrealResult<Vec<core_types::MotherTongue>> {
    let query = format!(
      "SELECT * FROM {MOTHER_TONGUE_TABLE} LIMIT {count} START {offset}"
    );
    tracing::info!("query = {query:?}");
    self.use_main().await?.query(query).await?.take(0)
  }
}
