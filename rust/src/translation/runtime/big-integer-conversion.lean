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

def convertJavaScriptBooleanBigInteger (value : Bool) : Int :=
  (if value then (1 : Int) else (0 : Int))

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
  (if ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[112, 97, 114, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83, 116, 114, 105, 110, 103, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[115, 116, 114, 105, 110, 103] : Array Nat) else (if ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 66, 111, 111, 108, 101, 97, 110, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 86, 97, 108, 117, 101] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[98, 111, 111, 108, 101, 97, 110] : Array Nat) else (if ((MLUtf16Text.compare ((operation).map Float.ofNat) (((#[99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 82, 101, 115, 117, 108, 116] : Array Nat)).map Float.ofNat)) == (0 : Float)) then (#[110, 117, 109, 98, 101, 114] : Array Nat) else (#[] : Array Nat))))

def acceptLinkedJavaScriptBigIntegerResult (kind : (Array Nat)) (operation : (Array Nat)) : Bool :=
  (((MLUtf16Text.compare ((kind).map Float.ofNat) (((#[109, 97, 116, 104] : Array Nat)).map Float.ofNat)) == (0 : Float)) && ((MLUtf16Text.compare (((readJavaScriptBigIntegerResultOperation operation)).map Float.ofNat) (((#[] : Array Nat)).map Float.ofNat)) != (0 : Float)))

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
          (let source_ml_source2_s7 := (if ((decide (scalar ≥ (194 : Float))) && (decide (scalar ≤ (223 : Float)))) then (let scalar_2 := (scalar - (192 : Float));
            (let remaining_2 := (1 : Float);
              (let minimum_2 := (128 : Float);
                (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_2 remaining_2 minimum_2)))) else (if ((decide (scalar ≥ (224 : Float))) && (decide (scalar ≤ (239 : Float)))) then (let scalar_3 := (scalar - (224 : Float));
            (let remaining_3 := (2 : Float);
              (let minimum_3 := (2048 : Float);
                (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_3 remaining_3 minimum_3)))) else (if ((decide (scalar ≥ (240 : Float))) && (decide (scalar ≤ (244 : Float)))) then (let scalar_4 := (scalar - (240 : Float));
            (let remaining_4 := (3 : Float);
              (let minimum_4 := (65536 : Float);
                (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_4 remaining_4 minimum_4)))) else (if (decide (scalar ≥ (128 : Float))) then (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_return (#[(-1 : Float)] : (Array Float))) else (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_next scalar remaining minimum)))));
            (match source_ml_source2_s7 with
              | source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3.source_ml_source1_temporary_readJavaScriptUtf8StringUnits_join3_next scalar_5 remaining_5 minimum_5 => (let source_ml_source2_s8 := (source_ml_source1_temporary_readJavaScriptUtf8StringUnits_loop4 bytes index_2 scalar_5 remaining_5);
                  (match source_ml_source2_s8 with
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

partial def source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop6 (units : (Array Float)) (start : Float) («end» : Float) : Float :=
  (if ((decide (start < «end»)) && (acceptJavaScriptWhitespaceUnit (match (let ml_index := start; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)))) then (let start_2 := (start + (1 : Float));
    (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop6 units start_2 «end»)) else start)

partial def source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop7 (units : (Array Float)) (start : Float) («end» : Float) : Float :=
  (if ((decide («end» > start)) && (acceptJavaScriptWhitespaceUnit (match (let ml_index := («end» - (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)))) then (let end_2 := («end» - (1 : Float));
    (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop7 units start end_2)) else «end»)

inductive source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10_result where
  | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10_done (value : Int) : source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10_result
  | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10_return (ml_source1_temporary_result10 : (Array Int)) : source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10 (units : (Array Float)) («end» : Float) (radix : Float) (base : Int) (digits : (Array Int)) (value : Int) (index : Float) : source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10_result :=
  (if (decide (index < «end»)) then (let digit := (readJavaScriptNumericDigitUnit (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)));
    (if ((decide (digit < (0 : Float))) || (decide (digit ≥ radix))) then (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10_result.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10_return (#[] : (Array Int))) else (let value_2 := ((value * base) + (match (let ml_index := digit; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then digits[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)));
      (let index_2 := (index + (1 : Float));
        (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10 units «end» radix base digits value_2 index_2))))) else (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10_result.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10_done value))

inductive source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join9 where
  | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join9_next (start : Float) (radix : Float) (base : Int) : source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join9
  deriving Repr, Inhabited

inductive source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join8 where
  | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join8_next (start : Float) (negative : Bool) (signed : Bool) : source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join8
  deriving Repr, Inhabited

inductive source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5_result where
  | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5_done : source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5_result
  | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5_return (ml_source1_temporary_result10 : (Array Int)) : source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5 (units : (Array Float)) (index : Float) : source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5_result :=
  (if (decide (index < (Float.ofNat units.size))) then (let unit := (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if (((!(ml_is_safe_integer unit)) || (decide (unit < (0 : Float)))) || (decide (unit > (65535 : Float)))) then (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5_result.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5_return (#[] : (Array Int))) else (let index_2 := (index + (1 : Float));
      (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5 units index_2)))) else source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5_result.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5_done)

def convertJavaScriptStringBigIntegerUnits (units : (Array Float)) : (Array Int) :=
  (let index := (0 : Float);
    (let source_ml_source2_s3 := (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5 units index);
      (match source_ml_source2_s3 with
        | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5_result.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5_done => (let start := (0 : Float);
            (let «end» := (Float.ofNat units.size);
              (let start_2 := (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop6 units start «end»);
                (let end_2 := (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop7 units start_2 «end»);
                  (if (start_2 == end_2) then (#[(0 : Int)] : (Array Int)) else (let negative := false;
                    (let signed := false;
                      (let sign := (match (let ml_index := start_2; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
                        (let source_ml_source2_s4 := (if ((sign == (43 : Float)) || (sign == (45 : Float))) then (let negative_2 := (sign == (45 : Float));
                          (let signed_2 := true;
                            (let start_3 := (start_2 + (1 : Float));
                              (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join8.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join8_next start_3 negative_2 signed_2)))) else (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join8.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join8_next start_2 negative signed));
                          (match source_ml_source2_s4 with
                            | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join8.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join8_next start_4 negative_3 signed_3 => (if (start_4 == end_2) then (#[] : (Array Int)) else (let radix := (10 : Float);
                                (let base := (10 : Int);
                                  (let source_ml_source2_s5 := (if (((!signed_3) && ((match (let ml_index := start_4; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)) == (48 : Float))) && (decide ((end_2 - start_4) ≥ (2 : Float)))) then (let marker := (match (let ml_index := (start_4 + (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
                                    (let selected := (if ((marker == (120 : Float)) || (marker == (88 : Float))) then (16 : Float) else (if ((marker == (98 : Float)) || (marker == (66 : Float))) then (2 : Float) else (if ((marker == (111 : Float)) || (marker == (79 : Float))) then (8 : Float) else (0 : Float))));
                                      (if (selected != (0 : Float)) then (let radix_2 := selected;
                                        (let base_2 := (if (selected == (16 : Float)) then (16 : Int) else (if (selected == (8 : Float)) then (8 : Int) else (2 : Int)));
                                          (let start_5 := (start_4 + (2 : Float));
                                            (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join9.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join9_next start_5 radix_2 base_2)))) else (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join9.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join9_next start_4 radix base)))) else (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join9.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join9_next start_4 radix base));
                                    (match source_ml_source2_s5 with
                                      | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join9.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_join9_next start_6 radix_3 base_3 => (if (start_6 == end_2) then (#[] : (Array Int)) else (let digits := (#[(0 : Int), (1 : Int), (2 : Int), (3 : Int), (4 : Int), (5 : Int), (6 : Int), (7 : Int), (8 : Int), (9 : Int), (10 : Int), (11 : Int), (12 : Int), (13 : Int), (14 : Int), (15 : Int)] : (Array Int));
                                          (let value := (0 : Int);
                                            (let index_2 := start_6;
                                              (let source_ml_source2_s6 := (source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10 units end_2 radix_3 base_3 digits value index_2);
                                                (match source_ml_source2_s6 with
                                                  | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10_result.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10_done value_2 => (#[(if negative_3 then (-value_2) else value_2)] : (Array Int))
                                                  | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10_result.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop10_return source_ml_source1_temporary_result10 => source_ml_source1_temporary_result10)))))))))))))))))))))
        | source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5_result.source_ml_source1_temporary_convertJavaScriptStringBigIntegerUnits_loop5_return source_ml_source1_temporary_result10_2 => source_ml_source1_temporary_result10_2)))

def convertJavaScriptUtf8BigInteger (bytes : (Array Float)) : (Array Int) :=
  (convertJavaScriptStringBigIntegerUnits (readJavaScriptUtf8StringUnits bytes))

partial def source_ml_source1_temporary_convertJavaScriptNumberBigInteger_loop11 (value : Float) (integer : Int) (weight : Int) : Int :=
  (if (decide (value ≥ (1 : Float))) then (let integer_3 := (if ((ml_float_rem value (2 : Float)) == (1 : Float)) then (let integer_2 := (integer + weight);
    integer_2) else integer);
    (let value_2 := (Float.floor (value / (2 : Float)));
      (let weight_2 := (weight * (2 : Int));
        (source_ml_source1_temporary_convertJavaScriptNumberBigInteger_loop11 value_2 integer_3 weight_2)))) else integer)

def convertJavaScriptNumberBigInteger (value : Float) : (Array Int) :=
  (if (!(ml_is_integer value)) then (#[] : (Array Int)) else (let negative := (decide (value < (0 : Float)));
    (let value_2 := (Float.abs value);
      (let integer := (0 : Int);
        (let weight := (1 : Int);
          (let integer_2 := (source_ml_source1_temporary_convertJavaScriptNumberBigInteger_loop11 value_2 integer weight);
            (#[(if negative then (-integer_2) else integer_2)] : (Array Int))))))))

def main : IO Unit := do
  pure ()

def convertUnits := convertJavaScriptStringBigIntegerUnits
def convertNumber := convertJavaScriptNumberBigInteger
def convertBoolean := convertJavaScriptBooleanBigInteger
def convertUtf8 := convertJavaScriptUtf8BigInteger
end MLBigIntegerConversion
