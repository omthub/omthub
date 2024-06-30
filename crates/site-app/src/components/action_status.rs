#![allow(non_snake_case)]

use leptos::*;

use crate::components::icons::HeroIconsCheck;

pub struct ActionStatus {
  loading: Box<dyn Fn() -> bool>,
  success: Box<dyn Fn() -> bool>,
}

impl ActionStatus {
  pub fn new<T, O: Clone, E: Clone>(
    action: &Action<T, Result<O, E>>,
  ) -> ActionStatus {
    let value = action.value();
    ActionStatus {
      loading: Box::new(action.pending()),
      success: Box::new(move || matches!(value(), Some(Ok(_)))),
    }
  }
}

impl IntoView for ActionStatus {
  fn into_view(self) -> View {
    (move || match ((self.loading)(), (self.success)()) {
      (true, true) => unimplemented!("should be impossible :)"),
      (true, false) => Some(ActionStatusLoading().into_view()),
      (false, true) => Some(ActionStatusSuccess().into_view()),
      (false, false) => None,
    })
    .into_view()
  }
}

fn ActionStatusLoading() -> impl IntoView {
  view! {
    <div
      class="spinner-circle spinner-xs animate-quick-fade-in"
      style="--spinner-color: var(--content1);"
    />
  }
}

fn ActionStatusSuccess() -> impl IntoView {
  view! {
    <div class="animate-quick-fade-in">
      <HeroIconsCheck />
    </div>
  }
}
