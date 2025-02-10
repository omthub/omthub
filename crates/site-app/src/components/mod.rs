pub mod action_status;
pub mod icons;
pub mod logout;
pub mod mini_pages;
pub mod mother_tongues_table;
pub mod navbar;
pub mod pagination;

use leptos::{prelude::*, *};
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

#[component]
pub fn Link(
  #[prop(into)] target: MaybeSignal<crate::LinkTarget>,
  #[prop(optional, into)] class: MaybeProp<String>,
  #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
  let target = match target {
    MaybeSignal::Static(v) => Signal::derive(move || v.clone()),
    MaybeSignal::Dynamic(s) => s,
  };

  let href = move || target().href();
  let target_attr = move || match target().new_tab() {
    true => "_blank",
    false => "",
  };
  let rel = move || match target().new_tab() {
    true => "noopener noreferrer",
    false => "",
  };
  let class = move || class.get().unwrap_or("".to_owned());

  view! {
    <a class=class href=href target=target_attr rel=rel>
      {children.map(|c| c())}
    </a>
  }
}

#[component]
pub fn BreadCrumbs(#[prop(into)] target: crate::LinkTarget) -> impl IntoView {
  let elements = target
    .full_chain()
    .into_iter()
    .map(|l| {
      view! {
        <li>
          <Link target={l.clone()}>{l.name()}</Link>
        </li>
      }
    })
    .collect_view();

  view! {
    <div class="breadcrumbs text-sm">
      <ul>
        { elements }
      </ul>
    </div>
  }
}
