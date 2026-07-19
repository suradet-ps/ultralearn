use crate::components::activities::{
    ChecklistSection, ExperimentTracker, FeedbackLog, FeynmanWorkspace, FlashcardSection,
    NotesSection, PomodoroTimer, RetentionSchedule,
};
use crate::components::icons::{ArrowLeft, CheckCircle2, Circle};
use crate::core::types::principles_data::principles;
use crate::stores::use_plan;
use leptos::prelude::*;
use leptos_router::NavigateOptions;
use leptos_router::hooks::{use_navigate, use_params_map};

#[component]
pub fn PrincipleDetail() -> impl IntoView {
    let store = use_plan();
    let params = use_params_map();
    let plan_id = Signal::derive(move || params.get().get("id").unwrap_or_default());
    let principle_id = Signal::derive(move || {
        params
            .get()
            .get("principleId")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0)
    });
    let navigate = use_navigate();

    let plan = Signal::derive(move || store.get_plan(&plan_id.get()));
    let principle = Signal::derive(move || {
        principles()
            .iter()
            .find(|p| p.id == principle_id.get())
            .cloned()
    });
    let completed = Signal::derive(move || {
        store
            .get_plan(&plan_id.get())
            .and_then(|p| {
                p.principles
                    .iter()
                    .find(|pp| pp.principle_id == principle_id.get())
                    .map(|pp| pp.completed)
            })
            .unwrap_or(false)
    });

    let navigate = StoredValue::new(navigate);

    let go_back = Callback::new(move |_: web_sys::MouseEvent| {
        let id = plan_id.get_untracked();
        navigate.with_value(|n| n(&format!("/plan/{id}"), NavigateOptions::default()));
    });

    let toggle = Callback::new(move |_: web_sys::MouseEvent| {
        let id = plan_id.get_untracked();
        let pid = principle_id.get_untracked();
        store.toggle_principle_completed(&id, pid);
    });

    let body = move || {
        let plan = plan.get();
        let principle = principle.get();
        match (plan, principle) {
            (Some(_), Some(pr)) => {
                let pid = pr.id;
                let color = pr.color.clone();
                let name = pr.name.clone();
                let tagline = pr.tagline.clone();
                let description = pr.description.clone();
                view! {
                    <div class="principle-detail">
                        <div class="container">
                            <div class="detail-header">
                                <button class="btn-ghost" on:click=move |ev| go_back.run(ev)>
                                    <ArrowLeft size=18 />
                                    "Back to Plan"
                                </button>
                            </div>

                            <div class="principle-hero" style=format!("border-color: {color};")>
                                <div class="principle-badge-lg" style=format!("background: {color};")>
                                    {pid}
                                </div>
                                <div class="principle-info">
                                    <div class="principle-tag" style=format!("color: {color};")>
                                        {format!("Principle {pid}")}
                                    </div>
                                    <h1>{name}</h1>
                                    <p class="principle-tagline">{tagline}</p>
                                </div>
                                <button
                                    class="complete-btn"
                                    class:completed=move || completed.get()
                                    on:click=move |ev| toggle.run(ev)
                                >
                                    <Show when=move || completed.get() fallback=|| view! { <Circle size=20 /> }>
                                        <CheckCircle2 size=20 />
                                    </Show>
                                    {move || if completed.get() { "Completed" } else { "Mark Complete" }}
                                </button>
                            </div>

                            <p class="principle-desc">{description}</p>

                            <div class="detail-body">
                                <div class="detail-main">
                                    <Show when=move || pid == 2 fallback=|| ()>
                                        <PomodoroTimer />
                                    </Show>
                                    <Show when=move || pid == 5 fallback=|| ()>
                                        <FlashcardSection plan_id=plan_id.get() principle_id=pid />
                                    </Show>
                                    <Show when=move || pid == 8 fallback=|| ()>
                                        <FeynmanWorkspace plan_id=plan_id.get() principle_id=pid />
                                    </Show>
                                    <Show when=move || pid == 6 fallback=|| ()>
                                        <FeedbackLog plan_id=plan_id.get() principle_id=pid />
                                    </Show>
                                    <Show when=move || pid == 9 fallback=|| ()>
                                        <ExperimentTracker plan_id=plan_id.get() principle_id=pid />
                                    </Show>
                                    <Show when=move || pid == 7 fallback=|| ()>
                                        <RetentionSchedule plan_id=plan_id.get() principle_id=pid />
                                    </Show>

                                    <ChecklistSection plan_id=plan_id.get() principle_id=pid />
                                    <NotesSection plan_id=plan_id.get() principle_id=pid />
                                </div>
                            </div>
                        </div>
                    </div>
                }
                .into_any()
            }
            _ => {
                view! {
                    <div class="not-found">
                        <div class="container">
                            <p>"Principle not found."</p>
                            <button class="btn btn-primary" on:click=move |ev| go_back.run(ev)>"Go back"</button>
                        </div>
                    </div>
                }
                .into_any()
            }
        }
    };

    view! { {body} }
}
