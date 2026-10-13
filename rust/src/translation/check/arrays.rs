//! JavaScript arrays: literals, element reads and lengths. The portable core
//! never mutates an array.

use super::{
    Checker, Env, Expr, FLOAT, INT, Language, Node, Result, SArrayItem, SExpr, Span,
    TranslationError, Type, array, type_error, unsupported,
};
use crate::translation::primitive_value_rules::accept_tuple_index;
use std::collections::HashMap;

/// A tuple-typed result needs an owned value proof; unknown call/parameter origins refuse.
pub(super) fn immutable_product_value(value: &Expr, bindings: &HashMap<String, bool>) -> bool {
    if !matches!(value.ty, Type::Tuple { .. }) {
        return true;
    }
    match &value.node {
        Node::Tuple { items, frozen, .. } => {
            *frozen
                && items
                    .iter()
                    .all(|item| immutable_product_value(item, bindings))
        }
        Node::Let { name, value, body } => {
            let mut inner = bindings.clone();
            inner.insert(name.clone(), immutable_product_value(value, bindings));
            immutable_product_value(body, &inner)
        }
        Node::If {
            then, otherwise, ..
        } => {
            immutable_product_value(then, bindings) && immutable_product_value(otherwise, bindings)
        }
        Node::Var { name } => bindings.get(name) == Some(&true),
        _ => false,
    }
}

impl Checker {
    /// `[a, ...xs, b]` is the array of the runs of elements between the spread
    /// arrays, appended in order; `[...xs]` is `xs` itself, since no array is
    /// ever mutated.
    pub(super) fn array_literal(
        &mut self,
        items: &[SArrayItem],
        inferred: Option<&Type>,
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
    ) -> Result<Expr> {
        if let Some(ty @ Type::Tuple { elements }) = expected {
            if items.len() != elements.len() || items.iter().any(|item| item.spread) {
                return Err(type_error(
                    "tuple literal has the wrong number of elements",
                    span,
                ));
            }
            let values = items
                .iter()
                .zip(elements)
                .map(|(item, ty)| {
                    let value = self.expr(&item.value, env, path, Some(ty), false)?;
                    self.coerce(value, ty, item.value.span.or(span))
                })
                .collect::<Result<Vec<_>>>()?;
            return Ok(Expr::new(
                Node::Tuple {
                    items: values,
                    frozen: false,
                    product_model: None,
                },
                ty.clone(),
            ));
        }
        let mut element = expected.and_then(Type::element).cloned();
        // Inference fixes the element type of an array literal with no element of its own.
        if element.is_none()
            && let Some(inferred) = inferred
        {
            element = Some(self.resolve_type(Some(inferred), path, span)?);
        }
        let mut parts: Vec<Expr> = Vec::new();
        let mut run: Option<usize> = None;
        for item in items {
            let want = element.as_ref().map(|element| {
                if item.spread {
                    array(element.clone())
                } else {
                    element.clone()
                }
            });
            let value = self.expr(&item.value, env, path, want.as_ref(), false)?;
            let place = item.value.span.or(span);
            let mut ty = if item.spread {
                value.ty.element().cloned().ok_or_else(|| {
                    unsupported(
                        &format!("spread of {}", value.ty.key()),
                        "only arrays are spread into an array literal",
                        place,
                    )
                })?
            } else {
                value.ty.clone()
            };
            // Guarded parameters are naturals, and every BigInt an integer.
            if element.is_none() && self.language == Language::JavaScript && ty == Type::Nat {
                ty = INT;
            }
            let element = element.get_or_insert(ty).clone();
            let target = if item.spread {
                array(element.clone())
            } else {
                element.clone()
            };
            let value = self.coerce(value, &target, place)?;
            if item.spread {
                parts.push(value);
                run = None;
            } else if let Some(Node::Array { items }) = run
                .and_then(|index| parts.get_mut(index))
                .map(|part| &mut part.node)
            {
                items.push(value);
            } else {
                run = Some(parts.len());
                parts.push(Expr::new(
                    Node::Array { items: vec![value] },
                    array(element),
                ));
            }
        }
        let Some(element) = element else {
            return Err(unsupported(
                "empty array of an unknown type",
                "nothing fixes the type of the elements of this empty array; declare it with JSDoc",
                span,
            ));
        };
        let ty = array(element);
        let mut parts = parts.into_iter();
        let Some(first) = parts.next() else {
            return Ok(Expr::new(Node::Array { items: Vec::new() }, ty));
        };
        Ok(parts.fold(first, |left, right| {
            Expr::new(
                Node::Append {
                    left: Box::new(left),
                    right: Box::new(right),
                },
                ty.clone(),
            )
        }))
    }

