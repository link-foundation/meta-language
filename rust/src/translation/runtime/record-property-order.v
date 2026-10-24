From Stdlib Require Import NArith ZArith Lia Recdef String List Ascii Floats.
From Stdlib Require Import NArith ZArith Lia Recdef String List Ascii Floats.
Module MLRecordPropertyOrder.
(* Translated from JavaScript by meta-language: portable core, Rocq target. *)
Set Warnings "-inexact-float".

Fixpoint ml_digits (fuel : nat) (n : N) (acc : string) : string :=
  match fuel with
  | O => acc
  | S rest =>
      let acc' := String (ascii_of_N (48 + N.modulo n 10)) acc in
      if N.ltb n 10 then acc' else ml_digits rest (N.div n 10) acc'
  end.
Definition ml_N_to_string (n : N) : string := ml_digits (S (N.size_nat n)) n EmptyString.

(* ECMAScript Number::toString for binary64: the shortest decimal that rounds
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
  end.

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


Definition refuseRequiredNumberFact : float :=
  (ml_raiseJavaScriptRangeError (109%N :: 105%N :: 115%N :: 115%N :: 105%N :: 110%N :: 103%N :: 32%N :: 114%N :: 101%N :: 113%N :: 117%N :: 105%N :: 114%N :: 101%N :: 100%N :: 32%N :: 78%N :: 117%N :: 109%N :: 98%N :: 101%N :: 114%N :: 32%N :: 102%N :: 97%N :: 99%N :: 116%N :: 32%N :: 99%N :: 111%N :: 108%N :: 117%N :: 109%N :: 110%N :: 32%N :: 118%N :: 97%N :: 108%N :: 117%N :: 101%N :: (@nil N))).

Definition refuseRequiredTextFact : (list N) :=
  (ml_raiseJavaScriptRangeError (109%N :: 105%N :: 115%N :: 115%N :: 105%N :: 110%N :: 103%N :: 32%N :: 114%N :: 101%N :: 113%N :: 117%N :: 105%N :: 114%N :: 101%N :: 100%N :: 32%N :: 116%N :: 101%N :: 120%N :: 116%N :: 32%N :: 102%N :: 97%N :: 99%N :: 116%N :: 32%N :: 99%N :: 111%N :: 108%N :: 117%N :: 109%N :: 110%N :: 32%N :: 118%N :: 97%N :: 108%N :: 117%N :: 101%N :: (@nil N))).

Definition refuseRequiredBooleanFact : bool :=
  (ml_raiseJavaScriptRangeError (109%N :: 105%N :: 115%N :: 115%N :: 105%N :: 110%N :: 103%N :: 32%N :: 114%N :: 101%N :: 113%N :: 117%N :: 105%N :: 114%N :: 101%N :: 100%N :: 32%N :: 66%N :: 111%N :: 111%N :: 108%N :: 101%N :: 97%N :: 110%N :: 32%N :: 102%N :: 97%N :: 99%N :: 116%N :: 32%N :: 99%N :: 111%N :: 108%N :: 117%N :: 109%N :: 110%N :: 32%N :: 118%N :: 97%N :: 108%N :: 117%N :: 101%N :: (@nil N))).

Definition readRequiredNumberFact (values : (list float)) (index : float) : float :=
  (if (orb (orb (PrimFloat.ltb index 0%float) (negb (PrimFloat.eqb (ml_float_rem index 1%float) 0%float))) (PrimFloat.leb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length values)))) index)) then (ml_raiseJavaScriptRangeError (105%N :: 110%N :: 118%N :: 97%N :: 108%N :: 105%N :: 100%N :: 32%N :: 114%N :: 101%N :: 113%N :: 117%N :: 105%N :: 114%N :: 101%N :: 100%N :: 32%N :: 78%N :: 117%N :: 109%N :: 98%N :: 101%N :: 114%N :: 32%N :: 102%N :: 97%N :: 99%N :: 116%N :: 32%N :: 99%N :: 111%N :: 108%N :: 117%N :: 109%N :: 110%N :: 32%N :: 112%N :: 111%N :: 115%N :: 105%N :: 116%N :: 105%N :: 111%N :: 110%N :: (@nil N))) else (match (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error values (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => refuseRequiredNumberFact end)).

Definition readRequiredTextFact (values : (list (list N))) (index : float) : (list N) :=
  (if (orb (orb (PrimFloat.ltb index 0%float) (negb (PrimFloat.eqb (ml_float_rem index 1%float) 0%float))) (PrimFloat.leb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length values)))) index)) then (ml_raiseJavaScriptRangeError (105%N :: 110%N :: 118%N :: 97%N :: 108%N :: 105%N :: 100%N :: 32%N :: 114%N :: 101%N :: 113%N :: 117%N :: 105%N :: 114%N :: 101%N :: 100%N :: 32%N :: 116%N :: 101%N :: 120%N :: 116%N :: 32%N :: 102%N :: 97%N :: 99%N :: 116%N :: 32%N :: 99%N :: 111%N :: 108%N :: 117%N :: 109%N :: 110%N :: 32%N :: 112%N :: 111%N :: 115%N :: 105%N :: 116%N :: 105%N :: 111%N :: 110%N :: (@nil N))) else (match (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error values (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => refuseRequiredTextFact end)).

Definition readRequiredBooleanFact (values : (list bool)) (index : float) : bool :=
  (if (orb (orb (PrimFloat.ltb index 0%float) (negb (PrimFloat.eqb (ml_float_rem index 1%float) 0%float))) (PrimFloat.leb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length values)))) index)) then (ml_raiseJavaScriptRangeError (105%N :: 110%N :: 118%N :: 97%N :: 108%N :: 105%N :: 100%N :: 32%N :: 114%N :: 101%N :: 113%N :: 117%N :: 105%N :: 114%N :: 101%N :: 100%N :: 32%N :: 66%N :: 111%N :: 111%N :: 108%N :: 101%N :: 97%N :: 110%N :: 32%N :: 102%N :: 97%N :: 99%N :: 116%N :: 32%N :: 99%N :: 111%N :: 108%N :: 117%N :: 109%N :: 110%N :: 32%N :: 112%N :: 111%N :: 115%N :: 105%N :: 116%N :: 105%N :: 111%N :: 110%N :: (@nil N))) else (match (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error values (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => refuseRequiredBooleanFact end)).

Definition refuseRequiredRecordName : (list float) :=
  (ml_raiseJavaScriptRangeError (109%N :: 105%N :: 115%N :: 115%N :: 105%N :: 110%N :: 103%N :: 32%N :: 114%N :: 101%N :: 113%N :: 117%N :: 105%N :: 114%N :: 101%N :: 100%N :: 32%N :: 114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: 32%N :: 110%N :: 97%N :: 109%N :: 101%N :: 32%N :: 114%N :: 111%N :: 119%N :: (@nil N))).

Definition readRequiredRecordName (names : (list (list float))) (index : float) : (list float) :=
  (if (orb (orb (PrimFloat.ltb index 0%float) (negb (PrimFloat.eqb (ml_float_rem index 1%float) 0%float))) (PrimFloat.leb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length names)))) index)) then (ml_raiseJavaScriptRangeError (105%N :: 110%N :: 118%N :: 97%N :: 108%N :: 105%N :: 100%N :: 32%N :: 114%N :: 101%N :: 113%N :: 117%N :: 105%N :: 114%N :: 101%N :: 100%N :: 32%N :: 114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: 32%N :: 110%N :: 97%N :: 109%N :: 101%N :: 32%N :: 112%N :: 111%N :: 115%N :: 105%N :: 116%N :: 105%N :: 111%N :: 110%N :: (@nil N))) else (match (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error names (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => refuseRequiredRecordName end)).

Definition readRecordProjection : (list N) :=
  (99%N :: 104%N :: 101%N :: 99%N :: 107%N :: 101%N :: 100%N :: 32%N :: 111%N :: 114%N :: 100%N :: 101%N :: 114%N :: 101%N :: 100%N :: 32%N :: 114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: 32%N :: 118%N :: 97%N :: 108%N :: 117%N :: 101%N :: 115%N :: (@nil N)).

Definition readRecordSpreadRole (kind : (list N)) : (list N) :=
  (if (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (118%N :: 97%N :: 114%N :: (@nil N)))) 0%float) then (111%N :: 112%N :: 101%N :: 110%N :: (@nil N)) else (if (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: (@nil N)))) 0%float) then (102%N :: 105%N :: 101%N :: 108%N :: 100%N :: 115%N :: (@nil N)) else (114%N :: 101%N :: 102%N :: 117%N :: 115%N :: 101%N :: (@nil N)))).

Definition readRecordTypeConstraintError (action : float) (name : (list N)) : (list N) :=
  (if (orb (PrimFloat.eqb action 3%float) (PrimFloat.eqb action 4%float)) then (List.app (List.app (114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: 32%N :: 102%N :: 105%N :: 101%N :: 108%N :: 100%N :: 32%N :: (@nil N)) name) (32%N :: 105%N :: 115%N :: 32%N :: 109%N :: 105%N :: 115%N :: 115%N :: 105%N :: 110%N :: 103%N :: (@nil N))) else ((@nil N))).

Definition readRecordMethodError (kind : (list N)) (operation : (list N)) (parameters : float) (argumentsCount : float) : (list N) :=
  (if (negb (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (102%N :: 117%N :: 110%N :: 99%N :: 116%N :: 105%N :: 111%N :: 110%N :: (@nil N)))) 0%float)) then (List.app (114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: 32%N :: 104%N :: 97%N :: 115%N :: 32%N :: 110%N :: 111%N :: 32%N :: 109%N :: 101%N :: 116%N :: 104%N :: 111%N :: 100%N :: 32%N :: (@nil N)) operation) else (if (negb (PrimFloat.eqb argumentsCount parameters)) then (List.app (List.app (List.app (List.app (46%N :: (@nil N)) operation) (40%N :: 41%N :: 32%N :: 101%N :: 120%N :: 112%N :: 101%N :: 99%N :: 116%N :: 115%N :: 32%N :: (@nil N))) (List.map (fun ml_unit => Z.to_N (ml_float_index ml_unit)) (MLUtf16Text.fromUtf8 ((fix ml_text_bytes (ml_text : string) : list float := match ml_text with EmptyString => nil | String ml_character ml_rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii ml_character)))) (ml_text_bytes ml_rest) end) (ml_js_number parameters))))) (32%N :: 97%N :: 114%N :: 103%N :: 117%N :: 109%N :: 101%N :: 110%N :: 116%N :: 115%N :: (@nil N))) else ((@nil N)))).

