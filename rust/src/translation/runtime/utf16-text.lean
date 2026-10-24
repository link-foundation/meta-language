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
