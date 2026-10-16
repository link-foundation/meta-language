// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_network_projection(projection: &str) -> bool {
    (((projection == "Lossless") || (projection == "ConcreteSyntax"))
        || (projection == "AbstractSyntax"))
        || (projection == "Semantic")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_network_projection_link(projection: &str, link_type: &str) -> bool {
    if projection == "Lossless" {
        true
    } else if projection == "ConcreteSyntax" {
        link_type != "Semantic"
    } else if projection == "AbstractSyntax" {
        (link_type != "SourceToken") && (link_type != "Trivia")
    } else if projection == "Semantic" {
        (((link_type == "Semantic") || (link_type == "Concept")) || (link_type == "Type"))
            || (link_type == "Language")
    } else {
        false
    }
}
