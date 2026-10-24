From Stdlib Require Import NArith ZArith Lia Recdef String List Ascii Floats.
Module MLUtf16Text.
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

Definition normalizeJavaScriptUnsignedInteger (value : float) : float :=
  (if (orb (negb (ml_is_finite value)) (PrimFloat.eqb value 0%float)) then 0%float else (let integral := (if (PrimFloat.ltb value 0%float) then (PrimFloat.opp (ml_floor (PrimFloat.opp value))) else (ml_floor value)) in (let remainder := (ml_float_rem integral 4294967296%float) in (if (PrimFloat.ltb remainder 0%float) then (PrimFloat.add remainder 4294967296%float) else (if (PrimFloat.eqb remainder 0%float) then 0%float else remainder))))).

Definition readUtf16TextInteger (value : float) : float :=
  (if (PrimFloat.is_nan value) then 0%float else (ml_trunc value)).

Definition readUtf16TextSlicePosition (value : float) (length : float) : float :=
  (let integer := (readUtf16TextInteger value) in (if (PrimFloat.ltb integer 0%float) then (ml_max (PrimFloat.add length integer) 0%float) else (ml_min integer length))).

Definition characterUtf16Text (text : (list float)) (position : float) : (list float) :=
  (let at_ := (readUtf16TextInteger position) in (if (orb (PrimFloat.ltb at_ 0%float) (PrimFloat.leb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text)))) at_)) then (@nil float) else ((match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error text (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%float end) :: @nil float))).

Definition relativeCharacterUtf16Text (text : (list float)) (position : float) : (list (list float)) :=
  (let integer := (readUtf16TextInteger position) in (let at_ := (if (PrimFloat.ltb integer 0%float) then (PrimFloat.add (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text)))) integer) else integer) in (if (orb (PrimFloat.ltb at_ 0%float) (PrimFloat.leb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text)))) at_)) then (@nil (list float)) else (((match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error text (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%float end) :: @nil float) :: @nil (list float))))).