Definition readJavaScriptLiteralPropertyForm (kind : (list N)) (index : float) : (list N) :=
  (if (andb (orb (orb (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (97%N :: 114%N :: 114%N :: 97%N :: 121%N :: (@nil N)))) 0%float) (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (116%N :: 117%N :: 112%N :: 108%N :: 101%N :: (@nil N)))) 0%float)) (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (115%N :: 116%N :: 114%N :: 105%N :: 110%N :: 103%N :: (@nil N)))) 0%float)) (PrimFloat.leb 0%float index)) then (105%N :: 110%N :: 100%N :: 101%N :: 120%N :: (@nil N)) else (102%N :: 105%N :: 101%N :: 108%N :: 100%N :: (@nil N))).

Definition readJavaScriptStringIndexNumberForm (kind : (list N)) (element : (list N)) : (list N) :=
  (if (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (102%N :: 108%N :: 111%N :: 97%N :: 116%N :: (@nil N)))) 0%float) then (105%N :: 100%N :: 101%N :: 110%N :: 116%N :: 105%N :: 116%N :: 121%N :: (@nil N)) else (if (orb (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (105%N :: 110%N :: 116%N :: (@nil N)))) 0%float) (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (110%N :: 97%N :: 116%N :: (@nil N)))) 0%float)) then (99%N :: 111%N :: 110%N :: 118%N :: 101%N :: 114%N :: 116%N :: (@nil N)) else (if (orb (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (117%N :: 110%N :: 105%N :: 116%N :: (@nil N)))) 0%float) (andb (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (111%N :: 112%N :: 116%N :: 105%N :: 111%N :: 110%N :: (@nil N)))) 0%float) (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) element) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (117%N :: 110%N :: 105%N :: 116%N :: (@nil N)))) 0%float))) then (97%N :: 98%N :: 115%N :: 101%N :: 110%N :: 116%N :: (@nil N)) else (if (andb (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (111%N :: 112%N :: 116%N :: 105%N :: 111%N :: 110%N :: (@nil N)))) 0%float) (orb (orb (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) element) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (102%N :: 108%N :: 111%N :: 97%N :: 116%N :: (@nil N)))) 0%float) (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) element) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (105%N :: 110%N :: 116%N :: (@nil N)))) 0%float)) (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) element) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (110%N :: 97%N :: 116%N :: (@nil N)))) 0%float))) then (111%N :: 112%N :: 116%N :: 105%N :: 111%N :: 110%N :: 97%N :: 108%N :: (@nil N)) else (114%N :: 101%N :: 102%N :: 117%N :: 115%N :: 101%N :: (@nil N)))))).

Definition refuseJavaScriptRecordOrderName : (list float) :=
  (ml_raiseJavaScriptTypeError (97%N :: 110%N :: 32%N :: 111%N :: 114%N :: 100%N :: 101%N :: 114%N :: 101%N :: 100%N :: 32%N :: 114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: 32%N :: 110%N :: 101%N :: 101%N :: 100%N :: 115%N :: 32%N :: 101%N :: 118%N :: 101%N :: 114%N :: 121%N :: 32%N :: 97%N :: 99%N :: 116%N :: 117%N :: 97%N :: 108%N :: 32%N :: 111%N :: 119%N :: 110%N :: 32%N :: 102%N :: 105%N :: 101%N :: 108%N :: 100%N :: 32%N :: 110%N :: 97%N :: 109%N :: 101%N :: (@nil N))).

Definition refuseJavaScriptRecordOrderPosition : float :=
  (ml_raiseJavaScriptTypeError (97%N :: 110%N :: 32%N :: 111%N :: 114%N :: 100%N :: 101%N :: 114%N :: 101%N :: 100%N :: 32%N :: 114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: 32%N :: 110%N :: 101%N :: 101%N :: 100%N :: 115%N :: 32%N :: 101%N :: 118%N :: 101%N :: 114%N :: 121%N :: 32%N :: 97%N :: 99%N :: 116%N :: 117%N :: 97%N :: 108%N :: 32%N :: 102%N :: 105%N :: 101%N :: 108%N :: 100%N :: 32%N :: 112%N :: 111%N :: 115%N :: 105%N :: 116%N :: 105%N :: 111%N :: 110%N :: (@nil N))).

Definition readJavaScriptRecordOrderName (names : (list (list float))) (index : float) : (list float) :=
  (match (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error names (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => refuseJavaScriptRecordOrderName end).

Definition readJavaScriptRecordOrderPosition (positions : (list float)) (index : float) : float :=
  (match (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error positions (Z.to_nat ml_index)) with | Some ml_value => ml_value | _ => refuseJavaScriptRecordOrderPosition end).

Definition source_ml_temporary_readRecordValueOrder_loop1 (count : float) (reverse : bool) (order : (list float)) (index : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : float * bool * (list float) * float -> (list float)) (ml_args : float * bool * (list float) * float) =>
    let '(count, reverse, order, index) := ml_args in (if (PrimFloat.ltb index count) then (let order_2 := (List.app order ((if reverse then (PrimFloat.sub (PrimFloat.sub count index) 1%float) else index) :: @nil float)) in (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (count, reverse, order_2, index_2)))) else order))
    (fun _ => (@nil float)) (count, reverse, order, index).

Definition readRecordValueOrder (count : float) (reverse : bool) : (list float) :=
  (if (orb (negb (ml_is_safe_integer count)) (PrimFloat.ltb count 0%float)) then ((PrimFloat.opp 1%float) :: @nil float) else (let order := (@nil float) in (let index := 0%float in (let order_2 := (source_ml_temporary_readRecordValueOrder_loop1 count reverse order index) in order_2)))).

Inductive source_ml_temporary_readRecordProjectionError_loop2_result : Type :=
| source_ml_temporary_readRecordProjectionError_loop2_done : source_ml_temporary_readRecordProjectionError_loop2_result
| source_ml_temporary_readRecordProjectionError_loop2_return : (list N) -> source_ml_temporary_readRecordProjectionError_loop2_result.

Inductive source_ml_temporary_acceptRecordFieldName_loop3_result : Type :=
| source_ml_temporary_acceptRecordFieldName_loop3_done : source_ml_temporary_acceptRecordFieldName_loop3_result
| source_ml_temporary_acceptRecordFieldName_loop3_return : bool -> source_ml_temporary_acceptRecordFieldName_loop3_result.

Definition source_ml_temporary_acceptRecordFieldName_loop3 (name : (list float)) (index : float) : source_ml_temporary_acceptRecordFieldName_loop3_result :=
  ml_fix 64 (fun (ml_rec : (list float) * float -> source_ml_temporary_acceptRecordFieldName_loop3_result) (ml_args : (list float) * float) =>
    let '(name, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length name))))) then (let unit_ := (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error name (Z.to_nat ml_index)) in (if (orb (orb (negb (match unit_ with | Some ml_present => (ml_is_integer ml_present) | None => false end)) (let source_ml_source1_comparison_operand18 := unit_ in (let source_ml_source1_comparison_operand19 := 0%float in (if (match (source_ml_source1_comparison_operand18, (@None float)) with | (Some ml_left, Some ml_right) => (PrimFloat.eqb ml_left ml_right) | (None, None) => true | _ => false end) then (PrimFloat.ltb PrimFloat.nan source_ml_source1_comparison_operand19) else (PrimFloat.ltb (match source_ml_source1_comparison_operand18 with | Some ml_present => ml_present | _ => 0%float end) source_ml_source1_comparison_operand19))))) (let source_ml_source1_comparison_operand20 := unit_ in (let source_ml_source1_comparison_operand21 := 65535%float in (if (match (source_ml_source1_comparison_operand20, (@None float)) with | (Some ml_left, Some ml_right) => (PrimFloat.eqb ml_left ml_right) | (None, None) => true | _ => false end) then (PrimFloat.ltb source_ml_source1_comparison_operand21 PrimFloat.nan) else (PrimFloat.ltb source_ml_source1_comparison_operand21 (match source_ml_source1_comparison_operand20 with | Some ml_present => ml_present | _ => 0%float end)))))) then (source_ml_temporary_acceptRecordFieldName_loop3_return false) else (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (name, index_2))))) else source_ml_temporary_acceptRecordFieldName_loop3_done))
    (fun _ => source_ml_temporary_acceptRecordFieldName_loop3_done) (name, index).

Definition acceptRecordFieldName (name : (list float)) : bool :=
  (let index := 0%float in (let source_ml_source1_s2 := (source_ml_temporary_acceptRecordFieldName_loop3 name index) in (match source_ml_source1_s2 with | source_ml_temporary_acceptRecordFieldName_loop3_done => true | source_ml_temporary_acceptRecordFieldName_loop3_return source_ml_temporary_result3 => source_ml_temporary_result3 end))).

Inductive source_ml_temporary_compareRecordFieldNames_loop4_result : Type :=
| source_ml_temporary_compareRecordFieldNames_loop4_done : source_ml_temporary_compareRecordFieldNames_loop4_result
| source_ml_temporary_compareRecordFieldNames_loop4_return : float -> source_ml_temporary_compareRecordFieldNames_loop4_result.

Definition source_ml_temporary_compareRecordFieldNames_loop4 (left_ : (list float)) (right_ : (list float)) (end_ : float) (index : float) : source_ml_temporary_compareRecordFieldNames_loop4_result :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float * float -> source_ml_temporary_compareRecordFieldNames_loop4_result) (ml_args : (list float) * (list float) * float * float) =>
    let '(left_, right_, end_, index) := ml_args in (if (PrimFloat.ltb index end_) then (if (PrimFloat.ltb (readRequiredNumberFact left_ index) (readRequiredNumberFact right_ index)) then (source_ml_temporary_compareRecordFieldNames_loop4_return (PrimFloat.opp 1%float)) else (if (PrimFloat.ltb (readRequiredNumberFact right_ index) (readRequiredNumberFact left_ index)) then (source_ml_temporary_compareRecordFieldNames_loop4_return 1%float) else (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (left_, right_, end_, index_2))))) else source_ml_temporary_compareRecordFieldNames_loop4_done))
    (fun _ => source_ml_temporary_compareRecordFieldNames_loop4_done) (left_, right_, end_, index).

Definition compareRecordFieldNames (left_ : (list float)) (right_ : (list float)) : float :=
  (let end_ := (if (PrimFloat.ltb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length left_)))) (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length right_))))) then (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length left_)))) else (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length right_))))) in (let index := 0%float in (let source_ml_source1_s3 := (source_ml_temporary_compareRecordFieldNames_loop4 left_ right_ end_ index) in (match source_ml_source1_s3 with | source_ml_temporary_compareRecordFieldNames_loop4_done => (if (PrimFloat.ltb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length left_)))) (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length right_))))) then (PrimFloat.opp 1%float) else (if (PrimFloat.ltb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length right_)))) (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length left_))))) then 1%float else 0%float)) | source_ml_temporary_compareRecordFieldNames_loop4_return source_ml_temporary_result4 => source_ml_temporary_result4 end)))).

