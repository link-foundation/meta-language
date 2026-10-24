namespace MLBigIntegerConversion
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


def acceptJavaScriptWhitespaceUnit (unit : Float) : Bool :=
  ((((((((((((decide (unit ≥ (9 : Float))) && (decide (unit ≤ (13 : Float)))) || (unit == (32 : Float))) || (unit == (160 : Float))) || (unit == (5760 : Float))) || ((decide (unit ≥ (8192 : Float))) && (decide (unit ≤ (8202 : Float))))) || (unit == (8232 : Float))) || (unit == (8233 : Float))) || (unit == (8239 : Float))) || (unit == (8287 : Float))) || (unit == (12288 : Float))) || (unit == (65279 : Float)))

def readJavaScriptNumericDigitUnit (unit : Float) : Float :=
  (if ((decide (unit ≥ (48 : Float))) && (decide (unit ≤ (57 : Float)))) then (unit - (48 : Float)) else (if ((decide (unit ≥ (65 : Float))) && (decide (unit ≤ (70 : Float)))) then (unit - (55 : Float)) else (if ((decide (unit ≥ (97 : Float))) && (decide (unit ≤ (102 : Float)))) then (unit - (87 : Float)) else (-1 : Float))))

def readStringNumberDigit (unit : Float) : Float :=
  (readJavaScriptNumericDigitUnit unit)

def readJavaScriptNumberBuiltinOperation (name : (Array Nat)) : (Array Nat) :=
  (if ((MLUtf16Text.compare ((name).map Float.ofNat) (((#[78, 117, 109, 98, 101, 114] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114] : Array Nat) else (if ((MLUtf16Text.compare ((name).map Float.ofNat) (((#[105, 115, 70, 105, 110, 105, 116, 101] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[116, 101, 115, 116, 67, 111, 101, 114, 99, 101, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 70, 105, 110, 105, 116, 101] : Array Nat) else (if ((MLUtf16Text.compare ((name).map Float.ofNat) (((#[105, 115, 78, 97, 78] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[116, 101, 115, 116, 67, 111, 101, 114, 99, 101, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 78, 97, 78] : Array Nat) else (#[] : Array Nat))))

def readJavaScriptNumberConversionResult (operation : (Array Nat)) : (Array Nat) :=
  (if (((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (#[102, 108, 111, 97, 116] : Array Nat) else (if (((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[116, 101, 115, 116, 67, 111, 101, 114, 99, 101, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 70, 105, 110, 105, 116, 101] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[116, 101, 115, 116, 67, 111, 101, 114, 99, 101, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 78, 97, 78] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (#[98, 111, 111, 108] : Array Nat) else (#[] : Array Nat)))

def readJavaScriptNumberConversionForm (kind : (Array Nat)) : (Array Nat) :=
  (if ((((((((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[102, 108, 111, 97, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[98, 111, 111, 108] : Array Nat)).map Float.ofNat)) == (0 : Float))) || ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[115, 116, 114, 105, 110, 103] : Array Nat)).map Float.ofNat)) == (0 : Float))) || ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[117, 110, 105, 116] : Array Nat)).map Float.ofNat)) == (0 : Float))) || ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[105, 110, 116] : Array Nat)).map Float.ofNat)) == (0 : Float))) || ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[110, 97, 116] : Array Nat)).map Float.ofNat)) == (0 : Float))) || ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[111, 112, 116, 105, 111, 110] : Array Nat)).map Float.ofNat)) == (0 : Float))) then kind else (#[117, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100] : Array Nat))

def acceptLinkedJavaScriptNumberConversion (kind : (Array Nat)) (operation : (Array Nat)) : Bool :=
  (((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[109, 97, 116, 104] : Array Nat)).map Float.ofNat)) == (0 : Float)) && (((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114] : Array Nat)).map Float.ofNat)) == (0 : Float))))

