use crate::components::icons::{BookOpen, Moon, Sun};
use leptos::prelude::*;
use leptos_router::components::A;

use crate::core::theme::{Theme, apply_theme, get_theme, set_theme};

#[component]
pub fn Topbar() -> impl IntoView {
    let theme = RwSignal::new(get_theme());

    let toggle_theme = move |_| {
        let next = match theme.get_untracked() {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        };
        set_theme(next);
        theme.set(next);
        apply_theme(next);
    };

    view! {
        <nav class="nav-bar">
            <div class="nav-inner">
                <span class="nav-logo">
                    <A href="/">
                        <BookOpen size=24 />
                        <span>"Ultralearn"</span>
                    </A>
                </span>
                <div class="nav-actions">
                    <button
                        class="btn-icon"
                        on:click=toggle_theme
                        aria-label="Toggle theme"
                    >
                        <Show when=move || theme.get() == Theme::Dark fallback=|| view! { <Sun size=18 /> }>
                            <Moon size=18 />
                        </Show>
                    </button>
                </div>
            </div>
        </nav>
    }
}
