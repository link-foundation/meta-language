//! Presence-preserving transport for the two new source-only binding forms.
//! Ordinary expressions retain the original surface serialization shape.
use serde::{Deserialize, Deserializer};

use super::{SExpr, SNode, STagTest, Span, SurfaceRawMetadata, raw_metadata};
use crate::translation::array_binding_rules::{self as rules, ml_string::Text};
use crate::translation::diagnostics::{Result as TranslationResult, type_error};

#[derive(Default)]
enum TagTestPresence {
    #[default]
    Missing,
    Present(Option<Box<STagTest>>),
}

#[derive(Deserialize)]
pub(super) struct SurfaceExpression {
    #[serde(flatten)]
    node: SNode,
    #[serde(default)]
    span: Option<Span>,
    #[serde(
        default,
        deserialize_with = "raw_metadata::read_present_surface_raw_metadata"
    )]
    parenthesized: Option<SurfaceRawMetadata>,
    #[serde(default, deserialize_with = "present_block")]
    block: Option<bool>,
    #[serde(rename = "tagTest", default, deserialize_with = "present_tag_test")]
    tag_test: TagTestPresence,
}

fn present_block<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<bool>, D::Error> {
    bool::deserialize(deserializer).map(Some)
}

fn present_tag_test<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<TagTestPresence, D::Error> {
    Option::<Box<STagTest>>::deserialize(deserializer).map(TagTestPresence::Present)
}

impl TryFrom<SurfaceExpression> for SExpr {
    type Error = String;

    fn try_from(raw: SurfaceExpression) -> Result<Self, Self::Error> {
        if let SNode::ReferenceIdentity { extra, .. } = &raw.node
            && (!extra.is_empty()
                || raw.parenthesized.is_some()
                || raw.block.is_some()
                || matches!(raw.tag_test, TagTestPresence::Present(_)))
        {
            return Err("explicit reference identity needs its closed owned operands".to_owned());
        }
        if let SNode::ReferencePayload { extra, .. } = &raw.node
            && (!extra.is_empty()
                || raw.parenthesized.is_some()
                || raw.block.is_some()
                || matches!(raw.tag_test, TagTestPresence::Present(_)))
        {
            return Err("a reference payload needs its closed owned expression".to_owned());
        }
        if matches!(
            raw.node,
            SNode::ArrayBindingRest { .. } | SNode::ArrayBindingDefault { .. }
        ) && (raw.block.is_some() || matches!(raw.tag_test, TagTestPresence::Present(_)))
        {
            return Err("array binding contains an unrepresented field".to_owned());
        }
        Ok(Self {
            node: raw.node,
            span: raw.span,
            parenthesized: raw.parenthesized,
            block: raw.block.unwrap_or(false),
            tag_test: match raw.tag_test {
                TagTestPresence::Missing => None,
                TagTestPresence::Present(value) => value,
            },
        })
    }
}

pub fn validate(node: &SExpr) -> TranslationResult<()> {
    let (SNode::ArrayBindingRest { extra, .. } | SNode::ArrayBindingDefault { extra, .. }) =
        &node.node
    else {
        return Err(type_error(
            "array binding needs its actual owned constructor",
            node.span,
        ));
    };
    for field in extra.keys() {
        if !rules::accept_java_script_array_binding_field(
            &Text::from_text(node.node.kind()),
            &Text::from_text(field),
        ) {
            return Err(type_error(
                "array binding contains an unrepresented field",
                node.span,
            ));
        }
    }
    // The remaining map contains only unconsumed fields. A direct typed host
    // value cannot use it to duplicate an already owned operand role either.
    if !extra.is_empty() {
        return Err(type_error(
            "array binding contains an unrepresented field",
            node.span,
        ));
    }
    if node.block || node.tag_test.is_some() {
        return Err(type_error(
            "array binding contains an unrepresented field",
            node.span,
        ));
    }
    if node
        .parenthesized
        .as_ref()
        .is_some_and(|value| value.as_boolean().is_none())
    {
        return Err(type_error(
            "array binding parentheses need an actual Boolean",
            node.span,
        ));
    }
    match &node.node {
        SNode::ArrayBindingRest { start, .. } => {
            offset(start, node.span)?;
        }
        SNode::ArrayBindingDefault { value, .. } if !matches!(value.node, SNode::Index { .. }) => {
            return Err(type_error(
                "array binding default needs its actual indexed element read",
                node.span,
            ));
        }
        _ => {}
    }
    Ok(())
}

pub fn offset(start: &serde_json::Value, span: Option<Span>) -> TranslationResult<SExpr> {
    let failure = || {
        type_error(
            "array rest start needs its actual nonnegative safe integer literal",
            span,
        )
    };
    let record = start.as_object().ok_or_else(failure)?;
    if record
        .keys()
        .any(|field| !rules::accept_java_script_array_binding_offset_field(&Text::from_text(field)))
    {
        return Err(type_error(
            "array rest start contains an unrepresented field",
            span,
        ));
    }
    let text = record
        .get("value")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(failure)?;
    let negative = match record.get("negative") {
        None => false,
        Some(serde_json::Value::Bool(value)) => *value,
        _ => return Err(failure()),
    };
    let literal = record.get("k").and_then(serde_json::Value::as_str) == Some("num");
    let trimmed = text.trim_matches(|character| {
        u16::try_from(u32::from(character)).is_ok_and(crate::translation::lexer::is_js_space)
    });
    let parsed = if trimmed.is_empty() {
        Some(0.0)
    } else {
        trimmed.parse::<f64>().ok().or_else(|| {
            crate::translation::js_number::js_number_literal(trimmed)
                .and_then(|value| value.parse::<f64>().ok())
        })
    };
    let value = parsed.unwrap_or(f64::NAN) * if negative { -1.0 } else { 1.0 };
    let safe_integer =
        value.is_finite() && value.fract() == 0.0 && value.abs() <= 9_007_199_254_740_991.0;
    if !rules::accept_java_script_array_binding_offset(literal, safe_integer, value < 0.0) {
        return Err(failure());
    }
    Ok(SExpr::new(
        SNode::Num {
            value: if value == 0.0 && value.is_sign_negative() {
                "-0".to_owned()
            } else {
                crate::translation::js_number::js_number(value)
            },
            ty: Some(crate::translation::types::FLOAT),
            negative: false,
            unit: false,
        },
        span,
    ))
}
