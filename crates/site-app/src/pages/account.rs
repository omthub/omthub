use leptos::*;
use leptos_router::Redirect;

use crate::{components::BreadCrumbs, helpers::get_auth_context, LinkTarget};

#[component]
pub fn AccountPage() -> impl IntoView {
  let auth_context = get_auth_context();

  let Some(user) = auth_context.0 else {
    return view! { <Redirect path="/" /> };
  };

  view! {
    <BreadCrumbs target=LinkTarget::Account />
    <div class="flex flex-col p-8 gap-4">
      <p class="text-5xl tracking-tight font-semibold">
        "User Account"
      </p>
      <p>{ format!("{:?}", user) }</p>
    </div>
  }
  .into_view()
}
