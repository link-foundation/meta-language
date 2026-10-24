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

def acceptJavaScriptWhitespaceUnit (unit : Float) : Bool :=
  ((((((((((((decide (unit ≥ (9 : Float))) && (decide (unit ≤ (13 : Float)))) || (unit == (32 : Float))) || (unit == (160 : Float))) || (unit == (5760 : Float))) || ((decide (unit ≥ (8192 : Float))) && (decide (unit ≤ (8202 : Float))))) || (unit == (8232 : Float))) || (unit == (8233 : Float))) || (unit == (8239 : Float))) || (unit == (8287 : Float))) || (unit == (12288 : Float))) || (unit == (65279 : Float)))

def readJavaScriptNumericDigitUnit (unit : Float) : Float :=
  (if ((decide (unit ≥ (48 : Float))) && (decide (unit ≤ (57 : Float)))) then (unit - (48 : Float)) else (if ((decide (unit ≥ (65 : Float))) && (decide (unit ≤ (70 : Float)))) then (unit - (55 : Float)) else (if ((decide (unit ≥ (97 : Float))) && (decide (unit ≤ (102 : Float)))) then (unit - (87 : Float)) else (-1 : Float))))

def convertJavaScriptBooleanBigInteger (value : Bool) : Int :=
  (if value then (1 : Int) else (0 : Int))

def readJavaScriptBigIntegerConversionForm (kind : String) : String :=
  (if ((kind == "int") || (kind == "nat")) then "identity" else (if (kind == "float") then "number" else (if (kind == "string") then "string" else (if (kind == "bool") then "boolean" else (if (kind == "unit") then "undefined" else (if (kind == "option") then "optional" else "unsupported"))))))

def readJavaScriptBigIntegerConversionErrorClass (form : String) : String :=
  (if (form == "string") then "SyntaxError" else (if (form == "number") then "RangeError" else (if ((form == "undefined") || (form == "null")) then "TypeError" else "")))

