use leptos::*;

#[component]
pub fn BadLinkError() -> impl IntoView {
  view! {
    <p class="text-4xl tracking-tight font-semibold">"Oops! Something went wrong."</p>
    <p class="text-content2">
      "It looks like your link is broken."
    </p>
  }
}

#[component]
pub fn MissingResourceError() -> impl IntoView {
  view! {
    <p class="text-4xl tracking-tight font-semibold">"Oops! Something went wrong."</p>
    <p class="text-content2">
      "It looks like that resource doesn't exist."
    </p>
  }
}