Definition codePointUtf16Text (text : (list float)) (position : float) : (list float) :=
  (let at_ := (readUtf16TextInteger position) in (if (orb (PrimFloat.ltb at_ 0%float) (PrimFloat.leb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text)))) at_)) then (@nil float) else (let first := (match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error text (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%float end) in (let second := (match (let ml_index := (ml_float_index (PrimFloat.add at_ 1%float)) in if Z.ltb ml_index 0 then None else List.nth_error text (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in ((if (andb (andb (andb (PrimFloat.leb 55296%float first) (PrimFloat.leb first 56319%float)) (PrimFloat.leb 56320%float second)) (PrimFloat.leb second 57343%float)) then (PrimFloat.sub (PrimFloat.add (PrimFloat.add 65536%float (PrimFloat.mul (PrimFloat.sub first 55296%float) 1024%float)) second) 56320%float) else first) :: @nil float))))).

Inductive source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_result : Type :=
| source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_done : (list float) -> source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_result
| source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_return : (list float) -> source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_result.

Inductive source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_result : Type :=
| source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_done : float -> float -> source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_result
| source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_return : (list float) -> source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_result.

Definition source_ml_temporary_readJavaScriptUtf8StringUnits_loop4 (bytes : (list float)) (index : float) (scalar : float) (remaining : float) : source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_result :=
  ml_fix 64 (fun (ml_rec : (list float) * float * float * float -> source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_result) (ml_args : (list float) * float * float * float) =>
    let '(bytes, index, scalar, remaining) := ml_args in (if (PrimFloat.ltb 0%float remaining) then (let next := (match (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error bytes (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (if (orb (PrimFloat.ltb next 128%float) (PrimFloat.ltb 191%float next)) then (source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_return ((PrimFloat.opp 1%float) :: @nil float)) else (let scalar_2 := (PrimFloat.sub (PrimFloat.add (PrimFloat.mul scalar 64%float) next) 128%float) in (let index_2 := (PrimFloat.add index 1%float) in (let remaining_2 := (PrimFloat.sub remaining 1%float) in (ml_rec (bytes, index_2, scalar_2, remaining_2))))))) else (source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_done index scalar)))
    (fun _ => (source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_done 0%float 0%float)) (bytes, index, scalar, remaining).

Inductive source_ml_temporary_readJavaScriptUtf8StringUnits_join3 : Type :=
| source_ml_temporary_readJavaScriptUtf8StringUnits_join3_next : float -> float -> float -> source_ml_temporary_readJavaScriptUtf8StringUnits_join3
| source_ml_temporary_readJavaScriptUtf8StringUnits_join3_return : (list float) -> source_ml_temporary_readJavaScriptUtf8StringUnits_join3.

Definition source_ml_temporary_readJavaScriptUtf8StringUnits_loop2 (bytes : (list float)) (units : (list float)) (index : float) : source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_result :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float -> source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_result) (ml_args : (list float) * (list float) * float) =>
    let '(bytes, units, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length bytes))))) then (let scalar := (match (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error bytes (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (let index_2 := (PrimFloat.add index 1%float) in (let remaining := 0%float in (let minimum := 0%float in (let source_ml_source1_s11 := (if (andb (PrimFloat.leb 194%float scalar) (PrimFloat.leb scalar 223%float)) then (let scalar_2 := (PrimFloat.sub scalar 192%float) in (let remaining_2 := 1%float in (let minimum_2 := 128%float in (source_ml_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_2 remaining_2 minimum_2)))) else (if (andb (PrimFloat.leb 224%float scalar) (PrimFloat.leb scalar 239%float)) then (let scalar_3 := (PrimFloat.sub scalar 224%float) in (let remaining_3 := 2%float in (let minimum_3 := 2048%float in (source_ml_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_3 remaining_3 minimum_3)))) else (if (andb (PrimFloat.leb 240%float scalar) (PrimFloat.leb scalar 244%float)) then (let scalar_4 := (PrimFloat.sub scalar 240%float) in (let remaining_4 := 3%float in (let minimum_4 := 65536%float in (source_ml_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_4 remaining_4 minimum_4)))) else (if (PrimFloat.leb 128%float scalar) then (source_ml_temporary_readJavaScriptUtf8StringUnits_join3_return ((PrimFloat.opp 1%float) :: @nil float)) else (source_ml_temporary_readJavaScriptUtf8StringUnits_join3_next scalar remaining minimum))))) in (match source_ml_source1_s11 with | source_ml_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_5 remaining_5 minimum_5 => (let source_ml_source1_s12 := (source_ml_temporary_readJavaScriptUtf8StringUnits_loop4 bytes index_2 scalar_5 remaining_5) in (match source_ml_source1_s12 with | source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_done index_3 scalar_6 => (if (orb (orb (PrimFloat.ltb scalar_6 minimum_5) (PrimFloat.ltb 1114111%float scalar_6)) (andb (PrimFloat.leb 55296%float scalar_6) (PrimFloat.leb scalar_6 57343%float))) then (source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_return ((PrimFloat.opp 1%float) :: @nil float)) else (if (PrimFloat.leb scalar_6 65535%float) then (let units_2 := (List.app units (scalar_6 :: @nil float)) in (ml_rec (bytes, units_2, index_3))) else (let units_3 := (List.app units ((PrimFloat.add 55296%float (ml_floor (PrimFloat.div (PrimFloat.sub scalar_6 65536%float) 1024%float))) :: @nil float)) in (let units_4 := (List.app units_3 ((PrimFloat.add 56320%float (ml_float_rem (PrimFloat.sub scalar_6 65536%float) 1024%float)) :: @nil float)) in (ml_rec (bytes, units_4, index_3)))))) | source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_return source_ml_temporary_result4 => (source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_return source_ml_temporary_result4) end)) | source_ml_temporary_readJavaScriptUtf8StringUnits_join3_return source_ml_temporary_result4_2 => (source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_return source_ml_temporary_result4_2) end)))))) else (source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_done units)))
    (fun _ => (source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_done (@nil float))) (bytes, units, index).

Inductive source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_result : Type :=
| source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_done : source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_result
| source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_return : (list float) -> source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_result.

Definition source_ml_temporary_readJavaScriptUtf8StringUnits_loop1 (bytes : (list float)) (index : float) : source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_result :=
  ml_fix 64 (fun (ml_rec : (list float) * float -> source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_result) (ml_args : (list float) * float) =>
    let '(bytes, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length bytes))))) then (let unit_ := (match (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error bytes (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (if (orb (orb (negb (ml_is_safe_integer unit_)) (PrimFloat.ltb unit_ 0%float)) (PrimFloat.ltb 255%float unit_)) then (source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_return ((PrimFloat.opp 1%float) :: @nil float)) else (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (bytes, index_2))))) else source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_done))
    (fun _ => source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_done) (bytes, index).

Definition readJavaScriptUtf8StringUnits (bytes : (list float)) : (list float) :=
  (let index := 0%float in (let source_ml_source1_s1 := (source_ml_temporary_readJavaScriptUtf8StringUnits_loop1 bytes index) in (match source_ml_source1_s1 with | source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_done => (let units := (@nil float) in (let index_2 := 0%float in (let source_ml_source1_s2 := (source_ml_temporary_readJavaScriptUtf8StringUnits_loop2 bytes units index_2) in (match source_ml_source1_s2 with | source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_done units_2 => units_2 | source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_return source_ml_temporary_result2 => source_ml_temporary_result2 end)))) | source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_return source_ml_temporary_result4 => source_ml_temporary_result4 end))).

Definition readUtf8TextUnits (bytes : (list float)) : (list float) :=
  (readJavaScriptUtf8StringUnits bytes).

Definition source_ml_temporary_sliceUtf16Text_loop5 (text : (list float)) (last : float) (result : (list float)) (at_ : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list float) * float * (list float) * float -> (list float)) (ml_args : (list float) * float * (list float) * float) =>
    let '(text, last, result, at_) := ml_args in (if (PrimFloat.ltb at_ last) then (let result_2 := (List.app result ((match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error text (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%float end) :: @nil float)) in (let at_2 := (PrimFloat.add at_ 1%float) in (ml_rec (text, last, result_2, at_2)))) else result))
    (fun _ => (@nil float)) (text, last, result, at_).

Definition sliceUtf16Text (text : (list float)) (start : float) (end_ : float) : (list float) :=
  (let first := (readUtf16TextSlicePosition start (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text))))) in (let last := (readUtf16TextSlicePosition end_ (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text))))) in (let result := (@nil float) in (let at_ := first in (let result_2 := (source_ml_temporary_sliceUtf16Text_loop5 text last result at_) in result_2))))).

Definition substringUtf16Text (text : (list float)) (start : float) (end_ : float) : (list float) :=
  (let first := (ml_min (ml_max (readUtf16TextInteger start) 0%float) (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text))))) in (let last := (ml_min (ml_max (readUtf16TextInteger end_) 0%float) (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text))))) in (sliceUtf16Text text (ml_min first last) (ml_max first last)))).

