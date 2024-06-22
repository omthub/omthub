use leptos::*;
use leptos_router::use_params_map;

use crate::{
  components::mini_pages::BadLinkError, functions::fetch::fetch_mother_tongue,
};

#[component]
pub fn MotherTonguePage() -> impl IntoView {
  let params = use_params_map();

  let id = move || {
    with!(|params| {
      params
        .get("id")
        .cloned()
        .map(|s| {
          s.parse::<core_types::Ulid>()
            .map(core_types::MotherTongueRecordId)
            .ok()
        })
        .flatten()
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
  let mother_tongue =
    create_resource(move || id, move |id| fetch_mother_tongue(id));

  view! {
    <Suspense fallback={move || view! { <p>"Loading..."</p> }}>
      { move || mother_tongue().map(|data| match data {
        Ok(tongue) => view! {
          <p>{ format!("tongue: {tongue:?}") }</p>
        }.into_view(),
        Err(e) => view! { <p>{ format!("failed to fetch mother tongue: {e}") }</p> }.into_view(),
      }) }
    </Suspense>
  }
}
