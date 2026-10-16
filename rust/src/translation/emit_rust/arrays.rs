//! JavaScript arrays, which the portable core never mutates, are Rust `Vec`s.

use super::{Expr, Node, Prelude, Result, RustEmitter};

// Generated from the JavaScript emitter; source adaptations are performed by
// its normal pipeline before a runtime string reaches either host adapter.
pub(super) const ARRAY_PRELUDE: &str = include_str!("../runtime/array-legacy.txt");

pub(super) const fn read_array_prelude(standard: bool) -> &'static str {
    if standard {
        include_str!("../runtime/array-standard.txt")
    } else {
        ARRAY_PRELUDE
    }
}

impl RustEmitter<'_> {
    /// An array literal, an append, an element read or a length.
    pub(super) fn array_expr(&mut self, e: &Expr) -> Result<String> {
        match &e.node {
            Node::Array { items } => {
                if items.is_empty() {
                    let element = e.ty.element().expect("an array literal has an array type");
                    return Ok(format!("Vec::<{}>::new()", self.ty(element)?));
                }
                let mut texts = Vec::with_capacity(items.len());
                for item in items {
                    texts.push(self.expr(item)?);
                }
                Ok(format!("vec![{}]", texts.join(", ")))
            }
            Node::Append { left, right } => {
                self.preludes.insert(Prelude::Array);
                if self
                    .array_final_uses
                    .contains(&std::ptr::from_ref(left.as_ref()))
                {
                    return Ok(format!(
                        "{{ let (ml_append_right, ml_append_left) = ({}, {}); crate::ml_array::append(ml_append_left, ml_append_right) }}",
                        self.expr(right)?,
                        self.expr(left)?,
                    ));
                }
                Ok(format!(
                    "crate::ml_array::append({}, {})",
                    self.expr(left)?,
                    self.expr(right)?
                ))
            }
            Node::OptionalIndex { array, index } => {
                self.preludes.insert(Prelude::Array);
                self.standard_array = true;
                let position = if index.ty.is_float() {
                    self.expr(index)?
                } else {
                    self.receiver(index)?
                };
                let values = self.borrow(array)?;
                Ok(
                    crate::translation::array_value_rules::render_optional_array_read(
                        "Rust",
                        index.ty.kind(),
                        &values,
                        &position,
                    ),
                )
            }
            Node::StringPoints { object } => {
                let value = self.receiver(object)?;
                Ok(
                    crate::translation::array_value_rules::render_string_point_iteration(
                        "Rust",
                        self.utf16_strings,
                        &value,
                    ),
                )
            }
            Node::CollectionItems { object, op } => {
                let map = matches!(object.ty, super::Type::Map { .. });
                let value = if op == "entries" && map || !map && op != "entries" {
                    self.expr(object)?
                } else {
                    self.receiver(object)?
                };
                Ok(
                    crate::translation::array_value_rules::render_collection_iteration(
                        "Rust", map, op, &value, "", "",
                    ),
                )
            }
            Node::Index {
                array,
                index,
                proven_in_bounds,
            } => {
                self.preludes.insert(Prelude::Array);
                if !proven_in_bounds {
                    self.state.array_read();
                }
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
                    if self.utf16_strings {
                        let value = self.receiver(array)?;
                        return Ok(crate::translation::primitive_method_rules::render_primitive_string_representation(true, "length", &value, "", ""));
                    }
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
