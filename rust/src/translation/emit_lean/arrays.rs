//! JavaScript arrays, which the portable core never mutates, are Lean `Array`s.

use std::collections::HashSet;

use super::{Expr, LeanEmitter, Node, Result, Type};

impl LeanEmitter<'_> {
    pub(super) fn array_map_expression(
        &mut self,
        object: &Expr,
        callback: &Expr,
        depth: usize,
    ) -> Result<String> {
        let object = self.expr(object, depth)?;
        let Type::Function { params, .. } = &callback.ty else {
            unreachable!("the map checker retained its actual function type");
        };
        let count = serde_json::Number::from(params.len())
            .as_f64()
            .expect("a checked parameter count has a Number representation");
        let callback = self.expr(callback, depth)?;
        Ok(
            crate::translation::array_from_rules::render_array_from_mapping(
                "Lean", &object, &callback, count, "",
            ),
        )
    }

    /// An array literal, an append, an element read or a length.
    pub(super) fn array_expr(&mut self, e: &Expr, depth: usize) -> Result<String> {
        Ok(match &e.node {
            Node::CollectionSize { object } => {
                let value = self.expr(object, depth)?;
                crate::translation::collection_access_rules::render_collection_size_expression(
                    "Lean", &value,
                )
            }
            Node::CollectionRead { object, op, key } => {
                let key_type = match &object.ty {
                    Type::Map { key, .. } | Type::Set { element: key, .. } => key.as_ref(),
                    _ => unreachable!("checked collection access owner"),
                };
                let float_key = key_type.is_float();
                let value = self.expr(object, depth)?;
                let search = self.expr(key, depth)?;
                crate::translation::collection_access_rules::render_collection_read_expression(
                    "Lean",
                    object.ty.kind(),
                    op,
                    &value,
                    &search,
                    float_key,
                    "",
                    "",
                    "",
                )
            }
            Node::Array { items } => {
                let mut texts = Vec::with_capacity(items.len());
                for item in items {
                    texts.push(self.expr(item, depth)?);
                }
                format!("(#[{}] : {})", texts.join(", "), self.ty(&e.ty)?)
            }
            Node::Set { items } | Node::Map { entries: items } => {
                let (kind, key) = match &e.ty {
                    Type::Set { element, .. } => ("set", element.as_ref()),
                    Type::Map { key, .. } => ("map", key.as_ref()),
                    _ => unreachable!("a collection constructor has its checked collection type"),
                };
                let values = self.expr(items, depth)?;
                let collection = self.ty(&e.ty)?;
                crate::translation::collection_construction_rules::render_collection_construction(
                    "Lean",
                    kind,
                    key.kind(),
                    &values,
                    &collection,
                    "",
                    "",
                )
            }
            Node::Append { left, right } => {
                format!(
                    "({} ++ {})",
                    self.expr(left, depth)?,
                    self.expr(right, depth)?
                )
            }
            Node::OptionalIndex { array, index } => {
                let position = self.expr(index, depth)?;
                let values = self.expr(array, depth)?;
                crate::translation::array_value_rules::render_optional_array_read(
                    "Lean",
                    index.ty.kind(),
                    &values,
                    &position,
                )
            }
            Node::StringPoints { object } => {
                let value = self.expr(object, depth)?;
                crate::translation::array_value_rules::render_string_point_iteration(
                    "Lean", false, &value,
                )
            }
            Node::CollectionItems { object, op } => {
                let map = matches!(object.ty, Type::Map { .. });
                let value = self.expr(object, depth)?;
                crate::translation::array_value_rules::render_collection_iteration(
                    "Lean", map, op, &value, "", "",
                )
            }
            Node::Index {
                array,
                index,
                proven_in_bounds,
            } => {
                if !proven_in_bounds {
                    self.state.array_read();
                }
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
                let (bits, signed) = match &e.ty {
                    Type::Fixed { bits, signed } => (*bits, *signed),
                    _ => (0, false),
                };
                if crate::translation::rust_array_length_rules::accept_rust_exact_array_length_result("length",e.ty.kind(),f64::from(bits),signed,array.ty.kind()) {
                    return Ok(crate::translation::rust_array_length_rules::render_rust_exact_array_length("Lean", &self.expr(array,depth)?));
                }
                if array.ty == Type::String {
                    return Ok(
                        crate::translation::frontend_rules::render_string_length_expression(
                            "Lean",
                            &self.expr(array, depth)?,
                        ),
                    );
                }
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
            Type::Array { element, .. } => self.undecidable_type(element, true, seen),
            Type::Tuple { elements } => elements
                .iter()
                .any(|element| self.undecidable_type(element, nested, seen)),
            Type::Data { name } => nested || self.undecidable(name, seen),
            _ => false,
        }
    }
}
