use leptos::*;

use crate::helpers::get_auth_context;

#[component]
pub fn AccountPage() -> impl IntoView {
  let auth_context = get_auth_context();

  view! {
    <div class="flex flex-col p-8 gap-4">
      <p class="text-5xl tracking-tight font-semibold">
        "User Account"
      </p>
      <p>{ format!("{:?}", auth_context.0) }</p>
    </div>
  }
}