Definition source_ml_temporary_pointsUtf16Text_loop6 (text : (list float)) (result : (list (list float))) (at_ : float) : (list (list float)) :=
  ml_fix 64 (fun (ml_rec : (list float) * (list (list float)) * float -> (list (list float))) (ml_args : (list float) * (list (list float)) * float) =>
    let '(text, result, at_) := ml_args in (if (PrimFloat.ltb at_ (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text))))) then (let first := (match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error text (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%float end) in (let second := (match (let ml_index := (ml_float_index (PrimFloat.add at_ 1%float)) in if Z.ltb ml_index 0 then None else List.nth_error text (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (if (andb (andb (andb (PrimFloat.leb 55296%float first) (PrimFloat.leb first 56319%float)) (PrimFloat.leb 56320%float second)) (PrimFloat.leb second 57343%float)) then (let result_2 := (List.app result ((first :: second :: @nil float) :: @nil (list float))) in (let at_2 := (PrimFloat.add at_ 1%float) in (let at_3 := (PrimFloat.add at_2 1%float) in (ml_rec (text, result_2, at_3))))) else (let result_3 := (List.app result ((first :: @nil float) :: @nil (list float))) in (let at_4 := (PrimFloat.add at_ 1%float) in (ml_rec (text, result_3, at_4))))))) else result))
    (fun _ => (@nil (list float))) (text, result, at_).

Definition pointsUtf16Text (text : (list float)) : (list (list float)) :=
  (let result := (@nil (list float)) in (let at_ := 0%float in (let result_2 := (source_ml_temporary_pointsUtf16Text_loop6 text result at_) in result_2))).

Definition source_ml_temporary_splitUtf16Text_loop7 (text : (list float)) (count : float) (result : (list (list float))) (at_ : float) : (list (list float)) :=
  ml_fix 64 (fun (ml_rec : (list float) * float * (list (list float)) * float -> (list (list float))) (ml_args : (list float) * float * (list (list float)) * float) =>
    let '(text, count, result, at_) := ml_args in (if (andb (PrimFloat.ltb at_ (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text))))) (PrimFloat.ltb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length result)))) count)) then (let result_2 := (List.app result (((match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error text (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%float end) :: @nil float) :: @nil (list float))) in (let at_2 := (PrimFloat.add at_ 1%float) in (ml_rec (text, count, result_2, at_2)))) else result))
    (fun _ => (@nil (list float))) (text, count, result, at_).

Inductive source_ml_temporary_splitUtf16Text_loop8_result : Type :=
| source_ml_temporary_splitUtf16Text_loop8_done : (list (list float)) -> float -> source_ml_temporary_splitUtf16Text_loop8_result
| source_ml_temporary_splitUtf16Text_loop8_return : (list (list float)) -> source_ml_temporary_splitUtf16Text_loop8_result.

Inductive source_ml_temporary_matchUtf16Text_loop9_result : Type :=
| source_ml_temporary_matchUtf16Text_loop9_done : source_ml_temporary_matchUtf16Text_loop9_result
| source_ml_temporary_matchUtf16Text_loop9_return : bool -> source_ml_temporary_matchUtf16Text_loop9_result.

Definition source_ml_temporary_matchUtf16Text_loop9 (text : (list float)) (search : (list float)) (start : float) (at_ : float) : source_ml_temporary_matchUtf16Text_loop9_result :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float * float -> source_ml_temporary_matchUtf16Text_loop9_result) (ml_args : (list float) * (list float) * float * float) =>
    let '(text, search, start, at_) := ml_args in (if (PrimFloat.ltb at_ (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length search))))) then (if (let source_ml_source1_comparison_operand13 := (let ml_index := (ml_float_index (PrimFloat.add start at_)) in if Z.ltb ml_index 0 then None else List.nth_error text (Z.to_nat ml_index)) in (let source_ml_source1_comparison_operand14 := (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error search (Z.to_nat ml_index)) in (negb (match (source_ml_source1_comparison_operand13, source_ml_source1_comparison_operand14) with | (Some ml_left, Some ml_right) => (PrimFloat.eqb ml_left ml_right) | (None, None) => true | _ => false end)))) then (source_ml_temporary_matchUtf16Text_loop9_return false) else (let at_2 := (PrimFloat.add at_ 1%float) in (ml_rec (text, search, start, at_2)))) else source_ml_temporary_matchUtf16Text_loop9_done))
    (fun _ => source_ml_temporary_matchUtf16Text_loop9_done) (text, search, start, at_).

Definition matchUtf16Text (text : (list float)) (search : (list float)) (start : float) : bool :=
  (if (orb (PrimFloat.ltb start 0%float) (PrimFloat.ltb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text)))) (PrimFloat.add start (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length search))))))) then false else (let at_ := 0%float in (let source_ml_source1_s4 := (source_ml_temporary_matchUtf16Text_loop9 text search start at_) in (match source_ml_source1_s4 with | source_ml_temporary_matchUtf16Text_loop9_done => true | source_ml_temporary_matchUtf16Text_loop9_return source_ml_temporary_result9 => source_ml_temporary_result9 end)))).

Definition startsUtf16Text (text : (list float)) (search : (list float)) : bool :=
  (matchUtf16Text text search 0%float).

Definition endsUtf16Text (text : (list float)) (search : (list float)) : bool :=
  (matchUtf16Text text search (PrimFloat.sub (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text)))) (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length search)))))).

Inductive source_ml_temporary_indexUtf16Text_loop10_result : Type :=
| source_ml_temporary_indexUtf16Text_loop10_done : source_ml_temporary_indexUtf16Text_loop10_result
| source_ml_temporary_indexUtf16Text_loop10_return : float -> source_ml_temporary_indexUtf16Text_loop10_result.

Definition source_ml_temporary_indexUtf16Text_loop10 (text : (list float)) (search : (list float)) (at_ : float) : source_ml_temporary_indexUtf16Text_loop10_result :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float -> source_ml_temporary_indexUtf16Text_loop10_result) (ml_args : (list float) * (list float) * float) =>
    let '(text, search, at_) := ml_args in (if (PrimFloat.leb at_ (PrimFloat.sub (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text)))) (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length search)))))) then (if (matchUtf16Text text search at_) then (source_ml_temporary_indexUtf16Text_loop10_return at_) else (let at_2 := (PrimFloat.add at_ 1%float) in (ml_rec (text, search, at_2)))) else source_ml_temporary_indexUtf16Text_loop10_done))
    (fun _ => source_ml_temporary_indexUtf16Text_loop10_done) (text, search, at_).

Definition indexUtf16Text (text : (list float)) (search : (list float)) (position : float) : float :=
  (let start := (ml_min (ml_max (readUtf16TextInteger position) 0%float) (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text))))) in (let at_ := start in (let source_ml_source1_s5 := (source_ml_temporary_indexUtf16Text_loop10 text search at_) in (match source_ml_source1_s5 with | source_ml_temporary_indexUtf16Text_loop10_done => (PrimFloat.opp 1%float) | source_ml_temporary_indexUtf16Text_loop10_return source_ml_temporary_result10 => source_ml_temporary_result10 end)))).

