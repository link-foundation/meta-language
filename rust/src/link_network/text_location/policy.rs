// Generated from the JavaScript text location policy.
pub(super) fn select_text_edit(diagonal: f64, deletion: f64, insertion: f64) -> f64 {
    if diagonal <= deletion && diagonal <= insertion {
        0.0
    } else if deletion <= insertion {
        1.0
    } else {
        2.0
    }
}

pub(super) fn normalize_quote_scalar(value: f64) -> f64 {
    if value == 171.0
        || value == 187.0
        || value == 8220.0
        || value == 8221.0
        || value == 8222.0
        || value == 8223.0
        || value == 8249.0
        || value == 8250.0
    {
        34.0
    } else if value == 8216.0 || value == 8217.0 || value == 8218.0 || value == 8219.0 {
        39.0
    } else {
        value
    }
}

pub(super) fn normalize_yo_scalar(value: f64) -> f64 {
    if value == 1025.0 {
        1045.0
    } else if value == 1105.0 {
        1077.0
    } else {
        value
    }
}
