//! The `Math` and `Number` functions on Rocq primitive floats: `PrimFloat`'s
//! where they agree with JavaScript, and the generated math helpers otherwise.

use super::{Expr, Helper, Node, Result, RocqEmitter};

pub(super) const MATH: &str = r"(* Math.trunc, exactly from the binary form: towards zero, a zero result
   keeping the sign of the argument. *)
Definition ml_trunc (x : float) : float :=
  match Prim2SF x with
  | S754_finite negative m e =>
      if Z.leb 0 e then x
      else
        let magnitude := PrimFloat.of_uint63 (Uint63.of_Z (Z.div (Z.pos m) (2 ^ (- e)))) in
        if negative then PrimFloat.opp magnitude else magnitude
  | _ => x
  end.
Definition ml_floor (x : float) : float :=
  let t := ml_trunc x in if PrimFloat.ltb x t then PrimFloat.sub t 1%float else t.
Definition ml_ceil (x : float) : float :=
  let t := ml_trunc x in if PrimFloat.ltb t x then PrimFloat.add t 1%float else t.
(* Math.round: the nearest integer, the one towards +Infinity on a tie, and -0
   from -0.5 up to 0. *)
Definition ml_round (x : float) : float :=
  if orb (orb (PrimFloat.is_nan x) (PrimFloat.is_infinity x)) (PrimFloat.is_zero x) then x
  else if andb (PrimFloat.ltb 0%float x) (PrimFloat.ltb x 0.5%float) then 0%float
  else if andb (PrimFloat.ltb x 0%float) (PrimFloat.leb (PrimFloat.opp 0.5%float) x) then PrimFloat.opp 0%float
  else let f := ml_floor x in if PrimFloat.leb 0.5%float (PrimFloat.sub x f) then PrimFloat.add f 1%float else f.
(* Math.sign: 1 or -1, and -0, 0 and NaN as they are. *)
Definition ml_sign (x : float) : float :=
  if PrimFloat.ltb 0%float x then 1%float else if PrimFloat.ltb x 0%float then PrimFloat.opp 1%float else x.
(* Math.max and Math.min of two Numbers: NaN when either is, and 0 above -0. *)
Definition ml_max (a b : float) : float :=
  if PrimFloat.ltb b a then a else if PrimFloat.ltb a b then b
  else if PrimFloat.eqb a b then (if PrimFloat.get_sign a then b else a) else PrimFloat.nan.
Definition ml_min (a b : float) : float :=
  if PrimFloat.ltb a b then a else if PrimFloat.ltb b a then b
  else if PrimFloat.eqb a b then (if PrimFloat.get_sign a then a else b) else PrimFloat.nan.
Definition ml_is_finite (x : float) : bool := negb (orb (PrimFloat.is_nan x) (PrimFloat.is_infinity x)).
Definition ml_is_integer (x : float) : bool := andb (ml_is_finite x) (PrimFloat.eqb (ml_trunc x) x).
Definition ml_is_safe_integer (x : float) : bool :=
  andb (ml_is_integer x) (PrimFloat.leb (PrimFloat.abs x) 9007199254740991%float).";