Definition includesUtf16Text (text : (list float)) (search : (list float)) : bool :=
  (PrimFloat.leb 0%float (indexUtf16Text text search 0%float)).

Definition source_ml_temporary_splitUtf16Text_loop8 (text : (list float)) (separator : (list float)) (count : float) (result : (list (list float))) (start : float) : source_ml_temporary_splitUtf16Text_loop8_result :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float * (list (list float)) * float -> source_ml_temporary_splitUtf16Text_loop8_result) (ml_args : (list float) * (list float) * float * (list (list float)) * float) =>
    let '(text, separator, count, result, start) := ml_args in (let found := (indexUtf16Text text separator start) in (if (PrimFloat.ltb found 0%float) then (source_ml_temporary_splitUtf16Text_loop8_done result start) else (let result_2 := (List.app result ((sliceUtf16Text text start found) :: @nil (list float))) in (if (PrimFloat.eqb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length result_2)))) count) then (source_ml_temporary_splitUtf16Text_loop8_return result_2) else (let start_2 := (PrimFloat.add found (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length separator))))) in (ml_rec (text, separator, count, result_2, start_2))))))))
    (fun _ => (source_ml_temporary_splitUtf16Text_loop8_done (@nil (list float)) 0%float)) (text, separator, count, result, start).

Definition splitUtf16Text (text : (list float)) (separator : (list float)) (limit : float) (missing : bool) : (list (list float)) :=
  (let count := (normalizeJavaScriptUnsignedInteger limit) in (let result := (@nil (list float)) in (if (PrimFloat.eqb count 0%float) then result else (if missing then ((sliceUtf16Text text 0%float (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text))))) :: @nil (list float)) else (if (PrimFloat.eqb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length separator)))) 0%float) then (let at_ := 0%float in (let result_2 := (source_ml_temporary_splitUtf16Text_loop7 text count result at_) in result_2)) else (let start := 0%float in (let source_ml_source1_s3 := (source_ml_temporary_splitUtf16Text_loop8 text separator count result start) in (match source_ml_source1_s3 with | source_ml_temporary_splitUtf16Text_loop8_done result_3 start_2 => (let result_4 := (List.app result_3 ((sliceUtf16Text text start_2 (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text))))) :: @nil (list float))) in result_4) | source_ml_temporary_splitUtf16Text_loop8_return source_ml_temporary_result8 => source_ml_temporary_result8 end)))))))).

Inductive source_ml_temporary_lastIndexUtf16Text_loop11_result : Type :=
| source_ml_temporary_lastIndexUtf16Text_loop11_done : source_ml_temporary_lastIndexUtf16Text_loop11_result
| source_ml_temporary_lastIndexUtf16Text_loop11_return : float -> source_ml_temporary_lastIndexUtf16Text_loop11_result.

Definition source_ml_temporary_lastIndexUtf16Text_loop11 (text : (list float)) (search : (list float)) (at_ : float) : source_ml_temporary_lastIndexUtf16Text_loop11_result :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float -> source_ml_temporary_lastIndexUtf16Text_loop11_result) (ml_args : (list float) * (list float) * float) =>
    let '(text, search, at_) := ml_args in (if (PrimFloat.leb 0%float at_) then (if (matchUtf16Text text search at_) then (source_ml_temporary_lastIndexUtf16Text_loop11_return at_) else (let at_2 := (PrimFloat.sub at_ 1%float) in (ml_rec (text, search, at_2)))) else source_ml_temporary_lastIndexUtf16Text_loop11_done))
    (fun _ => source_ml_temporary_lastIndexUtf16Text_loop11_done) (text, search, at_).

Definition lastIndexUtf16Text (text : (list float)) (search : (list float)) (position : float) : float :=
  (let integer := (if (PrimFloat.is_nan position) then PrimFloat.infinity else (readUtf16TextInteger position)) in (let start := (ml_min (ml_max integer 0%float) (PrimFloat.sub (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text)))) (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length search)))))) in (let at_ := start in (let source_ml_source1_s6 := (source_ml_temporary_lastIndexUtf16Text_loop11 text search at_) in (match source_ml_source1_s6 with | source_ml_temporary_lastIndexUtf16Text_loop11_done => (PrimFloat.opp 1%float) | source_ml_temporary_lastIndexUtf16Text_loop11_return source_ml_temporary_result11 => source_ml_temporary_result11 end))))).

Inductive source_ml_temporary_compareUtf16Text_loop12_result : Type :=
| source_ml_temporary_compareUtf16Text_loop12_done : source_ml_temporary_compareUtf16Text_loop12_result
| source_ml_temporary_compareUtf16Text_loop12_return : float -> source_ml_temporary_compareUtf16Text_loop12_result.

Definition source_ml_temporary_compareUtf16Text_loop12 (left_ : (list float)) (right_ : (list float)) (count : float) (at_ : float) : source_ml_temporary_compareUtf16Text_loop12_result :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float * float -> source_ml_temporary_compareUtf16Text_loop12_result) (ml_args : (list float) * (list float) * float * float) =>
    let '(left_, right_, count, at_) := ml_args in (if (PrimFloat.ltb at_ count) then (let a := (match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error left_ (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%float end) in (let b := (match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error right_ (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%float end) in (if (negb (PrimFloat.eqb a b)) then (source_ml_temporary_compareUtf16Text_loop12_return (if (PrimFloat.ltb a b) then (PrimFloat.opp 1%float) else 1%float)) else (let at_2 := (PrimFloat.add at_ 1%float) in (ml_rec (left_, right_, count, at_2)))))) else source_ml_temporary_compareUtf16Text_loop12_done))
    (fun _ => source_ml_temporary_compareUtf16Text_loop12_done) (left_, right_, count, at_).

