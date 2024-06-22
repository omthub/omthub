use leptos::*;

#[component]
pub fn BadLinkError() -> impl IntoView {
  view! {
    <p class="text-4xl tracking-tight semi-bold">"Oops! Something went wrong."</p>
    <p class="text-content2">
      "It looks like your link is broken."
    </p>
  }
}
