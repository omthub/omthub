use leptos::*;

#[component]
pub fn NavBar() -> impl IntoView {
  let user = crate::helpers::get_auth_context();

  view! {
    <div class="navbar navbar-sticky navbar-glass border-gray-6 border-b h-14 shadow-none">
      <div class="container mx-auto h-full flex flex-row">
        <div class="navbar-start">
          <a href="/" class="navbar-item font-semibold"><OmtHub/></a>
          <a href="/" class="navbar-item">"Home"</a>
          <a href="/all-translations" class="navbar-item">"All Translations"</a>
        </div>
        <div class="navbar-end">
          { match user.0 {
            Some(user) => view! {
              <p class="navbar-item">{ user.name }</p>
            }.into_view(),
            None => view! {
              <a href="/auth/signup" class="navbar-item">"Sign Up"</a>
              <a href="/auth/login" class="navbar-item">"Log In"</a>
            }.into_view(),
          }}
        </div>
      </div>
    </div>
    <div class="h-14"/>
  }
}

#[component]
pub fn PageWrapper(children: Children) -> impl IntoView {
  view! {
    <div class="min-h-dvh flex flex-col">
      <NavBar/>
      <div class="container mx-auto flex-1 flex flex-col gap-8 py-8">
        { children() }
      </div>
    </div>
  }
}

#[component]
pub fn OmtHub() -> impl IntoView {
  view! {
    <span><span class="text-blue-11">"OMT"</span>"Hub"</span>
  }
}
