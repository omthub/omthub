#![allow(unused)]

use leptos::*;

use crate::components::{logout::Logout, Link, LinkTarget};

#[component]
pub fn NavBar() -> impl IntoView {
  let user = crate::helpers::get_auth_context();

  view! {
    <div class="navbar navbar-sticky navbar-glass border-gray-6 border-b h-14 shadow-none">
      <div class="container mx-auto h-full flex flex-row">
        <div class="navbar-start">
          <Link target=LinkTarget::Home class="navbar-item font-semibold"><super::OmtHub/></Link>
          <Link target=LinkTarget::Home class="navbar-item">"Home"</Link>
          <Link target=LinkTarget::AllTranslations class="navbar-item">"All Translations"</Link>
        </div>
        <div class="navbar-end">
          { match user.0 {
            Some(user) => view! {
              <AccountDropdown user=user />
            }.into_view(),
            None => view! {
              <Link target=LinkTarget::Signup class="navbar-item">"Sign Up"</Link>
              <Link target=LinkTarget::Login class="navbar-item">"Log In"</Link>
            }.into_view(),
          }}
        </div>
      </div>
    </div>
    <div class="h-14"/>
  }
}

#[island]
pub fn AccountDropdown(user: core_types::PublicUser) -> impl IntoView {
  let logout_action = create_server_action::<Logout>();
  let logout_pending = logout_action.pending();
  let logout_value = logout_action.value();

  let show_loading =
    move || logout_pending() || matches!(logout_value(), Some(Ok(())));

  create_effect(move |_| {
    if matches!(logout_value(), Some(Ok(_))) {
      crate::helpers::navigation::reload();
    }
  });

  view! {
    <div class="dropdown">
      <label class="btn btn-rounded" tabindex="0">{ user.name }</label>
      <div class="dropdown-menu dropdown-menu-bottom-left border border-border">
        <Link target=LinkTarget::Account class="dropdown-item flex flex-row gap-2 items-center">
          <HeroIconsUserCircle />
          <p class="text-sm">"Account"</p>
        </Link>
        <button
          class="dropdown-item flex flex-row gap-2 items-center"
          on:click=move |_| { logout_action.dispatch(Logout {}); }
        >
          <HeroIconsArrowLeftStartOnRectangle />
          <p class="text-sm">"Log out"</p>
          <div class="flex-1" />
          { move || show_loading().then_some(view! {
            <div class="spinner-circle spinner-xs" style="--spinner-color: var(--content1);" />
          }) }
        </button>
      </div>
    </div>
  }
}

#[component]
pub fn HeroIconsUserCircle() -> impl IntoView {
  view! {
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
      <path stroke-linecap="round" stroke-linejoin="round" d="M17.982 18.725A7.488 7.488 0 0 0 12 15.75a7.488 7.488 0 0 0-5.982 2.975m11.963 0a9 9 0 1 0-11.963 0m11.963 0A8.966 8.966 0 0 1 12 21a8.966 8.966 0 0 1-5.982-2.275M15 9.75a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
    </svg>
  }
}

#[component]
pub fn HeroIconsArrowLeftStartOnRectangle() -> impl IntoView {
  view! {
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
      <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 9V5.25A2.25 2.25 0 0 1 10.5 3h6a2.25 2.25 0 0 1 2.25 2.25v13.5A2.25 2.25 0 0 1 16.5 21h-6a2.25 2.25 0 0 1-2.25-2.25V15m-3 0-3-3m0 0 3-3m-3 3H15" />
    </svg>
  }
}