Definition source_ml_temporary_readRecordProjectionError_loop2 (names : (list (list float))) (entries : (list bool)) (index : float) : source_ml_temporary_readRecordProjectionError_loop2_result :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * (list bool) * float -> source_ml_temporary_readRecordProjectionError_loop2_result) (ml_args : (list (list float)) * (list bool) * float) =>
    let '(names, entries, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length names))))) then (if (negb (match (let ml_index := (ml_float_index index) in if Z.ltb ml_index 0 then None else List.nth_error entries (Z.to_nat ml_index)) with | Some ml_present => ml_present | None => false end)) then (source_ml_temporary_readRecordProjectionError_loop2_return (99%N :: 104%N :: 101%N :: 99%N :: 107%N :: 101%N :: 100%N :: 32%N :: 114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: 32%N :: 112%N :: 114%N :: 111%N :: 106%N :: 101%N :: 99%N :: 116%N :: 105%N :: 111%N :: 110%N :: 32%N :: 104%N :: 97%N :: 115%N :: 32%N :: 97%N :: 32%N :: 109%N :: 97%N :: 108%N :: 102%N :: 111%N :: 114%N :: 109%N :: 101%N :: 100%N :: 32%N :: 102%N :: 105%N :: 101%N :: 108%N :: 100%N :: 32%N :: 101%N :: 110%N :: 116%N :: 114%N :: 121%N :: (@nil N))) else (if (negb (acceptRecordFieldName (readRequiredRecordName names index))) then (source_ml_temporary_readRecordProjectionError_loop2_return (99%N :: 104%N :: 101%N :: 99%N :: 107%N :: 101%N :: 100%N :: 32%N :: 114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: 32%N :: 112%N :: 114%N :: 111%N :: 106%N :: 101%N :: 99%N :: 116%N :: 105%N :: 111%N :: 110%N :: 32%N :: 104%N :: 97%N :: 115%N :: 32%N :: 97%N :: 110%N :: 32%N :: 105%N :: 110%N :: 118%N :: 97%N :: 108%N :: 105%N :: 100%N :: 32%N :: 85%N :: 84%N :: 70%N :: 49%N :: 54%N :: 32%N :: 102%N :: 105%N :: 101%N :: 108%N :: 100%N :: 32%N :: 110%N :: 97%N :: 109%N :: 101%N :: (@nil N))) else (if (andb (PrimFloat.ltb 0%float index) (PrimFloat.leb 0%float (compareRecordFieldNames (readRequiredRecordName names (PrimFloat.sub index 1%float)) (readRequiredRecordName names index)))) then (source_ml_temporary_readRecordProjectionError_loop2_return (99%N :: 104%N :: 101%N :: 99%N :: 107%N :: 101%N :: 100%N :: 32%N :: 114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: 32%N :: 112%N :: 114%N :: 111%N :: 106%N :: 101%N :: 99%N :: 116%N :: 105%N :: 111%N :: 110%N :: 32%N :: 110%N :: 101%N :: 101%N :: 100%N :: 115%N :: 32%N :: 117%N :: 110%N :: 105%N :: 113%N :: 117%N :: 101%N :: 32%N :: 102%N :: 105%N :: 101%N :: 108%N :: 100%N :: 115%N :: 32%N :: 105%N :: 110%N :: 32%N :: 85%N :: 84%N :: 70%N :: 49%N :: 54%N :: 32%N :: 111%N :: 114%N :: 100%N :: 101%N :: 114%N :: (@nil N))) else (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (names, entries, index_2)))))) else source_ml_temporary_readRecordProjectionError_loop2_done))
    (fun _ => source_ml_temporary_readRecordProjectionError_loop2_done) (names, entries, index).

Definition readRecordProjectionError (projection : (list N)) (names : (list (list float))) (entries : (list bool)) : (list N) :=
  (if (negb (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) projection) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) readRecordProjection)) 0%float)) then (117%N :: 110%N :: 107%N :: 110%N :: 111%N :: 119%N :: 110%N :: 32%N :: 99%N :: 104%N :: 101%N :: 99%N :: 107%N :: 101%N :: 100%N :: 32%N :: 114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: 32%N :: 112%N :: 114%N :: 111%N :: 106%N :: 101%N :: 99%N :: 116%N :: 105%N :: 111%N :: 110%N :: (@nil N)) else (if (negb (PrimFloat.eqb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length names)))) (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length entries)))))) then (99%N :: 104%N :: 101%N :: 99%N :: 107%N :: 101%N :: 100%N :: 32%N :: 114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: 32%N :: 112%N :: 114%N :: 111%N :: 106%N :: 101%N :: 99%N :: 116%N :: 105%N :: 111%N :: 110%N :: 32%N :: 104%N :: 97%N :: 115%N :: 32%N :: 105%N :: 110%N :: 99%N :: 111%N :: 110%N :: 115%N :: 105%N :: 115%N :: 116%N :: 101%N :: 110%N :: 116%N :: 32%N :: 101%N :: 110%N :: 116%N :: 114%N :: 121%N :: 32%N :: 102%N :: 97%N :: 99%N :: 116%N :: 115%N :: (@nil N)) else (let index := 0%float in (let source_ml_source1_s1 := (source_ml_temporary_readRecordProjectionError_loop2 names entries index) in (match source_ml_source1_s1 with | source_ml_temporary_readRecordProjectionError_loop2_done => ((@nil N)) | source_ml_temporary_readRecordProjectionError_loop2_return source_ml_temporary_result2 => source_ml_temporary_result2 end))))).

Inductive source_ml_temporary_readRecordFieldPosition_loop6_result : Type :=
| source_ml_temporary_readRecordFieldPosition_loop6_done : source_ml_temporary_readRecordFieldPosition_loop6_result
| source_ml_temporary_readRecordFieldPosition_loop6_return : float -> source_ml_temporary_readRecordFieldPosition_loop6_result.

Definition source_ml_temporary_readRecordFieldPosition_loop6 (fields : (list (list float))) (name : (list float)) (index : float) : source_ml_temporary_readRecordFieldPosition_loop6_result :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * (list float) * float -> source_ml_temporary_readRecordFieldPosition_loop6_result) (ml_args : (list (list float)) * (list float) * float) =>
    let '(fields, name, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length fields))))) then (let comparison := (compareRecordFieldNames (readRequiredRecordName fields index) name) in (if (andb (negb (PrimFloat.ltb comparison 0%float)) (negb (PrimFloat.ltb 0%float comparison))) then (source_ml_temporary_readRecordFieldPosition_loop6_return index) else (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (fields, name, index_2))))) else source_ml_temporary_readRecordFieldPosition_loop6_done))
    (fun _ => source_ml_temporary_readRecordFieldPosition_loop6_done) (fields, name, index).

Inductive source_ml_temporary_readRecordFieldPosition_loop5_result : Type :=
| source_ml_temporary_readRecordFieldPosition_loop5_done : source_ml_temporary_readRecordFieldPosition_loop5_result
| source_ml_temporary_readRecordFieldPosition_loop5_return : float -> source_ml_temporary_readRecordFieldPosition_loop5_result.

Definition source_ml_temporary_readRecordFieldPosition_loop5 (fields : (list (list float))) (index : float) : source_ml_temporary_readRecordFieldPosition_loop5_result :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * float -> source_ml_temporary_readRecordFieldPosition_loop5_result) (ml_args : (list (list float)) * float) =>
    let '(fields, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length fields))))) then (if (negb (acceptRecordFieldName (readRequiredRecordName fields index))) then (source_ml_temporary_readRecordFieldPosition_loop5_return (PrimFloat.opp 1%float)) else (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (fields, index_2)))) else source_ml_temporary_readRecordFieldPosition_loop5_done))
    (fun _ => source_ml_temporary_readRecordFieldPosition_loop5_done) (fields, index).

Definition readRecordFieldPosition (fields : (list (list float))) (name : (list float)) : float :=
  (if (negb (acceptRecordFieldName name)) then (PrimFloat.opp 1%float) else (let index := 0%float in (let source_ml_source1_s4 := (source_ml_temporary_readRecordFieldPosition_loop5 fields index) in (match source_ml_source1_s4 with | source_ml_temporary_readRecordFieldPosition_loop5_done => (let index_2 := 0%float in (let source_ml_source1_s5 := (source_ml_temporary_readRecordFieldPosition_loop6 fields name index_2) in (match source_ml_source1_s5 with | source_ml_temporary_readRecordFieldPosition_loop6_done => (PrimFloat.opp 1%float) | source_ml_temporary_readRecordFieldPosition_loop6_return source_ml_temporary_result6 => source_ml_temporary_result6 end))) | source_ml_temporary_readRecordFieldPosition_loop5_return source_ml_temporary_result6_2 => source_ml_temporary_result6_2 end)))).

Definition readRecordTypeConstraintStep (left_ : (list (list float))) (right_ : (list (list float))) (leftOpen : bool) (rightOpen : bool) (phase : float) (index : float) : (list float) :=
  (if (PrimFloat.ltb phase 1%float) then (if (PrimFloat.leb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length left_)))) index) then (5%float :: (PrimFloat.opp 1%float) :: (PrimFloat.opp 1%float) :: 1%float :: 0%float :: @nil float) else (let matching := (readRecordFieldPosition right_ (readRequiredRecordName left_ index)) in (if (PrimFloat.leb 0%float matching) then (0%float :: index :: matching :: 0%float :: (PrimFloat.add index 1%float) :: @nil float) else ((if rightOpen then 1%float else 3%float) :: index :: (PrimFloat.opp 1%float) :: 0%float :: (PrimFloat.add index 1%float) :: @nil float)))) else (if (PrimFloat.leb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length right_)))) index) then (6%float :: (PrimFloat.opp 1%float) :: (PrimFloat.opp 1%float) :: 1%float :: index :: @nil float) else (let matching_2 := (readRecordFieldPosition left_ (readRequiredRecordName right_ index)) in (if (PrimFloat.leb 0%float matching_2) then (5%float :: matching_2 :: index :: 1%float :: (PrimFloat.add index 1%float) :: @nil float) else ((if leftOpen then 2%float else 4%float) :: (PrimFloat.opp 1%float) :: index :: 1%float :: (PrimFloat.add index 1%float) :: @nil float))))).

Definition readRecordFieldInferenceStep (fields : (list (list float))) (name : (list float)) (open : bool) : (list float) :=
  (let index := (readRecordFieldPosition fields name) in (if (PrimFloat.leb 0%float index) then (0%float :: index :: @nil float) else ((if open then 1%float else 2%float) :: (PrimFloat.opp 1%float) :: @nil float))).

