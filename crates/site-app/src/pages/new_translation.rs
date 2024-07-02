use leptos::*;

use crate::{
  components::{icons::HeroIconsTrash, BreadCrumbs, Link},
  LinkTarget,
};

#[component]
pub fn NewTranslationPage() -> impl IntoView {
  view! {
    <BreadCrumbs target=LinkTarget::NewTranslation />
    <div class="flex flex-col p-8 gap-4">
      <div class="flex flex-row gap-4 items-center">
        <p class="text-5xl tracking-tight font-semibold">
          "New Translation"
        </p>
        <div class="flex-1" />
        <Link target=LinkTarget::Account
          class="btn btn-error flex flex-row gap-2 items-center"
        >
          <HeroIconsTrash />
          "Cancel"
        </Link>
      </div>
      <div class="h-[1px] border-gray-6 border-b mb-4"></div>
      <p class="text-content2 text-sm max-w-prose">
        "To create a translation, all you need to start with is the name of your new translation and the mother tongue that you're translating into."
      </p>
      <CreateTranslation />
    </div>
  }
}

#[island]
fn CreateTranslation() -> impl IntoView {
  view! {
    "Hello, World!"
  }
}
