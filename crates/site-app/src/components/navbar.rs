#![allow(unused)]

use leptos::*;

use crate::{
  components::{
    action_status::ActionStatus,
    icons::{
      HeroIconsArrowLeftStartOnRectangle, HeroIconsCheck, HeroIconsUserCircle,
    },
    logout::Logout,
    Link,
  },
  LinkTarget,
};

#[component]
pub fn NavBar() -> impl IntoView {
  let user = crate::helpers::get_auth_context();

  view! {
    <div class="navbar navbar-sticky navbar-glass border-gray-6 border-b h-14 shadow-none">
      <div class="container mx-auto h-full flex flex-row">
        <div class="navbar-start">
          <Link target=LinkTarget::Home class="navbar-item font-semibold"><super::OmtHub/></Link>
          <Link target=LinkTarget::Home class="navbar-item">{ LinkTarget::Home.name() }</Link>
          <Link target=LinkTarget::AllTongues class="navbar-item">{ LinkTarget::AllTongues.name() }</Link>
        </div>
        <div class="navbar-end">
          { match user.0 {
            Some(user) => view! {
              <AccountDropdown user=user />
            }.into_view(),
            None => view! {
              <Link target=LinkTarget::Signup class="navbar-item">{ LinkTarget::Signup.name() }</Link>
              <Link target=LinkTarget::Login class="navbar-item">{ LinkTarget::Login.name() }</Link>
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

  let logout_status = ActionStatus::new(&logout_action);

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
          <p class="text-sm">{ LinkTarget::Account.name() }</p>
        </Link>
        <button
          class="dropdown-item flex flex-row gap-2 items-center"
          on:click=move |_| { logout_action.dispatch(Logout {}); }
        >
          <HeroIconsArrowLeftStartOnRectangle />
          <p class="text-sm">"Log out"</p>
          <div class="flex-1" />
          { logout_status }
        </button>
      </div>
    </div>
  }
}
