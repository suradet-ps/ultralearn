use crate::components::icons::{ArrowRight, Search, Sparkles};
use crate::core::types::principles_data::principle_name;
use crate::stores::use_plan;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn LandingView() -> impl IntoView {
  let topic = RwSignal::new(String::new());
  let goal = RwSignal::new(String::new());
  let navigate = StoredValue::new(use_navigate());
  let store = use_plan();

  let search = RwSignal::new(String::new());
  let active_tag = RwSignal::new(String::new());

  let examples = [
    "Rust Programming",
    "Machine Learning",
    "Guitar",
    "Japanese Language",
    "Data Structures & Algorithms",
    "Digital Painting",
  ];

  let create = Callback::new(move |()| {
    let t = topic.get_untracked();
    if t.trim().is_empty() {
      return;
    }
    let plan = store.create_plan(t.trim(), goal.get_untracked().trim());
    let id = plan.id.clone();
    navigate.with_value(|n| {
      n(
        &format!("/plan/{id}"),
        leptos_router::NavigateOptions::default(),
      );
    });
  });

  // All tags across plans, for the filter chips.
  let all_tags = Signal::derive(move || {
    let mut tags: Vec<String> = store
      .plans
      .get()
      .iter()
      .flat_map(|p| p.tags.iter().cloned())
      .collect();
    tags.sort();
    tags.dedup();
    tags
  });

  // Plans filtered by the search box and the active tag chip.
  let filtered_plans = Signal::derive(move || {
    let q = search.get().trim().to_lowercase();
    let tag = active_tag.get();
    store
      .plans
      .get()
      .into_iter()
      .filter(|p| {
        let matches_q = q.is_empty()
          || p.topic.to_lowercase().contains(&q)
          || p.goal.to_lowercase().contains(&q)
          || p.tags.iter().any(|t| t.to_lowercase().contains(&q));
        let matches_tag = tag.is_empty() || p.tags.contains(&tag);
        matches_q && matches_tag
      })
      .collect::<Vec<_>>()
  });

  view! {
      <div class="landing">
          <section class="hero">
              <div class="hero-inner">
                  <div class="hero-badge">
                      <Sparkles size=14 />
                      <span>"Based on Scott Young's Ultralearning"</span>
                  </div>
                  <h1 class="hero-title">"What do you want to learn?"</h1>
                  <p class="hero-subtitle">
                      "Enter a topic and build a structured learning plan using the 9 Ultralearning principles."
                  </p>

                  <form
                      class="hero-form"
                      on:submit=move |ev| {
                          ev.prevent_default();
                          create.run(());
                      }
                  >
                      <div class="form-group">
                          <label for="topic" class="form-label">"Topic"</label>
                          <input
                              id="topic"
                              type="text"
                              placeholder="e.g. Rust Programming, Guitar, Machine Learning..."
                              required
                              prop:value=move || topic.get()
                              on:input=move |ev| topic.set(event_target_value(&ev))
                          />
                      </div>

                      <div class="form-group">
                          <label for="goal" class="form-label">"Goal (optional)"</label>
                          <textarea
                              id="goal"
                              rows=3
                              placeholder="What do you want to achieve? e.g. Build a CLI tool in Rust, Play 3 songs on guitar..."
                              prop:value=move || goal.get()
                              on:input=move |ev| goal.set(event_target_value(&ev))
                          ></textarea>
                      </div>

                      <button
                          type="submit"
                          class="btn btn-primary btn-lg"
                          disabled=Signal::derive(move || topic.get().trim().is_empty())
                      >
                          "Generate Learning Plan"
                          <ArrowRight size=18 />
                      </button>
                  </form>

                  <div class="examples">
                      <span class="examples-label">"Try:"</span>
                      <For
                          each=move || examples
                          key=|e| *e
                          children=move |ex| {
                              view! {
                                  <button class="example-chip" on:click=move |_| topic.set(ex.to_string())>
                                      {ex}
                                  </button>
                              }
                          }
                      />
                  </div>
              </div>
          </section>

          <section class="plan-library">
              <div class="container">
                  <h2 class="section-title">"Your Plans"</h2>
                  <p class="section-subtitle">
                      "Pick up where you left off, or start something new."
                  </p>

                  <div class="plan-library-controls">
                      <div class="search-box">
                          <Search size=18 />
                          <input
                              type="text"
                              placeholder="Search plans by topic, goal, or tag..."
                              prop:value=move || search.get()
                              on:input=move |ev| search.set(event_target_value(&ev))
                              aria-label="Search plans"
                          />
                      </div>
                  </div>

                  <Show
                      when=move || !all_tags.get().is_empty()
                      fallback=|| ()
                  >
                      <div class="tag-filters">
                          <button
                              class="tag-chip"
                              class:active=move || active_tag.get().is_empty()
                              on:click=move |_| active_tag.set(String::new())
                          >
                              "All"
                          </button>
                          <For
                              each=move || all_tags.get()
                              key=|t| t.clone()
                              children=move |t| {
                                  let t2 = t.clone();
                                  view! {
                                      <button
                                          class="tag-chip"
                                          class:active=move || active_tag.get() == t
                                          on:click=move |_| {
                                              let cur = active_tag.get_untracked();
                                              if cur == t2 {
                                                  active_tag.set(String::new());
                                              } else {
                                                  active_tag.set(t2.clone());
                                              }
                                          }
                                                          >
                                                              {t.clone()}
                                                          </button>
                                  }
                              }
                          />
                      </div>
                  </Show>

                  <Show
                      when=move || !store.plans.get().is_empty()
                      fallback=|| view! {
                          <div class="empty">
                              "No plans yet. Create your first one above."
                          </div>
                      }
                  >
                      <div class="plan-list">
                          <For
                              each=move || filtered_plans.get()
                              key=|p| p.id.clone()
                              children=move |p| {
                                  let id = p.id.clone();
                                  let topic = p.topic.clone();
                                  let progress = store.get_progress(&p.id);
                                  let tags_for_show = p.tags.clone();
                                  let tags_for_for = p.tags.clone();
                                  let navigate_inner = navigate;
                                  view! {
                                      <button
                                          class="plan-list-item"
                                          on:click=move |_| {
                                              navigate_inner.with_value(|n| {
                                                  n(
                                                      &format!("/plan/{id}"),
                                                      leptos_router::NavigateOptions::default(),
                                                  );
                                              });
                                          }
                                      >
                                          <div class="plan-list-main">
                                              <div class="plan-list-topic">{topic}</div>
                                              <Show when=move || !tags_for_show.is_empty() fallback=|| ()>
                                                  <div class="plan-list-tags">
                                                      {{
                                                          let tf = tags_for_for.clone();
                                                          view! {
                                                              <For
                                                                  each=move || tf.clone()
                                                                  key=|t| t.clone()
                                                                  children=move |t| {
                                                                      view! { <span class="tag-chip static">{t}</span> }
                                                                  }
                                                              />
                                                          }
                                                      }}
                                                  </div>
                                              </Show>
                                          </div>
                                          <div class="plan-list-progress">
                                              <div class="progress-bar">
                                                  <div
                                                      class="progress-bar-fill"
                                                      style=move || format!("width: {progress}%")
                                                  ></div>
                                              </div>
                                              <span class="progress-text">{progress}"%"</span>
                                          </div>
                                      </button>
                                  }
                              }
                          />
                      </div>
                      <Show
                          when=move || !store.plans.get().is_empty()
                              && filtered_plans.get().is_empty()
                          fallback=|| ()
                      >
                          <div class="empty">"No plans match your search."</div>
                      </Show>
                  </Show>
              </div>
          </section>

          <section class="principles-preview">
              <div class="container">
                  <h2 class="section-title">"The 9 Ultralearning Principles"</h2>
                  <p class="section-subtitle">
                      "Each plan is structured around these principles from Scott Young's framework."
                  </p>
                  <div class="principles-grid">
                      <For
                          each=move || 1u32..=9
                          key=|i| *i
                          children=move |i| {
                              let name = principle_name(i);
                              view! {
                                  <div class="principle-preview-card">
                                      <div class="principle-dot" style=format!("background: var(--p{i});")></div>
                                      <div>
                                          <div class="principle-number">{format!("P{i}")}</div>
                                          <div class="principle-name">{name}</div>
                                      </div>
                                  </div>
                              }
                          }
                      />
                  </div>
              </div>
          </section>
      </div>
  }
}
