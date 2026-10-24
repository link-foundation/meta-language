From Stdlib Require Import NArith ZArith Lia Recdef String List Ascii Floats.
Module MLStringNumber.
(* Translated from JavaScript by meta-language: portable core, Rocq target. *)
Set Warnings "-inexact-float".

(* ECMAScript % on binary64 (C fmod): the exact truncated remainder with the
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
  end.

(* Math.trunc, exactly from the binary form: towards zero, a zero result
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
  andb (ml_is_integer x) (PrimFloat.leb (PrimFloat.abs x) 9007199254740991%float).

(* The index a Number names, exactly from its binary form; -1 when it names none. *)
Definition ml_float_index (x : float) : Z :=
  match Prim2SF x with
  | S754_zero _ => 0%Z
  | S754_finite false m e =>
      if Z.leb 0 e then Z.shiftl (Z.pos m) e
      else if Z.eqb (Z.modulo (Z.pos m) (2 ^ (- e))) 0 then Z.div (Z.pos m) (2 ^ (- e)) else (-1)%Z
  | _ => (-1)%Z
  end.

(* The least fixed point of a one-step unfolding F, unfolded lazily: level k
   nests 2^k calls of F before it would reach the fallback. *)
Fixpoint ml_fix {A B : Type} (k : nat) (F : (A -> B) -> A -> B) (fallback : A -> B) : A -> B :=
  match k with
  | O => F fallback
  | S k => fun a => ml_fix k F (ml_fix k F fallback) a
  end.

Ltac ml_N_norm := repeat first
  [ rewrite N.pred_succ
  | rewrite (proj2 (N.eqb_neq (N.succ _) 0) (N.neq_succ_0 _))
  | rewrite N.eqb_refl ].
Ltac ml_obligation := intros; repeat match goal with H : N.eqb _ _ = false |- _ => apply N.eqb_neq in H end; lia.
Ltac ml_close := first [reflexivity | lia | nia | congruence].

Definition acceptJavaScriptWhitespaceUnit (unit_ : float) : bool :=
  (orb (orb (orb (orb (orb (orb (orb (orb (orb (orb (andb (PrimFloat.leb 9%float unit_) (PrimFloat.leb unit_ 13%float)) (PrimFloat.eqb unit_ 32%float)) (PrimFloat.eqb unit_ 160%float)) (PrimFloat.eqb unit_ 5760%float)) (andb (PrimFloat.leb 8192%float unit_) (PrimFloat.leb unit_ 8202%float))) (PrimFloat.eqb unit_ 8232%float)) (PrimFloat.eqb unit_ 8233%float)) (PrimFloat.eqb unit_ 8239%float)) (PrimFloat.eqb unit_ 8287%float)) (PrimFloat.eqb unit_ 12288%float)) (PrimFloat.eqb unit_ 65279%float)).

Definition readStringNumberDigit (unit_ : float) : float :=
  (if (andb (PrimFloat.leb 48%float unit_) (PrimFloat.leb unit_ 57%float)) then (PrimFloat.sub unit_ 48%float) else (if (andb (PrimFloat.leb 65%float unit_) (PrimFloat.leb unit_ 70%float)) then (PrimFloat.sub unit_ 55%float) else (if (andb (PrimFloat.leb 97%float unit_) (PrimFloat.leb unit_ 102%float)) then (PrimFloat.sub unit_ 87%float) else (PrimFloat.opp 1%float)))).

Definition readJavaScriptNumberBuiltinOperation (name : string) : string :=
  (if (String.eqb name "Number"%string) then "convertJavaScriptNumber"%string else (if (String.eqb name "isFinite"%string) then "testCoercedJavaScriptNumberFinite"%string else (if (String.eqb name "isNaN"%string) then "testCoercedJavaScriptNumberNaN"%string else ""%string))).

Definition readJavaScriptNumberConversionResult (operation : string) : string :=
  (if (orb (String.eqb operation "convertJavaScriptNumber"%string) (String.eqb operation "coerceJavaScriptNumber"%string)) then "float"%string else (if (orb (String.eqb operation "testCoercedJavaScriptNumberFinite"%string) (String.eqb operation "testCoercedJavaScriptNumberNaN"%string)) then "bool"%string else ""%string)).

Definition readJavaScriptNumberConversionForm (kind : string) : string :=
  (if (orb (orb (orb (orb (orb (orb (String.eqb kind "float"%string) (String.eqb kind "bool"%string)) (String.eqb kind "string"%string)) (String.eqb kind "unit"%string)) (String.eqb kind "int"%string)) (String.eqb kind "nat"%string)) (String.eqb kind "option"%string)) then kind else "unsupported"%string).

Definition acceptLinkedJavaScriptNumberConversion (kind : string) (operation : string) : bool :=
  (andb (String.eqb kind "math"%string) (orb (String.eqb operation "convertJavaScriptNumber"%string) (String.eqb operation "coerceJavaScriptNumber"%string))).

Definition readJavaScriptNumberConversionFailure (operation : string) (kind : string) : string :=
  (if (andb (String.eqb operation "coerceJavaScriptNumber"%string) (orb (String.eqb kind "int"%string) (String.eqb kind "nat"%string))) then "Cannot convert a BigInt value to a number"%string else ""%string).

Definition requireJavaScriptNumberConversionUnits (kind : string) (operation : string) (argument : string) : bool :=
  (andb (acceptLinkedJavaScriptNumberConversion kind operation) (String.eqb argument "string"%string)).

Definition readJavaScriptPrimitiveNumberOperand (language : string) (kind : string) (element : string) : string :=
  (if (negb (String.eqb language "JavaScript"%string)) then ""%string else (if (orb (orb (String.eqb kind "bool"%string) (String.eqb kind "string"%string)) (String.eqb kind "unit"%string)) then "coerceJavaScriptNumber"%string else (if (andb (String.eqb kind "option"%string) (String.eqb element "string"%string)) then "coerceJavaScriptNumber"%string else ""%string))).

Definition readJavaScriptPrimitiveNumericInferenceKind (kind : string) : string :=
  (if (orb (orb (String.eqb kind "bool"%string) (String.eqb kind "string"%string)) (String.eqb kind "unit"%string)) then "float"%string else ""%string).

Definition readJavaScriptPrimitiveUnaryOperation (symbol : string) : string :=
  (if (String.eqb symbol "+"%string) then "coerceJavaScriptNumber"%string else ""%string).

Definition readJavaScriptMathematicalConversionForm (language : string) (operation : string) : string :=
  (if (negb (String.eqb language "JavaScript"%string)) then ""%string else (if (orb (String.eqb operation "max"%string) (String.eqb operation "min"%string)) then "variadic"%string else (if (orb (orb (orb (orb (orb (orb (String.eqb operation "abs"%string) (String.eqb operation "floor"%string)) (String.eqb operation "ceil"%string)) (String.eqb operation "trunc"%string)) (String.eqb operation "round"%string)) (String.eqb operation "sign"%string)) (String.eqb operation "sqrt"%string)) then "unary"%string else ""%string))).

Definition renderJavaScriptNumberScalarConversion (format : string) (operation : string) (form : string) (value : string) (utf16 : bool) : string :=
  (if (orb (orb (negb (acceptLinkedJavaScriptNumberConversion "math"%string operation)) (String.eqb (readJavaScriptNumberConversionForm form) "unsupported"%string)) (String.eqb form "option"%string)) then ""%string else (if (String.eqb format "JavaScript"%string) then (if (String.eqb operation "convertJavaScriptNumber"%string) then (String.append (String.append "Number("%string value) ")"%string) else (String.append (String.append "(+("%string value) "))"%string)) else (if (orb (String.eqb format "Lean"%string) (String.eqb format "Rocq"%string)) then (if (negb (String.eqb (readJavaScriptNumberConversionFailure operation form) EmptyString)) then ""%string else (if (String.eqb form "float"%string) then value else (if (String.eqb form "unit"%string) then (if (String.eqb format "Lean"%string) then (String.append (String.append "(let _ := "%string value) "; ((0 : Float) / 0))"%string) else (String.append (String.append "(let _ := "%string value) " in PrimFloat.nan)"%string)) else (if (String.eqb form "bool"%string) then (if (String.eqb format "Lean"%string) then (String.append (String.append "(if "%string value) " then (1 : Float) else (0 : Float))"%string) else (String.append (String.append "(if "%string value) " then 1%float else 0%float)"%string)) else (if (orb (String.eqb form "int"%string) (String.eqb form "nat"%string)) then (String.append (String.append "(MLStringNumber.convertInteger "%string (if (String.eqb form "nat"%string) then (if (String.eqb format "Lean"%string) then (String.append (String.append "(Int.ofNat "%string value) ")"%string) else (String.append (String.append "(Z.of_N "%string value) ")"%string)) else value)) ")"%string) else (if (String.eqb format "Lean"%string) then (String.append (String.append "(MLStringNumber.convertUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] "%string value) "))"%string) else (String.append (String.append "(MLStringNumber.convertUtf8 ((fix ml_numeric_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_numeric_bytes rest) end) "%string value) "))"%string))))))) else (if (negb (String.eqb format "Rust"%string)) then ""%string else (if (String.eqb form "float"%string) then value else (if (String.eqb form "bool"%string) then (String.append (String.append "(if "%string value) " { 1.0f64 } else { 0.0f64 })"%string) else (if (String.eqb form "unit"%string) then (String.append (String.append "{ let _ = "%string value) "; f64::NAN }"%string) else (if (orb (String.eqb form "int"%string) (String.eqb form "nat"%string)) then (if (String.eqb operation "coerceJavaScriptNumber"%string) then (String.append (String.append "{ let _ = "%string value) "; panic!(""Cannot convert a BigInt value to a number"") }"%string) else (String.append (String.append "crate::ml_string_number::convert_java_script_big_integer_number("%string value) ")"%string)) else (let units := (if utf16 then (String.append (String.append "("%string value) ").0.iter().copied()"%string) else (String.append (String.append "("%string value) ").encode_utf16()"%string)) in (String.append (String.append "crate::ml_string_number::convert_java_script_string_number_units(&"%string units) ".map(f64::from).collect::<Vec<f64>>())"%string)))))))))).

Definition readJavaScriptNumberConversionStatement : string :=
  "ECMAScript Number and ToNumber preserve full UTF-16 StringNumericLiteral syntax, exact binary64 ties-to-even rounding, signed zero, Infinity, NaN, primitive absence and argument evaluation order; explicit BigInt conversion rounds once and implicit BigInt conversion throws"%string.

Definition source_ml_source1_temporary_readStringNumberDecimalPower_loop1 (exponent : float) (value : Z) (base : Z) : Z :=
  ml_fix 64 (fun (ml_rec : float * Z * Z -> Z) (ml_args : float * Z * Z) =>
    let '(exponent, value, base) := ml_args in (if (PrimFloat.ltb 0%float exponent) then (let value_3 := (if (PrimFloat.eqb (ml_float_rem exponent 2%float) 1%float) then (let value_2 := (Z.mul value base) in value_2) else value) in (let exponent_2 := (ml_floor (PrimFloat.div exponent 2%float)) in (if (PrimFloat.ltb 0%float exponent_2) then (let base_2 := (Z.mul base base) in (ml_rec (exponent_2, value_3, base_2))) else (ml_rec (exponent_2, value_3, base))))) else value))
    (fun _ => 0%Z) (exponent, value, base).

Definition readStringNumberDecimalPower (exponent : float) : Z :=
  (let value := 1%Z in (let base := 10%Z in (let value_2 := (source_ml_source1_temporary_readStringNumberDecimalPower_loop1 exponent value base) in value_2))).

Definition source_ml_source1_temporary_readStringNumberExactMantissa_loop2 (value : Z) (result : float) (scale : float) : float :=
  ml_fix 64 (fun (ml_rec : Z * float * float -> float) (ml_args : Z * float * float) =>
    let '(value, result, scale) := ml_args in (if (Z.ltb 0%Z value) then (let result_3 := (if (Z.eqb (Z.rem value 2%Z) 1%Z) then (let result_2 := (PrimFloat.add result scale) in result_2) else result) in (let value_2 := (Z.quot value 2%Z) in (let scale_2 := (PrimFloat.mul scale 2%float) in (ml_rec (value_2, result_3, scale_2))))) else result))
    (fun _ => 0%float) (value, result, scale).

Definition readStringNumberExactMantissa (value : Z) : float :=
  (let result := 0%float in (let scale := 1%float in (let result_2 := (source_ml_source1_temporary_readStringNumberExactMantissa_loop2 value result scale) in result_2))).

Definition source_ml_source1_temporary_readStringNumberBinaryPower_loop4 (exponent : float) (result : float) : float :=
  ml_fix 64 (fun (ml_rec : float * float -> float) (ml_args : float * float) =>
    let '(exponent, result) := ml_args in (if (PrimFloat.ltb exponent 0%float) then (let result_2 := (PrimFloat.div result 2%float) in (let exponent_2 := (PrimFloat.add exponent 1%float) in (ml_rec (exponent_2, result_2)))) else result))
    (fun _ => 0%float) (exponent, result).

Inductive source_ml_source1_temporary_readStringNumberBinaryPower_loop3_result : Type :=
| source_ml_source1_temporary_readStringNumberBinaryPower_loop3_done : float -> float -> source_ml_source1_temporary_readStringNumberBinaryPower_loop3_result.

Definition source_ml_source1_temporary_readStringNumberBinaryPower_loop3 (exponent : float) (result : float) : source_ml_source1_temporary_readStringNumberBinaryPower_loop3_result :=
  ml_fix 64 (fun (ml_rec : float * float -> source_ml_source1_temporary_readStringNumberBinaryPower_loop3_result) (ml_args : float * float) =>
    let '(exponent, result) := ml_args in (if (PrimFloat.ltb 0%float exponent) then (let result_2 := (PrimFloat.mul result 2%float) in (let exponent_2 := (PrimFloat.sub exponent 1%float) in (ml_rec (exponent_2, result_2)))) else (source_ml_source1_temporary_readStringNumberBinaryPower_loop3_done exponent result)))
    (fun _ => (source_ml_source1_temporary_readStringNumberBinaryPower_loop3_done 0%float 0%float)) (exponent, result).

Definition readStringNumberBinaryPower (exponent : float) : float :=
  (let result := 1%float in (let source_ml_source2_s1 := (source_ml_source1_temporary_readStringNumberBinaryPower_loop3 exponent result) in (match source_ml_source2_s1 with | source_ml_source1_temporary_readStringNumberBinaryPower_loop3_done exponent_2 result_2 => (let result_3 := (source_ml_source1_temporary_readStringNumberBinaryPower_loop4 exponent_2 result_2) in result_3) end))).

Inductive source_ml_source1_temporary_convertStringNumberRational_loop8_result : Type :=
| source_ml_source1_temporary_convertStringNumberRational_loop8_done : Z -> Z -> source_ml_source1_temporary_convertStringNumberRational_loop8_result.

Definition source_ml_source1_temporary_convertStringNumberRational_loop8 (quotient : Z) (remainder : Z) (scale : Z) (weight : Z) : source_ml_source1_temporary_convertStringNumberRational_loop8_result :=
  ml_fix 64 (fun (ml_rec : Z * Z * Z * Z -> source_ml_source1_temporary_convertStringNumberRational_loop8_result) (ml_args : Z * Z * Z * Z) =>
    let '(quotient, remainder, scale, weight) := ml_args in (if (Z.ltb 1%Z weight) then (let scale_2 := (Z.quot scale 2%Z) in (let weight_2 := (Z.quot weight 2%Z) in (if (Z.leb scale_2 remainder) then (let remainder_2 := (Z.sub remainder scale_2) in (let quotient_2 := (Z.add quotient weight_2) in (ml_rec (quotient_2, remainder_2, scale_2, weight_2)))) else (ml_rec (quotient, remainder, scale_2, weight_2))))) else (source_ml_source1_temporary_convertStringNumberRational_loop8_done quotient remainder)))
    (fun _ => (source_ml_source1_temporary_convertStringNumberRational_loop8_done 0%Z 0%Z)) (quotient, remainder, scale, weight).

Inductive source_ml_source1_temporary_convertStringNumberRational_loop7_result : Type :=
| source_ml_source1_temporary_convertStringNumberRational_loop7_done : Z -> Z -> source_ml_source1_temporary_convertStringNumberRational_loop7_result.

Definition source_ml_source1_temporary_convertStringNumberRational_loop7 (remainder : Z) (scale : Z) (weight : Z) : source_ml_source1_temporary_convertStringNumberRational_loop7_result :=
  ml_fix 64 (fun (ml_rec : Z * Z * Z -> source_ml_source1_temporary_convertStringNumberRational_loop7_result) (ml_args : Z * Z * Z) =>
    let '(remainder, scale, weight) := ml_args in (if (Z.leb scale remainder) then (let scale_2 := (Z.mul scale 2%Z) in (let weight_2 := (Z.mul weight 2%Z) in (ml_rec (remainder, scale_2, weight_2)))) else (source_ml_source1_temporary_convertStringNumberRational_loop7_done scale weight)))
    (fun _ => (source_ml_source1_temporary_convertStringNumberRational_loop7_done 0%Z 0%Z)) (remainder, scale, weight).

Inductive source_ml_source1_temporary_convertStringNumberRational_join6 : Type :=
| source_ml_source1_temporary_convertStringNumberRational_join6_next : Z -> Z -> source_ml_source1_temporary_convertStringNumberRational_join6.

Definition source_ml_source1_temporary_convertStringNumberRational_loop9 (numerator : Z) (shift : float) (index : float) : Z :=
  ml_fix 64 (fun (ml_rec : Z * float * float -> Z) (ml_args : Z * float * float) =>
    let '(numerator, shift, index) := ml_args in (if (PrimFloat.ltb index shift) then (let numerator_2 := (Z.mul numerator 2%Z) in (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (numerator_2, shift, index_2)))) else numerator))
    (fun _ => 0%Z) (numerator, shift, index).

Definition source_ml_source1_temporary_convertStringNumberRational_loop10 (denominator : Z) (shift : float) (index : float) : Z :=
  ml_fix 64 (fun (ml_rec : Z * float * float -> Z) (ml_args : Z * float * float) =>
    let '(denominator, shift, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.opp shift)) then (let denominator_2 := (Z.mul denominator 2%Z) in (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (denominator_2, shift, index_2)))) else denominator))
    (fun _ => 0%Z) (denominator, shift, index).

Inductive source_ml_source1_temporary_convertStringNumberRational_join5 : Type :=
| source_ml_source1_temporary_convertStringNumberRational_join5_next : float -> source_ml_source1_temporary_convertStringNumberRational_join5
| source_ml_source1_temporary_convertStringNumberRational_join5_return : float -> source_ml_source1_temporary_convertStringNumberRational_join5.

Inductive source_ml_source1_temporary_convertStringNumberRational_loop11_result : Type :=
| source_ml_source1_temporary_convertStringNumberRational_loop11_done : float -> source_ml_source1_temporary_convertStringNumberRational_loop11_result
| source_ml_source1_temporary_convertStringNumberRational_loop11_return : float -> source_ml_source1_temporary_convertStringNumberRational_loop11_result.

Definition source_ml_source1_temporary_convertStringNumberRational_loop11 (numerator : Z) (exponent : float) (scale : Z) : source_ml_source1_temporary_convertStringNumberRational_loop11_result :=
  ml_fix 64 (fun (ml_rec : Z * float * Z -> source_ml_source1_temporary_convertStringNumberRational_loop11_result) (ml_args : Z * float * Z) =>
    let '(numerator, exponent, scale) := ml_args in (if (Z.leb (Z.mul scale 2%Z) numerator) then (let scale_2 := (Z.mul scale 2%Z) in (let exponent_2 := (PrimFloat.add exponent 1%float) in (if (PrimFloat.leb 1024%float exponent_2) then (source_ml_source1_temporary_convertStringNumberRational_loop11_return PrimFloat.infinity) else (ml_rec (numerator, exponent_2, scale_2))))) else (source_ml_source1_temporary_convertStringNumberRational_loop11_done exponent)))
    (fun _ => (source_ml_source1_temporary_convertStringNumberRational_loop11_done 0%float)) (numerator, exponent, scale).

Inductive source_ml_source1_temporary_convertStringNumberRational_loop12_result : Type :=
| source_ml_source1_temporary_convertStringNumberRational_loop12_done : float -> source_ml_source1_temporary_convertStringNumberRational_loop12_result
| source_ml_source1_temporary_convertStringNumberRational_loop12_return : float -> source_ml_source1_temporary_convertStringNumberRational_loop12_result.

Definition source_ml_source1_temporary_convertStringNumberRational_loop12 (denominator : Z) (exponent : float) (scale : Z) : source_ml_source1_temporary_convertStringNumberRational_loop12_result :=
  ml_fix 64 (fun (ml_rec : Z * float * Z -> source_ml_source1_temporary_convertStringNumberRational_loop12_result) (ml_args : Z * float * Z) =>
    let '(denominator, exponent, scale) := ml_args in (if (Z.ltb scale denominator) then (let scale_2 := (Z.mul scale 2%Z) in (let exponent_2 := (PrimFloat.sub exponent 1%float) in (if (PrimFloat.ltb exponent_2 (PrimFloat.opp 1075%float)) then (source_ml_source1_temporary_convertStringNumberRational_loop12_return 0%float) else (ml_rec (denominator, exponent_2, scale_2))))) else (source_ml_source1_temporary_convertStringNumberRational_loop12_done exponent)))
    (fun _ => (source_ml_source1_temporary_convertStringNumberRational_loop12_done 0%float)) (denominator, exponent, scale).

Definition convertStringNumberRational (numerator : Z) (denominator : Z) : float :=
  (if (orb (Z.ltb numerator 0%Z) (Z.leb denominator 0%Z)) then PrimFloat.nan else (if (Z.eqb numerator 0%Z) then 0%float else (let exponent := 0%float in (let source_ml_source2_s4 := (if (Z.leb denominator numerator) then (let scale := denominator in (let source_ml_source2_s5 := (source_ml_source1_temporary_convertStringNumberRational_loop11 numerator exponent scale) in (match source_ml_source2_s5 with | source_ml_source1_temporary_convertStringNumberRational_loop11_done exponent_2 => (source_ml_source1_temporary_convertStringNumberRational_join5_next exponent_2) | source_ml_source1_temporary_convertStringNumberRational_loop11_return source_ml_source1_temporary_result11 => (source_ml_source1_temporary_convertStringNumberRational_join5_return source_ml_source1_temporary_result11) end))) else (let scale_2 := numerator in (let source_ml_source2_s6 := (source_ml_source1_temporary_convertStringNumberRational_loop12 denominator exponent scale_2) in (match source_ml_source2_s6 with | source_ml_source1_temporary_convertStringNumberRational_loop12_done exponent_3 => (source_ml_source1_temporary_convertStringNumberRational_join5_next exponent_3) | source_ml_source1_temporary_convertStringNumberRational_loop12_return source_ml_source1_temporary_result12 => (source_ml_source1_temporary_convertStringNumberRational_join5_return source_ml_source1_temporary_result12) end)))) in (match source_ml_source2_s4 with | source_ml_source1_temporary_convertStringNumberRational_join5_next exponent_4 => (let shift := (if (PrimFloat.leb (PrimFloat.opp 1022%float) exponent_4) then (PrimFloat.sub 52%float exponent_4) else 1074%float) in (let source_ml_source2_s7 := (if (PrimFloat.leb 0%float shift) then (let index := 0%float in (let numerator_2 := (source_ml_source1_temporary_convertStringNumberRational_loop9 numerator shift index) in (source_ml_source1_temporary_convertStringNumberRational_join6_next numerator_2 denominator))) else (let index_2 := 0%float in (let denominator_2 := (source_ml_source1_temporary_convertStringNumberRational_loop10 denominator shift index_2) in (source_ml_source1_temporary_convertStringNumberRational_join6_next numerator denominator_2)))) in (match source_ml_source2_s7 with | source_ml_source1_temporary_convertStringNumberRational_join6_next numerator_3 denominator_3 => (let quotient := 0%Z in (let remainder := numerator_3 in (let scale_3 := denominator_3 in (let weight := 1%Z in (let source_ml_source2_s8 := (source_ml_source1_temporary_convertStringNumberRational_loop7 remainder scale_3 weight) in (match source_ml_source2_s8 with | source_ml_source1_temporary_convertStringNumberRational_loop7_done scale_4 weight_2 => (let source_ml_source2_s9 := (source_ml_source1_temporary_convertStringNumberRational_loop8 quotient remainder scale_4 weight_2) in (match source_ml_source2_s9 with | source_ml_source1_temporary_convertStringNumberRational_loop8_done quotient_2 remainder_2 => (let twice := (Z.mul remainder_2 2%Z) in (let quotient_4 := (if (orb (Z.ltb denominator_3 twice) (andb (Z.eqb twice denominator_3) (Z.eqb (Z.rem quotient_2 2%Z) 1%Z))) then (let quotient_3 := (Z.add quotient_2 1%Z) in quotient_3) else quotient_2) in (PrimFloat.mul (readStringNumberExactMantissa quotient_4) (readStringNumberBinaryPower (PrimFloat.opp shift))))) end)) end)))))) end))) | source_ml_source1_temporary_convertStringNumberRational_join5_return source_ml_source1_temporary_result10 => source_ml_source1_temporary_result10 end))))).

Definition convertJavaScriptBigIntegerNumber (value : Z) : float :=
  (if (Z.ltb value 0%Z) then (PrimFloat.opp (convertStringNumberRational (Z.opp value) 1%Z)) else (convertStringNumberRational value 1%Z)).

Inductive source_ml_source1_temporary_acceptStringNumberInfinity_loop13_result : Type :=
| source_ml_source1_temporary_acceptStringNumberInfinity_loop13_done : source_ml_source1_temporary_acceptStringNumberInfinity_loop13_result
| source_ml_source1_temporary_acceptStringNumberInfinity_loop13_return : bool -> source_ml_source1_temporary_acceptStringNumberInfinity_loop13_result.

Definition source_ml_source1_temporary_acceptStringNumberInfinity_loop13 (units : (list float)) (start : float) (expected : (list float)) (index : float) : source_ml_source1_temporary_acceptStringNumberInfinity_loop13_result :=
  ml_fix 64 (fun (ml_rec : (list float) * float * (list float) * float -> source_ml_source1_temporary_acceptStringNumberInfinity_loop13_result) (ml_args : (list float) * float * (list float) * float) =>
    let '(units, start, expected, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length expected))))) then (if (negb (PrimFloat.eqb (match (let ml_index := (ml_float_index (PrimFloat.add start index)) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) (match (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error expected (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 2%float) end))) then (source_ml_source1_temporary_acceptStringNumberInfinity_loop13_return false) else (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (units, start, expected, index_2)))) else source_ml_source1_temporary_acceptStringNumberInfinity_loop13_done))
    (fun _ => source_ml_source1_temporary_acceptStringNumberInfinity_loop13_done) (units, start, expected, index).

Definition acceptStringNumberInfinity (units : (list float)) (start : float) (end_ : float) : bool :=
  (if (negb (PrimFloat.eqb (PrimFloat.sub end_ start) 8%float)) then false else (let expected := (73%float :: 110%float :: 102%float :: 105%float :: 110%float :: 105%float :: 116%float :: 121%float :: @nil float) in (let index := 0%float in (let source_ml_source2_s10 := (source_ml_source1_temporary_acceptStringNumberInfinity_loop13 units start expected index) in (match source_ml_source2_s10 with | source_ml_source1_temporary_acceptStringNumberInfinity_loop13_done => true | source_ml_source1_temporary_acceptStringNumberInfinity_loop13_return source_ml_source1_temporary_result13 => source_ml_source1_temporary_result13 end))))).

Definition source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop15 (units : (list float)) (start : float) (end_ : float) : float :=
  ml_fix 64 (fun (ml_rec : (list float) * float * float -> float) (ml_args : (list float) * float * float) =>
    let '(units, start, end_) := ml_args in (if (andb (PrimFloat.ltb start end_) (acceptJavaScriptWhitespaceUnit (match (let ml_index := (ml_float_index start) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end))) then (let start_2 := (PrimFloat.add start 1%float) in (ml_rec (units, start_2, end_))) else start))
    (fun _ => 0%float) (units, start, end_).

Definition source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop16 (units : (list float)) (start : float) (end_ : float) : float :=
  ml_fix 64 (fun (ml_rec : (list float) * float * float -> float) (ml_args : (list float) * float * float) =>
    let '(units, start, end_) := ml_args in (if (andb (PrimFloat.ltb start end_) (acceptJavaScriptWhitespaceUnit (match (let ml_index := (ml_float_index (PrimFloat.sub end_ 1%float)) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end))) then (let end_2 := (PrimFloat.sub end_ 1%float) in (ml_rec (units, start, end_2))) else end_))
    (fun _ => 0%float) (units, start, end_).

Definition source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop22 (units : (list float)) (integerDigits : (list Z)) (mantissaEnd : float) (mantissa : Z) (index : float) : Z :=
  ml_fix 64 (fun (ml_rec : (list float) * (list Z) * float * Z * float -> Z) (ml_args : (list float) * (list Z) * float * Z * float) =>
    let '(units, integerDigits, mantissaEnd, mantissa, index) := ml_args in (if (PrimFloat.ltb index mantissaEnd) then (let unit_ := (match (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (if (negb (PrimFloat.eqb unit_ 46%float)) then (let mantissa_2 := (Z.add (Z.mul mantissa 10%Z) (match (let ml_index := (ml_float_index (PrimFloat.sub unit_ 48%float)) in if Z.ltb ml_index 0 then None else List.nth_error integerDigits (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%Z end)) in (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (units, integerDigits, mantissaEnd, mantissa_2, index_2)))) else (let index_3 := (PrimFloat.add index 1%float) in (ml_rec (units, integerDigits, mantissaEnd, mantissa, index_3))))) else mantissa))
    (fun _ => 0%Z) (units, integerDigits, mantissaEnd, mantissa, index).

Inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21_result : Type :=
| source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21_done : float -> float -> source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21_result.

Definition source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21 (units : (list float)) (start : float) (exponent : float) (mantissaEnd : float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21_result :=
  ml_fix 64 (fun (ml_rec : (list float) * float * float * float -> source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21_result) (ml_args : (list float) * float * float * float) =>
    let '(units, start, exponent, mantissaEnd) := ml_args in (if (PrimFloat.ltb start mantissaEnd) then (let unit_ := (match (let ml_index := (ml_float_index (PrimFloat.sub mantissaEnd 1%float)) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (if (PrimFloat.eqb unit_ 46%float) then (let mantissaEnd_2 := (PrimFloat.sub mantissaEnd 1%float) in (ml_rec (units, start, exponent, mantissaEnd_2))) else (if (negb (PrimFloat.eqb unit_ 48%float)) then (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21_done exponent mantissaEnd) else (let mantissaEnd_3 := (PrimFloat.sub mantissaEnd 1%float) in (let exponent_2 := (PrimFloat.add exponent 1%float) in (ml_rec (units, start, exponent_2, mantissaEnd_3))))))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21_done exponent mantissaEnd)))
    (fun _ => (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21_done 0%float 0%float)) (units, start, exponent, mantissaEnd).

Inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20 : Type :=
| source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20_next : float -> float -> source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20
| source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20_return : float -> source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20.

Inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24_result : Type :=
| source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24_done : float -> float -> source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24_result.

Definition source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24 (units : (list float)) (end_ : float) (at_ : float) (exponent : float) (exponentLimit : float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24_result :=
  ml_fix 64 (fun (ml_rec : (list float) * float * float * float * float -> source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24_result) (ml_args : (list float) * float * float * float * float) =>
    let '(units, end_, at_, exponent, exponentLimit) := ml_args in (if (PrimFloat.ltb at_ end_) then (let digit := (PrimFloat.sub (match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) 48%float) in (if (orb (PrimFloat.ltb digit 0%float) (PrimFloat.ltb 9%float digit)) then (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24_done at_ exponent) else (let exponent_3 := (if (PrimFloat.ltb exponent exponentLimit) then (let exponent_2 := (ml_min exponentLimit (PrimFloat.add (PrimFloat.mul exponent 10%float) digit)) in exponent_2) else exponent) in (let at_2 := (PrimFloat.add at_ 1%float) in (ml_rec (units, end_, at_2, exponent_3, exponentLimit)))))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24_done at_ exponent)))
    (fun _ => (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24_done 0%float 0%float)) (units, end_, at_, exponent, exponentLimit).

Inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join23 : Type :=
| source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join23_next : float -> bool -> source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join23.

Inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19_result : Type :=
| source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19_done : float -> float -> float -> float -> source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19_result.

Definition source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19 (units : (list float)) (end_ : float) (at_ : float) (fraction : float) (digits : float) (significant : float) (dot : bool) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19_result :=
  ml_fix 64 (fun (ml_rec : (list float) * float * float * float * float * float * bool -> source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19_result) (ml_args : (list float) * float * float * float * float * float * bool) =>
    let '(units, end_, at_, fraction, digits, significant, dot) := ml_args in (if (PrimFloat.ltb at_ end_) then (let unit_ := (match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (if (andb (PrimFloat.leb 48%float unit_) (PrimFloat.leb unit_ 57%float)) then (let digits_2 := (PrimFloat.add digits 1%float) in (let fraction_3 := (if dot then (let fraction_2 := (PrimFloat.add fraction 1%float) in fraction_2) else fraction) in (let significant_3 := (if (orb (negb (PrimFloat.eqb unit_ 48%float)) (PrimFloat.ltb 0%float significant)) then (let significant_2 := (PrimFloat.add significant 1%float) in significant_2) else significant) in (let at_2 := (PrimFloat.add at_ 1%float) in (ml_rec (units, end_, at_2, fraction_3, digits_2, significant_3, dot)))))) else (if (andb (PrimFloat.eqb unit_ 46%float) (negb dot)) then (let dot_2 := true in (let at_3 := (PrimFloat.add at_ 1%float) in (ml_rec (units, end_, at_3, fraction, digits, significant, dot_2)))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19_done at_ fraction digits significant)))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19_done at_ fraction digits significant)))
    (fun _ => (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19_done 0%float 0%float 0%float 0%float)) (units, end_, at_, fraction, digits, significant, dot).

Inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18 : Type :=
| source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18_next : float -> source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18
| source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18_return : float -> source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18.

Inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_result : Type :=
| source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_done : Z -> bool -> source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_result
| source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_return : float -> source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_result.

Definition source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25 (units : (list float)) (end_ : float) (integerDigits : (list Z)) (radix : float) (value : Z) (overflow : bool) (limit : Z) (base : Z) (index : float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_result :=
  ml_fix 64 (fun (ml_rec : (list float) * float * (list Z) * float * Z * bool * Z * Z * float -> source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_result) (ml_args : (list float) * float * (list Z) * float * Z * bool * Z * Z * float) =>
    let '(units, end_, integerDigits, radix, value, overflow, limit, base, index) := ml_args in (if (PrimFloat.ltb index end_) then (let digit := (readStringNumberDigit (match (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end)) in (if (orb (PrimFloat.ltb digit 0%float) (PrimFloat.leb radix digit)) then (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_return PrimFloat.nan) else (if (negb overflow) then (let value_2 := (Z.add (Z.mul value base) (match (let ml_index := (ml_float_index digit) in if Z.ltb ml_index 0 then None else List.nth_error integerDigits (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%Z end)) in (let overflow_2 := (Z.ltb limit value_2) in (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (units, end_, integerDigits, radix, value_2, overflow_2, limit, base, index_2))))) else (let index_3 := (PrimFloat.add index 1%float) in (ml_rec (units, end_, integerDigits, radix, value, overflow, limit, base, index_3)))))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_done value overflow)))
    (fun _ => (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_done 0%Z false)) (units, end_, integerDigits, radix, value, overflow, limit, base, index).

Inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join17 : Type :=
| source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join17_next : float -> bool -> bool -> source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join17.

Inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_result : Type :=
| source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_done : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_result
| source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_return : float -> source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_result.

Definition source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14 (units : (list float)) (index : float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_result :=
  ml_fix 64 (fun (ml_rec : (list float) * float -> source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_result) (ml_args : (list float) * float) =>
    let '(units, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length units))))) then (let unit_ := (match (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (if (orb (orb (negb (ml_is_safe_integer unit_)) (PrimFloat.ltb unit_ 0%float)) (PrimFloat.ltb 65535%float unit_)) then (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_return PrimFloat.nan) else (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (units, index_2))))) else source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_done))
    (fun _ => source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_done) (units, index).

Definition convertJavaScriptStringNumberUnits (units : (list float)) : float :=
  (let index := 0%float in (let source_ml_source2_s11 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14 units index) in (match source_ml_source2_s11 with | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_done => (let start := 0%float in (let end_ := (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length units)))) in (let start_2 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop15 units start end_) in (let end_2 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop16 units start_2 end_) in (if (PrimFloat.eqb start_2 end_2) then 0%float else (let negative := false in (let signed := false in (let sign := (match (let ml_index := (ml_float_index start_2) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (let source_ml_source2_s12 := (if (orb (PrimFloat.eqb sign 43%float) (PrimFloat.eqb sign 45%float)) then (let negative_2 := (PrimFloat.eqb sign 45%float) in (let signed_2 := true in (let start_3 := (PrimFloat.add start_2 1%float) in (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join17_next start_3 negative_2 signed_2)))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join17_next start_2 negative signed)) in (match source_ml_source2_s12 with | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join17_next start_4 negative_3 signed_3 => (if (PrimFloat.eqb start_4 end_2) then PrimFloat.nan else (if (acceptStringNumberInfinity units start_4 end_2) then (if negative_3 then PrimFloat.neg_infinity else PrimFloat.infinity) else (let integerDigits := (0%Z :: 1%Z :: 2%Z :: 3%Z :: 4%Z :: 5%Z :: 6%Z :: 7%Z :: 8%Z :: 9%Z :: 10%Z :: 11%Z :: 12%Z :: 13%Z :: 14%Z :: 15%Z :: @nil Z) in (let source_ml_source2_s14 := (if (andb (andb (negb signed_3) (PrimFloat.eqb (match (let ml_index := (ml_float_index start_4) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) 48%float)) (PrimFloat.leb 2%float (PrimFloat.sub end_2 start_4))) then (let marker := (match (let ml_index := (ml_float_index (PrimFloat.add start_4 1%float)) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (let radix := (if (orb (PrimFloat.eqb marker 120%float) (PrimFloat.eqb marker 88%float)) then 16%float else (if (orb (PrimFloat.eqb marker 98%float) (PrimFloat.eqb marker 66%float)) then 2%float else (if (orb (PrimFloat.eqb marker 111%float) (PrimFloat.eqb marker 79%float)) then 8%float else 0%float))) in (if (negb (PrimFloat.eqb radix 0%float)) then (let value := 0%Z in (let overflow := false in (let limit := (readStringNumberDecimalPower 310%float) in (let base := (if (PrimFloat.eqb radix 16%float) then 16%Z else (if (PrimFloat.eqb radix 8%float) then 8%Z else 2%Z)) in (let start_5 := (PrimFloat.add start_4 2%float) in (if (PrimFloat.eqb start_5 end_2) then (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18_return PrimFloat.nan) else (let index_2 := start_5 in (let source_ml_source2_s15 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25 units end_2 integerDigits radix value overflow limit base index_2) in (match source_ml_source2_s15 with | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_done value_2 overflow_2 => (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18_return (if overflow_2 then PrimFloat.infinity else (convertStringNumberRational value_2 1%Z))) | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_return source_ml_source1_temporary_result25 => (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18_return source_ml_source1_temporary_result25) end))))))))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18_next start_4)))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18_next start_4)) in (match source_ml_source2_s14 with | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18_next start_6 => (let at_ := start_6 in (let fraction := 0%float in (let digits := 0%float in (let significant := 0%float in (let dot := false in (let source_ml_source2_s16 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19 units end_2 at_ fraction digits significant dot) in (match source_ml_source2_s16 with | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19_done at_2 fraction_2 digits_2 significant_2 => (if (PrimFloat.eqb digits_2 0%float) then PrimFloat.nan else (let decimalEnd := at_2 in (let exponent := 0%float in (let exponentLimit := (PrimFloat.add (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length units)))) 10000%float) in (let source_ml_source2_s19 := (if (andb (PrimFloat.ltb at_2 end_2) (orb (PrimFloat.eqb (match (let ml_index := (ml_float_index at_2) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) 101%float) (PrimFloat.eqb (match (let ml_index := (ml_float_index at_2) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) 69%float))) then (let at_3 := (PrimFloat.add at_2 1%float) in (let exponentNegative := false in (let exponentSign := (match (let ml_index := (ml_float_index at_3) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (let source_ml_source2_s20 := (if (orb (PrimFloat.eqb exponentSign 43%float) (PrimFloat.eqb exponentSign 45%float)) then (let exponentNegative_2 := (PrimFloat.eqb exponentSign 45%float) in (let at_4 := (PrimFloat.add at_3 1%float) in (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join23_next at_4 exponentNegative_2))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join23_next at_3 exponentNegative)) in (match source_ml_source2_s20 with | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join23_next at_5 exponentNegative_3 => (let first := at_5 in (let source_ml_source2_s21 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24 units end_2 at_5 exponent exponentLimit) in (match source_ml_source2_s21 with | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24_done at_6 exponent_2 => (if (PrimFloat.eqb at_6 first) then (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20_return PrimFloat.nan) else (if exponentNegative_3 then (let exponent_3 := (PrimFloat.opp exponent_2) in (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20_next at_6 exponent_3)) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20_next at_6 exponent_2))) end))) end))))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20_next at_2 exponent)) in (match source_ml_source2_s19 with | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20_next at_7 exponent_4 => (if (negb (PrimFloat.eqb at_7 end_2)) then PrimFloat.nan else (if (PrimFloat.eqb significant_2 0%float) then (if negative_3 then (PrimFloat.opp 0%float) else 0%float) else (let exponent_5 := (PrimFloat.sub exponent_4 fraction_2) in (let order := (PrimFloat.add significant_2 exponent_5) in (if (PrimFloat.ltb 310%float order) then (if negative_3 then PrimFloat.neg_infinity else PrimFloat.infinity) else (if (PrimFloat.ltb order (PrimFloat.opp 325%float)) then (if negative_3 then (PrimFloat.opp 0%float) else 0%float) else (let mantissaEnd := decimalEnd in (let source_ml_source2_s22 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21 units start_6 exponent_5 mantissaEnd) in (match source_ml_source2_s22 with | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21_done exponent_6 mantissaEnd_2 => (let mantissa := 0%Z in (let index_3 := start_6 in (let mantissa_2 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop22 units integerDigits mantissaEnd_2 mantissa index_3) in (let numerator := (if (PrimFloat.leb 0%float exponent_6) then (Z.mul mantissa_2 (readStringNumberDecimalPower exponent_6)) else mantissa_2) in (let denominator := (if (PrimFloat.ltb exponent_6 0%float) then (readStringNumberDecimalPower (PrimFloat.opp exponent_6)) else 1%Z) in (let value_3 := (convertStringNumberRational numerator denominator) in (if negative_3 then (PrimFloat.opp value_3) else value_3))))))) end))))))))) | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20_return source_ml_source1_temporary_result22 => source_ml_source1_temporary_result22 end)))))) end))))))) | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18_return source_ml_source1_temporary_result24 => source_ml_source1_temporary_result24 end))))) end)))))))))) | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_return source_ml_source1_temporary_result25_2 => source_ml_source1_temporary_result25_2 end))).

Inductive source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_result : Type :=
| source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_done : (list float) -> source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_result
| source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_return : float -> source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_result.

Inductive source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_result : Type :=
| source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_done : float -> float -> source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_result
| source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_return : float -> source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_result.

Definition source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29 (bytes : (list float)) (index : float) (scalar : float) (remaining : float) : source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_result :=
  ml_fix 64 (fun (ml_rec : (list float) * float * float * float -> source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_result) (ml_args : (list float) * float * float * float) =>
    let '(bytes, index, scalar, remaining) := ml_args in (if (PrimFloat.ltb 0%float remaining) then (let next := (match (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error bytes (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (if (orb (PrimFloat.ltb next 128%float) (PrimFloat.ltb 191%float next)) then (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_return PrimFloat.nan) else (let scalar_2 := (PrimFloat.sub (PrimFloat.add (PrimFloat.mul scalar 64%float) next) 128%float) in (let index_2 := (PrimFloat.add index 1%float) in (let remaining_2 := (PrimFloat.sub remaining 1%float) in (ml_rec (bytes, index_2, scalar_2, remaining_2))))))) else (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_done index scalar)))
    (fun _ => (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_done 0%float 0%float)) (bytes, index, scalar, remaining).

Inductive source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28 : Type :=
| source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28_next : float -> float -> float -> source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28
| source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28_return : float -> source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28.

Definition source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27 (bytes : (list float)) (units : (list float)) (index : float) : source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_result :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float -> source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_result) (ml_args : (list float) * (list float) * float) =>
    let '(bytes, units, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length bytes))))) then (let scalar := (match (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error bytes (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (let index_2 := (PrimFloat.add index 1%float) in (let remaining := 0%float in (let minimum := 0%float in (let source_ml_source2_s25 := (if (andb (PrimFloat.leb 194%float scalar) (PrimFloat.leb scalar 223%float)) then (let scalar_2 := (PrimFloat.sub scalar 192%float) in (let remaining_2 := 1%float in (let minimum_2 := 128%float in (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28_next scalar_2 remaining_2 minimum_2)))) else (if (andb (PrimFloat.leb 224%float scalar) (PrimFloat.leb scalar 239%float)) then (let scalar_3 := (PrimFloat.sub scalar 224%float) in (let remaining_3 := 2%float in (let minimum_3 := 2048%float in (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28_next scalar_3 remaining_3 minimum_3)))) else (if (andb (PrimFloat.leb 240%float scalar) (PrimFloat.leb scalar 244%float)) then (let scalar_4 := (PrimFloat.sub scalar 240%float) in (let remaining_4 := 3%float in (let minimum_4 := 65536%float in (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28_next scalar_4 remaining_4 minimum_4)))) else (if (PrimFloat.leb 128%float scalar) then (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28_return PrimFloat.nan) else (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28_next scalar remaining minimum))))) in (match source_ml_source2_s25 with | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28_next scalar_5 remaining_5 minimum_5 => (let source_ml_source2_s26 := (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29 bytes index_2 scalar_5 remaining_5) in (match source_ml_source2_s26 with | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_done index_3 scalar_6 => (if (orb (orb (PrimFloat.ltb scalar_6 minimum_5) (PrimFloat.ltb 1114111%float scalar_6)) (andb (PrimFloat.leb 55296%float scalar_6) (PrimFloat.leb scalar_6 57343%float))) then (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_return PrimFloat.nan) else (if (PrimFloat.leb scalar_6 65535%float) then (let units_2 := (List.app units (scalar_6 :: @nil float)) in (ml_rec (bytes, units_2, index_3))) else (let units_3 := (List.app units ((PrimFloat.add 55296%float (ml_floor (PrimFloat.div (PrimFloat.sub scalar_6 65536%float) 1024%float))) :: @nil float)) in (let units_4 := (List.app units_3 ((PrimFloat.add 56320%float (ml_float_rem (PrimFloat.sub scalar_6 65536%float) 1024%float)) :: @nil float)) in (ml_rec (bytes, units_4, index_3)))))) | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_return source_ml_source1_temporary_result29 => (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_return source_ml_source1_temporary_result29) end)) | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28_return source_ml_source1_temporary_result29_2 => (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_return source_ml_source1_temporary_result29_2) end)))))) else (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_done units)))
    (fun _ => (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_done (@nil float))) (bytes, units, index).

Inductive source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_result : Type :=
| source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_done : source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_result
| source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_return : float -> source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_result.

Definition source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26 (bytes : (list float)) (index : float) : source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_result :=
  ml_fix 64 (fun (ml_rec : (list float) * float -> source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_result) (ml_args : (list float) * float) =>
    let '(bytes, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length bytes))))) then (let unit_ := (match (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error bytes (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (if (orb (orb (negb (ml_is_safe_integer unit_)) (PrimFloat.ltb unit_ 0%float)) (PrimFloat.ltb 255%float unit_)) then (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_return PrimFloat.nan) else (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (bytes, index_2))))) else source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_done))
    (fun _ => source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_done) (bytes, index).

Definition convertJavaScriptUtf8StringNumberUnits (bytes : (list float)) : float :=
  (let index := 0%float in (let source_ml_source2_s23 := (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26 bytes index) in (match source_ml_source2_s23 with | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_done => (let units := (@nil float) in (let index_2 := 0%float in (let source_ml_source2_s24 := (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27 bytes units index_2) in (match source_ml_source2_s24 with | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_done units_2 => (convertJavaScriptStringNumberUnits units_2) | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_return source_ml_source1_temporary_result27 => source_ml_source1_temporary_result27 end)))) | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_return source_ml_source1_temporary_result29 => source_ml_source1_temporary_result29 end))).

Definition main : list string :=
  nil.

Eval vm_compute in main.

Definition convertUnits := convertJavaScriptStringNumberUnits.
Definition convertInteger := convertJavaScriptBigIntegerNumber.
Definition convertUtf8 := convertJavaScriptUtf8StringNumberUnits.
End MLStringNumber.
