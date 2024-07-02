use leptos::*;
use leptos_router::Redirect;

use crate::{
  components::{icons::HeroIconsPlus, BreadCrumbs, Link},
  helpers::get_auth_context,
  LinkTarget,
};

#[component]
pub fn AccountPage() -> impl IntoView {
  let auth_context = get_auth_context();

  let Some(user) = auth_context.0 else {
    return view! { <Redirect path="/" /> };
  };

  view! {
    <BreadCrumbs target=LinkTarget::Account />
    <div class="flex flex-col p-8 gap-4">
      <div class="flex flex-row gap-4 items-center">
        <p class="text-5xl tracking-tight font-semibold">
          "My Account"
        </p>
        <div class="flex-1" />
        <Link target=LinkTarget::NewTranslation
          class="btn btn-primary flex flex-row gap-2 items-center"
        >
          <HeroIconsPlus />
          "New Translation"
        </Link>
      </div>
      <div class="h-[1px] border-gray-6 border-b"></div>
      <p>{ format!("{:?}", user) }</p>
    </div>
  }
  .into_view()
}
