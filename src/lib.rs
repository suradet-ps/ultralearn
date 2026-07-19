#![allow(missing_docs)]

pub mod app;
pub mod components;
pub mod composables;
pub mod core;
pub mod stores;
pub mod views;

pub use app::App;

use console_error_panic_hook::set_once as set_panic_hook;
use console_log::init_with_level;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use log::Level;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
pub fn start() {
  set_panic_hook();
  init_with_level(Level::Debug).ok();

  mount_to_body(|| {
    // Install store singletons INSIDE the mount closure so their RwSignals
    // are created in the mount-root Owner and are never disposed.
    crate::stores::plan::install();
    view! { <App /> }
  });

  register_service_worker();
}

/// Register the offline service worker (no-op if unsupported or on file://).
fn register_service_worker() {
  if let Some(window) = web_sys::window() {
    let container = window.navigator().service_worker();
    let _ = container.register("/sw.js");
  }
}
