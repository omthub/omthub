use leptos::*;
use leptos_meta::*;
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  view! {
    <Stylesheet id="leptos" href="/pkg/site.css"/>

    <Title text="A template app"/>
    <Html lang="en" />
    <Meta charset="utf-8"/>
    <Meta name="viewport" content="width=device-width, initial-scale=1"/>

    <Router>
      <Routes>
        <Route path="/" view=HomePage />
      </Routes>
    </Router>
  }
}

fn get_auth_context() -> core_types::LoggedInUser {
  // flatten Option<LoggedInUser(Option<PublicUser>)> to LoggedInUser
  core_types::LoggedInUser(
    use_context::<core_types::LoggedInUser>()
      .map(|s| s.0)
      .flatten(),
  )
}

#[component]
pub fn HomePage() -> impl IntoView {
  let user = get_auth_context();

  view! {
    <p>"Hello, World!"</p>
    <p>{ format!("user: {user:#?}") }</p>
  }
}
