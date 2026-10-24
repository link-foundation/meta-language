namespace MLStringNumber
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

def readStringNumberDigit (unit : Float) : Float :=
  (readJavaScriptNumericDigitUnit unit)

def readJavaScriptNumberBuiltinOperation (name : String) : String :=
  (if (name == "Number") then "convertJavaScriptNumber" else (if (name == "isFinite") then "testCoercedJavaScriptNumberFinite" else (if (name == "isNaN") then "testCoercedJavaScriptNumberNaN" else "")))

def readJavaScriptNumberConversionResult (operation : String) : String :=
  (if ((operation == "convertJavaScriptNumber") || (operation == "coerceJavaScriptNumber")) then "float" else (if ((operation == "testCoercedJavaScriptNumberFinite") || (operation == "testCoercedJavaScriptNumberNaN")) then "bool" else ""))

def readJavaScriptNumberConversionForm (kind : String) : String :=
  (if (((((((kind == "float") || (kind == "bool")) || (kind == "string")) || (kind == "unit")) || (kind == "int")) || (kind == "nat")) || (kind == "option")) then kind else "unsupported")

def acceptLinkedJavaScriptNumberConversion (kind : String) (operation : String) : Bool :=
  ((kind == "math") && ((operation == "convertJavaScriptNumber") || (operation == "coerceJavaScriptNumber")))

def readJavaScriptNumberConversionFailure (operation : String) (kind : String) : String :=
  (if ((operation == "coerceJavaScriptNumber") && ((kind == "int") || (kind == "nat"))) then "Cannot convert a BigInt value to a number" else "")

def requireJavaScriptNumberConversionUnits (kind : String) (operation : String) (argument : String) : Bool :=
  ((acceptLinkedJavaScriptNumberConversion kind operation) && (argument == "string"))

def readJavaScriptPrimitiveNumberOperand (language : String) (kind : String) (element : String) : String :=
  (if (language != "JavaScript") then "" else (if (((kind == "bool") || (kind == "string")) || (kind == "unit")) then "coerceJavaScriptNumber" else (if ((kind == "option") && (element == "string")) then "coerceJavaScriptNumber" else "")))

def readJavaScriptPrimitiveNumericInferenceKind (kind : String) : String :=
  (if (((kind == "bool") || (kind == "string")) || (kind == "unit")) then "float" else "")

def readJavaScriptPrimitiveUnaryOperation (symbol : String) : String :=
  (if (symbol == "+") then "coerceJavaScriptNumber" else "")

def readJavaScriptMathematicalConversionForm (language : String) (operation : String) : String :=
  (if (language != "JavaScript") then "" else (if ((operation == "max") || (operation == "min")) then "variadic" else (if (((((((operation == "abs") || (operation == "floor")) || (operation == "ceil")) || (operation == "trunc")) || (operation == "round")) || (operation == "sign")) || (operation == "sqrt")) then "unary" else "")))

def renderJavaScriptNumberScalarConversion (format : String) (operation : String) (form : String) (value : String) (utf16 : Bool) : String :=
  (if (((!(acceptLinkedJavaScriptNumberConversion "math" operation)) || ((readJavaScriptNumberConversionForm form) == "unsupported")) || (form == "option")) then "" else (if (format == "JavaScript") then (if (operation == "convertJavaScriptNumber") then (("Number(" ++ value) ++ ")") else (("(+(" ++ value) ++ "))")) else (if ((format == "Lean") || (format == "Rocq")) then (if (!(readJavaScriptNumberConversionFailure operation form).isEmpty) then "" else (if (form == "float") then value else (if (form == "unit") then (if (format == "Lean") then (("(let _ := " ++ value) ++ "; ((0 : Float) / 0))") else (("(let _ := " ++ value) ++ " in PrimFloat.nan)")) else (if (form == "bool") then (if (format == "Lean") then (("(if " ++ value) ++ " then (1 : Float) else (0 : Float))") else (("(if " ++ value) ++ " then 1%float else 0%float)")) else (if ((form == "int") || (form == "nat")) then (("(MLStringNumber.convertInteger " ++ (if (form == "nat") then (if (format == "Lean") then (("(Int.ofNat " ++ value) ++ ")") else (("(Z.of_N " ++ value) ++ ")")) else value)) ++ ")") else (if (format == "Lean") then (("(MLStringNumber.convertUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] " ++ value) ++ "))") else (("(MLStringNumber.convertUtf8 ((fix ml_numeric_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_numeric_bytes rest) end) " ++ value) ++ "))"))))))) else (if (format != "Rust") then "" else (if (form == "float") then value else (if (form == "bool") then (("(if " ++ value) ++ " { 1.0f64 } else { 0.0f64 })") else (if (form == "unit") then (("{ let _ = " ++ value) ++ "; f64::NAN }") else (if ((form == "int") || (form == "nat")) then (if (operation == "coerceJavaScriptNumber") then (("{ let _ = " ++ value) ++ "; panic!(\"Cannot convert a BigInt value to a number\") }") else (("crate::ml_string_number::convert_java_script_big_integer_number(" ++ value) ++ ")")) else (let units := (if utf16 then (("(" ++ value) ++ ").0.iter().copied()") else (("(" ++ value) ++ ").encode_utf16()"));
    (("crate::ml_string_number::convert_java_script_string_number_units(&" ++ units) ++ ".map(f64::from).collect::<Vec<f64>>())"))))))))))

