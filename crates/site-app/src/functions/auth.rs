use std::fmt::Debug;

use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SignupParams {
  pub name:     String,
  pub email:    String,
  pub password: String,
  pub remember: bool,
}

impl Debug for SignupParams {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("User")
      .field("name", &self.name)
      .field("email", &"[redacted]")
      .field("pw_hash", &"[redacted]")
      .field("remember", &self.remember)
      .finish()
  }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginParams {
  pub email:    String,
  pub password: String,
  pub remember: bool,
}

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
pub async fn signup(params: SignupParams) -> Result<(), ServerFnError> {
  let SignupParams {
    name,
    email,
    password,
    remember,
  } = params;

  let auth_session = use_context::<auth::AuthSession>()
    .ok_or_else(|| ServerFnError::new("Failed to get auth session"))?;

  auth_session
    .backend
    .signup(name, email.clone(), password.clone())
    .await
    .map_err(|e| {
      logging::error!("Failed to sign up: {:?}", e);
      ServerFnError::new("Failed to sign up")
    })?;

  let login_result = login(LoginParams {
    email: email.clone(),
    password: password.clone(),
    remember,
  })
  .await
  .map_err(|e| ServerFnError::new(format!("Failed to log in: {e}")))?;

  if !login_result {
    return Err(ServerFnError::new("Failed to login after sign up"));
  }

  Ok(())
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
