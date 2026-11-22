// Generated from the JavaScript pest importer policy.
pub(super) fn allow_pest_implicit_trivia(
    inherited: bool,
    trivia_rule: bool,
    atomic: bool,
    non_atomic: bool,
) -> bool {
    !trivia_rule && (non_atomic || (inherited && !atomic))
}
