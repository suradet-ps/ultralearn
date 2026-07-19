use crate::components::icons::{Plus, X};
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
    Danger,
}

impl ButtonVariant {
    fn class(self) -> &'static str {
        match self {
            ButtonVariant::Primary => "btn-primary",
            ButtonVariant::Secondary => "btn-secondary",
            ButtonVariant::Ghost => "btn-ghost",
            ButtonVariant::Danger => "btn-danger",
        }
    }
}

#[component]
pub fn BaseButton(
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(default = false)] loading: bool,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = "button")] button_type: &'static str,
    #[prop(default = false)] block: bool,
    #[prop(default = false)] large: bool,
    #[prop(optional, into)] on_click: Option<Callback<web_sys::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let class = format!(
        "btn {}{}{}",
        variant.class(),
        if large { " btn-lg" } else { "" },
        if block { " btn-block" } else { "" },
    );
    let is_disabled = Signal::derive(move || disabled.get() || loading);
    view! {
        <button
            type=button_type
            class=class
            disabled=move || is_disabled.get()
            on:click=move |ev| {
                if !is_disabled.get()
                    && let Some(h) = on_click.as_ref()
                {
                    h.run(ev);
                }
            }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn BaseInput(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_input: Callback<String>,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(default = "text")] input_type: &'static str,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(default = false)] required: bool,
    #[prop(optional)] rows: Option<u32>,
) -> impl IntoView {
    let input_view = move || {
        if let Some(rows) = rows {
            view! {
                <textarea
                    id=id
                    rows=rows
                    placeholder=placeholder
                    required=required
                    prop:value=move || value.get()
                    on:input=move |ev| {
                        let v = event_target_value(&ev);
                        on_input.run(v);
                    }
                >
                </textarea>
            }
            .into_any()
        } else {
            view! {
                <input
                    id=id
                    type=input_type
                    placeholder=placeholder
                    required=required
                    prop:value=move || value.get()
                    on:input=move |ev| {
                        let v = event_target_value(&ev);
                        on_input.run(v);
                    }
                />
            }
            .into_any()
        }
    };

    if let Some(label) = label {
        view! {
            <div class="form-group">
                <label for=id class="form-label">
                    {label}
                </label>
                {input_view()}
            </div>
        }
        .into_any()
    } else {
        input_view()
    }
}

/// Small reusable "add" row with a text input + Plus button.
#[component]
pub fn AddRow(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_input: Callback<String>,
    #[prop(into)] on_add: Callback<()>,
    placeholder: &'static str,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let can_add = move || !value.get().trim().is_empty() && !disabled;
    let key_handler = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" && can_add() {
            on_add.run(());
        }
    };
    view! {
        <div class="add-item">
            <input
                type="text"
                placeholder=placeholder
                prop:value=move || value.get()
                on:input=move |ev| on_input.run(event_target_value(&ev))
                on:keydown=key_handler
            />
            <BaseButton
                variant=ButtonVariant::Secondary
                disabled=Signal::derive(move || !can_add())
                on_click=Callback::new(move |_| on_add.run(()))
            >
                <Plus size=16 />
            </BaseButton>
        </div>
    }
}

/// Small reusable remove button (renders an X icon).
#[component]
pub fn RemoveButton(
    #[prop(into)] on_click: Callback<web_sys::MouseEvent>,
    #[prop(optional)] label: Option<&'static str>,
) -> impl IntoView {
    let aria = label.unwrap_or("Remove");
    view! {
        <button class="remove-btn" on:click=move |ev| on_click.run(ev) aria-label=aria>
            <X size=14 />
        </button>
    }
}
