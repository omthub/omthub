use leptos::*;

#[cfg_attr(feature = "ssr", tracing::instrument)]
#[server(Logout)]
pub async fn logout() -> Result<(), ServerFnError> {
  let mut auth_session = use_context::<auth::AuthSession>()
    .ok_or_else(|| ServerFnError::new("Failed to get auth session"))?;

  auth_session.logout().await.map_err(|e| {
    logging::error!("Failed to log out: {:?}", e);
    ServerFnError::new("Failed to log out")
  })?;

  Ok(())
}
