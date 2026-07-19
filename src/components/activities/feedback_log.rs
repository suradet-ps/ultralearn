use crate::components::icons::Plus;
use crate::core::time::format_date_short;
use crate::core::types::{FeedbackEntry, FeedbackType};
use crate::stores::use_plan;
use leptos::prelude::*;

#[component]
pub fn FeedbackLog(plan_id: String, principle_id: u32) -> impl IntoView {
  let store = use_plan();
  let plan_id = RwSignal::new(plan_id);
  let text = RwSignal::new(String::new());
  let fb_type = RwSignal::new(FeedbackType::Outcome);

  let types = [
    (FeedbackType::Outcome, "Outcome", "var(--color-primary)"),
    (
      FeedbackType::Informational,
      "Informational",
      "var(--color-warning)",
    ),
    (
      FeedbackType::Corrective,
      "Corrective",
      "var(--color-success)",
    ),
  ];

  let log = Signal::derive(move || store.get_feedback_log(&plan_id.get(), principle_id));

  let add = Callback::new(move |()| {
    let t = text.get_untracked();
    if !t.trim().is_empty() {
      store.add_feedback(
        &plan_id.get(),
        principle_id,
        fb_type.get_untracked(),
        t.trim(),
      );
      text.set(String::new());
    }
  });

  view! {
      <div class="feedback-section">
          <h3>"Feedback Log"</h3>
          <div class="add-form">
              <div class="type-select">
                  <For
                      each=move || types
                      key=|t| t.0
                      children=move |t| {
                          let (ty, label, color) = t;
                          view! {
                              <button
                                  class="type-btn"
                                  class:active=move || fb_type.get() == ty
                                  style=move || if fb_type.get() == ty {
                                      format!("background: {color}; color: white;")
                                  } else {
                                      String::new()
                                  }
                                  on:click=move |_| fb_type.set(ty)
                              >
                                  {label}
                              </button>
                          }
                      }
                  />
              </div>
              <div class="form-row">
                  <input
                      type="text"
                      placeholder="What feedback did you get?"
                      prop:value=move || text.get()
                      on:input=move |ev| text.set(event_target_value(&ev))
                      on:keydown=move |ev: web_sys::KeyboardEvent| {
                          if ev.key() == "Enter" { add.run(()); }
                      }
                  />
                  <button
                      class="btn btn-primary"
                      disabled=Signal::derive(move || text.get().trim().is_empty())
                      on:click=move |_| add.run(())
                  >
                      <Plus size=16 />
                  </button>
              </div>
          </div>

          <Show
              when=move || log.get().is_empty()
              fallback=move || {
                  view! {
                      <div class="log-list">
                          <For
                              each=move || log.get()
                              key=|e: &FeedbackEntry| e.id.clone()
                              children=move |e| {
                                  let color = match e.r#type {
                                      FeedbackType::Outcome => "var(--color-primary)",
                                      FeedbackType::Informational => "var(--color-warning)",
                                      FeedbackType::Corrective => "var(--color-success)",
                                  };
                                  let label = match e.r#type {
                                      FeedbackType::Outcome => "Outcome",
                                      FeedbackType::Informational => "Informational",
                                      FeedbackType::Corrective => "Corrective",
                                  };
                                  let text = e.text.clone();
                                  let date = format_date_short(&e.date);
                                  view! {
                                      <div class="log-entry">
                                          <div class="log-type" style=format!("background: {color};")>
                                              {label}
                                          </div>
                                          <div class="log-content">
                                              <p>{text}</p>
                                              <span class="log-date">{date}</span>
                                          </div>
                                      </div>
                                  }
                              }
                          />
                      </div>
                  }
              }
          >
              <div class="empty">"No feedback logged yet."</div>
          </Show>
      </div>
  }
}