Definition source_ml_temporary_readRecordSortedFieldPositions_loop10 (fields : (list (list float))) (sorted : (list float)) (merged : (list float)) (middle : float) (end_ : float) (left_ : float) (right_ : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * (list float) * (list float) * float * float * float * float -> (list float)) (ml_args : (list (list float)) * (list float) * (list float) * float * float * float * float) =>
    let '(fields, sorted, merged, middle, end_, left_, right_) := ml_args in (if (orb (PrimFloat.ltb left_ middle) (PrimFloat.ltb right_ end_)) then (if (orb (PrimFloat.leb end_ right_) (andb (PrimFloat.ltb left_ middle) (PrimFloat.leb (compareRecordFieldNames (readRequiredRecordName fields (readRequiredNumberFact sorted left_)) (readRequiredRecordName fields (readRequiredNumberFact sorted right_))) 0%float))) then (let merged_2 := (List.app merged ((readRequiredNumberFact sorted left_) :: @nil float)) in (let left_2 := (PrimFloat.add left_ 1%float) in (ml_rec (fields, sorted, merged_2, middle, end_, left_2, right_)))) else (let merged_3 := (List.app merged ((readRequiredNumberFact sorted right_) :: @nil float)) in (let right_2 := (PrimFloat.add right_ 1%float) in (ml_rec (fields, sorted, merged_3, middle, end_, left_, right_2))))) else merged))
    (fun _ => (@nil float)) (fields, sorted, merged, middle, end_, left_, right_).

Definition source_ml_temporary_readRecordSortedFieldPositions_loop9 (fields : (list (list float))) (sorted : (list float)) (width : float) (merged : (list float)) (start : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * (list float) * float * (list float) * float -> (list float)) (ml_args : (list (list float)) * (list float) * float * (list float) * float) =>
    let '(fields, sorted, width, merged, start) := ml_args in (if (PrimFloat.ltb start (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length sorted))))) then (let middle := (if (PrimFloat.ltb (PrimFloat.add start width) (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length sorted))))) then (PrimFloat.add start width) else (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length sorted))))) in (let end_ := (if (PrimFloat.ltb (PrimFloat.add middle width) (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length sorted))))) then (PrimFloat.add middle width) else (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length sorted))))) in (let left_ := start in (let right_ := middle in (let merged_2 := (source_ml_temporary_readRecordSortedFieldPositions_loop10 fields sorted merged middle end_ left_ right_) in (let start_2 := (PrimFloat.add start (PrimFloat.mul width 2%float)) in (ml_rec (fields, sorted, width, merged_2, start_2)))))))) else merged))
    (fun _ => (@nil float)) (fields, sorted, width, merged, start).

Definition source_ml_temporary_readRecordSortedFieldPositions_loop8 (fields : (list (list float))) (sorted : (list float)) (width : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * (list float) * float -> (list float)) (ml_args : (list (list float)) * (list float) * float) =>
    let '(fields, sorted, width) := ml_args in (if (PrimFloat.ltb width (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length sorted))))) then (let merged := (@nil float) in (let start := 0%float in (let merged_2 := (source_ml_temporary_readRecordSortedFieldPositions_loop9 fields sorted width merged start) in (let sorted_2 := merged_2 in (let width_2 := (PrimFloat.mul width 2%float) in (ml_rec (fields, sorted_2, width_2))))))) else sorted))
    (fun _ => (@nil float)) (fields, sorted, width).

Inductive source_ml_temporary_readRecordSortedFieldPositions_loop7_result : Type :=
| source_ml_temporary_readRecordSortedFieldPositions_loop7_done : source_ml_temporary_readRecordSortedFieldPositions_loop7_result
| source_ml_temporary_readRecordSortedFieldPositions_loop7_return : (list float) -> source_ml_temporary_readRecordSortedFieldPositions_loop7_result.

Definition source_ml_temporary_readRecordSortedFieldPositions_loop7 (fields : (list (list float))) (index : float) : source_ml_temporary_readRecordSortedFieldPositions_loop7_result :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * float -> source_ml_temporary_readRecordSortedFieldPositions_loop7_result) (ml_args : (list (list float)) * float) =>
    let '(fields, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length fields))))) then (if (negb (acceptRecordFieldName (readRequiredRecordName fields index))) then (source_ml_temporary_readRecordSortedFieldPositions_loop7_return ((PrimFloat.opp 1%float) :: @nil float)) else (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (fields, index_2)))) else source_ml_temporary_readRecordSortedFieldPositions_loop7_done))
    (fun _ => source_ml_temporary_readRecordSortedFieldPositions_loop7_done) (fields, index).

Definition readRecordSortedFieldPositions (fields : (list (list float))) : (list float) :=
  (let index := 0%float in (let source_ml_source1_s6 := (source_ml_temporary_readRecordSortedFieldPositions_loop7 fields index) in (match source_ml_source1_s6 with | source_ml_temporary_readRecordSortedFieldPositions_loop7_done => (let sorted := (readRecordValueOrder (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length fields)))) false) in (let width := 1%float in (let sorted_2 := (source_ml_temporary_readRecordSortedFieldPositions_loop8 fields sorted width) in sorted_2))) | source_ml_temporary_readRecordSortedFieldPositions_loop7_return source_ml_temporary_result10 => source_ml_temporary_result10 end))).

Definition source_ml_temporary_readRecordFieldOrder_loop11 (fields : (list (list float))) (sorted : (list float)) (selected : (list float)) (index : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * (list float) * (list float) * float -> (list float)) (ml_args : (list (list float)) * (list float) * (list float) * float) =>
    let '(fields, sorted, selected, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length sorted))))) then (if (orb (PrimFloat.leb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length sorted)))) (PrimFloat.add index 1%float)) (PrimFloat.ltb (compareRecordFieldNames (readRequiredRecordName fields (readRequiredNumberFact sorted index)) (readRequiredRecordName fields (readRequiredNumberFact sorted (PrimFloat.add index 1%float)))) 0%float)) then (let selected_2 := (List.app selected ((readRequiredNumberFact sorted index) :: @nil float)) in (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (fields, sorted, selected_2, index_2)))) else (let index_3 := (PrimFloat.add index 1%float) in (ml_rec (fields, sorted, selected, index_3)))) else selected))
    (fun _ => (@nil float)) (fields, sorted, selected, index).

Definition readRecordFieldOrder (fields : (list (list float))) : (list float) :=
  (let sorted := (readRecordSortedFieldPositions fields) in (if (andb (PrimFloat.ltb 0%float (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length sorted))))) (PrimFloat.ltb (readRequiredNumberFact sorted 0%float) 0%float)) then ((PrimFloat.opp 1%float) :: @nil float) else (let selected := (@nil float) in (let index := 0%float in (let selected_2 := (source_ml_temporary_readRecordFieldOrder_loop11 fields sorted selected index) in selected_2))))).

Definition source_ml_temporary_readRecordFieldInsertionOrder_loop14 (fields : (list (list float))) (first : (list float)) (last : (list float)) (selected : (list float)) (index : float) (lower : float) (upper : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * (list float) * (list float) * (list float) * float * float * float -> (list float)) (ml_args : (list (list float)) * (list float) * (list float) * (list float) * float * float * float) =>
    let '(fields, first, last, selected, index, lower, upper) := ml_args in (if (PrimFloat.ltb lower upper) then (let middle := (ml_floor (PrimFloat.div (PrimFloat.add lower upper) 2%float)) in (let comparison := (compareRecordFieldNames (readRequiredRecordName fields index) (readRequiredRecordName fields (readRequiredNumberFact last middle))) in (if (PrimFloat.ltb comparison 0%float) then (let upper_2 := middle in (ml_rec (fields, first, last, selected, index, lower, upper_2))) else (if (PrimFloat.ltb 0%float comparison) then (let lower_2 := (PrimFloat.add middle 1%float) in (ml_rec (fields, first, last, selected, index, lower_2, upper))) else (let selected_3 := (if (andb (negb (PrimFloat.ltb (readRequiredNumberFact first middle) index)) (negb (PrimFloat.ltb index (readRequiredNumberFact first middle)))) then (let selected_2 := (List.app selected ((readRequiredNumberFact last middle) :: @nil float)) in selected_2) else selected) in (let lower_3 := upper in (ml_rec (fields, first, last, selected_3, index, lower_3, upper)))))))) else selected))
    (fun _ => (@nil float)) (fields, first, last, selected, index, lower, upper).

Definition source_ml_temporary_readRecordFieldInsertionOrder_loop13 (fields : (list (list float))) (first : (list float)) (last : (list float)) (selected : (list float)) (index : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * (list float) * (list float) * (list float) * float -> (list float)) (ml_args : (list (list float)) * (list float) * (list float) * (list float) * float) =>
    let '(fields, first, last, selected, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length fields))))) then (let lower := 0%float in (let upper := (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length last)))) in (let selected_2 := (source_ml_temporary_readRecordFieldInsertionOrder_loop14 fields first last selected index lower upper) in (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (fields, first, last, selected_2, index_2)))))) else selected))
    (fun _ => (@nil float)) (fields, first, last, selected, index).

Inductive source_ml_temporary_readRecordFieldInsertionOrder_loop12_result : Type :=
| source_ml_temporary_readRecordFieldInsertionOrder_loop12_done : (list float) -> (list float) -> source_ml_temporary_readRecordFieldInsertionOrder_loop12_result.

Definition source_ml_temporary_readRecordFieldInsertionOrder_loop12 (fields : (list (list float))) (sorted : (list float)) (first : (list float)) (last : (list float)) (start : float) (index : float) : source_ml_temporary_readRecordFieldInsertionOrder_loop12_result :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * (list float) * (list float) * (list float) * float * float -> source_ml_temporary_readRecordFieldInsertionOrder_loop12_result) (ml_args : (list (list float)) * (list float) * (list float) * (list float) * float * float) =>
    let '(fields, sorted, first, last, start, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length sorted))))) then (if (orb (PrimFloat.leb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length sorted)))) (PrimFloat.add index 1%float)) (PrimFloat.ltb (compareRecordFieldNames (readRequiredRecordName fields (readRequiredNumberFact sorted index)) (readRequiredRecordName fields (readRequiredNumberFact sorted (PrimFloat.add index 1%float)))) 0%float)) then (let first_2 := (List.app first ((readRequiredNumberFact sorted start) :: @nil float)) in (let last_2 := (List.app last ((readRequiredNumberFact sorted index) :: @nil float)) in (let start_2 := (PrimFloat.add index 1%float) in (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (fields, sorted, first_2, last_2, start_2, index_2)))))) else (let index_3 := (PrimFloat.add index 1%float) in (ml_rec (fields, sorted, first, last, start, index_3)))) else (source_ml_temporary_readRecordFieldInsertionOrder_loop12_done first last)))
    (fun _ => (source_ml_temporary_readRecordFieldInsertionOrder_loop12_done (@nil float) (@nil float))) (fields, sorted, first, last, start, index).

