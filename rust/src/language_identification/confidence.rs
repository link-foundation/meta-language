// Generated from the JavaScript language identification confidence rules.
pub const fn trigram_confidence(distance: f64, maximum_distance: f64, letters: f64) -> f64 {
    let support = if letters < 8.0 { letters / 8.0 } else { 1.0 };
    (1.0 - distance / maximum_distance) * support * support * support
}

pub const fn minimum_language_lead(first: f64, second: f64) -> bool {
    second < first * 0.97
}
