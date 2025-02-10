#![allow(unused)]
use leptos::{
  logging::{error, log},
  prelude::*,
};

pub fn navigate_to(path: &str) {
  log!("navigating to: {}", path);
  let result = web_sys::window()
    .expect("Failed to get window")
    .location()
    .set_href(path);
  if let Err(e) = result {
    error!("failed to navigate: {:?}", e);
  }
}

pub fn reload() {
  let result = web_sys::window()
    .expect("Failed to get window")
    .location()
    .reload();
  if let Err(e) = result {
    error!("failed to reload: {:?}", e);
  }
}
