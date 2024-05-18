use leptos::*;
use serde::{Deserialize, Serialize};

use crate::components::*;

#[island]
pub fn SignupPage() -> impl IntoView {
  let (name, set_name) = create_signal::<Option<String>>(None);
  let (email, set_email) = create_signal::<Option<String>>(None);
  let (password, set_password) = create_signal::<Option<String>>(None);
  let (confirm, set_confirm) = create_signal::<Option<String>>(None);
  let (remember, set_remember) = create_signal(true);

  let email_validated = create_memo(move |_| match email() {
    None => None,
    Some(email) => crate::helpers::validate_email(email),
  });

  let confirm_validated = create_memo(move |_| match (password(), confirm()) {
    (Some(password), Some(confirm)) => {
      if password != confirm {
        return Some("Passwords must match.");
      } else {
        None
      }
    }
    _ => None,
  });

  let params = create_memo(move |_| match (name(), email(), password()) {
    (Some(name), Some(email), Some(password)) => {
      if email_validated().is_none() && confirm_validated().is_none() {
        Some(SignupParams {
          name,
          email,
          password,
          remember: remember(),
        })
      } else {
        None
      }
    }
    _ => None,
  });

  create_effect(move |_| {
    logging::log!("params = {:#?}", params());
  });

  let signup_action = create_server_action::<Signup>();
  let value = signup_action.value();
  let pending = signup_action.pending();

  let dispatch = move |_| {
    signup_action.dispatch(Signup {
      params: params().unwrap(),
    });
  };

  view! {
    <div class="flex-1 flex flex-col p-8 gap-4 justify-center items-center">
      <div class="card border border-border">
        <div class="card-body gap-4">

          <div class="card-header">
            <p>"Sign up for "<OmtHub/></p>
          </div>

          <div class="form-group gap-4">

            <div class="form-field">
              <label class="form-label">"Name"</label>
              <input
                placeholder="Type here"
                class="input hover:input-primary focus:input-primary transition max-w-full"
                on:input=move |ev| {
                  set_name(Some(event_target_value(&ev)));
                }
                prop:value=move || name().unwrap_or_default()
              />
            </div>

            <div class="form-field">
              <label class="form-label">"Email address"</label>

              <input
                placeholder="Type here"
                type="email" class="input hover:input-primary focus:input-primary transition max-w-full"
                on:input=move |ev| {
                  set_email(Some(event_target_value(&ev)));
                }
                prop:value=move || email().unwrap_or_default()
              />
              { move || email_validated().map(move |message| view! {
                <label class="form-label">
                  <span class="form-label-alt text-red-11">{message}</span>
                </label>
              }) }
            </div>

            <div class="form-field">
              <label class="form-label">"Password"</label>
              <div class="form-control">
                <input
                  placeholder="Type here"
                  type="password" class="input hover:input-primary focus:input-primary transition max-w-full"
                  on:input=move |ev| {
                    set_password(Some(event_target_value(&ev)));
                  }
                  prop:value=move || password().unwrap_or_default()
                />
              </div>
            </div>

            <div class="form-field">
              <label class="form-label">"Confirm password"</label>
              <div class="form-control">
                <input
                  placeholder="Type here"
                  type="password" class="input hover:input-primary focus:input-primary transition max-w-full"
                  on:input=move |ev| {
                    set_confirm(Some(event_target_value(&ev)));
                  }
                  prop:value=move || confirm().unwrap_or_default()
                />
              </div>
              { move || confirm_validated().map(move |message| view! {
                <label class="form-label">
                  <span class="form-label-alt text-red-11">{message}</span>
                </label>
              }) }
            </div>

            <div class="form-field">
              <div class="flex gap-2">
                <input
                  type="checkbox" class="checkbox"
                  on:input=move |ev| {
                    set_remember(event_target_checked(&ev))
                  }
                  prop:checked=remember
                />
                <a href="#">"Remember me"</a>
              </div>
            </div>

            <div class="form-field pt-5">
              <div class="form-control justify-between">
                <button
                  type="button" class="btn btn-primary w-full"
                  on:click=dispatch
                >"Sign Up"</button>
              </div>
            </div>

            <div class="form-field">
              <div class="form-control justify-center">
                <a href="/auth/login" class="link link-underline link-primary text-sm">
                  "Already have an account? Log in."
                </a>
              </div>
            </div>
          </div>

        </div>
      </div>
    </div>
  }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SignupParams {
  pub name:     String,
  pub email:    String,
  pub password: String,
  pub remember: bool,
}

use std::fmt::Debug;

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

  let _login_result =
    crate::pages::login::login(crate::pages::login::LoginParams {
      email: email.clone(),
      password: password.clone(),
      remember,
    })
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to log in: {e}")))?;

  Ok(())
}
