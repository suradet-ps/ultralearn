use crate::components::icons::{FlaskConical, Plus};
use crate::core::types::{Experiment, ExperimentStatus};
use crate::stores::use_plan;
use leptos::prelude::*;

fn next_status(s: ExperimentStatus) -> ExperimentStatus {
  match s {
    ExperimentStatus::Planned => ExperimentStatus::Running,
    ExperimentStatus::Running => ExperimentStatus::Completed,
    ExperimentStatus::Completed => ExperimentStatus::Planned,
  }
}

#[component]
pub fn ExperimentTracker(plan_id: String, principle_id: u32) -> impl IntoView {
  let store = use_plan();
  let plan_id = RwSignal::new(plan_id);
  let hypothesis = RwSignal::new(String::new());
  let method = RwSignal::new(String::new());
  let show_form = RwSignal::new(false);

  let experiments = Signal::derive(move || store.get_experiments(&plan_id.get(), principle_id));

  let add = Callback::new(move |_: web_sys::MouseEvent| {
    let h = hypothesis.get_untracked();
    let m = method.get_untracked();
    if !h.trim().is_empty() && !m.trim().is_empty() {
      store.add_experiment(&plan_id.get(), principle_id, h.trim(), m.trim());
      hypothesis.set(String::new());
      method.set(String::new());
      show_form.set(false);
    }
  });

  let cycle = Callback::new(move |exp: Experiment| {
    let next = next_status(exp.status);
    let id = exp.id.clone();
    store.update_experiment(
      &plan_id.get(),
      principle_id,
      &id,
      &Experiment {
        status: next,
        ..exp
      },
    );
  });

  view! {
      <div class="experiment-section">
          <div class="section-header">
              <h3>"Experiments"</h3>
              <button
                  class="btn btn-secondary"
                  on:click=move |_| show_form.set(!show_form.get())
              >
                  <Plus size=16 />
                  "New Experiment"
              </button>
          </div>

          <Show when=move || show_form.get() fallback=|| ()>
              <div class="add-form">
                  <div class="form-group">
                      <label>"Hypothesis — What do you want to try?"</label>
                      <input
                          type="text"
                          placeholder="e.g. Learning from video tutorials works better than reading docs"
                          prop:value=move || hypothesis.get()
                          on:input=move |ev| hypothesis.set(event_target_value(&ev))
                      />
                  </div>
                  <div class="form-group">
                      <label>"Method — How will you test it?"</label>
                      <textarea
                          rows=3
                          placeholder="e.g. Spend 1 week on video tutorials, then 1 week on docs. Compare retention and speed."
                          prop:value=move || method.get()
                          on:input=move |ev| method.set(event_target_value(&ev))
                      ></textarea>
                  </div>
                  <div class="form-actions">
                      <button
                          class="btn btn-primary"
                          disabled=Signal::derive(move || {
                              hypothesis.get().trim().is_empty() || method.get().trim().is_empty()
                          })
                          on:click=move |ev| add.run(ev)
                      >
                          "Add Experiment"
                      </button>
                      <button class="btn btn-ghost" on:click=move |_| show_form.set(false)>
                          "Cancel"
                      </button>
                  </div>
              </div>
          </Show>

          <Show
              when=move || experiments.get().is_empty()
              fallback=move || {
                  view! {
                      <div class="exp-list">
                          <For
                              each=move || experiments.get()
                              key=|e: &Experiment| e.id.clone()
                              children=move |e| {
                                  let color = match e.status {
                                      ExperimentStatus::Planned => "var(--color-mute)",
                                      ExperimentStatus::Running => "var(--color-warning)",
                                      ExperimentStatus::Completed => "var(--color-success)",
                                  };
                                  let status_label = match e.status {
                                      ExperimentStatus::Planned => "planned",
                                      ExperimentStatus::Running => "running",
                                      ExperimentStatus::Completed => "completed",
                                  };
                                  let hypothesis = e.hypothesis.clone();
                                  let method_text = e.method.clone();
                                  view! {
                                      <div class="exp-card">
                                          <div class="exp-header">
                                              <button
                                                  class="status-badge"
                                                  style=format!("background: {color};")
                                                  on:click=move |_| cycle.run(e.clone())
                                              >
                                                  {status_label}
                                              </button>
                                          </div>
                                          <div class="exp-hypothesis">{hypothesis}</div>
                                          <div class="exp-method">{method_text}</div>
                                      </div>
                                  }
                              }
                          />
                      </div>
                  }
              }
          >
              <div class="empty">
                  <FlaskConical size=32 />
                  <p>"No experiments yet. Try something new and track the results."</p>
              </div>
          </Show>
      </div>
  }
}
