//! JavaScript names: a source name made a valid identifier, with `_` after a
//! word the emitter reserves.

pub(super) const KEYWORDS: &[&str] = &[
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "enum",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "function",
    "if",
    "import",
    "in",
    "instanceof",
    "new",
    "null",
    "return",
    "super",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "typeof",
    "var",
    "void",
    "while",
    "with",
    "yield",
    "await",
    "let",
    "static",
    "implements",
    "interface",
    "package",
    "private",
    "protected",
    "public",
    "arguments",
    "eval",
    "undefined",
    "NaN",
    "Infinity",
    "globalThis",
    "console",
    "process",
    "BigInt",
    "String",
    "Object",
    "Error",
    "RangeError",
    "Math",
    "Number",
    "Array",
    "JSON",
    "Symbol",
    "main",
    "__proto__",
    "constructor",
    "prototype",
    "of",
    "async",
    "get",
    "set",
];

pub(super) fn ident(name: &str) -> String {
    let mut result: String = name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' {
                character
            } else {
                '_'
            }
        })
        .collect();
    if result.starts_with(|character: char| character.is_ascii_digit()) {
        result.insert(0, 'x');
    }
    if KEYWORDS.contains(&result.as_str()) {
        result.push('_');
    }
    result
}

pub(super) fn field_key(name: &str) -> String {
    ident(name)
}
