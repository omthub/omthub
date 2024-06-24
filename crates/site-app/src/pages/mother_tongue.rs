use leptos::*;
use leptos_router::use_params_map;

use crate::{
  components::{
    mini_pages::{BadLinkError, MissingResourceError},
    BreadCrumbs,
  },
  functions::fetch::fetch_mother_tongue,
  LinkTarget,
};

#[component]
pub fn MotherTonguePage() -> impl IntoView {
  let params = use_params_map();

  let id = move || {
    with!(|params| {
      params.get("id").cloned().and_then(|s| {
        s.parse::<core_types::Ulid>()
          .map(core_types::MotherTongueRecordId)
          .ok()
      })
    })
  };

  view! {
    { move || match id() {
      Some(id) => view! { <MotherTongueFetcher id=id /> },
      None => view! { <BadLinkError /> },
    }}
  }
}

#[component]
fn MotherTongueFetcher(id: core_types::MotherTongueRecordId) -> impl IntoView {
  let mother_tongue = create_resource(move || id, fetch_mother_tongue);

  view! {
    <BreadCrumbs target=LinkTarget::MotherTongue(id) />
    <Suspense fallback={move || view! { <p>"Loading..."</p> }}>
      { move || mother_tongue().map(|data| match data {
        Ok(Some(data)) => view! { <MotherTongueData data=data /> }.into_view(),
        Ok(None) => view! { <MissingResourceError /> }.into_view(),
        Err(e) => view! { <p>{ format!("failed to fetch mother tongue: {e}") }</p> }.into_view(),
      }) }
    </Suspense>
  }
}

#[component]
fn MotherTongueData(data: core_types::MotherTongue) -> impl IntoView {
  view! {
    <p>{ format!("{:?}", data) }</p>
  }
}
