//! Actual source item/result facts for shared standard numeric reductions.
use super::{Result, RustParser, SourceOrigin, Token, Type, unsupported};
use crate::translation::rust_iterator_arithmetic_rules as rules;
pub(super) fn read(
    parser: &mut RustParser,
    item: &SourceOrigin,
    path: &[String],
    consumer: &Token,
) -> Result<(SourceOrigin, Token)> {
    let owner = item
        .expanded_source_owner
        .as_ref()
        .or(item.source_owner.as_ref());
    let ty = item.ty.as_ref();
    let (bits, signed) = match ty {
        Some(Type::Fixed { bits, signed }) => (bits.to_string(), *signed),
        _ => (String::new(), false),
    };
    let known_owner=owner.is_some_and(|owner|owner.kind=="primitive"&&owner.arguments.is_empty()&&crate::translation::rust_array_lookup_rules::read_rust_array_lookup_element_owner_error(ty.map_or("",Type::kind),&bits,signed,&owner.path.join("::")).is_empty());
    let mut result = item.clone();
    result.wrappers.clear();
    result.source_lifetimes.clear();
    if parser.cursor.eat("::").is_some() {
        parser
            .cursor
            .expect("<", Some("iterator arithmetic result type"))?;
        let previous = parser
            .iterator_item_type_inference_origin
            .replace(result.clone());
        let parsed = parser.parse_type();
        result = parser.origins.current_type.clone();
        parser.iterator_item_type_inference_origin = previous;
        parsed?;
        parser.split_type_closing_token();
        parser
            .cursor
            .expect(">", Some("iterator arithmetic result type"))?;
    }
    parser.cursor.expect("(", Some("iterator arithmetic"))?;
    let close = parser.cursor.expect(")", Some("iterator arithmetic"))?;
    let result_owner = result
        .expanded_source_owner
        .as_ref()
        .or(result.source_owner.as_ref());
    let agrees = if let (Some(owner), Some(actual)) = (owner, result_owner) {
        owner.kind == actual.kind && owner.path == actual.path && actual.arguments.is_empty()
    } else {
        false
    };
    let error = rules::read_rust_iterator_arithmetic_error(
        ty.map_or("", Type::kind),
        item.proven,
        known_owner,
        item.wrappers.len() as f64,
        result.proven,
        result.wrappers.len() as f64,
        item.ty == result.ty,
        agrees,
    );
    if !error.is_empty() {
        return Err(unsupported(
            "Rust iterator arithmetic",
            &error,
            Some(super::Span::new(consumer.start, close.end)),
        ));
    }
    Ok((result, close))
}