Definition readRecordFieldInsertionOrder (fields : (list (list float))) : (list float) :=
  (let sorted := (readRecordSortedFieldPositions fields) in (if (andb (PrimFloat.ltb 0%float (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length sorted))))) (PrimFloat.ltb (readRequiredNumberFact sorted 0%float) 0%float)) then ((PrimFloat.opp 1%float) :: @nil float) else (let first := (@nil float) in (let last := (@nil float) in (let start := 0%float in (let index := 0%float in (let source_ml_source1_s7 := (source_ml_temporary_readRecordFieldInsertionOrder_loop12 fields sorted first last start index) in (match source_ml_source1_s7 with | source_ml_temporary_readRecordFieldInsertionOrder_loop12_done first_2 last_2 => (let selected := (@nil float) in (let index_2 := 0%float in (let selected_2 := (source_ml_temporary_readRecordFieldInsertionOrder_loop13 fields first_2 last_2 selected index_2) in selected_2))) end)))))))).

Inductive source_ml_temporary_recordTypeFactField_loop15_result : Type :=
| source_ml_temporary_recordTypeFactField_loop15_done : source_ml_temporary_recordTypeFactField_loop15_result
| source_ml_temporary_recordTypeFactField_loop15_return : float -> source_ml_temporary_recordTypeFactField_loop15_result.

Definition source_ml_temporary_recordTypeFactField_loop15 (labels : (list (list N))) (targets : (list float)) (name : (list N)) (end_ : float) (edge : float) : source_ml_temporary_recordTypeFactField_loop15_result :=
  ml_fix 64 (fun (ml_rec : (list (list N)) * (list float) * (list N) * float * float -> source_ml_temporary_recordTypeFactField_loop15_result) (ml_args : (list (list N)) * (list float) * (list N) * float * float) =>
    let '(labels, targets, name, end_, edge) := ml_args in (if (PrimFloat.ltb edge end_) then (if (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (readRequiredTextFact labels edge)) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) name)) 0%float) then (source_ml_temporary_recordTypeFactField_loop15_return (readRequiredNumberFact targets edge)) else (let edge_2 := (PrimFloat.add edge 1%float) in (ml_rec (labels, targets, name, end_, edge_2)))) else source_ml_temporary_recordTypeFactField_loop15_done))
    (fun _ => source_ml_temporary_recordTypeFactField_loop15_done) (labels, targets, name, end_, edge).

Definition recordTypeFactField (starts : (list float)) (counts : (list float)) (labels : (list (list N))) (targets : (list float)) (owner : float) (name : (list N)) : float :=
  (if (PrimFloat.ltb owner 0%float) then (PrimFloat.opp 1%float) else (let start := (readRequiredNumberFact starts owner) in (let end_ := (PrimFloat.add start (readRequiredNumberFact counts owner)) in (let edge := start in (let source_ml_source1_s8 := (source_ml_temporary_recordTypeFactField_loop15 labels targets name end_ edge) in (match source_ml_source1_s8 with | source_ml_temporary_recordTypeFactField_loop15_done => (PrimFloat.opp 1%float) | source_ml_temporary_recordTypeFactField_loop15_return source_ml_temporary_result15 => source_ml_temporary_result15 end)))))).

Definition recordTypeFactKind (texts : (list (list N))) (starts : (list float)) (counts : (list float)) (labels : (list (list N))) (targets : (list float)) (owner : float) : (list N) :=
  (let kind := (recordTypeFactField starts counts labels targets owner (107%N :: 105%N :: 110%N :: 100%N :: (@nil N))) in (if (PrimFloat.ltb kind 0%float) then ((@nil N)) else (readRequiredTextFact texts kind))).

Definition source_ml_temporary_recordTypeFactElements_loop16 (targets : (list float)) (values : (list float)) (end_ : float) (edge : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float * float -> (list float)) (ml_args : (list float) * (list float) * float * float) =>
    let '(targets, values, end_, edge) := ml_args in (if (PrimFloat.ltb edge end_) then (let values_2 := (List.app values ((readRequiredNumberFact targets edge) :: @nil float)) in (let edge_2 := (PrimFloat.add edge 1%float) in (ml_rec (targets, values_2, end_, edge_2)))) else values))
    (fun _ => (@nil float)) (targets, values, end_, edge).

Definition recordTypeFactElements (starts : (list float)) (counts : (list float)) (targets : (list float)) (owner : float) : (list float) :=
  (let values := (@nil float) in (if (PrimFloat.ltb owner 0%float) then values else (let start := (readRequiredNumberFact starts owner) in (let end_ := (PrimFloat.add start (readRequiredNumberFact counts owner)) in (let edge := start in (let values_2 := (source_ml_temporary_recordTypeFactElements_loop16 targets values end_ edge) in values_2)))))).

Inductive source_ml_temporary_readRecordTypeOwnerPresence_loop17_result : Type :=
| source_ml_temporary_readRecordTypeOwnerPresence_loop17_done : source_ml_temporary_readRecordTypeOwnerPresence_loop17_result
| source_ml_temporary_readRecordTypeOwnerPresence_loop17_return : bool -> source_ml_temporary_readRecordTypeOwnerPresence_loop17_result.

Definition source_ml_temporary_readRecordTypeOwnerPresence_loop19 (seen : (list float)) (node : float) (visited : bool) (index : float) : bool :=
  ml_fix 64 (fun (ml_rec : (list float) * float * bool * float -> bool) (ml_args : (list float) * float * bool * float) =>
    let '(seen, node, visited, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length seen))))) then (if (PrimFloat.eqb (readRequiredNumberFact seen index) node) then (let visited_2 := true in (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (seen, node, visited_2, index_2)))) else (let index_3 := (PrimFloat.add index 1%float) in (ml_rec (seen, node, visited, index_3)))) else visited))
    (fun _ => false) (seen, node, visited, index).

Definition source_ml_temporary_readRecordTypeOwnerPresence_loop20 (pending : (list float)) (children : (list float)) (index : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float -> (list float)) (ml_args : (list float) * (list float) * float) =>
    let '(pending, children, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length children))))) then (let pending_2 := (List.app pending ((readRequiredNumberFact children index) :: @nil float)) in (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (pending_2, children, index_2)))) else pending))
    (fun _ => (@nil float)) (pending, children, index).

Definition source_ml_temporary_readRecordTypeOwnerPresence_loop21 (starts : (list float)) (counts : (list float)) (labels : (list (list N))) (targets : (list float)) (pending : (list float)) (fields : (list float)) (index : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * (list (list N)) * (list float) * (list float) * (list float) * float -> (list float)) (ml_args : (list float) * (list float) * (list (list N)) * (list float) * (list float) * (list float) * float) =>
    let '(starts, counts, labels, targets, pending, fields, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length fields))))) then (let pending_2 := (List.app pending ((recordTypeFactField starts counts labels targets (readRequiredNumberFact fields index) (116%N :: 121%N :: 112%N :: 101%N :: (@nil N))) :: @nil float)) in (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (starts, counts, labels, targets, pending_2, fields, index_2)))) else pending))
    (fun _ => (@nil float)) (starts, counts, labels, targets, pending, fields, index).

Inductive source_ml_temporary_readRecordTypeOwnerPresence_loop18_result : Type :=
| source_ml_temporary_readRecordTypeOwnerPresence_loop18_done : float -> (list N) -> source_ml_temporary_readRecordTypeOwnerPresence_loop18_result.

Definition source_ml_temporary_readRecordTypeOwnerPresence_loop18 (texts : (list (list N))) (numbers : (list float)) (starts : (list float)) (counts : (list float)) (labels : (list (list N))) (targets : (list float)) (bindings : (list float)) (node : float) (kind : (list N)) : source_ml_temporary_readRecordTypeOwnerPresence_loop18_result :=
  ml_fix 64 (fun (ml_rec : (list (list N)) * (list float) * (list float) * (list float) * (list (list N)) * (list float) * (list float) * float * (list N) -> source_ml_temporary_readRecordTypeOwnerPresence_loop18_result) (ml_args : (list (list N)) * (list float) * (list float) * (list float) * (list (list N)) * (list float) * (list float) * float * (list N)) =>
    let '(texts, numbers, starts, counts, labels, targets, bindings, node, kind) := ml_args in (if (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (118%N :: 97%N :: 114%N :: (@nil N)))) 0%float) then (let id := (recordTypeFactField starts counts labels targets node (105%N :: 100%N :: (@nil N))) in (let variable := (readRequiredNumberFact numbers id) in (if (orb (orb (negb (ml_is_integer variable)) (PrimFloat.ltb variable 0%float)) (PrimFloat.leb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length bindings)))) variable)) then (source_ml_temporary_readRecordTypeOwnerPresence_loop18_done node kind) else (let bound := (readRequiredNumberFact bindings variable) in (let boundKind := (recordTypeFactKind texts starts counts labels targets bound) in (if (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) boundKind) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) ((@nil N)))) 0%float) then (source_ml_temporary_readRecordTypeOwnerPresence_loop18_done node kind) else (let node_2 := bound in (let kind_2 := boundKind in (ml_rec (texts, numbers, starts, counts, labels, targets, bindings, node_2, kind_2)))))))))) else (source_ml_temporary_readRecordTypeOwnerPresence_loop18_done node kind)))
    (fun _ => (source_ml_temporary_readRecordTypeOwnerPresence_loop18_done 0%float (@nil N))) (texts, numbers, starts, counts, labels, targets, bindings, node, kind).

Definition source_ml_temporary_readRecordTypeOwnerPresence_loop17 (texts : (list (list N))) (numbers : (list float)) (starts : (list float)) (counts : (list float)) (labels : (list (list N))) (targets : (list float)) (bindings : (list float)) (owner : float) (pending : (list float)) (seen : (list float)) (position : float) : source_ml_temporary_readRecordTypeOwnerPresence_loop17_result :=
  ml_fix 64 (fun (ml_rec : (list (list N)) * (list float) * (list float) * (list float) * (list (list N)) * (list float) * (list float) * float * (list float) * (list float) * float -> source_ml_temporary_readRecordTypeOwnerPresence_loop17_result) (ml_args : (list (list N)) * (list float) * (list float) * (list float) * (list (list N)) * (list float) * (list float) * float * (list float) * (list float) * float) =>
    let '(texts, numbers, starts, counts, labels, targets, bindings, owner, pending, seen, position) := ml_args in (if (PrimFloat.ltb position (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length pending))))) then (let node := (readRequiredNumberFact pending position) in (let kind := (recordTypeFactKind texts starts counts labels targets node) in (let source_ml_source1_s22 := (source_ml_temporary_readRecordTypeOwnerPresence_loop18 texts numbers starts counts labels targets bindings node kind) in (match source_ml_source1_s22 with | source_ml_temporary_readRecordTypeOwnerPresence_loop18_done node_2 kind_2 => (if (PrimFloat.eqb node_2 owner) then (source_ml_temporary_readRecordTypeOwnerPresence_loop17_return true) else (let visited := false in (let index := 0%float in (let visited_2 := (source_ml_temporary_readRecordTypeOwnerPresence_loop19 seen node_2 visited index) in (if visited_2 then (let position_2 := (PrimFloat.add position 1%float) in (ml_rec (texts, numbers, starts, counts, labels, targets, bindings, owner, pending, seen, position_2))) else (let seen_2 := (List.app seen (node_2 :: @nil float)) in (if (orb (orb (orb (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind_2) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (97%N :: 114%N :: 114%N :: 97%N :: 121%N :: (@nil N)))) 0%float) (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind_2) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (111%N :: 112%N :: 116%N :: 105%N :: 111%N :: 110%N :: (@nil N)))) 0%float)) (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind_2) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (115%N :: 101%N :: 116%N :: (@nil N)))) 0%float)) (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind_2) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (99%N :: 101%N :: 108%N :: 108%N :: (@nil N)))) 0%float)) then (let pending_2 := (List.app pending ((recordTypeFactField starts counts labels targets node_2 (101%N :: 108%N :: 101%N :: 109%N :: 101%N :: 110%N :: 116%N :: (@nil N))) :: @nil float)) in (let position_3 := (PrimFloat.add position 1%float) in (ml_rec (texts, numbers, starts, counts, labels, targets, bindings, owner, pending_2, seen_2, position_3)))) else (if (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind_2) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (109%N :: 97%N :: 112%N :: (@nil N)))) 0%float) then (let pending_3 := (List.app pending ((recordTypeFactField starts counts labels targets node_2 (107%N :: 101%N :: 121%N :: (@nil N))) :: @nil float)) in (let pending_4 := (List.app pending_3 ((recordTypeFactField starts counts labels targets node_2 (118%N :: 97%N :: 108%N :: 117%N :: 101%N :: (@nil N))) :: @nil float)) in (let position_4 := (PrimFloat.add position 1%float) in (ml_rec (texts, numbers, starts, counts, labels, targets, bindings, owner, pending_4, seen_2, position_4))))) else (if (orb (orb (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind_2) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (116%N :: 117%N :: 112%N :: 108%N :: 101%N :: (@nil N)))) 0%float) (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind_2) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (117%N :: 110%N :: 105%N :: 111%N :: 110%N :: (@nil N)))) 0%float)) (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind_2) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (102%N :: 117%N :: 110%N :: 99%N :: 116%N :: 105%N :: 111%N :: 110%N :: (@nil N)))) 0%float)) then (let field := (if (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind_2) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (116%N :: 117%N :: 112%N :: 108%N :: 101%N :: (@nil N)))) 0%float) then (101%N :: 108%N :: 101%N :: 109%N :: 101%N :: 110%N :: 116%N :: 115%N :: (@nil N)) else (if (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind_2) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (117%N :: 110%N :: 105%N :: 111%N :: 110%N :: (@nil N)))) 0%float) then (97%N :: 108%N :: 116%N :: 101%N :: 114%N :: 110%N :: 97%N :: 116%N :: 105%N :: 118%N :: 101%N :: 115%N :: (@nil N)) else (112%N :: 97%N :: 114%N :: 97%N :: 109%N :: 115%N :: (@nil N)))) in (let sequence := (recordTypeFactField starts counts labels targets node_2 field) in (let children := (recordTypeFactElements starts counts targets sequence) in (let index_2 := 0%float in (let pending_5 := (source_ml_temporary_readRecordTypeOwnerPresence_loop20 pending children index_2) in (if (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind_2) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (102%N :: 117%N :: 110%N :: 99%N :: 116%N :: 105%N :: 111%N :: 110%N :: (@nil N)))) 0%float) then (let pending_6 := (List.app pending_5 ((recordTypeFactField starts counts labels targets node_2 (114%N :: 101%N :: 116%N :: (@nil N))) :: @nil float)) in (let position_5 := (PrimFloat.add position 1%float) in (ml_rec (texts, numbers, starts, counts, labels, targets, bindings, owner, pending_6, seen_2, position_5)))) else (let position_6 := (PrimFloat.add position 1%float) in (ml_rec (texts, numbers, starts, counts, labels, targets, bindings, owner, pending_5, seen_2, position_6))))))))) else (if (PrimFloat.eqb (MLUtf16Text.compare (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) kind_2) (List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) (114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: (@nil N)))) 0%float) then (let sequence_2 := (recordTypeFactField starts counts labels targets node_2 (102%N :: 105%N :: 101%N :: 108%N :: 100%N :: 115%N :: (@nil N))) in (let fields := (recordTypeFactElements starts counts targets sequence_2) in (let index_3 := 0%float in (let pending_7 := (source_ml_temporary_readRecordTypeOwnerPresence_loop21 starts counts labels targets pending fields index_3) in (let position_7 := (PrimFloat.add position 1%float) in (ml_rec (texts, numbers, starts, counts, labels, targets, bindings, owner, pending_7, seen_2, position_7))))))) else (let position_8 := (PrimFloat.add position 1%float) in (ml_rec (texts, numbers, starts, counts, labels, targets, bindings, owner, pending, seen_2, position_8))))))))))))) end)))) else source_ml_temporary_readRecordTypeOwnerPresence_loop17_done))
    (fun _ => source_ml_temporary_readRecordTypeOwnerPresence_loop17_done) (texts, numbers, starts, counts, labels, targets, bindings, owner, pending, seen, position).

Definition readRecordTypeOwnerPresence (texts : (list (list N))) (numbers : (list float)) (starts : (list float)) (counts : (list float)) (labels : (list (list N))) (targets : (list float)) (bindings : (list float)) (root : float) (owner : float) : bool :=
  (let pending := (root :: @nil float) in (let seen := (@nil float) in (let position := 0%float in (let source_ml_source1_s9 := (source_ml_temporary_readRecordTypeOwnerPresence_loop17 texts numbers starts counts labels targets bindings owner pending seen position) in (match source_ml_source1_s9 with | source_ml_temporary_readRecordTypeOwnerPresence_loop17_done => false | source_ml_temporary_readRecordTypeOwnerPresence_loop17_return source_ml_temporary_result17 => source_ml_temporary_result17 end))))).

Inductive source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_result : Type :=
| source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_done : float -> source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_result
| source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_return : float -> source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_result.

Definition source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22 (key : (list float)) (value : float) (at_ : float) : source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_result :=
  ml_fix 64 (fun (ml_rec : (list float) * float * float -> source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_result) (ml_args : (list float) * float * float) =>
    let '(key, value, at_) := ml_args in (if (PrimFloat.ltb at_ (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length key))))) then (let digit := (PrimFloat.sub (match (let ml_index := (ml_float_index at_) in if Z.ltb ml_index 0 then None else List.nth_error key (Z.to_nat ml_index)) with | Some ml_present => ml_present | None => PrimFloat.nan end) 48%float) in (if (orb (orb (negb (ml_is_integer digit)) (PrimFloat.ltb digit 0%float)) (PrimFloat.ltb 9%float digit)) then (source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_return (PrimFloat.opp 1%float)) else (let value_2 := (PrimFloat.add (PrimFloat.mul value 10%float) digit) in (let at_2 := (PrimFloat.add at_ 1%float) in (ml_rec (key, value_2, at_2)))))) else (source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_done value)))
    (fun _ => (source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_done 0%float)) (key, value, at_).

Definition readJavaScriptLiteralPropertyIndex (key : (list float)) : float :=
  (if (orb (PrimFloat.eqb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length key)))) 0%float) (PrimFloat.ltb 10%float (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length key)))))) then (PrimFloat.opp 1%float) else (if (andb (PrimFloat.ltb 1%float (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length key))))) (let source_ml_source1_comparison_operand10 := (let ml_index := (ml_float_index 0%float) in if Z.ltb ml_index 0 then None else List.nth_error key (Z.to_nat ml_index)) in (let source_ml_source1_comparison_operand11 := 48%float in (match (source_ml_source1_comparison_operand10, source_ml_source1_comparison_operand11) with | (Some ml_left, ml_right) => (PrimFloat.eqb ml_left ml_right) | _ => false end)))) then (PrimFloat.opp 1%float) else (let value := 0%float in (let at_ := 0%float in (let source_ml_source1_s12 := (source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22 key value at_) in (match source_ml_source1_s12 with | source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_done value_2 => (if (PrimFloat.leb value_2 4294967294%float) then value_2 else (PrimFloat.opp 1%float)) | source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_return source_ml_temporary_result22 => source_ml_temporary_result22 end)))))).

