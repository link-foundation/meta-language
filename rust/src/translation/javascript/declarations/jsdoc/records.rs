//! Construct actual record types from shared UTF16 structural decisions.

use super::syntax::{split_type, split_type_unit};
use super::{HashMap, HashSet, Result, Span, Type, js_trim, read_jsdoc_type, unsupported};
use crate::translation::Language;
use crate::translation::lexer::{TokenKind, tokenize};
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
        let field = if optional {
            js_trim(&field[..field.len() - 1])
        } else {
            field
        };
        let name = if field.starts_with(['\'', '"']) {
            let tokens = tokenize(field, Language::JavaScript)?;
            let token = &tokens.tokens[0];
            if token.kind != TokenKind::String || token.raw != field || token.string_units.is_some()
            {
                return Err(unsupported(
                    "quoted record property name",
                    "a field name needs one scalar JavaScript String literal",
                    Some(range),
                ));
            }
            token.value.clone()
        } else {
            field.to_owned()
        };
        let ty = read_jsdoc_type(&source_type, range, aliases, expanding)?;
        fields.push(TypeField {
            optional,
            name,
            ty: if optional {
                crate::translation::javascript_constructor_presence::field_type(ty)
            } else {
                ty
            },
        });
    }
    Ok(Some(Type::Record {
        source_class_owner: None,
        fields,
        open: false,
        reference: false,
        ordered: false,
    }))
}
