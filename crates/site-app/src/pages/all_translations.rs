use leptos::*;

#[component]
pub fn AllTranslationsPage() -> impl IntoView {
  view! {
    <div class="flex flex-col p-8 gap-4">
      <p class="text-5xl tracking-tight font-semibold">
        "All Translations"
      </p>
      <p class="text-3xl tracking-tight font-semibold">"Mother Tongues"</p>
      <crate::components::mother_tongues_table::MotherTonguesTable />
    </div>
  }
}
