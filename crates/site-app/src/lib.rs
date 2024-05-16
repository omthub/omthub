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

#[allow(unused)]
fn get_auth_context() -> core_types::LoggedInUser {
  // flatten Option<LoggedInUser(Option<PublicUser>)> to LoggedInUser
  core_types::LoggedInUser(
    use_context::<core_types::LoggedInUser>().and_then(|s| s.0),
  )
}

pub fn NavBar() -> impl IntoView {
  view! {
    <div class="navbar navbar-sticky navbar-glass border-gray-6 border-b py-2 shadow-none">
      <div class="navbar-start">
        <a class="navbar-item">OMT-Hub</a>
      </div>
      <div class="navbar-end">
        <a class="navbar-item">Home</a>
      </div>
    </div>
  }
}

#[component]
pub fn PageWrapper() -> impl IntoView {
  view! {
    <div>
      <NavBar/>
      <div class="container mx-auto min-h-dvh">

      </div>
    </div>
  }
}

#[component]
#[component]
pub fn HomePage() -> impl IntoView {
  let _user = get_auth_context();

  view! {
    <p>"Hello, World!"</p>
    <p>{ format!("user: {user:#?}") }</p>
  }
}