impl RocqEmitter<'_> {
    pub(super) fn math(&mut self, e: &Expr) -> Result<String> {
        let Node::Math { op, args } = &e.node else {
            unreachable!("not a Math expression")
        };
        if crate::translation::string_construction_rules::accept_linked_java_script_string_construction("math",op){
            self.floats();self.helpers.insert(super::Helper::Utf16Text);
            let value=self.expr(&args[0])?;
            return Ok(crate::translation::string_construction_rules::render_java_script_string_construction("Rocq",op,&value));
        }
        if crate::translation::big_integer_conversion_rules::accept_linked_java_script_big_integer_result("math",op) {
            self.floats();self.helpers.insert(super::Helper::BigIntegerConversion);
            let form=crate::translation::big_integer_conversion_rules::read_java_script_big_integer_result_operation(op);
            let value=self.expr(&args[0])?;
            if form=="number-power" {
                let right=self.expr(&args[1])?;
                return Ok(crate::translation::big_integer_conversion_rules::render_java_script_number_power("Rocq",&value,&right));
            }
            if form=="power" {
                let right=self.expr(&args[1])?;
                return Ok(crate::translation::big_integer_conversion_rules::render_java_script_big_integer_power("Rocq",&value,&right));
            }
            if form=="division" {
                let right=self.expr(&args[1])?;
                return Ok(crate::translation::big_integer_conversion_rules::render_java_script_big_integer_division("Rocq",&value,&right));
            }
            return Ok(crate::translation::big_integer_conversion_rules::render_java_script_big_integer_result_expression("Rocq",&form,&value,false));
        }
        if crate::translation::string_number_rules::accept_linked_java_script_number_conversion(
            "math", op,
        ) {
            self.floats();
            if matches!(args[0].ty.kind(), "string" | "int" | "nat") {
                self.helpers.insert(super::Helper::StringNumber);
            }
            let value = self.expr(&args[0])?;
            let text = crate::translation::string_number_rules::render_java_script_number_scalar_conversion("Rocq", op, args[0].ty.kind(), &value, false);
            if text.is_empty() {
                return Err(super::unsupported(
                    "numeric conversion operand",
                    "the checked conversion must retain its total value or threaded abort",
                    e.span,
                ));
            }
            return Ok(text);
        }
        if crate::translation::rust_shift_rules::accept_linked_rust_checked_shift("math", op) {
            let values = args
                .iter()
                .map(|argument| self.expr(argument))
                .collect::<Result<Vec<_>>>()?;
            return Ok(crate::translation::rust_shifts::emit_rust_checked_shift(
                "Rocq", e, &values,
            ));
        }
        if crate::translation::rust_primitive_bitwise_rules::read_rust_primitive_bitwise_arity(op)
            > 0.0
        {
            let values = args
                .iter()
                .map(|argument| self.expr(argument))
                .collect::<Result<Vec<_>>>()?;
            return Ok(
                crate::translation::rust_primitive_bitwise::emit_rust_primitive_bitwise(
                    "Rocq", e, &values,
                ),
            );
        }
        if crate::translation::big_integer_bitwise_rules::read_big_integer_bitwise_arity(op) > 0.0 {
            let values = args
                .iter()
                .map(|argument| self.expr(argument))
                .collect::<Result<Vec<_>>>()?;
            return Ok(crate::translation::big_integer_bitwise_rules::render_big_integer_bitwise_expression("Rocq", op, &values));
        }
        self.floats();
        let mut texts = Vec::with_capacity(args.len());
        for arg in args {
            texts.push(self.expr(arg)?);
        }
        let native = match op.as_str() {
            "abs" => Some("PrimFloat.abs"),
            "sqrt" => Some("PrimFloat.sqrt"),
            "isNaN" => Some("PrimFloat.is_nan"),
            _ => None,
        };
        if let Some(native) = native {
            return Ok(format!("({native} {})", texts[0]));
        }
        self.helpers.insert(Helper::Math);
        let bound = |max: bool| {
            if max {
                "PrimFloat.neg_infinity"
            } else {
                "PrimFloat.infinity"
            }
        };
        Ok(match op.as_str() {
            "max" | "min" => texts
                .into_iter()
                .reduce(|left, right| format!("(ml_{op} {left} {right})"))
                .unwrap_or_else(|| bound(op == "max").to_owned()),
            "maxOf" | "minOf" => format!(
                "(List.fold_left ml_{} {} {})",
                &op[..3],
                texts[0],
                bound(op == "maxOf")
            ),
            _ => {
                let name = match op.as_str() {
                    "isFinite" => "is_finite",
                    "isInteger" => "is_integer",
                    "isSafeInteger" => "is_safe_integer",
                    other => other,
                };
                format!("(ml_{name} {})", texts[0])
            }
        })
    }
}
