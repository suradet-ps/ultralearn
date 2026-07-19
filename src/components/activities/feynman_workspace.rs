use crate::components::icons::{AlertTriangle, BookOpen, PenLine};
use crate::stores::use_plan;
use leptos::prelude::*;

#[component]
pub fn FeynmanWorkspace(plan_id: String, principle_id: u32) -> impl IntoView {
  let store = use_plan();
  let plan_id = RwSignal::new(plan_id);
  let step = RwSignal::new(1u8);

  let concept = RwSignal::new(store.get_activity_text(
    &plan_id.get_untracked(),
    principle_id,
    "feynman_concept",
  ));
  let explanation = RwSignal::new(store.get_activity_text(
    &plan_id.get_untracked(),
    principle_id,
    "feynman_explanation",
  ));
  let gaps =
    RwSignal::new(store.get_activity_text(&plan_id.get_untracked(), principle_id, "feynman_gaps"));

  let set_concept = Callback::new(move |v: String| {
    store.set_activity_text(&plan_id.get(), principle_id, "feynman_concept", &v);
  });
  let set_explanation = Callback::new(move |v: String| {
    store.set_activity_text(&plan_id.get(), principle_id, "feynman_explanation", &v);
  });
  let set_gaps = Callback::new(move |v: String| {
    store.set_activity_text(&plan_id.get(), principle_id, "feynman_gaps", &v);
  });

  let step1 = Callback::new(move |_: web_sys::MouseEvent| step.set(1));
  let step2 = Callback::new(move |_: web_sys::MouseEvent| step.set(2));
  let step3 = Callback::new(move |_: web_sys::MouseEvent| step.set(3));

  view! {
      <div class="feynman">
          <h3>"Feynman Technique"</h3>
          <div class="steps">
              <button
                  class="step-btn"
                  class:active=move || step.get() == 1
                  on:click=move |ev| step1.run(ev)
              >
                  <PenLine size=16 />
                  "1. Explain"
              </button>
              <button
                  class="step-btn"
                  class:active=move || step.get() == 2
                  on:click=move |ev| step2.run(ev)
              >
                  <AlertTriangle size=16 />
                  "2. Find Gaps"
              </button>
              <button
                  class="step-btn"
                  class:active=move || step.get() == 3
                  on:click=move |ev| step3.run(ev)
              >
                  <BookOpen size=16 />
                  "3. Review"
              </button>
          </div>

          <Show when=move || step.get() == 1 fallback=|| ()>
              <div class="step-content">
                  <div class="form-group">
                      <label>"What concept do you want to understand?"</label>
                      <input
                          type="text"
                          placeholder="e.g. Ownership in Rust, Backpropagation..."
                          prop:value=move || concept.get()
                          on:input=move |ev| {
                              let v = event_target_value(&ev);
                              concept.set(v.clone());
                              set_concept.run(v);
                          }
                      />
                  </div>
                  <div class="form-group">
                      <label>"Explain it as if teaching a complete beginner"</label>
                      <textarea
                          rows=8
                          placeholder="Write your explanation here. Use simple language. If you get stuck, that's a gap you need to fill..."
                          prop:value=move || explanation.get()
                          on:input=move |ev| {
                              let v = event_target_value(&ev);
                              explanation.set(v.clone());
                              set_explanation.run(v);
                          }
                      ></textarea>
                  </div>
                  <button class="btn btn-primary" on:click=move |ev| step2.run(ev)>
                      "I've explained it — find my gaps"
                  </button>
              </div>
          </Show>

          <Show when=move || step.get() == 2 fallback=|| ()>
              <div class="step-content">
                  <div class="gap-prompt">
                      <AlertTriangle size=20 />
                      <p>
                          "Where did you get stuck? What parts were hard to explain? What did you have to look up? These are your understanding gaps."
                      </p>
                  </div>
                  <div class="form-group">
                      <label>"What gaps did you find?"</label>
                      <textarea
                          rows=6
                          placeholder="List the specific parts where your understanding broke down..."
                          prop:value=move || gaps.get()
                          on:input=move |ev| {
                              let v = event_target_value(&ev);
                              gaps.set(v.clone());
                              set_gaps.run(v);
                          }
                      ></textarea>
                  </div>
                  <button class="btn btn-primary" on:click=move |ev| step3.run(ev)>
                      "Review and fill gaps"
                  </button>
              </div>
          </Show>

          <Show when=move || step.get() == 3 fallback=|| ()>
              <div class="step-content">
                  <div class="review-summary">
                      <div class="review-block">
                          <h4>"Concept"</h4>
                          <p class="preserve-whitespace">
                              {move || { let v = concept.get(); if v.is_empty() { "(not filled)".to_string() } else { v } }}
                          </p>
                      </div>
                      <div class="review-block">
                          <h4>"Your Explanation"</h4>
                          <p class="preserve-whitespace">
                              {move || { let v = explanation.get(); if v.is_empty() { "(not filled)".to_string() } else { v } }}
                          </p>
                      </div>
                      <div class="review-block">
                          <h4>"Understanding Gaps"</h4>
                          <p class="preserve-whitespace">
                              {move || { let v = gaps.get(); if v.is_empty() { "(not filled)".to_string() } else { v } }}
                          </p>
                      </div>
                  </div>
                  <p class="review-hint">
                      "Go back to your resources, fill the gaps, then try explaining again from scratch."
                  </p>
                  <button class="btn btn-secondary" on:click=move |ev| step1.run(ev)>
                      "Start over"
                  </button>
              </div>
          </Show>
      </div>
  }
}