def readJavaScriptNumberConversionFailure (operation : (Array Nat)) (kind : (Array Nat)) : (Array Nat) :=
  (if (((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114] : Array Nat)).map Float.ofNat)) == (0 : Float)) && (((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[105, 110, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[110, 97, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)))) then (#[67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 110, 117, 109, 98, 101, 114] : Array Nat) else (#[] : Array Nat))

def requireJavaScriptNumberConversionUnits (kind : (Array Nat)) (operation : (Array Nat)) (argument : (Array Nat)) : Bool :=
  ((acceptLinkedJavaScriptNumberConversion kind operation) && ((MLUtf16Text.compare ((argument).map Float.ofNat) (((#[115, 116, 114, 105, 110, 103] : Array Nat)).map Float.ofNat)) == (0 : Float)))

def readJavaScriptPrimitiveNumberOperand (language : (Array Nat)) (kind : (Array Nat)) (element : (Array Nat)) : (Array Nat) :=
  (if ((MLUtf16Text.compare ((language).map Float.ofNat) (((#[74, 97, 118, 97, 83, 99, 114, 105, 112, 116] : Array Nat)).map Float.ofNat)) != (0 : Float)) then (#[] : Array Nat) else (if ((((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[98, 111, 111, 108] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[115, 116, 114, 105, 110, 103] : Array Nat)).map Float.ofNat)) == (0 : Float))) || ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[117, 110, 105, 116] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (#[99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114] : Array Nat) else (if (((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[111, 112, 116, 105, 111, 110] : Array Nat)).map Float.ofNat)) == (0 : Float)) && ((MLUtf16Text.compare ((element).map Float.ofNat) (((#[115, 116, 114, 105, 110, 103] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (#[99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114] : Array Nat) else (#[] : Array Nat))))

def readJavaScriptPrimitiveNumericInferenceKind (kind : (Array Nat)) : (Array Nat) :=
  (if ((((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[98, 111, 111, 108] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[115, 116, 114, 105, 110, 103] : Array Nat)).map Float.ofNat)) == (0 : Float))) || ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[117, 110, 105, 116] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (#[102, 108, 111, 97, 116] : Array Nat) else (#[] : Array Nat))

def readJavaScriptPrimitiveUnaryOperation (symbol : (Array Nat)) : (Array Nat) :=
  (if ((MLUtf16Text.compare ((symbol).map Float.ofNat) (((#[43] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114] : Array Nat) else (#[] : Array Nat))

def readJavaScriptMathematicalConversionForm (language : (Array Nat)) (operation : (Array Nat)) : (Array Nat) :=
  (if ((MLUtf16Text.compare ((language).map Float.ofNat) (((#[74, 97, 118, 97, 83, 99, 114, 105, 112, 116] : Array Nat)).map Float.ofNat)) != (0 : Float)) then (#[] : Array Nat) else (if ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[112, 111, 119, 101, 114, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 65, 114, 103, 117, 109, 101, 110, 116, 115] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[98, 105, 110, 97, 114, 121] : Array Nat) else (if (((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[109, 97, 120] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[109, 105, 110] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (#[118, 97, 114, 105, 97, 100, 105, 99] : Array Nat) else (if ((((((((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[97, 98, 115] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[102, 108, 111, 111, 114] : Array Nat)).map Float.ofNat)) == (0 : Float))) || ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[99, 101, 105, 108] : Array Nat)).map Float.ofNat)) == (0 : Float))) || ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[116, 114, 117, 110, 99] : Array Nat)).map Float.ofNat)) == (0 : Float))) || ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[114, 111, 117, 110, 100] : Array Nat)).map Float.ofNat)) == (0 : Float))) || ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[115, 105, 103, 110] : Array Nat)).map Float.ofNat)) == (0 : Float))) || ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[115, 113, 114, 116] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (#[117, 110, 97, 114, 121] : Array Nat) else (#[] : Array Nat)))))

def renderJavaScriptNumberScalarConversion (format : (Array Nat)) (operation : (Array Nat)) (form : (Array Nat)) (value : (Array Nat)) (utf16 : Bool) : (Array Nat) :=
  (if (((!(acceptLinkedJavaScriptNumberConversion (#[109, 97, 116, 104] : Array Nat) operation)) || ((MLUtf16Text.compare (((readJavaScriptNumberConversionForm form)).map Float.ofNat) (((#[117, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100] : Array Nat)).map Float.ofNat)) == (0 : Float))) || ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[111, 112, 116, 105, 111, 110] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (#[] : Array Nat) else (if ((MLUtf16Text.compare ((format).map Float.ofNat) (((#[74, 97, 118, 97, 83, 99, 114, 105, 112, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (if ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((#[78, 117, 109, 98, 101, 114, 40] : Array Nat) ++ value) ++ (#[41] : Array Nat)) else (((#[40, 43, 40] : Array Nat) ++ value) ++ (#[41, 41] : Array Nat))) else (if (((MLUtf16Text.compare ((format).map Float.ofNat) (((#[76, 101, 97, 110] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((format).map Float.ofNat) (((#[82, 111, 99, 113] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (if (!((readJavaScriptNumberConversionFailure operation form)).isEmpty) then (#[] : Array Nat) else (if ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[102, 108, 111, 97, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then value else (if ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[117, 110, 105, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (if ((MLUtf16Text.compare ((format).map Float.ofNat) (((#[76, 101, 97, 110] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((#[40, 108, 101, 116, 32, 95, 32, 58, 61, 32] : Array Nat) ++ value) ++ (#[59, 32, 40, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 47, 32, 48, 41, 41] : Array Nat)) else (((#[40, 108, 101, 116, 32, 95, 32, 58, 61, 32] : Array Nat) ++ value) ++ (#[32, 105, 110, 32, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 110, 97, 110, 41] : Array Nat))) else (if ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[98, 111, 111, 108] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (if ((MLUtf16Text.compare ((format).map Float.ofNat) (((#[76, 101, 97, 110] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((#[40, 105, 102, 32] : Array Nat) ++ value) ++ (#[32, 116, 104, 101, 110, 32, 40, 49, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 101, 108, 115, 101, 32, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 41] : Array Nat)) else (((#[40, 105, 102, 32] : Array Nat) ++ value) ++ (#[32, 116, 104, 101, 110, 32, 49, 37, 102, 108, 111, 97, 116, 32, 101, 108, 115, 101, 32, 48, 37, 102, 108, 111, 97, 116, 41] : Array Nat))) else (if (((MLUtf16Text.compare ((form).map Float.ofNat) (((#[105, 110, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[110, 97, 116] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (((#[40, 77, 76, 83, 116, 114, 105, 110, 103, 78, 117, 109, 98, 101, 114, 46, 99, 111, 110, 118, 101, 114, 116, 73, 110, 116, 101, 103, 101, 114, 32] : Array Nat) ++ (if ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[110, 97, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (if ((MLUtf16Text.compare ((format).map Float.ofNat) (((#[76, 101, 97, 110] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((#[40, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32] : Array Nat) ++ value) ++ (#[41] : Array Nat)) else (((#[40, 90, 46, 111, 102, 95, 78, 32] : Array Nat) ++ value) ++ (#[41] : Array Nat))) else value)) ++ (#[41] : Array Nat)) else (if ((MLUtf16Text.compare ((format).map Float.ofNat) (((#[76, 101, 97, 110] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((#[40, 77, 76, 83, 116, 114, 105, 110, 103, 78, 117, 109, 98, 101, 114, 46, 99, 111, 110, 118, 101, 114, 116, 85, 110, 105, 116, 115, 32, 40, 83, 116, 114, 105, 110, 103, 46, 102, 111, 108, 100, 108, 32, 40, 102, 117, 110, 32, 40, 117, 110, 105, 116, 115, 32, 58, 32, 65, 114, 114, 97, 121, 32, 70, 108, 111, 97, 116, 41, 32, 99, 32, 61, 62, 32, 105, 102, 32, 99, 46, 116, 111, 78, 97, 116, 32, 60, 32, 54, 53, 53, 51, 54, 32, 116, 104, 101, 110, 32, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 99, 46, 116, 111, 78, 97, 116, 41, 32, 101, 108, 115, 101, 32, 40, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 53, 50, 57, 54, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 47, 32, 49, 48, 50, 52, 41, 41, 41, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 54, 51, 50, 48, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 37, 32, 49, 48, 50, 52, 41, 41, 41, 32, 35, 91, 93, 32] : Array Nat) ++ value) ++ (#[41, 41] : Array Nat)) else (((#[40, 77, 76, 83, 116, 114, 105, 110, 103, 78, 117, 109, 98, 101, 114, 46, 99, 111, 110, 118, 101, 114, 116, 85, 116, 102, 56, 32, 40, 40, 102, 105, 120, 32, 109, 108, 95, 110, 117, 109, 101, 114, 105, 99, 95, 98, 121, 116, 101, 115, 32, 40, 116, 101, 120, 116, 32, 58, 32, 115, 116, 114, 105, 110, 103, 41, 32, 58, 32, 108, 105, 115, 116, 32, 102, 108, 111, 97, 116, 32, 58, 61, 32, 109, 97, 116, 99, 104, 32, 116, 101, 120, 116, 32, 119, 105, 116, 104, 32, 69, 109, 112, 116, 121, 83, 116, 114, 105, 110, 103, 32, 61, 62, 32, 110, 105, 108, 32, 124, 32, 83, 116, 114, 105, 110, 103, 32, 99, 32, 114, 101, 115, 116, 32, 61, 62, 32, 99, 111, 110, 115, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 111, 102, 95, 117, 105, 110, 116, 54, 51, 32, 40, 85, 105, 110, 116, 54, 51, 46, 111, 102, 95, 90, 32, 40, 90, 46, 111, 102, 95, 110, 97, 116, 32, 40, 65, 115, 99, 105, 105, 46, 110, 97, 116, 95, 111, 102, 95, 97, 115, 99, 105, 105, 32, 99, 41, 41, 41, 41, 32, 40, 109, 108, 95, 110, 117, 109, 101, 114, 105, 99, 95, 98, 121, 116, 101, 115, 32, 114, 101, 115, 116, 41, 32, 101, 110, 100, 41, 32] : Array Nat) ++ value) ++ (#[41, 41] : Array Nat)))))))) else (if ((MLUtf16Text.compare ((format).map Float.ofNat) (((#[82, 117, 115, 116] : Array Nat)).map Float.ofNat)) != (0 : Float)) then (#[] : Array Nat) else (if ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[102, 108, 111, 97, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then value else (if ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[98, 111, 111, 108] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((#[40, 105, 102, 32] : Array Nat) ++ value) ++ (#[32, 123, 32, 49, 46, 48, 102, 54, 52, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 48, 46, 48, 102, 54, 52, 32, 125, 41] : Array Nat)) else (if ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[117, 110, 105, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((#[123, 32, 108, 101, 116, 32, 95, 32, 61, 32] : Array Nat) ++ value) ++ (#[59, 32, 102, 54, 52, 58, 58, 78, 65, 78, 32, 125] : Array Nat)) else (if (((MLUtf16Text.compare ((form).map Float.ofNat) (((#[105, 110, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[110, 97, 116] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (if ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((#[123, 32, 108, 101, 116, 32, 95, 32, 61, 32] : Array Nat) ++ value) ++ (#[59, 32, 99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 115, 111, 117, 114, 99, 101, 95, 101, 120, 99, 101, 112, 116, 105, 111, 110, 58, 58, 83, 111, 117, 114, 99, 101, 69, 120, 99, 101, 112, 116, 105, 111, 110, 58, 58, 102, 114, 111, 109, 95, 116, 101, 120, 116, 40, 34, 84, 121, 112, 101, 69, 114, 114, 111, 114, 34, 44, 32, 34, 67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 110, 117, 109, 98, 101, 114, 34, 41, 46, 114, 97, 105, 115, 101, 40, 41, 32, 125] : Array Nat)) else (((#[99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 115, 116, 114, 105, 110, 103, 95, 110, 117, 109, 98, 101, 114, 58, 58, 99, 111, 110, 118, 101, 114, 116, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 110, 117, 109, 98, 101, 114, 40] : Array Nat) ++ value) ++ (#[41] : Array Nat))) else (let units := (if utf16 then (((#[40] : Array Nat) ++ value) ++ (#[41, 46, 48, 46, 105, 116, 101, 114, 40, 41, 46, 99, 111, 112, 105, 101, 100, 40, 41] : Array Nat)) else (((#[40] : Array Nat) ++ value) ++ (#[41, 46, 101, 110, 99, 111, 100, 101, 95, 117, 116, 102, 49, 54, 40, 41] : Array Nat)));
    (((#[99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 115, 116, 114, 105, 110, 103, 95, 110, 117, 109, 98, 101, 114, 58, 58, 99, 111, 110, 118, 101, 114, 116, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 115, 116, 114, 105, 110, 103, 95, 110, 117, 109, 98, 101, 114, 95, 117, 110, 105, 116, 115, 40, 38] : Array Nat) ++ units) ++ (#[46, 109, 97, 112, 40, 102, 54, 52, 58, 58, 102, 114, 111, 109, 41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101, 99, 60, 102, 54, 52, 62, 62, 40, 41, 41] : Array Nat)))))))))))

def readJavaScriptNumberConversionStatement : (Array Nat) :=
  (#[69, 67, 77, 65, 83, 99, 114, 105, 112, 116, 32, 78, 117, 109, 98, 101, 114, 32, 97, 110, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 112, 114, 101, 115, 101, 114, 118, 101, 32, 102, 117, 108, 108, 32, 85, 84, 70, 45, 49, 54, 32, 83, 116, 114, 105, 110, 103, 78, 117, 109, 101, 114, 105, 99, 76, 105, 116, 101, 114, 97, 108, 32, 115, 121, 110, 116, 97, 120, 44, 32, 101, 120, 97, 99, 116, 32, 98, 105, 110, 97, 114, 121, 54, 52, 32, 116, 105, 101, 115, 45, 116, 111, 45, 101, 118, 101, 110, 32, 114, 111, 117, 110, 100, 105, 110, 103, 44, 32, 115, 105, 103, 110, 101, 100, 32, 122, 101, 114, 111, 44, 32, 73, 110, 102, 105, 110, 105, 116, 121, 44, 32, 78, 97, 78, 44, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 97, 98, 115, 101, 110, 99, 101, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 101, 118, 97, 108, 117, 97, 116, 105, 111, 110, 32, 111, 114, 100, 101, 114, 59, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 66, 105, 103, 73, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 111, 117, 110, 100, 115, 32, 111, 110, 99, 101, 32, 97, 110, 100, 32, 105, 109, 112, 108, 105, 99, 105, 116, 32, 66, 105, 103, 73, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 116, 104, 114, 111, 119, 115] : Array Nat)

def convertJavaScriptBooleanBigInteger (value : Bool) : Int :=
  (if value then (1 : Int) else (0 : Int))

def acceptJavaScriptExponentiationBase («prefix» : (Array Nat)) (parenthesized : Bool) : Bool :=
  (parenthesized || (!(let ml_values := (#[(#[43] : Array Nat), (#[45] : Array Nat), (#[33] : Array Nat), (#[126] : Array Nat), (#[116, 121, 112, 101, 111, 102] : Array Nat), (#[118, 111, 105, 100] : Array Nat), (#[100, 101, 108, 101, 116, 101] : Array Nat), (#[97, 119, 97, 105, 116] : Array Nat)] : (Array (Array Nat))); let ml_search := «prefix»; let ml_start := (0 : Float); let ml_length := Float.ofNat ml_values.size; let ml_first := (MLUtf16Text.slicePosition ml_start ml_length).toUInt64.toNat; (ml_values.extract ml_first ml_values.size).any (fun ml_candidate => ((MLUtf16Text.compare ((ml_candidate).map Float.ofNat) ((ml_search).map Float.ofNat)) == (0 : Float))))))

def readJavaScriptBigIntegerExponentMessage : (Array Nat) :=
  (#[69, 120, 112, 111, 110, 101, 110, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101] : Array Nat)

def renderJavaScriptBigIntegerPower (target : (Array Nat)) (base : (Array Nat)) (exponent : (Array Nat)) : (Array Nat) :=
  (if ((MLUtf16Text.compare ((target).map Float.ofNat) (((#[74, 97, 118, 97, 83, 99, 114, 105, 112, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((((#[109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 112, 111, 119, 101, 114, 40] : Array Nat) ++ base) ++ (#[44] : Array Nat)) ++ exponent) ++ (#[41] : Array Nat)) else (if ((MLUtf16Text.compare ((target).map Float.ofNat) (((#[82, 117, 115, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((((#[99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 112, 111, 119, 101, 114, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 40] : Array Nat) ++ base) ++ (#[44] : Array Nat)) ++ exponent) ++ (#[41] : Array Nat)) else (if (((MLUtf16Text.compare ((target).map Float.ofNat) (((#[76, 101, 97, 110] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((target).map Float.ofNat) (((#[82, 111, 99, 113] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (((((#[40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 112, 111, 119, 101, 114, 32] : Array Nat) ++ base) ++ (#[32] : Array Nat)) ++ exponent) ++ (#[41] : Array Nat)) else (#[] : Array Nat))))

def acceptNonzeroJavaScriptIntegerLiteral (value : (Array Nat)) : Bool :=
  ((((MLUtf16Text.compare ((value).map Float.ofNat) (((#[] : Array Nat)).map Float.ofNat)) != (0 : Float)) && ((MLUtf16Text.compare ((value).map Float.ofNat) (((#[48] : Array Nat)).map Float.ofNat)) != (0 : Float))) && ((MLUtf16Text.compare ((value).map Float.ofNat) (((#[45, 48] : Array Nat)).map Float.ofNat)) != (0 : Float)))

def requireJavaScriptBigIntegerDivision (language : (Array Nat)) (operation : (Array Nat)) (kind : (Array Nat)) (literal : (Array Nat)) : Bool :=
  (((((MLUtf16Text.compare ((language).map Float.ofNat) (((#[74, 97, 118, 97, 83, 99, 114, 105, 112, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) && ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[105, 110, 116] : Array Nat)).map Float.ofNat)) == (0 : Float))) && (((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[100, 105, 118] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[114, 101, 109] : Array Nat)).map Float.ofNat)) == (0 : Float)))) && (!(acceptNonzeroJavaScriptIntegerLiteral literal)))

def readJavaScriptBigIntegerDivisionMessage : (Array Nat) :=
  (#[68, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32, 122, 101, 114, 111] : Array Nat)

def renderJavaScriptBigIntegerDivision (target : (Array Nat)) (left : (Array Nat)) (right : (Array Nat)) : (Array Nat) :=
  (if ((MLUtf16Text.compare ((target).map Float.ofNat) (((#[74, 97, 118, 97, 83, 99, 114, 105, 112, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((((#[109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 100, 105, 118, 105, 100, 101, 40] : Array Nat) ++ left) ++ (#[44] : Array Nat)) ++ right) ++ (#[41] : Array Nat)) else (if ((MLUtf16Text.compare ((target).map Float.ofNat) (((#[82, 117, 115, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((((#[99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 100, 105, 118, 105, 100, 101, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 40] : Array Nat) ++ left) ++ (#[44] : Array Nat)) ++ right) ++ (#[41] : Array Nat)) else (if (((MLUtf16Text.compare ((target).map Float.ofNat) (((#[76, 101, 97, 110] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((target).map Float.ofNat) (((#[82, 111, 99, 113] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (((((#[40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 100, 105, 118, 105, 100, 101, 32] : Array Nat) ++ left) ++ (#[32] : Array Nat)) ++ right) ++ (#[41] : Array Nat)) else (#[] : Array Nat))))

def readJavaScriptBigIntegerConversionForm (kind : (Array Nat)) : (Array Nat) :=
  (if (((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[105, 110, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[110, 97, 116] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (#[105, 100, 101, 110, 116, 105, 116, 121] : Array Nat) else (if ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[102, 108, 111, 97, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[110, 117, 109, 98, 101, 114] : Array Nat) else (if ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[115, 116, 114, 105, 110, 103] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[115, 116, 114, 105, 110, 103] : Array Nat) else (if ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[98, 111, 111, 108] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[98, 111, 111, 108, 101, 97, 110] : Array Nat) else (if ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[117, 110, 105, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[117, 110, 100, 101, 102, 105, 110, 101, 100] : Array Nat) else (if ((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[111, 112, 116, 105, 111, 110] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[111, 112, 116, 105, 111, 110, 97, 108] : Array Nat) else (#[117, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100] : Array Nat)))))))

def readJavaScriptBigIntegerConversionErrorClass (form : (Array Nat)) : (Array Nat) :=
  (if ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[115, 116, 114, 105, 110, 103] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114] : Array Nat) else (if ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[110, 117, 109, 98, 101, 114] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[82, 97, 110, 103, 101, 69, 114, 114, 111, 114] : Array Nat) else (if (((MLUtf16Text.compare ((form).map Float.ofNat) (((#[117, 110, 100, 101, 102, 105, 110, 101, 100] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[110, 117, 108, 108] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (#[84, 121, 112, 101, 69, 114, 114, 111, 114] : Array Nat) else (#[] : Array Nat))))

def readJavaScriptBigIntegerConversionMessagePieces (form : (Array Nat)) : (Array (Array Nat)) :=
  (if ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[110, 117, 109, 98, 101, 114] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[(#[84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32] : Array Nat), (#[32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114] : Array Nat)] : (Array (Array Nat))) else (if ((((MLUtf16Text.compare ((form).map Float.ofNat) (((#[115, 116, 114, 105, 110, 103] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[117, 110, 100, 101, 102, 105, 110, 101, 100] : Array Nat)).map Float.ofNat)) == (0 : Float))) || ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[110, 117, 108, 108] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (#[(#[67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32] : Array Nat), (#[32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116] : Array Nat)] : (Array (Array Nat))) else (#[] : (Array (Array Nat)))))

def renderJavaScriptBigIntegerConversionMessage (form : (Array Nat)) (value : (Array Nat)) : (Array Nat) :=
  (let pieces := (readJavaScriptBigIntegerConversionMessagePieces form);
    (if ((Float.ofNat pieces.size) == (2 : Float)) then (((match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then pieces[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (#[] : Array Nat)) ++ value) ++ (match (let ml_index := (1 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then pieces[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (#[] : Array Nat))) else (#[] : Array Nat)))

def readJavaScriptBigIntegerBuiltinOperation (name : (Array Nat)) : (Array Nat) :=
  (if ((MLUtf16Text.compare ((name).map Float.ofNat) (((#[66, 105, 103, 73, 110, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114] : Array Nat) else (#[] : Array Nat))

def readJavaScriptBigIntegerResultOperation (operation : (Array Nat)) : (Array Nat) :=
  (if ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[112, 111, 119, 101, 114, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 86, 97, 108, 117, 101] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[110, 117, 109, 98, 101, 114, 45, 112, 111, 119, 101, 114] : Array Nat) else (if ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[112, 111, 119, 101, 114, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 82, 101, 115, 117, 108, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[112, 111, 119, 101, 114] : Array Nat) else (if ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[100, 105, 118, 105, 100, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 82, 101, 115, 117, 108, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[100, 105, 118, 105, 115, 105, 111, 110] : Array Nat) else (if ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[112, 97, 114, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83, 116, 114, 105, 110, 103, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[115, 116, 114, 105, 110, 103] : Array Nat) else (if ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 66, 111, 111, 108, 101, 97, 110, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 86, 97, 108, 117, 101] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[98, 111, 111, 108, 101, 97, 110] : Array Nat) else (if ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 82, 101, 115, 117, 108, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[110, 117, 109, 98, 101, 114] : Array Nat) else (#[] : Array Nat)))))))

def acceptLinkedJavaScriptBigIntegerResult (kind : (Array Nat)) (operation : (Array Nat)) : Bool :=
  (((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[109, 97, 116, 104] : Array Nat)).map Float.ofNat)) == (0 : Float)) && ((MLUtf16Text.compare (((readJavaScriptBigIntegerResultOperation operation)).map Float.ofNat) (((#[] : Array Nat)).map Float.ofNat)) != (0 : Float)))

def renderJavaScriptNumberPower (target : (Array Nat)) (base : (Array Nat)) (exponent : (Array Nat)) : (Array Nat) :=
  (if ((MLUtf16Text.compare ((target).map Float.ofNat) (((#[74, 97, 118, 97, 83, 99, 114, 105, 112, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((((#[109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 112, 111, 119, 101, 114, 78, 117, 109, 98, 101, 114, 40] : Array Nat) ++ base) ++ (#[44] : Array Nat)) ++ exponent) ++ (#[41] : Array Nat)) else (if ((MLUtf16Text.compare ((target).map Float.ofNat) (((#[82, 117, 115, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((((#[99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 112, 111, 119, 101, 114, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 110, 117, 109, 98, 101, 114, 40] : Array Nat) ++ base) ++ (#[44] : Array Nat)) ++ exponent) ++ (#[41] : Array Nat)) else (if (((MLUtf16Text.compare ((target).map Float.ofNat) (((#[76, 101, 97, 110] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((target).map Float.ofNat) (((#[82, 111, 99, 113] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (((((#[40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 112, 111, 119, 101, 114, 78, 117, 109, 98, 101, 114, 32] : Array Nat) ++ base) ++ (#[32] : Array Nat)) ++ exponent) ++ (#[41] : Array Nat)) else (#[] : Array Nat))))

def renderJavaScriptBigIntegerResultExpression (target : (Array Nat)) (form : (Array Nat)) (value : (Array Nat)) (utf16 : Bool) : (Array Nat) :=
  (if ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[98, 111, 111, 108, 101, 97, 110] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (if ((MLUtf16Text.compare ((target).map Float.ofNat) (((#[74, 97, 118, 97, 83, 99, 114, 105, 112, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((#[109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 66, 111, 111, 108, 101, 97, 110, 40] : Array Nat) ++ value) ++ (#[41] : Array Nat)) else (if ((MLUtf16Text.compare ((target).map Float.ofNat) (((#[82, 117, 115, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((#[99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 99, 111, 110, 118, 101, 114, 116, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 98, 111, 111, 108, 101, 97, 110, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 40] : Array Nat) ++ value) ++ (#[41] : Array Nat)) else (if (((MLUtf16Text.compare ((target).map Float.ofNat) (((#[76, 101, 97, 110] : Array Nat)).map Float.ofNat)) == (0 : Float)) || ((MLUtf16Text.compare ((target).map Float.ofNat) (((#[82, 111, 99, 113] : Array Nat)).map Float.ofNat)) == (0 : Float))) then (((#[40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 66, 111, 111, 108, 101, 97, 110, 32] : Array Nat) ++ value) ++ (#[41] : Array Nat)) else (#[] : Array Nat)))) else (if (((MLUtf16Text.compare ((form).map Float.ofNat) (((#[115, 116, 114, 105, 110, 103] : Array Nat)).map Float.ofNat)) != (0 : Float)) && ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[110, 117, 109, 98, 101, 114] : Array Nat)).map Float.ofNat)) != (0 : Float))) then (#[] : Array Nat) else (if ((MLUtf16Text.compare ((target).map Float.ofNat) (((#[74, 97, 118, 97, 83, 99, 114, 105, 112, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (if ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[110, 117, 109, 98, 101, 114] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((#[109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 78, 117, 109, 98, 101, 114, 40] : Array Nat) ++ value) ++ (#[41] : Array Nat)) else (((#[40, 40, 109, 108, 95, 116, 101, 120, 116, 41, 61, 62, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 85, 110, 105, 116, 115, 40, 65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 40, 123, 108, 101, 110, 103, 116, 104, 58, 109, 108, 95, 116, 101, 120, 116, 46, 108, 101, 110, 103, 116, 104, 125, 44, 32, 40, 95, 44, 105, 110, 100, 101, 120, 41, 61, 62, 109, 108, 95, 116, 101, 120, 116, 46, 99, 104, 97, 114, 67, 111, 100, 101, 65, 116, 40, 105, 110, 100, 101, 120, 41, 41, 41, 41, 40] : Array Nat) ++ value) ++ (#[41] : Array Nat))) else (if ((MLUtf16Text.compare ((target).map Float.ofNat) (((#[82, 117, 115, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (if ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[110, 117, 109, 98, 101, 114] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((#[99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 99, 111, 110, 118, 101, 114, 116, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 110, 117, 109, 98, 101, 114, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 40] : Array Nat) ++ value) ++ (#[41] : Array Nat)) else (let units := (if utf16 then (((#[40] : Array Nat) ++ value) ++ (#[41, 46, 48, 46, 105, 116, 101, 114, 40, 41, 46, 99, 111, 112, 105, 101, 100, 40, 41] : Array Nat)) else (((#[40] : Array Nat) ++ value) ++ (#[41, 46, 101, 110, 99, 111, 100, 101, 95, 117, 116, 102, 49, 54, 40, 41] : Array Nat)));
    (((#[99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 99, 111, 110, 118, 101, 114, 116, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 115, 116, 114, 105, 110, 103, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 117, 110, 105, 116, 115, 40, 38] : Array Nat) ++ units) ++ (#[46, 109, 97, 112, 40, 102, 54, 52, 58, 58, 102, 114, 111, 109, 41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101, 99, 60, 102, 54, 52, 62, 62, 40, 41, 41] : Array Nat)))) else (if ((MLUtf16Text.compare ((target).map Float.ofNat) (((#[76, 101, 97, 110] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (if ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[110, 117, 109, 98, 101, 114] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((#[40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 78, 117, 109, 98, 101, 114, 32] : Array Nat) ++ value) ++ (#[41] : Array Nat)) else (((#[40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 85, 110, 105, 116, 115, 32, 40, 83, 116, 114, 105, 110, 103, 46, 102, 111, 108, 100, 108, 32, 40, 102, 117, 110, 32, 40, 117, 110, 105, 116, 115, 32, 58, 32, 65, 114, 114, 97, 121, 32, 70, 108, 111, 97, 116, 41, 32, 99, 32, 61, 62, 32, 105, 102, 32, 99, 46, 116, 111, 78, 97, 116, 32, 60, 32, 54, 53, 53, 51, 54, 32, 116, 104, 101, 110, 32, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 99, 46, 116, 111, 78, 97, 116, 41, 32, 101, 108, 115, 101, 32, 40, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 53, 50, 57, 54, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 47, 32, 49, 48, 50, 52, 41, 41, 41, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 54, 51, 50, 48, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 37, 32, 49, 48, 50, 52, 41, 41, 41, 32, 35, 91, 93, 32] : Array Nat) ++ value) ++ (#[41, 41] : Array Nat))) else (if ((MLUtf16Text.compare ((target).map Float.ofNat) (((#[82, 111, 99, 113] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (if ((MLUtf16Text.compare ((form).map Float.ofNat) (((#[110, 117, 109, 98, 101, 114] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (((#[40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 78, 117, 109, 98, 101, 114, 32] : Array Nat) ++ value) ++ (#[41] : Array Nat)) else (((#[40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 85, 116, 102, 56, 32, 40, 40, 102, 105, 120, 32, 109, 108, 95, 105, 110, 116, 101, 103, 101, 114, 95, 98, 121, 116, 101, 115, 32, 40, 116, 101, 120, 116, 32, 58, 32, 115, 116, 114, 105, 110, 103, 41, 32, 58, 32, 108, 105, 115, 116, 32, 102, 108, 111, 97, 116, 32, 58, 61, 32, 109, 97, 116, 99, 104, 32, 116, 101, 120, 116, 32, 119, 105, 116, 104, 32, 69, 109, 112, 116, 121, 83, 116, 114, 105, 110, 103, 32, 61, 62, 32, 110, 105, 108, 32, 124, 32, 83, 116, 114, 105, 110, 103, 32, 99, 32, 114, 101, 115, 116, 32, 61, 62, 32, 99, 111, 110, 115, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 111, 102, 95, 117, 105, 110, 116, 54, 51, 32, 40, 85, 105, 110, 116, 54, 51, 46, 111, 102, 95, 90, 32, 40, 90, 46, 111, 102, 95, 110, 97, 116, 32, 40, 65, 115, 99, 105, 105, 46, 110, 97, 116, 95, 111, 102, 95, 97, 115, 99, 105, 105, 32, 99, 41, 41, 41, 41, 32, 40, 109, 108, 95, 105, 110, 116, 101, 103, 101, 114, 95, 98, 121, 116, 101, 115, 32, 114, 101, 115, 116, 41, 32, 101, 110, 100, 41, 32] : Array Nat) ++ value) ++ (#[41, 41] : Array Nat))) else (#[] : Array Nat)))))))

inductive source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_result where
  | source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_done (units : (Array Float)) : source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_result
  | source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_return (ml_source1_temporary_result2 : (Array Float)) : source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_result
  deriving Repr, Inhabited

inductive source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4_result where
  | source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4_done (index : Float) (scalar : Float) : source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4_result
  | source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4_return (ml_source1_temporary_result4 : (Array Float)) : source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4 (bytes : (Array Float)) (index : Float) (scalar : Float) (remaining : Float) : source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4_result :=
  (if (decide (remaining > (0 : Float))) then (let next := (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then bytes[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if ((decide (next < (128 : Float))) || (decide (next > (191 : Float)))) then (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4_result.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4_return (#[(-1 : Float)] : (Array Float))) else (let scalar_2 := (((scalar * (64 : Float)) + next) - (128 : Float));
      (let index_2 := (index + (1 : Float));
        (let remaining_2 := (remaining - (1 : Float));
          (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4 bytes index_2 scalar_2 remaining_2)))))) else (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4_result.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4_done index scalar))

inductive source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3 where
  | source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_next (scalar : Float) (remaining : Float) (minimum : Float) : source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3
  | source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_return (ml_source1_temporary_result4 : (Array Float)) : source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2 (bytes : (Array Float)) (units : (Array Float)) (index : Float) : source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_result :=
  (if (decide (index < (Float.ofNat bytes.size))) then (let scalar := (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then bytes[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (let index_2 := (index + (1 : Float));
      (let remaining := (0 : Float);
        (let minimum := (0 : Float);
          (let source_ml_source2_s36 := (if ((decide (scalar ≥ (194 : Float))) && (decide (scalar ≤ (223 : Float)))) then (let scalar_2 := (scalar - (192 : Float));
            (let remaining_2 := (1 : Float);
              (let minimum_2 := (128 : Float);
                (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_2 remaining_2 minimum_2)))) else (if ((decide (scalar ≥ (224 : Float))) && (decide (scalar ≤ (239 : Float)))) then (let scalar_3 := (scalar - (224 : Float));
            (let remaining_3 := (2 : Float);
              (let minimum_3 := (2048 : Float);
                (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_3 remaining_3 minimum_3)))) else (if ((decide (scalar ≥ (240 : Float))) && (decide (scalar ≤ (244 : Float)))) then (let scalar_4 := (scalar - (240 : Float));
            (let remaining_4 := (3 : Float);
              (let minimum_4 := (65536 : Float);
                (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_4 remaining_4 minimum_4)))) else (if (decide (scalar ≥ (128 : Float))) then (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_return (#[(-1 : Float)] : (Array Float))) else (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_next scalar remaining minimum)))));
            (match source_ml_source2_s36 with
              | source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_5 remaining_5 minimum_5 => (let source_ml_source2_s37 := (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4 bytes index_2 scalar_5 remaining_5);
                  (match source_ml_source2_s37 with
                    | source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4_result.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4_done index_3 scalar_6 => (if (((decide (scalar_6 < minimum_5)) || (decide (scalar_6 > (1114111 : Float)))) || ((decide (scalar_6 ≥ (55296 : Float))) && (decide (scalar_6 ≤ (57343 : Float))))) then (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_result.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_return (#[(-1 : Float)] : (Array Float))) else (if (decide (scalar_6 ≤ (65535 : Float))) then (let units_2 := (units ++ (#[scalar_6] : (Array Float)));
                        (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2 bytes units_2 index_3)) else (let units_3 := (units ++ (#[((55296 : Float) + (Float.floor ((scalar_6 - (65536 : Float)) / (1024 : Float))))] : (Array Float)));
                        (let units_4 := (units_3 ++ (#[((56320 : Float) + (ml_float_rem (scalar_6 - (65536 : Float)) (1024 : Float)))] : (Array Float)));
                          (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2 bytes units_4 index_3)))))
                    | source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4_result.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4_return source_ml_source1_temporary_result4 => (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_result.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_return source_ml_source1_temporary_result4)))
              | source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_return source_ml_source1_temporary_result4_2 => (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_result.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_return source_ml_source1_temporary_result4_2))))))) else (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_result.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_done units))

inductive source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1_result where
  | source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1_done : source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1_result
  | source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1_return (ml_source1_temporary_result4 : (Array Float)) : source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1 (bytes : (Array Float)) (index : Float) : source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1_result :=
  (if (decide (index < (Float.ofNat bytes.size))) then (let unit := (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then bytes[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if (((!(ml_is_safe_integer unit)) || (decide (unit < (0 : Float)))) || (decide (unit > (255 : Float)))) then (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1_result.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1_return (#[(-1 : Float)] : (Array Float))) else (let index_2 := (index + (1 : Float));
      (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1 bytes index_2)))) else source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1_result.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1_done)

def readJavaScriptUtf8StringUnits (bytes : (Array Float)) : (Array Float) :=
  (let index := (0 : Float);
    (let source_ml_source2_s1 := (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1 bytes index);
      (match source_ml_source2_s1 with
        | source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1_result.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1_done => (let units := (#[] : (Array Float));
            (let index_2 := (0 : Float);
              (let source_ml_source2_s2 := (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2 bytes units index_2);
                (match source_ml_source2_s2 with
                  | source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_result.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_done units_2 => units_2
                  | source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_result.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop2_return source_ml_source1_temporary_result2 => source_ml_source1_temporary_result2))))
        | source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1_result.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop1_return source_ml_source1_temporary_result4 => source_ml_source1_temporary_result4)))

partial def source_ml_source1_temporary_readStringNumberDecimalPower_loop5 (exponent : Float) (value : Int) (base : Int) : Int :=
  (if (decide (exponent > (0 : Float))) then (let value_3 := (if ((ml_float_rem exponent (2 : Float)) == (1 : Float)) then (let value_2 := (value * base);
    value_2) else value);
    (let exponent_2 := (Float.floor (exponent / (2 : Float)));
      (if (decide (exponent_2 > (0 : Float))) then (let base_2 := (base * base);
        (source_ml_source1_temporary_readStringNumberDecimalPower_loop5 exponent_2 value_3 base_2)) else (source_ml_source1_temporary_readStringNumberDecimalPower_loop5 exponent_2 value_3 base)))) else value)

def readStringNumberDecimalPower (exponent : Float) : Int :=
  (let value := (1 : Int);
    (let base := (10 : Int);
      (let value_2 := (source_ml_source1_temporary_readStringNumberDecimalPower_loop5 exponent value base);
        value_2)))

partial def source_ml_source1_temporary_readStringNumberExactMantissa_loop6 (value : Int) (result : Float) (scale : Float) : Float :=
  (if (decide (value > (0 : Int))) then (let result_3 := (if ((Int.tmod value (2 : Int)) == (1 : Int)) then (let result_2 := (result + scale);
    result_2) else result);
    (let value_2 := (Int.tdiv value (2 : Int));
      (let scale_2 := (scale * (2 : Float));
        (source_ml_source1_temporary_readStringNumberExactMantissa_loop6 value_2 result_3 scale_2)))) else result)

def readStringNumberExactMantissa (value : Int) : Float :=
  (let result := (0 : Float);
    (let scale := (1 : Float);
      (let result_2 := (source_ml_source1_temporary_readStringNumberExactMantissa_loop6 value result scale);
        result_2)))

partial def source_ml_source1_temporary_readStringNumberBinaryPower_loop8 (exponent : Float) (result : Float) : Float :=
  (if (decide (exponent < (0 : Float))) then (let result_2 := (result / (2 : Float));
    (let exponent_2 := (exponent + (1 : Float));
      (source_ml_source1_temporary_readStringNumberBinaryPower_loop8 exponent_2 result_2))) else result)

inductive source_ml_source1_temporary_readStringNumberBinaryPower_loop7_result where
  | source_ml_source1_temporary_readStringNumberBinaryPower_loop7_done (exponent : Float) (result : Float) : source_ml_source1_temporary_readStringNumberBinaryPower_loop7_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_readStringNumberBinaryPower_loop7 (exponent : Float) (result : Float) : source_ml_source1_temporary_readStringNumberBinaryPower_loop7_result :=
  (if (decide (exponent > (0 : Float))) then (let result_2 := (result * (2 : Float));
    (let exponent_2 := (exponent - (1 : Float));
      (source_ml_source1_temporary_readStringNumberBinaryPower_loop7 exponent_2 result_2))) else (source_ml_source1_temporary_readStringNumberBinaryPower_loop7_result.source_ml_source1_temporary_readStringNumberBinaryPower_loop7_done exponent result))

def readStringNumberBinaryPower (exponent : Float) : Float :=
  (let result := (1 : Float);
    (let source_ml_source2_s3 := (source_ml_source1_temporary_readStringNumberBinaryPower_loop7 exponent result);
      (match source_ml_source2_s3 with
        | source_ml_source1_temporary_readStringNumberBinaryPower_loop7_result.source_ml_source1_temporary_readStringNumberBinaryPower_loop7_done exponent_2 result_2 => (let result_3 := (source_ml_source1_temporary_readStringNumberBinaryPower_loop8 exponent_2 result_2);
            result_3))))

inductive source_ml_source1_temporary_convertStringNumberRational_loop12_result where
  | source_ml_source1_temporary_convertStringNumberRational_loop12_done (quotient : Int) (remainder : Int) : source_ml_source1_temporary_convertStringNumberRational_loop12_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_source1_temporary_convertStringNumberRational_loop12 (quotient : Int) (remainder : Int) (scale : Int) (weight : Int) : source_ml_source1_temporary_convertStringNumberRational_loop12_result :=
  (if (decide (weight > (1 : Int))) then (let scale_2 := (Int.tdiv scale (2 : Int));
    (let weight_2 := (Int.tdiv weight (2 : Int));
      (if (decide (remainder ≥ scale_2)) then (let remainder_2 := (remainder - scale_2);
        (let quotient_2 := (quotient + weight_2);
          (source_ml_source1_temporary_convertStringNumberRational_loop12 quotient_2 remainder_2 scale_2 weight_2))) else (source_ml_source1_temporary_convertStringNumberRational_loop12 quotient remainder scale_2 weight_2)))) else (source_ml_source1_temporary_convertStringNumberRational_loop12_result.source_ml_source1_temporary_convertStringNumberRational_loop12_done quotient remainder))

inductive source_ml_source1_temporary_convertStringNumberRational_loop11_result where
  | source_ml_source1_temporary_convertStringNumberRational_loop11_done (scale : Int) (weight : Int) : source_ml_source1_temporary_convertStringNumberRational_loop11_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_source1_temporary_convertStringNumberRational_loop11 (remainder : Int) (scale : Int) (weight : Int) : source_ml_source1_temporary_convertStringNumberRational_loop11_result :=
  (if (decide (scale ≤ remainder)) then (let scale_2 := (scale * (2 : Int));
    (let weight_2 := (weight * (2 : Int));
      (source_ml_source1_temporary_convertStringNumberRational_loop11 remainder scale_2 weight_2))) else (source_ml_source1_temporary_convertStringNumberRational_loop11_result.source_ml_source1_temporary_convertStringNumberRational_loop11_done scale weight))

inductive source_ml_source1_temporary_convertStringNumberRational_join10 where
  | source_ml_source1_temporary_convertStringNumberRational_join10_next (numerator : Int) (denominator : Int) : source_ml_source1_temporary_convertStringNumberRational_join10
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_source1_temporary_convertStringNumberRational_loop13 (numerator : Int) (shift : Float) (index : Float) : Int :=
  (if (decide (index < shift)) then (let numerator_2 := (numerator * (2 : Int));
    (let index_2 := (index + (1 : Float));
      (source_ml_source1_temporary_convertStringNumberRational_loop13 numerator_2 shift index_2))) else numerator)

partial def source_ml_source1_temporary_convertStringNumberRational_loop14 (denominator : Int) (shift : Float) (index : Float) : Int :=
  (if (decide (index < (-shift))) then (let denominator_2 := (denominator * (2 : Int));
    (let index_2 := (index + (1 : Float));
      (source_ml_source1_temporary_convertStringNumberRational_loop14 denominator_2 shift index_2))) else denominator)

inductive source_ml_source1_temporary_convertStringNumberRational_join9 where
  | source_ml_source1_temporary_convertStringNumberRational_join9_next (exponent : Float) : source_ml_source1_temporary_convertStringNumberRational_join9
  | source_ml_source1_temporary_convertStringNumberRational_join9_return (ml_source1_temporary_result14 : Float) : source_ml_source1_temporary_convertStringNumberRational_join9
  deriving Repr, Inhabited

inductive source_ml_source1_temporary_convertStringNumberRational_loop15_result where
  | source_ml_source1_temporary_convertStringNumberRational_loop15_done (exponent : Float) : source_ml_source1_temporary_convertStringNumberRational_loop15_result
  | source_ml_source1_temporary_convertStringNumberRational_loop15_return (ml_source1_temporary_result15 : Float) : source_ml_source1_temporary_convertStringNumberRational_loop15_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_convertStringNumberRational_loop15 (numerator : Int) (exponent : Float) (scale : Int) : source_ml_source1_temporary_convertStringNumberRational_loop15_result :=
  (if (decide (numerator ≥ (scale * (2 : Int)))) then (let scale_2 := (scale * (2 : Int));
    (let exponent_2 := (exponent + (1 : Float));
      (if (decide (exponent_2 ≥ (1024 : Float))) then (source_ml_source1_temporary_convertStringNumberRational_loop15_result.source_ml_source1_temporary_convertStringNumberRational_loop15_return (1.0 / 0.0 : Float)) else (source_ml_source1_temporary_convertStringNumberRational_loop15 numerator exponent_2 scale_2)))) else (source_ml_source1_temporary_convertStringNumberRational_loop15_result.source_ml_source1_temporary_convertStringNumberRational_loop15_done exponent))

inductive source_ml_source1_temporary_convertStringNumberRational_loop16_result where
  | source_ml_source1_temporary_convertStringNumberRational_loop16_done (exponent : Float) : source_ml_source1_temporary_convertStringNumberRational_loop16_result
  | source_ml_source1_temporary_convertStringNumberRational_loop16_return (ml_source1_temporary_result16 : Float) : source_ml_source1_temporary_convertStringNumberRational_loop16_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_convertStringNumberRational_loop16 (denominator : Int) (exponent : Float) (scale : Int) : source_ml_source1_temporary_convertStringNumberRational_loop16_result :=
  (if (decide (scale < denominator)) then (let scale_2 := (scale * (2 : Int));
    (let exponent_2 := (exponent - (1 : Float));
      (if (decide (exponent_2 < (-1075 : Float))) then (source_ml_source1_temporary_convertStringNumberRational_loop16_result.source_ml_source1_temporary_convertStringNumberRational_loop16_return (0 : Float)) else (source_ml_source1_temporary_convertStringNumberRational_loop16 denominator exponent_2 scale_2)))) else (source_ml_source1_temporary_convertStringNumberRational_loop16_result.source_ml_source1_temporary_convertStringNumberRational_loop16_done exponent))

def convertStringNumberRational (numerator : Int) (denominator : Int) : Float :=
  (if ((decide (numerator < (0 : Int))) || (decide (denominator ≤ (0 : Int)))) then (0.0 / 0.0 : Float) else (if (numerator == (0 : Int)) then (0 : Float) else (let exponent := (0 : Float);
    (let source_ml_source2_s6 := (if (decide (numerator ≥ denominator)) then (let scale := denominator;
      (let source_ml_source2_s7 := (source_ml_source1_temporary_convertStringNumberRational_loop15 numerator exponent scale);
        (match source_ml_source2_s7 with
          | source_ml_source1_temporary_convertStringNumberRational_loop15_result.source_ml_source1_temporary_convertStringNumberRational_loop15_done exponent_2 => (source_ml_source1_temporary_convertStringNumberRational_join9.source_ml_source1_temporary_convertStringNumberRational_join9_next exponent_2)
          | source_ml_source1_temporary_convertStringNumberRational_loop15_result.source_ml_source1_temporary_convertStringNumberRational_loop15_return source_ml_source1_temporary_result15 => (source_ml_source1_temporary_convertStringNumberRational_join9.source_ml_source1_temporary_convertStringNumberRational_join9_return source_ml_source1_temporary_result15)))) else (let scale_2 := numerator;
      (let source_ml_source2_s8 := (source_ml_source1_temporary_convertStringNumberRational_loop16 denominator exponent scale_2);
        (match source_ml_source2_s8 with
          | source_ml_source1_temporary_convertStringNumberRational_loop16_result.source_ml_source1_temporary_convertStringNumberRational_loop16_done exponent_3 => (source_ml_source1_temporary_convertStringNumberRational_join9.source_ml_source1_temporary_convertStringNumberRational_join9_next exponent_3)
          | source_ml_source1_temporary_convertStringNumberRational_loop16_result.source_ml_source1_temporary_convertStringNumberRational_loop16_return source_ml_source1_temporary_result16 => (source_ml_source1_temporary_convertStringNumberRational_join9.source_ml_source1_temporary_convertStringNumberRational_join9_return source_ml_source1_temporary_result16)))));
      (match source_ml_source2_s6 with
        | source_ml_source1_temporary_convertStringNumberRational_join9.source_ml_source1_temporary_convertStringNumberRational_join9_next exponent_4 => (let shift := (if (decide (exponent_4 ≥ (-1022 : Float))) then ((52 : Float) - exponent_4) else (1074 : Float));
            (let source_ml_source2_s9 := (if (decide (shift ≥ (0 : Float))) then (let index := (0 : Float);
              (let numerator_2 := (source_ml_source1_temporary_convertStringNumberRational_loop13 numerator shift index);
                (source_ml_source1_temporary_convertStringNumberRational_join10.source_ml_source1_temporary_convertStringNumberRational_join10_next numerator_2 denominator))) else (let index_2 := (0 : Float);
              (let denominator_2 := (source_ml_source1_temporary_convertStringNumberRational_loop14 denominator shift index_2);
                (source_ml_source1_temporary_convertStringNumberRational_join10.source_ml_source1_temporary_convertStringNumberRational_join10_next numerator denominator_2))));
              (match source_ml_source2_s9 with
                | source_ml_source1_temporary_convertStringNumberRational_join10.source_ml_source1_temporary_convertStringNumberRational_join10_next numerator_3 denominator_3 => (let quotient := (0 : Int);
                    (let remainder := numerator_3;
                      (let scale_3 := denominator_3;
                        (let weight := (1 : Int);
                          (let source_ml_source2_s10 := (source_ml_source1_temporary_convertStringNumberRational_loop11 remainder scale_3 weight);
                            (match source_ml_source2_s10 with
                              | source_ml_source1_temporary_convertStringNumberRational_loop11_result.source_ml_source1_temporary_convertStringNumberRational_loop11_done scale_4 weight_2 => (let source_ml_source2_s11 := (source_ml_source1_temporary_convertStringNumberRational_loop12 quotient remainder scale_4 weight_2);
                                  (match source_ml_source2_s11 with
                                    | source_ml_source1_temporary_convertStringNumberRational_loop12_result.source_ml_source1_temporary_convertStringNumberRational_loop12_done quotient_2 remainder_2 => (let twice := (remainder_2 * (2 : Int));
                                        (let quotient_4 := (if ((decide (twice > denominator_3)) || ((twice == denominator_3) && ((Int.tmod quotient_2 (2 : Int)) == (1 : Int)))) then (let quotient_3 := (quotient_2 + (1 : Int));
                                          quotient_3) else quotient_2);
                                          ((readStringNumberExactMantissa quotient_4) * (readStringNumberBinaryPower (-shift))))))))))))))))
        | source_ml_source1_temporary_convertStringNumberRational_join9.source_ml_source1_temporary_convertStringNumberRational_join9_return source_ml_source1_temporary_result14 => source_ml_source1_temporary_result14)))))

def convertJavaScriptBigIntegerNumber (value : Int) : Float :=
  (if (decide (value < (0 : Int))) then (-(convertStringNumberRational (-value) (1 : Int))) else (convertStringNumberRational value (1 : Int)))

inductive source_ml_source1_temporary_acceptStringNumberInfinity_loop17_result where
  | source_ml_source1_temporary_acceptStringNumberInfinity_loop17_done : source_ml_source1_temporary_acceptStringNumberInfinity_loop17_result
  | source_ml_source1_temporary_acceptStringNumberInfinity_loop17_return (ml_source1_temporary_result17 : Bool) : source_ml_source1_temporary_acceptStringNumberInfinity_loop17_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_source1_temporary_acceptStringNumberInfinity_loop17 (units : (Array Float)) (start : Float) (expected : (Array Float)) (index : Float) : source_ml_source1_temporary_acceptStringNumberInfinity_loop17_result :=
  (if (decide (index < (Float.ofNat expected.size))) then (if ((match (let ml_index := (start + index); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)) != (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then expected[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-2 : Float))) then (source_ml_source1_temporary_acceptStringNumberInfinity_loop17_result.source_ml_source1_temporary_acceptStringNumberInfinity_loop17_return false) else (let index_2 := (index + (1 : Float));
    (source_ml_source1_temporary_acceptStringNumberInfinity_loop17 units start expected index_2))) else source_ml_source1_temporary_acceptStringNumberInfinity_loop17_result.source_ml_source1_temporary_acceptStringNumberInfinity_loop17_done)

def acceptStringNumberInfinity (units : (Array Float)) (start : Float) («end» : Float) : Bool :=
  (if ((«end» - start) != (8 : Float)) then false else (let expected := (#[(73 : Float), (110 : Float), (102 : Float), (105 : Float), (110 : Float), (105 : Float), (116 : Float), (121 : Float)] : (Array Float));
    (let index := (0 : Float);
      (let source_ml_source2_s12 := (source_ml_source1_temporary_acceptStringNumberInfinity_loop17 units start expected index);
        (match source_ml_source2_s12 with
          | source_ml_source1_temporary_acceptStringNumberInfinity_loop17_result.source_ml_source1_temporary_acceptStringNumberInfinity_loop17_done => true
          | source_ml_source1_temporary_acceptStringNumberInfinity_loop17_result.source_ml_source1_temporary_acceptStringNumberInfinity_loop17_return source_ml_source1_temporary_result17 => source_ml_source1_temporary_result17)))))

partial def source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19 (units : (Array Float)) (start : Float) («end» : Float) : Float :=
  (if ((decide (start < «end»)) && (acceptJavaScriptWhitespaceUnit (match (let ml_index := start; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)))) then (let start_2 := (start + (1 : Float));
    (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19 units start_2 «end»)) else start)

partial def source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop20 (units : (Array Float)) (start : Float) («end» : Float) : Float :=
  (if ((decide («end» > start)) && (acceptJavaScriptWhitespaceUnit (match (let ml_index := («end» - (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)))) then (let end_2 := («end» - (1 : Float));
    (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop20 units start end_2)) else «end»)

partial def source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop26 (units : (Array Float)) (integerDigits : (Array Int)) (mantissaEnd : Float) (mantissa : Int) (index : Float) : Int :=
  (if (decide (index < mantissaEnd)) then (let unit := (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if (unit != (46 : Float)) then (let mantissa_2 := ((mantissa * (10 : Int)) + (match (let ml_index := (unit - (48 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then integerDigits[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)));
      (let index_2 := (index + (1 : Float));
        (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop26 units integerDigits mantissaEnd mantissa_2 index_2))) else (let index_3 := (index + (1 : Float));
      (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop26 units integerDigits mantissaEnd mantissa index_3)))) else mantissa)

inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_result where
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_done (exponent : Float) (mantissaEnd : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25 (units : (Array Float)) (start : Float) (exponent : Float) (mantissaEnd : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_result :=
  (if (decide (mantissaEnd > start)) then (let unit := (match (let ml_index := (mantissaEnd - (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if (unit == (46 : Float)) then (let mantissaEnd_2 := (mantissaEnd - (1 : Float));
      (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25 units start exponent mantissaEnd_2)) else (if (unit != (48 : Float)) then (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_done exponent mantissaEnd) else (let mantissaEnd_3 := (mantissaEnd - (1 : Float));
      (let exponent_2 := (exponent + (1 : Float));
        (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25 units start exponent_2 mantissaEnd_3)))))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_done exponent mantissaEnd))

inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join24 where
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join24_next («at» : Float) (exponent : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join24
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join24_return (ml_source1_temporary_result26 : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join24
  deriving Repr, Inhabited

inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop28_result where
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop28_done («at» : Float) (exponent : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop28_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop28 (units : (Array Float)) («end» : Float) («at» : Float) (exponent : Float) (exponentLimit : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop28_result :=
  (if (decide («at» < «end»)) then (let digit := ((match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)) - (48 : Float));
    (if ((decide (digit < (0 : Float))) || (decide (digit > (9 : Float)))) then (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop28_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop28_done «at» exponent) else (let exponent_3 := (if (decide (exponent < exponentLimit)) then (let exponent_2 := (ml_min exponentLimit ((exponent * (10 : Float)) + digit));
      exponent_2) else exponent);
      (let at_2 := («at» + (1 : Float));
        (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop28 units «end» at_2 exponent_3 exponentLimit))))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop28_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop28_done «at» exponent))

inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join27 where
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join27_next («at» : Float) (exponentNegative : Bool) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join27
  deriving Repr, Inhabited

inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop23_result where
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop23_done («at» : Float) (fraction : Float) (digits : Float) (significant : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop23_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop23 (units : (Array Float)) («end» : Float) («at» : Float) (fraction : Float) (digits : Float) (significant : Float) (dot : Bool) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop23_result :=
  (if (decide («at» < «end»)) then (let unit := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if ((decide (unit ≥ (48 : Float))) && (decide (unit ≤ (57 : Float)))) then (let digits_2 := (digits + (1 : Float));
      (let fraction_3 := (if dot then (let fraction_2 := (fraction + (1 : Float));
        fraction_2) else fraction);
        (let significant_3 := (if ((unit != (48 : Float)) || (decide (significant > (0 : Float)))) then (let significant_2 := (significant + (1 : Float));
          significant_2) else significant);
          (let at_2 := («at» + (1 : Float));
            (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop23 units «end» at_2 fraction_3 digits_2 significant_3 dot))))) else (if ((unit == (46 : Float)) && (!dot)) then (let dot_2 := true;
      (let at_3 := («at» + (1 : Float));
        (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop23 units «end» at_3 fraction digits significant dot_2))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop23_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop23_done «at» fraction digits significant)))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop23_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop23_done «at» fraction digits significant))

inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join22 where
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join22_next (start : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join22
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join22_return (ml_source1_temporary_result28 : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join22
  deriving Repr, Inhabited

inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29_result where
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29_done (value : Int) (overflow : Bool) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29_result
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29_return (ml_source1_temporary_result29 : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29 (units : (Array Float)) («end» : Float) (integerDigits : (Array Int)) (radix : Float) (value : Int) (overflow : Bool) (limit : Int) (base : Int) (index : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29_result :=
  (if (decide (index < «end»)) then (let digit := (readStringNumberDigit (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)));
    (if ((decide (digit < (0 : Float))) || (decide (digit ≥ radix))) then (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29_return (0.0 / 0.0 : Float)) else (if (!overflow) then (let value_2 := ((value * base) + (match (let ml_index := digit; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then integerDigits[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)));
      (let overflow_2 := (decide (value_2 > limit));
        (let index_2 := (index + (1 : Float));
          (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29 units «end» integerDigits radix value_2 overflow_2 limit base index_2)))) else (let index_3 := (index + (1 : Float));
      (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29 units «end» integerDigits radix value overflow limit base index_3))))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29_done value overflow))

inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join21 where
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join21_next (start : Float) (negative : Bool) (signed : Bool) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join21
  deriving Repr, Inhabited

inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18_result where
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18_done : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18_result
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18_return (ml_source1_temporary_result29 : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18 (units : (Array Float)) (index : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18_result :=
  (if (decide (index < (Float.ofNat units.size))) then (let unit := (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if (((!(ml_is_safe_integer unit)) || (decide (unit < (0 : Float)))) || (decide (unit > (65535 : Float)))) then (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18_return (0.0 / 0.0 : Float)) else (let index_2 := (index + (1 : Float));
      (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18 units index_2)))) else source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18_done)

def convertJavaScriptStringNumberUnits (units : (Array Float)) : Float :=
  (let index := (0 : Float);
    (let source_ml_source2_s13 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18 units index);
      (match source_ml_source2_s13 with
        | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18_done => (let start := (0 : Float);
            (let «end» := (Float.ofNat units.size);
              (let start_2 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19 units start «end»);
                (let end_2 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop20 units start_2 «end»);
                  (if (start_2 == end_2) then (0 : Float) else (let negative := false;
                    (let signed := false;
                      (let sign := (match (let ml_index := start_2; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
                        (let source_ml_source2_s14 := (if ((sign == (43 : Float)) || (sign == (45 : Float))) then (let negative_2 := (sign == (45 : Float));
                          (let signed_2 := true;
                            (let start_3 := (start_2 + (1 : Float));
                              (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join21.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join21_next start_3 negative_2 signed_2)))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join21.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join21_next start_2 negative signed));
                          (match source_ml_source2_s14 with
                            | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join21.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join21_next start_4 negative_3 signed_3 => (if (start_4 == end_2) then (0.0 / 0.0 : Float) else (if (acceptStringNumberInfinity units start_4 end_2) then (if negative_3 then (-1.0 / 0.0 : Float) else (1.0 / 0.0 : Float)) else (let integerDigits := (#[(0 : Int), (1 : Int), (2 : Int), (3 : Int), (4 : Int), (5 : Int), (6 : Int), (7 : Int), (8 : Int), (9 : Int), (10 : Int), (11 : Int), (12 : Int), (13 : Int), (14 : Int), (15 : Int)] : (Array Int));
                                (let source_ml_source2_s16 := (if (((!signed_3) && ((match (let ml_index := start_4; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)) == (48 : Float))) && (decide ((end_2 - start_4) ≥ (2 : Float)))) then (let marker := (match (let ml_index := (start_4 + (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
                                  (let radix := (if ((marker == (120 : Float)) || (marker == (88 : Float))) then (16 : Float) else (if ((marker == (98 : Float)) || (marker == (66 : Float))) then (2 : Float) else (if ((marker == (111 : Float)) || (marker == (79 : Float))) then (8 : Float) else (0 : Float))));
                                    (if (radix != (0 : Float)) then (let value := (0 : Int);
                                      (let overflow := false;
                                        (let limit := (readStringNumberDecimalPower (310 : Float));
                                          (let base := (if (radix == (16 : Float)) then (16 : Int) else (if (radix == (8 : Float)) then (8 : Int) else (2 : Int)));
                                            (let start_5 := (start_4 + (2 : Float));
                                              (if (start_5 == end_2) then (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join22.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join22_return (0.0 / 0.0 : Float)) else (let index_2 := start_5;
                                                (let source_ml_source2_s17 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29 units end_2 integerDigits radix value overflow limit base index_2);
                                                  (match source_ml_source2_s17 with
                                                    | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29_done value_2 overflow_2 => (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join22.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join22_return (if overflow_2 then (1.0 / 0.0 : Float) else (convertStringNumberRational value_2 (1 : Int))))
                                                    | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop29_return source_ml_source1_temporary_result29 => (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join22.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join22_return source_ml_source1_temporary_result29)))))))))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join22.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join22_next start_4)))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join22.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join22_next start_4));
                                  (match source_ml_source2_s16 with
                                    | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join22.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join22_next start_6 => (let «at» := start_6;
                                        (let fraction := (0 : Float);
                                          (let digits := (0 : Float);
                                            (let significant := (0 : Float);
                                              (let dot := false;
                                                (let source_ml_source2_s18 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop23 units end_2 «at» fraction digits significant dot);
                                                  (match source_ml_source2_s18 with
                                                    | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop23_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop23_done at_2 fraction_2 digits_2 significant_2 => (if (digits_2 == (0 : Float)) then (0.0 / 0.0 : Float) else (let decimalEnd := at_2;
                                                        (let exponent := (0 : Float);
                                                          (let exponentLimit := ((Float.ofNat units.size) + (10000 : Float));
                                                            (let source_ml_source2_s21 := (if ((decide (at_2 < end_2)) && (((match (let ml_index := at_2; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)) == (101 : Float)) || ((match (let ml_index := at_2; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)) == (69 : Float)))) then (let at_3 := (at_2 + (1 : Float));
                                                              (let exponentNegative := false;
                                                                (let exponentSign := (match (let ml_index := at_3; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
                                                                  (let source_ml_source2_s22 := (if ((exponentSign == (43 : Float)) || (exponentSign == (45 : Float))) then (let exponentNegative_2 := (exponentSign == (45 : Float));
                                                                    (let at_4 := (at_3 + (1 : Float));
                                                                      (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join27.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join27_next at_4 exponentNegative_2))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join27.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join27_next at_3 exponentNegative));
                                                                    (match source_ml_source2_s22 with
                                                                      | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join27.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join27_next at_5 exponentNegative_3 => (let first := at_5;
                                                                          (let source_ml_source2_s23 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop28 units end_2 at_5 exponent exponentLimit);
                                                                            (match source_ml_source2_s23 with
                                                                              | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop28_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop28_done at_6 exponent_2 => (if (at_6 == first) then (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join24.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join24_return (0.0 / 0.0 : Float)) else (if exponentNegative_3 then (let exponent_3 := (-exponent_2);
                                                                                  (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join24.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join24_next at_6 exponent_3)) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join24.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join24_next at_6 exponent_2))))))))))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join24.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join24_next at_2 exponent));
                                                              (match source_ml_source2_s21 with
                                                                | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join24.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join24_next at_7 exponent_4 => (if (at_7 != end_2) then (0.0 / 0.0 : Float) else (if (significant_2 == (0 : Float)) then (if negative_3 then (-0 : Float) else (0 : Float)) else (let exponent_5 := (exponent_4 - fraction_2);
                                                                    (let order := (significant_2 + exponent_5);
                                                                      (if (decide (order > (310 : Float))) then (if negative_3 then (-1.0 / 0.0 : Float) else (1.0 / 0.0 : Float)) else (if (decide (order < (-325 : Float))) then (if negative_3 then (-0 : Float) else (0 : Float)) else (let mantissaEnd := decimalEnd;
                                                                        (let source_ml_source2_s24 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25 units start_6 exponent_5 mantissaEnd);
                                                                          (match source_ml_source2_s24 with
                                                                            | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_done exponent_6 mantissaEnd_2 => (let mantissa := (0 : Int);
                                                                                (let index_3 := start_6;
                                                                                  (let mantissa_2 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop26 units integerDigits mantissaEnd_2 mantissa index_3);
                                                                                    (let numerator := (if (decide (exponent_6 ≥ (0 : Float))) then (mantissa_2 * (readStringNumberDecimalPower exponent_6)) else mantissa_2);
                                                                                      (let denominator := (if (decide (exponent_6 < (0 : Float))) then (readStringNumberDecimalPower (-exponent_6)) else (1 : Int));
                                                                                        (let value_3 := (convertStringNumberRational numerator denominator);
                                                                                          (if negative_3 then (-value_3) else value_3))))))))))))))))
                                                                | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join24.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join24_return source_ml_source1_temporary_result26 => source_ml_source1_temporary_result26)))))))))))))
                                    | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join22.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join22_return source_ml_source1_temporary_result28 => source_ml_source1_temporary_result28)))))))))))))))
        | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop18_return source_ml_source1_temporary_result29_2 => source_ml_source1_temporary_result29_2)))

def convertJavaScriptUtf8StringNumberUnits (bytes : (Array Float)) : Float :=
  (convertJavaScriptStringNumberUnits (readJavaScriptUtf8StringUnits bytes))

partial def source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop31 (units : (Array Float)) (start : Float) («end» : Float) : Float :=
  (if ((decide (start < «end»)) && (acceptJavaScriptWhitespaceUnit (match (let ml_index := start; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)))) then (let start_2 := (start + (1 : Float));
    (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop31 units start_2 «end»)) else start)

partial def source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop32 (units : (Array Float)) (start : Float) («end» : Float) : Float :=
  (if ((decide («end» > start)) && (acceptJavaScriptWhitespaceUnit (match (let ml_index := («end» - (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)))) then (let end_2 := («end» - (1 : Float));
    (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop32 units start end_2)) else «end»)

inductive source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35_result where
  | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35_done (value : Int) : source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35_result
  | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35_return (ml_source1_temporary_result35 : (Array Int)) : source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35 (units : (Array Float)) («end» : Float) (radix : Float) (base : Int) (digits : (Array Int)) (value : Int) (index : Float) : source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35_result :=
  (if (decide (index < «end»)) then (let digit := (readJavaScriptNumericDigitUnit (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)));
    (if ((decide (digit < (0 : Float))) || (decide (digit ≥ radix))) then (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35_result.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35_return (#[] : (Array Int))) else (let value_2 := ((value * base) + (match (let ml_index := digit; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then digits[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)));
      (let index_2 := (index + (1 : Float));
        (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35 units «end» radix base digits value_2 index_2))))) else (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35_result.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35_done value))

inductive source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join34 where
  | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join34_next (start : Float) (radix : Float) (base : Int) : source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join34
  deriving Repr, Inhabited

inductive source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join33 where
  | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join33_next (start : Float) (negative : Bool) (signed : Bool) : source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join33
  deriving Repr, Inhabited

inductive source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30_result where
  | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30_done : source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30_result
  | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30_return (ml_source1_temporary_result35 : (Array Int)) : source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30 (units : (Array Float)) (index : Float) : source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30_result :=
  (if (decide (index < (Float.ofNat units.size))) then (let unit := (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if (((!(ml_is_safe_integer unit)) || (decide (unit < (0 : Float)))) || (decide (unit > (65535 : Float)))) then (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30_result.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30_return (#[] : (Array Int))) else (let index_2 := (index + (1 : Float));
      (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30 units index_2)))) else source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30_result.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30_done)

def convertJavaScriptStringBigIntegerUnits (units : (Array Float)) : (Array Int) :=
  (let index := (0 : Float);
    (let source_ml_source2_s25 := (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30 units index);
      (match source_ml_source2_s25 with
        | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30_result.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30_done => (let start := (0 : Float);
            (let «end» := (Float.ofNat units.size);
              (let start_2 := (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop31 units start «end»);
                (let end_2 := (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop32 units start_2 «end»);
                  (if (start_2 == end_2) then (#[(0 : Int)] : (Array Int)) else (let negative := false;
                    (let signed := false;
                      (let sign := (match (let ml_index := start_2; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
                        (let source_ml_source2_s26 := (if ((sign == (43 : Float)) || (sign == (45 : Float))) then (let negative_2 := (sign == (45 : Float));
                          (let signed_2 := true;
                            (let start_3 := (start_2 + (1 : Float));
                              (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join33.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join33_next start_3 negative_2 signed_2)))) else (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join33.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join33_next start_2 negative signed));
                          (match source_ml_source2_s26 with
                            | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join33.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join33_next start_4 negative_3 signed_3 => (if (start_4 == end_2) then (#[] : (Array Int)) else (let radix := (10 : Float);
                                (let base := (10 : Int);
                                  (let source_ml_source2_s27 := (if (((!signed_3) && ((match (let ml_index := start_4; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)) == (48 : Float))) && (decide ((end_2 - start_4) ≥ (2 : Float)))) then (let marker := (match (let ml_index := (start_4 + (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
                                    (let selected := (if ((marker == (120 : Float)) || (marker == (88 : Float))) then (16 : Float) else (if ((marker == (98 : Float)) || (marker == (66 : Float))) then (2 : Float) else (if ((marker == (111 : Float)) || (marker == (79 : Float))) then (8 : Float) else (0 : Float))));
                                      (if (selected != (0 : Float)) then (let radix_2 := selected;
                                        (let base_2 := (if (selected == (16 : Float)) then (16 : Int) else (if (selected == (8 : Float)) then (8 : Int) else (2 : Int)));
                                          (let start_5 := (start_4 + (2 : Float));
                                            (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join34.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join34_next start_5 radix_2 base_2)))) else (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join34.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join34_next start_4 radix base)))) else (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join34.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join34_next start_4 radix base));
                                    (match source_ml_source2_s27 with
                                      | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join34.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join34_next start_6 radix_3 base_3 => (if (start_6 == end_2) then (#[] : (Array Int)) else (let digits := (#[(0 : Int), (1 : Int), (2 : Int), (3 : Int), (4 : Int), (5 : Int), (6 : Int), (7 : Int), (8 : Int), (9 : Int), (10 : Int), (11 : Int), (12 : Int), (13 : Int), (14 : Int), (15 : Int)] : (Array Int));
                                          (let value := (0 : Int);
                                            (let index_2 := start_6;
                                              (let source_ml_source2_s28 := (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35 units end_2 radix_3 base_3 digits value index_2);
                                                (match source_ml_source2_s28 with
                                                  | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35_result.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35_done value_2 => (#[(if negative_3 then (-value_2) else value_2)] : (Array Int))
                                                  | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35_result.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop35_return source_ml_source1_temporary_result35 => source_ml_source1_temporary_result35)))))))))))))))))))))
        | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30_result.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop30_return source_ml_source1_temporary_result35_2 => source_ml_source1_temporary_result35_2)))

def convertJavaScriptUtf8BigInteger (bytes : (Array Float)) : (Array Int) :=
  (convertJavaScriptStringBigIntegerUnits (readJavaScriptUtf8StringUnits bytes))

partial def source_ml_source1_temporary_convertJavaScriptNumberBigInteger_loop36 (value : Float) (integer : Int) (weight : Int) : Int :=
  (if (decide (value ≥ (1 : Float))) then (let integer_3 := (if ((ml_float_rem value (2 : Float)) == (1 : Float)) then (let integer_2 := (integer + weight);
    integer_2) else integer);
    (let value_2 := (Float.floor (value / (2 : Float)));
      (let weight_2 := (weight * (2 : Int));
        (source_ml_source1_temporary_convertJavaScriptNumberBigInteger_loop36 value_2 integer_3 weight_2)))) else integer)

def convertJavaScriptNumberBigInteger (value : Float) : (Array Int) :=
  (if (!(ml_is_integer value)) then (#[] : (Array Int)) else (let negative := (decide (value < (0 : Float)));
    (let value_2 := (Float.abs value);
      (let integer := (0 : Int);
        (let weight := (1 : Int);
          (let integer_2 := (source_ml_source1_temporary_convertJavaScriptNumberBigInteger_loop36 value_2 integer weight);
            (#[(if negative then (-integer_2) else integer_2)] : (Array Int))))))))

inductive source_ml_source1_temporary_divideJavaScriptBigInteger_loop38_result where
  | source_ml_source1_temporary_divideJavaScriptBigInteger_loop38_done (remaining : Int) (quotient : Int) : source_ml_source1_temporary_divideJavaScriptBigInteger_loop38_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_source1_temporary_divideJavaScriptBigInteger_loop38 (remaining : Int) (shifted : Int) (weight : Int) (quotient : Int) : source_ml_source1_temporary_divideJavaScriptBigInteger_loop38_result :=
  (if (decide (weight > (1 : Int))) then (let shifted_2 := (Int.tdiv shifted (2 : Int));
    (let weight_2 := (Int.tdiv weight (2 : Int));
      (if (decide (shifted_2 ≤ remaining)) then (let remaining_2 := (remaining - shifted_2);
        (let quotient_2 := (quotient + weight_2);
          (source_ml_source1_temporary_divideJavaScriptBigInteger_loop38 remaining_2 shifted_2 weight_2 quotient_2))) else (source_ml_source1_temporary_divideJavaScriptBigInteger_loop38 remaining shifted_2 weight_2 quotient)))) else (source_ml_source1_temporary_divideJavaScriptBigInteger_loop38_result.source_ml_source1_temporary_divideJavaScriptBigInteger_loop38_done remaining quotient))

inductive source_ml_source1_temporary_divideJavaScriptBigInteger_loop37_result where
  | source_ml_source1_temporary_divideJavaScriptBigInteger_loop37_done (shifted : Int) (weight : Int) : source_ml_source1_temporary_divideJavaScriptBigInteger_loop37_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_source1_temporary_divideJavaScriptBigInteger_loop37 (remaining : Int) (shifted : Int) (weight : Int) : source_ml_source1_temporary_divideJavaScriptBigInteger_loop37_result :=
  (if (decide (shifted ≤ remaining)) then (let shifted_2 := (shifted * (2 : Int));
    (let weight_2 := (weight * (2 : Int));
      (source_ml_source1_temporary_divideJavaScriptBigInteger_loop37 remaining shifted_2 weight_2))) else (source_ml_source1_temporary_divideJavaScriptBigInteger_loop37_result.source_ml_source1_temporary_divideJavaScriptBigInteger_loop37_done shifted weight))

def divideJavaScriptBigInteger (left : Int) (right : Int) : (Array Int) :=
  (if (right == (0 : Int)) then (#[] : (Array Int)) else (let remaining := (if (decide (left < (0 : Int))) then (-left) else left);
    (let divisor := (if (decide (right < (0 : Int))) then (-right) else right);
      (let shifted := divisor;
        (let weight := (1 : Int);
          (let quotient := (0 : Int);
            (let source_ml_source2_s29 := (source_ml_source1_temporary_divideJavaScriptBigInteger_loop37 remaining shifted weight);
              (match source_ml_source2_s29 with
                | source_ml_source1_temporary_divideJavaScriptBigInteger_loop37_result.source_ml_source1_temporary_divideJavaScriptBigInteger_loop37_done shifted_2 weight_2 => (let source_ml_source2_s30 := (source_ml_source1_temporary_divideJavaScriptBigInteger_loop38 remaining shifted_2 weight_2 quotient);
                    (match source_ml_source2_s30 with
                      | source_ml_source1_temporary_divideJavaScriptBigInteger_loop38_result.source_ml_source1_temporary_divideJavaScriptBigInteger_loop38_done remaining_2 quotient_2 => (#[(if ((decide (left < (0 : Int))) != (decide (right < (0 : Int)))) then (-quotient_2) else quotient_2), (if (decide (left < (0 : Int))) then (-remaining_2) else remaining_2)] : (Array Int))))))))))))

def divideNumberPowerBound (numerator : Int) (denominator : Int) (upward : Bool) : Int :=
  (let result := (divideJavaScriptBigInteger numerator denominator);
    (let quotient := (match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then result[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int));
      (let remainder := (match (let ml_index := (1 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then result[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int));
        (quotient + (if (upward && (remainder != (0 : Int))) then (1 : Int) else (0 : Int))))))

partial def source_ml_source1_temporary_powerJavaScriptBigInteger_loop39 (result : Int) (weight : Int) (remaining : Int) : Int :=
  (if (decide (remaining > (0 : Int))) then (let result_3 := (if ((Int.tmod remaining (2 : Int)) == (1 : Int)) then (let result_2 := (result * weight);
    result_2) else result);
    (let remaining_2 := (Int.tdiv remaining (2 : Int));
      (if (decide (remaining_2 > (0 : Int))) then (let weight_2 := (weight * weight);
        (source_ml_source1_temporary_powerJavaScriptBigInteger_loop39 result_3 weight_2 remaining_2)) else (source_ml_source1_temporary_powerJavaScriptBigInteger_loop39 result_3 weight remaining_2)))) else result)

def powerJavaScriptBigInteger (base : Int) (exponent : Int) : (Array Int) :=
  (if (decide (exponent < (0 : Int))) then (#[] : (Array Int)) else (let result := (1 : Int);
    (let weight := base;
      (let remaining := exponent;
        (let result_2 := (source_ml_source1_temporary_powerJavaScriptBigInteger_loop39 result weight remaining);
          (#[result_2] : (Array Int)))))))

def roundNumberPowerBound (value : Int) (exponent : Int) (precision : Int) : Float :=
  (if ((value == (0 : Int)) || (decide (exponent < (-1076 : Int)))) then (0 : Float) else (if (decide (exponent > (1024 : Int))) then (1.0 / 0.0 : Float) else (let shift := (exponent - precision);
    (if (decide (shift ≥ (0 : Int))) then (convertStringNumberRational (value * (match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then (powerJavaScriptBigInteger (2 : Int) shift)[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int))) (1 : Int)) else (convertStringNumberRational value (match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then (powerJavaScriptBigInteger (2 : Int) (-shift))[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)))))))

inductive source_ml_source1_temporary_normalizeNumberPowerBound_loop41_result where
  | source_ml_source1_temporary_normalizeNumberPowerBound_loop41_done (value : Int) (exponent : Int) : source_ml_source1_temporary_normalizeNumberPowerBound_loop41_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_source1_temporary_normalizeNumberPowerBound_loop41 (value : Int) (exponent : Int) (scale : Int) : source_ml_source1_temporary_normalizeNumberPowerBound_loop41_result :=
  (if (decide (value < scale)) then (let value_2 := (value * (2 : Int));
    (let exponent_2 := (exponent - (1 : Int));
      (source_ml_source1_temporary_normalizeNumberPowerBound_loop41 value_2 exponent_2 scale))) else (source_ml_source1_temporary_normalizeNumberPowerBound_loop41_result.source_ml_source1_temporary_normalizeNumberPowerBound_loop41_done value exponent))

inductive source_ml_source1_temporary_normalizeNumberPowerBound_loop40_result where
  | source_ml_source1_temporary_normalizeNumberPowerBound_loop40_done (value : Int) (exponent : Int) : source_ml_source1_temporary_normalizeNumberPowerBound_loop40_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_source1_temporary_normalizeNumberPowerBound_loop40 (value : Int) (exponent : Int) (scale : Int) (upward : Bool) : source_ml_source1_temporary_normalizeNumberPowerBound_loop40_result :=
  (if (decide (value ≥ (scale * (2 : Int)))) then (let value_2 := (Int.tdiv (value + (if upward then (1 : Int) else (0 : Int))) (2 : Int));
    (let exponent_2 := (exponent + (1 : Int));
      (source_ml_source1_temporary_normalizeNumberPowerBound_loop40 value_2 exponent_2 scale upward))) else (source_ml_source1_temporary_normalizeNumberPowerBound_loop40_result.source_ml_source1_temporary_normalizeNumberPowerBound_loop40_done value exponent))

def normalizeNumberPowerBound (value : Int) (exponent : Int) (scale : Int) (upward : Bool) : (Array Int) :=
  (if (decide (value ≤ (0 : Int))) then (#[(0 : Int), (0 : Int)] : (Array Int)) else (let source_ml_source2_s31 := (source_ml_source1_temporary_normalizeNumberPowerBound_loop40 value exponent scale upward);
    (match source_ml_source2_s31 with
      | source_ml_source1_temporary_normalizeNumberPowerBound_loop40_result.source_ml_source1_temporary_normalizeNumberPowerBound_loop40_done value_2 exponent_2 => (let source_ml_source2_s32 := (source_ml_source1_temporary_normalizeNumberPowerBound_loop41 value_2 exponent_2 scale);
          (match source_ml_source2_s32 with
            | source_ml_source1_temporary_normalizeNumberPowerBound_loop41_result.source_ml_source1_temporary_normalizeNumberPowerBound_loop41_done value_3 exponent_3 => (#[value_3, exponent_3] : (Array Int)))))))

def multiplyNumberPowerInterval (left : (Array Int)) (right : (Array Int)) (scale : Int) : (Array Int) :=
  (let lower := (normalizeNumberPowerBound (divideNumberPowerBound ((match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then left[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)) * (match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then right[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int))) scale false) ((match (let ml_index := (1 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then left[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)) + (match (let ml_index := (1 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then right[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int))) scale false);
    (let upper := (normalizeNumberPowerBound (divideNumberPowerBound ((match (let ml_index := (2 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then left[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)) * (match (let ml_index := (2 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then right[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int))) scale true) ((match (let ml_index := (3 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then left[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)) + (match (let ml_index := (3 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then right[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int))) scale true);
      (#[(match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then lower[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)), (match (let ml_index := (1 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then lower[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)), (match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then upper[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)), (match (let ml_index := (1 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then upper[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int))] : (Array Int))))

def invertNumberPowerInterval (interval : (Array Int)) (scale : Int) : (Array Int) :=
  (let lower := (normalizeNumberPowerBound (divideNumberPowerBound (scale * scale) (match (let ml_index := (2 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then interval[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)) false) (-(match (let ml_index := (3 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then interval[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int))) scale false);
    (let upper := (normalizeNumberPowerBound (divideNumberPowerBound (scale * scale) (match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then interval[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)) true) (-(match (let ml_index := (1 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then interval[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int))) scale true);
      (#[(match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then lower[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)), (match (let ml_index := (1 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then lower[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)), (match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then upper[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)), (match (let ml_index := (1 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then upper[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int))] : (Array Int))))

partial def source_ml_source1_temporary_rootNumberPowerInteger_loop42 (root : Int) (remaining : Int) : Int :=
  (if (decide (remaining > (1 : Int))) then (let remaining_2 := (Int.tdiv remaining (4 : Int));
    (let root_2 := (root * (2 : Int));
      (source_ml_source1_temporary_rootNumberPowerInteger_loop42 root_2 remaining_2))) else root)

partial def source_ml_source1_temporary_rootNumberPowerInteger_loop43 (value : Int) (root : Int) : Int :=
  (let next := (Int.tdiv (root + (divideNumberPowerBound value root false)) (2 : Int));
    (if (decide (next ≥ root)) then root else (let root_2 := next;
      (source_ml_source1_temporary_rootNumberPowerInteger_loop43 value root_2))))

def rootNumberPowerInteger (value : Int) : Int :=
  (if (decide (value ≤ (0 : Int))) then (0 : Int) else (let root := (1 : Int);
    (let remaining := value;
      (let root_2 := (source_ml_source1_temporary_rootNumberPowerInteger_loop42 root remaining);
        (let root_3 := (root_2 * (2 : Int));
          (source_ml_source1_temporary_rootNumberPowerInteger_loop43 value root_3))))))

inductive source_ml_source1_temporary_rootNumberPowerBound_join44 where
  | source_ml_source1_temporary_rootNumberPowerBound_join44_next (value : Int) (exponent : Int) : source_ml_source1_temporary_rootNumberPowerBound_join44
  deriving Repr, DecidableEq, Inhabited

def rootNumberPowerBound (value : Int) (exponent : Int) (scale : Int) (upward : Bool) : (Array Int) :=
  (let source_ml_source2_s33 := (if ((Int.tmod exponent (2 : Int)) != (0 : Int)) then (let value_2 := (value * (2 : Int));
    (let exponent_2 := (exponent - (1 : Int));
      (source_ml_source1_temporary_rootNumberPowerBound_join44.source_ml_source1_temporary_rootNumberPowerBound_join44_next value_2 exponent_2))) else (source_ml_source1_temporary_rootNumberPowerBound_join44.source_ml_source1_temporary_rootNumberPowerBound_join44_next value exponent));
    (match source_ml_source2_s33 with
      | source_ml_source1_temporary_rootNumberPowerBound_join44.source_ml_source1_temporary_rootNumberPowerBound_join44_next value_3 exponent_3 => (let square := (value_3 * scale);
          (let root := (rootNumberPowerInteger square);
            (let root_3 := (if (upward && (decide ((root * root) < square))) then (let root_2 := (root + (1 : Int));
              root_2) else root);
              (normalizeNumberPowerBound root_3 (Int.tdiv exponent_3 (2 : Int)) scale upward))))))

def rootNumberPowerInterval (interval : (Array Int)) (scale : Int) : (Array Int) :=
  (let lower := (rootNumberPowerBound (match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then interval[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)) (match (let ml_index := (1 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then interval[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)) scale false);
    (let upper := (rootNumberPowerBound (match (let ml_index := (2 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then interval[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)) (match (let ml_index := (3 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then interval[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)) scale true);
      (#[(match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then lower[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)), (match (let ml_index := (1 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then lower[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)), (match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then upper[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)), (match (let ml_index := (1 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then upper[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int))] : (Array Int))))

inductive source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop46_result where
  | source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop46_done (value : Float) (exponent : Int) : source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop46_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop46 (value : Float) (exponent : Int) : source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop46_result :=
  (if (decide (value < (1 : Float))) then (let value_2 := (value * (2 : Float));
    (let exponent_2 := (exponent - (1 : Int));
      (source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop46 value_2 exponent_2))) else (source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop46_result.source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop46_done value exponent))

inductive source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop45_result where
  | source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop45_done (value : Float) (exponent : Int) : source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop45_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop45 (value : Float) (exponent : Int) : source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop45_result :=
  (if (decide (value ≥ (2 : Float))) then (let value_2 := (value / (2 : Float));
    (let exponent_2 := (exponent + (1 : Int));
      (source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop45 value_2 exponent_2))) else (source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop45_result.source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop45_done value exponent))

def decomposeNumberPowerMagnitude (value : Float) : (Array Int) :=
  (let exponent := (0 : Int);
    (let source_ml_source2_s34 := (source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop45 value exponent);
      (match source_ml_source2_s34 with
        | source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop45_result.source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop45_done value_2 exponent_2 => (let source_ml_source2_s35 := (source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop46 value_2 exponent_2);
            (match source_ml_source2_s35 with
              | source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop46_result.source_ml_source1_temporary_decomposeNumberPowerMagnitude_loop46_done value_3 exponent_3 => (#[(match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then (convertJavaScriptNumberBigInteger (value_3 * (4503599627370496 : Float)))[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)), (exponent_3 - (52 : Int))] : (Array Int)))))))

partial def source_ml_source1_temporary_powerJavaScriptNumber_loop49 (scale : Int) (interval : (Array Int)) (shift : Int) : (Array Int) :=
  (if (decide (shift < (0 : Int))) then (let interval_2 := (rootNumberPowerInterval interval scale);
    (let shift_2 := (shift + (1 : Int));
      (source_ml_source1_temporary_powerJavaScriptNumber_loop49 scale interval_2 shift_2))) else interval)

partial def source_ml_source1_temporary_powerJavaScriptNumber_loop50 (scale : Int) (interval : (Array Int)) (result : (Array Int)) (remaining : Int) : (Array Int) :=
  (if (decide (remaining > (0 : Int))) then (let result_3 := (if ((Int.tmod remaining (2 : Int)) == (1 : Int)) then (let result_2 := (multiplyNumberPowerInterval result interval scale);
    result_2) else result);
    (let remaining_2 := (Int.tdiv remaining (2 : Int));
      (if (decide (remaining_2 > (0 : Int))) then (let interval_2 := (multiplyNumberPowerInterval interval interval scale);
        (source_ml_source1_temporary_powerJavaScriptNumber_loop50 scale interval_2 result_3 remaining_2)) else (source_ml_source1_temporary_powerJavaScriptNumber_loop50 scale interval result_3 remaining_2)))) else result)

inductive source_ml_source1_temporary_powerJavaScriptNumber_loop48_result where
  | source_ml_source1_temporary_powerJavaScriptNumber_loop48_done (interval : (Array Int)) (shift : Int) : source_ml_source1_temporary_powerJavaScriptNumber_loop48_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_source1_temporary_powerJavaScriptNumber_loop48 (scale : Int) (interval : (Array Int)) (shift : Int) : source_ml_source1_temporary_powerJavaScriptNumber_loop48_result :=
  (if (decide (shift > (0 : Int))) then (let interval_2 := (multiplyNumberPowerInterval interval interval scale);
    (let shift_2 := (shift - (1 : Int));
      (source_ml_source1_temporary_powerJavaScriptNumber_loop48 scale interval_2 shift_2))) else (source_ml_source1_temporary_powerJavaScriptNumber_loop48_result.source_ml_source1_temporary_powerJavaScriptNumber_loop48_done interval shift))

partial def source_ml_source1_temporary_powerJavaScriptNumber_loop47 (exponent : Float) (negative : Bool) (input : (Array Int)) (power : (Array Int)) (precision : Int) : Float :=
  (let scale := (match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then (powerJavaScriptBigInteger (2 : Int) precision)[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int));
    (let value := ((match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then input[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)) * (match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then (powerJavaScriptBigInteger (2 : Int) (precision - (52 : Int)))[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)));
      (let initialExponent := ((match (let ml_index := (1 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then input[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)) + (52 : Int));
        (let interval := (#[value, initialExponent, value, initialExponent] : (Array Int));
          (let interval_3 := (if (decide (exponent < (0 : Float))) then (let interval_2 := (invertNumberPowerInterval interval scale);
            interval_2) else interval);
            (let shift := (match (let ml_index := (1 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then power[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int));
              (let source_ml_source2_s38 := (source_ml_source1_temporary_powerJavaScriptNumber_loop48 scale interval_3 shift);
                (match source_ml_source2_s38 with
                  | source_ml_source1_temporary_powerJavaScriptNumber_loop48_result.source_ml_source1_temporary_powerJavaScriptNumber_loop48_done interval_4 shift_2 => (let interval_5 := (source_ml_source1_temporary_powerJavaScriptNumber_loop49 scale interval_4 shift_2);
                      (let result := (#[scale, (0 : Int), scale, (0 : Int)] : (Array Int));
                        (let remaining := (match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then power[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int));
                          (let result_2 := (source_ml_source1_temporary_powerJavaScriptNumber_loop50 scale interval_5 result remaining);
                            (let lower := (roundNumberPowerBound (match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then result_2[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)) (match (let ml_index := (1 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then result_2[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)) precision);
                              (let upper := (roundNumberPowerBound (match (let ml_index := (2 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then result_2[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)) (match (let ml_index := (3 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then result_2[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)) precision);
                                (if (lower == upper) then (if negative then (-lower) else lower) else (let precision_2 := (precision * (2 : Int));
                                  (source_ml_source1_temporary_powerJavaScriptNumber_loop47 exponent negative input power precision_2)))))))))))))))))

def powerJavaScriptNumber (base : Float) (exponent : Float) : Float :=
  (if (Float.isNaN exponent) then (0.0 / 0.0 : Float) else (if (exponent == (0 : Float)) then (1 : Float) else (if (Float.isNaN base) then (0.0 / 0.0 : Float) else (let magnitude := (Float.abs base);
    (if (!(Float.isFinite exponent)) then (if (magnitude == (1 : Float)) then (0.0 / 0.0 : Float) else (if (decide (magnitude > (1 : Float))) then (if (decide (exponent > (0 : Float))) then (1.0 / 0.0 : Float) else (0 : Float)) else (if (decide (exponent > (0 : Float))) then (0 : Float) else (1.0 / 0.0 : Float)))) else (let odd := ((ml_is_integer exponent) && ((ml_float_rem (Float.abs exponent) (2 : Float)) == (1 : Float)));
      (let negative := (((decide (base < (0 : Float))) || ((base == (0 : Float)) && (decide (((1 : Float) / base) < (0 : Float))))) && odd);
        (if (base == (0 : Float)) then (let value := (if (decide (exponent > (0 : Float))) then (0 : Float) else (1.0 / 0.0 : Float));
          (if negative then (-value) else value)) else (if (!(Float.isFinite base)) then (let value_2 := (if (decide (exponent > (0 : Float))) then (1.0 / 0.0 : Float) else (0 : Float));
          (if negative then (-value_2) else value_2)) else (if ((decide (base < (0 : Float))) && (!(ml_is_integer exponent))) then (0.0 / 0.0 : Float) else (if (magnitude == (1 : Float)) then (if negative then (-1 : Float) else (1 : Float)) else (let input := (decomposeNumberPowerMagnitude magnitude);
          (let power := (decomposeNumberPowerMagnitude (Float.abs exponent));
            (let precision := (192 : Int);
              (source_ml_source1_temporary_powerJavaScriptNumber_loop47 exponent negative input power precision)))))))))))))))

def main : IO Unit := do
  pure ()

def convertUnits := convertJavaScriptStringBigIntegerUnits
def convertNumber := convertJavaScriptNumberBigInteger
def convertBoolean := convertJavaScriptBooleanBigInteger
def convertUtf8 := convertJavaScriptUtf8BigInteger
def divide := divideJavaScriptBigInteger
def power := powerJavaScriptBigInteger
def powerNumber := powerJavaScriptNumber
end MLBigIntegerConversion
