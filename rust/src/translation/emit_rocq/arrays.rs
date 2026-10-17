//! JavaScript arrays, which the portable core never mutates, are Rocq lists.

use super::{Expr, Helper, Node, Result, RocqEmitter};

pub(super) const LIST_AT: &str = r"(* The element at an index, walking the list; a read outside it, undefined
   in JavaScript, is outside the in-bounds assumption. *)
Fixpoint ml_list_at {A : Type} (values : list A) (index : Z) (fallback : A) : A :=
  match values with
  | nil => fallback
  | cons value rest => if Z.eqb index 0 then value else ml_list_at rest (Z.pred index) fallback
  end.";

pub(super) const FLOAT_INDEX: &str = r"(* The index a Number names, exactly from its binary form; -1 when it names none. *)
Definition ml_float_index (x : float) : Z :=
  match Prim2SF x with
  | S754_zero _ => 0%Z
  | S754_finite false m e =>
      if Z.leb 0 e then Z.shiftl (Z.pos m) e
      else if Z.eqb (Z.modulo (Z.pos m) (2 ^ (- e))) 0 then Z.div (Z.pos m) (2 ^ (- e)) else (-1)%Z
  | _ => (-1)%Z
  end.";

impl RocqEmitter<'_> {
    /// An array literal, an append, an element read or a length.
    pub(super) fn array_expr(&mut self, e: &Expr) -> Result<String> {
        match &e.node {
            Node::Array { items } => {
                let mut parts = Vec::with_capacity(items.len() + 1);
                for item in items {
                    parts.push(self.expr(item)?);
                }
                let element = e.ty.element().expect("an array literal has an array type");
                parts.push(format!("@nil {}", self.ty(element)?));
                Ok(format!("({})", parts.join(" :: ")))
            }
            Node::Set { items } | Node::Map { entries: items } => {
                let (kind, key) = match &e.ty {
                    super::Type::Set { element, .. } => ("set", element.as_ref()),
                    super::Type::Map { key, .. } => ("map", key.as_ref()),
                    _ => unreachable!("a collection constructor has its checked collection type"),
                };
                let value = crate::translation::collection_construction_rules::read_collection_construction_binding("Rocq", kind, "value");
                let previous = crate::translation::collection_construction_rules::read_collection_construction_binding("Rocq", kind, "previous");
                let equal = self.comparison(super::BinaryOp::Eq, key, &previous, &value)?;
                let values = self.expr(items)?;
                let element = match &e.ty {
                    super::Type::Set { element, .. } => self.ty(element)?,
                    super::Type::Map { key, value, .. } => {
                        format!("({} * {})%type", self.ty(key)?, self.ty(value)?)
                    }
                    _ => unreachable!("a collection constructor has its checked collection type"),
                };
                Ok(crate::translation::collection_construction_rules::render_collection_construction(
                    "Rocq", kind, key.kind(), &values, "", &element, &equal,
                ))
            }
            Node::Append { left, right } => Ok(format!(
                "(List.app {} {})",
                self.expr(left)?,
                self.expr(right)?
            )),
            Node::OptionalIndex { array, index } => {
                let position = self.expr(index)?;
                if index.ty.is_float() {
                    self.helpers.insert(Helper::FloatIndex);
                }
                let values = self.expr(array)?;
                Ok(
                    crate::translation::array_value_rules::render_optional_array_read(
                        "Rocq",
                        index.ty.kind(),
                        &values,
                        &position,
                    ),
                )
            }
            Node::StringPoints { .. } => Err(crate::translation::diagnostics::unsupported(
                "string iteration",
                "this target needs a Unicode code-point iterator over its UTF-8 string encoding",
                e.span,
            )),
            Node::CollectionItems { object, op } => {
                let map = matches!(object.ty, super::Type::Map { .. });
                let (key, item) = if op == "entries" {
                    (String::new(), String::new())
                } else {
                    match &object.ty {
                        super::Type::Map { key, value, .. } => (self.ty(key)?, self.ty(value)?),
                        _ => (String::new(), String::new()),
                    }
                };
                let value = self.expr(object)?;
                Ok(
                    crate::translation::array_value_rules::render_collection_iteration(
                        "Rocq", map, op, &value, &key, &item,
                    ),
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
                self.helpers.insert(Helper::ListAt);
                let mut text = self.expr(index)?;
                if index.ty.is_float() {
                    self.helpers.insert(Helper::FloatIndex);
                    text = format!("(ml_float_index {text})");
                } else if index.ty == super::Type::Nat {
                    text = format!("(Z.of_N {text})");
                }
                let values = self.expr(array)?;
                Ok(format!(
                    "(ml_list_at {values} {text} {})",
                    self.inhabitant(&e.ty)?
                ))
            }
            Node::Length { array } => {
                if array.ty == super::Type::String {
                    self.floats();
                    return Ok(
                        crate::translation::frontend_rules::render_string_length_expression(
                            "Rocq",
                            &self.expr(array)?,
                        ),
                    );
                }
                let length = format!("(Z.of_nat (List.length {}))", self.expr(array)?);
                if !e.ty.is_float() {
                    return Ok(length);
                }
                self.floats();
                Ok(format!("(PrimFloat.of_uint63 (Uint63.of_Z {length}))"))
            }
            _ => unreachable!("not an array expression"),
        }
    }
}
