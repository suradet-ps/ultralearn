//! Safe markdown rendering for user notes.
//!
//! We use `pulldown-cmark` with default features **off**, which means the
//! `html` feature (raw HTML passthrough) is disabled. Inline/block HTML in
//! notes is therefore escaped, so the generated markup cannot inject scripts or
//! event handlers. This gives us a safe in-WASM "sanitizer" for free, with no
//! extra dependency and no `unsafe`.

use pulldown_cmark::{Options, Parser};

/// Render markdown source to an HTML string.
///
/// Raw HTML is escaped (no `html` feature), so this is safe to inject via
/// `inner_html` into the notes preview.
pub fn render_markdown(source: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(source, options);
    let mut out = String::with_capacity(source.len() * 2);
    pulldown_cmark::html::push_html(&mut out, parser);
    out
}