Definition compareUtf16Text (left_ : (list float)) (right_ : (list float)) : float :=
  (let count := (ml_min (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length left_)))) (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length right_))))) in (let at_ := 0%float in (let source_ml_source1_s7 := (source_ml_temporary_compareUtf16Text_loop12 left_ right_ count at_) in (match source_ml_source1_s7 with | source_ml_temporary_compareUtf16Text_loop12_done => (if (PrimFloat.eqb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length left_)))) (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length right_))))) then 0%float else (if (PrimFloat.ltb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length left_)))) (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length right_))))) then (PrimFloat.opp 1%float) else 1%float)) | source_ml_temporary_compareUtf16Text_loop12_return source_ml_temporary_result12 => source_ml_temporary_result12 end)))).

Definition source_ml_temporary_joinUtf16Text_loop14 (separator : (list float)) (result : (list float)) (offset : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float -> (list float)) (ml_args : (list float) * (list float) * float) =>
    let '(separator, result, offset) := ml_args in (if (PrimFloat.ltb offset (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length separator))))) then (let result_2 := (List.app result ((match (let ml_index := (ml_float_index offset) in if Z.ltb ml_index 0 then None else List.nth_error separator (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%float end) :: @nil float)) in (let offset_2 := (PrimFloat.add offset 1%float) in (ml_rec (separator, result_2, offset_2)))) else result))
    (fun _ => (@nil float)) (separator, result, offset).

Definition source_ml_temporary_joinUtf16Text_loop15 (result : (list float)) (value : (list float)) (offset : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float -> (list float)) (ml_args : (list float) * (list float) * float) =>
    let '(result, value, offset) := ml_args in (if (PrimFloat.ltb offset (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length value))))) then (let result_2 := (List.app result ((match (let ml_index := (ml_float_index offset) in if Z.ltb ml_index 0 then None else List.nth_error value (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%float end) :: @nil float)) in (let offset_2 := (PrimFloat.add offset 1%float) in (ml_rec (result_2, value, offset_2)))) else result))
    (fun _ => (@nil float)) (result, value, offset).

Definition source_ml_temporary_joinUtf16Text_loop13 (values : (list (list float))) (separator : (list float)) (result : (list float)) (at_ : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * (list float) * (list float) * float -> (list float)) (ml_args : (list (list float)) * (list float) * (list float) * float) =>
    let '(values, separator, result, at_) := ml_args in (if (PrimFloat.ltb at_ (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length values))))) then (let result_3 := (if (negb (PrimFloat.eqb at_ 0%float)) then (let offset := 0%float in (let result_2 := (source_ml_temporary_joinUtf16Text_loop14 separator result offset) in result_2)) else result) in (let value := (match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error values (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (@nil float) end) in (let offset_2 := 0%float in (let result_4 := (source_ml_temporary_joinUtf16Text_loop15 result_3 value offset_2) in (let at_2 := (PrimFloat.add at_ 1%float) in (ml_rec (values, separator, result_4, at_2))))))) else result))
    (fun _ => (@nil float)) (values, separator, result, at_).

Definition joinUtf16Text (values : (list (list float))) (separator : (list float)) : (list float) :=
  (let result := (@nil float) in (let at_ := 0%float in (let result_2 := (source_ml_temporary_joinUtf16Text_loop13 values separator result at_) in result_2))).

Definition source_ml_temporary_trimUtf16Text_loop16 (text : (list float)) (first : float) (last : float) : float :=
  ml_fix 64 (fun (ml_rec : (list float) * float * float -> float) (ml_args : (list float) * float * float) =>
    let '(text, first, last) := ml_args in (if (andb (PrimFloat.ltb first last) (acceptJavaScriptWhitespaceUnit (match (let ml_index := (ml_float_index first) in if Z.ltb ml_index 0 then None else List.nth_error text (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end))) then (let first_2 := (PrimFloat.add first 1%float) in (ml_rec (text, first_2, last))) else first))
    (fun _ => 0%float) (text, first, last).

Definition source_ml_temporary_trimUtf16Text_loop17 (text : (list float)) (first : float) (last : float) : float :=
  ml_fix 64 (fun (ml_rec : (list float) * float * float -> float) (ml_args : (list float) * float * float) =>
    let '(text, first, last) := ml_args in (if (andb (PrimFloat.ltb first last) (acceptJavaScriptWhitespaceUnit (match (let ml_index := (ml_float_index (PrimFloat.sub last 1%float)) in if Z.ltb ml_index 0 then None else List.nth_error text (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end))) then (let last_2 := (PrimFloat.sub last 1%float) in (ml_rec (text, first, last_2))) else last))
    (fun _ => 0%float) (text, first, last).

Definition trimUtf16Text (text : (list float)) (start : bool) (end_ : bool) : (list float) :=
  (let first := 0%float in (let last := (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text)))) in (let first_3 := (if start then (let first_2 := (source_ml_temporary_trimUtf16Text_loop16 text first last) in first_2) else first) in (let last_3 := (if end_ then (let last_2 := (source_ml_temporary_trimUtf16Text_loop17 text first_3 last) in last_2) else last) in (sliceUtf16Text text first_3 last_3))))).

Definition source_ml_temporary_characterUnitsUtf16Text_loop18 (values : (list float)) (result : (list float)) (at_ : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float -> (list float)) (ml_args : (list float) * (list float) * float) =>
    let '(values, result, at_) := ml_args in (if (PrimFloat.ltb at_ (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length values))))) then (let value := (match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error values (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%float end) in (let integer := (if (ml_is_finite value) then (ml_trunc value) else 0%float) in (let residue := (ml_float_rem integer 65536%float) in (let result_2 := (List.app result ((if (PrimFloat.ltb residue 0%float) then (PrimFloat.add residue 65536%float) else (if (PrimFloat.eqb residue 0%float) then 0%float else residue)) :: @nil float)) in (let at_2 := (PrimFloat.add at_ 1%float) in (ml_rec (values, result_2, at_2))))))) else result))
    (fun _ => (@nil float)) (values, result, at_).

Definition characterUnitsUtf16Text (values : (list float)) : (list float) :=
  (let result := (@nil float) in (let at_ := 0%float in (let result_2 := (source_ml_temporary_characterUnitsUtf16Text_loop18 values result at_) in result_2))).