    /// Check each positional item with its own expected type.
    pub(super) fn tuple_literal(
        &mut self,
        items: &[SExpr],
        frozen: bool,
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
    ) -> Result<Expr> {
        if frozen && items.is_empty() {
            return Err(unsupported(
                "empty immutable tuple product",
                "an empty positional product is distinct from a unit value and needs its own target encoding",
                span,
            ));
        }
        let expected_elements = match expected {
            Some(Type::Tuple { elements }) => {
                if items.len() != elements.len() {
                    return Err(type_error(
                        "tuple literal has the wrong number of elements",
                        span,
                    ));
                }
                Some(elements)
            }
            Some(ty) => {
                return Err(type_error(
                    format!("tuple literal where {} is expected", ty.key()),
                    span,
                ));
            }
            None => None,
        };
        let values = items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let want = expected_elements.map(|elements| &elements[index]);
                let value = self.expr(item, env, path, want, false)?;
                match want {
                    Some(ty) => self.coerce(value, ty, item.span.or(span)),
                    None => Ok(value),
                }
            })
            .collect::<Result<Vec<_>>>()?;
        let ty = expected.cloned().unwrap_or_else(|| Type::Tuple {
            elements: values.iter().map(|value| value.ty.clone()).collect(),
        });
        if frozen
            && (!ty.immutable_product_type()
                || values
                    .iter()
                    .any(|value| !immutable_product_value(value, &env.immutable_products)))
        {
            return Err(unsupported(
                "immutable tuple product",
                "every element needs the recursive scalar Copy value model; a mutable inner tuple cannot be frozen by its outer product",
                span,
            ));
        }
        Ok(Expr::new(
            Node::Tuple {
                items: values,
                frozen,
                product_model: None,
            },
            ty,
        ))
    }

    /// Check a constant product projection without array coercions.
    pub(super) fn tuple_index(
        &mut self,
        object: &SExpr,
        index: usize,
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        let object = self.expr(object, env, path, None, false)?;
        let Type::Tuple { elements } = &object.ty else {
            return Err(type_error("tuple projection requires a tuple value", span));
        };
        let position = serde_json::Number::from(index)
            .as_f64()
            .expect("a host tuple position has a finite binary64 projection");
        let length = serde_json::Number::from(elements.len())
            .as_f64()
            .expect("a host tuple cardinality has a finite binary64 projection");
        if !accept_tuple_index(position, length) {
            return Err(type_error("tuple index out of range", span));
        }
        let ty = elements[index].clone();
        Ok(Expr::new(
            Node::TupleIndex {
                object: Box::new(object),
                index,
            },
            ty,
        ))
    }

    /// `xs[i]`, whose index is a Number or a `BigInt`.
    pub(super) fn index(
        &mut self,
        object: &SExpr,
        index: &SExpr,
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        let array = self.expr(object, env, path, None, false)?;
        if let Type::Tuple { elements } = &array.ty
            && let super::SNode::Num {
                value, negative, ..
            } = &index.node
        {
            let length = serde_json::Number::from(elements.len())
                .as_f64()
                .expect("a host tuple cardinality has a finite binary64 projection");
            let index = value
                .parse::<f64>()
                .ok()
                .map(|value| if *negative { -value } else { value })
                .filter(|value| accept_tuple_index(*value, length))
                .and_then(crate::translation::frontend_rules::ml_array::number_index)
                .ok_or_else(|| type_error("tuple index out of range", span))?;
            let ty = elements[index].clone();
            return Ok(Expr::new(
                Node::TupleIndex {
                    object: Box::new(array),
                    index,
                },
                ty,
            ));
        }
        let Some(element) = array.ty.element().cloned() else {
            return Err(unsupported(
                &format!("indexing of {}", array.ty.key()),
                "only arrays are indexed in the portable core",
                span,
            ));
        };
        let place = index.span.or(span);
        let index = self.expr(index, env, path, None, false)?;
        if !index.ty.is_float() && !matches!(index.ty, Type::Int | Type::Nat) {
            return Err(type_error(
                format!(
                    "array index of type {}; an index is a Number or a BigInt",
                    index.ty.key()
                ),
                place,
            ));
        }
        Ok(Expr::new(
            Node::Index {
                array: Box::new(array),
                index: Box::new(index),
            },
            element,
        ))
    }

    /// The length of an array: a Number, or for the counter of a for…of loop an integer.
    pub(super) fn length(
        &mut self,
        object: &SExpr,
        integer: bool,
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        let array = self.expr(object, env, path, None, false)?;
        if array.ty.element().is_none() {
            return Err(type_error(format!("length of {}", array.ty.key()), span));
        }
        let ty = if integer { INT } else { FLOAT };
        Ok(Expr::new(
            Node::Length {
                array: Box::new(array),
            },
            ty,
        ))
    }

    /// Array sizes and UTF-16 string lengths; `None` for other fields.
    pub(super) fn length_field(
        &mut self,
        object: &SExpr,
        env: &Env,
        path: &[String],
    ) -> Result<Option<Expr>> {
        let array = self.expr(object, env, path, None, false)?;
        if array.ty.element().is_some() || array.ty == Type::String {
            return Ok(Some(Expr::new(
                Node::Length {
                    array: Box::new(array),
                },
                FLOAT,
            )));
        }
        Ok(None)
    }
}

/// Arrays have a text, but not one the translation reproduces yet.
pub(super) fn array_text(span: Option<Span>) -> TranslationError {
    unsupported(
        "text of an array",
        "console.log lays an array out with util.inspect and String joins its elements with commas, which the translation does not reproduce yet; print the elements one by one",
        span,
    )
}
