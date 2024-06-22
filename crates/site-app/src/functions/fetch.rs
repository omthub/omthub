#[cfg(feature = "ssr")]
use eyre::{Context, Result};
use leptos::*;

#[cfg(feature = "ssr")]
use crate::functions::handle_error;

#[cfg(feature = "ssr")]
#[tracing::instrument]
async fn use_db() -> Result<db::DbConnection> {
  Ok(if let Some(db) = use_context::<db::DbConnection>() {
    tracing::debug!("got db connection from context");
    db
  } else {
    tracing::warn!(
      "started new db connection because it was missing from server fn context"
    );
    db::DbConnection::new()
      .await
      .wrap_err("failed to start db client")?
  })
}

#[server]
#[cfg_attr(feature = "ssr", tracing::instrument)]
pub async fn fetch_mother_tongues(
  term: Option<String>,
  offset: u32,
  count: u32,
) -> Result<(Vec<core_types::MotherTongue>, usize), ServerFnError> {
  async move {
    let db = use_db().await?;
    let tongues = db
      .select_mother_tongues(term, offset, count)
      .await
      .wrap_err("failed to select mother tongues from db")?;

    Ok(tongues)
  }
  .await
  .map_err(|e| handle_error(e, "fetch mother tongues"))
}

#[server]
#[cfg_attr(feature = "ssr", tracing::instrument)]
pub async fn fetch_mother_tongue(
  id: core_types::MotherTongueRecordId,
) -> Result<Option<core_types::MotherTongue>, ServerFnError> {
  async move {
    let db = use_db().await?;
    let tongue = db
      .select_mother_tongue(id)
      .await
      .wrap_err("failed to select mother tongue from db")?;

    Ok(tongue)
  }
  .await
  .map_err(|e| handle_error(e, "fetch mother tongue"))
}
