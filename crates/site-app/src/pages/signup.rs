use leptos::*;

use crate::{
  components::*,
  functions::auth::{Signup, SignupParams},
  helpers::navigation::navigate_to,
  LinkTarget,
};

#[derive(Clone, PartialEq)]
pub enum DispatchState {
  InsufficientInformation,
  Unsubmitted,
  Pending,
  Success,
  InternalError,
}

#[island]
pub fn SignupPage() -> impl IntoView {
  let (name, set_name) = create_signal::<Option<String>>(None);
  let (email, set_email) = create_signal::<Option<String>>(None);
  let (password, set_password) = create_signal::<Option<String>>(None);
  let (confirm, set_confirm) = create_signal::<Option<String>>(None);
  let (remember, set_remember) = create_signal(true);

  let name_validated = create_memo(move |_| match name() {
    None => None,
    Some(name) => crate::helpers::validate_name(name),
  });

  let email_validated = create_memo(move |_| match email() {
    None => None,
    Some(email) => crate::helpers::validate_email(email),
  });

  let password_validated = create_memo(move |_| match password() {
    None => None,
    Some(password) => crate::helpers::validate_password(password),
  });

  let confirm_validated = create_memo(move |_| {
    match (password(), confirm()) {
      (Some(password), Some(confirm)) => password != confirm,
      (Some(_), None) => true,
      (None, Some(_)) => true,
      (None, None) => false,
    }
    .then_some("Passwords must match.")
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

  let dispatch = move |_| match params() {
    Some(params) => signup_action.dispatch(Signup { params }),
    None => {
      if name().is_none() {
        set_name(Some(String::new()))
      }
      if email().is_none() {
        set_email(Some(String::new()))
      }
      if password().is_none() {
        set_password(Some(String::new()))
      }
      if confirm().is_none() {
        set_confirm(Some(String::new()))
      }
    }
  };

  let dispatch_state =
    create_memo(move |_| match (params(), pending(), value()) {
      (None, _, _) => DispatchState::InsufficientInformation,
      (Some(_), true, _) => DispatchState::Pending,
      (Some(_), false, None) => DispatchState::Unsubmitted,
      (Some(_), false, Some(Ok(()))) => DispatchState::Success,
      (Some(_), false, Some(Err(_))) => DispatchState::InternalError,
    });

  // redirect effect
  create_effect(move |_| {
    if dispatch_state() == DispatchState::Success {
      navigate_to("/account");
    }
  });

  let dispatch_button_styles = move || {
    format!("btn w-full transition {}", match dispatch_state() {
      DispatchState::InsufficientInformation => "btn-outline",
      DispatchState::Unsubmitted => "btn-primary",
      DispatchState::Pending => "btn-outline btn-loading",
      DispatchState::Success => "btn-outline",
      DispatchState::InternalError => "btn-outline",
    })
  };
  let dispatch_button_disabled =
    move || matches!(dispatch_state(), DispatchState::Pending);

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
              { move || name_validated().map(move |message| view! {
                <label class="form-label animate-slide-down">
                  <span class="form-label-alt text-red-11">{message}</span>
                </label>
              }) }
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
                <label class="form-label animate-slide-down">
                  <span class="form-label-alt text-red-11">{message}</span>
                </label>
              }) }
            </div>

            <div class="form-field">
              <label class="form-label">"Password"</label>
              <input
                placeholder="Type here"
                type="password" class="input hover:input-primary focus:input-primary transition max-w-full"
                on:input=move |ev| {
                  set_password(Some(event_target_value(&ev)));
                }
                prop:value=move || password().unwrap_or_default()
              />
              { move || password_validated().map(move |message| view! {
                <label class="form-label animate-slide-down">
                  <span class="form-label-alt text-red-11">{message}</span>
                </label>
              }) }
            </div>

            <div class="form-field">
              <label class="form-label">"Confirm password"</label>
              <input
                placeholder="Type here"
                type="password" class="input hover:input-primary focus:input-primary transition max-w-full"
                on:input=move |ev| {
                  set_confirm(Some(event_target_value(&ev)));
                }
                prop:value=move || confirm().unwrap_or_default()
              />
              { move || confirm_validated().map(move |message| view! {
                <label class="form-label animate-slide-down">
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
                <span>"Remember me"</span>
              </div>
            </div>

            <div class="form-field pt-5">
              <div class="form-control justify-between">
                <button
                  type="button" on:click=dispatch
                  class=dispatch_button_styles
                  disabled=dispatch_button_disabled
                >"Sign Up"</button>
              </div>
              { move || {
                match dispatch_state() {
                  DispatchState::Success => Some(view! {
                    <label class="form-label animate-slide-down">
                      <span class="form-label-alt text-green-11">
                        "Logged in successfully! Redirecting..."
                      </span>
                    </label>
                  }),
                  DispatchState::InternalError => Some(view! {
                    <label class="form-label animate-slide-down">
                      <span class="form-label-alt text-red-11">
                        "Something went wrong. Please try again."
                      </span>
                    </label>
                  }),
                  _ => None
                }
              }}
            </div>

            <div class="form-field">
              <div class="form-control justify-center">
                <Link target=LinkTarget::Login class="link link-underline link-primary text-sm">
                  "Already have an account? Log in."
                </Link>
              </div>
            </div>
          </div>

        </div>
      </div>
    </div>
  }
}