def readJavaScriptNumberConversionStatement : String :=
  "ECMAScript Number and ToNumber preserve full UTF-16 StringNumericLiteral syntax, exact binary64 ties-to-even rounding, signed zero, Infinity, NaN, primitive absence and argument evaluation order; explicit BigInt conversion rounds once and implicit BigInt conversion throws"

partial def source_ml_source1_temporary_readStringNumberDecimalPower_loop1 (exponent : Float) (value : Int) (base : Int) : Int :=
  (if (decide (exponent > (0 : Float))) then (let value_3 := (if ((ml_float_rem exponent (2 : Float)) == (1 : Float)) then (let value_2 := (value * base);
    value_2) else value);
    (let exponent_2 := (Float.floor (exponent / (2 : Float)));
      (if (decide (exponent_2 > (0 : Float))) then (let base_2 := (base * base);
        (source_ml_source1_temporary_readStringNumberDecimalPower_loop1 exponent_2 value_3 base_2)) else (source_ml_source1_temporary_readStringNumberDecimalPower_loop1 exponent_2 value_3 base)))) else value)

def readStringNumberDecimalPower (exponent : Float) : Int :=
  (let value := (1 : Int);
    (let base := (10 : Int);
      (let value_2 := (source_ml_source1_temporary_readStringNumberDecimalPower_loop1 exponent value base);
        value_2)))

partial def source_ml_source1_temporary_readStringNumberExactMantissa_loop2 (value : Int) (result : Float) (scale : Float) : Float :=
  (if (decide (value > (0 : Int))) then (let result_3 := (if ((Int.tmod value (2 : Int)) == (1 : Int)) then (let result_2 := (result + scale);
    result_2) else result);
    (let value_2 := (Int.tdiv value (2 : Int));
      (let scale_2 := (scale * (2 : Float));
        (source_ml_source1_temporary_readStringNumberExactMantissa_loop2 value_2 result_3 scale_2)))) else result)

def readStringNumberExactMantissa (value : Int) : Float :=
  (let result := (0 : Float);
    (let scale := (1 : Float);
      (let result_2 := (source_ml_source1_temporary_readStringNumberExactMantissa_loop2 value result scale);
        result_2)))

partial def source_ml_source1_temporary_readStringNumberBinaryPower_loop4 (exponent : Float) (result : Float) : Float :=
  (if (decide (exponent < (0 : Float))) then (let result_2 := (result / (2 : Float));
    (let exponent_2 := (exponent + (1 : Float));
      (source_ml_source1_temporary_readStringNumberBinaryPower_loop4 exponent_2 result_2))) else result)

inductive source_ml_source1_temporary_readStringNumberBinaryPower_loop3_result where
  | source_ml_source1_temporary_readStringNumberBinaryPower_loop3_done (exponent : Float) (result : Float) : source_ml_source1_temporary_readStringNumberBinaryPower_loop3_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_readStringNumberBinaryPower_loop3 (exponent : Float) (result : Float) : source_ml_source1_temporary_readStringNumberBinaryPower_loop3_result :=
  (if (decide (exponent > (0 : Float))) then (let result_2 := (result * (2 : Float));
    (let exponent_2 := (exponent - (1 : Float));
      (source_ml_source1_temporary_readStringNumberBinaryPower_loop3 exponent_2 result_2))) else (source_ml_source1_temporary_readStringNumberBinaryPower_loop3_result.source_ml_source1_temporary_readStringNumberBinaryPower_loop3_done exponent result))

def readStringNumberBinaryPower (exponent : Float) : Float :=
  (let result := (1 : Float);
    (let source_ml_source2_s1 := (source_ml_source1_temporary_readStringNumberBinaryPower_loop3 exponent result);
      (match source_ml_source2_s1 with
        | source_ml_source1_temporary_readStringNumberBinaryPower_loop3_result.source_ml_source1_temporary_readStringNumberBinaryPower_loop3_done exponent_2 result_2 => (let result_3 := (source_ml_source1_temporary_readStringNumberBinaryPower_loop4 exponent_2 result_2);
            result_3))))

inductive source_ml_source1_temporary_convertStringNumberRational_loop8_result where
  | source_ml_source1_temporary_convertStringNumberRational_loop8_done (quotient : Int) (remainder : Int) : source_ml_source1_temporary_convertStringNumberRational_loop8_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_source1_temporary_convertStringNumberRational_loop8 (quotient : Int) (remainder : Int) (scale : Int) (weight : Int) : source_ml_source1_temporary_convertStringNumberRational_loop8_result :=
  (if (decide (weight > (1 : Int))) then (let scale_2 := (Int.tdiv scale (2 : Int));
    (let weight_2 := (Int.tdiv weight (2 : Int));
      (if (decide (remainder ≥ scale_2)) then (let remainder_2 := (remainder - scale_2);
        (let quotient_2 := (quotient + weight_2);
          (source_ml_source1_temporary_convertStringNumberRational_loop8 quotient_2 remainder_2 scale_2 weight_2))) else (source_ml_source1_temporary_convertStringNumberRational_loop8 quotient remainder scale_2 weight_2)))) else (source_ml_source1_temporary_convertStringNumberRational_loop8_result.source_ml_source1_temporary_convertStringNumberRational_loop8_done quotient remainder))

