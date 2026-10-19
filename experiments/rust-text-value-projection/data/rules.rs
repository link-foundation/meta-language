// Experimental automatic translation; no native runtime integration.
/// Experimental source text-value decision.
#[must_use]
pub fn accept_rust_text_value_projection_model(model: &str) -> bool {
    model == "closed standard Rust text value projection 1"
}

/// Experimental source text-value decision.
#[must_use]
pub fn read_rust_text_value_projection_domain(standard: bool, slice: bool, layers: f64) -> String {
    if (standard && (layers == (0f64))) {
        String::from("owned standard String")
    } else {
        if (standard && (layers == (1f64))) {
            String::from("shared standard String")
        } else {
            if (slice && (layers == (1f64))) {
                String::from("shared primitive str")
            } else {
                String::from("")
            }
        }
    }
}

/// Experimental source text-value decision.
#[must_use]
pub fn accept_rust_text_value_projection_operation(operation: &str, domain: &str) -> bool {
    (((((operation == "owned standard String move") && (domain == "owned standard String"))
        || ((operation == "standard String construction from borrowed text")
            && ((domain == "shared primitive str")
                || (domain == "static primitive str literal"))))
        || ((operation == "standard String clone")
            && ((domain == "owned standard String") || (domain == "shared standard String"))))
        || ((operation == "standard borrowed str owned construction")
            && ((domain == "shared primitive str") || (domain == "static primitive str literal"))))
        || ((operation == "standard borrowed str display construction")
            && ((domain == "shared primitive str") || (domain == "static primitive str literal")))
}
