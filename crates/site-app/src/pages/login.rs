use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginParams {
  pub email:    String,
  pub password: String,
  pub remember: bool,
}

use std::fmt::Debug;

impl Debug for LoginParams {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("User")
      .field("email", &"[redacted]")
      .field("pw_hash", &"[redacted]")
      .field("remember", &self.remember)
      .finish()
  }
}

#[cfg_attr(feature = "ssr", tracing::instrument)]
#[server]
pub async fn login(params: LoginParams) -> Result<bool, ServerFnError> {
  let creds = auth::Credentials {
    email:    params.email,
    password: params.password,
    remember: params.remember,
  };
  let mut auth_session = use_context::<auth::AuthSession>()
    .ok_or_else(|| ServerFnError::new("Failed to get auth session"))?;
  let session = use_context::<tower_sessions::Session>()
    .ok_or_else(|| ServerFnError::new("Failed to get session"))?;

  let user = match auth_session.authenticate(creds.clone()).await {
    Ok(Some(user)) => user,
    Ok(None) => return Ok(false),
    Err(e) => {
      return Err(ServerFnError::new(format!("Failed to authenticate: {e:?}")))
    }
  };

  auth_session
    .login(&user)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to log in: {e}")))?;

  if creds.remember {
    session.set_expiry(Some(tower_sessions::Expiry::AtDateTime(
      time::OffsetDateTime::now_utc() + time::Duration::days(30),
    )));
  }

  tracing::info!("logged in user: {} ({})", user.name, user.id.0);
  Ok(true)
}
