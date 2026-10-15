//! Existing `JSDoc` type readers, separated to bound declaration source size.
use super::{
    BOOL, FLOAT, INT, ROOT, Result, STRING, Span, Type, array, is_identifier_name, js_trim,
    unsupported,
};

/// `T` of an async function's `@returns {Promise<T>}`.
pub(super) fn promised(text: &str) -> Option<&str> {
    js_trim(text)
        .strip_prefix("Promise")
        .map(js_trim)?
        .strip_prefix('<')?
        .strip_suffix('>')
}

/// The portable type a `JSDoc` type names.
pub(super) fn jsdoc_type(text: &str, range: Span) -> Result<Type> {
    let name = js_trim(text);
    match name {
        "bigint" => return Ok(INT),
        "boolean" => return Ok(BOOL),
        "string" => return Ok(STRING),
        "number" => return Ok(FLOAT),
        _ => {}
    }
    if let Some(element) = name.strip_suffix("[]") {
        return Ok(array(jsdoc_type(element, range)?));
    }
    if let Some(inner) = name
        .strip_prefix('[')
        .and_then(|text| text.strip_suffix(']'))
    {
        let mut depth = 0_i64;
        let mut start = 0;
        let mut elements = Vec::new();
        for (index, character) in inner.char_indices() {
            match character {
                '[' | '(' | '{' | '<' => depth += 1,
                ']' | ')' | '}' | '>' => depth -= 1,
                ',' if depth == 0 => {
                    let field = js_trim(&inner[start..index]);
                    if !field.is_empty() {
                        elements.push(jsdoc_type(field, range)?);
                    }
                    start = index + 1;
                }
                _ => {}
            }
        }
        let field = js_trim(&inner[start..]);
        if !field.is_empty() {
            elements.push(jsdoc_type(field, range)?);
        }
        return Ok(Type::Tuple { elements });
    }
    let generic = name
        .strip_prefix("Array<")
        .or_else(|| name.strip_prefix("ReadonlyArray<"))
        .and_then(|rest| rest.strip_suffix('>'))
        .filter(|element| !element.is_empty());
    if let Some(element) = generic {
        return Ok(array(jsdoc_type(element, range)?));
    }
    let reserved = [
        "object",
        "any",
        "unknown",
        "void",
        "undefined",
        "null",
        "Object",
        "Function",
        "Array",
        "symbol",
    ];
    if !is_identifier_name(name) || reserved.contains(&name) {
        return Err(unsupported(
            &format!("JSDoc type {{{name}}}"),
            "portable types are number, bigint, boolean, string, arrays T[] of them and @typedef data types",
            Some(range),
        ));
    }
    Ok(Type::Named {
        path: vec![ROOT.to_owned(), name.to_owned()],
        span: Some(range),
    })
}
