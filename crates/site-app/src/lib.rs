use leptos::*;
use leptos_meta::*;
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  view! {
    <Stylesheet id="leptos" href="/pkg/site.css"/>
    <Style>{include_str!("../style/fonts.css")}</Style>
    <Link rel="preload" href="/fonts/inter.ttf" as_="font" type_="font/ttf" crossorigin="anonymous" />

    <Title text="A template app"/>
    <Html lang="en" />
    <Meta charset="utf-8"/>
    <Meta name="viewport" content="width=device-width, initial-scale=1"/>

    <PageWrapper>
      <Router>
        <Routes>
          <Route path="/" view=HomePage />
          <Route path="/all-translations" view=AllTranslationsPage />
        </Routes>
      </Router>
    </PageWrapper>
  }
}

#[allow(unused)]
fn get_auth_context() -> core_types::LoggedInUser {
  // flatten Option<LoggedInUser(Option<PublicUser>)> to LoggedInUser
  core_types::LoggedInUser(
    use_context::<core_types::LoggedInUser>().and_then(|s| s.0),
  )
}

#[component]
pub fn NavBar() -> impl IntoView {
  view! {
    <div class="navbar navbar-sticky navbar-glass border-gray-6 border-b h-14 shadow-none font-semibold">
      <div class="container mx-auto h-full flex flex-row">
        <div class="navbar-start">
          <a class="navbar-item"><span class="text-primary">"OMT"</span>"Hub"</a>
        </div>
        <div class="navbar-end">
          <a href="/" class="navbar-item">"Home"</a>
          <a href="/all-translations" class="navbar-item">"All Translations"</a>
        </div>
      </div>
    </div>
    <div class="h-14"/>
  }
}

#[component]
pub fn PageWrapper(children: Children) -> impl IntoView {
  view! {
    <div class="flex flex-col min-h-dvh">
      <NavBar/>
      <div class="container mx-auto flex-1">
        { children() }
      </div>
    </div>
  }
}

#[component]
pub fn HomePage() -> impl IntoView {
  let _user = get_auth_context();

  view! {
    <div class="flex flex-col justify-center items-start gap-8 h-[36rem]">
      <p class="text-xl text-content2">"Welcome to "<span class="text-primary">"OMT"</span>"Hub."</p>
      <p class="text-7xl font-bold tracking-tight max-w-3xl">
        "Hear the Bible in your own "
        <span class="text-primary">"mother tongue"</span>
        "."
      </p>
      <a href="/all-translations" class="btn btn-lg btn-primary">
        "Get Started Now"
      </a>
    </div>
  }
}

#[component]
pub fn AllTranslationsPage() -> impl IntoView {
  view! {
    <div class="flex flex-col p-8 gap-4">
      <p class="text-5xl tracking-tight font-semibold">
        "All Translations"
      </p>
    </div>
  }
}
