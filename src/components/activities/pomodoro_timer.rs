use crate::components::icons::{Pause, Play, RotateCcw};
use crate::composables::{TimerMode, use_timer};
use crate::core::time::format_datetime_short;
use leptos::prelude::*;

fn fmt(seconds: u32) -> String {
    let m = seconds / 60;
    let s = seconds % 60;
    format!("{m:02}:{s:02}")
}

#[component]
pub fn PomodoroTimer(plan_id: String) -> impl IntoView {
    let timer = use_timer(plan_id);
    let modes = [TimerMode::Work, TimerMode::ShortBreak, TimerMode::LongBreak];

    view! {
        <div class="pomodoro">
            <h3>"Pomodoro Timer"</h3>
            <div class="timer-modes">
                <For
                    each=move || modes
                    key=|m| *m
                    children=move |m| {
                        view! {
                            <button
                                class="mode-btn"
                                class:active=move || timer.mode.get() == m
                                on:click=move |_| timer.set_mode.run(m)
                            >
                                {m.label()}
                            </button>
                        }
                    }
                />
            </div>
            <div class="timer-display">{move || fmt(timer.seconds.get())}</div>
            <div class="timer-progress">
                <div
                    class="timer-progress-fill"
                    style=move || format!("width: {}%", timer.progress.get())
                ></div>
            </div>
            <div class="timer-controls">
                <button class="btn btn-primary" on:click=move |_| {
                    if timer.is_running.get() { timer.stop.run(()) } else { timer.start.run(()) }
                }>
                    <Show when=move || timer.is_running.get() fallback=|| view! { <Play size=18 /> }>
                        <Pause size=18 />
                    </Show>
                    {move || if timer.is_running.get() { "Pause" } else { "Start" }}
                </button>
                <button class="btn btn-ghost" on:click=move |_| timer.reset.run(())>
                    <RotateCcw size=18 />
                    "Reset"
                </button>
            </div>
            <div class="sessions">"Sessions completed: " {move || timer.sessions.get()}</div>

            <Show
                when=move || !timer.history.get().is_empty()
                fallback=|| ()
            >
                <div class="session-history">
                    <h4>"Recent focus sessions"</h4>
                    <ul>
                        <For
                            each=move || timer.history.get()
                            key=|s| s.id.clone()
                            children=move |s| {
                                let mins = s.duration_secs / 60;
                                view! {
                                    <li>
                                        <span class="session-date">{format_datetime_short(&s.finished_at)}</span>
                                        <span class="session-dur">{format!("{mins} min")}</span>
                                    </li>
                                }
                            }
                        />
                    </ul>
                </div>
            </Show>
        </div>
    }
}