inductive source_ml_source1_temporary_convertStringNumberRational_loop7_result where
  | source_ml_source1_temporary_convertStringNumberRational_loop7_done (scale : Int) (weight : Int) : source_ml_source1_temporary_convertStringNumberRational_loop7_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_source1_temporary_convertStringNumberRational_loop7 (remainder : Int) (scale : Int) (weight : Int) : source_ml_source1_temporary_convertStringNumberRational_loop7_result :=
  (if (decide (scale ≤ remainder)) then (let scale_2 := (scale * (2 : Int));
    (let weight_2 := (weight * (2 : Int));
      (source_ml_source1_temporary_convertStringNumberRational_loop7 remainder scale_2 weight_2))) else (source_ml_source1_temporary_convertStringNumberRational_loop7_result.source_ml_source1_temporary_convertStringNumberRational_loop7_done scale weight))

inductive source_ml_source1_temporary_convertStringNumberRational_join6 where
  | source_ml_source1_temporary_convertStringNumberRational_join6_next (numerator : Int) (denominator : Int) : source_ml_source1_temporary_convertStringNumberRational_join6
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_source1_temporary_convertStringNumberRational_loop9 (numerator : Int) (shift : Float) (index : Float) : Int :=
  (if (decide (index < shift)) then (let numerator_2 := (numerator * (2 : Int));
    (let index_2 := (index + (1 : Float));
      (source_ml_source1_temporary_convertStringNumberRational_loop9 numerator_2 shift index_2))) else numerator)

partial def source_ml_source1_temporary_convertStringNumberRational_loop10 (denominator : Int) (shift : Float) (index : Float) : Int :=
  (if (decide (index < (-shift))) then (let denominator_2 := (denominator * (2 : Int));
    (let index_2 := (index + (1 : Float));
      (source_ml_source1_temporary_convertStringNumberRational_loop10 denominator_2 shift index_2))) else denominator)

inductive source_ml_source1_temporary_convertStringNumberRational_join5 where
  | source_ml_source1_temporary_convertStringNumberRational_join5_next (exponent : Float) : source_ml_source1_temporary_convertStringNumberRational_join5
  | source_ml_source1_temporary_convertStringNumberRational_join5_return (ml_source1_temporary_result10 : Float) : source_ml_source1_temporary_convertStringNumberRational_join5
  deriving Repr, Inhabited

inductive source_ml_source1_temporary_convertStringNumberRational_loop11_result where
  | source_ml_source1_temporary_convertStringNumberRational_loop11_done (exponent : Float) : source_ml_source1_temporary_convertStringNumberRational_loop11_result
  | source_ml_source1_temporary_convertStringNumberRational_loop11_return (ml_source1_temporary_result11 : Float) : source_ml_source1_temporary_convertStringNumberRational_loop11_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_convertStringNumberRational_loop11 (numerator : Int) (exponent : Float) (scale : Int) : source_ml_source1_temporary_convertStringNumberRational_loop11_result :=
  (if (decide (numerator ≥ (scale * (2 : Int)))) then (let scale_2 := (scale * (2 : Int));
    (let exponent_2 := (exponent + (1 : Float));
      (if (decide (exponent_2 ≥ (1024 : Float))) then (source_ml_source1_temporary_convertStringNumberRational_loop11_result.source_ml_source1_temporary_convertStringNumberRational_loop11_return (1.0 / 0.0 : Float)) else (source_ml_source1_temporary_convertStringNumberRational_loop11 numerator exponent_2 scale_2)))) else (source_ml_source1_temporary_convertStringNumberRational_loop11_result.source_ml_source1_temporary_convertStringNumberRational_loop11_done exponent))

inductive source_ml_source1_temporary_convertStringNumberRational_loop12_result where
  | source_ml_source1_temporary_convertStringNumberRational_loop12_done (exponent : Float) : source_ml_source1_temporary_convertStringNumberRational_loop12_result
  | source_ml_source1_temporary_convertStringNumberRational_loop12_return (ml_source1_temporary_result12 : Float) : source_ml_source1_temporary_convertStringNumberRational_loop12_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_convertStringNumberRational_loop12 (denominator : Int) (exponent : Float) (scale : Int) : source_ml_source1_temporary_convertStringNumberRational_loop12_result :=
  (if (decide (scale < denominator)) then (let scale_2 := (scale * (2 : Int));
    (let exponent_2 := (exponent - (1 : Float));
      (if (decide (exponent_2 < (-1075 : Float))) then (source_ml_source1_temporary_convertStringNumberRational_loop12_result.source_ml_source1_temporary_convertStringNumberRational_loop12_return (0 : Float)) else (source_ml_source1_temporary_convertStringNumberRational_loop12 denominator exponent_2 scale_2)))) else (source_ml_source1_temporary_convertStringNumberRational_loop12_result.source_ml_source1_temporary_convertStringNumberRational_loop12_done exponent))

