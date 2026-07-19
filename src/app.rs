use crate::components::layout::Topbar;
use crate::core::theme::apply_stored_theme;
use crate::views::{LandingView, PlanOverview, PrincipleDetail};
use leptos::prelude::*;
use leptos_meta::{Title, provide_meta_context};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    apply_stored_theme();

    view! {
        <Title text="Ultralearn" />
        <Router>
            <Topbar />
            <main class="main-content">
                <Routes fallback=|| view! { <LandingView /> }>
                    <Route path=path!("/") view=LandingView />
                    <Route path=path!("/plan/:id") view=PlanOverview />
                    <Route path=path!("/plan/:id/principle/:principleId") view=PrincipleDetail />
                </Routes>
            </main>
        </Router>
    }
}
