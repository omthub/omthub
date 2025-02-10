#![allow(non_snake_case)]

use leptos::{either::Either, prelude::*};

use crate::components::icons::HeroIconsCheck;

pub struct ActionStatus {
  loading: Box<dyn Fn() -> bool + Send>,
  success: Box<dyn Fn() -> bool + Send>,
}

impl ActionStatus {
  pub fn new<
    T: Send + Sync + 'static,
    O: Clone + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static,
  >(
    action: &Action<T, Result<O, E>>,
  ) -> ActionStatus {
    let value = action.value();
    ActionStatus {
      loading: Box::new(action.pending()),
      success: Box::new(move || matches!(value(), Some(Ok(_)))),
    }
  }
}

impl ActionStatus {
  pub fn view(self) -> impl IntoView {
    move || match ((self.loading)(), (self.success)()) {
      (true, true) => unimplemented!("should be impossible :)"),
      (true, false) => Some(Either::Left(ActionStatusLoading())),
      (false, true) => Some(Either::Right(ActionStatusSuccess())),
      (false, false) => None,
    }
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
