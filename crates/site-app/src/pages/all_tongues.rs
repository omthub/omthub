use leptos::*;

use crate::{components::BreadCrumbs, LinkTarget};

#[component]
pub fn AllTonguesPage() -> impl IntoView {
  view! {
    <BreadCrumbs target=LinkTarget::AllTongues />
    <div class="flex flex-col p-8 gap-4">
      <p class="text-5xl tracking-tight font-semibold">
        "All Mother Tongues"
      </p>
      <p class="text-3xl tracking-tight font-semibold">"Mother Tongues"</p>
      <crate::components::mother_tongues_table::MotherTonguesTable />
    </div>
  }
}
