use crate::components::icons::{ArrowRight, Sparkles};
use crate::core::types::principles_data::principle_name;
use crate::stores::use_plan;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn LandingView() -> impl IntoView {
    let topic = RwSignal::new(String::new());
    let goal = RwSignal::new(String::new());
    let navigate = use_navigate();

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
        let store = use_plan();
        let plan = store.create_plan(t.trim(), goal.get_untracked().trim());
        navigate(
            &format!("/plan/{}", plan.id),
            leptos_router::NavigateOptions::default(),
        );
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
