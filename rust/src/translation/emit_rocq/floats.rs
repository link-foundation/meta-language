//! The Rocq definitions for JavaScript Numbers, on primitive floats.

/// The `Require` lines of a file that uses Numbers. Number literals are
/// written as in the source; Rocq reads each one as the nearest float, as
/// JavaScript does, and would otherwise warn that it rounds.
pub const FLOAT_PRELUDE: &[&str] = &[
    "From Stdlib Require Import NArith ZArith Lia Recdef String List Ascii Floats.",
    r#"Set Warnings "-inexact-float"."#,
];

pub(super) const JS_NUMBER: &str = r#"(* ECMAScript Number::toString for binary64: the shortest decimal that rounds
   back to the value (the nearest one, the even one on a tie), laid out as
   JavaScript prints it. Exact Z arithmetic on the decomposed float. *)
Fixpoint ml_js_zeros (n : nat) : string :=
  match n with O => EmptyString | S k => String "0"%char (ml_js_zeros k) end.
Fixpoint ml_js_trim (s : string) : string :=
  match s with
  | EmptyString => EmptyString
  | String c rest =>
      let r := ml_js_trim rest in
      if andb (Ascii.eqb c "0"%char) (String.eqb r EmptyString) then EmptyString else String c r
  end.
Definition ml_js_layout (digits : string) (n : Z) : string :=
  let len := String.length digits in
  let k := Z.of_nat len in
  if andb (Z.leb k n) (Z.leb n 21) then (digits ++ ml_js_zeros (Z.to_nat (n - k)))%string
  else if andb (Z.ltb 0 n) (Z.leb n 21) then
    (substring 0 (Z.to_nat n) digits ++ "." ++ substring (Z.to_nat n) len digits)%string
  else if andb (Z.ltb (-6) n) (Z.leb n 0) then ("0." ++ ml_js_zeros (Z.to_nat (- n)) ++ digits)%string
  else
    let e := (n - 1)%Z in
    let exp := ((if Z.ltb e 0 then "e-" else "e+") ++ ml_N_to_string (Z.abs_N e))%string in
    if Z.eqb k 1 then (digits ++ exp)%string
    else (substring 0 1 digits ++ "." ++ substring 1 len digits ++ exp)%string.
(* The value is v * 2^sh for a scaled mantissa v. c * 10^q compared with
   x * 2^sh, and floor (v * 2^sh / 10^q), with shifts for the powers of 2. *)
Definition ml_js_cmp (c q x sh : Z) : comparison :=
  Z.compare (Z.shiftl (c * 10 ^ Z.max q 0) (Z.max (- sh) 0))
            (Z.shiftl (x * 10 ^ Z.max (- q) 0) (Z.max sh 0)).
Definition ml_js_floor (v q sh : Z) : Z :=
  Z.div (Z.shiftl (v * 10 ^ Z.max (- q) 0) (Z.max sh 0))
        (Z.shiftl (10 ^ Z.max q 0) (Z.max (- sh) 0)).
(* With p significant digits: the candidate in the rounding interval nearest
   to the value (the even one on a tie), as (digits, exponent). *)
Definition ml_js_candidate (p e10 v lo hi sh : Z) (inclusive : bool) : option (Z * Z) :=
  let q := (e10 - p + 1)%Z in
  let c := ml_js_floor v q sh in
  let inside (c : Z) :=
    andb (match ml_js_cmp c q lo sh with Gt => true | Eq => inclusive | Lt => false end)
         (match ml_js_cmp c q hi sh with Lt => true | Eq => inclusive | Gt => false end) in
  let low := inside c in
  let high := inside (c + 1)%Z in
  if andb low high then
    match ml_js_cmp (2 * c + 1) q (2 * v) sh with
    | Gt => Some (c, q) | Lt => Some ((c + 1)%Z, q) | Eq => Some (if Z.even c then c else (c + 1)%Z, q)
    end
  else if low then Some (c, q) else if high then Some ((c + 1)%Z, q) else None.
