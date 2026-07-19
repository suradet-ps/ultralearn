use crate::components::icons::{ArrowLeft, CheckCircle2, Circle, Copy, Download, Pencil, Trash2};
use crate::core::types::principles_data::principles;
use crate::core::utils::slugify;
use crate::stores::use_plan;
use leptos::prelude::*;
use leptos_router::NavigateOptions;
use leptos_router::hooks::{use_navigate, use_params_map};

use web_sys::wasm_bindgen::JsCast;

#[component]
pub fn PlanOverview() -> impl IntoView {
    let store = use_plan();
    let params = use_params_map();
    let plan_id = Signal::derive(move || params.get().get("id").unwrap_or_default());
    let navigate = use_navigate();

    let progress = Signal::derive(move || store.get_progress(&plan_id.get()));
    let editing = RwSignal::new(false);
    let edit_topic = RwSignal::new(String::new());
    let edit_goal = RwSignal::new(String::new());
    let new_tag = RwSignal::new(String::new());

    let navigate = StoredValue::new(navigate);

    let go_back = Callback::new(move |_: web_sys::MouseEvent| {
        navigate.with_value(|n| n("/", NavigateOptions::default()));
    });

    let delete = Callback::new(move |_: web_sys::MouseEvent| {
        if web_sys::window()
            .and_then(|w| {
                w.confirm_with_message("Delete this plan? This cannot be undone.")
                    .ok()
            })
            .unwrap_or(false)
        {
            let id = plan_id.get_untracked();
            store.delete_plan(&id);
            navigate.with_value(|n| n("/", NavigateOptions::default()));
        }
    });

    let export = Callback::new(move |_: web_sys::MouseEvent| {
        let id = plan_id.get_untracked();
        let json = store.export_plan(&id);
        if let Some(plan) = store.get_plan(&id)
            && let Some(window) = web_sys::window()
        {
            let filename = format!("ultralearn-{}.json", slugify(&plan.topic));
            let arr = js_sys::Array::of1(&json.into());
            if let Ok(blob) = web_sys::Blob::new_with_str_sequence(&arr)
                && let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob)
                && let Some(doc) = window.document()
                && let Ok(anchor) = doc.create_element("a")
                && let Ok(anchor) = anchor.dyn_into::<web_sys::HtmlAnchorElement>()
            {
                anchor.set_href(&url);
                anchor.set_download(&filename);
                anchor.click();
                web_sys::Url::revoke_object_url(&url).ok();
            }
        }
    });

    let duplicate = Callback::new(move |_: web_sys::MouseEvent| {
        let id = plan_id.get_untracked();
        if let Some(new_plan) = store.duplicate_plan(&id) {
            let new_id = new_plan.id.clone();
            navigate.with_value(|n| n(&format!("/plan/{new_id}"), NavigateOptions::default()));
        }
    });

    let start_edit = Callback::new(move |_: web_sys::MouseEvent| {
        if let Some(p) = store.get_plan(&plan_id.get_untracked()) {
            edit_topic.set(p.topic.clone());
            edit_goal.set(p.goal.clone());
            editing.set(true);
        }
    });

    let save_edit = Callback::new(move |_: web_sys::MouseEvent| {
        let id = plan_id.get_untracked();
        let t = edit_topic.get_untracked().trim().to_string();
        if !t.is_empty() {
            store.rename_plan(&id, &t);
            store.update_plan_goal(&id, edit_goal.get_untracked().trim());
        }
        editing.set(false);
    });

    let add_tag = Callback::new(move |()| {
        let id = plan_id.get_untracked();
        let t = new_tag.get_untracked();
        if !t.trim().is_empty() {
            store.add_tag(&id, &t);
            new_tag.set(String::new());
        }
    });

    let cards_view = move || {
        let pid = plan_id.get();
        let plan = store.get_plan(&pid);
        match plan {
            None => view! { <div class="not-found"><div class="container"><p>"Plan not found."</p><button class="btn btn-primary" on:click=move |ev| go_back.run(ev)>"Go back"</button></div></div> }.into_any(),
            Some(p) => {
                let topic = p.topic.clone();
                let goal = p.goal.clone();
                let tags = p.tags.clone();
                let topic_fb = topic.clone();
                let goal_fb = goal.clone();
                view! {
                    <div class="plan-overview">
                        <div class="container">
                            <div class="plan-header">
                                <button class="btn-ghost" on:click=move |ev| go_back.run(ev)>
                                    <ArrowLeft size=18 />
                                    "Back"
                                </button>
                                <div class="plan-actions">
                                    <Show when=move || !editing.get() fallback=|| ()>
                                        <button class="btn-icon" aria-label="Edit plan" on:click=move |ev| start_edit.run(ev)>
                                            <Pencil size=18 />
                                        </button>
                                    </Show>
                                    <button class="btn-icon" aria-label="Duplicate plan" on:click=move |ev| duplicate.run(ev)>
                                        <Copy size=18 />
                                    </button>
                                    <button class="btn-icon" aria-label="Export plan" on:click=move |ev| export.run(ev)>
                                        <Download size=18 />
                                    </button>
                                    <button class="btn-icon" aria-label="Delete plan" on:click=move |ev| delete.run(ev)>
                                        <Trash2 size=18 />
                                    </button>
                                </div>
                            </div>

                            <div class="plan-title-section">
                                <Show
                                    when=move || editing.get()
                                    fallback=move || {
                                        let t = topic_fb.clone();
                                        let g = goal_fb.clone();
                                        let has_goal = !g.is_empty();
                                        view! {
                                            <h1>{t}</h1>
                                            <Show when=move || has_goal fallback=|| ()>
                                                <p class="plan-goal">{g.clone()}</p>
                                            </Show>
                                        }
                                    }
                                >
                                    <div class="edit-form">
                                        <div class="form-group">
                                            <label class="form-label">"Topic"</label>
                                            <input
                                                type="text"
                                                prop:value=move || edit_topic.get()
                                                on:input=move |ev| edit_topic.set(event_target_value(&ev))
                                            />
                                        </div>
                                        <div class="form-group">
                                            <label class="form-label">"Goal (optional)"</label>
                                            <textarea
                                                rows=3
                                                prop:value=move || edit_goal.get()
                                                on:input=move |ev| edit_goal.set(event_target_value(&ev))
                                            ></textarea>
                                        </div>
                                        <div class="form-actions">
                                            <button class="btn btn-primary" on:click=move |ev| save_edit.run(ev)>
                                                "Save"
                                            </button>
                                            <button class="btn btn-ghost" on:click=move |_| editing.set(false)>
                                                "Cancel"
                                            </button>
                                        </div>
                                    </div>
                                </Show>

                                <div class="plan-progress">
                                    <div class="progress-bar">
                                        <div
                                            class="progress-bar-fill"
                                            style=move || format!("width: {}%", progress.get())
                                        ></div>
                                    </div>
                                    <span class="progress-text">{move || format!("{}% complete", progress.get())}</span>
                                </div>

                                <div class="tag-editor">
                                    <div class="tag-list">
                                        <For
                                            each=move || tags.clone()
                                            key=|t| t.clone()
                                            children=move |t| {
                                                let tag = t.clone();
                                                let id = plan_id;
                                                view! {
                                                    <span class="tag-chip">
                                                        {tag.clone()}
                                                        <button
                                                            class="tag-remove"
                                                            aria-label=format!("Remove tag {tag}")
                                                            on:click=move |_| store.remove_tag(&id.get(), &tag)
                                                        >
                                                            "×"
                                                        </button>
                                                    </span>
                                                }
                                            }
                                        />
                                    </div>
                                    <div class="add-item tag-add">
                                        <input
                                            type="text"
                                            placeholder="Add a tag..."
                                            prop:value=move || new_tag.get()
                                            on:input=move |ev| new_tag.set(event_target_value(&ev))
                                            on:keydown=move |ev: web_sys::KeyboardEvent| {
                                                if ev.key() == "Enter" { add_tag.run(()); }
                                            }
                                        />
                                        <button class="btn btn-secondary" on:click=move |_| add_tag.run(())>
                                            "Add"
                                        </button>
                                    </div>
                                </div>
                            </div>

                            <div class="focus-streak">
                                <span class="focus-streak-label">"Focus this week"</span>
                                <div class="focus-streak-days">
                                    <For
                                        each=move || 0..7
                                        key=|i| *i
                                        children=move |_i| {
                                            let pid_streak = plan_id;
                                            let lit = Signal::derive(move || {
                                                store.focus_streak(&pid_streak.get()) > 0
                                            });
                                            view! {
                                                <span
                                                    class="streak-dot"
                                                    class:lit=move || lit.get()
                                                ></span>
                                            }
                                        }
                                    />
                                </div>
                                <span class="progress-text">
                                    {move || format!("{} day(s) with focus", store.focus_streak(&plan_id.get()))}
                                </span>
                            </div>

                            <div class="principles-grid">
                                <For
                                    each=move || principles().to_vec()
                                    key=|pr| pr.id
                                    children=move |pr| {
                                        let pid_inner = plan_id;
                                        let id = pr.id;
                                        let color = pr.color.clone();
                                        let name = pr.name.clone();
                                        let tagline = pr.tagline.clone();
                                        let c = color.clone();
                                        let c_accent = color.clone();
                                        let c_badge = color.clone();
                                        let pid_progress = pid_inner;
                                        let pr_progress = Signal::derive(move || {
                                            store.get_principle_progress(&pid_progress.get(), id)
                                        });
                                        let pid_completed = pid_inner;
                                        let is_completed = Signal::derive(move || {
                                            store
                                                .get_plan(&pid_completed.get())
                                                .and_then(|pl| {
                                                    pl.principles
                                                        .iter()
                                                        .find(|x| x.principle_id == id)
                                                        .map(|c| c.completed)
                                                })
                                                .unwrap_or(false)
                                        });
                                        let plan_id_for_click = plan_id;
                                        view! {
                                            <button class="principle-card" on:click=move |_| {
                                                let id_txt = plan_id_for_click.get_untracked();
                                                navigate.with_value(|n| n(&format!("/plan/{id_txt}/principle/{id}"), NavigateOptions::default()));
                                            }>
                                                <div class="principle-accent" style=format!("background: {c_accent};")></div>
                                                <div class="principle-card-body">
                                                    <div class="principle-card-top">
                                                        <div class="principle-badge" style=format!("background: {c_badge};")>
                                                            {id}
                                                        </div>
                                                        <Show
                                                            when=move || is_completed.get()
                                                            fallback=|| view! { <Circle size=20 color="var(--color-mute)".to_string() /> }
                                                        >
                                                            <CheckCircle2 size=20 color="var(--color-success)".to_string() />
                                                        </Show>
                                                    </div>
                                                    <h3 class="principle-card-title">{name}</h3>
                                                    <p class="principle-card-tagline">{tagline}</p>
                                                    <div class="principle-card-progress">
                                                        <div class="progress-bar">
                                                            <div
                                                                class="progress-bar-fill"
                                                                style=move || format!("width: {}%; background: {c};", pr_progress.get())
                                                            ></div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </button>
                                        }
                                    }
                                />
                            </div>
                        </div>
                    </div>
                }
                .into_any()
            }
        }
    };

    // Keyboard shortcuts: digits 1-9 jump to the corresponding principle.
    let nav = navigate;
    Effect::new(move |_| {
        let nav = nav;
        let pid = plan_id;
        window_event_listener(leptos::ev::keydown, move |ev: leptos::ev::KeyboardEvent| {
            if crate::core::utils::is_typing(&ev) {
                return;
            }
            if let Some(d) = ev.key().chars().next().and_then(|c| c.to_digit(10))
                && (1..=9).contains(&d)
            {
                ev.prevent_default();
                let id = pid.get_untracked();
                nav.with_value(|n| {
                    n(
                        &format!("/plan/{id}/principle/{d}"),
                        NavigateOptions::default(),
                    );
                });
            }
        });
    });

    view! { {cards_view} }
}
