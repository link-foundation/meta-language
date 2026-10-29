// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryAcceptStringNumberInfinityLoop17Result;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Result;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result;
use super::MlSource1TemporaryConvertStringNumberRationalJoin9;
use super::MlSource1TemporaryConvertStringNumberRationalJoin10;
use super::MlSource1TemporaryConvertStringNumberRationalLoop11Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop12Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop15Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop16Result;
use super::MlSource1TemporaryReadStringNumberBinaryPowerLoop7Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::source_ml_source1_temporary_accept_string_number_infinity_loop17;
use super::source_ml_source1_temporary_convert_java_script_number_big_integer_loop36;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop30;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop31;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop32;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop35;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop18;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop19;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop20;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop23;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop25;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop26;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop28;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop29;
use super::source_ml_source1_temporary_convert_string_number_rational_loop11;
use super::source_ml_source1_temporary_convert_string_number_rational_loop12;
use super::source_ml_source1_temporary_convert_string_number_rational_loop13;
use super::source_ml_source1_temporary_convert_string_number_rational_loop14;
use super::source_ml_source1_temporary_convert_string_number_rational_loop15;
use super::source_ml_source1_temporary_convert_string_number_rational_loop16;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop7;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop8;
use super::source_ml_source1_temporary_read_string_number_decimal_power_loop5;
use super::source_ml_source1_temporary_read_string_number_exact_mantissa_loop6;

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallPrimitiveResult {
    PrimitiveValue(Box<self::JavaScriptSemanticValue>),
    PrimitiveInvalid(self::ml_string::Text),
    PrimitiveError(self::ml_string::Text, self::ml_string::Text),
    PrimitiveResourceLimit(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptScalarConversionTransition {
    ScalarContinuation(Box<self::JavaScriptPrimitiveConversionState>),
    ScalarCall(Box<self::JavaScriptPrimitiveConversionState>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>),
    ScalarCompleted(Box<self::JavaScriptSemanticValue>),
    ScalarThrow(Box<self::JavaScriptSemanticValue>),
    ScalarIntrinsicError(self::ml_string::Text, self::ml_string::Text),
    ScalarInvalid(self::ml_string::Text),
    ScalarResourceLimit(Box<self::JavaScriptSemanticValue>, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptTextCallState {
    TextProtocolState(Box<self::JavaScriptPrimitiveConversionState>),
    TextPrimitiveValue(Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallInstruction {
    Constant(f64, Box<self::JavaScriptSemanticValue>),
    Copy(f64, f64),
    Argument(f64, f64),
    Jump(f64),
    Branch(f64, f64, f64),
    Call(f64, f64, f64, Vec<f64>),
    Return(f64),
    Throw(f64),
    PushHandler(f64, f64),
    PopHandler,
    NumberBinary(f64, f64, f64, self::ml_string::Text),
    SameValue(f64, f64, f64),
    StrictEqual(f64, f64, f64),
    RequireWrite(f64),
    Primitive(f64, self::ml_string::Text, Vec<f64>),
    OrdinaryGet(f64, f64, f64, f64),
    OrdinarySet(f64, f64, f64, f64, f64),
    PropertyKey(f64, f64, self::ml::Big),
    PropertyDefinition(f64, f64, f64, f64, self::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallFunction {
    CallFunction(self::ml::Big, self::ml::Big, self::ml_string::Text, Box<self::JavaScriptSemanticValue>, f64, f64, Vec<self::JavaScriptSemanticValue>, Vec<self::JavaScriptCallInstruction>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallProgram {
    CallProgram(Vec<self::JavaScriptCallFunction>, Vec<self::ml::Big>, Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptDescriptorConversionState {
    DescriptorConversion(Box<self::JavaScriptSemanticValue>, f64, (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSemanticCompletion {
    NormalCompletion(Box<self::JavaScriptSemanticValue>),
    ThrowCompletion(Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptDescriptorConversionTransition {
    DescriptorContinuation(Box<self::JavaScriptDescriptorConversionState>),
    DescriptorCall(Box<self::JavaScriptDescriptorConversionState>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>),
    DescriptorCompleted((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)),
    DescriptorThrow(Box<self::JavaScriptSemanticValue>),
    DescriptorIntrinsicError(self::ml_string::Text, self::ml_string::Text),
    DescriptorInvalid(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPropertyKeyConversionTransition {
    PropertyKeyContinuation(Box<self::JavaScriptPrimitiveConversionState>),
    PropertyKeyCall(Box<self::JavaScriptPrimitiveConversionState>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>),
    PropertyKeyCompleted(Box<self::JavaScriptSemanticValue>),
    PropertyKeyThrow(Box<self::JavaScriptSemanticValue>),
    PropertyKeyIntrinsicError(self::ml_string::Text, self::ml_string::Text),
    PropertyKeyInvalid(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPropertyDefinitionState {
    PropertyDefinitionKey(Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptPrimitiveConversionState>),
    PropertyDefinitionDescriptor(Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptDescriptorConversionState>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPropertyDefinitionTransition {
    DefinitionContinuation(Box<self::JavaScriptPropertyDefinitionState>),
    DefinitionCall(Box<self::JavaScriptPropertyDefinitionState>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>),
    DefinitionCompleted(Box<self::JavaScriptOrdinaryHeap>, Box<self::JavaScriptSemanticValue>, Vec<self::ml_string::Text>),
    DefinitionThrow(Box<self::JavaScriptSemanticValue>),
    DefinitionIntrinsicError(self::ml_string::Text, self::ml_string::Text),
    DefinitionInvalid(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptNumberCallState {
    NumberProtocolState(Box<self::JavaScriptPrimitiveConversionState>),
    NumberPrimitiveValue(Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallHandler {
    CallHandler(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallPending {
    CallIdle,
    WaitCall(f64),
    WaitGet(f64),
    WaitSet(f64),
    WaitKey(f64, self::ml::Big, Box<self::JavaScriptPrimitiveConversionState>),
    WaitDefinition(f64, self::ml::Big, Box<self::JavaScriptPropertyDefinitionState>),
    WaitNumber(f64, self::ml::Big, Box<self::JavaScriptNumberCallState>),
    WaitText(f64, self::ml::Big, Box<self::JavaScriptTextCallState>),
    WaitSourceError(self::ml::Big, self::ml_string::Text, Box<self::JavaScriptSemanticValue>, bool),
    WaitError(self::ml::Big, self::ml_string::Text, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallFrame {
    CallFrame(self::ml::Big, f64, Vec<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>, Vec<self::JavaScriptCallHandler>, Box<self::JavaScriptCallPending>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallMachine {
    CallMachine(Box<self::JavaScriptOrdinaryHeap>, Vec<self::JavaScriptCallFrame>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallMachineTransition {
    MachineContinue(Box<self::JavaScriptCallMachine>),
    MachineCompleted(Box<self::JavaScriptOrdinaryHeap>, Box<self::JavaScriptSemanticValue>),
    MachineThrow(Box<self::JavaScriptOrdinaryHeap>, Box<self::JavaScriptSemanticValue>),
    MachineAllocateError(Box<self::JavaScriptCallMachine>, self::ml::Big, self::ml_string::Text, self::ml_string::Text),
    MachineAllocateSourceError(Box<self::JavaScriptCallMachine>, self::ml::Big, self::ml_string::Text, Box<self::JavaScriptSemanticValue>, bool),
    MachineInvalid(Box<self::JavaScriptOrdinaryHeap>, self::ml_string::Text),
    MachineResourceLimit(Box<self::JavaScriptCallMachine>, self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_whitespace_unit(unit: f64) -> bool {
    ((((((((((((unit >= (9f64)) && (unit <= (13f64))) || (unit == (32f64))) || (unit == (160f64))) || (unit == (5760f64))) || ((unit >= (8192f64)) && (unit <= (8202f64)))) || (unit == (8232f64))) || (unit == (8233f64))) || (unit == (8239f64))) || (unit == (8287f64))) || (unit == (12288f64))) || (unit == (65279f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_numeric_digit_unit(unit: f64) -> f64 {
    if ((unit >= (48f64)) && (unit <= (57f64))) {
        (unit - 48f64)
    } else {
        if ((unit >= (65f64)) && (unit <= (70f64))) {
            (unit - 55f64)
        } else {
            if ((unit >= (97f64)) && (unit <= (102f64))) {
                (unit - 87f64)
            } else {
                (-1f64)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_digit(unit: f64) -> f64 {
    self::read_java_script_numeric_digit_unit(unit)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_decimal_power(exponent: f64) -> self::ml::Big {
    {
        let value = self::ml::Big::from_i128(1);
        {
            let base = self::ml::Big::from_i128(10);
            {
                let value_2 = self::source_ml_source1_temporary_read_string_number_decimal_power_loop5(exponent, value.clone(), base.clone());
                value_2.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_exact_mantissa(value: self::ml::Big) -> f64 {
    {
        let result = 0f64;
        {
            let scale = 1f64;
            {
                let result_2 = self::source_ml_source1_temporary_read_string_number_exact_mantissa_loop6(value.clone(), result, scale);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_binary_power(exponent: f64) -> f64 {
    {
        let result = 1f64;
        {
            let source_ml_source2_s3 = self::source_ml_source1_temporary_read_string_number_binary_power_loop7(exponent, result);
            match source_ml_source2_s3.clone() {
                self::MlSource1TemporaryReadStringNumberBinaryPowerLoop7Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop7Done(exponent_2, result_2) => {
                    {
                        let result_3 = self::source_ml_source1_temporary_read_string_number_binary_power_loop8(exponent_2, result_2);
                        result_3
                    }
                }
            }
        }
    }
}

pub fn convert_string_number_rational(numerator: self::ml::Big, denominator: self::ml::Big) -> f64 {
    if ((numerator < (self::ml::Big::from_i128(0))) || (denominator <= (self::ml::Big::from_i128(0)))) {
        f64::NAN
    } else {
        if (numerator == (self::ml::Big::from_i128(0))) {
            0f64
        } else {
            {
                let exponent = 0f64;
                {
                    let source_ml_source2_s6 = if (numerator >= denominator) {
                        {
                            let source_ml_source2_s7 = self::source_ml_source1_temporary_convert_string_number_rational_loop15(numerator.clone(), exponent, denominator.clone());
                            match source_ml_source2_s7.clone() {
                                self::MlSource1TemporaryConvertStringNumberRationalLoop15Result::MlSource1TemporaryConvertStringNumberRationalLoop15Done(exponent_2) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Next(exponent_2)
                                }
                                self::MlSource1TemporaryConvertStringNumberRationalLoop15Result::MlSource1TemporaryConvertStringNumberRationalLoop15Return(source_ml_source1_temporary_result15) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Return(source_ml_source1_temporary_result15)
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_source2_s8 = self::source_ml_source1_temporary_convert_string_number_rational_loop16(denominator.clone(), exponent, numerator.clone());
                            match source_ml_source2_s8.clone() {
                                self::MlSource1TemporaryConvertStringNumberRationalLoop16Result::MlSource1TemporaryConvertStringNumberRationalLoop16Done(exponent_3) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Next(exponent_3)
                                }
                                self::MlSource1TemporaryConvertStringNumberRationalLoop16Result::MlSource1TemporaryConvertStringNumberRationalLoop16Return(source_ml_source1_temporary_result16) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Return(source_ml_source1_temporary_result16)
                                }
                            }
                        }
                    };
                    match source_ml_source2_s6.clone() {
                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Next(exponent_4) => {
                            {
                                let shift = if (exponent_4 >= ((-1022f64))) {
                                    (52f64 - exponent_4)
                                } else {
                                    1074f64
                                };
                                {
                                    let source_ml_source2_s9 = if (shift >= (0f64)) {
                                        {
                                            let index = 0f64;
                                            {
                                                let numerator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop13(numerator.clone(), shift, index);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin10::MlSource1TemporaryConvertStringNumberRationalJoin10Next(numerator_2.clone(), denominator.clone())
                                            }
                                        }
                                    } else {
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let denominator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop14(denominator.clone(), shift, index_2);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin10::MlSource1TemporaryConvertStringNumberRationalJoin10Next(numerator.clone(), denominator_2.clone())
                                            }
                                        }
                                    };
                                    match source_ml_source2_s9.clone() {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin10::MlSource1TemporaryConvertStringNumberRationalJoin10Next(numerator_3, denominator_3) => {
                                            {
                                                let quotient = self::ml::Big::from_i128(0);
                                                {
                                                    let remainder = numerator_3.clone();
                                                    {
                                                        let scale = denominator_3.clone();
                                                        {
                                                            let weight = self::ml::Big::from_i128(1);
                                                            {
                                                                let source_ml_source2_s10 = self::source_ml_source1_temporary_convert_string_number_rational_loop11(remainder.clone(), scale.clone(), weight.clone());
                                                                match source_ml_source2_s10.clone() {
                                                                    self::MlSource1TemporaryConvertStringNumberRationalLoop11Result::MlSource1TemporaryConvertStringNumberRationalLoop11Done(scale_2, weight_2) => {
                                                                        {
                                                                            let source_ml_source2_s11 = self::source_ml_source1_temporary_convert_string_number_rational_loop12(quotient.clone(), remainder.clone(), scale_2.clone(), weight_2.clone());
                                                                            match source_ml_source2_s11.clone() {
                                                                                self::MlSource1TemporaryConvertStringNumberRationalLoop12Result::MlSource1TemporaryConvertStringNumberRationalLoop12Done(quotient_2, remainder_2) => {
                                                                                    {
                                                                                        let twice = remainder_2.mul(&self::ml::Big::from_i128(2));
                                                                                        {
                                                                                            let quotient_4 = if ((twice > denominator_3) || ((twice == denominator_3) && ((self::ml::divide(&quotient_2, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))))) {
                                                                                                {
                                                                                                    let quotient_3 = quotient_2.add(&self::ml::Big::from_i128(1));
                                                                                                    quotient_3.clone()
                                                                                                }
                                                                                            } else {
                                                                                                quotient_2.clone()
                                                                                            };
                                                                                            (self::read_string_number_exact_mantissa(quotient_4.clone()) * self::read_string_number_binary_power((-shift)))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Return(source_ml_source1_temporary_result14) => {
                            source_ml_source1_temporary_result14
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_string_number_infinity(units: Vec<f64>, start: f64, end: f64) -> bool {
    if (((end - start)) != (8f64)) {
        false
    } else {
        {
            let expected = vec![73f64, 110f64, 102f64, 105f64, 110f64, 105f64, 116f64, 121f64];
            {
                let index = 0f64;
                {
                    let source_ml_source2_s12 = self::source_ml_source1_temporary_accept_string_number_infinity_loop17(units, start, expected, index);
                    match source_ml_source2_s12.clone() {
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop17Result::MlSource1TemporaryAcceptStringNumberInfinityLoop17Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop17Result::MlSource1TemporaryAcceptStringNumberInfinityLoop17Return(source_ml_source1_temporary_result17) => {
                            source_ml_source1_temporary_result17
                        }
                    }
                }
            }
        }
    }
}

pub fn convert_java_script_string_number_units(units: Vec<f64>) -> f64 {
    {
        let index = 0f64;
        {
            let source_ml_source2_s13 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop18(units.clone(), index);
            match source_ml_source2_s13.clone() {
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Done => {
                    {
                        let start = 0f64;
                        {
                            let end = (units.len() as f64);
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop19(units.clone(), start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop20(units.clone(), start_2, end);
                                    if (start_2 == end_2) {
                                        0f64
                                    } else {
                                        {
                                            let negative = false;
                                            {
                                                let signed = false;
                                                {
                                                    let sign = (self::ml_array::optional_at(&units, self::ml_array::number_index(start_2))).clone().unwrap_or_else(|| (-1f64));
                                                    {
                                                        let source_ml_source2_s14 = if ((sign == (43f64)) || (sign == (45f64))) {
                                                            {
                                                                let negative_2 = (sign == (45f64));
                                                                {
                                                                    let start_3 = (start_2 + 1f64);
                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21Next(start_3, negative_2, true)
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s14.clone() {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21Next(start_4, negative_3, signed_2) => {
                                                                if (start_4 == end_2) {
                                                                    f64::NAN
                                                                } else {
                                                                    if self::accept_string_number_infinity(units.clone(), start_4, end_2) {
                                                                        if negative_3 {
                                                                            f64::NEG_INFINITY
                                                                        } else {
                                                                            f64::INFINITY
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let integer_digits = vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(1), self::ml::Big::from_i128(2), self::ml::Big::from_i128(3), self::ml::Big::from_i128(4), self::ml::Big::from_i128(5), self::ml::Big::from_i128(6), self::ml::Big::from_i128(7), self::ml::Big::from_i128(8), self::ml::Big::from_i128(9), self::ml::Big::from_i128(10), self::ml::Big::from_i128(11), self::ml::Big::from_i128(12), self::ml::Big::from_i128(13), self::ml::Big::from_i128(14), self::ml::Big::from_i128(15)];
                                                                            {
                                                                                let source_ml_source2_s16 = if ((!signed_2 && (((self::ml_array::optional_at(&units, self::ml_array::number_index(start_4))).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((end_2 - start_4)) >= (2f64))) {
                                                                                    {
                                                                                        let marker = (self::ml_array::optional_at(&units, self::ml_array::number_index((start_4 + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                                                                                        {
                                                                                            let radix = if ((marker == (120f64)) || (marker == (88f64))) {
                                                                                                16f64
                                                                                            } else {
                                                                                                if ((marker == (98f64)) || (marker == (66f64))) {
                                                                                                    2f64
                                                                                                } else {
                                                                                                    if ((marker == (111f64)) || (marker == (79f64))) {
                                                                                                        8f64
                                                                                                    } else {
                                                                                                        0f64
                                                                                                    }
                                                                                                }
                                                                                            };
                                                                                            if (radix != (0f64)) {
                                                                                                {
                                                                                                    let value = self::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let overflow = false;
                                                                                                        {
                                                                                                            let limit = self::read_string_number_decimal_power(310f64);
                                                                                                            {
                                                                                                                let base = if (radix == (16f64)) {
                                                                                                                    self::ml::Big::from_i128(16)
                                                                                                                } else {
                                                                                                                    if (radix == (8f64)) {
                                                                                                                        self::ml::Big::from_i128(8)
                                                                                                                    } else {
                                                                                                                        self::ml::Big::from_i128(2)
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let start_5 = (start_4 + 2f64);
                                                                                                                    if (start_5 == end_2) {
                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Return(f64::NAN)
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let source_ml_source2_s17 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop29(units.clone(), end_2, integer_digits.clone(), radix, value.clone(), overflow, limit.clone(), base.clone(), start_5);
                                                                                                                            match source_ml_source2_s17.clone() {
                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Done(value_2, overflow_2) => {
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Return(if overflow_2 {
                                                                                                                                        f64::INFINITY
                                                                                                                                    } else {
                                                                                                                                        self::convert_string_number_rational(value_2.clone(), self::ml::Big::from_i128(1))
                                                                                                                                    })
                                                                                                                                }
                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Return(source_ml_source1_temporary_result29) => {
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Return(source_ml_source1_temporary_result29)
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Next(start_4)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Next(start_4)
                                                                                };
                                                                                match source_ml_source2_s16.clone() {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Next(start_6) => {
                                                                                        {
                                                                                            let at = start_6;
                                                                                            {
                                                                                                let fraction = 0f64;
                                                                                                {
                                                                                                    let digits = 0f64;
                                                                                                    {
                                                                                                        let significant = 0f64;
                                                                                                        {
                                                                                                            let source_ml_source2_s18 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop23(units.clone(), end_2, at, fraction, digits, significant, false);
                                                                                                            match source_ml_source2_s18.clone() {
                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Done(at_2, fraction_2, digits_2, significant_2) => {
                                                                                                                    if (digits_2 == (0f64)) {
                                                                                                                        f64::NAN
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let decimal_end = at_2;
                                                                                                                            {
                                                                                                                                let exponent = 0f64;
                                                                                                                                {
                                                                                                                                    let exponent_limit = ((units.len() as f64) + 10000f64);
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_s21 = if ((at_2 < end_2) && ((((self::ml_array::optional_at(&units, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| (-1f64))) == (101f64)) || (((self::ml_array::optional_at(&units, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| (-1f64))) == (69f64)))) {
                                                                                                                                            {
                                                                                                                                                let at_3 = (at_2 + 1f64);
                                                                                                                                                {
                                                                                                                                                    let exponent_negative = false;
                                                                                                                                                    {
                                                                                                                                                        let exponent_sign = (self::ml_array::optional_at(&units, self::ml_array::number_index(at_3))).clone().unwrap_or_else(|| (-1f64));
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_s22 = if ((exponent_sign == (43f64)) || (exponent_sign == (45f64))) {
                                                                                                                                                                {
                                                                                                                                                                    let exponent_negative_2 = (exponent_sign == (45f64));
                                                                                                                                                                    {
                                                                                                                                                                        let at_4 = (at_3 + 1f64);
                                                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_4, exponent_negative_2)
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            } else {
                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_3, exponent_negative)
                                                                                                                                                            };
                                                                                                                                                            match source_ml_source2_s22.clone() {
                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_5, exponent_negative_3) => {
                                                                                                                                                                    {
                                                                                                                                                                        let first = at_5;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_s23 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop28(units.clone(), end_2, at_5, exponent, exponent_limit);
                                                                                                                                                                            match source_ml_source2_s23.clone() {
                                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Done(at_6, exponent_2) => {
                                                                                                                                                                                    if (at_6 == first) {
                                                                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Return(f64::NAN)
                                                                                                                                                                                    } else {
                                                                                                                                                                                        if exponent_negative_3 {
                                                                                                                                                                                            {
                                                                                                                                                                                                let exponent_3 = (-exponent_2);
                                                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(at_6, exponent_3)
                                                                                                                                                                                            }
                                                                                                                                                                                        } else {
                                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(at_6, exponent_2)
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(at_2, exponent)
                                                                                                                                        };
                                                                                                                                        match source_ml_source2_s21.clone() {
                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(at_7, exponent_4) => {
                                                                                                                                                if (at_7 != end_2) {
                                                                                                                                                    f64::NAN
                                                                                                                                                } else {
                                                                                                                                                    if (significant_2 == (0f64)) {
                                                                                                                                                        if negative_3 {
                                                                                                                                                            (-0f64)
                                                                                                                                                        } else {
                                                                                                                                                            0f64
                                                                                                                                                        }
                                                                                                                                                    } else {
                                                                                                                                                        {
                                                                                                                                                            let exponent_5 = (exponent_4 - fraction_2);
                                                                                                                                                            {
                                                                                                                                                                let order = (significant_2 + exponent_5);
                                                                                                                                                                if (order > (310f64)) {
                                                                                                                                                                    if negative_3 {
                                                                                                                                                                        f64::NEG_INFINITY
                                                                                                                                                                    } else {
                                                                                                                                                                        f64::INFINITY
                                                                                                                                                                    }
                                                                                                                                                                } else {
                                                                                                                                                                    if (order < ((-325f64))) {
                                                                                                                                                                        if negative_3 {
                                                                                                                                                                            (-0f64)
                                                                                                                                                                        } else {
                                                                                                                                                                            0f64
                                                                                                                                                                        }
                                                                                                                                                                    } else {
                                                                                                                                                                        {
                                                                                                                                                                            let mantissa_end = decimal_end;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_s24 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop25(units.clone(), start_6, exponent_5, mantissa_end);
                                                                                                                                                                                match source_ml_source2_s24.clone() {
                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Done(exponent_6, mantissa_end_2) => {
                                                                                                                                                                                        {
                                                                                                                                                                                            let mantissa = self::ml::Big::from_i128(0);
                                                                                                                                                                                            {
                                                                                                                                                                                                let mantissa_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop26(units, integer_digits, mantissa_end_2, mantissa.clone(), start_6);
                                                                                                                                                                                                {
                                                                                                                                                                                                    let numerator = if (exponent_6 >= (0f64)) {
                                                                                                                                                                                                        mantissa_2.mul(&self::read_string_number_decimal_power(exponent_6))
                                                                                                                                                                                                    } else {
                                                                                                                                                                                                        mantissa_2.clone()
                                                                                                                                                                                                    };
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let denominator = if (exponent_6 < (0f64)) {
                                                                                                                                                                                                            self::read_string_number_decimal_power((-exponent_6))
                                                                                                                                                                                                        } else {
                                                                                                                                                                                                            self::ml::Big::from_i128(1)
                                                                                                                                                                                                        };
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let value_3 = self::convert_string_number_rational(numerator.clone(), denominator.clone());
                                                                                                                                                                                                            if negative_3 {
                                                                                                                                                                                                                (-value_3)
                                                                                                                                                                                                            } else {
                                                                                                                                                                                                                value_3
                                                                                                                                                                                                            }
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Return(source_ml_source1_temporary_result26) => {
                                                                                                                                                source_ml_source1_temporary_result26
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Return(source_ml_source1_temporary_result28) => {
                                                                                        source_ml_source1_temporary_result28
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Return(source_ml_source1_temporary_result29_2) => {
                    source_ml_source1_temporary_result29_2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_big_integer_number(value: self::ml::Big) -> f64 {
    if (value < (self::ml::Big::from_i128(0))) {
        (-(self::convert_string_number_rational(value.neg(), self::ml::Big::from_i128(1))))
    } else {
        self::convert_string_number_rational(value.clone(), self::ml::Big::from_i128(1))
    }
}

pub fn convert_java_script_string_big_integer_units(units: Vec<f64>) -> Vec<self::ml::Big> {
    {
        let index = 0f64;
        {
            let source_ml_source2_s25 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop30(units.clone(), index);
            match source_ml_source2_s25.clone() {
                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Done => {
                    {
                        let start = 0f64;
                        {
                            let end = (units.len() as f64);
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop31(units.clone(), start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop32(units.clone(), start_2, end);
                                    if (start_2 == end_2) {
                                        vec![self::ml::Big::from_i128(0)]
                                    } else {
                                        {
                                            let negative = false;
                                            {
                                                let signed = false;
                                                {
                                                    let sign = (self::ml_array::optional_at(&units, self::ml_array::number_index(start_2))).clone().unwrap_or_else(|| (-1f64));
                                                    {
                                                        let source_ml_source2_s26 = if ((sign == (43f64)) || (sign == (45f64))) {
                                                            {
                                                                let negative_2 = (sign == (45f64));
                                                                {
                                                                    let start_3 = (start_2 + 1f64);
                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33Next(start_3, negative_2, true)
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s26.clone() {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33Next(start_4, negative_3, signed_2) => {
                                                                if (start_4 == end_2) {
                                                                    Vec::<self::ml::Big>::new()
                                                                } else {
                                                                    {
                                                                        let radix = 10f64;
                                                                        {
                                                                            let base = self::ml::Big::from_i128(10);
                                                                            {
                                                                                let source_ml_source2_s27 = if ((!signed_2 && (((self::ml_array::optional_at(&units, self::ml_array::number_index(start_4))).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((end_2 - start_4)) >= (2f64))) {
                                                                                    {
                                                                                        let marker = (self::ml_array::optional_at(&units, self::ml_array::number_index((start_4 + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                                                                                        {
                                                                                            let selected = if ((marker == (120f64)) || (marker == (88f64))) {
                                                                                                16f64
                                                                                            } else {
                                                                                                if ((marker == (98f64)) || (marker == (66f64))) {
                                                                                                    2f64
                                                                                                } else {
                                                                                                    if ((marker == (111f64)) || (marker == (79f64))) {
                                                                                                        8f64
                                                                                                    } else {
                                                                                                        0f64
                                                                                                    }
                                                                                                }
                                                                                            };
                                                                                            if (selected != (0f64)) {
                                                                                                {
                                                                                                    let base_2 = if (selected == (16f64)) {
                                                                                                        self::ml::Big::from_i128(16)
                                                                                                    } else {
                                                                                                        if (selected == (8f64)) {
                                                                                                            self::ml::Big::from_i128(8)
                                                                                                        } else {
                                                                                                            self::ml::Big::from_i128(2)
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let start_5 = (start_4 + 2f64);
                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34Next(start_5, selected, base_2.clone())
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34Next(start_4, radix, base.clone())
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34Next(start_4, radix, base.clone())
                                                                                };
                                                                                match source_ml_source2_s27.clone() {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34Next(start_6, radix_2, base_3) => {
                                                                                        if (start_6 == end_2) {
                                                                                            Vec::<self::ml::Big>::new()
                                                                                        } else {
                                                                                            {
                                                                                                let digits = vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(1), self::ml::Big::from_i128(2), self::ml::Big::from_i128(3), self::ml::Big::from_i128(4), self::ml::Big::from_i128(5), self::ml::Big::from_i128(6), self::ml::Big::from_i128(7), self::ml::Big::from_i128(8), self::ml::Big::from_i128(9), self::ml::Big::from_i128(10), self::ml::Big::from_i128(11), self::ml::Big::from_i128(12), self::ml::Big::from_i128(13), self::ml::Big::from_i128(14), self::ml::Big::from_i128(15)];
                                                                                                {
                                                                                                    let value = self::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let source_ml_source2_s28 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop35(units, end_2, radix_2, base_3.clone(), digits, value.clone(), start_6);
                                                                                                        match source_ml_source2_s28.clone() {
                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Done(value_2) => {
                                                                                                                vec![if negative_3 {
                                                                                                                    value_2.neg()
                                                                                                                } else {
                                                                                                                    value_2.clone()
                                                                                                                }]
                                                                                                            }
                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Return(source_ml_source1_temporary_result35) => {
                                                                                                                source_ml_source1_temporary_result35
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Return(source_ml_source1_temporary_result35_2) => {
                    source_ml_source1_temporary_result35_2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_number_big_integer(value: f64) -> Vec<self::ml::Big> {
    if !(self::ml_math::is_integer(value)) {
        Vec::<self::ml::Big>::new()
    } else {
        {
            let negative = (value < (0f64));
            {
                let value_2 = value.abs();
                {
                    let integer = self::ml::Big::from_i128(0);
                    {
                        let weight = self::ml::Big::from_i128(1);
                        {
                            let integer_2 = self::source_ml_source1_temporary_convert_java_script_number_big_integer_loop36(value_2, integer.clone(), weight.clone());
                            vec![if negative {
                                integer_2.neg()
                            } else {
                                integer_2.clone()
                            }]
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_boolean_big_integer(value: bool) -> self::ml::Big {
    if value {
        self::ml::Big::from_i128(1)
    } else {
        self::ml::Big::from_i128(0)
    }
}
