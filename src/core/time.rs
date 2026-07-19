use web_time::{SystemTime, UNIX_EPOCH};

/// Generate a short, collision-resistant id for client-side records.
///
/// Mirrors the original `Date.now().toString(36) + Math.random()...` scheme:
/// a time component plus a random suffix. Works on `wasm32` (uses `web-time`
/// which delegates to JS `Date`/`Math.random`).
pub fn generate_id() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis());
    let rand: u32 = js_sys_random();
    format!("{secs:x}{rand:x}", rand = rand & 0xff_ffff)
}

fn js_sys_random() -> u32 {
    // Math.random() -> [0,1) -> u32
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = Math)]
        fn random() -> f64;
    }
    (random() * f64::from(u32::MAX)) as u32
}

/// Current time as an RFC-3339 / ISO-8601 string (stable, no locale drift).
pub fn now_iso() -> String {
    use chrono::DateTime;
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64);
    DateTime::from_timestamp(secs, 0).map_or_else(String::new, |d| d.to_rfc3339())
}

/// Number of days from now as an `YYYY-MM-DD` date string.
pub fn date_plus_days(days: i64) -> String {
    use chrono::DateTime;
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64);
    let target = secs + days * 86_400;
    DateTime::from_timestamp(target, 0)
        .map_or_else(String::new, |d| d.format("%Y-%m-%d").to_string())
}

/// Format an ISO date string as a short human label, e.g. `Jul 17`.
pub fn format_date_short(iso: &str) -> String {
    // iso may be `YYYY-MM-DD` or a full RFC-3339 timestamp.
    let date_part = iso.split('T').next().unwrap_or(iso);
    let parts: Vec<&str> = date_part.split('-').collect();
    if parts.len() != 3 {
        return iso.to_string();
    }
    let month = match parts[1] {
        "01" => "Jan",
        "02" => "Feb",
        "03" => "Mar",
        "04" => "Apr",
        "05" => "May",
        "06" => "Jun",
        "07" => "Jul",
        "08" => "Aug",
        "09" => "Sep",
        "10" => "Oct",
        "11" => "Nov",
        "12" => "Dec",
        _ => return iso.to_string(),
    };
    format!("{month} {}", parts[2])
}

/// Format an ISO timestamp as a local-ish short datetime for feedback log.
pub fn format_datetime_short(iso: &str) -> String {
    let date_part = iso.split('T').next().unwrap_or(iso);
    let parts: Vec<&str> = date_part.split('-').collect();
    if parts.len() != 3 {
        return iso.to_string();
    }
    let month = match parts[1] {
        "01" => "Jan",
        "02" => "Feb",
        "03" => "Mar",
        "04" => "Apr",
        "05" => "May",
        "06" => "Jun",
        "07" => "Jul",
        "08" => "Aug",
        "09" => "Sep",
        "10" => "Oct",
        "11" => "Nov",
        "12" => "Dec",
        _ => return iso.to_string(),
    };
    format!("{month} {}", parts[2])
}
