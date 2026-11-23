// Generated from js/src/grammar-importers/json-schema-tag-policy.js.
pub(super) fn string_sets_are_disjoint(left: &[&str], right: &[&str]) -> bool {
    left.iter().all(|value| !right.contains(value))
}
