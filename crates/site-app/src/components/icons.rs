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