(* A candidate with p digits is one with p + 1 digits too, so the fewest
   digits are found by bisection over [first, last]; 17 always suffice. *)
Fixpoint ml_js_fewest (fuel : nat) (first last e10 v lo hi sh : Z) (inclusive : bool) : Z * Z :=
  match fuel with
  | S fuel =>
      if Z.ltb first last then
        let middle := Z.div (first + last) 2 in
        match ml_js_candidate middle e10 v lo hi sh inclusive with
        | Some _ => ml_js_fewest fuel first middle e10 v lo hi sh inclusive
        | None => ml_js_fewest fuel (middle + 1) last e10 v lo hi sh inclusive
        end
      else match ml_js_candidate last e10 v lo hi sh inclusive with Some r => r | None => (0, 0)%Z end
  | O => match ml_js_candidate last e10 v lo hi sh inclusive with Some r => r | None => (0, 0)%Z end
  end.
Definition ml_js_shortest (m e : Z) (lowerCloser : bool) : string :=
  let sh := (e - 2)%Z in
  let v := (4 * m)%Z in
  let lo := if lowerCloser then (v - 1)%Z else (v - 2)%Z in
  let guess := Z.div ((Z.log2 m + e) * 78913) 262144 in
  let e10 :=
    match ml_js_cmp 1 (guess + 1) v sh with
    | Gt => match ml_js_cmp 1 guess v sh with Gt => (guess - 1)%Z | _ => guess end
    | _ => (guess + 1)%Z
    end in
  let (pick, q) := ml_js_fewest 5 1 17 e10 v lo (v + 2) sh (Z.even m) in
  let text := ml_N_to_string (Z.to_N pick) in
  ml_js_layout (ml_js_trim text) (q + Z.of_nat (String.length text)).
Definition ml_js_number (x : float) : string :=
  match Prim2SF x with
  | S754_zero _ => "0"
  | S754_infinity negative => if negative then "-Infinity" else "Infinity"
  | S754_nan => "NaN"
  | S754_finite negative m e =>
      let body := ml_js_shortest (Zpos m) e (andb (Pos.eqb m (2 ^ 52)) (Z.ltb (-1074) e)) in
      if negative then ("-" ++ body)%string else body
  end."#;

pub(super) const JS_CONSOLE: &str = r#"(* What console.log prints: -0 as "-0", where String(-0) is "0". *)
Definition ml_js_console (x : float) : string :=
  if andb (PrimFloat.is_zero x) (PrimFloat.get_sign x) then "-0" else ml_js_number x."#;

pub(super) const FLOAT_SAME: &str = r"(* SameValue: NaN equals NaN, and 0 and -0 differ. *)
Definition ml_float_same (a b : float) : bool :=
  if PrimFloat.is_nan a then PrimFloat.is_nan b
  else andb (PrimFloat.eqb a b) (Bool.eqb (PrimFloat.get_sign a) (PrimFloat.get_sign b)).";

pub(super) const FLOAT_REM: &str = r"(* ECMAScript % on binary64 (C fmod): the exact truncated remainder with the
   sign of the dividend, computed on the mantissas, so it is exact. *)
Definition ml_float_rem (x y : float) : float :=
  if orb (orb (PrimFloat.is_nan x) (PrimFloat.is_nan y)) (orb (PrimFloat.is_infinity x) (PrimFloat.is_zero y))
  then PrimFloat.div 0 0
  else if orb (PrimFloat.is_infinity y) (PrimFloat.is_zero x) then x
  else match Prim2SF x, Prim2SF y with
  | S754_finite sx mx ex, S754_finite _ my ey =>
      let e := Z.min ex ey in
      let r := Z.modulo (Z.pos mx * 2 ^ (ex - e)) (Z.pos my * 2 ^ (ey - e)) in
      let magnitude := Z.ldexp (PrimFloat.of_uint63 (Uint63.of_Z r)) e in
      if sx then PrimFloat.opp magnitude else magnitude
  | _, _ => x
  end.";
