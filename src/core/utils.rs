//! Small formatting helpers shared across views.

/// Slugify a string for use in filenames / ids. Mirrors
/// `topic.toLowerCase().replace(/\s+/g, '-')`.
pub fn slugify(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .map(|c| if c.is_whitespace() { '-' } else { c })
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect()
}
