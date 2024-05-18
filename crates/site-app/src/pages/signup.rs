use leptos::*;

use crate::components::*;

#[island]
pub fn SignupPage() -> impl IntoView {
  let (name, set_name) = create_signal(String::new());
  let (email, set_email) = create_signal(String::new());
  let (password, set_password) = create_signal(String::new());
  let (confirm, set_confirm) = create_signal(String::new());

  create_effect(move |_| {
    logging::log!("name: {}", name());
  });

  view! {
    <div class="flex-1 flex flex-col p-8 gap-4 justify-center items-center">
      <div class="card card-border">
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
                  set_name(event_target_value(&ev));
                }
                prop:value=name
              />
            </div>

            <div class="form-field">
              <label class="form-label">"Email address"</label>

              <input
                placeholder="Type here"
                type="email" class="input max-w-full"
                on:input=move |ev| {
                  set_email(event_target_value(&ev));
                }
                prop:value=email
              />
              <label class="form-label">
                <span class="form-label-alt">"Please enter a valid email."</span>
              </label>
            </div>

            <div class="form-field">
              <label class="form-label">"Password"</label>
              <div class="form-control">
                <input
                  placeholder="Type here"
                  type="password" class="input max-w-full"
                  on:input=move |ev| {
                    set_password(event_target_value(&ev));
                  }
                  prop:value=password
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
                    set_confirm(event_target_value(&ev));
                  }
                  prop:value=confirm
                />
              </div>
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
