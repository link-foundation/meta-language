//! Object property recognition and explicit freeze admission.

use super::{
    JavaScriptParser, Result, SExpr, SNode, Span, TokenKind, describe, node, span, type_error,
    unsupported,
};
use crate::translation::primitive_value_rules::{
    accept_object_freeze_operand, accept_object_property_definition,
};
use crate::translation::surface::ConstructorPrototypeDefinition;

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
        let mut prototype_definition = None;
        let mut property_index = 0;
        while !self.cursor.is("}") {
            let index = property_index;
            property_index += 1;
            let mut key = self.peek();
            let mut here = Some(span(&key, &key));
            if self.cursor.is("...") {
                return Err(unsupported("object spread", "name every field", here));
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
            if !accept_object_property_definition(key.value.clone(), computed, self.cursor.is(":"))
            {
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
            } else if key.value == "$" {
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
                if fields.iter().any(|(name, _)| *name == key.value) {
                    return Err(type_error(format!("duplicate field {}", key.value), here));
                }
                if self.cursor.is("(") {
                    return Err(unsupported(
                        "method in an object value",
                        "functions as values are outside the portable core",
                        here,
                    ));
                }
                let value = if self.cursor.eat(":").is_some() {
                    self.expr()?
                } else {
                    if computed || key.kind != TokenKind::Identifier {
                        return Err(Self::fail(
                            "expected : after the object property name",
                            &key,
                        ));
                    }
                    self.reference(&key)?
                };
                fields.push((key.value, value));
            }
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect("}", Some("object"))?;
        let Some(tag) = tag else {
            if let Some(prototype) = prototype_definition {
                return Err(unsupported(
                    "object prototype definition",
                    "an uncomputed __proto__: entry is prototype-definition syntax rather than an own data property; the native surface model does not yet represent untagged records",
                    Some(prototype.span),
                ));
            }
            return Err(unsupported(
                "object without a $ tag",
                "a portable object is an alternative of a @typedef data type",
                Some(self.to_here(&start)),
            ));
        };
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
        let expression = match &arg.node {
            SNode::CtorObject { .. } => "ctorObject",
            SNode::Array { .. } => "array",
            _ => "other",
        };
        let readonly_literal = self.readonly_constant_initializer
            && expression == "array"
            && literal_array_value(&arg);
        if !accept_object_freeze_operand(expression.to_owned(), readonly_literal) {
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
