namespace MLRecordPropertyOrder
-- Translated from JavaScript by meta-language: portable core, Lean target.
-- Source binders are kept even where unused or named like a constructor, and proof hints are shared by every closing tactic.
set_option linter.unusedVariables false
set_option linter.unusedSimpArgs false
set_option linter.constructorNameAsVariable false

/-- ECMAScript Number::toString: the shortest decimal that reads back as the
value, the nearer one and then the even one on a tie, laid out as JavaScript
prints it; exact Nat arithmetic on the bits. -/
def ml_js_layout (digits : List Char) (n : Int) : String :=
  let k : Int := digits.length
  let text := String.ofList
  let zeros (count : Int) := List.replicate count.toNat '0'
  if k ≤ n ∧ n ≤ 21 then text (digits ++ zeros (n - k))
  else if 0 < n ∧ n ≤ 21 then text (digits.take n.toNat ++ '.' :: digits.drop n.toNat)
  else if -6 < n ∧ n ≤ 0 then text ('0' :: '.' :: zeros (-n) ++ digits)
  else
    let e := n - 1
    let exp := (if e < 0 then "e-" else "e+") ++ toString e.natAbs
    if k == 1 then text digits ++ exp
    else text (digits.take 1 ++ '.' :: digits.drop 1) ++ exp

/-- `c · 10^q` compared with `x · s / d`. -/
def ml_js_cmp (c : Nat) (q : Int) (x s d : Nat) : Ordering :=
  if 0 ≤ q then compare (c * 10 ^ q.toNat * d) (x * s)
  else compare (c * d) (x * s * 10 ^ (-q).toNat)

def ml_js_shortest (m : Nat) (e : Int) (lowerCloser : Bool) : String :=
  -- The value is 4m and the rounding interval [lo, hi], in units of 2^(e-2) = s / d.
  let s := 2 ^ (e - 2).toNat
  let d := 2 ^ (2 - e).toNat
  let v := 4 * m
  let lo := if lowerCloser then v - 1 else v - 2
  let hi := v + 2
  let inclusive := m % 2 == 0
  let above (c : Nat) (q : Int) : Bool :=
    match ml_js_cmp c q lo s d with
    | .gt => true | .eq => inclusive | .lt => false
  let below (c : Nat) (q : Int) : Bool :=
    match ml_js_cmp c q hi s d with
    | .lt => true | .eq => inclusive | .gt => false
  -- floor (log10 value) from the bit length, then corrected exactly
  let bits : Int := (Nat.log2 m : Int) + 1 + e
  let guess : Int := Int.fdiv ((bits - 1) * 78913) 262144
  let e10 :=
    if ml_js_cmp 1 (guess + 1) v s d != .gt then guess + 1
    else if ml_js_cmp 1 guess v s d == .gt then guess - 1
    else guess
  let rec search (fuel : Nat) (p : Nat) : String :=
    match fuel with
    | 0 => ""
    | fuel + 1 =>
      let q : Int := e10 - p + 1
      let c :=
        if 0 ≤ q then v * s / (d * 10 ^ q.toNat)
        else v * s * 10 ^ (-q).toNat / d
      let lowOk := above c q ∧ below c q
      let highOk := above (c + 1) q ∧ below (c + 1) q
      if lowOk ∨ highOk then
        let pick :=
          if lowOk ∧ highOk then
            match ml_js_cmp (2 * c + 1) q (2 * v) s d with
            | .gt => c | .lt => c + 1 | .eq => if c % 2 == 0 then c else c + 1
          else if lowOk then c else c + 1
        let text := (toString pick).toList
        let n : Int := q + text.length
        ml_js_layout (text.reverse.dropWhile (· == '0')).reverse n
      else search fuel (p + 1)
  search 17 1

def ml_js_number (x : Float) : String :=
  let bits : Nat := x.toBits.toNat
  let negative := bits ≥ 2 ^ 63
  let exponent : Nat := (bits / 2 ^ 52) % 2048
  let fraction : Nat := bits % 2 ^ 52
  if exponent == 2047 then
    if fraction != 0 then "NaN" else if negative then "-Infinity" else "Infinity"
  else if exponent == 0 ∧ fraction == 0 then "0"
  else
    let body :=
      if exponent == 0 then ml_js_shortest fraction (-1074) false
      else ml_js_shortest (fraction + 2 ^ 52) ((exponent : Int) - 1075) (fraction == 0 ∧ exponent > 1)
    if negative then "-" ++ body else body

/-- ECMAScript `%` on binary64 (C fmod): the exact remainder, truncated, with
the sign of the dividend; computed on the bits, so it is exact. -/
def ml_float_rem (x y : Float) : Float :=
  let decompose (f : Float) : Nat × Int :=
    let b : Nat := f.toBits.toNat
    let exponent : Nat := (b / 2 ^ 52) % 2048
    let fraction : Nat := b % 2 ^ 52
    if exponent == 0 then (fraction, -1074) else (fraction + 2 ^ 52, (exponent : Int) - 1075)
  if x.isNaN || y.isNaN || x.isInf || y == 0 then 0 / 0
  else if y.isInf || x == 0 then x
  else
    let (mx, ex) := decompose x
    let (my, ey) := decompose y
    let e := min ex ey
    let r := (mx * 2 ^ (ex - e).toNat) % (my * 2 ^ (ey - e).toNat)
    let magnitude := Float.scaleB (Float.ofNat r) e
    if x.toBits.toNat ≥ 2 ^ 63 then -magnitude else magnitude

/-- Math.trunc: towards zero, a zero result keeping the sign of the argument. -/
def ml_trunc (x : Float) : Float := if x < 0 then x.ceil else x.floor
/-- Math.round: the nearest integer, the one towards +Infinity on a tie, and -0 from -0.5 up to 0. -/
def ml_round (x : Float) : Float :=
  if x.isNaN || x.isInf || x == 0 then x
  else if x > 0 ∧ x < 0.5 then 0
  else if x < 0 ∧ x ≥ -0.5 then -0.0
  else if x - x.floor ≥ 0.5 then x.floor + 1 else x.floor
/-- Math.sign: 1 or -1, and -0, 0 and NaN as they are. -/
def ml_sign (x : Float) : Float := if x > 0 then 1 else if x < 0 then -1 else x
/-- Math.max of two Numbers: NaN when either is, and 0 above -0. -/
def ml_max (a b : Float) : Float :=
  if a > b then a else if b > a then b else if a == b then (if 1 / a < 0 then b else a) else 0.0 / 0.0
/-- Math.min of two Numbers: NaN when either is, and -0 below 0. -/
def ml_min (a b : Float) : Float :=
  if a < b then a else if b < a then b else if a == b then (if 1 / a < 0 then a else b) else 0.0 / 0.0
def ml_is_integer (x : Float) : Bool := x.isFinite && ml_trunc x == x
def ml_is_safe_integer (x : Float) : Bool := ml_is_integer x && decide (x.abs ≤ 9007199254740991)

namespace MLUtf16Text
-- Translated from JavaScript by meta-language: portable core, Lean target.
-- Source binders are kept even where unused or named like a constructor, and proof hints are shared by every closing tactic.
set_option linter.unusedVariables false
set_option linter.unusedSimpArgs false
set_option linter.constructorNameAsVariable false

/-- ECMAScript `%` on binary64 (C fmod): the exact remainder, truncated, with
the sign of the dividend; computed on the bits, so it is exact. -/
def ml_float_rem (x y : Float) : Float :=
  let decompose (f : Float) : Nat × Int :=
    let b : Nat := f.toBits.toNat
    let exponent : Nat := (b / 2 ^ 52) % 2048
    let fraction : Nat := b % 2 ^ 52
    if exponent == 0 then (fraction, -1074) else (fraction + 2 ^ 52, (exponent : Int) - 1075)
  if x.isNaN || y.isNaN || x.isInf || y == 0 then 0 / 0
  else if y.isInf || x == 0 then x
  else
    let (mx, ex) := decompose x
    let (my, ey) := decompose y
    let e := min ex ey
    let r := (mx * 2 ^ (ex - e).toNat) % (my * 2 ^ (ey - e).toNat)
    let magnitude := Float.scaleB (Float.ofNat r) e
    if x.toBits.toNat ≥ 2 ^ 63 then -magnitude else magnitude

/-- Math.trunc: towards zero, a zero result keeping the sign of the argument. -/
def ml_trunc (x : Float) : Float := if x < 0 then x.ceil else x.floor
/-- Math.round: the nearest integer, the one towards +Infinity on a tie, and -0 from -0.5 up to 0. -/
def ml_round (x : Float) : Float :=
  if x.isNaN || x.isInf || x == 0 then x
  else if x > 0 ∧ x < 0.5 then 0
  else if x < 0 ∧ x ≥ -0.5 then -0.0
  else if x - x.floor ≥ 0.5 then x.floor + 1 else x.floor
/-- Math.sign: 1 or -1, and -0, 0 and NaN as they are. -/
def ml_sign (x : Float) : Float := if x > 0 then 1 else if x < 0 then -1 else x
/-- Math.max of two Numbers: NaN when either is, and 0 above -0. -/
def ml_max (a b : Float) : Float :=
  if a > b then a else if b > a then b else if a == b then (if 1 / a < 0 then b else a) else 0.0 / 0.0
/-- Math.min of two Numbers: NaN when either is, and -0 below 0. -/
def ml_min (a b : Float) : Float :=
  if a < b then a else if b < a then b else if a == b then (if 1 / a < 0 then a else b) else 0.0 / 0.0
def ml_is_integer (x : Float) : Bool := x.isFinite && ml_trunc x == x
def ml_is_safe_integer (x : Float) : Bool := ml_is_integer x && decide (x.abs ≤ 9007199254740991)

def acceptJavaScriptWhitespaceUnit (unit : Float) : Bool :=
  ((((((((((((decide (unit ≥ (9 : Float))) && (decide (unit ≤ (13 : Float)))) || (unit == (32 : Float))) || (unit == (160 : Float))) || (unit == (5760 : Float))) || ((decide (unit ≥ (8192 : Float))) && (decide (unit ≤ (8202 : Float))))) || (unit == (8232 : Float))) || (unit == (8233 : Float))) || (unit == (8239 : Float))) || (unit == (8287 : Float))) || (unit == (12288 : Float))) || (unit == (65279 : Float)))

def normalizeJavaScriptUnsignedInteger (value : Float) : Float :=
  (if ((!(Float.isFinite value)) || (value == (0 : Float))) then (0 : Float) else (let integral := (if (decide (value < (0 : Float))) then (-(Float.floor (-value))) else (Float.floor value));
    (let remainder := (ml_float_rem integral (4294967296 : Float));
      (if (decide (remainder < (0 : Float))) then (remainder + (4294967296 : Float)) else (if (remainder == (0 : Float)) then (0 : Float) else remainder)))))

def readUtf16TextInteger (value : Float) : Float :=
  (if (Float.isNaN value) then (0 : Float) else (ml_trunc value))

def readUtf16TextSlicePosition (value : Float) (length : Float) : Float :=
  (let integer := (readUtf16TextInteger value);
    (if (decide (integer < (0 : Float))) then (ml_max (length + integer) (0 : Float)) else (ml_min integer length)))