Inductive source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_result : Type :=
| source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_done : source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_result
| source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_return : bool -> source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_result.

Definition source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23 (names : (list (list float))) (index : float) : source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_result :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * float -> source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_result) (ml_args : (list (list float)) * float) =>
    let '(names, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length names))))) then (let name := (readJavaScriptRecordOrderName names index) in (if (negb (acceptRecordFieldName name)) then (source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_return false) else (if (andb (PrimFloat.ltb 0%float index) (PrimFloat.leb 0%float (compareRecordFieldNames (readJavaScriptRecordOrderName names (PrimFloat.sub index 1%float)) name))) then (source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_return false) else (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (names, index_2)))))) else source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_done))
    (fun _ => source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_done) (names, index).

Definition acceptJavaScriptRecordOrderNames (names : (list (list float))) : bool :=
  (let index := 0%float in (let source_ml_source1_s13 := (source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23 names index) in (match source_ml_source1_s13 with | source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_done => true | source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_return source_ml_temporary_result23 => source_ml_temporary_result23 end))).

Inductive source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_result : Type :=
| source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_done : source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_result
| source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_return : bool -> source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_result.

Inductive source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_result : Type :=
| source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_done : source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_result
| source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_return : bool -> source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_result.

Definition source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25 (order : (list float)) (index : float) (value : float) (previous : float) : source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_result :=
  ml_fix 64 (fun (ml_rec : (list float) * float * float * float -> source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_result) (ml_args : (list float) * float * float * float) =>
    let '(order, index, value, previous) := ml_args in (if (PrimFloat.ltb previous index) then (if (PrimFloat.eqb (readJavaScriptRecordOrderPosition order previous) value) then (source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_return false) else (let previous_2 := (PrimFloat.add previous 1%float) in (ml_rec (order, index, value, previous_2)))) else source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_done))
    (fun _ => source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_done) (order, index, value, previous).

Definition source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24 (count : float) (order : (list float)) (index : float) : source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_result :=
  ml_fix 64 (fun (ml_rec : float * (list float) * float -> source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_result) (ml_args : float * (list float) * float) =>
    let '(count, order, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length order))))) then (let value := (readJavaScriptRecordOrderPosition order index) in (if (orb (orb (orb (negb (ml_is_integer value)) (PrimFloat.ltb value 0%float)) (PrimFloat.leb count value)) (andb (PrimFloat.eqb value 0%float) (PrimFloat.ltb (PrimFloat.div 1%float value) 0%float))) then (source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_return false) else (let previous := 0%float in (let source_ml_source1_s23 := (source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25 order index value previous) in (match source_ml_source1_s23 with | source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_done => (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (count, order, index_2))) | source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_return source_ml_temporary_result25 => (source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_return source_ml_temporary_result25) end))))) else source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_done))
    (fun _ => source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_done) (count, order, index).

Definition acceptJavaScriptRecordOrderPermutation (count : float) (order : (list float)) : bool :=
  (if (orb (orb (orb (negb (ml_is_safe_integer count)) (PrimFloat.ltb count 0%float)) (andb (PrimFloat.eqb count 0%float) (PrimFloat.ltb (PrimFloat.div 1%float count) 0%float))) (negb (PrimFloat.eqb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length order)))) count))) then false else (let index := 0%float in (let source_ml_source1_s14 := (source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24 count order index) in (match source_ml_source1_s14 with | source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_done => true | source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_return source_ml_temporary_result24 => source_ml_temporary_result24 end)))).

