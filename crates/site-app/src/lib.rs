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

    <Title text="OMTHub"/>
    <Html lang="en" />
    <Meta charset="utf-8"/>
    <Meta name="viewport" content="width=device-width, initial-scale=1"/>
    <Meta name="description" content="A hub for Oral Mother Tongue translations of the bible"/>

    <crate::components::PageWrapper>
      <Router trailing_slash=leptos_router::TrailingSlash::Redirect>
        <Routes>
          <Route path="/" view=crate::pages::homepage::HomePage />
          <Route path="/all-translations" view=crate::pages::all_translations::AllTranslationsPage />
          <Route path="/auth/signup" view=crate::pages::signup::SignupPage />
          <Route path="/auth/login" view=crate::pages::login::LoginPage />
          <Route path="/account" view=crate::pages::account::AccountPage />
        </Routes>
      </Router>
    </crate::components::PageWrapper>
  }
}
