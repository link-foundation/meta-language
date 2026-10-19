//! Object property recognition and explicit freeze admission.

use super::{
    JavaScriptParser, Result, SExpr, SNode, Span, TokenKind, describe, node, span, type_error,
    unsupported,
};
use crate::translation::primitive_value_rules::{
    accept_object_freeze_operand, accept_object_property_definition,
};
use crate::translation::surface::{ConstructorPrototypeDefinition, SRecordEntry};

fn literal_array_value(value: &SExpr) -> bool {
    match &value.node {
        SNode::Num { .. } | SNode::Bool { .. } | SNode::Str { .. } => true,
        SNode::Array { items, .. } => items
            .iter()
            .all(|item| !item.spread && literal_array_value(&item.value)),
        _ => false,
    }
}

impl JavaScriptParser {
    /// `{ $: 'node', left, value: v }` constructs the `node` alternative of a data type.
    pub(super) fn object_literal(&mut self) -> Result<SExpr> {
        let start = self.cursor.advance();
        let mut tag = None;
        let mut fields: Vec<(String, SExpr)> = Vec::new();
        let mut entries = Vec::new();
        let mut duplicate = None;
        let mut prototype_definition = None;
        let mut property_index = 0;
        while !self.cursor.is("}") {
            let index = property_index;
            property_index += 1;
            let mut key = self.peek();
            let mut here = Some(span(&key, &key));
            if self.cursor.eat("...").is_some() {
                entries.push(SRecordEntry {
                    name: None,
                    spread: true,
                    value: self.expr()?,
                });
                if self.cursor.eat(",").is_none() {
                    break;
                }
                continue;
            }
            let computed = self.cursor.eat("[").is_some();
            if computed {
                let value = self.expr()?;
                let close = self.cursor.expect("]", Some("computed object key"))?;
                let SNode::Str { value } = value.node else {
                    return Err(unsupported(
                        "computed object key",
                        "the portable record shape needs a literal string key",
                        Some(span(&key, &close)),
                    ));
                };
                key.kind = TokenKind::String;
                key.value = value;
                key.end = close.end;
                here = Some(span(&key, &key));
            } else {
                if !matches!(key.kind, TokenKind::Identifier | TokenKind::String) {
                    return Err(unsupported(
                        &format!("object key {}", describe(&key)),
                        "keys are literal names",
                        here,
                    ));
                }
                self.cursor.advance();
            }
            if !accept_object_property_definition(&key.value, computed, self.cursor.is(":")) {
                if prototype_definition.is_some() {
                    return Err(Self::fail("duplicate object prototype definition", &key));
                }
                self.cursor
                    .expect(":", Some("object prototype definition"))?;
                let value = self.expr().map_err(|error| {
                    if error.kind.as_str() == "unsupported" {
                        unsupported("object prototype definition",
                            "an uncomputed __proto__: entry is prototype-definition syntax rather than an own data property; its source expression is outside the native surface model", here)
                    } else { error }
                })?;
                prototype_definition = Some(ConstructorPrototypeDefinition {
                    value: Box::new(value),
                    span: span(&key, &key),
                    index,
                });
            } else if super::surface_rules::read_java_script_object_property_form(
                &key.value,
                self.cursor.is("("),
                self.cursor.is(":"),
                !computed && key.kind == TokenKind::Identifier,
            ) == "tag"
            {
                self.cursor.expect(":", Some("object"))?;
                let value = self.cursor.advance();
                if value.kind != TokenKind::String {
                    return Err(unsupported(
                        "computed tag",
                        "the $ tag is a string literal",
                        Some(span(&value, &value)),
                    ));
                }
                tag = Some(value.value);
            } else {
                if duplicate.is_none() && fields.iter().any(|(name, _)| *name == key.value) {
                    duplicate = Some(key.clone());
                }
                let form = super::surface_rules::read_java_script_object_property_form(
                    &key.value,
                    self.cursor.is("("),
                    self.cursor.is(":"),
                    !computed && key.kind == TokenKind::Identifier,
                );
                let value = if form == "method" {
                    self.read_closure(true)?
                } else if form == "field" {
                    self.cursor.advance();
                    self.expr()?
                } else {
                    if form == "missing-colon" {
                        return Err(Self::fail(
                            "expected : after the object property name",
                            &key,
                        ));
                    }
                    self.reference(&key)?
                };
                entries.push(SRecordEntry {
                    name: Some(key.value.clone()),
                    spread: false,
                    value: value.clone(),
                });
                if let Some((_, current)) = fields.iter_mut().find(|(name, _)| *name == key.value) {
                    *current = value;
                } else {
                    fields.push((key.value, value));
                }
            }
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect("}", Some("object"))?;
        let form = super::surface_rules::read_java_script_object_value_form(
            tag.is_some(),
            duplicate.is_some(),
            entries.iter().any(|entry| entry.spread),
        );
        if form == "record" {
            return Ok(node(
                SNode::Record {
                    entries,
                    record_projection: None,
                    ty: None,
                    prototype_definition,
                },
                self.to_here(&start),
            ));
        }
        if form == "duplicate" {
            let key = duplicate.expect("shared duplicate selection");
            return Err(type_error(
                format!("duplicate field {}", key.value),
                Some(span(&key, &key)),
            ));
        }
        if form == "spread" {
            return Err(unsupported(
                "tagged object spread",
                "constructor spreads require a complete tagged field projection",
                Some(self.to_here(&start)),
            ));
        }
        let tag = tag.expect("shared tagged constructor selection");
        Ok(node(
            SNode::CtorObject {
                tag,
                fields,
                frozen: false,
                prototype_definition,
            },
            self.to_here(&start),
        ))
    }

    pub(super) fn freeze_object_argument(&self, mut arg: SExpr, called: Span) -> Result<SExpr> {
        if self.function_names.contains("Object")
            || self
                .externals
                .iter()
                .any(|external| external.name() == "Object")
        {
            return Err(unsupported(
                "shadowed Object.freeze",
                "immutable products require the actual unshadowed Object.freeze built-in",
                Some(called),
            ));
        }
        let expression = match &arg.node {
            SNode::CtorObject { .. } => "ctorObject",
            SNode::Array { .. } => "array",
            _ => "other",
        };
        let readonly_literal = self.readonly_constants.initializer
            && expression == "array"
            && literal_array_value(&arg);
        if self.readonly_constants.initializer && expression == "array" && !readonly_literal {
            return Err(unsupported(
                "Object.freeze operand",
                "readonly module constants require literal values; evaluated elements need a separate module initialization model",
                Some(called),
            ));
        }
        if !readonly_literal && let SNode::Array { items, .. } = arg.node {
            if items.is_empty()
                || items
                    .iter()
                    .any(|item| item.spread || matches!(item.value.node, SNode::Array { .. }))
            {
                return Err(unsupported(
                    "immutable tuple product",
                    "a fresh nonempty dense array needs scalar Copy elements or separately frozen nested products",
                    Some(called),
                ));
            }
            return Ok(node(
                SNode::Tuple {
                    items: items.into_iter().map(|item| item.value).collect(),
                    frozen: true,
                    product_model: Some("immutable value product".to_owned()),
                },
                called,
            ));
        }
        if !accept_object_freeze_operand(expression, readonly_literal) {
            return Err(unsupported(
                "Object.freeze operand",
                "freezing this value needs an explicit property-descriptor operation; only direct tagged objects and readonly literal module constants are represented",
                Some(called),
            ));
        }
        if let SNode::CtorObject { frozen, .. } = &mut arg.node {
            *frozen = true;
        }
        arg.span = Some(called);
        Ok(arg)
    }
}
