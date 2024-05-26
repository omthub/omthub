use leptos::*;

#[server]
#[cfg_attr(feature = "ssr", tracing::instrument)]
pub async fn fetch_all_translations(
  offset: u32,
  count: u32,
) -> Result<(), ServerFnError> {
  Ok(())
}