Inductive source_ml_temporary_codePointUnitsUtf16Text_loop19_result : Type :=
| source_ml_temporary_codePointUnitsUtf16Text_loop19_done : (list float) -> source_ml_temporary_codePointUnitsUtf16Text_loop19_result
| source_ml_temporary_codePointUnitsUtf16Text_loop19_return : (list float) -> source_ml_temporary_codePointUnitsUtf16Text_loop19_result.

Definition source_ml_temporary_codePointUnitsUtf16Text_loop19 (values : (list float)) (result : (list float)) (at_ : float) : source_ml_temporary_codePointUnitsUtf16Text_loop19_result :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float -> source_ml_temporary_codePointUnitsUtf16Text_loop19_result) (ml_args : (list float) * (list float) * float) =>
    let '(values, result, at_) := ml_args in (if (PrimFloat.ltb at_ (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length values))))) then (let value := (match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error values (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (if (orb (orb (negb (ml_is_integer value)) (PrimFloat.ltb value 0%float)) (PrimFloat.ltb 1114111%float value)) then (source_ml_temporary_codePointUnitsUtf16Text_loop19_return (at_ :: @nil float)) else (if (PrimFloat.ltb value 65536%float) then (let result_2 := (List.app result ((if (PrimFloat.eqb value 0%float) then 0%float else value) :: @nil float)) in (let at_2 := (PrimFloat.add at_ 1%float) in (ml_rec (values, result_2, at_2)))) else (let result_3 := (List.app result ((PrimFloat.add 55296%float (ml_floor (PrimFloat.div (PrimFloat.sub value 65536%float) 1024%float))) :: (PrimFloat.add 56320%float (ml_float_rem (PrimFloat.sub value 65536%float) 1024%float)) :: @nil float)) in (let at_3 := (PrimFloat.add at_ 1%float) in (ml_rec (values, result_3, at_3))))))) else (source_ml_temporary_codePointUnitsUtf16Text_loop19_done result)))
    (fun _ => (source_ml_temporary_codePointUnitsUtf16Text_loop19_done (@nil float))) (values, result, at_).

Definition codePointUnitsUtf16Text (values : (list float)) : (list float) :=
  (let result := ((PrimFloat.opp 1%float) :: @nil float) in (let at_ := 0%float in (let source_ml_source1_s8 := (source_ml_temporary_codePointUnitsUtf16Text_loop19 values result at_) in (match source_ml_source1_s8 with | source_ml_temporary_codePointUnitsUtf16Text_loop19_done result_2 => result_2 | source_ml_temporary_codePointUnitsUtf16Text_loop19_return source_ml_temporary_result19 => source_ml_temporary_result19 end)))).

Inductive source_ml_temporary_displayPointsUtf16Text_join21 : Type :=
| source_ml_temporary_displayPointsUtf16Text_join21_next : float -> float -> source_ml_temporary_displayPointsUtf16Text_join21.

Definition source_ml_temporary_displayPointsUtf16Text_loop20 (text : (list float)) (result : (list float)) (at_ : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float -> (list float)) (ml_args : (list float) * (list float) * float) =>
    let '(text, result, at_) := ml_args in (if (PrimFloat.ltb at_ (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text))))) then (let point := (match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error text (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 65533%float end) in (let source_ml_source1_s15 := (if (andb (PrimFloat.leb 55296%float point) (PrimFloat.leb point 56319%float)) then (let low := (match (let ml_index := (ml_float_index (PrimFloat.add at_ 1%float)) in if Z.ltb ml_index 0 then None else List.nth_error text (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (if (andb (PrimFloat.leb 56320%float low) (PrimFloat.leb low 57343%float)) then (let point_2 := (PrimFloat.sub (PrimFloat.add (PrimFloat.add 65536%float (PrimFloat.mul (PrimFloat.sub point 55296%float) 1024%float)) low) 56320%float) in (let at_2 := (PrimFloat.add at_ 1%float) in (source_ml_temporary_displayPointsUtf16Text_join21_next at_2 point_2))) else (let point_3 := 65533%float in (source_ml_temporary_displayPointsUtf16Text_join21_next at_ point_3)))) else (if (andb (PrimFloat.leb 56320%float point) (PrimFloat.leb point 57343%float)) then (let point_4 := 65533%float in (source_ml_temporary_displayPointsUtf16Text_join21_next at_ point_4)) else (source_ml_temporary_displayPointsUtf16Text_join21_next at_ point))) in (match source_ml_source1_s15 with | source_ml_temporary_displayPointsUtf16Text_join21_next at_3 point_5 => (let result_2 := (List.app result (point_5 :: @nil float)) in (let at_4 := (PrimFloat.add at_3 1%float) in (ml_rec (text, result_2, at_4)))) end))) else result))
    (fun _ => (@nil float)) (text, result, at_).

Definition displayPointsUtf16Text (text : (list float)) : (list float) :=
  (let result := (@nil float) in (let at_ := 0%float in (let result_2 := (source_ml_temporary_displayPointsUtf16Text_loop20 text result at_) in result_2))).

