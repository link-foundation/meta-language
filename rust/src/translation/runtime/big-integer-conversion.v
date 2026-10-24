From Stdlib Require Import NArith ZArith Lia Recdef String List Ascii Floats.
Module MLBigIntegerConversion.
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

Definition readJavaScriptNumericDigitUnit (unit_ : float) : float :=
  (if (andb (PrimFloat.leb 48%float unit_) (PrimFloat.leb unit_ 57%float)) then (PrimFloat.sub unit_ 48%float) else (if (andb (PrimFloat.leb 65%float unit_) (PrimFloat.leb unit_ 70%float)) then (PrimFloat.sub unit_ 55%float) else (if (andb (PrimFloat.leb 97%float unit_) (PrimFloat.leb unit_ 102%float)) then (PrimFloat.sub unit_ 87%float) else (PrimFloat.opp 1%float)))).

Definition convertJavaScriptBooleanBigInteger (value : bool) : Z :=
  (if value then 1%Z else 0%Z).

Definition readJavaScriptBigIntegerConversionForm (kind : string) : string :=
  (if (orb (String.eqb kind "int"%string) (String.eqb kind "nat"%string)) then "identity"%string else (if (String.eqb kind "float"%string) then "number"%string else (if (String.eqb kind "string"%string) then "string"%string else (if (String.eqb kind "bool"%string) then "boolean"%string else (if (String.eqb kind "unit"%string) then "undefined"%string else (if (String.eqb kind "option"%string) then "optional"%string else "unsupported"%string)))))).

Definition readJavaScriptBigIntegerConversionErrorClass (form : string) : string :=
  (if (String.eqb form "string"%string) then "SyntaxError"%string else (if (String.eqb form "number"%string) then "RangeError"%string else (if (orb (String.eqb form "undefined"%string) (String.eqb form "null"%string)) then "TypeError"%string else ""%string))).

Definition readJavaScriptBigIntegerConversionMessagePieces (form : string) : (list string) :=
  (if (String.eqb form "number"%string) then ("The number "%string :: " cannot be converted to a BigInt because it is not an integer"%string :: @nil string) else (if (orb (orb (String.eqb form "string"%string) (String.eqb form "undefined"%string)) (String.eqb form "null"%string)) then ("Cannot convert "%string :: " to a BigInt"%string :: @nil string) else (@nil string))).

