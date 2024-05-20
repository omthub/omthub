mod logout;
mod navbar;

use leptos::*;
pub use navbar::*;

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