def convertStringNumberRational (numerator : Int) (denominator : Int) : Float :=
  (if ((decide (numerator < (0 : Int))) || (decide (denominator ≤ (0 : Int)))) then (0.0 / 0.0 : Float) else (if (numerator == (0 : Int)) then (0 : Float) else (let exponent := (0 : Float);
    (let source_ml_source2_s4 := (if (decide (numerator ≥ denominator)) then (let scale := denominator;
      (let source_ml_source2_s5 := (source_ml_source1_temporary_convertStringNumberRational_loop11 numerator exponent scale);
        (match source_ml_source2_s5 with
          | source_ml_source1_temporary_convertStringNumberRational_loop11_result.source_ml_source1_temporary_convertStringNumberRational_loop11_done exponent_2 => (source_ml_source1_temporary_convertStringNumberRational_join5.source_ml_source1_temporary_convertStringNumberRational_join5_next exponent_2)
          | source_ml_source1_temporary_convertStringNumberRational_loop11_result.source_ml_source1_temporary_convertStringNumberRational_loop11_return source_ml_source1_temporary_result11 => (source_ml_source1_temporary_convertStringNumberRational_join5.source_ml_source1_temporary_convertStringNumberRational_join5_return source_ml_source1_temporary_result11)))) else (let scale_2 := numerator;
      (let source_ml_source2_s6 := (source_ml_source1_temporary_convertStringNumberRational_loop12 denominator exponent scale_2);
        (match source_ml_source2_s6 with
          | source_ml_source1_temporary_convertStringNumberRational_loop12_result.source_ml_source1_temporary_convertStringNumberRational_loop12_done exponent_3 => (source_ml_source1_temporary_convertStringNumberRational_join5.source_ml_source1_temporary_convertStringNumberRational_join5_next exponent_3)
          | source_ml_source1_temporary_convertStringNumberRational_loop12_result.source_ml_source1_temporary_convertStringNumberRational_loop12_return source_ml_source1_temporary_result12 => (source_ml_source1_temporary_convertStringNumberRational_join5.source_ml_source1_temporary_convertStringNumberRational_join5_return source_ml_source1_temporary_result12)))));
      (match source_ml_source2_s4 with
        | source_ml_source1_temporary_convertStringNumberRational_join5.source_ml_source1_temporary_convertStringNumberRational_join5_next exponent_4 => (let shift := (if (decide (exponent_4 ≥ (-1022 : Float))) then ((52 : Float) - exponent_4) else (1074 : Float));
            (let source_ml_source2_s7 := (if (decide (shift ≥ (0 : Float))) then (let index := (0 : Float);
              (let numerator_2 := (source_ml_source1_temporary_convertStringNumberRational_loop9 numerator shift index);
                (source_ml_source1_temporary_convertStringNumberRational_join6.source_ml_source1_temporary_convertStringNumberRational_join6_next numerator_2 denominator))) else (let index_2 := (0 : Float);
              (let denominator_2 := (source_ml_source1_temporary_convertStringNumberRational_loop10 denominator shift index_2);
                (source_ml_source1_temporary_convertStringNumberRational_join6.source_ml_source1_temporary_convertStringNumberRational_join6_next numerator denominator_2))));
              (match source_ml_source2_s7 with
                | source_ml_source1_temporary_convertStringNumberRational_join6.source_ml_source1_temporary_convertStringNumberRational_join6_next numerator_3 denominator_3 => (let quotient := (0 : Int);
                    (let remainder := numerator_3;
                      (let scale_3 := denominator_3;
                        (let weight := (1 : Int);
                          (let source_ml_source2_s8 := (source_ml_source1_temporary_convertStringNumberRational_loop7 remainder scale_3 weight);
                            (match source_ml_source2_s8 with
                              | source_ml_source1_temporary_convertStringNumberRational_loop7_result.source_ml_source1_temporary_convertStringNumberRational_loop7_done scale_4 weight_2 => (let source_ml_source2_s9 := (source_ml_source1_temporary_convertStringNumberRational_loop8 quotient remainder scale_4 weight_2);
                                  (match source_ml_source2_s9 with
                                    | source_ml_source1_temporary_convertStringNumberRational_loop8_result.source_ml_source1_temporary_convertStringNumberRational_loop8_done quotient_2 remainder_2 => (let twice := (remainder_2 * (2 : Int));
                                        (let quotient_4 := (if ((decide (twice > denominator_3)) || ((twice == denominator_3) && ((Int.tmod quotient_2 (2 : Int)) == (1 : Int)))) then (let quotient_3 := (quotient_2 + (1 : Int));
                                          quotient_3) else quotient_2);
                                          ((readStringNumberExactMantissa quotient_4) * (readStringNumberBinaryPower (-shift))))))))))))))))
        | source_ml_source1_temporary_convertStringNumberRational_join5.source_ml_source1_temporary_convertStringNumberRational_join5_return source_ml_source1_temporary_result10 => source_ml_source1_temporary_result10)))))

def convertJavaScriptBigIntegerNumber (value : Int) : Float :=
  (if (decide (value < (0 : Int))) then (-(convertStringNumberRational (-value) (1 : Int))) else (convertStringNumberRational value (1 : Int)))

inductive source_ml_source1_temporary_acceptStringNumberInfinity_loop13_result where
  | source_ml_source1_temporary_acceptStringNumberInfinity_loop13_done : source_ml_source1_temporary_acceptStringNumberInfinity_loop13_result
  | source_ml_source1_temporary_acceptStringNumberInfinity_loop13_return (ml_source1_temporary_result13 : Bool) : source_ml_source1_temporary_acceptStringNumberInfinity_loop13_result
  deriving Repr, DecidableEq, Inhabited

