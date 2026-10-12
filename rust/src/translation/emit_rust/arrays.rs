//! JavaScript arrays, which the portable core never mutates, are Rust `Vec`s.

use super::{Expr, Node, Prelude, Result, RustEmitter};

// Generated from the JavaScript emitter; source adaptations are performed by
// its normal pipeline before a runtime string reaches either host adapter.
pub(super) const ARRAY_PRELUDE: &str = include_str!("../runtime/array-legacy.txt");

impl RustEmitter<'_> {
    /// An array literal, an append, an element read or a length.
    pub(super) fn array_expr(&mut self, e: &Expr) -> Result<String> {
        match &e.node {
            Node::Array { items } => {
                if items.is_empty() {
                    let element = e.ty.element().expect("an array literal has an array type");
                    return Ok(format!("Vec::<{}>::new()", self.ty(element)));
                }
                let mut texts = Vec::with_capacity(items.len());
                for item in items {
                    texts.push(self.expr(item)?);
                }
                Ok(format!("vec![{}]", texts.join(", ")))
            }
            Node::Append { left, right } => {
                self.preludes.insert(Prelude::Array);
                Ok(format!(
                    "crate::ml_array::append({}, {})",
                    self.expr(left)?,
                    self.expr(right)?
                ))
            }
            Node::Index { array, index } => {
                self.preludes.insert(Prelude::Array);
                self.state.array_read();
                let index = if index.ty.is_float() {
                    format!("crate::ml_array::number_index({})", self.expr(index)?)
                } else {
                    format!("{}.to_index()", self.receiver(index)?)
                };
                Ok(format!(
                    "crate::ml_array::at({}, {index})",
                    self.borrow(array)?
                ))
            }
            Node::Length { array } => {
                if array.ty == super::Type::String {
                    return Ok(
                        crate::translation::frontend_rules::render_string_length_expression(
                            "Rust",
                            &self.receiver(array)?,
                        ),
                    );
                }
                if e.ty.is_float() {
                    return Ok(format!("({}.len() as f64)", self.receiver(array)?));
                }
                self.preludes.insert(Prelude::Big);
                Ok(format!(
                    "crate::ml::Big::from_u128({}.len() as u128)",
                    self.receiver(array)?
                ))
            }
            _ => unreachable!("not an array expression"),
        }
    }
}
