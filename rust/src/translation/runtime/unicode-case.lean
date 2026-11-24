namespace MLUnicodeCase
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
          (let source_ml_source1_s8 := (if ((decide (scalar ≥ (194 : Float))) && (decide (scalar ≤ (223 : Float)))) then (let scalar_2 := (scalar - (192 : Float));
            (let remaining_2 := (1 : Float);
              (let minimum_2 := (128 : Float);
                (source_ml_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_2 remaining_2 minimum_2)))) else (if ((decide (scalar ≥ (224 : Float))) && (decide (scalar ≤ (239 : Float)))) then (let scalar_3 := (scalar - (224 : Float));
            (let remaining_3 := (2 : Float);
              (let minimum_3 := (2048 : Float);
                (source_ml_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_3 remaining_3 minimum_3)))) else (if ((decide (scalar ≥ (240 : Float))) && (decide (scalar ≤ (244 : Float)))) then (let scalar_4 := (scalar - (240 : Float));
            (let remaining_4 := (3 : Float);
              (let minimum_4 := (65536 : Float);
                (source_ml_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_4 remaining_4 minimum_4)))) else (if (decide (scalar ≥ (128 : Float))) then (source_ml_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_temporary_readJavaScriptUtf8StringUnits_join3_return (#[(-1 : Float)] : (Array Float))) else (source_ml_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_temporary_readJavaScriptUtf8StringUnits_join3_next scalar remaining minimum)))));
            (match source_ml_source1_s8 with
              | source_ml_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_5 remaining_5 minimum_5 => (let source_ml_source1_s9 := (source_ml_temporary_readJavaScriptUtf8StringUnits_loop4 bytes index_2 scalar_5 remaining_5);
                  (match source_ml_source1_s9 with
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

inductive source_ml_temporary_matchUtf16Text_loop7_result where
  | source_ml_temporary_matchUtf16Text_loop7_done : source_ml_temporary_matchUtf16Text_loop7_result
  | source_ml_temporary_matchUtf16Text_loop7_return (ml_temporary_result7 : Bool) : source_ml_temporary_matchUtf16Text_loop7_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_temporary_matchUtf16Text_loop7 (text : (Array Float)) (search : (Array Float)) (start : Float) («at» : Float) : source_ml_temporary_matchUtf16Text_loop7_result :=
  (if (decide («at» < (Float.ofNat search.size))) then (if (!(match ((let ml_index := (start + «at»); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none), (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then search[ml_index.toUInt64.toNat]? else none)) with | (some ml_left, some ml_right) => (ml_left == ml_right) | (none, none) => true | _ => false)) then (source_ml_temporary_matchUtf16Text_loop7_result.source_ml_temporary_matchUtf16Text_loop7_return false) else (let at_2 := («at» + (1 : Float));
    (source_ml_temporary_matchUtf16Text_loop7 text search start at_2))) else source_ml_temporary_matchUtf16Text_loop7_result.source_ml_temporary_matchUtf16Text_loop7_done)

def matchUtf16Text (text : (Array Float)) (search : (Array Float)) (start : Float) : Bool :=
  (if ((decide (start < (0 : Float))) || (decide ((start + (Float.ofNat search.size)) > (Float.ofNat text.size)))) then false else (let «at» := (0 : Float);
    (let source_ml_source1_s3 := (source_ml_temporary_matchUtf16Text_loop7 text search start «at»);
      (match source_ml_source1_s3 with
        | source_ml_temporary_matchUtf16Text_loop7_result.source_ml_temporary_matchUtf16Text_loop7_done => true
        | source_ml_temporary_matchUtf16Text_loop7_result.source_ml_temporary_matchUtf16Text_loop7_return source_ml_temporary_result7 => source_ml_temporary_result7))))

def startsUtf16Text (text : (Array Float)) (search : (Array Float)) : Bool :=
  (matchUtf16Text text search (0 : Float))

def endsUtf16Text (text : (Array Float)) (search : (Array Float)) : Bool :=
  (matchUtf16Text text search ((Float.ofNat text.size) - (Float.ofNat search.size)))

inductive source_ml_temporary_indexUtf16Text_loop8_result where
  | source_ml_temporary_indexUtf16Text_loop8_done : source_ml_temporary_indexUtf16Text_loop8_result
  | source_ml_temporary_indexUtf16Text_loop8_return (ml_temporary_result8 : Float) : source_ml_temporary_indexUtf16Text_loop8_result
  deriving Repr, Inhabited

partial def source_ml_temporary_indexUtf16Text_loop8 (text : (Array Float)) (search : (Array Float)) («at» : Float) : source_ml_temporary_indexUtf16Text_loop8_result :=
  (if (decide («at» ≤ ((Float.ofNat text.size) - (Float.ofNat search.size)))) then (if (matchUtf16Text text search «at») then (source_ml_temporary_indexUtf16Text_loop8_result.source_ml_temporary_indexUtf16Text_loop8_return «at») else (let at_2 := («at» + (1 : Float));
    (source_ml_temporary_indexUtf16Text_loop8 text search at_2))) else source_ml_temporary_indexUtf16Text_loop8_result.source_ml_temporary_indexUtf16Text_loop8_done)

def indexUtf16Text (text : (Array Float)) (search : (Array Float)) (position : Float) : Float :=
  (let start := (ml_min (ml_max (readUtf16TextInteger position) (0 : Float)) (Float.ofNat text.size));
    (let «at» := start;
      (let source_ml_source1_s4 := (source_ml_temporary_indexUtf16Text_loop8 text search «at»);
        (match source_ml_source1_s4 with
          | source_ml_temporary_indexUtf16Text_loop8_result.source_ml_temporary_indexUtf16Text_loop8_done => (-1 : Float)
          | source_ml_temporary_indexUtf16Text_loop8_result.source_ml_temporary_indexUtf16Text_loop8_return source_ml_temporary_result8 => source_ml_temporary_result8))))

def includesUtf16Text (text : (Array Float)) (search : (Array Float)) : Bool :=
  (decide ((indexUtf16Text text search (0 : Float)) ≥ (0 : Float)))

inductive source_ml_temporary_lastIndexUtf16Text_loop9_result where
  | source_ml_temporary_lastIndexUtf16Text_loop9_done : source_ml_temporary_lastIndexUtf16Text_loop9_result
  | source_ml_temporary_lastIndexUtf16Text_loop9_return (ml_temporary_result9 : Float) : source_ml_temporary_lastIndexUtf16Text_loop9_result
  deriving Repr, Inhabited

partial def source_ml_temporary_lastIndexUtf16Text_loop9 (text : (Array Float)) (search : (Array Float)) («at» : Float) : source_ml_temporary_lastIndexUtf16Text_loop9_result :=
  (if (decide («at» ≥ (0 : Float))) then (if (matchUtf16Text text search «at») then (source_ml_temporary_lastIndexUtf16Text_loop9_result.source_ml_temporary_lastIndexUtf16Text_loop9_return «at») else (let at_2 := («at» - (1 : Float));
    (source_ml_temporary_lastIndexUtf16Text_loop9 text search at_2))) else source_ml_temporary_lastIndexUtf16Text_loop9_result.source_ml_temporary_lastIndexUtf16Text_loop9_done)

def lastIndexUtf16Text (text : (Array Float)) (search : (Array Float)) (position : Float) : Float :=
  (let integer := (if (Float.isNaN position) then (1.0 / 0.0 : Float) else (readUtf16TextInteger position));
    (let start := (ml_min (ml_max integer (0 : Float)) ((Float.ofNat text.size) - (Float.ofNat search.size)));
      (let «at» := start;
        (let source_ml_source1_s5 := (source_ml_temporary_lastIndexUtf16Text_loop9 text search «at»);
          (match source_ml_source1_s5 with
            | source_ml_temporary_lastIndexUtf16Text_loop9_result.source_ml_temporary_lastIndexUtf16Text_loop9_done => (-1 : Float)
            | source_ml_temporary_lastIndexUtf16Text_loop9_result.source_ml_temporary_lastIndexUtf16Text_loop9_return source_ml_temporary_result9 => source_ml_temporary_result9)))))

inductive source_ml_temporary_compareUtf16Text_loop10_result where
  | source_ml_temporary_compareUtf16Text_loop10_done : source_ml_temporary_compareUtf16Text_loop10_result
  | source_ml_temporary_compareUtf16Text_loop10_return (ml_temporary_result10 : Float) : source_ml_temporary_compareUtf16Text_loop10_result
  deriving Repr, Inhabited

partial def source_ml_temporary_compareUtf16Text_loop10 (left : (Array Float)) (right : (Array Float)) (count : Float) («at» : Float) : source_ml_temporary_compareUtf16Text_loop10_result :=
  (if (decide («at» < count)) then (let a := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then left[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float));
    (let b := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then right[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float));
      (if (a != b) then (source_ml_temporary_compareUtf16Text_loop10_result.source_ml_temporary_compareUtf16Text_loop10_return (if (decide (a < b)) then (-1 : Float) else (1 : Float))) else (let at_2 := («at» + (1 : Float));
        (source_ml_temporary_compareUtf16Text_loop10 left right count at_2))))) else source_ml_temporary_compareUtf16Text_loop10_result.source_ml_temporary_compareUtf16Text_loop10_done)

def compareUtf16Text (left : (Array Float)) (right : (Array Float)) : Float :=
  (let count := (ml_min (Float.ofNat left.size) (Float.ofNat right.size));
    (let «at» := (0 : Float);
      (let source_ml_source1_s6 := (source_ml_temporary_compareUtf16Text_loop10 left right count «at»);
        (match source_ml_source1_s6 with
          | source_ml_temporary_compareUtf16Text_loop10_result.source_ml_temporary_compareUtf16Text_loop10_done => (if ((Float.ofNat left.size) == (Float.ofNat right.size)) then (0 : Float) else (if (decide ((Float.ofNat left.size) < (Float.ofNat right.size))) then (-1 : Float) else (1 : Float)))
          | source_ml_temporary_compareUtf16Text_loop10_result.source_ml_temporary_compareUtf16Text_loop10_return source_ml_temporary_result10 => source_ml_temporary_result10))))

partial def source_ml_temporary_joinUtf16Text_loop12 (separator : (Array Float)) (result : (Array Float)) (offset : Float) : (Array Float) :=
  (if (decide (offset < (Float.ofNat separator.size))) then (let result_2 := (result ++ (#[(match (let ml_index := offset; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then separator[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float))] : (Array Float)));
    (let offset_2 := (offset + (1 : Float));
      (source_ml_temporary_joinUtf16Text_loop12 separator result_2 offset_2))) else result)

partial def source_ml_temporary_joinUtf16Text_loop13 (result : (Array Float)) (value : (Array Float)) (offset : Float) : (Array Float) :=
  (if (decide (offset < (Float.ofNat value.size))) then (let result_2 := (result ++ (#[(match (let ml_index := offset; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then value[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float))] : (Array Float)));
    (let offset_2 := (offset + (1 : Float));
      (source_ml_temporary_joinUtf16Text_loop13 result_2 value offset_2))) else result)

partial def source_ml_temporary_joinUtf16Text_loop11 (values : (Array (Array Float))) (separator : (Array Float)) (result : (Array Float)) («at» : Float) : (Array Float) :=
  (if (decide («at» < (Float.ofNat values.size))) then (let result_3 := (if («at» != (0 : Float)) then (let offset := (0 : Float);
    (let result_2 := (source_ml_temporary_joinUtf16Text_loop12 separator result offset);
      result_2)) else result);
    (let value := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then values[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (#[] : (Array Float)));
      (let offset_2 := (0 : Float);
        (let result_4 := (source_ml_temporary_joinUtf16Text_loop13 result_3 value offset_2);
          (let at_2 := («at» + (1 : Float));
            (source_ml_temporary_joinUtf16Text_loop11 values separator result_4 at_2)))))) else result)

def joinUtf16Text (values : (Array (Array Float))) (separator : (Array Float)) : (Array Float) :=
  (let result := (#[] : (Array Float));
    (let «at» := (0 : Float);
      (let result_2 := (source_ml_temporary_joinUtf16Text_loop11 values separator result «at»);
        result_2)))

partial def source_ml_temporary_trimUtf16Text_loop14 (text : (Array Float)) (first : Float) (last : Float) : Float :=
  (if ((decide (first < last)) && (acceptJavaScriptWhitespaceUnit (match (let ml_index := first; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)))) then (let first_2 := (first + (1 : Float));
    (source_ml_temporary_trimUtf16Text_loop14 text first_2 last)) else first)

partial def source_ml_temporary_trimUtf16Text_loop15 (text : (Array Float)) (first : Float) (last : Float) : Float :=
  (if ((decide (last > first)) && (acceptJavaScriptWhitespaceUnit (match (let ml_index := (last - (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)))) then (let last_2 := (last - (1 : Float));
    (source_ml_temporary_trimUtf16Text_loop15 text first last_2)) else last)

def trimUtf16Text (text : (Array Float)) (start : Bool) («end» : Bool) : (Array Float) :=
  (let first := (0 : Float);
    (let last := (Float.ofNat text.size);
      (let first_3 := (if start then (let first_2 := (source_ml_temporary_trimUtf16Text_loop14 text first last);
        first_2) else first);
        (let last_3 := (if «end» then (let last_2 := (source_ml_temporary_trimUtf16Text_loop15 text first_3 last);
          last_2) else last);
          (sliceUtf16Text text first_3 last_3)))))

partial def source_ml_temporary_characterUnitsUtf16Text_loop16 (values : (Array Float)) (result : (Array Float)) («at» : Float) : (Array Float) :=
  (if (decide («at» < (Float.ofNat values.size))) then (let value := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then values[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float));
    (let integer := (if (Float.isFinite value) then (ml_trunc value) else (0 : Float));
      (let residue := (ml_float_rem integer (65536 : Float));
        (let result_2 := (result ++ (#[(if (decide (residue < (0 : Float))) then (residue + (65536 : Float)) else (if (residue == (0 : Float)) then (0 : Float) else residue))] : (Array Float)));
          (let at_2 := («at» + (1 : Float));
            (source_ml_temporary_characterUnitsUtf16Text_loop16 values result_2 at_2)))))) else result)

def characterUnitsUtf16Text (values : (Array Float)) : (Array Float) :=
  (let result := (#[] : (Array Float));
    (let «at» := (0 : Float);
      (let result_2 := (source_ml_temporary_characterUnitsUtf16Text_loop16 values result «at»);
        result_2)))

inductive source_ml_temporary_codePointUnitsUtf16Text_loop17_result where
  | source_ml_temporary_codePointUnitsUtf16Text_loop17_done (result : (Array Float)) : source_ml_temporary_codePointUnitsUtf16Text_loop17_result
  | source_ml_temporary_codePointUnitsUtf16Text_loop17_return (ml_temporary_result17 : (Array Float)) : source_ml_temporary_codePointUnitsUtf16Text_loop17_result
  deriving Repr, Inhabited

partial def source_ml_temporary_codePointUnitsUtf16Text_loop17 (values : (Array Float)) (result : (Array Float)) («at» : Float) : source_ml_temporary_codePointUnitsUtf16Text_loop17_result :=
  (if (decide («at» < (Float.ofNat values.size))) then (let value := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then values[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if (((!(ml_is_integer value)) || (decide (value < (0 : Float)))) || (decide (value > (1114111 : Float)))) then (source_ml_temporary_codePointUnitsUtf16Text_loop17_result.source_ml_temporary_codePointUnitsUtf16Text_loop17_return (#[«at»] : (Array Float))) else (if (decide (value < (65536 : Float))) then (let result_2 := (result ++ (#[(if (value == (0 : Float)) then (0 : Float) else value)] : (Array Float)));
      (let at_2 := («at» + (1 : Float));
        (source_ml_temporary_codePointUnitsUtf16Text_loop17 values result_2 at_2))) else (let result_3 := (result ++ (#[((55296 : Float) + (Float.floor ((value - (65536 : Float)) / (1024 : Float)))), ((56320 : Float) + (ml_float_rem (value - (65536 : Float)) (1024 : Float)))] : (Array Float)));
      (let at_3 := («at» + (1 : Float));
        (source_ml_temporary_codePointUnitsUtf16Text_loop17 values result_3 at_3)))))) else (source_ml_temporary_codePointUnitsUtf16Text_loop17_result.source_ml_temporary_codePointUnitsUtf16Text_loop17_done result))

def codePointUnitsUtf16Text (values : (Array Float)) : (Array Float) :=
  (let result := (#[(-1 : Float)] : (Array Float));
    (let «at» := (0 : Float);
      (let source_ml_source1_s7 := (source_ml_temporary_codePointUnitsUtf16Text_loop17 values result «at»);
        (match source_ml_source1_s7 with
          | source_ml_temporary_codePointUnitsUtf16Text_loop17_result.source_ml_temporary_codePointUnitsUtf16Text_loop17_done result_2 => result_2
          | source_ml_temporary_codePointUnitsUtf16Text_loop17_result.source_ml_temporary_codePointUnitsUtf16Text_loop17_return source_ml_temporary_result17 => source_ml_temporary_result17))))

inductive source_ml_temporary_displayPointsUtf16Text_join19 where
  | source_ml_temporary_displayPointsUtf16Text_join19_next («at» : Float) (point : Float) : source_ml_temporary_displayPointsUtf16Text_join19
  deriving Repr, Inhabited

partial def source_ml_temporary_displayPointsUtf16Text_loop18 (text : (Array Float)) (result : (Array Float)) («at» : Float) : (Array Float) :=
  (if (decide («at» < (Float.ofNat text.size))) then (let point := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (65533 : Float));
    (let source_ml_source1_s10 := (if ((decide (point ≥ (55296 : Float))) && (decide (point ≤ (56319 : Float)))) then (let low := (match (let ml_index := («at» + (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
      (if ((decide (low ≥ (56320 : Float))) && (decide (low ≤ (57343 : Float)))) then (let point_2 := ((((65536 : Float) + ((point - (55296 : Float)) * (1024 : Float))) + low) - (56320 : Float));
        (let at_2 := («at» + (1 : Float));
          (source_ml_temporary_displayPointsUtf16Text_join19.source_ml_temporary_displayPointsUtf16Text_join19_next at_2 point_2))) else (let point_3 := (65533 : Float);
        (source_ml_temporary_displayPointsUtf16Text_join19.source_ml_temporary_displayPointsUtf16Text_join19_next «at» point_3)))) else (if ((decide (point ≥ (56320 : Float))) && (decide (point ≤ (57343 : Float)))) then (let point_4 := (65533 : Float);
      (source_ml_temporary_displayPointsUtf16Text_join19.source_ml_temporary_displayPointsUtf16Text_join19_next «at» point_4)) else (source_ml_temporary_displayPointsUtf16Text_join19.source_ml_temporary_displayPointsUtf16Text_join19_next «at» point)));
      (match source_ml_source1_s10 with
        | source_ml_temporary_displayPointsUtf16Text_join19.source_ml_temporary_displayPointsUtf16Text_join19_next at_3 point_5 => (let result_2 := (result ++ (#[point_5] : (Array Float)));
            (let at_4 := (at_3 + (1 : Float));
              (source_ml_temporary_displayPointsUtf16Text_loop18 text result_2 at_4)))))) else result)

def displayPointsUtf16Text (text : (Array Float)) : (Array Float) :=
  (let result := (#[] : (Array Float));
    (let «at» := (0 : Float);
      (let result_2 := (source_ml_temporary_displayPointsUtf16Text_loop18 text result «at»);
        result_2)))

partial def source_ml_temporary_displayUtf8FromUtf16Text_loop20 (bytes : (Array Float)) (points : (Array Float)) («at» : Float) : (Array Float) :=
  (if (decide («at» < (Float.ofNat points.size))) then (let point := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then points[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (65533 : Float));
    (if (decide (point < (128 : Float))) then (let bytes_2 := (bytes ++ (#[point] : (Array Float)));
      (let at_2 := («at» + (1 : Float));
        (source_ml_temporary_displayUtf8FromUtf16Text_loop20 bytes_2 points at_2))) else (if (decide (point < (2048 : Float))) then (let bytes_3 := (bytes ++ (#[((192 : Float) + (Float.floor (point / (64 : Float)))), ((128 : Float) + (ml_float_rem point (64 : Float)))] : (Array Float)));
      (let at_3 := («at» + (1 : Float));
        (source_ml_temporary_displayUtf8FromUtf16Text_loop20 bytes_3 points at_3))) else (if (decide (point < (65536 : Float))) then (let bytes_4 := (bytes ++ (#[((224 : Float) + (Float.floor (point / (4096 : Float)))), ((128 : Float) + (ml_float_rem (Float.floor (point / (64 : Float))) (64 : Float))), ((128 : Float) + (ml_float_rem point (64 : Float)))] : (Array Float)));
      (let at_4 := («at» + (1 : Float));
        (source_ml_temporary_displayUtf8FromUtf16Text_loop20 bytes_4 points at_4))) else (let bytes_5 := (bytes ++ (#[((240 : Float) + (Float.floor (point / (262144 : Float)))), ((128 : Float) + (ml_float_rem (Float.floor (point / (4096 : Float))) (64 : Float))), ((128 : Float) + (ml_float_rem (Float.floor (point / (64 : Float))) (64 : Float))), ((128 : Float) + (ml_float_rem point (64 : Float)))] : (Array Float)));
      (let at_5 := («at» + (1 : Float));
        (source_ml_temporary_displayUtf8FromUtf16Text_loop20 bytes_5 points at_5))))))) else bytes)

def displayUtf8FromUtf16Text (text : (Array Float)) : (Array Float) :=
  (let bytes := (#[] : (Array Float));
    (let points := (displayPointsUtf16Text text);
      (let «at» := (0 : Float);
        (let bytes_2 := (source_ml_temporary_displayUtf8FromUtf16Text_loop20 bytes points «at»);
          bytes_2))))

def main : IO Unit := do
  pure ()

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


inductive source_ml_source2_io1 where
  | source_ml_source2_io1_mk (output : List (Array Nat)) (value : (Array Float)) : source_ml_source2_io1
  | source_ml_source2_io1_abort (output : List (Array Nat)) (message : (Array Nat)) (className : (Array Nat)) (messagePresent : Bool) : source_ml_source2_io1
  deriving Inhabited

def readUnicodeCaseLowerRanges : (Array Float) :=
  (#[(65 : Float), (90 : Float), (1 : Float), (32 : Float), (192 : Float), (214 : Float), (1 : Float), (32 : Float), (216 : Float), (222 : Float), (1 : Float), (32 : Float), (256 : Float), (302 : Float), (2 : Float), (1 : Float), (306 : Float), (310 : Float), (2 : Float), (1 : Float), (313 : Float), (327 : Float), (2 : Float), (1 : Float), (330 : Float), (374 : Float), (2 : Float), (1 : Float), (376 : Float), (376 : Float), (1 : Float), (-121 : Float), (377 : Float), (381 : Float), (2 : Float), (1 : Float), (385 : Float), (385 : Float), (1 : Float), (210 : Float), (386 : Float), (388 : Float), (2 : Float), (1 : Float), (390 : Float), (390 : Float), (1 : Float), (206 : Float), (391 : Float), (391 : Float), (1 : Float), (1 : Float), (393 : Float), (394 : Float), (1 : Float), (205 : Float), (395 : Float), (395 : Float), (1 : Float), (1 : Float), (398 : Float), (398 : Float), (1 : Float), (79 : Float), (399 : Float), (399 : Float), (1 : Float), (202 : Float), (400 : Float), (400 : Float), (1 : Float), (203 : Float), (401 : Float), (401 : Float), (1 : Float), (1 : Float), (403 : Float), (403 : Float), (1 : Float), (205 : Float), (404 : Float), (404 : Float), (1 : Float), (207 : Float), (406 : Float), (406 : Float), (1 : Float), (211 : Float), (407 : Float), (407 : Float), (1 : Float), (209 : Float), (408 : Float), (408 : Float), (1 : Float), (1 : Float), (412 : Float), (412 : Float), (1 : Float), (211 : Float), (413 : Float), (413 : Float), (1 : Float), (213 : Float), (415 : Float), (415 : Float), (1 : Float), (214 : Float), (416 : Float), (420 : Float), (2 : Float), (1 : Float), (422 : Float), (422 : Float), (1 : Float), (218 : Float), (423 : Float), (423 : Float), (1 : Float), (1 : Float), (425 : Float), (425 : Float), (1 : Float), (218 : Float), (428 : Float), (428 : Float), (1 : Float), (1 : Float), (430 : Float), (430 : Float), (1 : Float), (218 : Float), (431 : Float), (431 : Float), (1 : Float), (1 : Float), (433 : Float), (434 : Float), (1 : Float), (217 : Float), (435 : Float), (437 : Float), (2 : Float), (1 : Float), (439 : Float), (439 : Float), (1 : Float), (219 : Float), (440 : Float), (440 : Float), (1 : Float), (1 : Float), (444 : Float), (444 : Float), (1 : Float), (1 : Float), (452 : Float), (452 : Float), (1 : Float), (2 : Float), (453 : Float), (453 : Float), (1 : Float), (1 : Float), (455 : Float), (455 : Float), (1 : Float), (2 : Float), (456 : Float), (456 : Float), (1 : Float), (1 : Float), (458 : Float), (458 : Float), (1 : Float), (2 : Float), (459 : Float), (475 : Float), (2 : Float), (1 : Float), (478 : Float), (494 : Float), (2 : Float), (1 : Float), (497 : Float), (497 : Float), (1 : Float), (2 : Float), (498 : Float), (500 : Float), (2 : Float), (1 : Float), (502 : Float), (502 : Float), (1 : Float), (-97 : Float), (503 : Float), (503 : Float), (1 : Float), (-56 : Float), (504 : Float), (542 : Float), (2 : Float), (1 : Float), (544 : Float), (544 : Float), (1 : Float), (-130 : Float), (546 : Float), (562 : Float), (2 : Float), (1 : Float), (570 : Float), (570 : Float), (1 : Float), (10795 : Float), (571 : Float), (571 : Float), (1 : Float), (1 : Float), (573 : Float), (573 : Float), (1 : Float), (-163 : Float), (574 : Float), (574 : Float), (1 : Float), (10792 : Float), (577 : Float), (577 : Float), (1 : Float), (1 : Float), (579 : Float), (579 : Float), (1 : Float), (-195 : Float), (580 : Float), (580 : Float), (1 : Float), (69 : Float), (581 : Float), (581 : Float), (1 : Float), (71 : Float), (582 : Float), (590 : Float), (2 : Float), (1 : Float), (880 : Float), (882 : Float), (2 : Float), (1 : Float), (886 : Float), (886 : Float), (1 : Float), (1 : Float), (895 : Float), (895 : Float), (1 : Float), (116 : Float), (902 : Float), (902 : Float), (1 : Float), (38 : Float), (904 : Float), (906 : Float), (1 : Float), (37 : Float), (908 : Float), (908 : Float), (1 : Float), (64 : Float), (910 : Float), (911 : Float), (1 : Float), (63 : Float), (913 : Float), (929 : Float), (1 : Float), (32 : Float), (931 : Float), (939 : Float), (1 : Float), (32 : Float), (975 : Float), (975 : Float), (1 : Float), (8 : Float), (984 : Float), (1006 : Float), (2 : Float), (1 : Float), (1012 : Float), (1012 : Float), (1 : Float), (-60 : Float), (1015 : Float), (1015 : Float), (1 : Float), (1 : Float), (1017 : Float), (1017 : Float), (1 : Float), (-7 : Float), (1018 : Float), (1018 : Float), (1 : Float), (1 : Float), (1021 : Float), (1023 : Float), (1 : Float), (-130 : Float), (1024 : Float), (1039 : Float), (1 : Float), (80 : Float), (1040 : Float), (1071 : Float), (1 : Float), (32 : Float), (1120 : Float), (1152 : Float), (2 : Float), (1 : Float), (1162 : Float), (1214 : Float), (2 : Float), (1 : Float), (1216 : Float), (1216 : Float), (1 : Float), (15 : Float), (1217 : Float), (1229 : Float), (2 : Float), (1 : Float), (1232 : Float), (1326 : Float), (2 : Float), (1 : Float), (1329 : Float), (1366 : Float), (1 : Float), (48 : Float), (4256 : Float), (4293 : Float), (1 : Float), (7264 : Float), (4295 : Float), (4295 : Float), (1 : Float), (7264 : Float), (4301 : Float), (4301 : Float), (1 : Float), (7264 : Float), (5024 : Float), (5103 : Float), (1 : Float), (38864 : Float), (5104 : Float), (5109 : Float), (1 : Float), (8 : Float), (7305 : Float), (7305 : Float), (1 : Float), (1 : Float), (7312 : Float), (7354 : Float), (1 : Float), (-3008 : Float), (7357 : Float), (7359 : Float), (1 : Float), (-3008 : Float), (7680 : Float), (7828 : Float), (2 : Float), (1 : Float), (7838 : Float), (7838 : Float), (1 : Float), (-7615 : Float), (7840 : Float), (7934 : Float), (2 : Float), (1 : Float), (7944 : Float), (7951 : Float), (1 : Float), (-8 : Float), (7960 : Float), (7965 : Float), (1 : Float), (-8 : Float), (7976 : Float), (7983 : Float), (1 : Float), (-8 : Float), (7992 : Float), (7999 : Float), (1 : Float), (-8 : Float), (8008 : Float), (8013 : Float), (1 : Float), (-8 : Float), (8025 : Float), (8031 : Float), (2 : Float), (-8 : Float), (8040 : Float), (8047 : Float), (1 : Float), (-8 : Float), (8072 : Float), (8079 : Float), (1 : Float), (-8 : Float), (8088 : Float), (8095 : Float), (1 : Float), (-8 : Float), (8104 : Float), (8111 : Float), (1 : Float), (-8 : Float), (8120 : Float), (8121 : Float), (1 : Float), (-8 : Float), (8122 : Float), (8123 : Float), (1 : Float), (-74 : Float), (8124 : Float), (8124 : Float), (1 : Float), (-9 : Float), (8136 : Float), (8139 : Float), (1 : Float), (-86 : Float), (8140 : Float), (8140 : Float), (1 : Float), (-9 : Float), (8152 : Float), (8153 : Float), (1 : Float), (-8 : Float), (8154 : Float), (8155 : Float), (1 : Float), (-100 : Float), (8168 : Float), (8169 : Float), (1 : Float), (-8 : Float), (8170 : Float), (8171 : Float), (1 : Float), (-112 : Float), (8172 : Float), (8172 : Float), (1 : Float), (-7 : Float), (8184 : Float), (8185 : Float), (1 : Float), (-128 : Float), (8186 : Float), (8187 : Float), (1 : Float), (-126 : Float), (8188 : Float), (8188 : Float), (1 : Float), (-9 : Float), (8486 : Float), (8486 : Float), (1 : Float), (-7517 : Float), (8490 : Float), (8490 : Float), (1 : Float), (-8383 : Float), (8491 : Float), (8491 : Float), (1 : Float), (-8262 : Float), (8498 : Float), (8498 : Float), (1 : Float), (28 : Float), (8544 : Float), (8559 : Float), (1 : Float), (16 : Float), (8579 : Float), (8579 : Float), (1 : Float), (1 : Float), (9398 : Float), (9423 : Float), (1 : Float), (26 : Float), (11264 : Float), (11311 : Float), (1 : Float), (48 : Float), (11360 : Float), (11360 : Float), (1 : Float), (1 : Float), (11362 : Float), (11362 : Float), (1 : Float), (-10743 : Float), (11363 : Float), (11363 : Float), (1 : Float), (-3814 : Float), (11364 : Float), (11364 : Float), (1 : Float), (-10727 : Float), (11367 : Float), (11371 : Float), (2 : Float), (1 : Float), (11373 : Float), (11373 : Float), (1 : Float), (-10780 : Float), (11374 : Float), (11374 : Float), (1 : Float), (-10749 : Float), (11375 : Float), (11375 : Float), (1 : Float), (-10783 : Float), (11376 : Float), (11376 : Float), (1 : Float), (-10782 : Float), (11378 : Float), (11378 : Float), (1 : Float), (1 : Float), (11381 : Float), (11381 : Float), (1 : Float), (1 : Float), (11390 : Float), (11391 : Float), (1 : Float), (-10815 : Float), (11392 : Float), (11490 : Float), (2 : Float), (1 : Float), (11499 : Float), (11501 : Float), (2 : Float), (1 : Float), (11506 : Float), (11506 : Float), (1 : Float), (1 : Float), (42560 : Float), (42604 : Float), (2 : Float), (1 : Float), (42624 : Float), (42650 : Float), (2 : Float), (1 : Float), (42786 : Float), (42798 : Float), (2 : Float), (1 : Float), (42802 : Float), (42862 : Float), (2 : Float), (1 : Float), (42873 : Float), (42875 : Float), (2 : Float), (1 : Float), (42877 : Float), (42877 : Float), (1 : Float), (-35332 : Float), (42878 : Float), (42886 : Float), (2 : Float), (1 : Float), (42891 : Float), (42891 : Float), (1 : Float), (1 : Float), (42893 : Float), (42893 : Float), (1 : Float), (-42280 : Float), (42896 : Float), (42898 : Float), (2 : Float), (1 : Float), (42902 : Float), (42920 : Float), (2 : Float), (1 : Float), (42922 : Float), (42922 : Float), (1 : Float), (-42308 : Float), (42923 : Float), (42923 : Float), (1 : Float), (-42319 : Float), (42924 : Float), (42924 : Float), (1 : Float), (-42315 : Float), (42925 : Float), (42925 : Float), (1 : Float), (-42305 : Float), (42926 : Float), (42926 : Float), (1 : Float), (-42308 : Float), (42928 : Float), (42928 : Float), (1 : Float), (-42258 : Float), (42929 : Float), (42929 : Float), (1 : Float), (-42282 : Float), (42930 : Float), (42930 : Float), (1 : Float), (-42261 : Float), (42931 : Float), (42931 : Float), (1 : Float), (928 : Float), (42932 : Float), (42946 : Float), (2 : Float), (1 : Float), (42948 : Float), (42948 : Float), (1 : Float), (-48 : Float), (42949 : Float), (42949 : Float), (1 : Float), (-42307 : Float), (42950 : Float), (42950 : Float), (1 : Float), (-35384 : Float), (42951 : Float), (42953 : Float), (2 : Float), (1 : Float), (42955 : Float), (42955 : Float), (1 : Float), (-42343 : Float), (42956 : Float), (42956 : Float), (1 : Float), (1 : Float), (42960 : Float), (42960 : Float), (1 : Float), (1 : Float), (42966 : Float), (42970 : Float), (2 : Float), (1 : Float), (42972 : Float), (42972 : Float), (1 : Float), (-42561 : Float), (42997 : Float), (42997 : Float), (1 : Float), (1 : Float), (65313 : Float), (65338 : Float), (1 : Float), (32 : Float), (66560 : Float), (66599 : Float), (1 : Float), (40 : Float), (66736 : Float), (66771 : Float), (1 : Float), (40 : Float), (66928 : Float), (66938 : Float), (1 : Float), (39 : Float), (66940 : Float), (66954 : Float), (1 : Float), (39 : Float), (66956 : Float), (66962 : Float), (1 : Float), (39 : Float), (66964 : Float), (66965 : Float), (1 : Float), (39 : Float), (68736 : Float), (68786 : Float), (1 : Float), (64 : Float), (68944 : Float), (68965 : Float), (1 : Float), (32 : Float), (71840 : Float), (71871 : Float), (1 : Float), (32 : Float), (93760 : Float), (93791 : Float), (1 : Float), (32 : Float), (125184 : Float), (125217 : Float), (1 : Float), (34 : Float)] : (Array Float))

def readUnicodeCaseLowerIndexes : (Array Float) :=
  (#[(304 : Float), (0 : Float), (2 : Float)] : (Array Float))

def readUnicodeCaseLowerValues : (Array Float) :=
  (#[(105 : Float), (775 : Float)] : (Array Float))

def readUnicodeCaseUpperRanges : (Array Float) :=
  (#[(97 : Float), (122 : Float), (1 : Float), (-32 : Float), (181 : Float), (181 : Float), (1 : Float), (743 : Float), (224 : Float), (246 : Float), (1 : Float), (-32 : Float), (248 : Float), (254 : Float), (1 : Float), (-32 : Float), (255 : Float), (255 : Float), (1 : Float), (121 : Float), (257 : Float), (303 : Float), (2 : Float), (-1 : Float), (305 : Float), (305 : Float), (1 : Float), (-232 : Float), (307 : Float), (311 : Float), (2 : Float), (-1 : Float), (314 : Float), (328 : Float), (2 : Float), (-1 : Float), (331 : Float), (375 : Float), (2 : Float), (-1 : Float), (378 : Float), (382 : Float), (2 : Float), (-1 : Float), (383 : Float), (383 : Float), (1 : Float), (-300 : Float), (384 : Float), (384 : Float), (1 : Float), (195 : Float), (387 : Float), (389 : Float), (2 : Float), (-1 : Float), (392 : Float), (392 : Float), (1 : Float), (-1 : Float), (396 : Float), (396 : Float), (1 : Float), (-1 : Float), (402 : Float), (402 : Float), (1 : Float), (-1 : Float), (405 : Float), (405 : Float), (1 : Float), (97 : Float), (409 : Float), (409 : Float), (1 : Float), (-1 : Float), (410 : Float), (410 : Float), (1 : Float), (163 : Float), (411 : Float), (411 : Float), (1 : Float), (42561 : Float), (414 : Float), (414 : Float), (1 : Float), (130 : Float), (417 : Float), (421 : Float), (2 : Float), (-1 : Float), (424 : Float), (424 : Float), (1 : Float), (-1 : Float), (429 : Float), (429 : Float), (1 : Float), (-1 : Float), (432 : Float), (432 : Float), (1 : Float), (-1 : Float), (436 : Float), (438 : Float), (2 : Float), (-1 : Float), (441 : Float), (441 : Float), (1 : Float), (-1 : Float), (445 : Float), (445 : Float), (1 : Float), (-1 : Float), (447 : Float), (447 : Float), (1 : Float), (56 : Float), (453 : Float), (453 : Float), (1 : Float), (-1 : Float), (454 : Float), (454 : Float), (1 : Float), (-2 : Float), (456 : Float), (456 : Float), (1 : Float), (-1 : Float), (457 : Float), (457 : Float), (1 : Float), (-2 : Float), (459 : Float), (459 : Float), (1 : Float), (-1 : Float), (460 : Float), (460 : Float), (1 : Float), (-2 : Float), (462 : Float), (476 : Float), (2 : Float), (-1 : Float), (477 : Float), (477 : Float), (1 : Float), (-79 : Float), (479 : Float), (495 : Float), (2 : Float), (-1 : Float), (498 : Float), (498 : Float), (1 : Float), (-1 : Float), (499 : Float), (499 : Float), (1 : Float), (-2 : Float), (501 : Float), (501 : Float), (1 : Float), (-1 : Float), (505 : Float), (543 : Float), (2 : Float), (-1 : Float), (547 : Float), (563 : Float), (2 : Float), (-1 : Float), (572 : Float), (572 : Float), (1 : Float), (-1 : Float), (575 : Float), (576 : Float), (1 : Float), (10815 : Float), (578 : Float), (578 : Float), (1 : Float), (-1 : Float), (583 : Float), (591 : Float), (2 : Float), (-1 : Float), (592 : Float), (592 : Float), (1 : Float), (10783 : Float), (593 : Float), (593 : Float), (1 : Float), (10780 : Float), (594 : Float), (594 : Float), (1 : Float), (10782 : Float), (595 : Float), (595 : Float), (1 : Float), (-210 : Float), (596 : Float), (596 : Float), (1 : Float), (-206 : Float), (598 : Float), (599 : Float), (1 : Float), (-205 : Float), (601 : Float), (601 : Float), (1 : Float), (-202 : Float), (603 : Float), (603 : Float), (1 : Float), (-203 : Float), (604 : Float), (604 : Float), (1 : Float), (42319 : Float), (608 : Float), (608 : Float), (1 : Float), (-205 : Float), (609 : Float), (609 : Float), (1 : Float), (42315 : Float), (611 : Float), (611 : Float), (1 : Float), (-207 : Float), (612 : Float), (612 : Float), (1 : Float), (42343 : Float), (613 : Float), (613 : Float), (1 : Float), (42280 : Float), (614 : Float), (614 : Float), (1 : Float), (42308 : Float), (616 : Float), (616 : Float), (1 : Float), (-209 : Float), (617 : Float), (617 : Float), (1 : Float), (-211 : Float), (618 : Float), (618 : Float), (1 : Float), (42308 : Float), (619 : Float), (619 : Float), (1 : Float), (10743 : Float), (620 : Float), (620 : Float), (1 : Float), (42305 : Float), (623 : Float), (623 : Float), (1 : Float), (-211 : Float), (625 : Float), (625 : Float), (1 : Float), (10749 : Float), (626 : Float), (626 : Float), (1 : Float), (-213 : Float), (629 : Float), (629 : Float), (1 : Float), (-214 : Float), (637 : Float), (637 : Float), (1 : Float), (10727 : Float), (640 : Float), (640 : Float), (1 : Float), (-218 : Float), (642 : Float), (642 : Float), (1 : Float), (42307 : Float), (643 : Float), (643 : Float), (1 : Float), (-218 : Float), (647 : Float), (647 : Float), (1 : Float), (42282 : Float), (648 : Float), (648 : Float), (1 : Float), (-218 : Float), (649 : Float), (649 : Float), (1 : Float), (-69 : Float), (650 : Float), (651 : Float), (1 : Float), (-217 : Float), (652 : Float), (652 : Float), (1 : Float), (-71 : Float), (658 : Float), (658 : Float), (1 : Float), (-219 : Float), (669 : Float), (669 : Float), (1 : Float), (42261 : Float), (670 : Float), (670 : Float), (1 : Float), (42258 : Float), (837 : Float), (837 : Float), (1 : Float), (84 : Float), (881 : Float), (883 : Float), (2 : Float), (-1 : Float), (887 : Float), (887 : Float), (1 : Float), (-1 : Float), (891 : Float), (893 : Float), (1 : Float), (130 : Float), (940 : Float), (940 : Float), (1 : Float), (-38 : Float), (941 : Float), (943 : Float), (1 : Float), (-37 : Float), (945 : Float), (961 : Float), (1 : Float), (-32 : Float), (962 : Float), (962 : Float), (1 : Float), (-31 : Float), (963 : Float), (971 : Float), (1 : Float), (-32 : Float), (972 : Float), (972 : Float), (1 : Float), (-64 : Float), (973 : Float), (974 : Float), (1 : Float), (-63 : Float), (976 : Float), (976 : Float), (1 : Float), (-62 : Float), (977 : Float), (977 : Float), (1 : Float), (-57 : Float), (981 : Float), (981 : Float), (1 : Float), (-47 : Float), (982 : Float), (982 : Float), (1 : Float), (-54 : Float), (983 : Float), (983 : Float), (1 : Float), (-8 : Float), (985 : Float), (1007 : Float), (2 : Float), (-1 : Float), (1008 : Float), (1008 : Float), (1 : Float), (-86 : Float), (1009 : Float), (1009 : Float), (1 : Float), (-80 : Float), (1010 : Float), (1010 : Float), (1 : Float), (7 : Float), (1011 : Float), (1011 : Float), (1 : Float), (-116 : Float), (1013 : Float), (1013 : Float), (1 : Float), (-96 : Float), (1016 : Float), (1016 : Float), (1 : Float), (-1 : Float), (1019 : Float), (1019 : Float), (1 : Float), (-1 : Float), (1072 : Float), (1103 : Float), (1 : Float), (-32 : Float), (1104 : Float), (1119 : Float), (1 : Float), (-80 : Float), (1121 : Float), (1153 : Float), (2 : Float), (-1 : Float), (1163 : Float), (1215 : Float), (2 : Float), (-1 : Float), (1218 : Float), (1230 : Float), (2 : Float), (-1 : Float), (1231 : Float), (1231 : Float), (1 : Float), (-15 : Float), (1233 : Float), (1327 : Float), (2 : Float), (-1 : Float), (1377 : Float), (1414 : Float), (1 : Float), (-48 : Float), (4304 : Float), (4346 : Float), (1 : Float), (3008 : Float), (4349 : Float), (4351 : Float), (1 : Float), (3008 : Float), (5112 : Float), (5117 : Float), (1 : Float), (-8 : Float), (7296 : Float), (7296 : Float), (1 : Float), (-6254 : Float), (7297 : Float), (7297 : Float), (1 : Float), (-6253 : Float), (7298 : Float), (7298 : Float), (1 : Float), (-6244 : Float), (7299 : Float), (7300 : Float), (1 : Float), (-6242 : Float), (7301 : Float), (7301 : Float), (1 : Float), (-6243 : Float), (7302 : Float), (7302 : Float), (1 : Float), (-6236 : Float), (7303 : Float), (7303 : Float), (1 : Float), (-6181 : Float), (7304 : Float), (7304 : Float), (1 : Float), (35266 : Float), (7306 : Float), (7306 : Float), (1 : Float), (-1 : Float), (7545 : Float), (7545 : Float), (1 : Float), (35332 : Float), (7549 : Float), (7549 : Float), (1 : Float), (3814 : Float), (7566 : Float), (7566 : Float), (1 : Float), (35384 : Float), (7681 : Float), (7829 : Float), (2 : Float), (-1 : Float), (7835 : Float), (7835 : Float), (1 : Float), (-59 : Float), (7841 : Float), (7935 : Float), (2 : Float), (-1 : Float), (7936 : Float), (7943 : Float), (1 : Float), (8 : Float), (7952 : Float), (7957 : Float), (1 : Float), (8 : Float), (7968 : Float), (7975 : Float), (1 : Float), (8 : Float), (7984 : Float), (7991 : Float), (1 : Float), (8 : Float), (8000 : Float), (8005 : Float), (1 : Float), (8 : Float), (8017 : Float), (8023 : Float), (2 : Float), (8 : Float), (8032 : Float), (8039 : Float), (1 : Float), (8 : Float), (8048 : Float), (8049 : Float), (1 : Float), (74 : Float), (8050 : Float), (8053 : Float), (1 : Float), (86 : Float), (8054 : Float), (8055 : Float), (1 : Float), (100 : Float), (8056 : Float), (8057 : Float), (1 : Float), (128 : Float), (8058 : Float), (8059 : Float), (1 : Float), (112 : Float), (8060 : Float), (8061 : Float), (1 : Float), (126 : Float), (8112 : Float), (8113 : Float), (1 : Float), (8 : Float), (8126 : Float), (8126 : Float), (1 : Float), (-7205 : Float), (8144 : Float), (8145 : Float), (1 : Float), (8 : Float), (8160 : Float), (8161 : Float), (1 : Float), (8 : Float), (8165 : Float), (8165 : Float), (1 : Float), (7 : Float), (8526 : Float), (8526 : Float), (1 : Float), (-28 : Float), (8560 : Float), (8575 : Float), (1 : Float), (-16 : Float), (8580 : Float), (8580 : Float), (1 : Float), (-1 : Float), (9424 : Float), (9449 : Float), (1 : Float), (-26 : Float), (11312 : Float), (11359 : Float), (1 : Float), (-48 : Float), (11361 : Float), (11361 : Float), (1 : Float), (-1 : Float), (11365 : Float), (11365 : Float), (1 : Float), (-10795 : Float), (11366 : Float), (11366 : Float), (1 : Float), (-10792 : Float), (11368 : Float), (11372 : Float), (2 : Float), (-1 : Float), (11379 : Float), (11379 : Float), (1 : Float), (-1 : Float), (11382 : Float), (11382 : Float), (1 : Float), (-1 : Float), (11393 : Float), (11491 : Float), (2 : Float), (-1 : Float), (11500 : Float), (11502 : Float), (2 : Float), (-1 : Float), (11507 : Float), (11507 : Float), (1 : Float), (-1 : Float), (11520 : Float), (11557 : Float), (1 : Float), (-7264 : Float), (11559 : Float), (11559 : Float), (1 : Float), (-7264 : Float), (11565 : Float), (11565 : Float), (1 : Float), (-7264 : Float), (42561 : Float), (42605 : Float), (2 : Float), (-1 : Float), (42625 : Float), (42651 : Float), (2 : Float), (-1 : Float), (42787 : Float), (42799 : Float), (2 : Float), (-1 : Float), (42803 : Float), (42863 : Float), (2 : Float), (-1 : Float), (42874 : Float), (42876 : Float), (2 : Float), (-1 : Float), (42879 : Float), (42887 : Float), (2 : Float), (-1 : Float), (42892 : Float), (42892 : Float), (1 : Float), (-1 : Float), (42897 : Float), (42899 : Float), (2 : Float), (-1 : Float), (42900 : Float), (42900 : Float), (1 : Float), (48 : Float), (42903 : Float), (42921 : Float), (2 : Float), (-1 : Float), (42933 : Float), (42947 : Float), (2 : Float), (-1 : Float), (42952 : Float), (42954 : Float), (2 : Float), (-1 : Float), (42957 : Float), (42957 : Float), (1 : Float), (-1 : Float), (42961 : Float), (42961 : Float), (1 : Float), (-1 : Float), (42967 : Float), (42971 : Float), (2 : Float), (-1 : Float), (42998 : Float), (42998 : Float), (1 : Float), (-1 : Float), (43859 : Float), (43859 : Float), (1 : Float), (-928 : Float), (43888 : Float), (43967 : Float), (1 : Float), (-38864 : Float), (65345 : Float), (65370 : Float), (1 : Float), (-32 : Float), (66600 : Float), (66639 : Float), (1 : Float), (-40 : Float), (66776 : Float), (66811 : Float), (1 : Float), (-40 : Float), (66967 : Float), (66977 : Float), (1 : Float), (-39 : Float), (66979 : Float), (66993 : Float), (1 : Float), (-39 : Float), (66995 : Float), (67001 : Float), (1 : Float), (-39 : Float), (67003 : Float), (67004 : Float), (1 : Float), (-39 : Float), (68800 : Float), (68850 : Float), (1 : Float), (-64 : Float), (68976 : Float), (68997 : Float), (1 : Float), (-32 : Float), (71872 : Float), (71903 : Float), (1 : Float), (-32 : Float), (93792 : Float), (93823 : Float), (1 : Float), (-32 : Float), (125218 : Float), (125251 : Float), (1 : Float), (-34 : Float)] : (Array Float))

def readUnicodeCaseUpperIndexes : (Array Float) :=
  (#[(223 : Float), (0 : Float), (2 : Float), (329 : Float), (2 : Float), (2 : Float), (496 : Float), (4 : Float), (2 : Float), (912 : Float), (6 : Float), (3 : Float), (944 : Float), (9 : Float), (3 : Float), (1415 : Float), (12 : Float), (2 : Float), (7830 : Float), (14 : Float), (2 : Float), (7831 : Float), (16 : Float), (2 : Float), (7832 : Float), (18 : Float), (2 : Float), (7833 : Float), (20 : Float), (2 : Float), (7834 : Float), (22 : Float), (2 : Float), (8016 : Float), (24 : Float), (2 : Float), (8018 : Float), (26 : Float), (3 : Float), (8020 : Float), (29 : Float), (3 : Float), (8022 : Float), (32 : Float), (3 : Float), (8064 : Float), (35 : Float), (2 : Float), (8065 : Float), (37 : Float), (2 : Float), (8066 : Float), (39 : Float), (2 : Float), (8067 : Float), (41 : Float), (2 : Float), (8068 : Float), (43 : Float), (2 : Float), (8069 : Float), (45 : Float), (2 : Float), (8070 : Float), (47 : Float), (2 : Float), (8071 : Float), (49 : Float), (2 : Float), (8072 : Float), (51 : Float), (2 : Float), (8073 : Float), (53 : Float), (2 : Float), (8074 : Float), (55 : Float), (2 : Float), (8075 : Float), (57 : Float), (2 : Float), (8076 : Float), (59 : Float), (2 : Float), (8077 : Float), (61 : Float), (2 : Float), (8078 : Float), (63 : Float), (2 : Float), (8079 : Float), (65 : Float), (2 : Float), (8080 : Float), (67 : Float), (2 : Float), (8081 : Float), (69 : Float), (2 : Float), (8082 : Float), (71 : Float), (2 : Float), (8083 : Float), (73 : Float), (2 : Float), (8084 : Float), (75 : Float), (2 : Float), (8085 : Float), (77 : Float), (2 : Float), (8086 : Float), (79 : Float), (2 : Float), (8087 : Float), (81 : Float), (2 : Float), (8088 : Float), (83 : Float), (2 : Float), (8089 : Float), (85 : Float), (2 : Float), (8090 : Float), (87 : Float), (2 : Float), (8091 : Float), (89 : Float), (2 : Float), (8092 : Float), (91 : Float), (2 : Float), (8093 : Float), (93 : Float), (2 : Float), (8094 : Float), (95 : Float), (2 : Float), (8095 : Float), (97 : Float), (2 : Float), (8096 : Float), (99 : Float), (2 : Float), (8097 : Float), (101 : Float), (2 : Float), (8098 : Float), (103 : Float), (2 : Float), (8099 : Float), (105 : Float), (2 : Float), (8100 : Float), (107 : Float), (2 : Float), (8101 : Float), (109 : Float), (2 : Float), (8102 : Float), (111 : Float), (2 : Float), (8103 : Float), (113 : Float), (2 : Float), (8104 : Float), (115 : Float), (2 : Float), (8105 : Float), (117 : Float), (2 : Float), (8106 : Float), (119 : Float), (2 : Float), (8107 : Float), (121 : Float), (2 : Float), (8108 : Float), (123 : Float), (2 : Float), (8109 : Float), (125 : Float), (2 : Float), (8110 : Float), (127 : Float), (2 : Float), (8111 : Float), (129 : Float), (2 : Float), (8114 : Float), (131 : Float), (2 : Float), (8115 : Float), (133 : Float), (2 : Float), (8116 : Float), (135 : Float), (2 : Float), (8118 : Float), (137 : Float), (2 : Float), (8119 : Float), (139 : Float), (3 : Float), (8124 : Float), (142 : Float), (2 : Float), (8130 : Float), (144 : Float), (2 : Float), (8131 : Float), (146 : Float), (2 : Float), (8132 : Float), (148 : Float), (2 : Float), (8134 : Float), (150 : Float), (2 : Float), (8135 : Float), (152 : Float), (3 : Float), (8140 : Float), (155 : Float), (2 : Float), (8146 : Float), (157 : Float), (3 : Float), (8147 : Float), (160 : Float), (3 : Float), (8150 : Float), (163 : Float), (2 : Float), (8151 : Float), (165 : Float), (3 : Float), (8162 : Float), (168 : Float), (3 : Float), (8163 : Float), (171 : Float), (3 : Float), (8164 : Float), (174 : Float), (2 : Float), (8166 : Float), (176 : Float), (2 : Float), (8167 : Float), (178 : Float), (3 : Float), (8178 : Float), (181 : Float), (2 : Float), (8179 : Float), (183 : Float), (2 : Float), (8180 : Float), (185 : Float), (2 : Float), (8182 : Float), (187 : Float), (2 : Float), (8183 : Float), (189 : Float), (3 : Float), (8188 : Float), (192 : Float), (2 : Float), (64256 : Float), (194 : Float), (2 : Float), (64257 : Float), (196 : Float), (2 : Float), (64258 : Float), (198 : Float), (2 : Float), (64259 : Float), (200 : Float), (3 : Float), (64260 : Float), (203 : Float), (3 : Float), (64261 : Float), (206 : Float), (2 : Float), (64262 : Float), (208 : Float), (2 : Float), (64275 : Float), (210 : Float), (2 : Float), (64276 : Float), (212 : Float), (2 : Float), (64277 : Float), (214 : Float), (2 : Float), (64278 : Float), (216 : Float), (2 : Float), (64279 : Float), (218 : Float), (2 : Float)] : (Array Float))

def readUnicodeCaseUpperValues : (Array Float) :=
  (#[(83 : Float), (83 : Float), (700 : Float), (78 : Float), (74 : Float), (780 : Float), (921 : Float), (776 : Float), (769 : Float), (933 : Float), (776 : Float), (769 : Float), (1333 : Float), (1362 : Float), (72 : Float), (817 : Float), (84 : Float), (776 : Float), (87 : Float), (778 : Float), (89 : Float), (778 : Float), (65 : Float), (702 : Float), (933 : Float), (787 : Float), (933 : Float), (787 : Float), (768 : Float), (933 : Float), (787 : Float), (769 : Float), (933 : Float), (787 : Float), (834 : Float), (7944 : Float), (921 : Float), (7945 : Float), (921 : Float), (7946 : Float), (921 : Float), (7947 : Float), (921 : Float), (7948 : Float), (921 : Float), (7949 : Float), (921 : Float), (7950 : Float), (921 : Float), (7951 : Float), (921 : Float), (7944 : Float), (921 : Float), (7945 : Float), (921 : Float), (7946 : Float), (921 : Float), (7947 : Float), (921 : Float), (7948 : Float), (921 : Float), (7949 : Float), (921 : Float), (7950 : Float), (921 : Float), (7951 : Float), (921 : Float), (7976 : Float), (921 : Float), (7977 : Float), (921 : Float), (7978 : Float), (921 : Float), (7979 : Float), (921 : Float), (7980 : Float), (921 : Float), (7981 : Float), (921 : Float), (7982 : Float), (921 : Float), (7983 : Float), (921 : Float), (7976 : Float), (921 : Float), (7977 : Float), (921 : Float), (7978 : Float), (921 : Float), (7979 : Float), (921 : Float), (7980 : Float), (921 : Float), (7981 : Float), (921 : Float), (7982 : Float), (921 : Float), (7983 : Float), (921 : Float), (8040 : Float), (921 : Float), (8041 : Float), (921 : Float), (8042 : Float), (921 : Float), (8043 : Float), (921 : Float), (8044 : Float), (921 : Float), (8045 : Float), (921 : Float), (8046 : Float), (921 : Float), (8047 : Float), (921 : Float), (8040 : Float), (921 : Float), (8041 : Float), (921 : Float), (8042 : Float), (921 : Float), (8043 : Float), (921 : Float), (8044 : Float), (921 : Float), (8045 : Float), (921 : Float), (8046 : Float), (921 : Float), (8047 : Float), (921 : Float), (8122 : Float), (921 : Float), (913 : Float), (921 : Float), (902 : Float), (921 : Float), (913 : Float), (834 : Float), (913 : Float), (834 : Float), (921 : Float), (913 : Float), (921 : Float), (8138 : Float), (921 : Float), (919 : Float), (921 : Float), (905 : Float), (921 : Float), (919 : Float), (834 : Float), (919 : Float), (834 : Float), (921 : Float), (919 : Float), (921 : Float), (921 : Float), (776 : Float), (768 : Float), (921 : Float), (776 : Float), (769 : Float), (921 : Float), (834 : Float), (921 : Float), (776 : Float), (834 : Float), (933 : Float), (776 : Float), (768 : Float), (933 : Float), (776 : Float), (769 : Float), (929 : Float), (787 : Float), (933 : Float), (834 : Float), (933 : Float), (776 : Float), (834 : Float), (8186 : Float), (921 : Float), (937 : Float), (921 : Float), (911 : Float), (921 : Float), (937 : Float), (834 : Float), (937 : Float), (834 : Float), (921 : Float), (937 : Float), (921 : Float), (70 : Float), (70 : Float), (70 : Float), (73 : Float), (70 : Float), (76 : Float), (70 : Float), (70 : Float), (73 : Float), (70 : Float), (70 : Float), (76 : Float), (83 : Float), (84 : Float), (83 : Float), (84 : Float), (1348 : Float), (1350 : Float), (1348 : Float), (1333 : Float), (1348 : Float), (1339 : Float), (1358 : Float), (1350 : Float), (1348 : Float), (1341 : Float)] : (Array Float))

def readUnicodeCaseCasedRanges : (Array Float) :=
  (#[(65 : Float), (90 : Float), (97 : Float), (122 : Float), (170 : Float), (170 : Float), (181 : Float), (181 : Float), (186 : Float), (186 : Float), (192 : Float), (214 : Float), (216 : Float), (246 : Float), (248 : Float), (442 : Float), (444 : Float), (447 : Float), (452 : Float), (659 : Float), (661 : Float), (687 : Float), (688 : Float), (696 : Float), (704 : Float), (705 : Float), (736 : Float), (740 : Float), (837 : Float), (837 : Float), (880 : Float), (883 : Float), (886 : Float), (887 : Float), (890 : Float), (890 : Float), (891 : Float), (893 : Float), (895 : Float), (895 : Float), (902 : Float), (902 : Float), (904 : Float), (906 : Float), (908 : Float), (908 : Float), (910 : Float), (929 : Float), (931 : Float), (1013 : Float), (1015 : Float), (1153 : Float), (1162 : Float), (1327 : Float), (1329 : Float), (1366 : Float), (1376 : Float), (1416 : Float), (4256 : Float), (4293 : Float), (4295 : Float), (4295 : Float), (4301 : Float), (4301 : Float), (4304 : Float), (4346 : Float), (4348 : Float), (4348 : Float), (4349 : Float), (4351 : Float), (5024 : Float), (5109 : Float), (5112 : Float), (5117 : Float), (7296 : Float), (7306 : Float), (7312 : Float), (7354 : Float), (7357 : Float), (7359 : Float), (7424 : Float), (7467 : Float), (7468 : Float), (7530 : Float), (7531 : Float), (7543 : Float), (7544 : Float), (7544 : Float), (7545 : Float), (7578 : Float), (7579 : Float), (7615 : Float), (7680 : Float), (7957 : Float), (7960 : Float), (7965 : Float), (7968 : Float), (8005 : Float), (8008 : Float), (8013 : Float), (8016 : Float), (8023 : Float), (8025 : Float), (8025 : Float), (8027 : Float), (8027 : Float), (8029 : Float), (8029 : Float), (8031 : Float), (8061 : Float), (8064 : Float), (8116 : Float), (8118 : Float), (8124 : Float), (8126 : Float), (8126 : Float), (8130 : Float), (8132 : Float), (8134 : Float), (8140 : Float), (8144 : Float), (8147 : Float), (8150 : Float), (8155 : Float), (8160 : Float), (8172 : Float), (8178 : Float), (8180 : Float), (8182 : Float), (8188 : Float), (8305 : Float), (8305 : Float), (8319 : Float), (8319 : Float), (8336 : Float), (8348 : Float), (8450 : Float), (8450 : Float), (8455 : Float), (8455 : Float), (8458 : Float), (8467 : Float), (8469 : Float), (8469 : Float), (8473 : Float), (8477 : Float), (8484 : Float), (8484 : Float), (8486 : Float), (8486 : Float), (8488 : Float), (8488 : Float), (8490 : Float), (8493 : Float), (8495 : Float), (8500 : Float), (8505 : Float), (8505 : Float), (8508 : Float), (8511 : Float), (8517 : Float), (8521 : Float), (8526 : Float), (8526 : Float), (8544 : Float), (8575 : Float), (8579 : Float), (8580 : Float), (9398 : Float), (9449 : Float), (11264 : Float), (11387 : Float), (11388 : Float), (11389 : Float), (11390 : Float), (11492 : Float), (11499 : Float), (11502 : Float), (11506 : Float), (11507 : Float), (11520 : Float), (11557 : Float), (11559 : Float), (11559 : Float), (11565 : Float), (11565 : Float), (42560 : Float), (42605 : Float), (42624 : Float), (42651 : Float), (42652 : Float), (42653 : Float), (42786 : Float), (42863 : Float), (42864 : Float), (42864 : Float), (42865 : Float), (42887 : Float), (42891 : Float), (42894 : Float), (42896 : Float), (42957 : Float), (42960 : Float), (42961 : Float), (42963 : Float), (42963 : Float), (42965 : Float), (42972 : Float), (42994 : Float), (42996 : Float), (42997 : Float), (42998 : Float), (43000 : Float), (43001 : Float), (43002 : Float), (43002 : Float), (43824 : Float), (43866 : Float), (43868 : Float), (43871 : Float), (43872 : Float), (43880 : Float), (43881 : Float), (43881 : Float), (43888 : Float), (43967 : Float), (64256 : Float), (64262 : Float), (64275 : Float), (64279 : Float), (65313 : Float), (65338 : Float), (65345 : Float), (65370 : Float), (66560 : Float), (66639 : Float), (66736 : Float), (66771 : Float), (66776 : Float), (66811 : Float), (66928 : Float), (66938 : Float), (66940 : Float), (66954 : Float), (66956 : Float), (66962 : Float), (66964 : Float), (66965 : Float), (66967 : Float), (66977 : Float), (66979 : Float), (66993 : Float), (66995 : Float), (67001 : Float), (67003 : Float), (67004 : Float), (67456 : Float), (67456 : Float), (67459 : Float), (67461 : Float), (67463 : Float), (67504 : Float), (67506 : Float), (67514 : Float), (68736 : Float), (68786 : Float), (68800 : Float), (68850 : Float), (68944 : Float), (68965 : Float), (68976 : Float), (68997 : Float), (71840 : Float), (71903 : Float), (93760 : Float), (93823 : Float), (119808 : Float), (119892 : Float), (119894 : Float), (119964 : Float), (119966 : Float), (119967 : Float), (119970 : Float), (119970 : Float), (119973 : Float), (119974 : Float), (119977 : Float), (119980 : Float), (119982 : Float), (119993 : Float), (119995 : Float), (119995 : Float), (119997 : Float), (120003 : Float), (120005 : Float), (120069 : Float), (120071 : Float), (120074 : Float), (120077 : Float), (120084 : Float), (120086 : Float), (120092 : Float), (120094 : Float), (120121 : Float), (120123 : Float), (120126 : Float), (120128 : Float), (120132 : Float), (120134 : Float), (120134 : Float), (120138 : Float), (120144 : Float), (120146 : Float), (120485 : Float), (120488 : Float), (120512 : Float), (120514 : Float), (120538 : Float), (120540 : Float), (120570 : Float), (120572 : Float), (120596 : Float), (120598 : Float), (120628 : Float), (120630 : Float), (120654 : Float), (120656 : Float), (120686 : Float), (120688 : Float), (120712 : Float), (120714 : Float), (120744 : Float), (120746 : Float), (120770 : Float), (120772 : Float), (120779 : Float), (122624 : Float), (122633 : Float), (122635 : Float), (122654 : Float), (122661 : Float), (122666 : Float), (122928 : Float), (122989 : Float), (125184 : Float), (125251 : Float), (127280 : Float), (127305 : Float), (127312 : Float), (127337 : Float), (127344 : Float), (127369 : Float)] : (Array Float))

def readUnicodeCaseIgnorableRanges : (Array Float) :=
  (#[(39 : Float), (39 : Float), (46 : Float), (46 : Float), (58 : Float), (58 : Float), (94 : Float), (94 : Float), (96 : Float), (96 : Float), (168 : Float), (168 : Float), (173 : Float), (173 : Float), (175 : Float), (175 : Float), (180 : Float), (180 : Float), (183 : Float), (183 : Float), (184 : Float), (184 : Float), (688 : Float), (705 : Float), (706 : Float), (709 : Float), (710 : Float), (721 : Float), (722 : Float), (735 : Float), (736 : Float), (740 : Float), (741 : Float), (747 : Float), (748 : Float), (748 : Float), (749 : Float), (749 : Float), (750 : Float), (750 : Float), (751 : Float), (767 : Float), (768 : Float), (879 : Float), (884 : Float), (884 : Float), (885 : Float), (885 : Float), (890 : Float), (890 : Float), (900 : Float), (901 : Float), (903 : Float), (903 : Float), (1155 : Float), (1159 : Float), (1160 : Float), (1161 : Float), (1369 : Float), (1369 : Float), (1375 : Float), (1375 : Float), (1425 : Float), (1469 : Float), (1471 : Float), (1471 : Float), (1473 : Float), (1474 : Float), (1476 : Float), (1477 : Float), (1479 : Float), (1479 : Float), (1524 : Float), (1524 : Float), (1536 : Float), (1541 : Float), (1552 : Float), (1562 : Float), (1564 : Float), (1564 : Float), (1600 : Float), (1600 : Float), (1611 : Float), (1631 : Float), (1648 : Float), (1648 : Float), (1750 : Float), (1756 : Float), (1757 : Float), (1757 : Float), (1759 : Float), (1764 : Float), (1765 : Float), (1766 : Float), (1767 : Float), (1768 : Float), (1770 : Float), (1773 : Float), (1807 : Float), (1807 : Float), (1809 : Float), (1809 : Float), (1840 : Float), (1866 : Float), (1958 : Float), (1968 : Float), (2027 : Float), (2035 : Float), (2036 : Float), (2037 : Float), (2042 : Float), (2042 : Float), (2045 : Float), (2045 : Float), (2070 : Float), (2073 : Float), (2074 : Float), (2074 : Float), (2075 : Float), (2083 : Float), (2084 : Float), (2084 : Float), (2085 : Float), (2087 : Float), (2088 : Float), (2088 : Float), (2089 : Float), (2093 : Float), (2137 : Float), (2139 : Float), (2184 : Float), (2184 : Float), (2192 : Float), (2193 : Float), (2199 : Float), (2207 : Float), (2249 : Float), (2249 : Float), (2250 : Float), (2273 : Float), (2274 : Float), (2274 : Float), (2275 : Float), (2306 : Float), (2362 : Float), (2362 : Float), (2364 : Float), (2364 : Float), (2369 : Float), (2376 : Float), (2381 : Float), (2381 : Float), (2385 : Float), (2391 : Float), (2402 : Float), (2403 : Float), (2417 : Float), (2417 : Float), (2433 : Float), (2433 : Float), (2492 : Float), (2492 : Float), (2497 : Float), (2500 : Float), (2509 : Float), (2509 : Float), (2530 : Float), (2531 : Float), (2558 : Float), (2558 : Float), (2561 : Float), (2562 : Float), (2620 : Float), (2620 : Float), (2625 : Float), (2626 : Float), (2631 : Float), (2632 : Float), (2635 : Float), (2637 : Float), (2641 : Float), (2641 : Float), (2672 : Float), (2673 : Float), (2677 : Float), (2677 : Float), (2689 : Float), (2690 : Float), (2748 : Float), (2748 : Float), (2753 : Float), (2757 : Float), (2759 : Float), (2760 : Float), (2765 : Float), (2765 : Float), (2786 : Float), (2787 : Float), (2810 : Float), (2815 : Float), (2817 : Float), (2817 : Float), (2876 : Float), (2876 : Float), (2879 : Float), (2879 : Float), (2881 : Float), (2884 : Float), (2893 : Float), (2893 : Float), (2901 : Float), (2902 : Float), (2914 : Float), (2915 : Float), (2946 : Float), (2946 : Float), (3008 : Float), (3008 : Float), (3021 : Float), (3021 : Float), (3072 : Float), (3072 : Float), (3076 : Float), (3076 : Float), (3132 : Float), (3132 : Float), (3134 : Float), (3136 : Float), (3142 : Float), (3144 : Float), (3146 : Float), (3149 : Float), (3157 : Float), (3158 : Float), (3170 : Float), (3171 : Float), (3201 : Float), (3201 : Float), (3260 : Float), (3260 : Float), (3263 : Float), (3263 : Float), (3270 : Float), (3270 : Float), (3276 : Float), (3277 : Float), (3298 : Float), (3299 : Float), (3328 : Float), (3329 : Float), (3387 : Float), (3388 : Float), (3393 : Float), (3396 : Float), (3405 : Float), (3405 : Float), (3426 : Float), (3427 : Float), (3457 : Float), (3457 : Float), (3530 : Float), (3530 : Float), (3538 : Float), (3540 : Float), (3542 : Float), (3542 : Float), (3633 : Float), (3633 : Float), (3636 : Float), (3642 : Float), (3654 : Float), (3654 : Float), (3655 : Float), (3662 : Float), (3761 : Float), (3761 : Float), (3764 : Float), (3772 : Float), (3782 : Float), (3782 : Float), (3784 : Float), (3790 : Float), (3864 : Float), (3865 : Float), (3893 : Float), (3893 : Float), (3895 : Float), (3895 : Float), (3897 : Float), (3897 : Float), (3953 : Float), (3966 : Float), (3968 : Float), (3972 : Float), (3974 : Float), (3975 : Float), (3981 : Float), (3991 : Float), (3993 : Float), (4028 : Float), (4038 : Float), (4038 : Float), (4141 : Float), (4144 : Float), (4146 : Float), (4151 : Float), (4153 : Float), (4154 : Float), (4157 : Float), (4158 : Float), (4184 : Float), (4185 : Float), (4190 : Float), (4192 : Float), (4209 : Float), (4212 : Float), (4226 : Float), (4226 : Float), (4229 : Float), (4230 : Float), (4237 : Float), (4237 : Float), (4253 : Float), (4253 : Float), (4348 : Float), (4348 : Float), (4957 : Float), (4959 : Float), (5906 : Float), (5908 : Float), (5938 : Float), (5939 : Float), (5970 : Float), (5971 : Float), (6002 : Float), (6003 : Float), (6068 : Float), (6069 : Float), (6071 : Float), (6077 : Float), (6086 : Float), (6086 : Float), (6089 : Float), (6099 : Float), (6103 : Float), (6103 : Float), (6109 : Float), (6109 : Float), (6155 : Float), (6157 : Float), (6158 : Float), (6158 : Float), (6159 : Float), (6159 : Float), (6211 : Float), (6211 : Float), (6277 : Float), (6278 : Float), (6313 : Float), (6313 : Float), (6432 : Float), (6434 : Float), (6439 : Float), (6440 : Float), (6450 : Float), (6450 : Float), (6457 : Float), (6459 : Float), (6679 : Float), (6680 : Float), (6683 : Float), (6683 : Float), (6742 : Float), (6742 : Float), (6744 : Float), (6750 : Float), (6752 : Float), (6752 : Float), (6754 : Float), (6754 : Float), (6757 : Float), (6764 : Float), (6771 : Float), (6780 : Float), (6783 : Float), (6783 : Float), (6823 : Float), (6823 : Float), (6832 : Float), (6845 : Float), (6846 : Float), (6846 : Float), (6847 : Float), (6862 : Float), (6912 : Float), (6915 : Float), (6964 : Float), (6964 : Float), (6966 : Float), (6970 : Float), (6972 : Float), (6972 : Float), (6978 : Float), (6978 : Float), (7019 : Float), (7027 : Float), (7040 : Float), (7041 : Float), (7074 : Float), (7077 : Float), (7080 : Float), (7081 : Float), (7083 : Float), (7085 : Float), (7142 : Float), (7142 : Float), (7144 : Float), (7145 : Float), (7149 : Float), (7149 : Float), (7151 : Float), (7153 : Float), (7212 : Float), (7219 : Float), (7222 : Float), (7223 : Float), (7288 : Float), (7293 : Float), (7376 : Float), (7378 : Float), (7380 : Float), (7392 : Float), (7394 : Float), (7400 : Float), (7405 : Float), (7405 : Float), (7412 : Float), (7412 : Float), (7416 : Float), (7417 : Float), (7468 : Float), (7530 : Float), (7544 : Float), (7544 : Float), (7579 : Float), (7615 : Float), (7616 : Float), (7679 : Float), (8125 : Float), (8125 : Float), (8127 : Float), (8129 : Float), (8141 : Float), (8143 : Float), (8157 : Float), (8159 : Float), (8173 : Float), (8175 : Float), (8189 : Float), (8190 : Float), (8203 : Float), (8207 : Float), (8216 : Float), (8216 : Float), (8217 : Float), (8217 : Float), (8228 : Float), (8228 : Float), (8231 : Float), (8231 : Float), (8234 : Float), (8238 : Float), (8288 : Float), (8292 : Float), (8294 : Float), (8303 : Float), (8305 : Float), (8305 : Float), (8319 : Float), (8319 : Float), (8336 : Float), (8348 : Float), (8400 : Float), (8412 : Float), (8413 : Float), (8416 : Float), (8417 : Float), (8417 : Float), (8418 : Float), (8420 : Float), (8421 : Float), (8432 : Float), (11388 : Float), (11389 : Float), (11503 : Float), (11505 : Float), (11631 : Float), (11631 : Float), (11647 : Float), (11647 : Float), (11744 : Float), (11775 : Float), (11823 : Float), (11823 : Float), (12293 : Float), (12293 : Float), (12330 : Float), (12333 : Float), (12337 : Float), (12341 : Float), (12347 : Float), (12347 : Float), (12441 : Float), (12442 : Float), (12443 : Float), (12444 : Float), (12445 : Float), (12446 : Float), (12540 : Float), (12542 : Float), (40981 : Float), (40981 : Float), (42232 : Float), (42237 : Float), (42508 : Float), (42508 : Float), (42607 : Float), (42607 : Float), (42608 : Float), (42610 : Float), (42612 : Float), (42621 : Float), (42623 : Float), (42623 : Float), (42652 : Float), (42653 : Float), (42654 : Float), (42655 : Float), (42736 : Float), (42737 : Float), (42752 : Float), (42774 : Float), (42775 : Float), (42783 : Float), (42784 : Float), (42785 : Float), (42864 : Float), (42864 : Float), (42888 : Float), (42888 : Float), (42889 : Float), (42890 : Float), (42994 : Float), (42996 : Float), (43000 : Float), (43001 : Float), (43010 : Float), (43010 : Float), (43014 : Float), (43014 : Float), (43019 : Float), (43019 : Float), (43045 : Float), (43046 : Float), (43052 : Float), (43052 : Float), (43204 : Float), (43205 : Float), (43232 : Float), (43249 : Float), (43263 : Float), (43263 : Float), (43302 : Float), (43309 : Float), (43335 : Float), (43345 : Float), (43392 : Float), (43394 : Float), (43443 : Float), (43443 : Float), (43446 : Float), (43449 : Float), (43452 : Float), (43453 : Float), (43471 : Float), (43471 : Float), (43493 : Float), (43493 : Float), (43494 : Float), (43494 : Float), (43561 : Float), (43566 : Float), (43569 : Float), (43570 : Float), (43573 : Float), (43574 : Float), (43587 : Float), (43587 : Float), (43596 : Float), (43596 : Float), (43632 : Float), (43632 : Float), (43644 : Float), (43644 : Float), (43696 : Float), (43696 : Float), (43698 : Float), (43700 : Float), (43703 : Float), (43704 : Float), (43710 : Float), (43711 : Float), (43713 : Float), (43713 : Float), (43741 : Float), (43741 : Float), (43756 : Float), (43757 : Float), (43763 : Float), (43764 : Float), (43766 : Float), (43766 : Float), (43867 : Float), (43867 : Float), (43868 : Float), (43871 : Float), (43881 : Float), (43881 : Float), (43882 : Float), (43883 : Float), (44005 : Float), (44005 : Float), (44008 : Float), (44008 : Float), (44013 : Float), (44013 : Float), (64286 : Float), (64286 : Float), (64434 : Float), (64450 : Float), (65024 : Float), (65039 : Float), (65043 : Float), (65043 : Float), (65056 : Float), (65071 : Float), (65106 : Float), (65106 : Float), (65109 : Float), (65109 : Float), (65279 : Float), (65279 : Float), (65287 : Float), (65287 : Float), (65294 : Float), (65294 : Float), (65306 : Float), (65306 : Float), (65342 : Float), (65342 : Float), (65344 : Float), (65344 : Float), (65392 : Float), (65392 : Float), (65438 : Float), (65439 : Float), (65507 : Float), (65507 : Float), (65529 : Float), (65531 : Float), (66045 : Float), (66045 : Float), (66272 : Float), (66272 : Float), (66422 : Float), (66426 : Float), (67456 : Float), (67461 : Float), (67463 : Float), (67504 : Float), (67506 : Float), (67514 : Float), (68097 : Float), (68099 : Float), (68101 : Float), (68102 : Float), (68108 : Float), (68111 : Float), (68152 : Float), (68154 : Float), (68159 : Float), (68159 : Float), (68325 : Float), (68326 : Float), (68900 : Float), (68903 : Float), (68942 : Float), (68942 : Float), (68969 : Float), (68973 : Float), (68975 : Float), (68975 : Float), (69291 : Float), (69292 : Float), (69372 : Float), (69375 : Float), (69446 : Float), (69456 : Float), (69506 : Float), (69509 : Float), (69633 : Float), (69633 : Float), (69688 : Float), (69702 : Float), (69744 : Float), (69744 : Float), (69747 : Float), (69748 : Float), (69759 : Float), (69761 : Float), (69811 : Float), (69814 : Float), (69817 : Float), (69818 : Float), (69821 : Float), (69821 : Float), (69826 : Float), (69826 : Float), (69837 : Float), (69837 : Float), (69888 : Float), (69890 : Float), (69927 : Float), (69931 : Float), (69933 : Float), (69940 : Float), (70003 : Float), (70003 : Float), (70016 : Float), (70017 : Float), (70070 : Float), (70078 : Float), (70089 : Float), (70092 : Float), (70095 : Float), (70095 : Float), (70191 : Float), (70193 : Float), (70196 : Float), (70196 : Float), (70198 : Float), (70199 : Float), (70206 : Float), (70206 : Float), (70209 : Float), (70209 : Float), (70367 : Float), (70367 : Float), (70371 : Float), (70378 : Float), (70400 : Float), (70401 : Float), (70459 : Float), (70460 : Float), (70464 : Float), (70464 : Float), (70502 : Float), (70508 : Float), (70512 : Float), (70516 : Float), (70587 : Float), (70592 : Float), (70606 : Float), (70606 : Float), (70608 : Float), (70608 : Float), (70610 : Float), (70610 : Float), (70625 : Float), (70626 : Float), (70712 : Float), (70719 : Float), (70722 : Float), (70724 : Float), (70726 : Float), (70726 : Float), (70750 : Float), (70750 : Float), (70835 : Float), (70840 : Float), (70842 : Float), (70842 : Float), (70847 : Float), (70848 : Float), (70850 : Float), (70851 : Float), (71090 : Float), (71093 : Float), (71100 : Float), (71101 : Float), (71103 : Float), (71104 : Float), (71132 : Float), (71133 : Float), (71219 : Float), (71226 : Float), (71229 : Float), (71229 : Float), (71231 : Float), (71232 : Float), (71339 : Float), (71339 : Float), (71341 : Float), (71341 : Float), (71344 : Float), (71349 : Float), (71351 : Float), (71351 : Float), (71453 : Float), (71453 : Float), (71455 : Float), (71455 : Float), (71458 : Float), (71461 : Float), (71463 : Float), (71467 : Float), (71727 : Float), (71735 : Float), (71737 : Float), (71738 : Float), (71995 : Float), (71996 : Float), (71998 : Float), (71998 : Float), (72003 : Float), (72003 : Float), (72148 : Float), (72151 : Float), (72154 : Float), (72155 : Float), (72160 : Float), (72160 : Float), (72193 : Float), (72202 : Float), (72243 : Float), (72248 : Float), (72251 : Float), (72254 : Float), (72263 : Float), (72263 : Float), (72273 : Float), (72278 : Float), (72281 : Float), (72283 : Float), (72330 : Float), (72342 : Float), (72344 : Float), (72345 : Float), (72752 : Float), (72758 : Float), (72760 : Float), (72765 : Float), (72767 : Float), (72767 : Float), (72850 : Float), (72871 : Float), (72874 : Float), (72880 : Float), (72882 : Float), (72883 : Float), (72885 : Float), (72886 : Float), (73009 : Float), (73014 : Float), (73018 : Float), (73018 : Float), (73020 : Float), (73021 : Float), (73023 : Float), (73029 : Float), (73031 : Float), (73031 : Float), (73104 : Float), (73105 : Float), (73109 : Float), (73109 : Float), (73111 : Float), (73111 : Float), (73459 : Float), (73460 : Float), (73472 : Float), (73473 : Float), (73526 : Float), (73530 : Float), (73536 : Float), (73536 : Float), (73538 : Float), (73538 : Float), (73562 : Float), (73562 : Float), (78896 : Float), (78911 : Float), (78912 : Float), (78912 : Float), (78919 : Float), (78933 : Float), (90398 : Float), (90409 : Float), (90413 : Float), (90415 : Float), (92912 : Float), (92916 : Float), (92976 : Float), (92982 : Float), (92992 : Float), (92995 : Float), (93504 : Float), (93506 : Float), (93547 : Float), (93548 : Float), (94031 : Float), (94031 : Float), (94095 : Float), (94098 : Float), (94099 : Float), (94111 : Float), (94176 : Float), (94177 : Float), (94179 : Float), (94179 : Float), (94180 : Float), (94180 : Float), (110576 : Float), (110579 : Float), (110581 : Float), (110587 : Float), (110589 : Float), (110590 : Float), (113821 : Float), (113822 : Float), (113824 : Float), (113827 : Float), (118528 : Float), (118573 : Float), (118576 : Float), (118598 : Float), (119143 : Float), (119145 : Float), (119155 : Float), (119162 : Float), (119163 : Float), (119170 : Float), (119173 : Float), (119179 : Float), (119210 : Float), (119213 : Float), (119362 : Float), (119364 : Float), (121344 : Float), (121398 : Float), (121403 : Float), (121452 : Float), (121461 : Float), (121461 : Float), (121476 : Float), (121476 : Float), (121499 : Float), (121503 : Float), (121505 : Float), (121519 : Float), (122880 : Float), (122886 : Float), (122888 : Float), (122904 : Float), (122907 : Float), (122913 : Float), (122915 : Float), (122916 : Float), (122918 : Float), (122922 : Float), (122928 : Float), (122989 : Float), (123023 : Float), (123023 : Float), (123184 : Float), (123190 : Float), (123191 : Float), (123197 : Float), (123566 : Float), (123566 : Float), (123628 : Float), (123631 : Float), (124139 : Float), (124139 : Float), (124140 : Float), (124143 : Float), (124398 : Float), (124399 : Float), (125136 : Float), (125142 : Float), (125252 : Float), (125258 : Float), (125259 : Float), (125259 : Float), (127995 : Float), (127999 : Float), (917505 : Float), (917505 : Float), (917536 : Float), (917631 : Float), (917760 : Float), (917999 : Float)] : (Array Float))

inductive source_ml_temporary_acceptUnicodeCaseProperty_loop1_result where
  | source_ml_temporary_acceptUnicodeCaseProperty_loop1_done : source_ml_temporary_acceptUnicodeCaseProperty_loop1_result
  | source_ml_temporary_acceptUnicodeCaseProperty_loop1_return (ml_temporary_result1 : Bool) : source_ml_temporary_acceptUnicodeCaseProperty_loop1_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_temporary_acceptUnicodeCaseProperty_loop1 (point : Float) (ranges : (Array Float)) (first : Float) (last : Float) : source_ml_temporary_acceptUnicodeCaseProperty_loop1_result :=
  (if (decide (first < last)) then (let middle := (Float.floor ((first + last) / (2 : Float)));
    (let start := (match (let ml_index := (middle * (2 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then ranges[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
      (let «end» := (match (let ml_index := ((middle * (2 : Float)) + (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then ranges[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
        (if (decide (point < start)) then (let last_2 := middle;
          (source_ml_temporary_acceptUnicodeCaseProperty_loop1 point ranges first last_2)) else (if (decide (point > «end»)) then (let first_2 := (middle + (1 : Float));
          (source_ml_temporary_acceptUnicodeCaseProperty_loop1 point ranges first_2 last)) else (source_ml_temporary_acceptUnicodeCaseProperty_loop1_result.source_ml_temporary_acceptUnicodeCaseProperty_loop1_return true)))))) else source_ml_temporary_acceptUnicodeCaseProperty_loop1_result.source_ml_temporary_acceptUnicodeCaseProperty_loop1_done)

def acceptUnicodeCaseProperty (point : Float) (ranges : (Array Float)) : Bool :=
  (let first := (0 : Float);
    (let last := (Float.floor ((Float.ofNat ranges.size) / (2 : Float)));
      (let source_ml_source1_s1 := (source_ml_temporary_acceptUnicodeCaseProperty_loop1 point ranges first last);
        (match source_ml_source1_s1 with
          | source_ml_temporary_acceptUnicodeCaseProperty_loop1_result.source_ml_temporary_acceptUnicodeCaseProperty_loop1_done => false
          | source_ml_temporary_acceptUnicodeCaseProperty_loop1_result.source_ml_temporary_acceptUnicodeCaseProperty_loop1_return source_ml_temporary_result1 => source_ml_temporary_result1))))

inductive source_ml_temporary_readUnicodeCaseMapping_loop3_result where
  | source_ml_temporary_readUnicodeCaseMapping_loop3_done : source_ml_temporary_readUnicodeCaseMapping_loop3_result
  | source_ml_temporary_readUnicodeCaseMapping_loop3_return (ml_temporary_result3 : (Array Float)) : source_ml_temporary_readUnicodeCaseMapping_loop3_result
  deriving Repr, Inhabited

partial def source_ml_temporary_readUnicodeCaseMapping_loop3 (point : Float) (ranges : (Array Float)) (first : Float) (last : Float) : source_ml_temporary_readUnicodeCaseMapping_loop3_result :=
  (if (decide (first < last)) then (let middle := (Float.floor ((first + last) / (2 : Float)));
    (let start := (match (let ml_index := (middle * (4 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then ranges[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
      (let «end» := (match (let ml_index := ((middle * (4 : Float)) + (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then ranges[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
        (if (decide (point < start)) then (let last_2 := middle;
          (source_ml_temporary_readUnicodeCaseMapping_loop3 point ranges first last_2)) else (if (decide (point > «end»)) then (let first_2 := (middle + (1 : Float));
          (source_ml_temporary_readUnicodeCaseMapping_loop3 point ranges first_2 last)) else (let stride := (match (let ml_index := ((middle * (4 : Float)) + (2 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then ranges[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (1 : Float));
          (let delta := (match (let ml_index := ((middle * (4 : Float)) + (3 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then ranges[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float));
            (source_ml_temporary_readUnicodeCaseMapping_loop3_result.source_ml_temporary_readUnicodeCaseMapping_loop3_return (#[(if ((ml_float_rem (point - start) stride) == (0 : Float)) then (point + delta) else point)] : (Array Float)))))))))) else source_ml_temporary_readUnicodeCaseMapping_loop3_result.source_ml_temporary_readUnicodeCaseMapping_loop3_done)

inductive source_ml_temporary_readUnicodeCaseMapping_loop2_result where
  | source_ml_temporary_readUnicodeCaseMapping_loop2_done (first : Float) (last : Float) : source_ml_temporary_readUnicodeCaseMapping_loop2_result
  | source_ml_temporary_readUnicodeCaseMapping_loop2_return (ml_temporary_result3 : (Array Float)) : source_ml_temporary_readUnicodeCaseMapping_loop2_result
  deriving Repr, Inhabited

partial def source_ml_temporary_readUnicodeCaseMapping_loop4 (values : (Array Float)) (offset : Float) (count : Float) (result : (Array Float)) («at» : Float) : (Array Float) :=
  (if (decide («at» < count)) then (let result_2 := (result ++ (#[(match (let ml_index := (offset + «at»); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then values[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float))] : (Array Float)));
    (let at_2 := («at» + (1 : Float));
      (source_ml_temporary_readUnicodeCaseMapping_loop4 values offset count result_2 at_2))) else result)

partial def source_ml_temporary_readUnicodeCaseMapping_loop2 (point : Float) (indexes : (Array Float)) (values : (Array Float)) (first : Float) (last : Float) : source_ml_temporary_readUnicodeCaseMapping_loop2_result :=
  (if (decide (first < last)) then (let middle := (Float.floor ((first + last) / (2 : Float)));
    (let key := (match (let ml_index := (middle * (3 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then indexes[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
      (if (decide (point < key)) then (let last_2 := middle;
        (source_ml_temporary_readUnicodeCaseMapping_loop2 point indexes values first last_2)) else (if (decide (point > key)) then (let first_2 := (middle + (1 : Float));
        (source_ml_temporary_readUnicodeCaseMapping_loop2 point indexes values first_2 last)) else (let offset := (match (let ml_index := ((middle * (3 : Float)) + (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then indexes[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float));
        (let count := (match (let ml_index := ((middle * (3 : Float)) + (2 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then indexes[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float));
          (let result := (#[] : (Array Float));
            (let «at» := (0 : Float);
              (let result_2 := (source_ml_temporary_readUnicodeCaseMapping_loop4 values offset count result «at»);
                (source_ml_temporary_readUnicodeCaseMapping_loop2_result.source_ml_temporary_readUnicodeCaseMapping_loop2_return result_2)))))))))) else (source_ml_temporary_readUnicodeCaseMapping_loop2_result.source_ml_temporary_readUnicodeCaseMapping_loop2_done first last))

def readUnicodeCaseMapping (point : Float) (ranges : (Array Float)) (indexes : (Array Float)) (values : (Array Float)) : (Array Float) :=
  (let first := (0 : Float);
    (let last := (Float.floor ((Float.ofNat indexes.size) / (3 : Float)));
      (let source_ml_source1_s2 := (source_ml_temporary_readUnicodeCaseMapping_loop2 point indexes values first last);
        (match source_ml_source1_s2 with
          | source_ml_temporary_readUnicodeCaseMapping_loop2_result.source_ml_temporary_readUnicodeCaseMapping_loop2_done first_2 last_2 => (let first_3 := (0 : Float);
              (let last_3 := (Float.floor ((Float.ofNat ranges.size) / (4 : Float)));
                (let source_ml_source1_s3 := (source_ml_temporary_readUnicodeCaseMapping_loop3 point ranges first_3 last_3);
                  (match source_ml_source1_s3 with
                    | source_ml_temporary_readUnicodeCaseMapping_loop3_result.source_ml_temporary_readUnicodeCaseMapping_loop3_done => (#[point] : (Array Float))
                    | source_ml_temporary_readUnicodeCaseMapping_loop3_result.source_ml_temporary_readUnicodeCaseMapping_loop3_return source_ml_temporary_result3 => source_ml_temporary_result3))))
          | source_ml_temporary_readUnicodeCaseMapping_loop2_result.source_ml_temporary_readUnicodeCaseMapping_loop2_return source_ml_temporary_result3_2 => source_ml_temporary_result3_2))))

partial def source_ml_temporary_mapUnicodeCaseUtf16Columns_loop5 (text : (Array Float)) (points : (Array Float)) («at» : Float) (source_ml_source2_out : List (Array Nat)) : source_ml_source2_io1 :=
  (if (decide («at» < (Float.ofNat text.size))) then (let first := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if (((!(ml_is_safe_integer first)) || (decide (first < (0 : Float)))) || (decide (first > (65535 : Float)))) then (source_ml_source2_io1.source_ml_source2_io1_abort source_ml_source2_out (#[85, 110, 105, 99, 111, 100, 101, 32, 99, 97, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 115] : Array Nat) (#[69, 114, 114, 111, 114] : Array Nat) true) else (let second := (match (let ml_index := («at» + (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then text[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
      (if (((((decide (first ≥ (55296 : Float))) && (decide (first ≤ (56319 : Float)))) && (ml_is_safe_integer second)) && (decide (second ≥ (56320 : Float)))) && (decide (second ≤ (57343 : Float)))) then (let points_2 := (points ++ (#[((((65536 : Float) + ((first - (55296 : Float)) * (1024 : Float))) + second) - (56320 : Float))] : (Array Float)));
        (let at_2 := («at» + (1 : Float));
          (let at_3 := (at_2 + (1 : Float));
            (source_ml_temporary_mapUnicodeCaseUtf16Columns_loop5 text points_2 at_3 source_ml_source2_out)))) else (let points_3 := (points ++ (#[first] : (Array Float)));
        (let at_4 := («at» + (1 : Float));
          (source_ml_temporary_mapUnicodeCaseUtf16Columns_loop5 text points_3 at_4 source_ml_source2_out))))))) else (source_ml_source2_io1.source_ml_source2_io1_mk source_ml_source2_out points))

partial def source_ml_temporary_mapUnicodeCaseUtf16Columns_loop6 (cased : (Array Float)) (ignorable : (Array Float)) (points : (Array Float)) (following : (Array Bool)) (next : Bool) («at» : Float) : (Array Bool) :=
  (if (decide («at» ≥ (0 : Float))) then (let point := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then points[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float));
    (let following_2 := (following ++ (#[next] : (Array Bool)));
      (if (!(acceptUnicodeCaseProperty point ignorable)) then (let next_2 := (acceptUnicodeCaseProperty point cased);
        (let at_2 := («at» - (1 : Float));
          (source_ml_temporary_mapUnicodeCaseUtf16Columns_loop6 cased ignorable points following_2 next_2 at_2))) else (let at_3 := («at» - (1 : Float));
        (source_ml_temporary_mapUnicodeCaseUtf16Columns_loop6 cased ignorable points following_2 next at_3))))) else following)

partial def source_ml_temporary_mapUnicodeCaseUtf16Columns_loop8 (result : (Array Float)) (mapped : (Array Float)) (index : Float) : (Array Float) :=
  (if (decide (index < (Float.ofNat mapped.size))) then (let value := (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then mapped[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float));
    (if (decide (value < (65536 : Float))) then (let result_2 := (result ++ (#[value] : (Array Float)));
      (let index_2 := (index + (1 : Float));
        (source_ml_temporary_mapUnicodeCaseUtf16Columns_loop8 result_2 mapped index_2))) else (let result_3 := (result ++ (#[((55296 : Float) + (Float.floor ((value - (65536 : Float)) / (1024 : Float))))] : (Array Float)));
      (let result_4 := (result_3 ++ (#[((56320 : Float) + (ml_float_rem (value - (65536 : Float)) (1024 : Float)))] : (Array Float)));
        (let index_3 := (index + (1 : Float));
          (source_ml_temporary_mapUnicodeCaseUtf16Columns_loop8 result_4 mapped index_3)))))) else result)

partial def source_ml_temporary_mapUnicodeCaseUtf16Columns_loop7 (upper : Bool) (ranges : (Array Float)) (indexes : (Array Float)) (values : (Array Float)) (cased : (Array Float)) (ignorable : (Array Float)) (points : (Array Float)) (following : (Array Bool)) (result : (Array Float)) (previous : Bool) («at» : Float) : (Array Float) :=
  (if (decide («at» < (Float.ofNat points.size))) then (let point := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then points[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Float));
    (let finalSigma := ((((!upper) && (point == (931 : Float))) && previous) && (!(match (let ml_index := (((Float.ofNat points.size) - (1 : Float)) - «at»); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then following[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => false)));
      (let mapped := (if finalSigma then (#[(962 : Float)] : (Array Float)) else (readUnicodeCaseMapping point ranges indexes values));
        (let index := (0 : Float);
          (let result_2 := (source_ml_temporary_mapUnicodeCaseUtf16Columns_loop8 result mapped index);
            (if (!(acceptUnicodeCaseProperty point ignorable)) then (let previous_2 := (acceptUnicodeCaseProperty point cased);
              (let at_2 := («at» + (1 : Float));
                (source_ml_temporary_mapUnicodeCaseUtf16Columns_loop7 upper ranges indexes values cased ignorable points following result_2 previous_2 at_2))) else (let at_3 := («at» + (1 : Float));
              (source_ml_temporary_mapUnicodeCaseUtf16Columns_loop7 upper ranges indexes values cased ignorable points following result_2 previous at_3)))))))) else result)

def mapUnicodeCaseUtf16Columns (text : (Array Float)) (upper : Bool) (ranges : (Array Float)) (indexes : (Array Float)) (values : (Array Float)) (cased : (Array Float)) (ignorable : (Array Float)) (source_ml_source2_out : List (Array Nat)) : source_ml_source2_io1 :=
  (let points := (#[] : (Array Float));
    (let «at» := (0 : Float);
      (match (source_ml_temporary_mapUnicodeCaseUtf16Columns_loop5 text points «at» source_ml_source2_out) with
        | source_ml_source2_io1.source_ml_source2_io1_mk source_ml_source2_o1 source_ml_source2_v2 => (let points_2 := source_ml_source2_v2;
            (source_ml_source2_io1.source_ml_source2_io1_mk source_ml_source2_o1 (let following := (#[] : (Array Bool));
              (let next := false;
                (let at_2 := ((Float.ofNat points_2.size) - (1 : Float));
                  (let following_2 := (source_ml_temporary_mapUnicodeCaseUtf16Columns_loop6 cased ignorable points_2 following next at_2);
                    (let result := (#[] : (Array Float));
                      (let previous := false;
                        (let at_3 := (0 : Float);
                          (let result_2 := (source_ml_temporary_mapUnicodeCaseUtf16Columns_loop7 upper ranges indexes values cased ignorable points_2 following_2 result previous at_3);
                            result_2))))))))))
        | source_ml_source2_io1.source_ml_source2_io1_abort source_ml_source2_o3 source_ml_source2_m4 source_ml_source2_exception_class5 source_ml_source2_exception_present6 => (source_ml_source2_io1.source_ml_source2_io1_abort source_ml_source2_o3 source_ml_source2_m4 source_ml_source2_exception_class5 source_ml_source2_exception_present6))))

def mapUnicodeCaseUtf16Text (text : (Array Float)) (upper : Bool) (source_ml_source2_out : List (Array Nat)) : source_ml_source2_io1 :=
  (let ranges := (if upper then readUnicodeCaseUpperRanges else readUnicodeCaseLowerRanges);
    (let indexes := (if upper then readUnicodeCaseUpperIndexes else readUnicodeCaseLowerIndexes);
      (let values := (if upper then readUnicodeCaseUpperValues else readUnicodeCaseLowerValues);
        (mapUnicodeCaseUtf16Columns text upper ranges indexes values readUnicodeCaseCasedRanges readUnicodeCaseIgnorableRanges source_ml_source2_out))))

def ml_source_exception_observation : ExceptT ((Array Nat) × (Array Nat) × Bool) IO Unit := do
  pure ()

def main : IO Unit := do
  let ml_result ← ml_source_exception_observation.run
  match ml_result with
  | .ok _ => pure ()
  | .error ml_error => throw (IO.userError (String.ofList ((MLUtf16Text.displayPoints ((ml_error.2.1).map Float.ofNat)).toList.map (fun ml_point => Char.ofNat ml_point.toUInt64.toNat))))

def mapCase := mapUnicodeCaseUtf16Text
end MLUnicodeCase
