use leptos::*;

#[island]
pub fn LogoutButton(
  #[prop(optional)] class: String,
  children: Children,
) -> impl IntoView {
  let logout_action = create_server_action::<Logout>();
  let logout_value = logout_action.value();

  create_effect(move |_| {
    if matches!(logout_value(), Some(Ok(_))) {
      crate::helpers::navigation::reload();
    }
  });

  view! {
    <button class={class} on:click=move |_| {
      logout_action.dispatch(Logout {});
    }>{children()}</button>
  }
}

#[cfg_attr(feature = "ssr", tracing::instrument)]
#[server(Logout)]
pub async fn logout() -> Result<(), ServerFnError> {
  let mut auth_session = use_context::<auth::AuthSession>()
    .ok_or_else(|| ServerFnError::new("Failed to get auth session"))?;

  auth_session.logout().await.map_err(|e| {
    logging::error!("Failed to log out: {:?}", e);
    ServerFnError::new("Failed to log out")
  })?;

  Ok(())
}
