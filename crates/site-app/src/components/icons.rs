use leptos::*;

macro_rules! hero_icons_component_outline {
  ($name:ident, $path:expr) => {
    #[component]
    pub fn $name(
      #[prop(optional, into)] class: MaybeProp<String>,
    ) -> impl IntoView {
      let class = class.into_signal();
      view! {
        <svg
          xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
          stroke-width="1.5" stroke="currentColor"
          class={move || format!("size-6 {}", class().unwrap_or_default())}
        >
          <path stroke-linecap="round" stroke-linejoin="round" d=$path />
        </svg>
      }
    }
  };
}

hero_icons_component_outline!(
  HeroIconsUserCircle,
  "M17.982 18.725A7.488 7.488 0 0 0 12 15.75a7.488 7.488 0 0 0-5.982 \
   2.975m11.963 0a9 9 0 1 0-11.963 0m11.963 0A8.966 8.966 0 0 1 12 21a8.966 \
   8.966 0 0 1-5.982-2.275M15 9.75a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
);
hero_icons_component_outline!(
  HeroIconsArrowLeftStartOnRectangle,
  "M8.25 9V5.25A2.25 2.25 0 0 1 10.5 3h6a2.25 2.25 0 0 1 2.25 2.25v13.5A2.25 \
   2.25 0 0 1 16.5 21h-6a2.25 2.25 0 0 1-2.25-2.25V15m-3 0-3-3m0 0 3-3m-3 3H15"
);
hero_icons_component_outline!(HeroIconsCheck, "m4.5 12.75 6 6 9-13.5");
hero_icons_component_outline!(HeroIconsPlus, "M12 4.5v15m7.5-7.5h-15");
hero_icons_component_outline!(
  HeroIconsTrash,
  "m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 \
   1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 \
   2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 \
   0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 \
   3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 \
   0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 \
   0-7.5 0"
);