partial def source_ml_source1_temporary_acceptStringNumberInfinity_loop13 (units : (Array Float)) (start : Float) (expected : (Array Float)) (index : Float) : source_ml_source1_temporary_acceptStringNumberInfinity_loop13_result :=
  (if (decide (index < (Float.ofNat expected.size))) then (if ((match (let ml_index := (start + index); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)) != (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then expected[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-2 : Float))) then (source_ml_source1_temporary_acceptStringNumberInfinity_loop13_result.source_ml_source1_temporary_acceptStringNumberInfinity_loop13_return false) else (let index_2 := (index + (1 : Float));
    (source_ml_source1_temporary_acceptStringNumberInfinity_loop13 units start expected index_2))) else source_ml_source1_temporary_acceptStringNumberInfinity_loop13_result.source_ml_source1_temporary_acceptStringNumberInfinity_loop13_done)

def acceptStringNumberInfinity (units : (Array Float)) (start : Float) («end» : Float) : Bool :=
  (if ((«end» - start) != (8 : Float)) then false else (let expected := (#[(73 : Float), (110 : Float), (102 : Float), (105 : Float), (110 : Float), (105 : Float), (116 : Float), (121 : Float)] : (Array Float));
    (let index := (0 : Float);
      (let source_ml_source2_s10 := (source_ml_source1_temporary_acceptStringNumberInfinity_loop13 units start expected index);
        (match source_ml_source2_s10 with
          | source_ml_source1_temporary_acceptStringNumberInfinity_loop13_result.source_ml_source1_temporary_acceptStringNumberInfinity_loop13_done => true
          | source_ml_source1_temporary_acceptStringNumberInfinity_loop13_result.source_ml_source1_temporary_acceptStringNumberInfinity_loop13_return source_ml_source1_temporary_result13 => source_ml_source1_temporary_result13)))))

partial def source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop15 (units : (Array Float)) (start : Float) («end» : Float) : Float :=
  (if ((decide (start < «end»)) && (acceptJavaScriptWhitespaceUnit (match (let ml_index := start; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)))) then (let start_2 := (start + (1 : Float));
    (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop15 units start_2 «end»)) else start)

partial def source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop16 (units : (Array Float)) (start : Float) («end» : Float) : Float :=
  (if ((decide («end» > start)) && (acceptJavaScriptWhitespaceUnit (match (let ml_index := («end» - (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)))) then (let end_2 := («end» - (1 : Float));
    (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop16 units start end_2)) else «end»)

partial def source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop22 (units : (Array Float)) (integerDigits : (Array Int)) (mantissaEnd : Float) (mantissa : Int) (index : Float) : Int :=
  (if (decide (index < mantissaEnd)) then (let unit := (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if (unit != (46 : Float)) then (let mantissa_2 := ((mantissa * (10 : Int)) + (match (let ml_index := (unit - (48 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then integerDigits[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)));
      (let index_2 := (index + (1 : Float));
        (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop22 units integerDigits mantissaEnd mantissa_2 index_2))) else (let index_3 := (index + (1 : Float));
      (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop22 units integerDigits mantissaEnd mantissa index_3)))) else mantissa)

inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21_result where
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21_done (exponent : Float) (mantissaEnd : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21 (units : (Array Float)) (start : Float) (exponent : Float) (mantissaEnd : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21_result :=
  (if (decide (mantissaEnd > start)) then (let unit := (match (let ml_index := (mantissaEnd - (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if (unit == (46 : Float)) then (let mantissaEnd_2 := (mantissaEnd - (1 : Float));
      (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21 units start exponent mantissaEnd_2)) else (if (unit != (48 : Float)) then (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21_done exponent mantissaEnd) else (let mantissaEnd_3 := (mantissaEnd - (1 : Float));
      (let exponent_2 := (exponent + (1 : Float));
        (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21 units start exponent_2 mantissaEnd_3)))))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21_done exponent mantissaEnd))

inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20 where
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20_next («at» : Float) (exponent : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20_return (ml_source1_temporary_result22 : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20
  deriving Repr, Inhabited

inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24_result where
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24_done («at» : Float) (exponent : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24 (units : (Array Float)) («end» : Float) («at» : Float) (exponent : Float) (exponentLimit : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24_result :=
  (if (decide («at» < «end»)) then (let digit := ((match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)) - (48 : Float));
    (if ((decide (digit < (0 : Float))) || (decide (digit > (9 : Float)))) then (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24_done «at» exponent) else (let exponent_3 := (if (decide (exponent < exponentLimit)) then (let exponent_2 := (ml_min exponentLimit ((exponent * (10 : Float)) + digit));
      exponent_2) else exponent);
      (let at_2 := («at» + (1 : Float));
        (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24 units «end» at_2 exponent_3 exponentLimit))))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24_done «at» exponent))

inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join23 where
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join23_next («at» : Float) (exponentNegative : Bool) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join23
  deriving Repr, Inhabited

inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19_result where
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19_done («at» : Float) (fraction : Float) (digits : Float) (significant : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19 (units : (Array Float)) («end» : Float) («at» : Float) (fraction : Float) (digits : Float) (significant : Float) (dot : Bool) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19_result :=
  (if (decide («at» < «end»)) then (let unit := (match (let ml_index := «at»; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if ((decide (unit ≥ (48 : Float))) && (decide (unit ≤ (57 : Float)))) then (let digits_2 := (digits + (1 : Float));
      (let fraction_3 := (if dot then (let fraction_2 := (fraction + (1 : Float));
        fraction_2) else fraction);
        (let significant_3 := (if ((unit != (48 : Float)) || (decide (significant > (0 : Float)))) then (let significant_2 := (significant + (1 : Float));
          significant_2) else significant);
          (let at_2 := («at» + (1 : Float));
            (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19 units «end» at_2 fraction_3 digits_2 significant_3 dot))))) else (if ((unit == (46 : Float)) && (!dot)) then (let dot_2 := true;
      (let at_3 := («at» + (1 : Float));
        (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19 units «end» at_3 fraction digits significant dot_2))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19_done «at» fraction digits significant)))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19_done «at» fraction digits significant))

inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18 where
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18_next (start : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18_return (ml_source1_temporary_result24 : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18
  deriving Repr, Inhabited

inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_result where
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_done (value : Int) (overflow : Bool) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_result
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_return (ml_source1_temporary_result25 : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25 (units : (Array Float)) («end» : Float) (integerDigits : (Array Int)) (radix : Float) (value : Int) (overflow : Bool) (limit : Int) (base : Int) (index : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_result :=
  (if (decide (index < «end»)) then (let digit := (readStringNumberDigit (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)));
    (if ((decide (digit < (0 : Float))) || (decide (digit ≥ radix))) then (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_return (0.0 / 0.0 : Float)) else (if (!overflow) then (let value_2 := ((value * base) + (match (let ml_index := digit; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then integerDigits[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (0 : Int)));
      (let overflow_2 := (decide (value_2 > limit));
        (let index_2 := (index + (1 : Float));
          (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25 units «end» integerDigits radix value_2 overflow_2 limit base index_2)))) else (let index_3 := (index + (1 : Float));
      (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25 units «end» integerDigits radix value overflow limit base index_3))))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_done value overflow))

inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join17 where
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join17_next (start : Float) (negative : Bool) (signed : Bool) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join17
  deriving Repr, Inhabited

inductive source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_result where
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_done : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_result
  | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_return (ml_source1_temporary_result25 : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14 (units : (Array Float)) (index : Float) : source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_result :=
  (if (decide (index < (Float.ofNat units.size))) then (let unit := (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if (((!(ml_is_safe_integer unit)) || (decide (unit < (0 : Float)))) || (decide (unit > (65535 : Float)))) then (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_return (0.0 / 0.0 : Float)) else (let index_2 := (index + (1 : Float));
      (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14 units index_2)))) else source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_done)

def convertJavaScriptStringNumberUnits (units : (Array Float)) : Float :=
  (let index := (0 : Float);
    (let source_ml_source2_s11 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14 units index);
      (match source_ml_source2_s11 with
        | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_done => (let start := (0 : Float);
            (let «end» := (Float.ofNat units.size);
              (let start_2 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop15 units start «end»);
                (let end_2 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop16 units start_2 «end»);
                  (if (start_2 == end_2) then (0 : Float) else (let negative := false;
                    (let signed := false;
                      (let sign := (match (let ml_index := start_2; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
                        (let source_ml_source2_s12 := (if ((sign == (43 : Float)) || (sign == (45 : Float))) then (let negative_2 := (sign == (45 : Float));
                          (let signed_2 := true;
                            (let start_3 := (start_2 + (1 : Float));
                              (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join17.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join17_next start_3 negative_2 signed_2)))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join17.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join17_next start_2 negative signed));
                          (match source_ml_source2_s12 with
                            | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join17.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join17_next start_4 negative_3 signed_3 => (if (start_4 == end_2) then (0.0 / 0.0 : Float) else (if (acceptStringNumberInfinity units start_4 end_2) then (if negative_3 then (-1.0 / 0.0 : Float) else (1.0 / 0.0 : Float)) else (let integerDigits := (#[(0 : Int), (1 : Int), (2 : Int), (3 : Int), (4 : Int), (5 : Int), (6 : Int), (7 : Int), (8 : Int), (9 : Int), (10 : Int), (11 : Int), (12 : Int), (13 : Int), (14 : Int), (15 : Int)] : (Array Int));
                                (let source_ml_source2_s14 := (if (((!signed_3) && ((match (let ml_index := start_4; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)) == (48 : Float))) && (decide ((end_2 - start_4) ≥ (2 : Float)))) then (let marker := (match (let ml_index := (start_4 + (1 : Float)); if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
                                  (let radix := (if ((marker == (120 : Float)) || (marker == (88 : Float))) then (16 : Float) else (if ((marker == (98 : Float)) || (marker == (66 : Float))) then (2 : Float) else (if ((marker == (111 : Float)) || (marker == (79 : Float))) then (8 : Float) else (0 : Float))));
                                    (if (radix != (0 : Float)) then (let value := (0 : Int);
                                      (let overflow := false;
                                        (let limit := (readStringNumberDecimalPower (310 : Float));
                                          (let base := (if (radix == (16 : Float)) then (16 : Int) else (if (radix == (8 : Float)) then (8 : Int) else (2 : Int)));
                                            (let start_5 := (start_4 + (2 : Float));
                                              (if (start_5 == end_2) then (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18_return (0.0 / 0.0 : Float)) else (let index_2 := start_5;
                                                (let source_ml_source2_s15 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25 units end_2 integerDigits radix value overflow limit base index_2);
                                                  (match source_ml_source2_s15 with
                                                    | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_done value_2 overflow_2 => (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18_return (if overflow_2 then (1.0 / 0.0 : Float) else (convertStringNumberRational value_2 (1 : Int))))
                                                    | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop25_return source_ml_source1_temporary_result25 => (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18_return source_ml_source1_temporary_result25)))))))))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18_next start_4)))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18_next start_4));
                                  (match source_ml_source2_s14 with
                                    | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18_next start_6 => (let «at» := start_6;
                                        (let fraction := (0 : Float);
                                          (let digits := (0 : Float);
                                            (let significant := (0 : Float);
                                              (let dot := false;
                                                (let source_ml_source2_s16 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19 units end_2 «at» fraction digits significant dot);
                                                  (match source_ml_source2_s16 with
                                                    | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop19_done at_2 fraction_2 digits_2 significant_2 => (if (digits_2 == (0 : Float)) then (0.0 / 0.0 : Float) else (let decimalEnd := at_2;
                                                        (let exponent := (0 : Float);
                                                          (let exponentLimit := ((Float.ofNat units.size) + (10000 : Float));
                                                            (let source_ml_source2_s19 := (if ((decide (at_2 < end_2)) && (((match (let ml_index := at_2; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)) == (101 : Float)) || ((match (let ml_index := at_2; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float)) == (69 : Float)))) then (let at_3 := (at_2 + (1 : Float));
                                                              (let exponentNegative := false;
                                                                (let exponentSign := (match (let ml_index := at_3; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then units[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
                                                                  (let source_ml_source2_s20 := (if ((exponentSign == (43 : Float)) || (exponentSign == (45 : Float))) then (let exponentNegative_2 := (exponentSign == (45 : Float));
                                                                    (let at_4 := (at_3 + (1 : Float));
                                                                      (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join23.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join23_next at_4 exponentNegative_2))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join23.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join23_next at_3 exponentNegative));
                                                                    (match source_ml_source2_s20 with
                                                                      | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join23.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join23_next at_5 exponentNegative_3 => (let first := at_5;
                                                                          (let source_ml_source2_s21 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24 units end_2 at_5 exponent exponentLimit);
                                                                            (match source_ml_source2_s21 with
                                                                              | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop24_done at_6 exponent_2 => (if (at_6 == first) then (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20_return (0.0 / 0.0 : Float)) else (if exponentNegative_3 then (let exponent_3 := (-exponent_2);
                                                                                  (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20_next at_6 exponent_3)) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20_next at_6 exponent_2))))))))))) else (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20_next at_2 exponent));
                                                              (match source_ml_source2_s19 with
                                                                | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20_next at_7 exponent_4 => (if (at_7 != end_2) then (0.0 / 0.0 : Float) else (if (significant_2 == (0 : Float)) then (if negative_3 then (-0 : Float) else (0 : Float)) else (let exponent_5 := (exponent_4 - fraction_2);
                                                                    (let order := (significant_2 + exponent_5);
                                                                      (if (decide (order > (310 : Float))) then (if negative_3 then (-1.0 / 0.0 : Float) else (1.0 / 0.0 : Float)) else (if (decide (order < (-325 : Float))) then (if negative_3 then (-0 : Float) else (0 : Float)) else (let mantissaEnd := decimalEnd;
                                                                        (let source_ml_source2_s22 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21 units start_6 exponent_5 mantissaEnd);
                                                                          (match source_ml_source2_s22 with
                                                                            | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop21_done exponent_6 mantissaEnd_2 => (let mantissa := (0 : Int);
                                                                                (let index_3 := start_6;
                                                                                  (let mantissa_2 := (source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop22 units integerDigits mantissaEnd_2 mantissa index_3);
                                                                                    (let numerator := (if (decide (exponent_6 ≥ (0 : Float))) then (mantissa_2 * (readStringNumberDecimalPower exponent_6)) else mantissa_2);
                                                                                      (let denominator := (if (decide (exponent_6 < (0 : Float))) then (readStringNumberDecimalPower (-exponent_6)) else (1 : Int));
                                                                                        (let value_3 := (convertStringNumberRational numerator denominator);
                                                                                          (if negative_3 then (-value_3) else value_3))))))))))))))))
                                                                | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join20_return source_ml_source1_temporary_result22 => source_ml_source1_temporary_result22)))))))))))))
                                    | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_join18_return source_ml_source1_temporary_result24 => source_ml_source1_temporary_result24)))))))))))))))
        | source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_result.source_ml_source1_temporary_convertJavaScriptStringNumberUnits_loop14_return source_ml_source1_temporary_result25_2 => source_ml_source1_temporary_result25_2)))

inductive source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_result where
  | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_done (units : (Array Float)) : source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_result
  | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_return (ml_source1_temporary_result27 : Float) : source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_result
  deriving Repr, Inhabited

inductive source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_result where
  | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_done (index : Float) (scalar : Float) : source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_result
  | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_return (ml_source1_temporary_result29 : Float) : source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29 (bytes : (Array Float)) (index : Float) (scalar : Float) (remaining : Float) : source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_result :=
  (if (decide (remaining > (0 : Float))) then (let next := (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then bytes[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if ((decide (next < (128 : Float))) || (decide (next > (191 : Float)))) then (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_result.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_return (0.0 / 0.0 : Float)) else (let scalar_2 := (((scalar * (64 : Float)) + next) - (128 : Float));
      (let index_2 := (index + (1 : Float));
        (let remaining_2 := (remaining - (1 : Float));
          (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29 bytes index_2 scalar_2 remaining_2)))))) else (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_result.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_done index scalar))

inductive source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28 where
  | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28_next (scalar : Float) (remaining : Float) (minimum : Float) : source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28
  | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28_return (ml_source1_temporary_result29 : Float) : source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27 (bytes : (Array Float)) (units : (Array Float)) (index : Float) : source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_result :=
  (if (decide (index < (Float.ofNat bytes.size))) then (let scalar := (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then bytes[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (let index_2 := (index + (1 : Float));
      (let remaining := (0 : Float);
        (let minimum := (0 : Float);
          (let source_ml_source2_s25 := (if ((decide (scalar ≥ (194 : Float))) && (decide (scalar ≤ (223 : Float)))) then (let scalar_2 := (scalar - (192 : Float));
            (let remaining_2 := (1 : Float);
              (let minimum_2 := (128 : Float);
                (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28_next scalar_2 remaining_2 minimum_2)))) else (if ((decide (scalar ≥ (224 : Float))) && (decide (scalar ≤ (239 : Float)))) then (let scalar_3 := (scalar - (224 : Float));
            (let remaining_3 := (2 : Float);
              (let minimum_3 := (2048 : Float);
                (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28_next scalar_3 remaining_3 minimum_3)))) else (if ((decide (scalar ≥ (240 : Float))) && (decide (scalar ≤ (244 : Float)))) then (let scalar_4 := (scalar - (240 : Float));
            (let remaining_4 := (3 : Float);
              (let minimum_4 := (65536 : Float);
                (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28_next scalar_4 remaining_4 minimum_4)))) else (if (decide (scalar ≥ (128 : Float))) then (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28_return (0.0 / 0.0 : Float)) else (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28_next scalar remaining minimum)))));
            (match source_ml_source2_s25 with
              | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28_next scalar_5 remaining_5 minimum_5 => (let source_ml_source2_s26 := (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29 bytes index_2 scalar_5 remaining_5);
                  (match source_ml_source2_s26 with
                    | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_result.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_done index_3 scalar_6 => (if (((decide (scalar_6 < minimum_5)) || (decide (scalar_6 > (1114111 : Float)))) || ((decide (scalar_6 ≥ (55296 : Float))) && (decide (scalar_6 ≤ (57343 : Float))))) then (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_result.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_return (0.0 / 0.0 : Float)) else (if (decide (scalar_6 ≤ (65535 : Float))) then (let units_2 := (units ++ (#[scalar_6] : (Array Float)));
                        (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27 bytes units_2 index_3)) else (let units_3 := (units ++ (#[((55296 : Float) + (Float.floor ((scalar_6 - (65536 : Float)) / (1024 : Float))))] : (Array Float)));
                        (let units_4 := (units_3 ++ (#[((56320 : Float) + (ml_float_rem (scalar_6 - (65536 : Float)) (1024 : Float)))] : (Array Float)));
                          (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27 bytes units_4 index_3)))))
                    | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_result.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop29_return source_ml_source1_temporary_result29 => (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_result.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_return source_ml_source1_temporary_result29)))
              | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_join28_return source_ml_source1_temporary_result29_2 => (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_result.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_return source_ml_source1_temporary_result29_2))))))) else (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_result.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_done units))

inductive source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_result where
  | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_done : source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_result
  | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_return (ml_source1_temporary_result29 : Float) : source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_result
  deriving Repr, Inhabited

partial def source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26 (bytes : (Array Float)) (index : Float) : source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_result :=
  (if (decide (index < (Float.ofNat bytes.size))) then (let unit := (match (let ml_index := index; if ml_index ≥ 0 && ml_index.floor == ml_index && ml_index < 9007199254740992 then bytes[ml_index.toUInt64.toNat]? else none) with | some ml_value => ml_value | _ => (-1 : Float));
    (if (((!(ml_is_safe_integer unit)) || (decide (unit < (0 : Float)))) || (decide (unit > (255 : Float)))) then (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_result.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_return (0.0 / 0.0 : Float)) else (let index_2 := (index + (1 : Float));
      (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26 bytes index_2)))) else source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_result.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_done)

def convertJavaScriptUtf8StringNumberUnits (bytes : (Array Float)) : Float :=
  (let index := (0 : Float);
    (let source_ml_source2_s23 := (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26 bytes index);
      (match source_ml_source2_s23 with
        | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_result.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_done => (let units := (#[] : (Array Float));
            (let index_2 := (0 : Float);
              (let source_ml_source2_s24 := (source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27 bytes units index_2);
                (match source_ml_source2_s24 with
                  | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_result.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_done units_2 => (convertJavaScriptStringNumberUnits units_2)
                  | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_result.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop27_return source_ml_source1_temporary_result27 => source_ml_source1_temporary_result27))))
        | source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_result.source_ml_source1_temporary_convertJavaScriptUtf8StringNumberUnits_loop26_return source_ml_source1_temporary_result29 => source_ml_source1_temporary_result29)))

def main : IO Unit := do
  pure ()

def convertUnits := convertJavaScriptStringNumberUnits
def convertInteger := convertJavaScriptBigIntegerNumber
def convertUtf8 := convertJavaScriptUtf8StringNumberUnits
end MLStringNumber
