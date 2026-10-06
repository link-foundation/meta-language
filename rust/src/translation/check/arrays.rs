//! JavaScript arrays: literals, element reads and lengths. The portable core
//! never mutates an array.

use super::{
    Checker, Env, Expr, FLOAT, INT, Language, Node, Result, SArrayItem, SExpr, Span,
    TranslationError, Type, array, type_error, unsupported,
};

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

    /// `xs.length`, and the refusal of `s.length` of a string; `None` for other fields.
    pub(super) fn length_field(
        &mut self,
        object: &SExpr,
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Option<Expr>> {
        let array = self.expr(object, env, path, None, false)?;
        if array.ty.element().is_some() {
            return Ok(Some(Expr::new(
                Node::Length {
                    array: Box::new(array),
                },
                FLOAT,
            )));
        }
        if array.ty == Type::String {
            return Err(unsupported(
                "length of a string",
                "String.prototype.length counts UTF-16 code units, which the portable string types do not keep",
                span,
            ));
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
