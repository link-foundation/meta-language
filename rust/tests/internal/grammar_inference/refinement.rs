#[cfg(test)]
mod tests {
    use super::rules;
    #[test]
    fn shared_journal_profile_keeps_bounds_versions_and_source_identities() {
        for count in [0.0, 1.0, 8.0] {
            assert!(rules::accept_regular_grammar_refinement_count(count));
        }
        for count in [-1.0, 0.5, 9.0, f64::NAN, f64::INFINITY] {
            assert!(!rules::accept_regular_grammar_refinement_count(count));
        }
        assert!(rules::accept_regular_grammar_refinement_journal(
            1.0,
            "labelled regular grammar refinement 1"
        ));
        assert!(!rules::accept_regular_grammar_refinement_journal(
            2.0,
            "labelled regular grammar refinement 1"
        ));
        assert!(rules::accept_regular_grammar_refinement_identifier(
            "new",
            &["old".to_owned()]
        ));
        assert!(!rules::accept_regular_grammar_refinement_identifier(
            "old",
            &["old".to_owned()]
        ));
    }
}
