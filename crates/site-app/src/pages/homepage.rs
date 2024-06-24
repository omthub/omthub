use leptos::*;

use crate::{components::*, LinkTarget};

#[component]
pub fn HomePage() -> impl IntoView {
  let _user = crate::helpers::get_auth_context();

  view! {
    <div class="flex flex-col justify-center items-start gap-8 h-[36rem]">
      <p class="text-xl text-content2">"Welcome to "<OmtHub/>"."</p>
      <p class="text-7xl font-bold tracking-tight max-w-3xl">
        "Hear the Bible in your own "
        <span class="text-primary">"mother tongue"</span>
        "."
      </p>
      <Link target=LinkTarget::AllTongues class="btn btn-lg btn-primary">
        "Get Started Now"
      </Link>
    </div>
  }
}
