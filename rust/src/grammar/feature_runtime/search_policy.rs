// Generated from the JavaScript grammar search policy.
pub(super) const fn inside_grammar_search_word(previous: bool, following: bool) -> bool {
    previous && following
}

pub(super) const fn accept_grammar_search_end(advances: bool, accepted_boundary: bool) -> bool {
    advances && accepted_boundary
}

pub(super) const fn skip_grammar_search_start(overlaps: bool, overlapping: bool) -> bool {
    overlaps && !overlapping
}
