#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }
}

const THEME_KEY: &str = "ultralearn-theme";

pub fn get_theme() -> Theme {
    use gloo_storage::{LocalStorage, Storage};
    match LocalStorage::get::<String>(THEME_KEY) {
        Ok(s) if s == "dark" => Theme::Dark,
        _ => Theme::Light,
    }
}

pub fn set_theme(theme: Theme) {
    use gloo_storage::{LocalStorage, Storage};
    let _ = LocalStorage::set(THEME_KEY, theme.as_str());
}

/// Apply the stored theme to <html data-theme> on startup (called from App).
pub fn apply_stored_theme() {
    apply_theme(get_theme());
}

pub fn apply_theme(theme: Theme) {
    if let Some(el) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element())
    {
        let _ = el.set_attribute("data-theme", theme.as_str());
    }
}
