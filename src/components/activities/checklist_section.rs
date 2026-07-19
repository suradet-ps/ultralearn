use crate::components::common::{AddRow, RemoveButton};
use crate::components::icons::Check;
use crate::stores::use_plan;
use leptos::prelude::*;

#[component]
pub fn ChecklistSection(plan_id: String, principle_id: u32) -> impl IntoView {
  let store = use_plan();
  let plan_id = RwSignal::new(plan_id);
  let new_item = RwSignal::new(String::new());

  let progress = Signal::derive(move || {
    store.get_plan(&plan_id.get()).and_then(|p| {
      p.principles
        .into_iter()
        .find(|pp| pp.principle_id == principle_id)
    })
  });

  let add = Callback::new(move |()| {
    let text = new_item.get_untracked();
    if !text.trim().is_empty() {
      store.add_checklist_item(&plan_id.get(), principle_id, text.trim());
      new_item.set(String::new());
    }
  });

  view! {
      <Show when=move || progress.get().is_some() fallback=|| ()>
          <div class="checklist-section">
              <h3>"Checklist"</h3>
              <div class="checklist">
                  <For
                      each=move || {
                          progress
                              .get()
                              .map(|p| p.checklist.clone())
                              .unwrap_or_default()
                      }
                      key=|item| item.id.clone()
                      children=move |item| {
                          let item_id = item.id.clone();
                          let item_id2 = item.id.clone();
                          let checked = item.checked;
                          let text = item.text.clone();
                          let pid = plan_id;
                          view! {
                              <div class="checklist-item" class:checked=checked>
                                  <button
                                      class="check-btn"
                                      aria-label=if checked { "Uncheck" } else { "Check" }
                                      on:click=move |_| {
                                          store.toggle_checklist_item(&pid.get(), principle_id, &item_id);
                                      }
                                  >
                                      <Show when=move || checked fallback=|| ()>
                                          <Check size=14 />
                                      </Show>
                                  </button>
                                  <span class="checklist-text">{text}</span>
                                  <RemoveButton
                                      on_click=Callback::new(move |_| {
                                          store.remove_checklist_item(&pid.get(), principle_id, &item_id2);
                                      })
                                  />
                              </div>
                          }
                      }
                  />
              </div>
              <AddRow
                  value=Signal::derive(move || new_item.get())
                  on_input=Callback::new(move |v| new_item.set(v))
                  on_add=add
                  placeholder="Add a custom item..."
              />
          </div>
      </Show>
  }
}
