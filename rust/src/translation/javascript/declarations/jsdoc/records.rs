//! Construct actual record types from shared UTF16 structural decisions.

use super::syntax::{split_type, split_type_unit};
use super::{HashMap, HashSet, Result, Span, Type, js_trim, read_jsdoc_type, unsupported};
use crate::translation::record_type_syntax_rules::{
    accept_record_type_boundary, accept_record_type_optional_name, read_record_type_field_colon,
};
use crate::translation::types::TypeField;

pub(super) fn read_record_type(
    name: &str,
    range: Span,
    aliases: &HashMap<String, String>,
    expanding: &mut HashSet<String>,
) -> Result<Option<Type>> {
    let first = name.encode_utf16().next().map_or(f64::NAN, f64::from);
    let last = name.encode_utf16().last().map_or(f64::NAN, f64::from);
    if !accept_record_type_boundary(first, last) {
        return Ok(None);
    }
    // The selected delimiters are actual ASCII braces, hence byte boundaries.
    let mut fields = Vec::new();
    for part in split_type(&name[1..name.len() - 1], u16::from(b','), range)? {
        if part.is_empty() {
            continue;
        }
        let numeric = part.encode_utf16().map(f64::from).collect::<Vec<_>>();
        let colon = read_record_type_field_colon(&numeric);
        if colon < 0.0 {
            return Err(unsupported(
                &format!("record type {{{name}}}"),
                "every field has a name and a type",
                Some(range),
            ));
        }
        let (field, source_type) = split_type_unit(&part, colon, range)?;
        let field = js_trim(&field);
        let last = field.encode_utf16().last().map_or(f64::NAN, f64::from);
        let optional = accept_record_type_optional_name(last);
        let ty = read_jsdoc_type(&source_type, range, aliases, expanding)?;
        fields.push(TypeField {
            name: if optional {
                field[..field.len() - 1].to_owned()
            } else {
                field.to_owned()
            },
            ty: if optional {
                Type::Option {
                    element: Box::new(ty),
                    absence: "undefined".to_owned(),
                }
            } else {
                ty
            },
        });
    }
    Ok(Some(Type::Record {
        fields,
        open: false,
        reference: false,
        ordered: false,
    }))
}
