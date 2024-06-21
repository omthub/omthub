pub mod auth;
pub mod fetch;

#[cfg(feature = "ssr")]
use leptos::ServerFnError;

#[cfg(feature = "ssr")]
pub fn handle_error(
  error: eyre::Report,
  failed_action: &'static str,
) -> ServerFnError {
  tracing::error!("Failed to {failed_action}: {error:?}");
  ServerFnError::new(error)
}
