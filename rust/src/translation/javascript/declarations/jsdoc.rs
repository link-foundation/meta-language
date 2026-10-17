//! Existing `JSDoc` type readers, separated to bound declaration source size.
use std::collections::{HashMap, HashSet};

use super::{
    BOOL, FLOAT, INT, ROOT, Result, STRING, Span, Type, array, is_identifier_name, js_trim,
    unsupported,
};

mod callables;
mod records;
mod syntax;

/// `T` of an async function's `@returns {Promise<T>}`.
pub(super) fn promised(text: &str) -> Option<&str> {
    js_trim(text)
        .strip_prefix("Promise")
        .map(js_trim)?
        .strip_prefix('<')?
        .strip_suffix('>')
}

/// The portable type a `JSDoc` type names.
pub(super) fn jsdoc_type(
    text: &str,
    range: Span,
    aliases: &HashMap<String, String>,
) -> Result<Type> {
    read_jsdoc_type(text, range, aliases, &mut HashSet::new())
}

pub(super) fn read_jsdoc_type(
    text: &str,
    range: Span,
    aliases: &HashMap<String, String>,
    expanding: &mut HashSet<String>,
) -> Result<Type> {
    let name = js_trim(text);
    if let Some(callable) = callables::read_callable_type(name, range, aliases, expanding)? {
        return Ok(callable);
    }
    let alternatives = syntax::split_type(name, u16::from(b'|'), range)?;
    let first = name.encode_utf16().next().map_or(f64::NAN, f64::from);
    let nullable = crate::translation::nullable_type_syntax_rules::read_nullable_type_syntax(
        &alternatives,
        first,
    );
    let [action, position, absence] = nullable.as_slice() else {
        unreachable!("the shared nullable source type plan contains three cells")
    };
    let action = crate::translation::frontend_rules::ml_array::number_index(*action)
        .expect("the shared nullable action selects its declared constructor");
    let absence =
        crate::translation::nullable_type_syntax_rules::read_nullable_type_absence(*absence);
    match action {
        1 => {
            let position = crate::translation::frontend_rules::ml_array::number_index(*position)
                .expect("the shared nullable value position is an actual alternative");
            return Ok(Type::Option {
                element: Box::new(read_jsdoc_type(
                    &alternatives[position],
                    range,
                    aliases,
                    expanding,
                )?),
                absence,
            });
        }
        2 => {
            return Ok(Type::Union {
                alternatives: alternatives
                    .iter()
                    .map(|part| read_jsdoc_type(part, range, aliases, expanding))
                    .collect::<Result<Vec<_>>>()?,
            });
        }
        3 => return Ok(Type::Absence { absence }),
        4 => {
            return Ok(Type::Option {
                element: Box::new(read_jsdoc_type(&name[1..], range, aliases, expanding)?),
                absence,
            });
        }
        0 => {}
        _ => unreachable!("the shared nullable constructor is declared"),
    }
    match name {
        "bigint" => return Ok(INT),
        "boolean" => return Ok(BOOL),
        "string" => return Ok(STRING),
        "number" => return Ok(FLOAT),
        _ => {}
    }
    if let Some(element) = name.strip_suffix("[]") {
        return Ok(array(read_jsdoc_type(element, range, aliases, expanding)?));
    }
    if let Some(inner) = name
        .strip_prefix('[')
        .and_then(|text| text.strip_suffix(']'))
    {
        let mut elements = Vec::new();
        for field in syntax::split_type(inner, u16::from(b','), range)? {
            if !field.is_empty() {
                elements.push(read_jsdoc_type(&field, range, aliases, expanding)?);
            }
        }
        return Ok(Type::Tuple { elements });
    }
    if let Some(record) = records::read_record_type(name, range, aliases, expanding)? {
        return Ok(record);
    }
    let collection = name
        .strip_prefix("Set<")
        .map(|inner| ("set", inner))
        .or_else(|| name.strip_prefix("Map<").map(|inner| ("map", inner)))
        .and_then(|(kind, inner)| {
            inner
                .strip_suffix('>')
                .filter(|inner| !inner.is_empty())
                .map(|inner| (kind, inner))
        });
    if let Some((kind, inner)) = collection {
        let arguments = syntax::split_type(inner, u16::from(b','), range)?;
        return match (kind, arguments.as_slice()) {
            ("set", [element]) => Ok(Type::Set {
                element: Box::new(read_jsdoc_type(element, range, aliases, expanding)?),
                reference: false,
            }),
            ("map", [key, value]) => Ok(Type::Map {
                key: Box::new(read_jsdoc_type(key, range, aliases, expanding)?),
                value: Box::new(read_jsdoc_type(value, range, aliases, expanding)?),
                reference: false,
            }),
            _ => Err(unsupported(
                &format!("JSDoc type {{{name}}}"),
                "Set has one type argument and Map has two",
                Some(range),
            )),
        };
    }
    let generic = name
        .strip_prefix("Array<")
        .or_else(|| name.strip_prefix("ReadonlyArray<"))
        .and_then(|rest| rest.strip_suffix('>'))
        .filter(|element| !element.is_empty());
    if let Some(element) = generic {
        return Ok(array(read_jsdoc_type(element, range, aliases, expanding)?));
    }
    if let Some(expanded) = aliases.get(name)
        && expanding.insert(name.to_owned())
    {
        let result = read_jsdoc_type(expanded, range, aliases, expanding);
        expanding.remove(name);
        return result;
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
