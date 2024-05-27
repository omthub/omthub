use leptos::*;

#[server]
#[cfg_attr(feature = "ssr", tracing::instrument)]
pub async fn fetch_all_translations(
  offset: u32,
  count: u32,
) -> Result<Vec<core_types::MotherTongue>, ServerFnError> {
  let db: db::DbConnection = expect_context();
  let tongues = db.select_mother_tongues(offset, count).await.map_err(|e| {
    ServerFnError::new(format!("failed to fetch mother tongues: {e}"))
  })?;

  Ok(tongues)
}