Definition readJavaScriptRecordOrderCreationError (names : (list (list float))) (insertion : (list float)) : (list N) :=
  (if (orb (negb (acceptJavaScriptRecordOrderNames names)) (negb (acceptJavaScriptRecordOrderPermutation (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length names)))) insertion))) then (105%N :: 110%N :: 118%N :: 97%N :: 108%N :: 105%N :: 100%N :: 32%N :: 111%N :: 114%N :: 100%N :: 101%N :: 114%N :: 101%N :: 100%N :: 32%N :: 114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: 32%N :: 102%N :: 105%N :: 101%N :: 108%N :: 100%N :: 32%N :: 115%N :: 101%N :: 113%N :: 117%N :: 101%N :: 110%N :: 99%N :: 101%N :: (@nil N)) else ((@nil N))).

Definition source_ml_temporary_createJavaScriptRecordPropertyOrder_loop27 (names : (list (list float))) (numeric : (list float)) (key : float) (position : float) : float :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * (list float) * float * float -> float) (ml_args : (list (list float)) * (list float) * float * float) =>
    let '(names, numeric, key, position) := ml_args in (if (andb (PrimFloat.ltb position (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length numeric))))) (PrimFloat.ltb (readJavaScriptLiteralPropertyIndex (readJavaScriptRecordOrderName names (readJavaScriptRecordOrderPosition numeric position))) key)) then (let position_2 := (PrimFloat.add position 1%float) in (ml_rec (names, numeric, key, position_2))) else position))
    (fun _ => 0%float) (names, numeric, key, position).

Definition source_ml_temporary_createJavaScriptRecordPropertyOrder_loop26 (names : (list (list float))) (numeric : (list float)) (field : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * (list float) * float -> (list float)) (ml_args : (list (list float)) * (list float) * float) =>
    let '(names, numeric, field) := ml_args in (if (PrimFloat.ltb field (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length names))))) then (let key := (readJavaScriptLiteralPropertyIndex (readJavaScriptRecordOrderName names field)) in (if (PrimFloat.ltb key 0%float) then (let field_2 := (PrimFloat.add field 1%float) in (ml_rec (names, numeric, field_2))) else (let position := 0%float in (let position_2 := (source_ml_temporary_createJavaScriptRecordPropertyOrder_loop27 names numeric key position) in (let numeric_2 := (let source_ml_source1_concat_value24 := (let ml_values := numeric in let ml_begin := 0%float in let ml_end := position_2 in let ml_length := PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length ml_values))) in let ml_first := Z.to_nat (ml_float_index (MLUtf16Text.slicePosition ml_begin ml_length)) in let ml_last := Z.to_nat (ml_float_index (MLUtf16Text.slicePosition ml_end ml_length)) in List.firstn (Nat.sub ml_last ml_first) (List.skipn ml_first ml_values)) in (let source_ml_source1_concat_value25 := (field :: @nil float) in (let source_ml_source1_concat_value26 := (let ml_values := numeric in let ml_begin := position_2 in let ml_end := PrimFloat.infinity in let ml_length := PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length ml_values))) in let ml_first := Z.to_nat (ml_float_index (MLUtf16Text.slicePosition ml_begin ml_length)) in let ml_last := Z.to_nat (ml_float_index (MLUtf16Text.slicePosition ml_end ml_length)) in List.firstn (Nat.sub ml_last ml_first) (List.skipn ml_first ml_values)) in (List.app (List.app (List.app (@nil float) source_ml_source1_concat_value24) source_ml_source1_concat_value25) source_ml_source1_concat_value26)))) in (let field_3 := (PrimFloat.add field 1%float) in (ml_rec (names, numeric_2, field_3)))))))) else numeric))
    (fun _ => (@nil float)) (names, numeric, field).

Definition source_ml_temporary_createJavaScriptRecordPropertyOrder_loop28 (names : (list (list float))) (insertion : (list float)) (result : (list float)) (index : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * (list float) * (list float) * float -> (list float)) (ml_args : (list (list float)) * (list float) * (list float) * float) =>
    let '(names, insertion, result, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length insertion))))) then (let field := (readJavaScriptRecordOrderPosition insertion index) in (if (PrimFloat.ltb (readJavaScriptLiteralPropertyIndex (readJavaScriptRecordOrderName names field)) 0%float) then (let result_2 := (let source_ml_source1_concat_value27 := result in (let source_ml_source1_concat_value28 := (field :: @nil float) in (List.app (List.app (@nil float) source_ml_source1_concat_value27) source_ml_source1_concat_value28))) in (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (names, insertion, result_2, index_2)))) else (let index_3 := (PrimFloat.add index 1%float) in (ml_rec (names, insertion, result, index_3))))) else result))
    (fun _ => (@nil float)) (names, insertion, result, index).

Definition createJavaScriptRecordPropertyOrder (names : (list (list float))) (insertion : (list float)) : (list float) :=
  (let error := (readJavaScriptRecordOrderCreationError names insertion) in (if (match error with nil => false | _ => true end) then (ml_raiseJavaScriptTypeError error) else (let numeric := (@nil float) in (let field := 0%float in (let numeric_2 := (source_ml_temporary_createJavaScriptRecordPropertyOrder_loop26 names numeric field) in (let result := numeric_2 in (let index := 0%float in (let result_2 := (source_ml_temporary_createJavaScriptRecordPropertyOrder_loop28 names insertion result index) in result_2)))))))).

Inductive source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_result : Type :=
| source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_done : source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_result
| source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_return : bool -> source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_result.

Definition source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29 (order : (list float)) (expected : (list float)) (index : float) : source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_result :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float -> source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_result) (ml_args : (list float) * (list float) * float) =>
    let '(order, expected, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length order))))) then (if (negb (PrimFloat.eqb (readJavaScriptRecordOrderPosition order index) (readJavaScriptRecordOrderPosition expected index))) then (source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_return false) else (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (order, expected, index_2)))) else source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_done))
    (fun _ => source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_done) (order, expected, index).

Definition acceptJavaScriptRecordPropertyOrder (names : (list (list float))) (order : (list float)) : bool :=
  (if (orb (negb (acceptJavaScriptRecordOrderNames names)) (negb (acceptJavaScriptRecordOrderPermutation (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length names)))) order))) then false else (let expected := (createJavaScriptRecordPropertyOrder names order) in (let index := 0%float in (let source_ml_source1_s15 := (source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29 order expected index) in (match source_ml_source1_s15 with | source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_done => true | source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_return source_ml_temporary_result29 => source_ml_temporary_result29 end))))).

Definition readJavaScriptRecordOrderKeyError (names : (list (list float))) (order : (list float)) : (list N) :=
  (if (acceptJavaScriptRecordPropertyOrder names order) then ((@nil N)) else (105%N :: 110%N :: 118%N :: 97%N :: 108%N :: 105%N :: 100%N :: 32%N :: 111%N :: 114%N :: 100%N :: 101%N :: 114%N :: 101%N :: 100%N :: 32%N :: 114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: 32%N :: 112%N :: 114%N :: 111%N :: 112%N :: 101%N :: 114%N :: 116%N :: 121%N :: 32%N :: 111%N :: 114%N :: 100%N :: 101%N :: 114%N :: (@nil N))).

Inductive source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_result : Type :=
| source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_done : (list float) -> source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_result
| source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_return : (list N) -> source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_result.

Inductive source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_result : Type :=
| source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_done : (list float) -> source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_result
| source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_return : (list N) -> source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_result.

Definition source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31 (names : (list (list float))) (insertion : (list float)) (values : (list float)) (index : float) : source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_result :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * (list float) * (list float) * float -> source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_result) (ml_args : (list (list float)) * (list float) * (list float) * float) =>
    let '(names, insertion, values, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length values))))) then (let field := (readJavaScriptRecordOrderPosition values index) in (if (orb (orb (orb (negb (ml_is_integer field)) (PrimFloat.ltb field 0%float)) (PrimFloat.leb (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length names)))) field)) (andb (PrimFloat.eqb field 0%float) (PrimFloat.ltb (PrimFloat.div 1%float field) 0%float))) then (source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_return (105%N :: 110%N :: 118%N :: 97%N :: 108%N :: 105%N :: 100%N :: 32%N :: 111%N :: 114%N :: 100%N :: 101%N :: 114%N :: 101%N :: 100%N :: 32%N :: 114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: 32%N :: 115%N :: 111%N :: 117%N :: 114%N :: 99%N :: 101%N :: 32%N :: 102%N :: 105%N :: 101%N :: 108%N :: 100%N :: (@nil N))) else (if (negb (let ml_values := insertion in let ml_search := field in let ml_start := 0%float in let ml_length := PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length ml_values))) in let ml_first := Z.to_nat (ml_float_index (MLUtf16Text.slicePosition ml_start ml_length)) in List.existsb (fun ml_candidate => (orb (PrimFloat.eqb ml_candidate ml_search) (andb (PrimFloat.is_nan ml_candidate) (PrimFloat.is_nan ml_search)))) (List.skipn ml_first ml_values))) then (let insertion_2 := (let source_ml_source1_concat_value29 := insertion in (let source_ml_source1_concat_value30 := (field :: @nil float) in (List.app (List.app (@nil float) source_ml_source1_concat_value29) source_ml_source1_concat_value30))) in (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (names, insertion_2, values, index_2)))) else (let index_3 := (PrimFloat.add index 1%float) in (ml_rec (names, insertion, values, index_3)))))) else (source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_done insertion)))
    (fun _ => (source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_done (@nil float))) (names, insertion, values, index).

