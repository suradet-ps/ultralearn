use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimerMode {
    Work,
    ShortBreak,
    LongBreak,
}

impl TimerMode {
    pub fn label(self) -> &'static str {
        match self {
            TimerMode::Work => "Focus",
            TimerMode::ShortBreak => "Short Break",
            TimerMode::LongBreak => "Long Break",
        }
    }
    pub fn duration(self) -> u32 {
        match self {
            TimerMode::Work => 25 * 60,
            TimerMode::ShortBreak => 5 * 60,
            TimerMode::LongBreak => 15 * 60,
        }
    }
}

pub struct TimerHandle {
    pub seconds: Signal<u32>,
    pub is_running: Signal<bool>,
    pub mode: Signal<TimerMode>,
    pub sessions: Signal<u32>,
    pub progress: Signal<f64>,
    pub start: Callback<()>,
    pub stop: Callback<()>,
    pub reset: Callback<()>,
    pub set_mode: Callback<TimerMode>,
}

/// A Pomodoro timer composable. Mirrors `PomodoroTimer.vue`.
///
/// Uses `web_sys` `setInterval`/`clearInterval` and clears the handle on
/// cleanup via `on_cleanup` so navigating away doesn't leak the timer.
pub fn use_timer() -> TimerHandle {
    let seconds = RwSignal::new(TimerMode::Work.duration());
    let is_running = RwSignal::new(false);
    let mode = RwSignal::new(TimerMode::Work);
    let sessions = RwSignal::new(0u32);
    let handle: RwSignal<Option<i32>> = RwSignal::new(None);

    let clear = move || {
        if let Some(h) = handle.get_untracked() {
            let window = web_sys::window().expect("no window");
            window.clear_interval_with_handle(h);
            handle.set(None);
        }
        is_running.set(false);
    };

    let switch_mode = move |new_mode: TimerMode| {
        clear();
        mode.set(new_mode);
        seconds.set(new_mode.duration());
    };

    let tick = move || {
        if seconds.get_untracked() > 0 {
            seconds.set(seconds.get_untracked() - 1);
        } else {
            let finished = mode.get_untracked();
            if finished == TimerMode::Work {
                let s = sessions.get_untracked() + 1;
                sessions.set(s);
                if s.is_multiple_of(4) {
                    switch_mode(TimerMode::LongBreak);
                } else {
                    switch_mode(TimerMode::ShortBreak);
                }
            } else {
                switch_mode(TimerMode::Work);
            }
        }
    };

    let start = Callback::new(move |()| {
        if is_running.get_untracked() {
            return;
        }
        is_running.set(true);
        let closure: Closure<dyn Fn()> = Closure::wrap(Box::new(tick));
        let window = web_sys::window().expect("no window");
        let h = window.set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            1000,
        );
        if let Ok(h) = h {
            // Leak the closure for the lifetime of the interval; cleared on stop.
            std::mem::forget(closure);
            handle.set(Some(h));
        }
    });

    let stop = Callback::new(move |()| clear());

    let reset = Callback::new(move |()| {
        clear();
        seconds.set(mode.get_untracked().duration());
    });

    let set_mode = Callback::new(move |m: TimerMode| switch_mode(m));

    on_cleanup(clear);

    let progress = Signal::derive(move || {
        let total = mode.get().duration();
        let remaining = seconds.get();
        if total == 0 {
            0.0
        } else {
            (f64::from(total - remaining) / f64::from(total)) * 100.0
        }
    });

    TimerHandle {
        seconds: seconds.into(),
        is_running: is_running.into(),
        mode: mode.into(),
        sessions: sessions.into(),
        progress,
        start,
        stop,
        reset,
        set_mode,
    }
}
