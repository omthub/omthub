mod components;
mod helpers;
mod pages;

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

    <crate::components::PageWrapper>
      <Router>
        <Routes>
          <Route path="/" view=crate::pages::homepage::HomePage />
          <Route path="/all-translations" view=crate::pages::all_translations::AllTranslationsPage />
        </Routes>
      </Router>
    </crate::components::PageWrapper>
  }
}
