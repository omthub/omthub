mod components;
mod error_template;
mod functions;
mod helpers;
mod pages;

use leptos::*;
use leptos_meta::*;
use leptos_router::{Route, Router, Routes};

#[allow(dead_code)]
#[derive(Clone)]
pub enum LinkTarget {
  Home,
  Login,
  Signup,
  Account,
  MotherTongue(core_types::MotherTongueRecordId),
  AllTongues,
  External(String),
}

impl LinkTarget {
  pub fn href(&self) -> String {
    match self {
      LinkTarget::Home => "/".to_owned(),
      LinkTarget::Login => "/auth/login".to_owned(),
      LinkTarget::Signup => "/auth/signup".to_owned(),
      LinkTarget::Account => "/account".to_owned(),
      LinkTarget::MotherTongue(id) => format!("/tongue/{}", id.0),
      LinkTarget::AllTongues => "/all-tongues".to_owned(),
      LinkTarget::External(href) => href.to_owned(),
    }
  }

  pub fn full_chain(&self) -> Vec<Self> {
    match self {
      LinkTarget::Home => vec![LinkTarget::Home],
      LinkTarget::Login => vec![LinkTarget::Home, LinkTarget::Login],
      LinkTarget::Signup => vec![LinkTarget::Home, LinkTarget::Signup],
      LinkTarget::Account => vec![LinkTarget::Home, LinkTarget::Account],
      LinkTarget::MotherTongue(id) => vec![
        LinkTarget::Home,
        LinkTarget::AllTongues,
        LinkTarget::MotherTongue(*id),
      ],
      LinkTarget::AllTongues => vec![LinkTarget::Home, LinkTarget::AllTongues],
      LinkTarget::External(_) => {
        unimplemented!("cannot calculate link chain for eternal link")
      }
    }
  }

  pub fn new_tab(&self) -> bool { matches!(self, LinkTarget::External(_)) }
}

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
      <Router
        trailing_slash=leptos_router::TrailingSlash::Redirect
        fallback=|| {
          let mut outside_errors = Errors::default();
          outside_errors.insert_with_default_key(error_template::AppError::NotFound);
          view! { <error_template::ErrorTemplate outside_errors/> }.into_view()
        }
      >
        <Routes>
          <Route path="/" view=crate::pages::homepage::HomePage />
          <Route path="/all-tongues" view=crate::pages::all_tongues::AllTonguesPage />
          <Route path="/auth/signup" view=crate::pages::signup::SignupPage />
          <Route path="/auth/login" view=crate::pages::login::LoginPage />
          <Route path="/account" view=crate::pages::account::AccountPage />
          <Route path="/tongue/:id" view=crate::pages::mother_tongue::MotherTonguePage />
        </Routes>
      </Router>
    </crate::components::PageWrapper>
  }
}
