//! JavaScript arrays, which the portable core never mutates, are Lean `Array`s.

use std::collections::HashSet;

use super::{Expr, LeanEmitter, Node, Result, Type};

impl LeanEmitter<'_> {
    /// An array literal, an append, an element read or a length.
    pub(super) fn array_expr(&mut self, e: &Expr, depth: usize) -> Result<String> {
        Ok(match &e.node {
            Node::Array { items } => {
                let mut texts = Vec::with_capacity(items.len());
                for item in items {
                    texts.push(self.expr(item, depth)?);
                }
                format!("(#[{}] : {})", texts.join(", "), self.ty(&e.ty)?)
            }
            Node::Append { left, right } => {
                format!(
                    "({} ++ {})",
                    self.expr(left, depth)?,
                    self.expr(right, depth)?
                )
            }
            Node::Index { array, index } => {
                self.state.array_read();
                self.state.abort_to_total("array index out of range");
                self.helpers.insert("arrayAt");
                let values = self.expr(array, depth)?;
                let text = self.expr(index, depth)?;
                match index.ty {
                    Type::Float => {
                        self.helpers.insert("arrayAtFloat");
                        format!("(ml_array_at_float {values} {text})")
                    }
                    Type::Nat => format!("(ml_array_at {values} (Int.ofNat {text}))"),
                    _ => format!("(ml_array_at {values} {text})"),
                }
            }
            Node::Length { array } => {
                let of = if e.ty.is_float() {
                    "Float.ofNat"
                } else {
                    "Int.ofNat"
                };
                format!("({of} {}.size)", self.expr(array, depth)?)
            }
            _ => unreachable!("not an array expression"),
        })
    }

    /// Lean derives no `DecidableEq` for a Float, nor through an Array of data.
    pub(super) fn undecidable(&self, name: &str, seen: &mut HashSet<String>) -> bool {
        if !seen.insert(name.to_owned()) {
            return false;
        }
        self.program.data(name).ctors.iter().any(|ctor| {
            ctor.fields
                .iter()
                .any(|field| self.undecidable_type(&field.ty, false, seen))
        })
    }

    fn undecidable_type(&self, ty: &Type, nested: bool, seen: &mut HashSet<String>) -> bool {
        match ty {
            Type::Float => true,
            Type::Array { element } => self.undecidable_type(element, true, seen),
            Type::Data { name } => nested || self.undecidable(name, seen),
            _ => false,
        }
    }
}
