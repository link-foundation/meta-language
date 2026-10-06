//! The rule names the name tier of reconciliation compares, the twin of
//! `comparedName` in js/src/grammar-reconcile.js.

// A rule name up to case, `_` and `-`, without a leading name of its language
// that a case change or `_` ends: `htmlElement` and `HTML_ELEMENT` of HTML
// compare as `element`, `comment` of C stays `comment`.
pub(super) fn compared_name(name: &str, language: &str) -> String {
    let bare = |text: &str| -> String {
        text.to_lowercase()
            .chars()
            .filter(|character| *character != '-' && *character != '_')
            .collect()
    };
    let prefix: String = language
        .to_lowercase()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect();
    let characters: Vec<char> = name.chars().collect();
    if !prefix.is_empty()
        && characters.len() > prefix.len()
        && characters[..prefix.len()]
            .iter()
            .collect::<String>()
            .to_lowercase()
            == prefix
    {
        let rest: String = characters[prefix.len()..].iter().collect();
        let next = characters[prefix.len()];
        let last = characters[prefix.len() - 1];
        let boundary = next == '-'
            || next == '_'
            || (next.is_ascii_uppercase() && last.to_uppercase().next() != Some(last));
        if boundary && !bare(&rest).is_empty() {
            return bare(&rest);
        }
    }
    bare(name)
}