Definition renderJavaScriptBigIntegerConversionMessage (form : string) (value : string) : string :=
  (let pieces := (readJavaScriptBigIntegerConversionMessagePieces form) in (if (PrimFloat.eqb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length pieces)))) 2%float) then (String.append (String.append (match (let ml_index := (ml_float_index 0%float) in if Z.ltb ml_index 0 then None else List.nth_error pieces (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => ""%string end) value) (match (let ml_index := (ml_float_index 1%float) in if Z.ltb ml_index 0 then None else List.nth_error pieces (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => ""%string end)) else ""%string)).

Definition source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop2 (units : (list float)) (start : float) (end_ : float) : float :=
  ml_fix 64 (fun (ml_rec : (list float) * float * float -> float) (ml_args : (list float) * float * float) =>
    let '(units, start, end_) := ml_args in (if (andb (PrimFloat.ltb start end_) (acceptJavaScriptWhitespaceUnit (match (let ml_index := (ml_float_index start) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end))) then (let start_2 := (PrimFloat.add start 1%float) in (ml_rec (units, start_2, end_))) else start))
    (fun _ => 0%float) (units, start, end_).

Definition source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop3 (units : (list float)) (start : float) (end_ : float) : float :=
  ml_fix 64 (fun (ml_rec : (list float) * float * float -> float) (ml_args : (list float) * float * float) =>
    let '(units, start, end_) := ml_args in (if (andb (PrimFloat.ltb start end_) (acceptJavaScriptWhitespaceUnit (match (let ml_index := (ml_float_index (PrimFloat.sub end_ 1%float)) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end))) then (let end_2 := (PrimFloat.sub end_ 1%float) in (ml_rec (units, start, end_2))) else end_))
    (fun _ => 0%float) (units, start, end_).

Inductive source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_result : Type :=
| source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_done : Z -> source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_result
| source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_return : (list Z) -> source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_result.

Definition source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6 (units : (list float)) (end_ : float) (radix : float) (base : Z) (digits : (list Z)) (value : Z) (index : float) : source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_result :=
  ml_fix 64 (fun (ml_rec : (list float) * float * float * Z * (list Z) * Z * float -> source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_result) (ml_args : (list float) * float * float * Z * (list Z) * Z * float) =>
    let '(units, end_, radix, base, digits, value, index) := ml_args in (if (PrimFloat.ltb index end_) then (let digit := (readJavaScriptNumericDigitUnit (match (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end)) in (if (orb (PrimFloat.ltb digit 0%float) (PrimFloat.leb radix digit)) then (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_return (@nil Z)) else (let value_2 := (Z.add (Z.mul value base) (match (let ml_index := (ml_float_index digit) in if Z.ltb ml_index 0 then None else List.nth_error digits (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%Z end)) in (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (units, end_, radix, base, digits, value_2, index_2)))))) else (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_done value)))
    (fun _ => (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_done 0%Z)) (units, end_, radix, base, digits, value, index).

Inductive source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join5 : Type :=
| source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join5_next : float -> float -> Z -> source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join5.

Inductive source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join4 : Type :=
| source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join4_next : float -> bool -> bool -> source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join4.

Inductive source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_result : Type :=
| source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_done : source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_result
| source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_return : (list Z) -> source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_result.

Definition source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1 (units : (list float)) (index : float) : source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_result :=
  ml_fix 64 (fun (ml_rec : (list float) * float -> source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_result) (ml_args : (list float) * float) =>
    let '(units, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length units))))) then (let unit_ := (match (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (if (orb (orb (negb (ml_is_safe_integer unit_)) (PrimFloat.ltb unit_ 0%float)) (PrimFloat.ltb 65535%float unit_)) then (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_return (@nil Z)) else (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (units, index_2))))) else source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_done))
    (fun _ => source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_done) (units, index).

Definition convertJavaScriptStringBigIntegerUnits (units : (list float)) : (list Z) :=
  (let index := 0%float in (let source_ml_source1_s1 := (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1 units index) in (match source_ml_source1_s1 with | source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_done => (let start := 0%float in (let end_ := (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length units)))) in (let start_2 := (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop2 units start end_) in (let end_2 := (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop3 units start_2 end_) in (if (PrimFloat.eqb start_2 end_2) then (0%Z :: @nil Z) else (let negative := false in (let signed := false in (let sign := (match (let ml_index := (ml_float_index start_2) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (let source_ml_source1_s2 := (if (orb (PrimFloat.eqb sign 43%float) (PrimFloat.eqb sign 45%float)) then (let negative_2 := (PrimFloat.eqb sign 45%float) in (let signed_2 := true in (let start_3 := (PrimFloat.add start_2 1%float) in (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join4_next start_3 negative_2 signed_2)))) else (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join4_next start_2 negative signed)) in (match source_ml_source1_s2 with | source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join4_next start_4 negative_3 signed_3 => (if (PrimFloat.eqb start_4 end_2) then (@nil Z) else (let radix := 10%float in (let base := 10%Z in (let source_ml_source1_s3 := (if (andb (andb (negb signed_3) (PrimFloat.eqb (match (let ml_index := (ml_float_index start_4) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) 48%float)) (PrimFloat.leb 2%float (PrimFloat.sub end_2 start_4))) then (let marker := (match (let ml_index := (ml_float_index (PrimFloat.add start_4 1%float)) in if Z.ltb ml_index 0 then None else List.nth_error units (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (let selected := (if (orb (PrimFloat.eqb marker 120%float) (PrimFloat.eqb marker 88%float)) then 16%float else (if (orb (PrimFloat.eqb marker 98%float) (PrimFloat.eqb marker 66%float)) then 2%float else (if (orb (PrimFloat.eqb marker 111%float) (PrimFloat.eqb marker 79%float)) then 8%float else 0%float))) in (if (negb (PrimFloat.eqb selected 0%float)) then (let radix_2 := selected in (let base_2 := (if (PrimFloat.eqb selected 16%float) then 16%Z else (if (PrimFloat.eqb selected 8%float) then 8%Z else 2%Z)) in (let start_5 := (PrimFloat.add start_4 2%float) in (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join5_next start_5 radix_2 base_2)))) else (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join5_next start_4 radix base)))) else (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join5_next start_4 radix base)) in (match source_ml_source1_s3 with | source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join5_next start_6 radix_3 base_3 => (if (PrimFloat.eqb start_6 end_2) then (@nil Z) else (let digits := (0%Z :: 1%Z :: 2%Z :: 3%Z :: 4%Z :: 5%Z :: 6%Z :: 7%Z :: 8%Z :: 9%Z :: 10%Z :: 11%Z :: 12%Z :: 13%Z :: 14%Z :: 15%Z :: @nil Z) in (let value := 0%Z in (let index_2 := start_6 in (let source_ml_source1_s4 := (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6 units end_2 radix_3 base_3 digits value index_2) in (match source_ml_source1_s4 with | source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_done value_2 => ((if negative_3 then (Z.opp value_2) else value_2) :: @nil Z) | source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_return source_ml_temporary_result6 => source_ml_temporary_result6 end)))))) end))))) end)))))))))) | source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_return source_ml_temporary_result6_2 => source_ml_temporary_result6_2 end))).

Definition source_ml_temporary_convertJavaScriptNumberBigInteger_loop7 (value : float) (integer : Z) (weight : Z) : Z :=
  ml_fix 64 (fun (ml_rec : float * Z * Z -> Z) (ml_args : float * Z * Z) =>
    let '(value, integer, weight) := ml_args in (if (PrimFloat.leb 1%float value) then (let integer_3 := (if (PrimFloat.eqb (ml_float_rem value 2%float) 1%float) then (let integer_2 := (Z.add integer weight) in integer_2) else integer) in (let value_2 := (ml_floor (PrimFloat.div value 2%float)) in (let weight_2 := (Z.mul weight 2%Z) in (ml_rec (value_2, integer_3, weight_2))))) else integer))
    (fun _ => 0%Z) (value, integer, weight).

Definition convertJavaScriptNumberBigInteger (value : float) : (list Z) :=
  (if (negb (ml_is_integer value)) then (@nil Z) else (let negative := (PrimFloat.ltb value 0%float) in (let value_2 := (PrimFloat.abs value) in (let integer := 0%Z in (let weight := 1%Z in (let integer_2 := (source_ml_temporary_convertJavaScriptNumberBigInteger_loop7 value_2 integer weight) in ((if negative then (Z.opp integer_2) else integer_2) :: @nil Z))))))).

Definition main : list string :=
  nil.

Eval vm_compute in main.

Definition convertUnits := convertJavaScriptStringBigIntegerUnits.
Definition convertNumber := convertJavaScriptNumberBigInteger.
Definition convertBoolean := convertJavaScriptBooleanBigInteger.
End MLBigIntegerConversion.
