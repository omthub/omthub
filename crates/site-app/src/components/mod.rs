use leptos::*;

#[component]
pub fn NavBar() -> impl IntoView {
  view! {
    <div class="navbar navbar-sticky navbar-glass border-gray-6 border-b h-14 shadow-none font-semibold">
      <div class="container mx-auto h-full flex flex-row">
        <div class="navbar-start">
          <a class="navbar-item"><OmtHub/></a>
        </div>
        <div class="navbar-end">
          <a href="/" class="navbar-item">"Home"</a>
          <a href="/all-translations" class="navbar-item">"All Translations"</a>
          <a href="/auth/signup" class="navbar-item">"Sign Up"</a>
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
