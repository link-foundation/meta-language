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

// Bound expansion of the constructor body to the actual finite source choices.
fn accept_constructor_tag_expression(expression: &SExpr) -> bool {
    let mut pending = vec![expression];
    let mut alternatives = 0.0;
    while let Some(value) = pending.pop() {
        match &value.node {
            SNode::If {
                then, otherwise, ..
            } => {
                pending.push(otherwise);
                pending.push(then);
            }
            SNode::Str { .. }
                if alternatives
                    < super::surface_rules::read_java_script_constructor_tag_alternative_limit(
                    ) =>
            {
                alternatives += 1.0;
            }
            _ => return false,
        }
    }
    true
}

fn expand_constructor_tag(tag: SExpr, fields: &[(String, SExpr)], place: Span) -> SExpr {
    let expression = match tag.node {
        SNode::Str { value } => SNode::CtorObject {
            tag: value,
            fields: fields.to_vec(),
            frozen: false,
            prototype_definition: None,
        },
        SNode::If {
            cond,
            then,
            otherwise,
        } => SNode::If {
            cond,
            then: Box::new(expand_constructor_tag(*then, fields, place)),
            otherwise: Box::new(expand_constructor_tag(*otherwise, fields, place)),
        },
        _ => unreachable!("constructor tag alternatives were admitted before expansion"),
    };
    node(expression, place)
}

impl JavaScriptParser {
    /// `{ $: 'node', left, value: v }` constructs the `node` alternative of a data type.
    pub(super) fn object_literal(&mut self) -> Result<SExpr> {
        let start = self.cursor.advance();
        let mut tag: Option<SExpr> = None;
        let mut tag_field_count = 0;
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
                self.require_scalar_string_token(&key, "record property key")?;
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
                let value = self.expr()?;
                if !accept_constructor_tag_expression(&value)
                    || tag.as_ref().is_some_and(|prior| {
                        !matches!(prior.node, SNode::Str { .. })
                            || !matches!(value.node, SNode::Str { .. })
                    })
                {
                    return Err(unsupported(
                        "computed tag",
                        "a constructor tag needs literal strings in a finite conditional expression",
                        value.span,
                    ));
                }
                tag = Some(value);
                tag_field_count = entries.len();
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
                    property_order: None,
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
        let tag = tag.expect("shared tagged constructor selection");
        let place = self.to_here(&start);
        if form == "spread" {
            if let Some(prototype) = prototype_definition {
                return Err(unsupported(
                    "object prototype definition",
                    "the portable core does not represent object prototypes",
                    Some(prototype.span),
                ));
            }
            return super::object_constructor_spreads::expand_constructor_spreads(
                tag,
                entries,
                || {
                    self.generated_count += 1;
                    format!(
                        "{}temporary_constructor_field{}",
                        self.generated_prefix, self.generated_count
                    )
                },
                place,
            );
        }
        if let SNode::Str { value } = tag.node {
            return Ok(node(
                SNode::CtorObject {
                    tag: value,
                    fields,
                    frozen: false,
                    prototype_definition,
                },
                place,
            ));
        }
        if let Some(prototype) = prototype_definition {
            return Err(unsupported(
                "object prototype definition",
                "an uncomputed __proto__: entry is prototype-definition syntax rather than an own data property; the portable core does not represent object prototypes",
                Some(prototype.span),
            ));
        }
        let mut bindings = Vec::new();
        for (_, value) in fields.iter_mut().take(tag_field_count) {
            self.generated_count += 1;
            let binding = format!(
                "{}temporary_constructor_field{}",
                self.generated_prefix, self.generated_count
            );
            let reference = SExpr::new(
                SNode::Name {
                    path: vec![binding.clone()],
                },
                value.span,
            );
            bindings.push((binding, std::mem::replace(value, reference)));
        }
        let mut result = expand_constructor_tag(tag, &fields, place);
        for (name, value) in bindings.into_iter().rev() {
            result = node(
                SNode::Let {
                    name,
                    ty: None,
                    value: Box::new(value),
                    body: Box::new(result),
                },
                place,
            );
        }
        Ok(result)
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
