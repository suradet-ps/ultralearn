use crate::components::common::RemoveButton;
use crate::components::icons::{ChevronLeft, ChevronRight, Plus, RotateCcw};
use crate::stores::use_plan;
use leptos::prelude::*;

#[component]
pub fn FlashcardSection(plan_id: String, principle_id: u32) -> impl IntoView {
  let store = use_plan();
  let plan_id = RwSignal::new(plan_id);
  let front = RwSignal::new(String::new());
  let back = RwSignal::new(String::new());
  let show_form = RwSignal::new(false);
  let current = RwSignal::new(0usize);
  let flipped = RwSignal::new(false);

  let cards = Signal::derive(move || store.get_flashcards(&plan_id.get(), principle_id));
  let count = move || cards.get().len();

  let add = Callback::new(move |()| {
    let f = front.get_untracked();
    let b = back.get_untracked();
    if !f.trim().is_empty() && !b.trim().is_empty() {
      store.add_flashcard(&plan_id.get(), principle_id, f.trim(), b.trim());
      front.set(String::new());
      back.set(String::new());
      show_form.set(false);
    }
  });

  let remove = Callback::new(move |card_id: String| {
    store.remove_flashcard(&plan_id.get(), principle_id, &card_id);
    let len = cards.get_untracked().len();
    if current.get_untracked() >= len && len > 0 {
      current.set(len - 1);
    } else if len == 0 {
      current.set(0);
    }
  });

  let next = Callback::new(move |_: web_sys::MouseEvent| {
    flipped.set(false);
    let len = count();
    if len > 0 {
      current.set((current.get_untracked() + 1) % len);
    }
  });
  let prev = Callback::new(move |_: web_sys::MouseEvent| {
    flipped.set(false);
    let len = count();
    if len > 0 {
      current.set((current.get_untracked() + len - 1) % len);
    }
  });
  let flip = Callback::new(move |_: web_sys::MouseEvent| flipped.set(!flipped.get_untracked()));

  view! {
      <div class="flashcard-section">
          <div class="section-header">
              <h3>"Flashcards"</h3>
              <button
                  class="btn btn-secondary"
                  on:click=move |_| show_form.set(!show_form.get())
              >
                  <Plus size=16 />
                  "Add Card"
              </button>
          </div>

          <Show when=move || show_form.get() fallback=|| ()>
              <div class="add-form">
                  <input
                      type="text"
                      placeholder="Front (question)"
                      prop:value=move || front.get()
                      on:input=move |ev| front.set(event_target_value(&ev))
                  />
                  <input
                      type="text"
                      placeholder="Back (answer)"
                      prop:value=move || back.get()
                      on:input=move |ev| back.set(event_target_value(&ev))
                      on:keydown=move |ev: web_sys::KeyboardEvent| {
                          if ev.key() == "Enter" { add.run(()); }
                      }
                  />
                  <div class="form-actions">
                      <button
                          class="btn btn-primary"
                          disabled=Signal::derive(move || {
                              front.get().trim().is_empty() || back.get().trim().is_empty()
                          })
                          on:click=move |_| add.run(())
                      >
                          "Add"
                      </button>
                      <button class="btn btn-ghost" on:click=move |_| show_form.set(false)>
                          "Cancel"
                      </button>
                  </div>
              </div>
          </Show>

          <Show
              when=move || count() == 0
              fallback=move || {
                  let card = Signal::derive(move || cards.get().into_iter().nth(current.get()));
                  view! {
                      <div class="card-view">
                          <div class="card-counter">
                              {move || format!("{} / {}", current.get() + 1, count())}
                          </div>
                          <div
                              class="flashcard"
                              class:flipped=move || flipped.get()
                              on:click=move |ev| flip.run(ev)
                          >
                              <div class="flashcard-face front">
                                  <div class="face-label">"Question"</div>
                                  <div class="face-content">
                                      {move || card.get().map(|c| c.front).unwrap_or_default()}
                                  </div>
                                  <div class="face-hint">"Click to flip"</div>
                              </div>
                              <div class="flashcard-face back">
                                  <div class="face-label">"Answer"</div>
                                  <div class="face-content">
                                      {move || card.get().map(|c| c.back).unwrap_or_default()}
                                  </div>
                                  <div class="face-hint">"Click to flip"</div>
                              </div>
                          </div>
                          <div class="card-controls">
                              <button
                                  class="btn btn-ghost"
                                  disabled=Signal::derive(move || count() <= 1)
                                  on:click=move |ev| prev.run(ev)
                              >
                                  <ChevronLeft size=18 />
                              </button>
                              <button class="btn btn-ghost" on:click=move |ev| flip.run(ev)>
                                  <RotateCcw size=18 />
                              </button>
                              <button
                                  class="btn btn-ghost"
                                  disabled=Signal::derive(move || count() <= 1)
                                  on:click=move |ev| next.run(ev)
                              >
                                  <ChevronRight size=18 />
                              </button>
                          </div>
                          <div class="card-list">
                              <For
                                  each=move || cards.get()
                                  key=|c| c.id.clone()
                                  children=move |c| {
                                      let id = c.id.clone();
                                      let front_text = c.front.clone();
                                      view! {
                                          <div class="card-list-item">
                                              <span class="card-list-front">{front_text}</span>
                                              <RemoveButton
                                                  on_click=Callback::new(move |_| {
                                                      remove.run(id.clone());
                                                  })
                                              />
                                          </div>
                                      }
                                  }
                              />
                          </div>
                      </div>
                  }
              }
          >
              <div class="empty">"No flashcards yet. Add some to practise retrieval."</div>
          </Show>
      </div>
  }
}
