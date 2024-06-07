use leptos::*;

use crate::functions::fetch::fetch_all_translations;

#[island]
pub fn AllTranslationsPage() -> impl IntoView {
  let (offset, _set_offset) = create_signal(0_u32);
  let tongues =
    create_resource(offset, move |offset| fetch_all_translations(offset, 25));

  view! {
    <div class="flex flex-col p-8 gap-4">
      <p class="text-5xl tracking-tight font-semibold">
        "All Translations"
      </p>
      <p class="text-3xl tracking-tight font-semibold">"Mother Tongues"</p>
      <Suspense fallback={move || view! { <p>"Loading..."</p> } }>
        {move || tongues().map(|data| match data {
          Ok(data) => view! {
            <For each=move || data.clone() key={|t| t.id} children={|t| view! {
              <p>{ format!("{t:?}") }</p>
            }} />
          },
          Err(_) => todo!(),
        })}
      </Suspense>
    </div>
  }
}
