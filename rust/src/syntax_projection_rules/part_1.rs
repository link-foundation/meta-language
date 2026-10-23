// Generated from complete js/src/syntax-projection-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_syntax_projection_term(role: &str) -> String {
    if role == "language" {
        String::from("meta-language syntax projection")
    } else if role == "root" {
        String::from("abstract syntax")
    } else if role == "node" {
        String::from("abstract syntax node")
    } else if role == "children" {
        String::from("abstract syntax children")
    } else if role == "edge" {
        String::from("abstract syntax edge")
    } else if role == "index" {
        String::from("concrete syntax child position")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_concrete_syntax_node_type(type_: &str) -> bool {
    ((((type_ == "node") || (type_ == "token")) || (type_ == "embed")) || (type_ == "error"))
        || (type_ == "missing")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_abstract_syntax_node(type_: &str, trivia: bool, dropped: bool) -> bool {
    !dropped && (((type_ == "error") || (type_ == "missing")) || !trivia)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_concrete_syntax_child_role(type_: &str, role: &str) -> bool {
    if type_ == "node" {
        role == "children"
    } else if type_ == "embed" {
        ((role == "root") || (role == "sourceChildren")) || (role == "alternative root")
    } else {
        false
    }
}
