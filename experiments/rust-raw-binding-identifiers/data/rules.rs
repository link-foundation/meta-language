// Experimental automatic translation; no native runtime integration.
/// Experimental raw binding identity decision.
#[must_use]
pub fn accept_rust_raw_binding_identifier_model(model: &str) -> bool {
    model == "Rust raw binding identifier interpretation 1"
}

/// Experimental raw binding identity decision.
#[must_use]
pub fn accept_rust_raw_binding_identifier_name(
    name: &str,
    raw_: bool,
    characters_accepted: bool,
) -> bool {
    if ((((((!characters_accepted || (name == "")) || (name == "_")) || (name == "crate"))
        || (name == "self"))
        || (name == "super"))
        || (name == "Self"))
    {
        false
    } else {
        (raw_
            || !((((((((((((((((((((((((((((((((((((((((((((((((((((name == "as")
                || (name == "async"))
                || (name == "await"))
                || (name == "break"))
                || (name == "const"))
                || (name == "continue"))
                || (name == "crate"))
                || (name == "dyn"))
                || (name == "else"))
                || (name == "enum"))
                || (name == "extern"))
                || (name == "false"))
                || (name == "fn"))
                || (name == "for"))
                || (name == "if"))
                || (name == "impl"))
                || (name == "in"))
                || (name == "let"))
                || (name == "loop"))
                || (name == "match"))
                || (name == "mod"))
                || (name == "move"))
                || (name == "mut"))
                || (name == "pub"))
                || (name == "ref"))
                || (name == "return"))
                || (name == "self"))
                || (name == "Self"))
                || (name == "static"))
                || (name == "struct"))
                || (name == "super"))
                || (name == "trait"))
                || (name == "true"))
                || (name == "type"))
                || (name == "unsafe"))
                || (name == "use"))
                || (name == "where"))
                || (name == "while"))
                || (name == "abstract"))
                || (name == "become"))
                || (name == "box"))
                || (name == "do"))
                || (name == "final"))
                || (name == "gen"))
                || (name == "macro"))
                || (name == "override"))
                || (name == "priv"))
                || (name == "try"))
                || (name == "typeof"))
                || (name == "unsized"))
                || (name == "virtual"))
                || (name == "yield")))
    }
}

/// Experimental raw binding identity decision.
#[must_use]
pub fn accept_rust_raw_binding_identifier_type(kind: &str, signed: bool, bits: f64) -> bool {
    (kind == "bool") || (((kind == "fixed") && signed) && (bits.to_bits() == 32f64.to_bits()))
}

/// Experimental raw binding identity decision.
#[must_use]
pub const fn accept_rust_raw_binding_identifier_closure(recursive: bool, mutual: bool) -> bool {
    !recursive && !mutual
}
