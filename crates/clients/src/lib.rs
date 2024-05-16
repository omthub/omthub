use std::ops::Deref;

pub use diesel;
use diesel::{
  r2d2::{ConnectionManager, Pool},
  PgConnection,
};
use eyre::{Context, Result};

pub fn db_url() -> Result<String> {
  std::env::var("DATABASE_URL")
    .wrap_err("could not find `DATABASE_URL` env var")
}

#[derive(Clone, Debug)]
pub struct DbConnection(Pool<ConnectionManager<PgConnection>>);

impl DbConnection {
  pub async fn new() -> Result<Self> {
    tracing::debug!("starting new connection to db with diesel...");
    let manager = ConnectionManager::<PgConnection>::new(&db_url()?);
    let pool = DbConnection(Pool::builder().build(manager)?);
    tracing::debug!("connected to db with diesel");

    Ok(pool)
  }
}

impl Deref for DbConnection {
  type Target = Pool<ConnectionManager<PgConnection>>;
  fn deref(&self) -> &Self::Target { &self.0 }
}
