//! Small formatting helpers shared across views.

use leptos::ev::KeyboardEvent;
use wasm_bindgen::JsCast;

/// Slugify a string for use in filenames / ids. Mirrors
/// `topic.toLowerCase().replace(/\s+/g, '-')`.
pub fn slugify(input: &str) -> String {
  input
    .to_lowercase()
    .chars()
    .map(|c| if c.is_whitespace() { '-' } else { c })
    .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
    .collect()
}

/// True when a keyboard event originated from a text field or editable region,
/// so global shortcuts can stand down while the user is typing.
pub fn is_typing(ev: &KeyboardEvent) -> bool {
  let Some(target) = ev.target() else {
    return false;
  };
  let Ok(el) = target.dyn_into::<web_sys::HtmlElement>() else {
    return false;
  };
  let tag = el.tag_name().to_lowercase();
  if tag == "input" || tag == "textarea" {
    return true;
  }
  el.is_content_editable()
}
