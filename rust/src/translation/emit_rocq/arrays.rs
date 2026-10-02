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
            Node::Append { left, right } => Ok(format!(
                "(List.app {} {})",
                self.expr(left)?,
                self.expr(right)?
            )),
            Node::Index { array, index } => {
                self.state.array_read();
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
