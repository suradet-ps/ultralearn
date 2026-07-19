use crate::core::sanitize::render_markdown;
use crate::stores::use_plan;
use leptos::prelude::*;

/// Render markdown source to a sanitized HTML string for display.
pub fn md_preview(source: &str) -> String {
  render_markdown(source)
}

#[component]
pub fn NotesSection(plan_id: String, principle_id: u32) -> impl IntoView {
  let store = use_plan();
  let plan_id = RwSignal::new(plan_id);
  let notes = RwSignal::new(store.get_principle_notes(&plan_id.get_untracked(), principle_id));
  let preview = RwSignal::new(false);

  let html = Signal::derive(move || md_preview(&notes.get()));

  view! {
      <div class="notes-section">
          <div class="section-header">
              <h3>"Notes"</h3>
              <div class="notes-toggle">
                  <button
                      class="mode-btn"
                      class:active=move || !preview.get()
                      on:click=move |_| preview.set(false)
                  >
                      "Write"
                  </button>
                  <button
                      class="mode-btn"
                      class:active=move || preview.get()
                      on:click=move |_| preview.set(true)
                  >
                      "Preview"
                  </button>
              </div>
          </div>

          <Show when=move || preview.get() fallback=move || {
              view! {
                  <textarea
                      placeholder="Write your notes in markdown — **bold**, *italic*, lists, `code`..."
                      prop:value=move || notes.get()
                      on:input=move |ev| {
                          let v = event_target_value(&ev);
                          notes.set(v.clone());
                          store.update_principle_notes(&plan_id.get(), principle_id, &v);
                      }
                  ></textarea>
              }
          }>
              <div class="notes-preview markdown-body" inner_html=move || html.get()></div>
          </Show>
      </div>
  }
}