Definition source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30 (names : (list (list float))) (chunks : (list (list float))) (insertion : (list float)) (chunk : float) : source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_result :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * (list (list float)) * (list float) * float -> source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_result) (ml_args : (list (list float)) * (list (list float)) * (list float) * float) =>
    let '(names, chunks, insertion, chunk) := ml_args in (if (PrimFloat.ltb chunk (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length chunks))))) then (let values := (readJavaScriptRecordOrderName chunks chunk) in (let index := 0%float in (let source_ml_source1_s31 := (source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31 names insertion values index) in (match source_ml_source1_s31 with | source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_done insertion_2 => (let chunk_2 := (PrimFloat.add chunk 1%float) in (ml_rec (names, chunks, insertion_2, chunk_2))) | source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_return source_ml_temporary_result31 => (source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_return source_ml_temporary_result31) end)))) else (source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_done insertion)))
    (fun _ => (source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_done (@nil float))) (names, chunks, insertion, chunk).

Definition readJavaScriptRecordOrderMergeError (names : (list (list float))) (chunks : (list (list float))) : (list N) :=
  (if (negb (acceptJavaScriptRecordOrderNames names)) then (105%N :: 110%N :: 118%N :: 97%N :: 108%N :: 105%N :: 100%N :: 32%N :: 111%N :: 114%N :: 100%N :: 101%N :: 114%N :: 101%N :: 100%N :: 32%N :: 114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: 32%N :: 102%N :: 105%N :: 101%N :: 108%N :: 100%N :: 32%N :: 110%N :: 97%N :: 109%N :: 101%N :: 115%N :: (@nil N)) else (let insertion := (@nil float) in (let chunk := 0%float in (let source_ml_source1_s16 := (source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30 names chunks insertion chunk) in (match source_ml_source1_s16 with | source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_done insertion_2 => (readJavaScriptRecordOrderCreationError names insertion_2) | source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_return source_ml_temporary_result30 => source_ml_temporary_result30 end))))).

Definition source_ml_temporary_mergeJavaScriptRecordPropertyOrder_loop33 (insertion : (list float)) (values : (list float)) (index : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list float) * (list float) * float -> (list float)) (ml_args : (list float) * (list float) * float) =>
    let '(insertion, values, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length values))))) then (let field := (readJavaScriptRecordOrderPosition values index) in (if (negb (let ml_values := insertion in let ml_search := field in let ml_start := 0%float in let ml_length := PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length ml_values))) in let ml_first := Z.to_nat (ml_float_index (MLUtf16Text.slicePosition ml_start ml_length)) in List.existsb (fun ml_candidate => (orb (PrimFloat.eqb ml_candidate ml_search) (andb (PrimFloat.is_nan ml_candidate) (PrimFloat.is_nan ml_search)))) (List.skipn ml_first ml_values))) then (let insertion_2 := (let source_ml_source1_concat_value32 := insertion in (let source_ml_source1_concat_value33 := (field :: @nil float) in (List.app (List.app (@nil float) source_ml_source1_concat_value32) source_ml_source1_concat_value33))) in (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (insertion_2, values, index_2)))) else (let index_3 := (PrimFloat.add index 1%float) in (ml_rec (insertion, values, index_3))))) else insertion))
    (fun _ => (@nil float)) (insertion, values, index).

Definition source_ml_temporary_mergeJavaScriptRecordPropertyOrder_loop32 (chunks : (list (list float))) (insertion : (list float)) (chunk : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * (list float) * float -> (list float)) (ml_args : (list (list float)) * (list float) * float) =>
    let '(chunks, insertion, chunk) := ml_args in (if (PrimFloat.ltb chunk (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length chunks))))) then (let values := (readJavaScriptRecordOrderName chunks chunk) in (let index := 0%float in (let insertion_2 := (source_ml_temporary_mergeJavaScriptRecordPropertyOrder_loop33 insertion values index) in (let chunk_2 := (PrimFloat.add chunk 1%float) in (ml_rec (chunks, insertion_2, chunk_2)))))) else insertion))
    (fun _ => (@nil float)) (chunks, insertion, chunk).

Definition mergeJavaScriptRecordPropertyOrder (names : (list (list float))) (chunks : (list (list float))) : (list float) :=
  (let error := (readJavaScriptRecordOrderMergeError names chunks) in (if (match error with nil => false | _ => true end) then (ml_raiseJavaScriptTypeError error) else (let insertion := (@nil float) in (let chunk := 0%float in (let insertion_2 := (source_ml_temporary_mergeJavaScriptRecordPropertyOrder_loop32 chunks insertion chunk) in (createJavaScriptRecordPropertyOrder names insertion_2)))))).

Inductive source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_result : Type :=
| source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_done : source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_result
| source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_return : (list N) -> source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_result.

Definition source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34 (target : (list (list float))) (source : (list (list float))) (index : float) : source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_result :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * (list (list float)) * float -> source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_result) (ml_args : (list (list float)) * (list (list float)) * float) =>
    let '(target, source, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length source))))) then (if (PrimFloat.ltb (readRecordFieldPosition target (readJavaScriptRecordOrderName source index)) 0%float) then (source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_return (97%N :: 110%N :: 32%N :: 111%N :: 114%N :: 100%N :: 101%N :: 114%N :: 101%N :: 100%N :: 32%N :: 114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: 32%N :: 115%N :: 112%N :: 114%N :: 101%N :: 97%N :: 100%N :: 32%N :: 102%N :: 105%N :: 101%N :: 108%N :: 100%N :: 32%N :: 105%N :: 115%N :: 32%N :: 109%N :: 105%N :: 115%N :: 115%N :: 105%N :: 110%N :: 103%N :: 32%N :: 102%N :: 114%N :: 111%N :: 109%N :: 32%N :: 105%N :: 116%N :: 115%N :: 32%N :: 116%N :: 97%N :: 114%N :: 103%N :: 101%N :: 116%N :: (@nil N))) else (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (target, source, index_2)))) else source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_done))
    (fun _ => source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_done) (target, source, index).

Definition readJavaScriptRecordOrderRebaseError (target : (list (list float))) (source : (list (list float))) (order : (list float)) : (list N) :=
  (if (orb (negb (acceptJavaScriptRecordOrderNames target)) (negb (acceptJavaScriptRecordPropertyOrder source order))) then (105%N :: 110%N :: 118%N :: 97%N :: 108%N :: 105%N :: 100%N :: 32%N :: 111%N :: 114%N :: 100%N :: 101%N :: 114%N :: 101%N :: 100%N :: 32%N :: 114%N :: 101%N :: 99%N :: 111%N :: 114%N :: 100%N :: 32%N :: 115%N :: 112%N :: 114%N :: 101%N :: 97%N :: 100%N :: 32%N :: 115%N :: 111%N :: 117%N :: 114%N :: 99%N :: 101%N :: (@nil N)) else (let index := 0%float in (let source_ml_source1_s17 := (source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34 target source index) in (match source_ml_source1_s17 with | source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_done => ((@nil N)) | source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_return source_ml_temporary_result34 => source_ml_temporary_result34 end)))).

Definition source_ml_temporary_rebaseJavaScriptRecordPropertyOrder_loop35 (target : (list (list float))) (source : (list (list float))) (order : (list float)) (result : (list float)) (index : float) : (list float) :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * (list (list float)) * (list float) * (list float) * float -> (list float)) (ml_args : (list (list float)) * (list (list float)) * (list float) * (list float) * float) =>
    let '(target, source, order, result, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length order))))) then (let name := (readJavaScriptRecordOrderName source (readJavaScriptRecordOrderPosition order index)) in (let field := (readRecordFieldPosition target name) in (let result_2 := (let source_ml_source1_concat_value34 := result in (let source_ml_source1_concat_value35 := (field :: @nil float) in (List.app (List.app (@nil float) source_ml_source1_concat_value34) source_ml_source1_concat_value35))) in (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (target, source, order, result_2, index_2)))))) else result))
    (fun _ => (@nil float)) (target, source, order, result, index).

Definition rebaseJavaScriptRecordPropertyOrder (target : (list (list float))) (source : (list (list float))) (order : (list float)) : (list float) :=
  (let error := (readJavaScriptRecordOrderRebaseError target source order) in (if (match error with nil => false | _ => true end) then (ml_raiseJavaScriptTypeError error) else (let result := (@nil float) in (let index := 0%float in (let result_2 := (source_ml_temporary_rebaseJavaScriptRecordPropertyOrder_loop35 target source order result index) in result_2))))).

Definition source_ml_temporary_readJavaScriptRecordPropertyKeys_loop36 (names : (list (list float))) (order : (list float)) (result : (list (list float))) (index : float) : (list (list float)) :=
  ml_fix 64 (fun (ml_rec : (list (list float)) * (list float) * (list (list float)) * float -> (list (list float))) (ml_args : (list (list float)) * (list float) * (list (list float)) * float) =>
    let '(names, order, result, index) := ml_args in (if (PrimFloat.ltb index (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length order))))) then (let result_2 := (let source_ml_source1_concat_value36 := result in (let source_ml_source1_concat_value37 := ((readJavaScriptRecordOrderName names (readJavaScriptRecordOrderPosition order index)) :: @nil (list float)) in (List.app (List.app (@nil (list float)) source_ml_source1_concat_value36) source_ml_source1_concat_value37))) in (let index_2 := (PrimFloat.add index 1%float) in (ml_rec (names, order, result_2, index_2)))) else result))
    (fun _ => (@nil (list float))) (names, order, result, index).

Definition readJavaScriptRecordPropertyKeys (names : (list (list float))) (order : (list float)) : (list (list float)) :=
  (let error := (readJavaScriptRecordOrderKeyError names order) in (if (match error with nil => false | _ => true end) then (ml_raiseJavaScriptTypeError error) else (let result := (@nil (list float)) in (let index := 0%float in (let result_2 := (source_ml_temporary_readJavaScriptRecordPropertyKeys_loop36 names order result index) in result_2))))).

Definition main : list string :=
  nil.

Eval vm_compute in main.

Definition create := createJavaScriptRecordPropertyOrder.
Definition merge := mergeJavaScriptRecordPropertyOrder.
Definition rebase := rebaseJavaScriptRecordPropertyOrder.
Definition keys := readJavaScriptRecordPropertyKeys.
Definition valid := acceptJavaScriptRecordPropertyOrder.
End MLRecordPropertyOrder.
