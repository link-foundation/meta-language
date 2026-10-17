// Automatically translated from the complete shared representation policy. Do not edit.
/// Apply the retained shared UTF16 representation decision.
#[must_use]
pub fn accept_self_translation_utf16_member(name: &str) -> bool {
    (((((((((name == "split") || (name == "slice")) || (name == "indexOf"))
        || (name == "lastIndexOf"))
        || (name == "replace"))
        || (name == "replaceAll"))
        || (name == "codePointAt"))
        || (name == "repeat"))
        || (name == "padStart"))
        || (name == "padEnd")
        || (name == "charCodeAt")
}

/// Apply the retained shared UTF16 representation decision.
#[must_use]
pub fn require_self_translation_utf16_representation(
    explicit: bool,
    family: &str,
    member: bool,
) -> bool {
    explicit || ((family == "JavaScript") && member)
}

/// Apply the retained shared UTF16 representation decision.
#[must_use]
pub fn require_self_translation_utf16_text(kind: &str) -> bool {
    kind == "string"
}
