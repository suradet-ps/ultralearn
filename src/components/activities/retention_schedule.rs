use crate::components::icons::{Calendar, Check};
use crate::core::time::format_date_short;
use crate::core::types::RetentionSchedule;
use crate::stores::use_plan;
use leptos::prelude::*;

use web_time::{SystemTime, UNIX_EPOCH};

#[component]
pub fn RetentionSchedule(plan_id: String, principle_id: u32) -> impl IntoView {
    let store = use_plan();
    let plan_id = RwSignal::new(plan_id);
    let topic = RwSignal::new(String::new());

    let schedules = Signal::derive(move || store.get_schedules(&plan_id.get(), principle_id));

    let add = Callback::new(move |()| {
        let t = topic.get_untracked();
        if !t.trim().is_empty() {
            store.add_schedule(&plan_id.get(), principle_id, t.trim());
            topic.set(String::new());
        }
    });

    let toggle = Callback::new(move |(schedule_id, date): (String, String)| {
        store.toggle_schedule_date(&plan_id.get(), principle_id, &schedule_id, &date);
    });

    view! {
        <div class="retention-section">
            <h3>"Spaced Repetition Schedule"</h3>
            <p class="desc">"Create review schedules: Day 1, 3, 7, 14, 30"</p>

            <div class="add-form">
                <input
                    type="text"
                    placeholder="Topic or concept to schedule for review"
                    prop:value=move || topic.get()
                    on:input=move |ev| topic.set(event_target_value(&ev))
                    on:keydown=move |ev: web_sys::KeyboardEvent| {
                        if ev.key() == "Enter" { add.run(()); }
                    }
                />
                <button
                    class="btn btn-primary"
                    disabled=Signal::derive(move || topic.get().trim().is_empty())
                    on:click=move |_| add.run(())
                >
                    <Calendar size=16 />
                    "Generate Schedule"
                </button>
            </div>

            <Show
                when=move || schedules.get().is_empty()
                fallback=move || {
                    view! {
                        <div class="schedule-list">
                            <For
                                each=move || schedules.get()
                                key=|s: &RetentionSchedule| s.id.clone()
                                children=move |s| {
                                    let schedule_id = s.id.clone();
                                    let topic_text = s.topic.clone();
                                    let dates = s.dates.clone();
                                    let completed_dates = s.completed_dates.clone();
                                    view! {
                                        <div class="schedule-card">
                                            <div class="schedule-topic">{topic_text}</div>
                                            <div class="schedule-dates">
                                                <For
                                                    each=move || dates.clone()
                                                    key=|d| d.clone()
                                                    children=move |date| {
                                                        let sid = schedule_id.clone();
                                                        let completed = completed_dates.contains(&date.clone());
                                                        let is_past = is_past(&date) && !completed;
                                                        let date_label = format_date_short(&date);
                                                        let date_owned = date.clone();
                                                        view! {
                                                            <button
                                                                class="date-btn"
                                                                class:completed
                                                                class:due=is_past
                                                                on:click=move |_| toggle.run((sid.clone(), date_owned.clone()))
                                                            >
                                                                <Show when=move || completed fallback=|| ()>
                                                                    <Check size=14 />
                                                                </Show>
                                                                {date_label}
                                                            </button>
                                                        }
                                                    }
                                                />
                                            </div>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    }
                }
            >
                <div class="empty">
                    "No schedules yet. Add topics to track your spaced repetition."
                </div>
            </Show>
        </div>
    }
}

fn is_past(date_str: &str) -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let day_start = |s: &str| -> Option<i64> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return None;
        }
        let y: i64 = parts[0].parse().ok()?;
        let m: i64 = parts[1].parse().ok()?;
        let d: i64 = parts[2].parse().ok()?;
        Some((y - 1970) * 365 + m * 30 + d)
    };
    let now_date = {
        use chrono::DateTime;
        DateTime::from_timestamp(now as i64, 0)
            .map_or_else(String::new, |d| d.format("%Y-%m-%d").to_string())
    };
    match (day_start(date_str), day_start(&now_date)) {
        (Some(d), Some(t)) => d <= t,
        _ => false,
    }
}
