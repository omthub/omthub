use leptos::*;

use crate::components::*;

#[island]
pub fn SignupPage() -> impl IntoView {
  let (name, set_name) = create_signal::<Option<String>>(None);
  let (email, set_email) = create_signal::<Option<String>>(None);
  let (password, set_password) = create_signal::<Option<String>>(None);
  let (confirm, set_confirm) = create_signal::<Option<String>>(None);

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
                class="input max-w-full"
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
                type="email" class="input max-w-full"
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
                  type="password" class="input max-w-full"
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
                  type="password" class="input max-w-full"
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
                <input type="checkbox" class="checkbox" />
                <a href="#">"Remember me"</a>
              </div>
            </div>

            <div class="form-field pt-5">
              <div class="form-control justify-between">
                <button type="button" class="btn btn-primary w-full">"Sign in"</button>
              </div>
            </div>

            <div class="form-field">
              <div class="form-control justify-center">
                <a href="/auth/signin" class="link link-underline link-primary text-sm">
                  "Already have an account? Sign in."
                </a>
              </div>
            </div>
          </div>

        </div>
      </div>
    </div>
  }
}
