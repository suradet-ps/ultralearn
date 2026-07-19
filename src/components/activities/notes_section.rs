use crate::stores::use_plan;
use leptos::prelude::*;

#[component]
pub fn NotesSection(plan_id: String, principle_id: u32) -> impl IntoView {
    let store = use_plan();
    let plan_id = RwSignal::new(plan_id);
    let notes = RwSignal::new(store.get_principle_notes(&plan_id.get_untracked(), principle_id));

    view! {
        <div class="notes-section">
            <h3>"Notes"</h3>
            <textarea
                placeholder="Write your notes, reflections, and plans here..."
                prop:value=move || notes.get()
                on:input=move |ev| {
                    let v = event_target_value(&ev);
                    notes.set(v.clone());
                    store.update_principle_notes(&plan_id.get(), principle_id, &v);
                }
            ></textarea>
        </div>
    }
}