def characterUtf16Text (text : (Array Float)) (position : Float) : (Array Float) :=
  (let «at» := (readUtf16TextInteger position);
    (if ((decide («at» < (0 : Float))) || (decide («at» ≥ (Float.ofNat text.size)))) then (#[] : (Array Float)) else (#[(match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float))] : (Array Float))))

def relativeCharacterUtf16Text (text : (Array Float)) (position : Float) : (Array (Array Float)) :=
  (let integer := (readUtf16TextInteger position);
    (let «at» := (if (decide (integer < (0 : Float))) then ((Float.ofNat text.size) + integer) else integer);
      (if ((decide («at» < (0 : Float))) || (decide («at» ≥ (Float.ofNat text.size)))) then (#[] : (Array (Array Float))) else (#[(#[(match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float))] : (Array Float))] : (Array (Array Float))))))

def codePointUtf16Text (text : (Array Float)) (position : Float) : (Array Float) :=
  (let «at» := (readUtf16TextInteger position);
    (if ((decide («at» < (0 : Float))) || (decide («at» ≥ (Float.ofNat text.size)))) then (#[] : (Array Float)) else (let first := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float));
      (let second := (match (let ml_index := («at» + (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
        (#[(if ((((decide (first ≥ (55296 : Float))) && (decide (first ≤ (56319 : Float)))) && (decide (second ≥ (56320 : Float)))) && (decide (second ≤ (57343 : Float)))) then ((((65536 : Float) + ((first - (55296 : Float)) * (1024 : Float))) + second) - (56320 : Float)) else first)] : (Array Float))))))

inductive source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_result where
  | source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_done (units : (Array Float)) : source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_result
  | source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_return (ml_temporary_result2 : (Array Float)) : source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_result
  deriving Repr, Inhabited

inductive source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_result where
  | source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_done (index : Float) (scalar : Float) : source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_result
  | source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_return (ml_temporary_result4 : (Array Float)) : source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_result
  deriving Repr, Inhabited

partial def source_ml_temporary_readJavaScriptUtf8StringUnits_loop4 (bytes : (Array Float)) (index : Float) (scalar : Float) (remaining : Float) : source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_result :=
  (if (decide (remaining > (0 : Float))) then (let next := (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then bytes[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if ((decide (next < (128 : Float))) || (decide (next > (191 : Float)))) then (source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_result.source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_return (#[(-1 : Float)] : (Array Float))) else (let scalar_2 := (((scalar * (64 : Float)) + next) - (128 : Float));
      (let index_2 := (index + (1 : Float));
        (let remaining_2 := (remaining - (1 : Float));
          (source_ml_temporary_readJavaScriptUtf8StringUnits_loop4 bytes index_2 scalar_2 remaining_2)))))) else (source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_result.source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_done index scalar))

inductive source_ml_temporary_readJavaScriptUtf8StringUnits_join3 where
  | source_ml_temporary_readJavaScriptUtf8StringUnits_join3_next (scalar : Float) (remaining : Float) (minimum : Float) : source_ml_temporary_readJavaScriptUtf8StringUnits_join3
  | source_ml_temporary_readJavaScriptUtf8StringUnits_join3_return (ml_temporary_result4 : (Array Float)) : source_ml_temporary_readJavaScriptUtf8StringUnits_join3
  deriving Repr, Inhabited

partial def source_ml_temporary_readJavaScriptUtf8StringUnits_loop2 (bytes : (Array Float)) (units : (Array Float)) (index : Float) : source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_result :=
  (if (decide (index < (Float.ofNat bytes.size))) then (let scalar := (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then bytes[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (let index_2 := (index + (1 : Float));
      (let remaining := (0 : Float);
        (let minimum := (0 : Float);
          (let source_ml_source1_s11 := (if ((decide (scalar ≥ (194 : Float))) && (decide (scalar ≤ (223 : Float)))) then (let scalar_2 := (scalar - (192 : Float));
            (let remaining_2 := (1 : Float);
              (let minimum_2 := (128 : Float);
                (source_ml_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_2 remaining_2 minimum_2)))) else (if ((decide (scalar ≥ (224 : Float))) && (decide (scalar ≤ (239 : Float)))) then (let scalar_3 := (scalar - (224 : Float));
            (let remaining_3 := (2 : Float);
              (let minimum_3 := (2048 : Float);
                (source_ml_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_3 remaining_3 minimum_3)))) else (if ((decide (scalar ≥ (240 : Float))) && (decide (scalar ≤ (244 : Float)))) then (let scalar_4 := (scalar - (240 : Float));
            (let remaining_4 := (3 : Float);
              (let minimum_4 := (65536 : Float);
                (source_ml_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_4 remaining_4 minimum_4)))) else (if (decide (scalar ≥ (128 : Float))) then (source_ml_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_temporary_readJavaScriptUtf8StringUnits_join3_return (#[(-1 : Float)] : (Array Float))) else (source_ml_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_temporary_readJavaScriptUtf8StringUnits_join3_next scalar remaining minimum)))));
            (match source_ml_source1_s11 with
              | source_ml_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_5 remaining_5 minimum_5 => (let source_ml_source1_s12 := (source_ml_temporary_readJavaScriptUtf8StringUnits_loop4 bytes index_2 scalar_5 remaining_5);
                  (match source_ml_source1_s12 with
                    | source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_result.source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_done index_3 scalar_6 => (if (((decide (scalar_6 < minimum_5)) || (decide (scalar_6 > (1114111 : Float)))) || ((decide (scalar_6 ≥ (55296 : Float))) && (decide (scalar_6 ≤ (57343 : Float))))) then (source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_result.source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_return (#[(-1 : Float)] : (Array Float))) else (if (decide (scalar_6 ≤ (65535 : Float))) then (let units_2 := (units ++ (#[scalar_6] : (Array Float)));
                        (source_ml_temporary_readJavaScriptUtf8StringUnits_loop2 bytes units_2 index_3)) else (let units_3 := (units ++ (#[((55296 : Float) + (Float.floor ((scalar_6 - (65536 : Float)) / (1024 : Float))))] : (Array Float)));
                        (let units_4 := (units_3 ++ (#[((56320 : Float) + (ml_float_rem (scalar_6 - (65536 : Float)) (1024 : Float)))] : (Array Float)));
                          (source_ml_temporary_readJavaScriptUtf8StringUnits_loop2 bytes units_4 index_3)))))
                    | source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_result.source_ml_temporary_readJavaScriptUtf8StringUnits_loop4_return source_ml_temporary_result4 => (source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_result.source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_return source_ml_temporary_result4)))
              | source_ml_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_temporary_readJavaScriptUtf8StringUnits_join3_return source_ml_temporary_result4_2 => (source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_result.source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_return source_ml_temporary_result4_2))))))) else (source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_result.source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_done units))

inductive source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_result where
  | source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_done : source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_result
  | source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_return (ml_temporary_result4 : (Array Float)) : source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_result
  deriving Repr, Inhabited

partial def source_ml_temporary_readJavaScriptUtf8StringUnits_loop1 (bytes : (Array Float)) (index : Float) : source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_result :=
  (if (decide (index < (Float.ofNat bytes.size))) then (let unit := (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then bytes[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if (((!(ml_is_safe_integer unit)) || (decide (unit < (0 : Float)))) || (decide (unit > (255 : Float)))) then (source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_result.source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_return (#[(-1 : Float)] : (Array Float))) else (let index_2 := (index + (1 : Float));
      (source_ml_temporary_readJavaScriptUtf8StringUnits_loop1 bytes index_2)))) else source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_result.source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_done)

def readJavaScriptUtf8StringUnits (bytes : (Array Float)) : (Array Float) :=
  (let index := (0 : Float);
    (let source_ml_source1_s1 := (source_ml_temporary_readJavaScriptUtf8StringUnits_loop1 bytes index);
      (match source_ml_source1_s1 with
        | source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_result.source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_done => (let units := (#[] : (Array Float));
            (let index_2 := (0 : Float);
              (let source_ml_source1_s2 := (source_ml_temporary_readJavaScriptUtf8StringUnits_loop2 bytes units index_2);
                (match source_ml_source1_s2 with
                  | source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_result.source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_done units_2 => units_2
                  | source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_result.source_ml_temporary_readJavaScriptUtf8StringUnits_loop2_return source_ml_temporary_result2 => source_ml_temporary_result2))))
        | source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_result.source_ml_temporary_readJavaScriptUtf8StringUnits_loop1_return source_ml_temporary_result4 => source_ml_temporary_result4)))

def readUtf8TextUnits (bytes : (Array Float)) : (Array Float) :=
  (readJavaScriptUtf8StringUnits bytes)

partial def source_ml_temporary_sliceUtf16Text_loop5 (text : (Array Float)) (last : Float) (result : (Array Float)) («at» : Float) : (Array Float) :=
  (if (decide («at» < last)) then (let result_2 := (result ++ (#[(match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float))] : (Array Float)));
    (let at_2 := («at» + (1 : Float));
      (source_ml_temporary_sliceUtf16Text_loop5 text last result_2 at_2))) else result)

def sliceUtf16Text (text : (Array Float)) (start : Float) («end» : Float) : (Array Float) :=
  (let first := (readUtf16TextSlicePosition start (Float.ofNat text.size));
    (let last := (readUtf16TextSlicePosition «end» (Float.ofNat text.size));
      (let result := (#[] : (Array Float));
        (let «at» := first;
          (let result_2 := (source_ml_temporary_sliceUtf16Text_loop5 text last result «at»);
            result_2)))))

def substringUtf16Text (text : (Array Float)) (start : Float) («end» : Float) : (Array Float) :=
  (let first := (ml_min (ml_max (readUtf16TextInteger start) (0 : Float)) (Float.ofNat text.size));
    (let last := (ml_min (ml_max (readUtf16TextInteger «end») (0 : Float)) (Float.ofNat text.size));
      (sliceUtf16Text text (ml_min first last) (ml_max first last))))

partial def source_ml_temporary_pointsUtf16Text_loop6 (text : (Array Float)) (result : (Array (Array Float))) («at» : Float) : (Array (Array Float)) :=
  (if (decide («at» < (Float.ofNat text.size))) then (let first := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float));
    (let second := (match (let ml_index := («at» + (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
      (if ((((decide (first ≥ (55296 : Float))) && (decide (first ≤ (56319 : Float)))) && (decide (second ≥ (56320 : Float)))) && (decide (second ≤ (57343 : Float)))) then (let result_2 := (result ++ (#[(#[first, second] : (Array Float))] : (Array (Array Float))));
        (let at_2 := («at» + (1 : Float));
          (let at_3 := (at_2 + (1 : Float));
            (source_ml_temporary_pointsUtf16Text_loop6 text result_2 at_3)))) else (let result_3 := (result ++ (#[(#[first] : (Array Float))] : (Array (Array Float))));
        (let at_4 := («at» + (1 : Float));
          (source_ml_temporary_pointsUtf16Text_loop6 text result_3 at_4)))))) else result)

def pointsUtf16Text (text : (Array Float)) : (Array (Array Float)) :=
  (let result := (#[] : (Array (Array Float)));
    (let «at» := (0 : Float);
      (let result_2 := (source_ml_temporary_pointsUtf16Text_loop6 text result «at»);
        result_2)))

partial def source_ml_temporary_splitUtf16Text_loop7 (text : (Array Float)) (count : Float) (result : (Array (Array Float))) («at» : Float) : (Array (Array Float)) :=
  (if ((decide («at» < (Float.ofNat text.size))) && (decide ((Float.ofNat result.size) < count))) then (let result_2 := (result ++ (#[(#[(match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float))] : (Array Float))] : (Array (Array Float))));
    (let at_2 := («at» + (1 : Float));
      (source_ml_temporary_splitUtf16Text_loop7 text count result_2 at_2))) else result)

inductive source_ml_temporary_splitUtf16Text_loop8_result where
  | source_ml_temporary_splitUtf16Text_loop8_done (result : (Array (Array Float))) (start : Float) : source_ml_temporary_splitUtf16Text_loop8_result
  | source_ml_temporary_splitUtf16Text_loop8_return (ml_temporary_result8 : (Array (Array Float))) : source_ml_temporary_splitUtf16Text_loop8_result
  deriving Repr, Inhabited

inductive source_ml_temporary_matchUtf16Text_loop9_result where
  | source_ml_temporary_matchUtf16Text_loop9_done : source_ml_temporary_matchUtf16Text_loop9_result
  | source_ml_temporary_matchUtf16Text_loop9_return (ml_temporary_result9 : Bool) : source_ml_temporary_matchUtf16Text_loop9_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_temporary_matchUtf16Text_loop9 (text : (Array Float)) (search : (Array Float)) (start : Float) («at» : Float) : source_ml_temporary_matchUtf16Text_loop9_result :=
  (if (decide («at» < (Float.ofNat search.size))) then (if (let source_ml_source1_comparison_operand13 := (let ml_index := (start + «at»); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none);
    (let source_ml_source1_comparison_operand14 := (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then search[ml_index.toUInt64.toNat]? else none);
      (!(match (source_ml_source1_comparison_operand13, source_ml_source1_comparison_operand14) with | (some ml_left, some ml_right) => (ml_left == ml_right) | (none, none) => true | _ => false)))) then (source_ml_temporary_matchUtf16Text_loop9_result.source_ml_temporary_matchUtf16Text_loop9_return false) else (let at_2 := («at» + (1 : Float));
    (source_ml_temporary_matchUtf16Text_loop9 text search start at_2))) else source_ml_temporary_matchUtf16Text_loop9_result.source_ml_temporary_matchUtf16Text_loop9_done)

def matchUtf16Text (text : (Array Float)) (search : (Array Float)) (start : Float) : Bool :=
  (if ((decide (start < (0 : Float))) || (decide ((start + (Float.ofNat search.size)) > (Float.ofNat text.size)))) then false else (let «at» := (0 : Float);
    (let source_ml_source1_s4 := (source_ml_temporary_matchUtf16Text_loop9 text search start «at»);
      (match source_ml_source1_s4 with
        | source_ml_temporary_matchUtf16Text_loop9_result.source_ml_temporary_matchUtf16Text_loop9_done => true
        | source_ml_temporary_matchUtf16Text_loop9_result.source_ml_temporary_matchUtf16Text_loop9_return source_ml_temporary_result9 => source_ml_temporary_result9))))

def startsUtf16Text (text : (Array Float)) (search : (Array Float)) : Bool :=
  (matchUtf16Text text search (0 : Float))

def endsUtf16Text (text : (Array Float)) (search : (Array Float)) : Bool :=
  (matchUtf16Text text search ((Float.ofNat text.size) - (Float.ofNat search.size)))

inductive source_ml_temporary_indexUtf16Text_loop10_result where
  | source_ml_temporary_indexUtf16Text_loop10_done : source_ml_temporary_indexUtf16Text_loop10_result
  | source_ml_temporary_indexUtf16Text_loop10_return (ml_temporary_result10 : Float) : source_ml_temporary_indexUtf16Text_loop10_result
  deriving Repr, Inhabited

partial def source_ml_temporary_indexUtf16Text_loop10 (text : (Array Float)) (search : (Array Float)) («at» : Float) : source_ml_temporary_indexUtf16Text_loop10_result :=
  (if (decide («at» ≤ ((Float.ofNat text.size) - (Float.ofNat search.size)))) then (if (matchUtf16Text text search «at») then (source_ml_temporary_indexUtf16Text_loop10_result.source_ml_temporary_indexUtf16Text_loop10_return «at») else (let at_2 := («at» + (1 : Float));
    (source_ml_temporary_indexUtf16Text_loop10 text search at_2))) else source_ml_temporary_indexUtf16Text_loop10_result.source_ml_temporary_indexUtf16Text_loop10_done)

def indexUtf16Text (text : (Array Float)) (search : (Array Float)) (position : Float) : Float :=
  (let start := (ml_min (ml_max (readUtf16TextInteger position) (0 : Float)) (Float.ofNat text.size));
    (let «at» := start;
      (let source_ml_source1_s5 := (source_ml_temporary_indexUtf16Text_loop10 text search «at»);
        (match source_ml_source1_s5 with
          | source_ml_temporary_indexUtf16Text_loop10_result.source_ml_temporary_indexUtf16Text_loop10_done => (-1 : Float)
          | source_ml_temporary_indexUtf16Text_loop10_result.source_ml_temporary_indexUtf16Text_loop10_return source_ml_temporary_result10 => source_ml_temporary_result10))))

def includesUtf16Text (text : (Array Float)) (search : (Array Float)) : Bool :=
  (decide ((indexUtf16Text text search (0 : Float)) ≥ (0 : Float)))

partial def source_ml_temporary_splitUtf16Text_loop8 (text : (Array Float)) (separator : (Array Float)) (count : Float) (result : (Array (Array Float))) (start : Float) : source_ml_temporary_splitUtf16Text_loop8_result :=
  (let found := (indexUtf16Text text separator start);
    (if (decide (found < (0 : Float))) then (source_ml_temporary_splitUtf16Text_loop8_result.source_ml_temporary_splitUtf16Text_loop8_done result start) else (let result_2 := (result ++ (#[(sliceUtf16Text text start found)] : (Array (Array Float))));
      (if ((Float.ofNat result_2.size) == count) then (source_ml_temporary_splitUtf16Text_loop8_result.source_ml_temporary_splitUtf16Text_loop8_return result_2) else (let start_2 := (found + (Float.ofNat separator.size));
        (source_ml_temporary_splitUtf16Text_loop8 text separator count result_2 start_2))))))

def splitUtf16Text (text : (Array Float)) (separator : (Array Float)) (limit : Float) (missing : Bool) : (Array (Array Float)) :=
  (let count := (normalizeJavaScriptUnsignedInteger limit);
    (let result := (#[] : (Array (Array Float)));
      (if (count == (0 : Float)) then result else (if missing then (#[(sliceUtf16Text text (0 : Float) (Float.ofNat text.size))] : (Array (Array Float))) else (if ((Float.ofNat separator.size) == (0 : Float)) then (let «at» := (0 : Float);
        (let result_2 := (source_ml_temporary_splitUtf16Text_loop7 text count result «at»);
          result_2)) else (let start := (0 : Float);
        (let source_ml_source1_s3 := (source_ml_temporary_splitUtf16Text_loop8 text separator count result start);
          (match source_ml_source1_s3 with
            | source_ml_temporary_splitUtf16Text_loop8_result.source_ml_temporary_splitUtf16Text_loop8_done result_3 start_2 => (let result_4 := (result_3 ++ (#[(sliceUtf16Text text start_2 (Float.ofNat text.size))] : (Array (Array Float))));
                result_4)
            | source_ml_temporary_splitUtf16Text_loop8_result.source_ml_temporary_splitUtf16Text_loop8_return source_ml_temporary_result8 => source_ml_temporary_result8))))))))

inductive source_ml_temporary_lastIndexUtf16Text_loop11_result where
  | source_ml_temporary_lastIndexUtf16Text_loop11_done : source_ml_temporary_lastIndexUtf16Text_loop11_result
  | source_ml_temporary_lastIndexUtf16Text_loop11_return (ml_temporary_result11 : Float) : source_ml_temporary_lastIndexUtf16Text_loop11_result
  deriving Repr, Inhabited

partial def source_ml_temporary_lastIndexUtf16Text_loop11 (text : (Array Float)) (search : (Array Float)) («at» : Float) : source_ml_temporary_lastIndexUtf16Text_loop11_result :=
  (if (decide («at» ≥ (0 : Float))) then (if (matchUtf16Text text search «at») then (source_ml_temporary_lastIndexUtf16Text_loop11_result.source_ml_temporary_lastIndexUtf16Text_loop11_return «at») else (let at_2 := («at» - (1 : Float));
    (source_ml_temporary_lastIndexUtf16Text_loop11 text search at_2))) else source_ml_temporary_lastIndexUtf16Text_loop11_result.source_ml_temporary_lastIndexUtf16Text_loop11_done)

def lastIndexUtf16Text (text : (Array Float)) (search : (Array Float)) (position : Float) : Float :=
  (let integer := (if (Float.isNaN position) then (1.0 / 0.0 : Float) else (readUtf16TextInteger position));
    (let start := (ml_min (ml_max integer (0 : Float)) ((Float.ofNat text.size) - (Float.ofNat search.size)));
      (let «at» := start;
        (let source_ml_source1_s6 := (source_ml_temporary_lastIndexUtf16Text_loop11 text search «at»);
          (match source_ml_source1_s6 with
            | source_ml_temporary_lastIndexUtf16Text_loop11_result.source_ml_temporary_lastIndexUtf16Text_loop11_done => (-1 : Float)
            | source_ml_temporary_lastIndexUtf16Text_loop11_result.source_ml_temporary_lastIndexUtf16Text_loop11_return source_ml_temporary_result11 => source_ml_temporary_result11)))))

inductive source_ml_temporary_compareUtf16Text_loop12_result where
  | source_ml_temporary_compareUtf16Text_loop12_done : source_ml_temporary_compareUtf16Text_loop12_result
  | source_ml_temporary_compareUtf16Text_loop12_return (ml_temporary_result12 : Float) : source_ml_temporary_compareUtf16Text_loop12_result
  deriving Repr, Inhabited

partial def source_ml_temporary_compareUtf16Text_loop12 (left : (Array Float)) (right : (Array Float)) (count : Float) («at» : Float) : source_ml_temporary_compareUtf16Text_loop12_result :=
  (if (decide («at» < count)) then (let a := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then left[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float));
    (let b := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then right[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float));
      (if (a != b) then (source_ml_temporary_compareUtf16Text_loop12_result.source_ml_temporary_compareUtf16Text_loop12_return (if (decide (a < b)) then (-1 : Float) else (1 : Float))) else (let at_2 := («at» + (1 : Float));
        (source_ml_temporary_compareUtf16Text_loop12 left right count at_2))))) else source_ml_temporary_compareUtf16Text_loop12_result.source_ml_temporary_compareUtf16Text_loop12_done)

def compareUtf16Text (left : (Array Float)) (right : (Array Float)) : Float :=
  (let count := (ml_min (Float.ofNat left.size) (Float.ofNat right.size));
    (let «at» := (0 : Float);
      (let source_ml_source1_s7 := (source_ml_temporary_compareUtf16Text_loop12 left right count «at»);
        (match source_ml_source1_s7 with
          | source_ml_temporary_compareUtf16Text_loop12_result.source_ml_temporary_compareUtf16Text_loop12_done => (if ((Float.ofNat left.size) == (Float.ofNat right.size)) then (0 : Float) else (if (decide ((Float.ofNat left.size) < (Float.ofNat right.size))) then (-1 : Float) else (1 : Float)))
          | source_ml_temporary_compareUtf16Text_loop12_result.source_ml_temporary_compareUtf16Text_loop12_return source_ml_temporary_result12 => source_ml_temporary_result12))))

partial def source_ml_temporary_joinUtf16Text_loop14 (separator : (Array Float)) (result : (Array Float)) (offset : Float) : (Array Float) :=
  (if (decide (offset < (Float.ofNat separator.size))) then (let result_2 := (result ++ (#[(match (let ml_index := offset; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then separator[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float))] : (Array Float)));
    (let offset_2 := (offset + (1 : Float));
      (source_ml_temporary_joinUtf16Text_loop14 separator result_2 offset_2))) else result)

partial def source_ml_temporary_joinUtf16Text_loop15 (result : (Array Float)) (value : (Array Float)) (offset : Float) : (Array Float) :=
  (if (decide (offset < (Float.ofNat value.size))) then (let result_2 := (result ++ (#[(match (let ml_index := offset; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then value[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float))] : (Array Float)));
    (let offset_2 := (offset + (1 : Float));
      (source_ml_temporary_joinUtf16Text_loop15 result_2 value offset_2))) else result)

partial def source_ml_temporary_joinUtf16Text_loop13 (values : (Array (Array Float))) (separator : (Array Float)) (result : (Array Float)) («at» : Float) : (Array Float) :=
  (if (decide («at» < (Float.ofNat values.size))) then (let result_3 := (if («at» != (0 : Float)) then (let offset := (0 : Float);
    (let result_2 := (source_ml_temporary_joinUtf16Text_loop14 separator result offset);
      result_2)) else result);
    (let value := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then values[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (#[] : (Array Float)));
      (let offset_2 := (0 : Float);
        (let result_4 := (source_ml_temporary_joinUtf16Text_loop15 result_3 value offset_2);
          (let at_2 := («at» + (1 : Float));
            (source_ml_temporary_joinUtf16Text_loop13 values separator result_4 at_2)))))) else result)

def joinUtf16Text (values : (Array (Array Float))) (separator : (Array Float)) : (Array Float) :=
  (let result := (#[] : (Array Float));
    (let «at» := (0 : Float);
      (let result_2 := (source_ml_temporary_joinUtf16Text_loop13 values separator result «at»);
        result_2)))

partial def source_ml_temporary_trimUtf16Text_loop16 (text : (Array Float)) (first : Float) (last : Float) : Float :=
  (if ((decide (first < last)) && (acceptJavaScriptWhitespaceUnit (match (let ml_index := first; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)))) then (let first_2 := (first + (1 : Float));
    (source_ml_temporary_trimUtf16Text_loop16 text first_2 last)) else first)

partial def source_ml_temporary_trimUtf16Text_loop17 (text : (Array Float)) (first : Float) (last : Float) : Float :=
  (if ((decide (last > first)) && (acceptJavaScriptWhitespaceUnit (match (let ml_index := (last - (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)))) then (let last_2 := (last - (1 : Float));
    (source_ml_temporary_trimUtf16Text_loop17 text first last_2)) else last)

def trimUtf16Text (text : (Array Float)) (start : Bool) («end» : Bool) : (Array Float) :=
  (let first := (0 : Float);
    (let last := (Float.ofNat text.size);
      (let first_3 := (if start then (let first_2 := (source_ml_temporary_trimUtf16Text_loop16 text first last);
        first_2) else first);
        (let last_3 := (if «end» then (let last_2 := (source_ml_temporary_trimUtf16Text_loop17 text first_3 last);
          last_2) else last);
          (sliceUtf16Text text first_3 last_3)))))

partial def source_ml_temporary_characterUnitsUtf16Text_loop18 (values : (Array Float)) (result : (Array Float)) («at» : Float) : (Array Float) :=
  (if (decide («at» < (Float.ofNat values.size))) then (let value := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then values[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float));
    (let integer := (if (Float.isFinite value) then (ml_trunc value) else (0 : Float));
      (let residue := (ml_float_rem integer (65536 : Float));
        (let result_2 := (result ++ (#[(if (decide (residue < (0 : Float))) then (residue + (65536 : Float)) else (if (residue == (0 : Float)) then (0 : Float) else residue))] : (Array Float)));
          (let at_2 := («at» + (1 : Float));
            (source_ml_temporary_characterUnitsUtf16Text_loop18 values result_2 at_2)))))) else result)

def characterUnitsUtf16Text (values : (Array Float)) : (Array Float) :=
  (let result := (#[] : (Array Float));
    (let «at» := (0 : Float);
      (let result_2 := (source_ml_temporary_characterUnitsUtf16Text_loop18 values result «at»);
        result_2)))

inductive source_ml_temporary_codePointUnitsUtf16Text_loop19_result where
  | source_ml_temporary_codePointUnitsUtf16Text_loop19_done (result : (Array Float)) : source_ml_temporary_codePointUnitsUtf16Text_loop19_result
  | source_ml_temporary_codePointUnitsUtf16Text_loop19_return (ml_temporary_result19 : (Array Float)) : source_ml_temporary_codePointUnitsUtf16Text_loop19_result
  deriving Repr, Inhabited

partial def source_ml_temporary_codePointUnitsUtf16Text_loop19 (values : (Array Float)) (result : (Array Float)) («at» : Float) : source_ml_temporary_codePointUnitsUtf16Text_loop19_result :=
  (if (decide («at» < (Float.ofNat values.size))) then (let value := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then values[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if (((!(ml_is_integer value)) || (decide (value < (0 : Float)))) || (decide (value > (1114111 : Float)))) then (source_ml_temporary_codePointUnitsUtf16Text_loop19_result.source_ml_temporary_codePointUnitsUtf16Text_loop19_return (#[«at»] : (Array Float))) else (if (decide (value < (65536 : Float))) then (let result_2 := (result ++ (#[(if (value == (0 : Float)) then (0 : Float) else value)] : (Array Float)));
      (let at_2 := («at» + (1 : Float));
        (source_ml_temporary_codePointUnitsUtf16Text_loop19 values result_2 at_2))) else (let result_3 := (result ++ (#[((55296 : Float) + (Float.floor ((value - (65536 : Float)) / (1024 : Float)))), ((56320 : Float) + (ml_float_rem (value - (65536 : Float)) (1024 : Float)))] : (Array Float)));
      (let at_3 := («at» + (1 : Float));
        (source_ml_temporary_codePointUnitsUtf16Text_loop19 values result_3 at_3)))))) else (source_ml_temporary_codePointUnitsUtf16Text_loop19_result.source_ml_temporary_codePointUnitsUtf16Text_loop19_done result))

def codePointUnitsUtf16Text (values : (Array Float)) : (Array Float) :=
  (let result := (#[(-1 : Float)] : (Array Float));
    (let «at» := (0 : Float);
      (let source_ml_source1_s8 := (source_ml_temporary_codePointUnitsUtf16Text_loop19 values result «at»);
        (match source_ml_source1_s8 with
          | source_ml_temporary_codePointUnitsUtf16Text_loop19_result.source_ml_temporary_codePointUnitsUtf16Text_loop19_done result_2 => result_2
          | source_ml_temporary_codePointUnitsUtf16Text_loop19_result.source_ml_temporary_codePointUnitsUtf16Text_loop19_return source_ml_temporary_result19 => source_ml_temporary_result19))))

inductive source_ml_temporary_displayPointsUtf16Text_join21 where
  | source_ml_temporary_displayPointsUtf16Text_join21_next («at» : Float) (point : Float) : source_ml_temporary_displayPointsUtf16Text_join21
  deriving Repr, Inhabited

partial def source_ml_temporary_displayPointsUtf16Text_loop20 (text : (Array Float)) (result : (Array Float)) («at» : Float) : (Array Float) :=
  (if (decide («at» < (Float.ofNat text.size))) then (let point := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (65533 : Float));
    (let source_ml_source1_s15 := (if ((decide (point ≥ (55296 : Float))) && (decide (point ≤ (56319 : Float)))) then (let low := (match (let ml_index := («at» + (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
      (if ((decide (low ≥ (56320 : Float))) && (decide (low ≤ (57343 : Float)))) then (let point_2 := ((((65536 : Float) + ((point - (55296 : Float)) * (1024 : Float))) + low) - (56320 : Float));
        (let at_2 := («at» + (1 : Float));
          (source_ml_temporary_displayPointsUtf16Text_join21.source_ml_temporary_displayPointsUtf16Text_join21_next at_2 point_2))) else (let point_3 := (65533 : Float);
        (source_ml_temporary_displayPointsUtf16Text_join21.source_ml_temporary_displayPointsUtf16Text_join21_next «at» point_3)))) else (if ((decide (point ≥ (56320 : Float))) && (decide (point ≤ (57343 : Float)))) then (let point_4 := (65533 : Float);
      (source_ml_temporary_displayPointsUtf16Text_join21.source_ml_temporary_displayPointsUtf16Text_join21_next «at» point_4)) else (source_ml_temporary_displayPointsUtf16Text_join21.source_ml_temporary_displayPointsUtf16Text_join21_next «at» point)));
      (match source_ml_source1_s15 with
        | source_ml_temporary_displayPointsUtf16Text_join21.source_ml_temporary_displayPointsUtf16Text_join21_next at_3 point_5 => (let result_2 := (result ++ (#[point_5] : (Array Float)));
            (let at_4 := (at_3 + (1 : Float));
              (source_ml_temporary_displayPointsUtf16Text_loop20 text result_2 at_4)))))) else result)

def displayPointsUtf16Text (text : (Array Float)) : (Array Float) :=
  (let result := (#[] : (Array Float));
    (let «at» := (0 : Float);
      (let result_2 := (source_ml_temporary_displayPointsUtf16Text_loop20 text result «at»);
        result_2)))

partial def source_ml_temporary_displayUtf8FromUtf16Text_loop22 (bytes : (Array Float)) (points : (Array Float)) («at» : Float) : (Array Float) :=
  (if (decide («at» < (Float.ofNat points.size))) then (let point := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then points[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (65533 : Float));
    (if (decide (point < (128 : Float))) then (let bytes_2 := (bytes ++ (#[point] : (Array Float)));
      (let at_2 := («at» + (1 : Float));
        (source_ml_temporary_displayUtf8FromUtf16Text_loop22 bytes_2 points at_2))) else (if (decide (point < (2048 : Float))) then (let bytes_3 := (bytes ++ (#[((192 : Float) + (Float.floor (point / (64 : Float)))), ((128 : Float) + (ml_float_rem point (64 : Float)))] : (Array Float)));
      (let at_3 := («at» + (1 : Float));
        (source_ml_temporary_displayUtf8FromUtf16Text_loop22 bytes_3 points at_3))) else (if (decide (point < (65536 : Float))) then (let bytes_4 := (bytes ++ (#[((224 : Float) + (Float.floor (point / (4096 : Float)))), ((128 : Float) + (ml_float_rem (Float.floor (point / (64 : Float))) (64 : Float))), ((128 : Float) + (ml_float_rem point (64 : Float)))] : (Array Float)));
      (let at_4 := («at» + (1 : Float));
        (source_ml_temporary_displayUtf8FromUtf16Text_loop22 bytes_4 points at_4))) else (let bytes_5 := (bytes ++ (#[((240 : Float) + (Float.floor (point / (262144 : Float)))), ((128 : Float) + (ml_float_rem (Float.floor (point / (4096 : Float))) (64 : Float))), ((128 : Float) + (ml_float_rem (Float.floor (point / (64 : Float))) (64 : Float))), ((128 : Float) + (ml_float_rem point (64 : Float)))] : (Array Float)));
      (let at_5 := («at» + (1 : Float));
        (source_ml_temporary_displayUtf8FromUtf16Text_loop22 bytes_5 points at_5))))))) else bytes)

def displayUtf8FromUtf16Text (text : (Array Float)) : (Array Float) :=
  (let bytes := (#[] : (Array Float));
    (let points := (displayPointsUtf16Text text);
      (let «at» := (0 : Float);
        (let bytes_2 := (source_ml_temporary_displayUtf8FromUtf16Text_loop22 bytes points «at»);
          bytes_2))))

inductive source_ml_temporary_planUtf16TextSubstitution_loop23_result where
  | source_ml_temporary_planUtf16TextSubstitution_loop23_done (result : (Array Float)) (start : Float) : source_ml_temporary_planUtf16TextSubstitution_loop23_result
  deriving Repr, Inhabited

partial def source_ml_temporary_planUtf16TextSubstitution_loop23 (replacement : (Array Float)) (result : (Array Float)) (start : Float) («at» : Float) : source_ml_temporary_planUtf16TextSubstitution_loop23_result :=
  (if (decide («at» < (Float.ofNat replacement.size))) then (let next := (match (let ml_index := («at» + (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then replacement[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if (((match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then replacement[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float)) == (36 : Float)) && ((((next == (36 : Float)) || (next == (38 : Float))) || (next == (96 : Float))) || (next == (39 : Float)))) then (let result_3 := (if (decide (start < «at»)) then (let result_2 := (result ++ (#[(0 : Float), start, «at»] : (Array Float)));
      result_2) else result);
      (let result_6 := (if (next == (36 : Float)) then (let result_4 := (result_3 ++ (#[(0 : Float), «at», («at» + (1 : Float))] : (Array Float)));
        result_4) else (let result_5 := (result_3 ++ (#[(if (next == (38 : Float)) then (1 : Float) else (if (next == (96 : Float)) then (2 : Float) else (3 : Float))), (0 : Float), (0 : Float)] : (Array Float)));
        result_5));
        (let at_2 := («at» + (2 : Float));
          (let start_2 := at_2;
            (source_ml_temporary_planUtf16TextSubstitution_loop23 replacement result_6 start_2 at_2))))) else (let at_3 := («at» + (1 : Float));
      (source_ml_temporary_planUtf16TextSubstitution_loop23 replacement result start at_3)))) else (source_ml_temporary_planUtf16TextSubstitution_loop23_result.source_ml_temporary_planUtf16TextSubstitution_loop23_done result start))

def planUtf16TextSubstitution (replacement : (Array Float)) : (Array Float) :=
  (let result := (#[] : (Array Float));
    (let start := (0 : Float);
      (let «at» := (0 : Float);
        (let source_ml_source1_s9 := (source_ml_temporary_planUtf16TextSubstitution_loop23 replacement result start «at»);
          (match source_ml_source1_s9 with
            | source_ml_temporary_planUtf16TextSubstitution_loop23_result.source_ml_temporary_planUtf16TextSubstitution_loop23_done result_2 start_2 => (let result_4 := (if (decide (start_2 < (Float.ofNat replacement.size))) then (let result_3 := (result_2 ++ (#[(0 : Float), start_2, (Float.ofNat replacement.size)] : (Array Float)));
                result_3) else result_2);
                result_4))))))

partial def source_ml_temporary_replaceUtf16Text_loop25 (text : (Array Float)) (result : (Array Float)) («at» : Float) : (Array Float) :=
  (if (decide («at» < (Float.ofNat text.size))) then (let result_2 := (result ++ (#[(match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float))] : (Array Float)));
    (let at_2 := («at» + (1 : Float));
      (source_ml_temporary_replaceUtf16Text_loop25 text result_2 at_2))) else result)

inductive source_ml_temporary_replaceUtf16Text_loop24_result where
  | source_ml_temporary_replaceUtf16Text_loop24_done (result : (Array Float)) (start : Float) : source_ml_temporary_replaceUtf16Text_loop24_result
  deriving Repr, Inhabited

partial def source_ml_temporary_replaceUtf16Text_loop26 (text : (Array Float)) (result : (Array Float)) (found : Float) («at» : Float) : (Array Float) :=
  (if (decide («at» < found)) then (let result_2 := (result ++ (#[(match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float))] : (Array Float)));
    (let at_2 := («at» + (1 : Float));
      (source_ml_temporary_replaceUtf16Text_loop26 text result_2 found at_2))) else result)

partial def source_ml_temporary_replaceUtf16Text_loop28 (text : (Array Float)) (replacement : (Array Float)) (result : (Array Float)) (kind : Float) (last : Float) («at» : Float) : (Array Float) :=
  (if (decide («at» < last)) then (let result_2 := (result ++ (#[(if (kind == (0 : Float)) then (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then replacement[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float)) else (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float)))] : (Array Float)));
    (let at_2 := («at» + (1 : Float));
      (source_ml_temporary_replaceUtf16Text_loop28 text replacement result_2 kind last at_2))) else result)

partial def source_ml_temporary_replaceUtf16Text_loop27 (text : (Array Float)) (search : (Array Float)) (replacement : (Array Float)) (result : (Array Float)) (plan : (Array Float)) (found : Float) (instruction : Float) : (Array Float) :=
  (if (decide (instruction < (Float.ofNat plan.size))) then (let kind := (match (let ml_index := instruction; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then plan[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float));
    (let first := (if (kind == (0 : Float)) then (match (let ml_index := (instruction + (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then plan[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float)) else (if (kind == (2 : Float)) then (0 : Float) else (if (kind == (1 : Float)) then found else (found + (Float.ofNat search.size)))));
      (let last := (if (kind == (0 : Float)) then (match (let ml_index := (instruction + (2 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then plan[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float)) else (if (kind == (2 : Float)) then found else (if (kind == (1 : Float)) then (found + (Float.ofNat search.size)) else (Float.ofNat text.size))));
        (let «at» := first;
          (let result_2 := (source_ml_temporary_replaceUtf16Text_loop28 text replacement result kind last «at»);
            (let instruction_2 := (instruction + (3 : Float));
              (source_ml_temporary_replaceUtf16Text_loop27 text search replacement result_2 plan found instruction_2))))))) else result)

partial def source_ml_temporary_replaceUtf16Text_loop24 (text : (Array Float)) (search : (Array Float)) (replacement : (Array Float)) (all : Bool) (result : (Array Float)) (plan : (Array Float)) (start : Float) («from» : Float) : source_ml_temporary_replaceUtf16Text_loop24_result :=
  (let found := (indexUtf16Text text search «from»);
    (if (decide (found < (0 : Float))) then (source_ml_temporary_replaceUtf16Text_loop24_result.source_ml_temporary_replaceUtf16Text_loop24_done result start) else (let «at» := start;
      (let result_2 := (source_ml_temporary_replaceUtf16Text_loop26 text result found «at»);
        (let instruction := (0 : Float);
          (let result_3 := (source_ml_temporary_replaceUtf16Text_loop27 text search replacement result_2 plan found instruction);
            (let start_2 := (found + (Float.ofNat search.size));
              (if ((!all) || (((Float.ofNat search.size) == (0 : Float)) && (found == (Float.ofNat text.size)))) then (source_ml_temporary_replaceUtf16Text_loop24_result.source_ml_temporary_replaceUtf16Text_loop24_done result_3 start_2) else (let from_2 := (start_2 + (if ((Float.ofNat search.size) == (0 : Float)) then (1 : Float) else (0 : Float)));
                (source_ml_temporary_replaceUtf16Text_loop24 text search replacement all result_3 plan start_2 from_2))))))))))

def replaceUtf16Text (text : (Array Float)) (search : (Array Float)) (replacement : (Array Float)) (all : Bool) : (Array Float) :=
  (let result := (#[] : (Array Float));
    (let plan := (planUtf16TextSubstitution replacement);
      (let start := (0 : Float);
        (let «from» := (0 : Float);
          (let source_ml_source1_s10 := (source_ml_temporary_replaceUtf16Text_loop24 text search replacement all result plan start «from»);
            (match source_ml_source1_s10 with
              | source_ml_temporary_replaceUtf16Text_loop24_result.source_ml_temporary_replaceUtf16Text_loop24_done result_2 start_2 => (let «at» := start_2;
                  (let result_3 := (source_ml_temporary_replaceUtf16Text_loop25 text result_2 «at»);
                    result_3))))))))

def main : IO Unit := do
  pure ()

def replace := replaceUtf16Text
def substitutionPlan := planUtf16TextSubstitution
def split := splitUtf16Text
def slicePosition := readUtf16TextSlicePosition
def slice := sliceUtf16Text
def substring := substringUtf16Text
def character := characterUtf16Text
def relativeCharacter := relativeCharacterUtf16Text
def codePoint := codePointUtf16Text
def points := pointsUtf16Text
def index := indexUtf16Text
def lastIndex := lastIndexUtf16Text
def compare := compareUtf16Text
def starts := startsUtf16Text
def ends := endsUtf16Text
def includes := includesUtf16Text
def trim := trimUtf16Text
def characterUnits := characterUnitsUtf16Text
def codePointUnits := codePointUnitsUtf16Text
def join := joinUtf16Text
def displayUtf8 := displayUtf8FromUtf16Text
def displayPoints := displayPointsUtf16Text
def fromUtf8 := readUtf8TextUnits
end MLUtf16Text


def refuseRequiredNumberFact : Float :=
  (ml_raiseJavaScriptRangeError (#[109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108, 117, 101] : Array Nat))

def refuseRequiredTextFact : (Array Nat) :=
  (ml_raiseJavaScriptRangeError (#[109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 116, 101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108, 117, 101] : Array Nat))

def refuseRequiredBooleanFact : Bool :=
  (ml_raiseJavaScriptRangeError (#[109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 66, 111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108, 117, 101] : Array Nat))

def readRequiredNumberFact (values : (Array Float)) (index : Float) : Float :=
  (if (((decide (index < (0 : Float))) || ((ml_float_rem index (1 : Float)) != (0 : Float))) || (decide (index ≥ (Float.ofNat values.size)))) then (ml_raiseJavaScriptRangeError (#[105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110] : Array Nat)) else (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then values[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => refuseRequiredNumberFact))

def readRequiredTextFact (values : (Array (Array Nat))) (index : Float) : (Array Nat) :=
  (if (((decide (index < (0 : Float))) || ((ml_float_rem index (1 : Float)) != (0 : Float))) || (decide (index ≥ (Float.ofNat values.size)))) then (ml_raiseJavaScriptRangeError (#[105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 116, 101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110] : Array Nat)) else (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then values[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => refuseRequiredTextFact))

def readRequiredBooleanFact (values : (Array Bool)) (index : Float) : Bool :=
  (if (((decide (index < (0 : Float))) || ((ml_float_rem index (1 : Float)) != (0 : Float))) || (decide (index ≥ (Float.ofNat values.size)))) then (ml_raiseJavaScriptRangeError (#[105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 66, 111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110] : Array Nat)) else (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then values[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => refuseRequiredBooleanFact))

def refuseRequiredRecordName : (Array Float) :=
  (ml_raiseJavaScriptRangeError (#[109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 110, 97, 109, 101, 32, 114, 111, 119] : Array Nat))

def readRequiredRecordName (names : (Array (Array Float))) (index : Float) : (Array Float) :=
  (if (((decide (index < (0 : Float))) || ((ml_float_rem index (1 : Float)) != (0 : Float))) || (decide (index ≥ (Float.ofNat names.size)))) then (ml_raiseJavaScriptRangeError (#[105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 110, 97, 109, 101, 32, 112, 111, 115, 105, 116, 105, 111, 110] : Array Nat)) else (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then names[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => refuseRequiredRecordName))

def readRecordProjection : (Array Nat) :=
  (#[99, 104, 101, 99, 107, 101, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 118, 97, 108, 117, 101, 115] : Array Nat)

def readRecordSpreadRole (kind : (Array Nat)) : (Array Nat) :=
  (if ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[118, 97, 114] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[111, 112, 101, 110] : Array Nat) else (if ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[114, 101, 99, 111, 114, 100] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[102, 105, 101, 108, 100, 115] : Array Nat) else (#[114, 101, 102, 117, 115, 101] : Array Nat)))

def readRecordTypeConstraintError (action : Float) (name : (Array Nat)) : (Array Nat) :=
  (if ((action == (3 : Float)) || (action == (4 : Float))) then (((#[114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32] : Array Nat) ++ name) ++ (#[32, 105, 115, 32, 109, 105, 115, 115, 105, 110, 103] : Array Nat)) else (#[] : Array Nat))

def readRecordMethodError (kind : (Array Nat)) (operation : (Array Nat)) (parameters : Float) (argumentsCount : Float) : (Array Nat) :=
  (if ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[102, 117, 110, 99, 116, 105, 111, 110] : Array Nat)).map Float.ofNat)) != (0 : Float)) then ((#[114, 101, 99, 111, 114, 100, 32, 104, 97, 115, 32, 110, 111, 32, 109, 101, 116, 104, 111, 100, 32] : Array Nat) ++ operation) else (if (argumentsCount != parameters) then (((((#[46] : Array Nat) ++ operation) ++ (#[40, 41, 32, 101, 120, 112, 101, 99, 116, 115, 32] : Array Nat)) ++ (String.foldl (fun (ml_units : Array Nat) ml_character => if ml_character.toNat < 65536 then ml_units.push ml_character.toNat else (ml_units.push (55296 + (ml_character.toNat - 65536) / 1024)).push (56320 + (ml_character.toNat - 65536) % 1024)) #[] (ml_js_number parameters))) ++ (#[32, 97, 114, 103, 117, 109, 101, 110, 116, 115] : Array Nat)) else (#[] : Array Nat)))

def readJavaScriptLiteralPropertyForm (kind : (Array Nat)) (index : Float) : (Array Nat) :=
  (if (((((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[97, 114, 114, 97, 121] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[116, 117, 112, 108, 101] : Array Nat)).map Float.ofNat)) == (0 : Float))) || ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[115, 116, 114, 105, 110, 103] : Array Nat)).map Float.ofNat)) == (0 : Float))) && (decide (index ≥ (0 : Float)))) then (#[105, 110, 100, 101, 120] : Array Nat) else (#[102, 105, 101, 108, 100] : Array Nat))

def readJavaScriptStringIndexNumberForm (kind : (Array Nat)) (element : (Array Nat)) : (Array Nat) :=
  (if ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[102, 108, 111, 97, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[105, 100, 101, 110, 116, 105, 116, 121] : Array Nat) else (if (((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[105, 110, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[110, 97, 116] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (#[99, 111, 110, 118, 101, 114, 116] : Array Nat) else (if (((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[117, 110, 105, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) || (((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[111, 112, 116, 105, 111, 110] : Array Nat)).map Float.ofNat)) == (0 : Float)) && ((MLUtf16Text.compare ((element).map Float.ofNat) (((#[117, 110, 105, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)))) then (#[97, 98, 115, 101, 110, 116] : Array Nat) else (if (((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[111, 112, 116, 105, 111, 110] : Array Nat)).map Float.ofNat)) == (0 : Float)) && ((((MLUtf16Text.compare ((element).map Float.ofNat) (((#[102, 108, 111, 97, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((element).map Float.ofNat) (((#[105, 110, 116] : Array Nat)).map Float.ofNat)) == (0 : Float))) || ((MLUtf16Text.compare ((element).map Float.ofNat) (((#[110, 97, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)))) then (#[111, 112, 116, 105, 111, 110, 97, 108] : Array Nat) else (#[114, 101, 102, 117, 115, 101] : Array Nat)))))

def refuseJavaScriptRecordOrderName : (Array Float) :=
  (ml_raiseJavaScriptTypeError (#[97, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 110, 101, 101, 100, 115, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 32, 102, 105, 101, 108, 100, 32, 110, 97, 109, 101] : Array Nat))

def refuseJavaScriptRecordOrderPosition : Float :=
  (ml_raiseJavaScriptTypeError (#[97, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 110, 101, 101, 100, 115, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 102, 105, 101, 108, 100, 32, 112, 111, 115, 105, 116, 105, 111, 110] : Array Nat))

def readJavaScriptRecordOrderName (names : (Array (Array Float))) (index : Float) : (Array Float) :=
  (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then names[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => refuseJavaScriptRecordOrderName)

def readJavaScriptRecordOrderPosition (positions : (Array Float)) (index : Float) : Float :=
  (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then positions[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => refuseJavaScriptRecordOrderPosition)

partial def source_ml_temporary_readRecordValueOrder_loop1 (count : Float) (reverse : Bool) (order : (Array Float)) (index : Float) : (Array Float) :=
  (if (decide (index < count)) then (let order_2 := (order ++ (#[(if reverse then ((count - index) - (1 : Float)) else index)] : (Array Float)));
    (let index_2 := (index + (1 : Float));
      (source_ml_temporary_readRecordValueOrder_loop1 count reverse order_2 index_2))) else order)

def readRecordValueOrder (count : Float) (reverse : Bool) : (Array Float) :=
  (if ((!(ml_is_safe_integer count)) || (decide (count < (0 : Float)))) then (#[(-1 : Float)] : (Array Float)) else (let order := (#[] : (Array Float));
    (let index := (0 : Float);
      (let order_2 := (source_ml_temporary_readRecordValueOrder_loop1 count reverse order index);
        order_2))))

inductive source_ml_temporary_readRecordProjectionError_loop2_result where
  | source_ml_temporary_readRecordProjectionError_loop2_done : source_ml_temporary_readRecordProjectionError_loop2_result
  | source_ml_temporary_readRecordProjectionError_loop2_return (ml_temporary_result2 : (Array Nat)) : source_ml_temporary_readRecordProjectionError_loop2_result
  deriving Repr, DecidableEq, Inhabited

inductive source_ml_temporary_acceptRecordFieldName_loop3_result where
  | source_ml_temporary_acceptRecordFieldName_loop3_done : source_ml_temporary_acceptRecordFieldName_loop3_result
  | source_ml_temporary_acceptRecordFieldName_loop3_return (ml_temporary_result3 : Bool) : source_ml_temporary_acceptRecordFieldName_loop3_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_temporary_acceptRecordFieldName_loop3 (name : (Array Float)) (index : Float) : source_ml_temporary_acceptRecordFieldName_loop3_result :=
  (if (decide (index < (Float.ofNat name.size))) then (let unit := (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then name[ml_index.toUInt64.toNat]? else none);
    (if (((!(match unit with | some ml_present => (ml_is_integer ml_present) | none => false)) || (let source_ml_source1_comparison_operand18 := unit;
      (let source_ml_source1_comparison_operand19 := (0 : Float);
        (if (match (source_ml_source1_comparison_operand18, none) with | (some ml_left, some ml_right) => (ml_left == ml_right) | (none, none) => true | _ => false) then (decide ((0.0 / 0.0 : Float) < source_ml_source1_comparison_operand19)) else (decide ((source_ml_source1_comparison_operand18.get!) < source_ml_source1_comparison_operand19)))))) || (let source_ml_source1_comparison_operand20 := unit;
      (let source_ml_source1_comparison_operand21 := (65535 : Float);
        (if (match (source_ml_source1_comparison_operand20, none) with | (some ml_left, some ml_right) => (ml_left == ml_right) | (none, none) => true | _ => false) then (decide ((0.0 / 0.0 : Float) > source_ml_source1_comparison_operand21)) else (decide ((source_ml_source1_comparison_operand20.get!) > source_ml_source1_comparison_operand21)))))) then (source_ml_temporary_acceptRecordFieldName_loop3_result.source_ml_temporary_acceptRecordFieldName_loop3_return false) else (let index_2 := (index + (1 : Float));
      (source_ml_temporary_acceptRecordFieldName_loop3 name index_2)))) else source_ml_temporary_acceptRecordFieldName_loop3_result.source_ml_temporary_acceptRecordFieldName_loop3_done)

def acceptRecordFieldName (name : (Array Float)) : Bool :=
  (let index := (0 : Float);
    (let source_ml_source1_s2 := (source_ml_temporary_acceptRecordFieldName_loop3 name index);
      (match source_ml_source1_s2 with
        | source_ml_temporary_acceptRecordFieldName_loop3_result.source_ml_temporary_acceptRecordFieldName_loop3_done => true
        | source_ml_temporary_acceptRecordFieldName_loop3_result.source_ml_temporary_acceptRecordFieldName_loop3_return source_ml_temporary_result3 => source_ml_temporary_result3)))

inductive source_ml_temporary_compareRecordFieldNames_loop4_result where
  | source_ml_temporary_compareRecordFieldNames_loop4_done : source_ml_temporary_compareRecordFieldNames_loop4_result
  | source_ml_temporary_compareRecordFieldNames_loop4_return (ml_temporary_result4 : Float) : source_ml_temporary_compareRecordFieldNames_loop4_result
  deriving Repr, Inhabited

partial def source_ml_temporary_compareRecordFieldNames_loop4 (left : (Array Float)) (right : (Array Float)) («end» : Float) (index : Float) : source_ml_temporary_compareRecordFieldNames_loop4_result :=
  (if (decide (index < «end»)) then (if (decide ((readRequiredNumberFact left index) < (readRequiredNumberFact right index))) then (source_ml_temporary_compareRecordFieldNames_loop4_result.source_ml_temporary_compareRecordFieldNames_loop4_return (-1 : Float)) else (if (decide ((readRequiredNumberFact left index) > (readRequiredNumberFact right index))) then (source_ml_temporary_compareRecordFieldNames_loop4_result.source_ml_temporary_compareRecordFieldNames_loop4_return (1 : Float)) else (let index_2 := (index + (1 : Float));
    (source_ml_temporary_compareRecordFieldNames_loop4 left right «end» index_2)))) else source_ml_temporary_compareRecordFieldNames_loop4_result.source_ml_temporary_compareRecordFieldNames_loop4_done)

def compareRecordFieldNames (left : (Array Float)) (right : (Array Float)) : Float :=
  (let «end» := (if (decide ((Float.ofNat left.size) < (Float.ofNat right.size))) then (Float.ofNat left.size) else (Float.ofNat right.size));
    (let index := (0 : Float);
      (let source_ml_source1_s3 := (source_ml_temporary_compareRecordFieldNames_loop4 left right «end» index);
        (match source_ml_source1_s3 with
          | source_ml_temporary_compareRecordFieldNames_loop4_result.source_ml_temporary_compareRecordFieldNames_loop4_done => (if (decide ((Float.ofNat left.size) < (Float.ofNat right.size))) then (-1 : Float) else (if (decide ((Float.ofNat left.size) > (Float.ofNat right.size))) then (1 : Float) else (0 : Float)))
          | source_ml_temporary_compareRecordFieldNames_loop4_result.source_ml_temporary_compareRecordFieldNames_loop4_return source_ml_temporary_result4 => source_ml_temporary_result4))))

partial def source_ml_temporary_readRecordProjectionError_loop2 (names : (Array (Array Float))) (entries : (Array Bool)) (index : Float) : source_ml_temporary_readRecordProjectionError_loop2_result :=
  (if (decide (index < (Float.ofNat names.size))) then (if (!(match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then entries[ml_index.toUInt64.toNat]? else none) with | some ml_present => ml_present | none => false)) then (source_ml_temporary_readRecordProjectionError_loop2_result.source_ml_temporary_readRecordProjectionError_loop2_return (#[99, 104, 101, 99, 107, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110, 32, 104, 97, 115, 32, 97, 32, 109, 97, 108, 102, 111, 114, 109, 101, 100, 32, 102, 105, 101, 108, 100, 32, 101, 110, 116, 114, 121] : Array Nat)) else (if (!(acceptRecordFieldName (readRequiredRecordName names index))) then (source_ml_temporary_readRecordProjectionError_loop2_result.source_ml_temporary_readRecordProjectionError_loop2_return (#[99, 104, 101, 99, 107, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110, 32, 104, 97, 115, 32, 97, 110, 32, 105, 110, 118, 97, 108, 105, 100, 32, 85, 84, 70, 49, 54, 32, 102, 105, 101, 108, 100, 32, 110, 97, 109, 101] : Array Nat)) else (if ((decide (index > (0 : Float))) && (decide ((compareRecordFieldNames (readRequiredRecordName names (index - (1 : Float))) (readRequiredRecordName names index)) ≥ (0 : Float)))) then (source_ml_temporary_readRecordProjectionError_loop2_result.source_ml_temporary_readRecordProjectionError_loop2_return (#[99, 104, 101, 99, 107, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 117, 110, 105, 113, 117, 101, 32, 102, 105, 101, 108, 100, 115, 32, 105, 110, 32, 85, 84, 70, 49, 54, 32, 111, 114, 100, 101, 114] : Array Nat)) else (let index_2 := (index + (1 : Float));
    (source_ml_temporary_readRecordProjectionError_loop2 names entries index_2))))) else source_ml_temporary_readRecordProjectionError_loop2_result.source_ml_temporary_readRecordProjectionError_loop2_done)

def readRecordProjectionError (projection : (Array Nat)) (names : (Array (Array Float))) (entries : (Array Bool)) : (Array Nat) :=
  (if ((MLUtf16Text.compare ((projection).map Float.ofNat) ((readRecordProjection).map Float.ofNat)) != (0 : Float)) then (#[117, 110, 107, 110, 111, 119, 110, 32, 99, 104, 101, 99, 107, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110] : Array Nat) else (if ((Float.ofNat names.size) != (Float.ofNat entries.size)) then (#[99, 104, 101, 99, 107, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110, 32, 104, 97, 115, 32, 105, 110, 99, 111, 110, 115, 105, 115, 116, 101, 110, 116, 32, 101, 110, 116, 114, 121, 32, 102, 97, 99, 116, 115] : Array Nat) else (let index := (0 : Float);
    (let source_ml_source1_s1 := (source_ml_temporary_readRecordProjectionError_loop2 names entries index);
      (match source_ml_source1_s1 with
        | source_ml_temporary_readRecordProjectionError_loop2_result.source_ml_temporary_readRecordProjectionError_loop2_done => (#[] : Array Nat)
        | source_ml_temporary_readRecordProjectionError_loop2_result.source_ml_temporary_readRecordProjectionError_loop2_return source_ml_temporary_result2 => source_ml_temporary_result2)))))

inductive source_ml_temporary_readRecordFieldPosition_loop6_result where
  | source_ml_temporary_readRecordFieldPosition_loop6_done : source_ml_temporary_readRecordFieldPosition_loop6_result
  | source_ml_temporary_readRecordFieldPosition_loop6_return (ml_temporary_result6 : Float) : source_ml_temporary_readRecordFieldPosition_loop6_result
  deriving Repr, Inhabited

partial def source_ml_temporary_readRecordFieldPosition_loop6 (fields : (Array (Array Float))) (name : (Array Float)) (index : Float) : source_ml_temporary_readRecordFieldPosition_loop6_result :=
  (if (decide (index < (Float.ofNat fields.size))) then (let comparison := (compareRecordFieldNames (readRequiredRecordName fields index) name);
    (if ((!(decide (comparison < (0 : Float)))) && (!(decide (comparison > (0 : Float))))) then (source_ml_temporary_readRecordFieldPosition_loop6_result.source_ml_temporary_readRecordFieldPosition_loop6_return index) else (let index_2 := (index + (1 : Float));
      (source_ml_temporary_readRecordFieldPosition_loop6 fields name index_2)))) else source_ml_temporary_readRecordFieldPosition_loop6_result.source_ml_temporary_readRecordFieldPosition_loop6_done)

inductive source_ml_temporary_readRecordFieldPosition_loop5_result where
  | source_ml_temporary_readRecordFieldPosition_loop5_done : source_ml_temporary_readRecordFieldPosition_loop5_result
  | source_ml_temporary_readRecordFieldPosition_loop5_return (ml_temporary_result6 : Float) : source_ml_temporary_readRecordFieldPosition_loop5_result
  deriving Repr, Inhabited

partial def source_ml_temporary_readRecordFieldPosition_loop5 (fields : (Array (Array Float))) (index : Float) : source_ml_temporary_readRecordFieldPosition_loop5_result :=
  (if (decide (index < (Float.ofNat fields.size))) then (if (!(acceptRecordFieldName (readRequiredRecordName fields index))) then (source_ml_temporary_readRecordFieldPosition_loop5_result.source_ml_temporary_readRecordFieldPosition_loop5_return (-1 : Float)) else (let index_2 := (index + (1 : Float));
    (source_ml_temporary_readRecordFieldPosition_loop5 fields index_2))) else source_ml_temporary_readRecordFieldPosition_loop5_result.source_ml_temporary_readRecordFieldPosition_loop5_done)

def readRecordFieldPosition (fields : (Array (Array Float))) (name : (Array Float)) : Float :=
  (if (!(acceptRecordFieldName name)) then (-1 : Float) else (let index := (0 : Float);
    (let source_ml_source1_s4 := (source_ml_temporary_readRecordFieldPosition_loop5 fields index);
      (match source_ml_source1_s4 with
        | source_ml_temporary_readRecordFieldPosition_loop5_result.source_ml_temporary_readRecordFieldPosition_loop5_done => (let index_2 := (0 : Float);
            (let source_ml_source1_s5 := (source_ml_temporary_readRecordFieldPosition_loop6 fields name index_2);
              (match source_ml_source1_s5 with
                | source_ml_temporary_readRecordFieldPosition_loop6_result.source_ml_temporary_readRecordFieldPosition_loop6_done => (-1 : Float)
                | source_ml_temporary_readRecordFieldPosition_loop6_result.source_ml_temporary_readRecordFieldPosition_loop6_return source_ml_temporary_result6 => source_ml_temporary_result6)))
        | source_ml_temporary_readRecordFieldPosition_loop5_result.source_ml_temporary_readRecordFieldPosition_loop5_return source_ml_temporary_result6_2 => source_ml_temporary_result6_2))))

def readRecordTypeConstraintStep (left : (Array (Array Float))) (right : (Array (Array Float))) (leftOpen : Bool) (rightOpen : Bool) (phase : Float) (index : Float) : (Array Float) :=
  (if (decide (phase < (1 : Float))) then (if (decide (index ≥ (Float.ofNat left.size))) then (#[(5 : Float), (-1 : Float), (-1 : Float), (1 : Float), (0 : Float)] : (Array Float)) else (let matching := (readRecordFieldPosition right (readRequiredRecordName left index));
    (if (decide (matching ≥ (0 : Float))) then (#[(0 : Float), index, matching, (0 : Float), (index + (1 : Float))] : (Array Float)) else (#[(if rightOpen then (1 : Float) else (3 : Float)), index, (-1 : Float), (0 : Float), (index + (1 : Float))] : (Array Float))))) else (if (decide (index ≥ (Float.ofNat right.size))) then (#[(6 : Float), (-1 : Float), (-1 : Float), (1 : Float), index] : (Array Float)) else (let matching_2 := (readRecordFieldPosition left (readRequiredRecordName right index));
    (if (decide (matching_2 ≥ (0 : Float))) then (#[(5 : Float), matching_2, index, (1 : Float), (index + (1 : Float))] : (Array Float)) else (#[(if leftOpen then (2 : Float) else (4 : Float)), (-1 : Float), index, (1 : Float), (index + (1 : Float))] : (Array Float))))))

def readRecordFieldInferenceStep (fields : (Array (Array Float))) (name : (Array Float)) («open» : Bool) : (Array Float) :=
  (let index := (readRecordFieldPosition fields name);
    (if (decide (index ≥ (0 : Float))) then (#[(0 : Float), index] : (Array Float)) else (#[(if «open» then (1 : Float) else (2 : Float)), (-1 : Float)] : (Array Float))))

partial def source_ml_temporary_readRecordSortedFieldPositions_loop10 (fields : (Array (Array Float))) (sorted : (Array Float)) (merged : (Array Float)) (middle : Float) («end» : Float) (left : Float) (right : Float) : (Array Float) :=
  (if ((decide (left < middle)) || (decide (right < «end»))) then (if ((decide (right ≥ «end»)) || ((decide (left < middle)) && (decide ((compareRecordFieldNames (readRequiredRecordName fields (readRequiredNumberFact sorted left)) (readRequiredRecordName fields (readRequiredNumberFact sorted right))) ≤ (0 : Float))))) then (let merged_2 := (merged ++ (#[(readRequiredNumberFact sorted left)] : (Array Float)));
    (let left_2 := (left + (1 : Float));
      (source_ml_temporary_readRecordSortedFieldPositions_loop10 fields sorted merged_2 middle «end» left_2 right))) else (let merged_3 := (merged ++ (#[(readRequiredNumberFact sorted right)] : (Array Float)));
    (let right_2 := (right + (1 : Float));
      (source_ml_temporary_readRecordSortedFieldPositions_loop10 fields sorted merged_3 middle «end» left right_2)))) else merged)

partial def source_ml_temporary_readRecordSortedFieldPositions_loop9 (fields : (Array (Array Float))) (sorted : (Array Float)) (width : Float) (merged : (Array Float)) (start : Float) : (Array Float) :=
  (if (decide (start < (Float.ofNat sorted.size))) then (let middle := (if (decide ((start + width) < (Float.ofNat sorted.size))) then (start + width) else (Float.ofNat sorted.size));
    (let «end» := (if (decide ((middle + width) < (Float.ofNat sorted.size))) then (middle + width) else (Float.ofNat sorted.size));
      (let left := start;
        (let right := middle;
          (let merged_2 := (source_ml_temporary_readRecordSortedFieldPositions_loop10 fields sorted merged middle «end» left right);
            (let start_2 := (start + (width * (2 : Float)));
              (source_ml_temporary_readRecordSortedFieldPositions_loop9 fields sorted width merged_2 start_2))))))) else merged)

partial def source_ml_temporary_readRecordSortedFieldPositions_loop8 (fields : (Array (Array Float))) (sorted : (Array Float)) (width : Float) : (Array Float) :=
  (if (decide (width < (Float.ofNat sorted.size))) then (let merged := (#[] : (Array Float));
    (let start := (0 : Float);
      (let merged_2 := (source_ml_temporary_readRecordSortedFieldPositions_loop9 fields sorted width merged start);
        (let sorted_2 := merged_2;
          (let width_2 := (width * (2 : Float));
            (source_ml_temporary_readRecordSortedFieldPositions_loop8 fields sorted_2 width_2)))))) else sorted)

inductive source_ml_temporary_readRecordSortedFieldPositions_loop7_result where
  | source_ml_temporary_readRecordSortedFieldPositions_loop7_done : source_ml_temporary_readRecordSortedFieldPositions_loop7_result
  | source_ml_temporary_readRecordSortedFieldPositions_loop7_return (ml_temporary_result10 : (Array Float)) : source_ml_temporary_readRecordSortedFieldPositions_loop7_result
  deriving Repr, Inhabited

partial def source_ml_temporary_readRecordSortedFieldPositions_loop7 (fields : (Array (Array Float))) (index : Float) : source_ml_temporary_readRecordSortedFieldPositions_loop7_result :=
  (if (decide (index < (Float.ofNat fields.size))) then (if (!(acceptRecordFieldName (readRequiredRecordName fields index))) then (source_ml_temporary_readRecordSortedFieldPositions_loop7_result.source_ml_temporary_readRecordSortedFieldPositions_loop7_return (#[(-1 : Float)] : (Array Float))) else (let index_2 := (index + (1 : Float));
    (source_ml_temporary_readRecordSortedFieldPositions_loop7 fields index_2))) else source_ml_temporary_readRecordSortedFieldPositions_loop7_result.source_ml_temporary_readRecordSortedFieldPositions_loop7_done)

def readRecordSortedFieldPositions (fields : (Array (Array Float))) : (Array Float) :=
  (let index := (0 : Float);
    (let source_ml_source1_s6 := (source_ml_temporary_readRecordSortedFieldPositions_loop7 fields index);
      (match source_ml_source1_s6 with
        | source_ml_temporary_readRecordSortedFieldPositions_loop7_result.source_ml_temporary_readRecordSortedFieldPositions_loop7_done => (let sorted := (readRecordValueOrder (Float.ofNat fields.size) false);
            (let width := (1 : Float);
              (let sorted_2 := (source_ml_temporary_readRecordSortedFieldPositions_loop8 fields sorted width);
                sorted_2)))
        | source_ml_temporary_readRecordSortedFieldPositions_loop7_result.source_ml_temporary_readRecordSortedFieldPositions_loop7_return source_ml_temporary_result10 => source_ml_temporary_result10)))

partial def source_ml_temporary_readRecordFieldOrder_loop11 (fields : (Array (Array Float))) (sorted : (Array Float)) (selected : (Array Float)) (index : Float) : (Array Float) :=
  (if (decide (index < (Float.ofNat sorted.size))) then (if ((decide ((index + (1 : Float)) ≥ (Float.ofNat sorted.size))) || (decide ((compareRecordFieldNames (readRequiredRecordName fields (readRequiredNumberFact sorted index)) (readRequiredRecordName fields (readRequiredNumberFact sorted (index + (1 : Float))))) < (0 : Float)))) then (let selected_2 := (selected ++ (#[(readRequiredNumberFact sorted index)] : (Array Float)));
    (let index_2 := (index + (1 : Float));
      (source_ml_temporary_readRecordFieldOrder_loop11 fields sorted selected_2 index_2))) else (let index_3 := (index + (1 : Float));
    (source_ml_temporary_readRecordFieldOrder_loop11 fields sorted selected index_3))) else selected)

def readRecordFieldOrder (fields : (Array (Array Float))) : (Array Float) :=
  (let sorted := (readRecordSortedFieldPositions fields);
    (if ((decide ((Float.ofNat sorted.size) > (0 : Float))) && (decide ((readRequiredNumberFact sorted (0 : Float)) < (0 : Float)))) then (#[(-1 : Float)] : (Array Float)) else (let selected := (#[] : (Array Float));
      (let index := (0 : Float);
        (let selected_2 := (source_ml_temporary_readRecordFieldOrder_loop11 fields sorted selected index);
          selected_2)))))

partial def source_ml_temporary_readRecordFieldInsertionOrder_loop14 (fields : (Array (Array Float))) (first : (Array Float)) (last : (Array Float)) (selected : (Array Float)) (index : Float) (lower : Float) (upper : Float) : (Array Float) :=
  (if (decide (lower < upper)) then (let middle := (Float.floor ((lower + upper) / (2 : Float)));
    (let comparison := (compareRecordFieldNames (readRequiredRecordName fields index) (readRequiredRecordName fields (readRequiredNumberFact last middle)));
      (if (decide (comparison < (0 : Float))) then (let upper_2 := middle;
        (source_ml_temporary_readRecordFieldInsertionOrder_loop14 fields first last selected index lower upper_2)) else (if (decide (comparison > (0 : Float))) then (let lower_2 := (middle + (1 : Float));
        (source_ml_temporary_readRecordFieldInsertionOrder_loop14 fields first last selected index lower_2 upper)) else (let selected_3 := (if ((!(decide ((readRequiredNumberFact first middle) < index))) && (!(decide ((readRequiredNumberFact first middle) > index)))) then (let selected_2 := (selected ++ (#[(readRequiredNumberFact last middle)] : (Array Float)));
        selected_2) else selected);
        (let lower_3 := upper;
          (source_ml_temporary_readRecordFieldInsertionOrder_loop14 fields first last selected_3 index lower_3 upper))))))) else selected)

partial def source_ml_temporary_readRecordFieldInsertionOrder_loop13 (fields : (Array (Array Float))) (first : (Array Float)) (last : (Array Float)) (selected : (Array Float)) (index : Float) : (Array Float) :=
  (if (decide (index < (Float.ofNat fields.size))) then (let lower := (0 : Float);
    (let upper := (Float.ofNat last.size);
      (let selected_2 := (source_ml_temporary_readRecordFieldInsertionOrder_loop14 fields first last selected index lower upper);
        (let index_2 := (index + (1 : Float));
          (source_ml_temporary_readRecordFieldInsertionOrder_loop13 fields first last selected_2 index_2))))) else selected)

inductive source_ml_temporary_readRecordFieldInsertionOrder_loop12_result where
  | source_ml_temporary_readRecordFieldInsertionOrder_loop12_done (first : (Array Float)) (last : (Array Float)) : source_ml_temporary_readRecordFieldInsertionOrder_loop12_result
  deriving Repr, Inhabited

partial def source_ml_temporary_readRecordFieldInsertionOrder_loop12 (fields : (Array (Array Float))) (sorted : (Array Float)) (first : (Array Float)) (last : (Array Float)) (start : Float) (index : Float) : source_ml_temporary_readRecordFieldInsertionOrder_loop12_result :=
  (if (decide (index < (Float.ofNat sorted.size))) then (if ((decide ((index + (1 : Float)) ≥ (Float.ofNat sorted.size))) || (decide ((compareRecordFieldNames (readRequiredRecordName fields (readRequiredNumberFact sorted index)) (readRequiredRecordName fields (readRequiredNumberFact sorted (index + (1 : Float))))) < (0 : Float)))) then (let first_2 := (first ++ (#[(readRequiredNumberFact sorted start)] : (Array Float)));
    (let last_2 := (last ++ (#[(readRequiredNumberFact sorted index)] : (Array Float)));
      (let start_2 := (index + (1 : Float));
        (let index_2 := (index + (1 : Float));
          (source_ml_temporary_readRecordFieldInsertionOrder_loop12 fields sorted first_2 last_2 start_2 index_2))))) else (let index_3 := (index + (1 : Float));
    (source_ml_temporary_readRecordFieldInsertionOrder_loop12 fields sorted first last start index_3))) else (source_ml_temporary_readRecordFieldInsertionOrder_loop12_result.source_ml_temporary_readRecordFieldInsertionOrder_loop12_done first last))

def readRecordFieldInsertionOrder (fields : (Array (Array Float))) : (Array Float) :=
  (let sorted := (readRecordSortedFieldPositions fields);
    (if ((decide ((Float.ofNat sorted.size) > (0 : Float))) && (decide ((readRequiredNumberFact sorted (0 : Float)) < (0 : Float)))) then (#[(-1 : Float)] : (Array Float)) else (let first := (#[] : (Array Float));
      (let last := (#[] : (Array Float));
        (let start := (0 : Float);
          (let index := (0 : Float);
            (let source_ml_source1_s7 := (source_ml_temporary_readRecordFieldInsertionOrder_loop12 fields sorted first last start index);
              (match source_ml_source1_s7 with
                | source_ml_temporary_readRecordFieldInsertionOrder_loop12_result.source_ml_temporary_readRecordFieldInsertionOrder_loop12_done first_2 last_2 => (let selected := (#[] : (Array Float));
                    (let index_2 := (0 : Float);
                      (let selected_2 := (source_ml_temporary_readRecordFieldInsertionOrder_loop13 fields first_2 last_2 selected index_2);
                        selected_2)))))))))))

inductive source_ml_temporary_recordTypeFactField_loop15_result where
  | source_ml_temporary_recordTypeFactField_loop15_done : source_ml_temporary_recordTypeFactField_loop15_result
  | source_ml_temporary_recordTypeFactField_loop15_return (ml_temporary_result15 : Float) : source_ml_temporary_recordTypeFactField_loop15_result
  deriving Repr, Inhabited

partial def source_ml_temporary_recordTypeFactField_loop15 (labels : (Array (Array Nat))) (targets : (Array Float)) (name : (Array Nat)) («end» : Float) (edge : Float) : source_ml_temporary_recordTypeFactField_loop15_result :=
  (if (decide (edge < «end»)) then (if ((MLUtf16Text.compare (((readRequiredTextFact labels edge)).map Float.ofNat) ((name).map Float.ofNat)) == (0 : Float)) then (source_ml_temporary_recordTypeFactField_loop15_result.source_ml_temporary_recordTypeFactField_loop15_return (readRequiredNumberFact targets edge)) else (let edge_2 := (edge + (1 : Float));
    (source_ml_temporary_recordTypeFactField_loop15 labels targets name «end» edge_2))) else source_ml_temporary_recordTypeFactField_loop15_result.source_ml_temporary_recordTypeFactField_loop15_done)

def recordTypeFactField (starts : (Array Float)) (counts : (Array Float)) (labels : (Array (Array Nat))) (targets : (Array Float)) (owner : Float) (name : (Array Nat)) : Float :=
  (if (decide (owner < (0 : Float))) then (-1 : Float) else (let start := (readRequiredNumberFact starts owner);
    (let «end» := (start + (readRequiredNumberFact counts owner));
      (let edge := start;
        (let source_ml_source1_s8 := (source_ml_temporary_recordTypeFactField_loop15 labels targets name «end» edge);
          (match source_ml_source1_s8 with
            | source_ml_temporary_recordTypeFactField_loop15_result.source_ml_temporary_recordTypeFactField_loop15_done => (-1 : Float)
            | source_ml_temporary_recordTypeFactField_loop15_result.source_ml_temporary_recordTypeFactField_loop15_return source_ml_temporary_result15 => source_ml_temporary_result15))))))

def recordTypeFactKind (texts : (Array (Array Nat))) (starts : (Array Float)) (counts : (Array Float)) (labels : (Array (Array Nat))) (targets : (Array Float)) (owner : Float) : (Array Nat) :=
  (let kind := (recordTypeFactField starts counts labels targets owner (#[107, 105, 110, 100] : Array Nat));
    (if (decide (kind < (0 : Float))) then (#[] : Array Nat) else (readRequiredTextFact texts kind)))

partial def source_ml_temporary_recordTypeFactElements_loop16 (targets : (Array Float)) (values : (Array Float)) («end» : Float) (edge : Float) : (Array Float) :=
  (if (decide (edge < «end»)) then (let values_2 := (values ++ (#[(readRequiredNumberFact targets edge)] : (Array Float)));
    (let edge_2 := (edge + (1 : Float));
      (source_ml_temporary_recordTypeFactElements_loop16 targets values_2 «end» edge_2))) else values)

def recordTypeFactElements (starts : (Array Float)) (counts : (Array Float)) (targets : (Array Float)) (owner : Float) : (Array Float) :=
  (let values := (#[] : (Array Float));
    (if (decide (owner < (0 : Float))) then values else (let start := (readRequiredNumberFact starts owner);
      (let «end» := (start + (readRequiredNumberFact counts owner));
        (let edge := start;
          (let values_2 := (source_ml_temporary_recordTypeFactElements_loop16 targets values «end» edge);
            values_2))))))

inductive source_ml_temporary_readRecordTypeOwnerPresence_loop17_result where
  | source_ml_temporary_readRecordTypeOwnerPresence_loop17_done : source_ml_temporary_readRecordTypeOwnerPresence_loop17_result
  | source_ml_temporary_readRecordTypeOwnerPresence_loop17_return (ml_temporary_result17 : Bool) : source_ml_temporary_readRecordTypeOwnerPresence_loop17_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_temporary_readRecordTypeOwnerPresence_loop19 (seen : (Array Float)) (node : Float) (visited : Bool) (index : Float) : Bool :=
  (if (decide (index < (Float.ofNat seen.size))) then (if ((readRequiredNumberFact seen index) == node) then (let visited_2 := true;
    (let index_2 := (index + (1 : Float));
      (source_ml_temporary_readRecordTypeOwnerPresence_loop19 seen node visited_2 index_2))) else (let index_3 := (index + (1 : Float));
    (source_ml_temporary_readRecordTypeOwnerPresence_loop19 seen node visited index_3))) else visited)

partial def source_ml_temporary_readRecordTypeOwnerPresence_loop20 (pending : (Array Float)) (children : (Array Float)) (index : Float) : (Array Float) :=
  (if (decide (index < (Float.ofNat children.size))) then (let pending_2 := (pending ++ (#[(readRequiredNumberFact children index)] : (Array Float)));
    (let index_2 := (index + (1 : Float));
      (source_ml_temporary_readRecordTypeOwnerPresence_loop20 pending_2 children index_2))) else pending)

partial def source_ml_temporary_readRecordTypeOwnerPresence_loop21 (starts : (Array Float)) (counts : (Array Float)) (labels : (Array (Array Nat))) (targets : (Array Float)) (pending : (Array Float)) (fields : (Array Float)) (index : Float) : (Array Float) :=
  (if (decide (index < (Float.ofNat fields.size))) then (let pending_2 := (pending ++ (#[(recordTypeFactField starts counts labels targets (readRequiredNumberFact fields index) (#[116, 121, 112, 101] : Array Nat))] : (Array Float)));
    (let index_2 := (index + (1 : Float));
      (source_ml_temporary_readRecordTypeOwnerPresence_loop21 starts counts labels targets pending_2 fields index_2))) else pending)

inductive source_ml_temporary_readRecordTypeOwnerPresence_loop18_result where
  | source_ml_temporary_readRecordTypeOwnerPresence_loop18_done (node : Float) (kind : (Array Nat)) : source_ml_temporary_readRecordTypeOwnerPresence_loop18_result
  deriving Repr, Inhabited

partial def source_ml_temporary_readRecordTypeOwnerPresence_loop18 (texts : (Array (Array Nat))) (numbers : (Array Float)) (starts : (Array Float)) (counts : (Array Float)) (labels : (Array (Array Nat))) (targets : (Array Float)) (bindings : (Array Float)) (node : Float) (kind : (Array Nat)) : source_ml_temporary_readRecordTypeOwnerPresence_loop18_result :=
  (if ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[118, 97, 114] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (let id := (recordTypeFactField starts counts labels targets node (#[105, 100] : Array Nat));
    (let «variable» := (readRequiredNumberFact numbers id);
      (if (((!(ml_is_integer «variable»)) || (decide («variable» < (0 : Float)))) || (decide («variable» ≥ (Float.ofNat bindings.size)))) then (source_ml_temporary_readRecordTypeOwnerPresence_loop18_result.source_ml_temporary_readRecordTypeOwnerPresence_loop18_done node kind) else (let bound := (readRequiredNumberFact bindings «variable»);
        (let boundKind := (recordTypeFactKind texts starts counts labels targets bound);
          (if ((MLUtf16Text.compare ((boundKind).map Float.ofNat) (((#[] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (source_ml_temporary_readRecordTypeOwnerPresence_loop18_result.source_ml_temporary_readRecordTypeOwnerPresence_loop18_done node kind) else (let node_2 := bound;
            (let kind_2 := boundKind;
              (source_ml_temporary_readRecordTypeOwnerPresence_loop18 texts numbers starts counts labels targets bindings node_2 kind_2))))))))) else (source_ml_temporary_readRecordTypeOwnerPresence_loop18_result.source_ml_temporary_readRecordTypeOwnerPresence_loop18_done node kind))

partial def source_ml_temporary_readRecordTypeOwnerPresence_loop17 (texts : (Array (Array Nat))) (numbers : (Array Float)) (starts : (Array Float)) (counts : (Array Float)) (labels : (Array (Array Nat))) (targets : (Array Float)) (bindings : (Array Float)) (owner : Float) (pending : (Array Float)) (seen : (Array Float)) (position : Float) : source_ml_temporary_readRecordTypeOwnerPresence_loop17_result :=
  (if (decide (position < (Float.ofNat pending.size))) then (let node := (readRequiredNumberFact pending position);
    (let kind := (recordTypeFactKind texts starts counts labels targets node);
      (let source_ml_source1_s22 := (source_ml_temporary_readRecordTypeOwnerPresence_loop18 texts numbers starts counts labels targets bindings node kind);
        (match source_ml_source1_s22 with
          | source_ml_temporary_readRecordTypeOwnerPresence_loop18_result.source_ml_temporary_readRecordTypeOwnerPresence_loop18_done node_2 kind_2 => (if (node_2 == owner) then (source_ml_temporary_readRecordTypeOwnerPresence_loop17_result.source_ml_temporary_readRecordTypeOwnerPresence_loop17_return true) else (let visited := false;
              (let index := (0 : Float);
                (let visited_2 := (source_ml_temporary_readRecordTypeOwnerPresence_loop19 seen node_2 visited index);
                  (if visited_2 then (let position_2 := (position + (1 : Float));
                    (source_ml_temporary_readRecordTypeOwnerPresence_loop17 texts numbers starts counts labels targets bindings owner pending seen position_2)) else (let seen_2 := (seen ++ (#[node_2] : (Array Float)));
                    (if (((((MLUtf16Text.compare ((kind_2).map Float.ofNat) (((#[97, 114, 114, 97, 121] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((kind_2).map Float.ofNat) (((#[111, 112, 116, 105, 111, 110] : Array Nat)).map Float.ofNat)) == (0 : Float))) || ((MLUtf16Text.compare ((kind_2).map Float.ofNat) (((#[115, 101, 116] : Array Nat)).map Float.ofNat)) == (0 : Float))) || ((MLUtf16Text.compare ((kind_2).map Float.ofNat) (((#[99, 101, 108, 108] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (let pending_2 := (pending ++ (#[(recordTypeFactField starts counts labels targets node_2 (#[101, 108, 101, 109, 101, 110, 116] : Array Nat))] : (Array Float)));
                      (let position_3 := (position + (1 : Float));
                        (source_ml_temporary_readRecordTypeOwnerPresence_loop17 texts numbers starts counts labels targets bindings owner pending_2 seen_2 position_3))) else (if ((MLUtf16Text.compare ((kind_2).map Float.ofNat) (((#[109, 97, 112] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (let pending_3 := (pending ++ (#[(recordTypeFactField starts counts labels targets node_2 (#[107, 101, 121] : Array Nat))] : (Array Float)));
                      (let pending_4 := (pending_3 ++ (#[(recordTypeFactField starts counts labels targets node_2 (#[118, 97, 108, 117, 101] : Array Nat))] : (Array Float)));
                        (let position_4 := (position + (1 : Float));
                          (source_ml_temporary_readRecordTypeOwnerPresence_loop17 texts numbers starts counts labels targets bindings owner pending_4 seen_2 position_4)))) else (if ((((MLUtf16Text.compare ((kind_2).map Float.ofNat) (((#[116, 117, 112, 108, 101] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((kind_2).map Float.ofNat) (((#[117, 110, 105, 111, 110] : Array Nat)).map Float.ofNat)) == (0 : Float))) || ((MLUtf16Text.compare ((kind_2).map Float.ofNat) (((#[102, 117, 110, 99, 116, 105, 111, 110] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (let field := (if ((MLUtf16Text.compare ((kind_2).map Float.ofNat) (((#[116, 117, 112, 108, 101] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[101, 108, 101, 109, 101, 110, 116, 115] : Array Nat) else (if ((MLUtf16Text.compare ((kind_2).map Float.ofNat) (((#[117, 110, 105, 111, 110] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[97, 108, 116, 101, 114, 110, 97, 116, 105, 118, 101, 115] : Array Nat) else (#[112, 97, 114, 97, 109, 115] : Array Nat)));
                      (let sequence := (recordTypeFactField starts counts labels targets node_2 field);
                        (let children := (recordTypeFactElements starts counts targets sequence);
                          (let index_2 := (0 : Float);
                            (let pending_5 := (source_ml_temporary_readRecordTypeOwnerPresence_loop20 pending children index_2);
                              (if ((MLUtf16Text.compare ((kind_2).map Float.ofNat) (((#[102, 117, 110, 99, 116, 105, 111, 110] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (let pending_6 := (pending_5 ++ (#[(recordTypeFactField starts counts labels targets node_2 (#[114, 101, 116] : Array Nat))] : (Array Float)));
                                (let position_5 := (position + (1 : Float));
                                  (source_ml_temporary_readRecordTypeOwnerPresence_loop17 texts numbers starts counts labels targets bindings owner pending_6 seen_2 position_5))) else (let position_6 := (position + (1 : Float));
                                (source_ml_temporary_readRecordTypeOwnerPresence_loop17 texts numbers starts counts labels targets bindings owner pending_5 seen_2 position_6)))))))) else (if ((MLUtf16Text.compare ((kind_2).map Float.ofNat) (((#[114, 101, 99, 111, 114, 100] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (let sequence_2 := (recordTypeFactField starts counts labels targets node_2 (#[102, 105, 101, 108, 100, 115] : Array Nat));
                      (let fields := (recordTypeFactElements starts counts targets sequence_2);
                        (let index_3 := (0 : Float);
                          (let pending_7 := (source_ml_temporary_readRecordTypeOwnerPresence_loop21 starts counts labels targets pending fields index_3);
                            (let position_7 := (position + (1 : Float));
                              (source_ml_temporary_readRecordTypeOwnerPresence_loop17 texts numbers starts counts labels targets bindings owner pending_7 seen_2 position_7)))))) else (let position_8 := (position + (1 : Float));
                      (source_ml_temporary_readRecordTypeOwnerPresence_loop17 texts numbers starts counts labels targets bindings owner pending seen_2 position_8)))))))))))))))) else source_ml_temporary_readRecordTypeOwnerPresence_loop17_result.source_ml_temporary_readRecordTypeOwnerPresence_loop17_done)

def readRecordTypeOwnerPresence (texts : (Array (Array Nat))) (numbers : (Array Float)) (starts : (Array Float)) (counts : (Array Float)) (labels : (Array (Array Nat))) (targets : (Array Float)) (bindings : (Array Float)) (root : Float) (owner : Float) : Bool :=
  (let pending := (#[root] : (Array Float));
    (let seen := (#[] : (Array Float));
      (let position := (0 : Float);
        (let source_ml_source1_s9 := (source_ml_temporary_readRecordTypeOwnerPresence_loop17 texts numbers starts counts labels targets bindings owner pending seen position);
          (match source_ml_source1_s9 with
            | source_ml_temporary_readRecordTypeOwnerPresence_loop17_result.source_ml_temporary_readRecordTypeOwnerPresence_loop17_done => false
            | source_ml_temporary_readRecordTypeOwnerPresence_loop17_result.source_ml_temporary_readRecordTypeOwnerPresence_loop17_return source_ml_temporary_result17 => source_ml_temporary_result17)))))

inductive source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_result where
  | source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_done (value : Float) : source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_result
  | source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_return (ml_temporary_result22 : Float) : source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_result
  deriving Repr, Inhabited

partial def source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22 (key : (Array Float)) (value : Float) («at» : Float) : source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_result :=
  (if (decide («at» < (Float.ofNat key.size))) then (let digit := ((match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then key[ml_index.toUInt64.toNat]? else none) with | some ml_present => ml_present | none => (0 / 0 : Float)) - (48 : Float));
    (if (((!(ml_is_integer digit)) || (decide (digit < (0 : Float)))) || (decide (digit > (9 : Float)))) then (source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_result.source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_return (-1 : Float)) else (let value_2 := ((value * (10 : Float)) + digit);
      (let at_2 := («at» + (1 : Float));
        (source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22 key value_2 at_2))))) else (source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_result.source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_done value))

def readJavaScriptLiteralPropertyIndex (key : (Array Float)) : Float :=
  (if (((Float.ofNat key.size) == (0 : Float)) || (decide ((Float.ofNat key.size) > (10 : Float)))) then (-1 : Float) else (if ((decide ((Float.ofNat key.size) > (1 : Float))) && (let source_ml_source1_comparison_operand10 := (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then key[ml_index.toUInt64.toNat]? else none);
    (let source_ml_source1_comparison_operand11 := (48 : Float);
      (match (source_ml_source1_comparison_operand10, source_ml_source1_comparison_operand11) with | (some ml_left, ml_right) => (ml_left == ml_right) | _ => false)))) then (-1 : Float) else (let value := (0 : Float);
    (let «at» := (0 : Float);
      (let source_ml_source1_s12 := (source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22 key value «at»);
        (match source_ml_source1_s12 with
          | source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_result.source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_done value_2 => (if (decide (value_2 ≤ (4294967294 : Float))) then value_2 else (-1 : Float))
          | source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_result.source_ml_temporary_readJavaScriptLiteralPropertyIndex_loop22_return source_ml_temporary_result22 => source_ml_temporary_result22))))))

inductive source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_result where
  | source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_done : source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_result
  | source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_return (ml_temporary_result23 : Bool) : source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23 (names : (Array (Array Float))) (index : Float) : source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_result :=
  (if (decide (index < (Float.ofNat names.size))) then (let name := (readJavaScriptRecordOrderName names index);
    (if (!(acceptRecordFieldName name)) then (source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_result.source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_return false) else (if ((decide (index > (0 : Float))) && (decide ((compareRecordFieldNames (readJavaScriptRecordOrderName names (index - (1 : Float))) name) ≥ (0 : Float)))) then (source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_result.source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_return false) else (let index_2 := (index + (1 : Float));
      (source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23 names index_2))))) else source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_result.source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_done)

def acceptJavaScriptRecordOrderNames (names : (Array (Array Float))) : Bool :=
  (let index := (0 : Float);
    (let source_ml_source1_s13 := (source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23 names index);
      (match source_ml_source1_s13 with
        | source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_result.source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_done => true
        | source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_result.source_ml_temporary_acceptJavaScriptRecordOrderNames_loop23_return source_ml_temporary_result23 => source_ml_temporary_result23)))

inductive source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_result where
  | source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_done : source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_result
  | source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_return (ml_temporary_result24 : Bool) : source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_result
  deriving Repr, DecidableEq, Inhabited

inductive source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_result where
  | source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_done : source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_result
  | source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_return (ml_temporary_result25 : Bool) : source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25 (order : (Array Float)) (index : Float) (value : Float) (previous : Float) : source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_result :=
  (if (decide (previous < index)) then (if ((readJavaScriptRecordOrderPosition order previous) == value) then (source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_result.source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_return false) else (let previous_2 := (previous + (1 : Float));
    (source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25 order index value previous_2))) else source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_result.source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_done)

partial def source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24 (count : Float) (order : (Array Float)) (index : Float) : source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_result :=
  (if (decide (index < (Float.ofNat order.size))) then (let value := (readJavaScriptRecordOrderPosition order index);
    (if ((((!(ml_is_integer value)) || (decide (value < (0 : Float)))) || (decide (value ≥ count))) || ((value == (0 : Float)) && (decide (((1 : Float) / value) < (0 : Float))))) then (source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_result.source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_return false) else (let previous := (0 : Float);
      (let source_ml_source1_s23 := (source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25 order index value previous);
        (match source_ml_source1_s23 with
          | source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_result.source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_done => (let index_2 := (index + (1 : Float));
              (source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24 count order index_2))
          | source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_result.source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop25_return source_ml_temporary_result25 => (source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_result.source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_return source_ml_temporary_result25)))))) else source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_result.source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_done)

def acceptJavaScriptRecordOrderPermutation (count : Float) (order : (Array Float)) : Bool :=
  (if ((((!(ml_is_safe_integer count)) || (decide (count < (0 : Float)))) || ((count == (0 : Float)) && (decide (((1 : Float) / count) < (0 : Float))))) || ((Float.ofNat order.size) != count)) then false else (let index := (0 : Float);
    (let source_ml_source1_s14 := (source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24 count order index);
      (match source_ml_source1_s14 with
        | source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_result.source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_done => true
        | source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_result.source_ml_temporary_acceptJavaScriptRecordOrderPermutation_loop24_return source_ml_temporary_result24 => source_ml_temporary_result24))))

def readJavaScriptRecordOrderCreationError (names : (Array (Array Float))) (insertion : (Array Float)) : (Array Nat) :=
  (if ((!(acceptJavaScriptRecordOrderNames names)) || (!(acceptJavaScriptRecordOrderPermutation (Float.ofNat names.size) insertion))) then (#[105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 115, 101, 113, 117, 101, 110, 99, 101] : Array Nat) else (#[] : Array Nat))

partial def source_ml_temporary_createJavaScriptRecordPropertyOrder_loop27 (names : (Array (Array Float))) (numeric : (Array Float)) (key : Float) (position : Float) : Float :=
  (if ((decide (position < (Float.ofNat numeric.size))) && (decide ((readJavaScriptLiteralPropertyIndex (readJavaScriptRecordOrderName names (readJavaScriptRecordOrderPosition numeric position))) < key))) then (let position_2 := (position + (1 : Float));
    (source_ml_temporary_createJavaScriptRecordPropertyOrder_loop27 names numeric key position_2)) else position)

partial def source_ml_temporary_createJavaScriptRecordPropertyOrder_loop26 (names : (Array (Array Float))) (numeric : (Array Float)) (field : Float) : (Array Float) :=
  (if (decide (field < (Float.ofNat names.size))) then (let key := (readJavaScriptLiteralPropertyIndex (readJavaScriptRecordOrderName names field));
    (if (decide (key < (0 : Float))) then (let field_2 := (field + (1 : Float));
      (source_ml_temporary_createJavaScriptRecordPropertyOrder_loop26 names numeric field_2)) else (let position := (0 : Float);
      (let position_2 := (source_ml_temporary_createJavaScriptRecordPropertyOrder_loop27 names numeric key position);
        (let numeric_2 := (let source_ml_source1_concat_value24 := (let ml_values := numeric; let ml_begin := (0 : Float); let ml_end := position_2; let ml_length := Float.ofNat ml_values.size; let ml_first := (MLUtf16Text.slicePosition ml_begin ml_length).toUInt64.toNat; let ml_last := (MLUtf16Text.slicePosition ml_end ml_length).toUInt64.toNat; ml_values.extract ml_first ml_last);
          (let source_ml_source1_concat_value25 := (#[field] : (Array Float));
            (let source_ml_source1_concat_value26 := (let ml_values := numeric; let ml_begin := position_2; let ml_end := (1.0 / 0.0 : Float); let ml_length := Float.ofNat ml_values.size; let ml_first := (MLUtf16Text.slicePosition ml_begin ml_length).toUInt64.toNat; let ml_last := (MLUtf16Text.slicePosition ml_end ml_length).toUInt64.toNat; ml_values.extract ml_first ml_last);
              ((((#[] : (Array Float)) ++ source_ml_source1_concat_value24) ++ source_ml_source1_concat_value25) ++ source_ml_source1_concat_value26))));
          (let field_3 := (field + (1 : Float));
            (source_ml_temporary_createJavaScriptRecordPropertyOrder_loop26 names numeric_2 field_3))))))) else numeric)

partial def source_ml_temporary_createJavaScriptRecordPropertyOrder_loop28 (names : (Array (Array Float))) (insertion : (Array Float)) (result : (Array Float)) (index : Float) : (Array Float) :=
  (if (decide (index < (Float.ofNat insertion.size))) then (let field := (readJavaScriptRecordOrderPosition insertion index);
    (if (decide ((readJavaScriptLiteralPropertyIndex (readJavaScriptRecordOrderName names field)) < (0 : Float))) then (let result_2 := (let source_ml_source1_concat_value27 := result;
      (let source_ml_source1_concat_value28 := (#[field] : (Array Float));
        (((#[] : (Array Float)) ++ source_ml_source1_concat_value27) ++ source_ml_source1_concat_value28)));
      (let index_2 := (index + (1 : Float));
        (source_ml_temporary_createJavaScriptRecordPropertyOrder_loop28 names insertion result_2 index_2))) else (let index_3 := (index + (1 : Float));
      (source_ml_temporary_createJavaScriptRecordPropertyOrder_loop28 names insertion result index_3)))) else result)

def createJavaScriptRecordPropertyOrder (names : (Array (Array Float))) (insertion : (Array Float)) : (Array Float) :=
  (let error := (readJavaScriptRecordOrderCreationError names insertion);
    (if (!(error).isEmpty) then (ml_raiseJavaScriptTypeError error) else (let numeric := (#[] : (Array Float));
      (let field := (0 : Float);
        (let numeric_2 := (source_ml_temporary_createJavaScriptRecordPropertyOrder_loop26 names numeric field);
          (let result := numeric_2;
            (let index := (0 : Float);
              (let result_2 := (source_ml_temporary_createJavaScriptRecordPropertyOrder_loop28 names insertion result index);
                result_2))))))))

inductive source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_result where
  | source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_done : source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_result
  | source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_return (ml_temporary_result29 : Bool) : source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29 (order : (Array Float)) (expected : (Array Float)) (index : Float) : source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_result :=
  (if (decide (index < (Float.ofNat order.size))) then (if ((readJavaScriptRecordOrderPosition order index) != (readJavaScriptRecordOrderPosition expected index)) then (source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_result.source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_return false) else (let index_2 := (index + (1 : Float));
    (source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29 order expected index_2))) else source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_result.source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_done)

def acceptJavaScriptRecordPropertyOrder (names : (Array (Array Float))) (order : (Array Float)) : Bool :=
  (if ((!(acceptJavaScriptRecordOrderNames names)) || (!(acceptJavaScriptRecordOrderPermutation (Float.ofNat names.size) order))) then false else (let expected := (createJavaScriptRecordPropertyOrder names order);
    (let index := (0 : Float);
      (let source_ml_source1_s15 := (source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29 order expected index);
        (match source_ml_source1_s15 with
          | source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_result.source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_done => true
          | source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_result.source_ml_temporary_acceptJavaScriptRecordPropertyOrder_loop29_return source_ml_temporary_result29 => source_ml_temporary_result29)))))

def readJavaScriptRecordOrderKeyError (names : (Array (Array Float))) (order : (Array Float)) : (Array Nat) :=
  (if (acceptJavaScriptRecordPropertyOrder names order) then (#[] : Array Nat) else (#[105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 111, 114, 100, 101, 114] : Array Nat))

inductive source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_result where
  | source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_done (insertion : (Array Float)) : source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_result
  | source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_return (ml_temporary_result30 : (Array Nat)) : source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_result
  deriving Repr, Inhabited

inductive source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_result where
  | source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_done (insertion : (Array Float)) : source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_result
  | source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_return (ml_temporary_result31 : (Array Nat)) : source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_result
  deriving Repr, Inhabited

partial def source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31 (names : (Array (Array Float))) (insertion : (Array Float)) (values : (Array Float)) (index : Float) : source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_result :=
  (if (decide (index < (Float.ofNat values.size))) then (let field := (readJavaScriptRecordOrderPosition values index);
    (if ((((!(ml_is_integer field)) || (decide (field < (0 : Float)))) || (decide (field ≥ (Float.ofNat names.size)))) || ((field == (0 : Float)) && (decide (((1 : Float) / field) < (0 : Float))))) then (source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_result.source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_return (#[105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 115, 111, 117, 114, 99, 101, 32, 102, 105, 101, 108, 100] : Array Nat)) else (if (!(let ml_values := insertion; let ml_search := field; let ml_start := (0 : Float); let ml_length := Float.ofNat ml_values.size; let ml_first := (MLUtf16Text.slicePosition ml_start ml_length).toUInt64.toNat; (ml_values.extract ml_first ml_values.size).any (fun ml_candidate => ((ml_candidate == ml_search) || ((ml_candidate).isNaN && (ml_search).isNaN))))) then (let insertion_2 := (let source_ml_source1_concat_value29 := insertion;
      (let source_ml_source1_concat_value30 := (#[field] : (Array Float));
        (((#[] : (Array Float)) ++ source_ml_source1_concat_value29) ++ source_ml_source1_concat_value30)));
      (let index_2 := (index + (1 : Float));
        (source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31 names insertion_2 values index_2))) else (let index_3 := (index + (1 : Float));
      (source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31 names insertion values index_3))))) else (source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_result.source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_done insertion))

partial def source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30 (names : (Array (Array Float))) (chunks : (Array (Array Float))) (insertion : (Array Float)) (chunk : Float) : source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_result :=
  (if (decide (chunk < (Float.ofNat chunks.size))) then (let values := (readJavaScriptRecordOrderName chunks chunk);
    (let index := (0 : Float);
      (let source_ml_source1_s31 := (source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31 names insertion values index);
        (match source_ml_source1_s31 with
          | source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_result.source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_done insertion_2 => (let chunk_2 := (chunk + (1 : Float));
              (source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30 names chunks insertion_2 chunk_2))
          | source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_result.source_ml_temporary_readJavaScriptRecordOrderMergeError_loop31_return source_ml_temporary_result31 => (source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_result.source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_return source_ml_temporary_result31))))) else (source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_result.source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_done insertion))

def readJavaScriptRecordOrderMergeError (names : (Array (Array Float))) (chunks : (Array (Array Float))) : (Array Nat) :=
  (if (!(acceptJavaScriptRecordOrderNames names)) then (#[105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 110, 97, 109, 101, 115] : Array Nat) else (let insertion := (#[] : (Array Float));
    (let chunk := (0 : Float);
      (let source_ml_source1_s16 := (source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30 names chunks insertion chunk);
        (match source_ml_source1_s16 with
          | source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_result.source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_done insertion_2 => (readJavaScriptRecordOrderCreationError names insertion_2)
          | source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_result.source_ml_temporary_readJavaScriptRecordOrderMergeError_loop30_return source_ml_temporary_result30 => source_ml_temporary_result30)))))

partial def source_ml_temporary_mergeJavaScriptRecordPropertyOrder_loop33 (insertion : (Array Float)) (values : (Array Float)) (index : Float) : (Array Float) :=
  (if (decide (index < (Float.ofNat values.size))) then (let field := (readJavaScriptRecordOrderPosition values index);
    (if (!(let ml_values := insertion; let ml_search := field; let ml_start := (0 : Float); let ml_length := Float.ofNat ml_values.size; let ml_first := (MLUtf16Text.slicePosition ml_start ml_length).toUInt64.toNat; (ml_values.extract ml_first ml_values.size).any (fun ml_candidate => ((ml_candidate == ml_search) || ((ml_candidate).isNaN && (ml_search).isNaN))))) then (let insertion_2 := (let source_ml_source1_concat_value32 := insertion;
      (let source_ml_source1_concat_value33 := (#[field] : (Array Float));
        (((#[] : (Array Float)) ++ source_ml_source1_concat_value32) ++ source_ml_source1_concat_value33)));
      (let index_2 := (index + (1 : Float));
        (source_ml_temporary_mergeJavaScriptRecordPropertyOrder_loop33 insertion_2 values index_2))) else (let index_3 := (index + (1 : Float));
      (source_ml_temporary_mergeJavaScriptRecordPropertyOrder_loop33 insertion values index_3)))) else insertion)

partial def source_ml_temporary_mergeJavaScriptRecordPropertyOrder_loop32 (chunks : (Array (Array Float))) (insertion : (Array Float)) (chunk : Float) : (Array Float) :=
  (if (decide (chunk < (Float.ofNat chunks.size))) then (let values := (readJavaScriptRecordOrderName chunks chunk);
    (let index := (0 : Float);
      (let insertion_2 := (source_ml_temporary_mergeJavaScriptRecordPropertyOrder_loop33 insertion values index);
        (let chunk_2 := (chunk + (1 : Float));
          (source_ml_temporary_mergeJavaScriptRecordPropertyOrder_loop32 chunks insertion_2 chunk_2))))) else insertion)

def mergeJavaScriptRecordPropertyOrder (names : (Array (Array Float))) (chunks : (Array (Array Float))) : (Array Float) :=
  (let error := (readJavaScriptRecordOrderMergeError names chunks);
    (if (!(error).isEmpty) then (ml_raiseJavaScriptTypeError error) else (let insertion := (#[] : (Array Float));
      (let chunk := (0 : Float);
        (let insertion_2 := (source_ml_temporary_mergeJavaScriptRecordPropertyOrder_loop32 chunks insertion chunk);
          (createJavaScriptRecordPropertyOrder names insertion_2))))))

inductive source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_result where
  | source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_done : source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_result
  | source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_return (ml_temporary_result34 : (Array Nat)) : source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34 (target : (Array (Array Float))) (source : (Array (Array Float))) (index : Float) : source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_result :=
  (if (decide (index < (Float.ofNat source.size))) then (if (decide ((readRecordFieldPosition target (readJavaScriptRecordOrderName source index)) < (0 : Float))) then (source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_result.source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_return (#[97, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 115, 112, 114, 101, 97, 100, 32, 102, 105, 101, 108, 100, 32, 105, 115, 32, 109, 105, 115, 115, 105, 110, 103, 32, 102, 114, 111, 109, 32, 105, 116, 115, 32, 116, 97, 114, 103, 101, 116] : Array Nat)) else (let index_2 := (index + (1 : Float));
    (source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34 target source index_2))) else source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_result.source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_done)

def readJavaScriptRecordOrderRebaseError (target : (Array (Array Float))) (source : (Array (Array Float))) (order : (Array Float)) : (Array Nat) :=
  (if ((!(acceptJavaScriptRecordOrderNames target)) || (!(acceptJavaScriptRecordPropertyOrder source order))) then (#[105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 115, 112, 114, 101, 97, 100, 32, 115, 111, 117, 114, 99, 101] : Array Nat) else (let index := (0 : Float);
    (let source_ml_source1_s17 := (source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34 target source index);
      (match source_ml_source1_s17 with
        | source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_result.source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_done => (#[] : Array Nat)
        | source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_result.source_ml_temporary_readJavaScriptRecordOrderRebaseError_loop34_return source_ml_temporary_result34 => source_ml_temporary_result34))))

partial def source_ml_temporary_rebaseJavaScriptRecordPropertyOrder_loop35 (target : (Array (Array Float))) (source : (Array (Array Float))) (order : (Array Float)) (result : (Array Float)) (index : Float) : (Array Float) :=
  (if (decide (index < (Float.ofNat order.size))) then (let name := (readJavaScriptRecordOrderName source (readJavaScriptRecordOrderPosition order index));
    (let field := (readRecordFieldPosition target name);
      (let result_2 := (let source_ml_source1_concat_value34 := result;
        (let source_ml_source1_concat_value35 := (#[field] : (Array Float));
          (((#[] : (Array Float)) ++ source_ml_source1_concat_value34) ++ source_ml_source1_concat_value35)));
        (let index_2 := (index + (1 : Float));
          (source_ml_temporary_rebaseJavaScriptRecordPropertyOrder_loop35 target source order result_2 index_2))))) else result)

def rebaseJavaScriptRecordPropertyOrder (target : (Array (Array Float))) (source : (Array (Array Float))) (order : (Array Float)) : (Array Float) :=
  (let error := (readJavaScriptRecordOrderRebaseError target source order);
    (if (!(error).isEmpty) then (ml_raiseJavaScriptTypeError error) else (let result := (#[] : (Array Float));
      (let index := (0 : Float);
        (let result_2 := (source_ml_temporary_rebaseJavaScriptRecordPropertyOrder_loop35 target source order result index);
          result_2)))))

partial def source_ml_temporary_readJavaScriptRecordPropertyKeys_loop36 (names : (Array (Array Float))) (order : (Array Float)) (result : (Array (Array Float))) (index : Float) : (Array (Array Float)) :=
  (if (decide (index < (Float.ofNat order.size))) then (let result_2 := (let source_ml_source1_concat_value36 := result;
    (let source_ml_source1_concat_value37 := (#[(readJavaScriptRecordOrderName names (readJavaScriptRecordOrderPosition order index))] : (Array (Array Float)));
      (((#[] : (Array (Array Float))) ++ source_ml_source1_concat_value36) ++ source_ml_source1_concat_value37)));
    (let index_2 := (index + (1 : Float));
      (source_ml_temporary_readJavaScriptRecordPropertyKeys_loop36 names order result_2 index_2))) else result)

def readJavaScriptRecordPropertyKeys (names : (Array (Array Float))) (order : (Array Float)) : (Array (Array Float)) :=
  (let error := (readJavaScriptRecordOrderKeyError names order);
    (if (!(error).isEmpty) then (ml_raiseJavaScriptTypeError error) else (let result := (#[] : (Array (Array Float)));
      (let index := (0 : Float);
        (let result_2 := (source_ml_temporary_readJavaScriptRecordPropertyKeys_loop36 names order result index);
          result_2)))))

partial def source_ml_temporary_readJavaScriptRecordPropertyKeyTexts_loop38 (units : (Array Float)) (key : (Array Nat)) («at» : Float) : (Array Nat) :=
  (if (decide («at» < (Float.ofNat units.size))) then (let key_2 := (key ++ (let source_ml_source1_character_argument38 := (readJavaScriptRecordOrderPosition units «at»);
    (((MLUtf16Text.characterUnits (#[source_ml_source1_character_argument38] : (Array Float)))).map (fun ml_unit => ml_unit.toUInt64.toNat))));
    (let at_2 := («at» + (1 : Float));
      (source_ml_temporary_readJavaScriptRecordPropertyKeyTexts_loop38 units key_2 at_2))) else key)

partial def source_ml_temporary_readJavaScriptRecordPropertyKeyTexts_loop37 (rows : (Array (Array Float))) (result : (Array (Array Nat))) (index : Float) : (Array (Array Nat)) :=
  (if (decide (index < (Float.ofNat rows.size))) then (let units := (readJavaScriptRecordOrderName rows index);
    (let key := (#[] : Array Nat);
      (let «at» := (0 : Float);
        (let key_2 := (source_ml_temporary_readJavaScriptRecordPropertyKeyTexts_loop38 units key «at»);
          (let result_2 := (let source_ml_source1_concat_value39 := result;
            (let source_ml_source1_concat_value40 := (#[key_2] : (Array (Array Nat)));
              (((#[] : (Array (Array Nat))) ++ source_ml_source1_concat_value39) ++ source_ml_source1_concat_value40)));
            (let index_2 := (index + (1 : Float));
              (source_ml_temporary_readJavaScriptRecordPropertyKeyTexts_loop37 rows result_2 index_2))))))) else result)

def readJavaScriptRecordPropertyKeyTexts (names : (Array (Array Float))) (order : (Array Float)) : (Array (Array Nat)) :=
  (let rows := (readJavaScriptRecordPropertyKeys names order);
    (let result := (#[] : (Array (Array Nat)));
      (let index := (0 : Float);
        (let result_2 := (source_ml_temporary_readJavaScriptRecordPropertyKeyTexts_loop37 rows result index);
          result_2))))

partial def source_ml_temporary_readJavaScriptRecordPropertyIndex_loop39 (key : (Array Nat)) (units : (Array Float)) (index : Float) : (Array Float) :=
  (if (decide (index < (Float.ofNat (key).size))) then (let units_2 := (let source_ml_source1_concat_value50 := units;
    (let source_ml_source1_concat_value51 := (#[(let source_ml_source1_string_unit_operand41 := key;
      (let source_ml_source1_string_unit_operand42 := index;
        ((fun (text : (Array Nat)) (position : Float) => (let index_2 := (if (Float.isNaN position) then (0 : Float) else (ml_trunc position));
          (if ((decide (index_2 < (0 : Float))) || (decide (index_2 ≥ (Float.ofNat (text).size)))) then (0.0 / 0.0 : Float) else (match (let source_ml_source1_string_unit_operand48 := (let source_ml_source1_string_range_operand43 := text;
            (let source_ml_source1_string_range_operand44 := index_2;
              (let source_ml_source1_string_range_operand45 := (index_2 + (1 : Float));
                (let source_ml_source1_string_range_position46 := source_ml_source1_string_range_operand44;
                  (let source_ml_source1_string_range_position47 := source_ml_source1_string_range_operand45;
                    (((MLUtf16Text.slice ((source_ml_source1_string_range_operand43).map Float.ofNat) source_ml_source1_string_range_position46 source_ml_source1_string_range_position47)).map (fun ml_unit => ml_unit.toUInt64.toNat)))))));
            (let source_ml_source1_string_unit_operand49 := (0 : Float);
              ((MLUtf16Text.codePoint ((source_ml_source1_string_unit_operand48).map Float.ofNat) source_ml_source1_string_unit_operand49))[0]?)) with | some ml_value => ml_value | _ => (0.0 / 0.0 : Float))))) source_ml_source1_string_unit_operand41 source_ml_source1_string_unit_operand42)))] : (Array Float));
      (((#[] : (Array Float)) ++ source_ml_source1_concat_value50) ++ source_ml_source1_concat_value51)));
    (let index_2 := (index + (1 : Float));
      (source_ml_temporary_readJavaScriptRecordPropertyIndex_loop39 key units_2 index_2))) else units)

def readJavaScriptRecordPropertyIndex (names : (Array (Array Float))) (key : (Array Nat)) : Float :=
  (let units := (#[] : (Array Float));
    (let index := (0 : Float);
      (let units_2 := (source_ml_temporary_readJavaScriptRecordPropertyIndex_loop39 key units index);
        (readRecordFieldPosition names units_2))))

partial def source_ml_temporary_createJavaScriptDensePropertyKeyTexts_loop40 (length : Float) (result : (Array (Array Nat))) (index : Float) : (Array (Array Nat)) :=
  (if (decide (index < length)) then (let result_2 := (let source_ml_source1_concat_value53 := result;
    (let source_ml_source1_concat_value54 := (#[(let source_ml_source1_string_argument52 := index;
      (String.foldl (fun (ml_units : Array Nat) ml_character => if ml_character.toNat < 65536 then ml_units.push ml_character.toNat else (ml_units.push (55296 + (ml_character.toNat - 65536) / 1024)).push (56320 + (ml_character.toNat - 65536) % 1024)) #[] (ml_js_number source_ml_source1_string_argument52)))] : (Array (Array Nat)));
      (((#[] : (Array (Array Nat))) ++ source_ml_source1_concat_value53) ++ source_ml_source1_concat_value54)));
    (let index_2 := (index + (1 : Float));
      (source_ml_temporary_createJavaScriptDensePropertyKeyTexts_loop40 length result_2 index_2))) else result)

def createJavaScriptDensePropertyKeyTexts (length : Float) : (Array (Array Nat)) :=
  (if (((!(ml_is_safe_integer length)) || (decide (length < (0 : Float)))) || ((length == (0 : Float)) && (decide (((1 : Float) / length) < (0 : Float))))) then (ml_raiseJavaScriptTypeError (#[105, 110, 118, 97, 108, 105, 100, 32, 100, 101, 110, 115, 101, 32, 111, 119, 110, 45, 102, 105, 101, 108, 100, 32, 108, 101, 110, 103, 116, 104] : Array Nat)) else (let result := (#[] : (Array (Array Nat)));
    (let index := (0 : Float);
      (let result_2 := (source_ml_temporary_createJavaScriptDensePropertyKeyTexts_loop40 length result index);
        result_2))))

partial def source_ml_temporary_createJavaScriptStringOwnPropertyValues_loop41 (text : (Array Nat)) (result : (Array (Array Nat))) (index : Float) : (Array (Array Nat)) :=
  (if (decide (index < (Float.ofNat (text).size))) then (let result_2 := (let source_ml_source1_concat_value62 := result;
    (let source_ml_source1_concat_value63 := (#[(let source_ml_source1_string_unit_operand55 := text;
      (let source_ml_source1_string_unit_operand56 := index;
        ((fun (text_2 : (Array Nat)) (position : Float) => (let index_2 := (if (Float.isNaN position) then (0 : Float) else (ml_trunc position));
          (if ((decide (index_2 < (0 : Float))) || (decide (index_2 ≥ (Float.ofNat (text_2).size)))) then (#[] : Array Nat) else (let source_ml_source1_string_range_operand57 := text_2;
            (let source_ml_source1_string_range_operand58 := index_2;
              (let source_ml_source1_string_range_operand59 := (index_2 + (1 : Float));
                (let source_ml_source1_string_range_position60 := source_ml_source1_string_range_operand58;
                  (let source_ml_source1_string_range_position61 := source_ml_source1_string_range_operand59;
                    (((MLUtf16Text.slice ((source_ml_source1_string_range_operand57).map Float.ofNat) source_ml_source1_string_range_position60 source_ml_source1_string_range_position61)).map (fun ml_unit => ml_unit.toUInt64.toNat)))))))))) source_ml_source1_string_unit_operand55 source_ml_source1_string_unit_operand56)))] : (Array (Array Nat)));
      (((#[] : (Array (Array Nat))) ++ source_ml_source1_concat_value62) ++ source_ml_source1_concat_value63)));
    (let index_2 := (index + (1 : Float));
      (source_ml_temporary_createJavaScriptStringOwnPropertyValues_loop41 text result_2 index_2))) else result)

def createJavaScriptStringOwnPropertyValues (text : (Array Nat)) : (Array (Array Nat)) :=
  (let result := (#[] : (Array (Array Nat)));
    (let index := (0 : Float);
      (let result_2 := (source_ml_temporary_createJavaScriptStringOwnPropertyValues_loop41 text result index);
        result_2)))

def main : IO Unit := do
  pure ()

def create := createJavaScriptRecordPropertyOrder
def merge := mergeJavaScriptRecordPropertyOrder
def rebase := rebaseJavaScriptRecordPropertyOrder
def keys := readJavaScriptRecordPropertyKeys
def valid := acceptJavaScriptRecordPropertyOrder
end MLRecordPropertyOrder
