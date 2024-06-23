pub mod logout;
pub mod mini_pages;
pub mod mother_tongues_table;
pub mod navbar;
pub mod pagination;

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

#[derive(Clone)]
pub enum LinkTarget {
  Home,
  Login,
  Signup,
  Account,
  MotherTongue(core_types::MotherTongueRecordId),
  AllTranslations,
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
      LinkTarget::AllTranslations => "/all-translations".to_owned(),
      LinkTarget::External(href) => href.to_owned(),
    }
  }

  pub fn new_tab(&self) -> bool { matches!(self, LinkTarget::External(_)) }
}

#[component]
pub fn Link(
  #[prop(into)] target: MaybeSignal<LinkTarget>,
  #[prop(optional, into)] class: MaybeProp<String>,
  #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
  let target = target.into_signal();
  let href = move || with!(|target| target.href());
  let target_attr = move || {
    with!(|target| match target.new_tab() {
      true => "_blank",
      false => "",
    })
  };
  let rel = move || {
    with!(|target| match target.new_tab() {
      true => "noopener noreferrer",
      false => "",
    })
  };

  view! {
    <a class=class href=href target=target_attr rel=rel>
      {children.map(|c| c())}
    </a>
  }
}
