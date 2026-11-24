// Generated from complete js/src/translation/rust-concrete-source-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_concrete_item_kind(term: &str) -> String {
    {
        let source_ml_temporary_switch1 = term.to_owned();
        if source_ml_temporary_switch1 == "structure_item" {
            String::from("struct_item")
        } else if source_ml_temporary_switch1 == "enumeration_item" {
            String::from("enum_item")
        } else if source_ml_temporary_switch1 == "constant_item" {
            String::from("const_item")
        } else if source_ml_temporary_switch1 == "module_item" {
            String::from("mod_item")
        } else if source_ml_temporary_switch1 == "implementation_item" {
            String::from("impl_item")
        } else {
            term.to_owned()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_concrete_selectable_item_kind(term: &str) -> bool {
    {
        let kind = self::read_rust_concrete_item_kind(term);
        ((((((((kind == "function_item") || (kind == "struct_item")) || (kind == "enum_item"))
            || (kind == "union_item"))
            || (kind == "trait_item"))
            || (kind == "type_item"))
            || (kind == "const_item"))
            || (kind == "static_item"))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_concrete_source_containment(
    parent_start: f64,
    parent_end: f64,
    child_start: f64,
    child_end: f64,
) -> bool {
    (parent_start <= child_start) && (child_end <= parent_end)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_concrete_source_enclosure(
    parent_start: f64,
    parent_end: f64,
    child_start: f64,
    child_end: f64,
) -> bool {
    self::test_rust_concrete_source_containment(parent_start, parent_end, child_start, child_end)
        && ((parent_start < child_start) || (child_end < parent_end))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_concrete_module_declaration(
    term: &str,
    start: f64,
    end: f64,
    module_start: f64,
    module_end: f64,
) -> bool {
    (((self::read_rust_concrete_item_kind(term)) == "mod_item") && (start <= module_start))
        && ((end).partial_cmp(&(module_end)) == Some(std::cmp::Ordering::Equal))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_concrete_entry_owner_kind(term: &str) -> bool {
    {
        let kind = self::read_rust_concrete_item_kind(term);
        ((((kind == "mod_item") || (kind == "function_item")) || (kind == "impl_item"))
            || (kind == "trait_item"))
    }
}