Definition source_ml_temporary_displayUtf8FromUtf16Text_loop22 (bytes : (list float)) (points : (list float)) (at_ : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float -> (list float)) (ml_args : (list float) * (list float) * float) =>
    let '(bytes, points, at_) := ml_args in (if (PrimFloat.ltb at_ (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length points))))) then (let point := (match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error points (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 65533%float end) in (if (PrimFloat.ltb point 128%float) then (let bytes_2 := (List.app bytes (point :: @nil float)) in (let at_2 := (PrimFloat.add at_ 1%float) in (ml_rec (bytes_2, points, at_2)))) else (if (PrimFloat.ltb point 2048%float) then (let bytes_3 := (List.app bytes ((PrimFloat.add 192%float (ml_floor (PrimFloat.div point 64%float))) :: (PrimFloat.add 128%float (ml_float_rem point 64%float)) :: @nil float)) in (let at_3 := (PrimFloat.add at_ 1%float) in (ml_rec (bytes_3, points, at_3)))) else (if (PrimFloat.ltb point 65536%float) then (let bytes_4 := (List.app bytes ((PrimFloat.add 224%float (ml_floor (PrimFloat.div point 4096%float))) :: (PrimFloat.add 128%float (ml_float_rem (ml_floor (PrimFloat.div point 64%float)) 64%float)) :: (PrimFloat.add 128%float (ml_float_rem point 64%float)) :: @nil float)) in (let at_4 := (PrimFloat.add at_ 1%float) in (ml_rec (bytes_4, points, at_4)))) else (let bytes_5 := (List.app bytes ((PrimFloat.add 240%float (ml_floor (PrimFloat.div point 262144%float))) :: (PrimFloat.add 128%float (ml_float_rem (ml_floor (PrimFloat.div point 4096%float)) 64%float)) :: (PrimFloat.add 128%float (ml_float_rem (ml_floor (PrimFloat.div point 64%float)) 64%float)) :: (PrimFloat.add 128%float (ml_float_rem point 64%float)) :: @nil float)) in (let at_5 := (PrimFloat.add at_ 1%float) in (ml_rec (bytes_5, points, at_5)))))))) else bytes))
    (fun _ => (@nil float)) (bytes, points, at_).

Definition displayUtf8FromUtf16Text (text : (list float)) : (list float) :=
  (let bytes := (@nil float) in (let points := (displayPointsUtf16Text text) in (let at_ := 0%float in (let bytes_2 := (source_ml_temporary_displayUtf8FromUtf16Text_loop22 bytes points at_) in bytes_2)))).

Inductive source_ml_temporary_planUtf16TextSubstitution_loop23_result : Type :=
| source_ml_temporary_planUtf16TextSubstitution_loop23_done : (list float) -> float -> source_ml_temporary_planUtf16TextSubstitution_loop23_result.

Definition source_ml_temporary_planUtf16TextSubstitution_loop23 (replacement : (list float)) (result : (list float)) (start : float) (at_ : float) : source_ml_temporary_planUtf16TextSubstitution_loop23_result :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float * float -> source_ml_temporary_planUtf16TextSubstitution_loop23_result) (ml_args : (list float) * (list float) * float * float) =>
    let '(replacement, result, start, at_) := ml_args in (if (PrimFloat.ltb at_ (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length replacement))))) then (let next := (match (let ml_index := (ml_float_index (PrimFloat.add at_ 1%float)) in if Z.ltb ml_index 0 then None else List.nth_error replacement (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => (PrimFloat.opp 1%float) end) in (if (andb (PrimFloat.eqb (match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error replacement (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%float end) 36%float) (orb (orb (orb (PrimFloat.eqb next 36%float) (PrimFloat.eqb next 38%float)) (PrimFloat.eqb next 96%float)) (PrimFloat.eqb next 39%float))) then (let result_3 := (if (PrimFloat.ltb start at_) then (let result_2 := (List.app result (0%float :: start :: at_ :: @nil float)) in result_2) else result) in (let result_6 := (if (PrimFloat.eqb next 36%float) then (let result_4 := (List.app result_3 (0%float :: at_ :: (PrimFloat.add at_ 1%float) :: @nil float)) in result_4) else (let result_5 := (List.app result_3 ((if (PrimFloat.eqb next 38%float) then 1%float else (if (PrimFloat.eqb next 96%float) then 2%float else 3%float)) :: 0%float :: 0%float :: @nil float)) in result_5)) in (let at_2 := (PrimFloat.add at_ 2%float) in (let start_2 := at_2 in (ml_rec (replacement, result_6, start_2, at_2)))))) else (let at_3 := (PrimFloat.add at_ 1%float) in (ml_rec (replacement, result, start, at_3))))) else (source_ml_temporary_planUtf16TextSubstitution_loop23_done result start)))
    (fun _ => (source_ml_temporary_planUtf16TextSubstitution_loop23_done (@nil float) 0%float)) (replacement, result, start, at_).

Definition planUtf16TextSubstitution (replacement : (list float)) : (list float) :=
  (let result := (@nil float) in (let start := 0%float in (let at_ := 0%float in (let source_ml_source1_s9 := (source_ml_temporary_planUtf16TextSubstitution_loop23 replacement result start at_) in (match source_ml_source1_s9 with | source_ml_temporary_planUtf16TextSubstitution_loop23_done result_2 start_2 => (let result_4 := (if (PrimFloat.ltb start_2 (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length replacement))))) then (let result_3 := (List.app result_2 (0%float :: start_2 :: (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length replacement)))) :: @nil float)) in result_3) else result_2) in result_4) end))))).

Definition source_ml_temporary_replaceUtf16Text_loop25 (text : (list float)) (result : (list float)) (at_ : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float -> (list float)) (ml_args : (list float) * (list float) * float) =>
    let '(text, result, at_) := ml_args in (if (PrimFloat.ltb at_ (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text))))) then (let result_2 := (List.app result ((match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error text (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%float end) :: @nil float)) in (let at_2 := (PrimFloat.add at_ 1%float) in (ml_rec (text, result_2, at_2)))) else result))
    (fun _ => (@nil float)) (text, result, at_).

Inductive source_ml_temporary_replaceUtf16Text_loop24_result : Type :=
| source_ml_temporary_replaceUtf16Text_loop24_done : (list float) -> float -> source_ml_temporary_replaceUtf16Text_loop24_result.

Definition source_ml_temporary_replaceUtf16Text_loop26 (text : (list float)) (result : (list float)) (found : float) (at_ : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float * float -> (list float)) (ml_args : (list float) * (list float) * float * float) =>
    let '(text, result, found, at_) := ml_args in (if (PrimFloat.ltb at_ found) then (let result_2 := (List.app result ((match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error text (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%float end) :: @nil float)) in (let at_2 := (PrimFloat.add at_ 1%float) in (ml_rec (text, result_2, found, at_2)))) else result))
    (fun _ => (@nil float)) (text, result, found, at_).

