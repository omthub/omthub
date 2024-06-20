use std::sync::Arc;

use core_types::{MOTHER_TONGUE_TABLE, USER_TABLE};
use eyre::{Context, Result};
use include_dir::{include_dir, Dir};
use serde::Deserialize;
pub use surrealdb::{
  engine::remote::ws::Client as WsClient, Error as SurrealError,
  Result as SurrealResult,
};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

const MIGRATIONS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/migrations");

#[derive(Deserialize)]
pub struct Count {
  pub count: usize,
}

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
    term: Option<String>,
    offset: u32,
    count: u32,
  ) -> SurrealResult<(Vec<core_types::MotherTongue>, usize)> {
    let mut query = if let Some(term) = term {
      let where_clause = "WHERE name @0@ $term || description @1@ $term";

      let count_query = format!(
        "SELECT count() FROM {MOTHER_TONGUE_TABLE} {where_clause} GROUP all"
      );
      let content_query = format!(
        "SELECT *, search::score(0) * 2 + search::score(1) * 1 AS relevance \
         FROM {MOTHER_TONGUE_TABLE} {where_clause} ORDER BY relevance DESC \
         LIMIT {count} START {offset}"
      );

      self
        .use_main()
        .await?
        .query(count_query)
        .query(content_query)
        .bind(("term", term.to_lowercase()))
        .await?
    } else {
      let count_query =
        format!("SELECT count() FROM {MOTHER_TONGUE_TABLE} GROUP all");
      let content_query = format!(
        "SELECT * FROM {MOTHER_TONGUE_TABLE} LIMIT {count} START {offset}"
      );

      self
        .use_main()
        .await?
        .query(count_query)
        .query(content_query)
        .await?
    };

    let count: Option<Count> = query.take(0)?;
    let content: Vec<core_types::MotherTongue> = query.take(1)?;
    // we can reasonably always expect surreal to return this bc of the GROUP
    let count = count.map(|c| c.count).unwrap_or(0);

    Ok((content, count))
  }

  #[tracing::instrument(skip(self))]
  pub async fn run_migrations(&self) -> Result<()> {
    let db = self.use_main().await?;

    surrealdb_migrations::MigrationRunner::new(&db)
      .load_files(&MIGRATIONS_DIR)
      .up()
      .await?;

    Ok(())
  }
}
