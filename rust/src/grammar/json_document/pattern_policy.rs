// Generated from js/src/grammar-importers/json-schema-pattern-policy.js.
pub fn intersect_scalar_intervals(first: f64, last: f64, low: f64, high: f64) -> (f64, f64) {
    (first.max(low), last.min(high))
}

pub fn surrogate_pair_boundaries(first: f64, last: f64) -> (f64, f64, f64, f64) {
    let first_low = (first - 65536.0) % 1024.0;
    let last_low = (last - 65536.0) % 1024.0;
    (
        55296.0 + (first - 65536.0 - first_low) / 1024.0,
        56320.0 + first_low,
        55296.0 + (last - 65536.0 - last_low) / 1024.0,
        56320.0 + last_low,
    )
}

pub(super) const fn scalar_from_surrogate_pair(lead: u32, trail: u32) -> u32 {
    0x1_0000 + (lead - 0xd800) * 1024 + (trail - 0xdc00)
}

pub fn consuming_pattern_lower_bound(nullable: bool, minimum: f64) -> f64 {
    if nullable { 0.0 } else { minimum }
}

pub fn bounded_pattern_repetition(minimum: f64, maximum: f64) -> bool {
    minimum <= 1024.0 && (maximum < 0.0 || (maximum <= 1024.0 && minimum <= maximum))
}

pub fn string_intersection_limits() -> (f64, f64, f64) {
    (12288f64, 4096f64, 3000000f64)
}