Definition source_ml_temporary_replaceUtf16Text_loop28 (text : (list float)) (replacement : (list float)) (result : (list float)) (kind : float) (last : float) (at_ : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * (list float) * float * float * float -> (list float)) (ml_args : (list float) * (list float) * (list float) * float * float * float) =>
    let '(text, replacement, result, kind, last, at_) := ml_args in (if (PrimFloat.ltb at_ last) then (let result_2 := (List.app result ((if (PrimFloat.eqb kind 0%float) then (match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error replacement (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%float end) else (match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error text (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%float end)) :: @nil float)) in (let at_2 := (PrimFloat.add at_ 1%float) in (ml_rec (text, replacement, result_2, kind, last, at_2)))) else result))
    (fun _ => (@nil float)) (text, replacement, result, kind, last, at_).

Definition source_ml_temporary_replaceUtf16Text_loop27 (text : (list float)) (search : (list float)) (replacement : (list float)) (result : (list float)) (plan : (list float)) (found : float) (instruction : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * (list float) * (list float) * (list float) * float * float -> (list float)) (ml_args : (list float) * (list float) * (list float) * (list float) * (list float) * float * float) =>
    let '(text, search, replacement, result, plan, found, instruction) := ml_args in (if (PrimFloat.ltb instruction (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length plan))))) then (let kind := (match (let ml_index := (ml_float_index instruction) in if Z.ltb ml_index 0 then None else List.nth_error plan (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%float end) in (let first := (if (PrimFloat.eqb kind 0%float) then (match (let ml_index := (ml_float_index (PrimFloat.add instruction 1%float)) in if Z.ltb ml_index 0 then None else List.nth_error plan (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%float end) else (if (PrimFloat.eqb kind 2%float) then 0%float else (if (PrimFloat.eqb kind 1%float) then found else (PrimFloat.add found (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length search)))))))) in (let last := (if (PrimFloat.eqb kind 0%float) then (match (let ml_index := (ml_float_index (PrimFloat.add instruction 2%float)) in if Z.ltb ml_index 0 then None else List.nth_error plan (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => 0%float end) else (if (PrimFloat.eqb kind 2%float) then found else (if (PrimFloat.eqb kind 1%float) then (PrimFloat.add found (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length search))))) else (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text))))))) in (let at_ := first in (let result_2 := (source_ml_temporary_replaceUtf16Text_loop28 text replacement result kind last at_) in (let instruction_2 := (PrimFloat.add instruction 3%float) in (ml_rec (text, search, replacement, result_2, plan, found, instruction_2)))))))) else result))
    (fun _ => (@nil float)) (text, search, replacement, result, plan, found, instruction).

Definition source_ml_temporary_replaceUtf16Text_loop24 (text : (list float)) (search : (list float)) (replacement : (list float)) (all : bool) (result : (list float)) (plan : (list float)) (start : float) (from : float) : source_ml_temporary_replaceUtf16Text_loop24_result :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * (list float) * bool * (list float) * (list float) * float * float -> source_ml_temporary_replaceUtf16Text_loop24_result) (ml_args : (list float) * (list float) * (list float) * bool * (list float) * (list float) * float * float) =>
    let '(text, search, replacement, all, result, plan, start, from) := ml_args in (let found := (indexUtf16Text text search from) in (if (PrimFloat.ltb found 0%float) then (source_ml_temporary_replaceUtf16Text_loop24_done result start) else (let at_ := start in (let result_2 := (source_ml_temporary_replaceUtf16Text_loop26 text result found at_) in (let instruction := 0%float in (let result_3 := (source_ml_temporary_replaceUtf16Text_loop27 text search replacement result_2 plan found instruction) in (let start_2 := (PrimFloat.add found (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length search))))) in (if (orb (negb all) (andb (PrimFloat.eqb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length search)))) 0%float) (PrimFloat.eqb found (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length text))))))) then (source_ml_temporary_replaceUtf16Text_loop24_done result_3 start_2) else (let from_2 := (PrimFloat.add start_2 (if (PrimFloat.eqb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length search)))) 0%float) then 1%float else 0%float)) in (ml_rec (text, search, replacement, all, result_3, plan, start_2, from_2))))))))))))
    (fun _ => (source_ml_temporary_replaceUtf16Text_loop24_done (@nil float) 0%float)) (text, search, replacement, all, result, plan, start, from).

Definition replaceUtf16Text (text : (list float)) (search : (list float)) (replacement : (list float)) (all : bool) : (list float) :=
  (let result := (@nil float) in (let plan := (planUtf16TextSubstitution replacement) in (let start := 0%float in (let from := 0%float in (let source_ml_source1_s10 := (source_ml_temporary_replaceUtf16Text_loop24 text search replacement all result plan start from) in (match source_ml_source1_s10 with | source_ml_temporary_replaceUtf16Text_loop24_done result_2 start_2 => (let at_ := start_2 in (let result_3 := (source_ml_temporary_replaceUtf16Text_loop25 text result_2 at_) in result_3)) end)))))).

Definition main : list string :=
  nil.

Eval vm_compute in main.

Definition replace := replaceUtf16Text.
Definition substitutionPlan := planUtf16TextSubstitution.
Definition split := splitUtf16Text.
Definition slicePosition := readUtf16TextSlicePosition.
Definition slice := sliceUtf16Text.
Definition substring := substringUtf16Text.
Definition character := characterUtf16Text.
Definition relativeCharacter := relativeCharacterUtf16Text.
Definition codePoint := codePointUtf16Text.
Definition points := pointsUtf16Text.
Definition index := indexUtf16Text.
Definition lastIndex := lastIndexUtf16Text.
Definition compare := compareUtf16Text.
Definition starts := startsUtf16Text.
Definition ends := endsUtf16Text.
Definition includes := includesUtf16Text.
Definition trim := trimUtf16Text.
Definition characterUnits := characterUnitsUtf16Text.
Definition codePointUnits := codePointUnitsUtf16Text.
Definition join := joinUtf16Text.
Definition displayUtf8 := displayUtf8FromUtf16Text.
Definition displayPoints := displayPointsUtf16Text.
Definition fromUtf8 := readUtf8TextUnits.
End MLUtf16Text.