def readJavaScriptBigIntegerConversionMessagePieces (form : String) : (Array String) :=
  (if (form == "number") then (#["The number ", " cannot be converted to a BigInt because it is not an integer"] : (Array String)) else (if (((form == "string") || (form == "undefined")) || (form == "null")) then (#["Cannot convert ", " to a BigInt"] : (Array String)) else (#[] : (Array String))))

def renderJavaScriptBigIntegerConversionMessage (form : String) (value : String) : String :=
  (let pieces := (readJavaScriptBigIntegerConversionMessagePieces form);
    (if ((Float.ofNat pieces.size) == (2 : Float)) then (((match (let ml_index := (0 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then pieces[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => "") ++ value) ++ (match (let ml_index := (1 : Float); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then pieces[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => "")) else ""))

partial def source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop2 (units : (Array Float)) (start : Float) («end» : Float) : Float :=
  (if ((decide (start < «end»)) && (acceptJavaScriptWhitespaceUnit (match (let ml_index := start; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)))) then (let start_2 := (start + (1 : Float));
    (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop2 units start_2 «end»)) else start)

partial def source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop3 (units : (Array Float)) (start : Float) («end» : Float) : Float :=
  (if ((decide («end» > start)) && (acceptJavaScriptWhitespaceUnit (match (let ml_index := («end» - (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)))) then (let end_2 := («end» - (1 : Float));
    (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop3 units start end_2)) else «end»)

inductive source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_result where
  | source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_done (value : Int) : source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_result
  | source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_return (ml_temporary_result6 : (Array Int)) : source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6 (units : (Array Float)) («end» : Float) (radix : Float) (base : Int) (digits : (Array Int)) (value : Int) (index : Float) : source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_result :=
  (if (decide (index < «end»)) then (let digit := (readJavaScriptNumericDigitUnit (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)));
    (if ((decide (digit < (0 : Float))) || (decide (digit ≥ radix))) then (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_result.source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_return (#[] : (Array Int))) else (let value_2 := ((value * base) + (match (let ml_index := digit; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then digits[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)));
      (let index_2 := (index + (1 : Float));
        (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6 units «end» radix base digits value_2 index_2))))) else (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_result.source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_done value))

inductive source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join5 where
  | source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join5_next (start : Float) (radix : Float) (base : Int) : source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join5
  deriving Repr, Inhabited

inductive source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join4 where
  | source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join4_next (start : Float) (negative : Bool) (signed : Bool) : source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join4
  deriving Repr, Inhabited

inductive source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_result where
  | source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_done : source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_result
  | source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_return (ml_temporary_result6 : (Array Int)) : source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1 (units : (Array Float)) (index : Float) : source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_result :=
  (if (decide (index < (Float.ofNat units.size))) then (let unit := (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if (((!(ml_is_safe_integer unit)) || (decide (unit < (0 : Float)))) || (decide (unit > (65535 : Float)))) then (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_result.source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_return (#[] : (Array Int))) else (let index_2 := (index + (1 : Float));
      (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1 units index_2)))) else source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_result.source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_done)

def convertJavaScriptStringBigIntegerUnits (units : (Array Float)) : (Array Int) :=
  (let index := (0 : Float);
    (let source_ml_source1_s1 := (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1 units index);
      (match source_ml_source1_s1 with
        | source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_result.source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_done => (let start := (0 : Float);
            (let «end» := (Float.ofNat units.size);
              (let start_2 := (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop2 units start «end»);
                (let end_2 := (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop3 units start_2 «end»);
                  (if (start_2 == end_2) then (#[(0 : Int)] : (Array Int)) else (let negative := false;
                    (let signed := false;
                      (let sign := (match (let ml_index := start_2; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
                        (let source_ml_source1_s2 := (if ((sign == (43 : Float)) || (sign == (45 : Float))) then (let negative_2 := (sign == (45 : Float));
                          (let signed_2 := true;
                            (let start_3 := (start_2 + (1 : Float));
                              (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join4.source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join4_next start_3 negative_2 signed_2)))) else (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join4.source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join4_next start_2 negative signed));
                          (match source_ml_source1_s2 with
                            | source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join4.source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join4_next start_4 negative_3 signed_3 => (if (start_4 == end_2) then (#[] : (Array Int)) else (let radix := (10 : Float);
                                (let base := (10 : Int);
                                  (let source_ml_source1_s3 := (if (((!signed_3) && ((match (let ml_index := start_4; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)) == (48 : Float))) && (decide ((end_2 - start_4) ≥ (2 : Float)))) then (let marker := (match (let ml_index := (start_4 + (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
                                    (let selected := (if ((marker == (120 : Float)) || (marker == (88 : Float))) then (16 : Float) else (if ((marker == (98 : Float)) || (marker == (66 : Float))) then (2 : Float) else (if ((marker == (111 : Float)) || (marker == (79 : Float))) then (8 : Float) else (0 : Float))));
                                      (if (selected != (0 : Float)) then (let radix_2 := selected;
                                        (let base_2 := (if (selected == (16 : Float)) then (16 : Int) else (if (selected == (8 : Float)) then (8 : Int) else (2 : Int)));
                                          (let start_5 := (start_4 + (2 : Float));
                                            (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join5.source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join5_next start_5 radix_2 base_2)))) else (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join5.source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join5_next start_4 radix base)))) else (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join5.source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join5_next start_4 radix base));
                                    (match source_ml_source1_s3 with
                                      | source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join5.source_ml_temporary_convertJavaScriptStringBigIntegerUnits_join5_next start_6 radix_3 base_3 => (if (start_6 == end_2) then (#[] : (Array Int)) else (let digits := (#[(0 : Int), (1 : Int), (2 : Int), (3 : Int), (4 : Int), (5 : Int), (6 : Int), (7 : Int), (8 : Int), (9 : Int), (10 : Int), (11 : Int), (12 : Int), (13 : Int), (14 : Int), (15 : Int)] : (Array Int));
                                          (let value := (0 : Int);
                                            (let index_2 := start_6;
                                              (let source_ml_source1_s4 := (source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6 units end_2 radix_3 base_3 digits value index_2);
                                                (match source_ml_source1_s4 with
                                                  | source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_result.source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_done value_2 => (#[(if negative_3 then (-value_2) else value_2)] : (Array Int))
                                                  | source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_result.source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop6_return source_ml_temporary_result6 => source_ml_temporary_result6)))))))))))))))))))))
        | source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_result.source_ml_temporary_convertJavaScriptStringBigIntegerUnits_loop1_return source_ml_temporary_result6_2 => source_ml_temporary_result6_2)))

partial def source_ml_temporary_convertJavaScriptNumberBigInteger_loop7 (value : Float) (integer : Int) (weight : Int) : Int :=
  (if (decide (value ≥ (1 : Float))) then (let integer_3 := (if ((ml_float_rem value (2 : Float)) == (1 : Float)) then (let integer_2 := (integer + weight);
    integer_2) else integer);
    (let value_2 := (Float.floor (value / (2 : Float)));
      (let weight_2 := (weight * (2 : Int));
        (source_ml_temporary_convertJavaScriptNumberBigInteger_loop7 value_2 integer_3 weight_2)))) else integer)

def convertJavaScriptNumberBigInteger (value : Float) : (Array Int) :=
  (if (!(ml_is_integer value)) then (#[] : (Array Int)) else (let negative := (decide (value < (0 : Float)));
    (let value_2 := (Float.abs value);
      (let integer := (0 : Int);
        (let weight := (1 : Int);
          (let integer_2 := (source_ml_temporary_convertJavaScriptNumberBigInteger_loop7 value_2 integer weight);
            (#[(if negative then (-integer_2) else integer_2)] : (Array Int))))))))

def main : IO Unit := do
  pure ()

def convertUnits := convertJavaScriptStringBigIntegerUnits
def convertNumber := convertJavaScriptNumberBigInteger
def convertBoolean := convertJavaScriptBooleanBigInteger
end MLBigIntegerConversion
