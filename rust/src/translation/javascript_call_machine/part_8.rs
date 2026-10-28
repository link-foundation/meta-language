// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop5Result;
use super::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop7Result;
use super::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3;
use super::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result;
use super::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop6Result;
use super::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::source_ml_source1_temporary_find_java_script_ordinary_object_loop5;
use super::source_ml_source1_temporary_find_java_script_ordinary_property_loop7;
use super::source_ml_source1_temporary_read_java_script_semantic_value_error_loop1;
use super::source_ml_source1_temporary_test_java_script_ordinary_symbol_loop6;
use super::source_ml_source1_temporary_test_java_script_semantic_same_value_loop2;

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
pub fn read_java_script_semantic_value_error(value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Text(units) => {
            if (((units.len() as f64)) > (1048576f64)) {
                self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116])
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s5 = self::source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(&(units), at);
                        match source_ml_source2_s5.clone() {
                            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(source_ml_source1_temporary_result1) => {
                                source_ml_source1_temporary_result1.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::Symbol(identity) => {
            if ((identity > (self::ml::Big::from_i128(0))) && (identity <= (self::ml::Big::from_i128(18446744073709551615)))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![97, 32, 115, 121, 109, 98, 111, 108, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121])
            }
        }
        self::JavaScriptSemanticValue::Object(identity_2, _) => {
            if ((identity_2 > (self::ml::Big::from_i128(0))) && (identity_2 <= (self::ml::Big::from_i128(18446744073709551615)))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![97, 110, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121])
            }
        }
        _ => {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_same_value(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    if (((self::read_java_script_semantic_value_error(left.clone())) != (self::ml_string::Text::from_units(vec![]))) || ((self::read_java_script_semantic_value_error(right.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![83, 97, 109, 101, 86, 97, 108, 117, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 119, 111, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97, 108, 117, 101, 115, 32, 105, 110, 32, 116, 104, 101, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 109, 111, 100, 101, 108])).0, message_present: true }).raise()
    } else {
        match left.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Null => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Boolean(left_value) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Boolean(value) => {
                        (left_value == value)
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Number(left_value_2) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Number(value_2) => {
                        if (left_value_2 == value_2) {
                            ((left_value_2 != (0f64)) || (((1f64 / left_value_2)) == ((1f64 / value_2))))
                        } else {
                            (left_value_2.is_nan() && value_2.is_nan())
                        }
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(left_value_3) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Integer(value_3) => {
                        (left_value_3 == value_3)
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Text(left_units) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Text(units) => {
                        if (((left_units.len() as f64)) != ((units.len() as f64))) {
                            false
                        } else {
                            {
                                let at = 0f64;
                                {
                                    let source_ml_source2_s45 = self::source_ml_source1_temporary_test_java_script_semantic_same_value_loop2(&(left_units), &(units), at);
                                    match source_ml_source2_s45.clone() {
                                        self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Done => {
                                            true
                                        }
                                        self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Return(source_ml_source1_temporary_result2) => {
                                            source_ml_source1_temporary_result2
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(left_identity) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Symbol(identity) => {
                        (left_identity == identity)
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Object(left_identity_2, _) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Object(identity_2, _) => {
                        (left_identity_2 == identity_2)
                    }
                    _ => {
                        false
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_accessor(value: self::JavaScriptSemanticValue) -> bool {
    if ((self::read_java_script_semantic_value_error(value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
        false
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                true
            }
            self::JavaScriptSemanticValue::Object(_, source_ml_source2_f64) => {
                source_ml_source2_f64
            }
            _ => {
                false
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_descriptor_kind(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> self::ml_string::Text {
    {
        let data = ((descriptor.clone()).9 || (descriptor.clone()).11);
        {
            let accessor = ((descriptor.clone()).5 || (descriptor.clone()).7);
            if (data && accessor) {
                self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100])
            } else {
                if data {
                    self::ml_string::Text::from_units(vec![100, 97, 116, 97])
                } else {
                    if accessor {
                        self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])
                    } else {
                        self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99])
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_descriptor_error(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> self::ml_string::Text {
    if ((self::read_java_script_property_descriptor_kind(descriptor.clone())) == (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]))) {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
    } else {
        if ((descriptor.clone()).9 && ((self::read_java_script_semantic_value_error((descriptor.clone()).8)) != (self::ml_string::Text::from_units(vec![])))) {
            self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 83, 101, 109, 97, 110, 116, 105, 99, 86, 97, 108, 117, 101])
        } else {
            if ((descriptor.clone()).5 && (((self::read_java_script_semantic_value_error((descriptor.clone()).4)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).4)))) {
                self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
            } else {
                if ((descriptor.clone()).7 && (((self::read_java_script_semantic_value_error((descriptor.clone()).6)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).6)))) {
                    self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_complete_java_script_property_descriptor(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> bool {
    if ((!((descriptor.clone()).1) || !((descriptor.clone()).3)) || ((self::read_java_script_property_descriptor_error(descriptor.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        false
    } else {
        {
            let kind = self::read_java_script_property_descriptor_kind(descriptor.clone());
            if (kind == (self::ml_string::Text::from_units(vec![100, 97, 116, 97]))) {
                ((descriptor.clone()).9 && (descriptor.clone()).11)
            } else {
                (((kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) && (descriptor.clone()).5) && (descriptor.clone()).7)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_property_descriptor(kind: &self::ml_string::Text, configurable: bool, enumerable: bool) -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source2_field111 = ((*kind) != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
        {
            let source_ml_source2_field112 = self::JavaScriptSemanticValue::Undefined;
            {
                let source_ml_source2_field113 = ((*kind) != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                {
                    let source_ml_source2_field115 = ((*kind) == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                    {
                        let source_ml_source2_field116 = self::JavaScriptSemanticValue::Undefined;
                        {
                            let source_ml_source2_field117 = ((*kind) == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                            {
                                let source_ml_source2_field118 = self::JavaScriptSemanticValue::Undefined;
                                (configurable, true, enumerable, true, source_ml_source2_field116.clone(), source_ml_source2_field115, source_ml_source2_field118.clone(), source_ml_source2_field117, source_ml_source2_field112.clone(), source_ml_source2_field111, false, source_ml_source2_field113)
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_property_descriptor_fields(base: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), proposed: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source2_field120 = if (proposed.clone()).1 {
            (proposed.clone()).0
        } else {
            (base.clone()).0
        };
        {
            let source_ml_source2_field122 = if (proposed.clone()).3 {
                (proposed.clone()).2
            } else {
                (base.clone()).2
            };
            {
                let source_ml_source2_field123 = (base.clone()).9;
                {
                    let source_ml_source2_field124 = if ((base.clone()).9 && (proposed.clone()).9) {
                        (proposed.clone()).8
                    } else {
                        (base.clone()).8
                    };
                    {
                        let source_ml_source2_field125 = (base.clone()).11;
                        {
                            let source_ml_source2_field126 = if ((base.clone()).11 && (proposed.clone()).11) {
                                (proposed.clone()).10
                            } else {
                                (base.clone()).10
                            };
                            {
                                let source_ml_source2_field127 = (base.clone()).5;
                                {
                                    let source_ml_source2_field128 = if ((base.clone()).5 && (proposed.clone()).5) {
                                        (proposed.clone()).4
                                    } else {
                                        (base.clone()).4
                                    };
                                    {
                                        let source_ml_source2_field129 = (base.clone()).7;
                                        {
                                            let source_ml_source2_field130 = if ((base.clone()).7 && (proposed.clone()).7) {
                                                (proposed.clone()).6
                                            } else {
                                                (base.clone()).6
                                            };
                                            (source_ml_source2_field120, true, source_ml_source2_field122, true, source_ml_source2_field128.clone(), source_ml_source2_field127, source_ml_source2_field130.clone(), source_ml_source2_field129, source_ml_source2_field124.clone(), source_ml_source2_field123, source_ml_source2_field126, source_ml_source2_field125)
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_descriptor_writes(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> Vec<self::ml_string::Text> {
    {
        let writes = Vec::<self::ml_string::Text>::new();
        {
            let writes_3 = if (descriptor.clone()).1 {
                {
                    let writes_2 = {
                        let source_ml_source2_concat_value132 = vec![self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101])];
                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), writes), source_ml_source2_concat_value132)
                    };
                    writes_2
                }
            } else {
                writes
            };
            {
                let writes_5 = if (descriptor.clone()).3 {
                    {
                        let writes_4 = {
                            let source_ml_source2_concat_value134 = vec![self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101])];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), writes_3), source_ml_source2_concat_value134)
                        };
                        writes_4
                    }
                } else {
                    writes_3
                };
                {
                    let writes_7 = if (descriptor.clone()).9 {
                        {
                            let writes_6 = {
                                let source_ml_source2_concat_value136 = vec![self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])];
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), writes_5), source_ml_source2_concat_value136)
                            };
                            writes_6
                        }
                    } else {
                        writes_5
                    };
                    {
                        let writes_9 = if (descriptor.clone()).11 {
                            {
                                let writes_8 = {
                                    let source_ml_source2_concat_value138 = vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])];
                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), writes_7), source_ml_source2_concat_value138)
                                };
                                writes_8
                            }
                        } else {
                            writes_7
                        };
                        {
                            let writes_11 = if (descriptor.clone()).5 {
                                {
                                    let writes_10 = {
                                        let source_ml_source2_concat_value140 = vec![self::ml_string::Text::from_units(vec![103, 101, 116])];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), writes_9), source_ml_source2_concat_value140)
                                    };
                                    writes_10
                                }
                            } else {
                                writes_9
                            };
                            {
                                let writes_13 = if (descriptor.clone()).7 {
                                    {
                                        let writes_12 = {
                                            let source_ml_source2_concat_value142 = vec![self::ml_string::Text::from_units(vec![115, 101, 116])];
                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), writes_11), source_ml_source2_concat_value142)
                                        };
                                        writes_12
                                    }
                                } else {
                                    writes_11
                                };
                                writes_13
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_property_transition(disposition: &self::ml_string::Text, error: &self::ml_string::Text, descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), writes: Vec<self::ml_string::Text>) -> ((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), self::ml_string::Text, self::ml_string::Text, Vec<self::ml_string::Text>) {
    (descriptor.clone(), disposition.clone(), error.clone(), writes)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_property_definition(exists: bool, extensible: bool, current: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), proposed: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> ((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), self::ml_string::Text, self::ml_string::Text, Vec<self::ml_string::Text>) {
    {
        let error = self::read_java_script_property_descriptor_error(proposed.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(error), current.clone(), Vec::<self::ml_string::Text>::new())
        } else {
            if (exists && !(self::test_complete_java_script_property_descriptor(current.clone()))) {
                self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 67, 117, 114, 114, 101, 110, 116, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114])), current.clone(), Vec::<self::ml_string::Text>::new())
            } else {
                {
                    let kind = self::read_java_script_property_descriptor_kind(proposed.clone());
                    if !exists {
                        if !extensible {
                            self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new())
                        } else {
                            {
                                let base = self::create_java_script_property_descriptor(&(kind), false, false);
                                {
                                    let descriptor = self::apply_java_script_property_descriptor_fields(base.clone(), proposed.clone());
                                    self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![99, 114, 101, 97, 116, 101])), &(self::ml_string::Text::from_units(vec![])), descriptor.clone(), self::read_java_script_property_descriptor_writes(descriptor.clone()))
                                }
                            }
                        }
                    } else {
                        if (((((!((proposed.clone()).1) && !((proposed.clone()).3)) && !((proposed.clone()).9)) && !((proposed.clone()).11)) && !((proposed.clone()).5)) && !((proposed.clone()).7)) {
                            self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![107, 101, 101, 112])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new())
                        } else {
                            {
                                let original = self::read_java_script_property_descriptor_kind(current.clone());
                                {
                                    let source_ml_source2_s147 = if !((current.clone()).0) {
                                        if ((proposed.clone()).1 && (proposed.clone()).0) {
                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                        } else {
                                            if ((proposed.clone()).3 && (((proposed.clone()).2) != ((current.clone()).2))) {
                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                            } else {
                                                if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                } else {
                                                    if (original == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) {
                                                        if ((proposed.clone()).5 && !(self::test_java_script_semantic_same_value((proposed.clone()).4, (current.clone()).4))) {
                                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                        } else {
                                                            if ((proposed.clone()).7 && !(self::test_java_script_semantic_same_value((proposed.clone()).6, (current.clone()).6))) {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                            } else {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3Next
                                                            }
                                                        }
                                                    } else {
                                                        if !((current.clone()).10) {
                                                            if ((proposed.clone()).11 && (proposed.clone()).10) {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                            } else {
                                                                if (proposed.clone()).9 {
                                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3Return(self::create_java_script_property_transition(&(if self::test_java_script_semantic_same_value((proposed.clone()).8, (current.clone()).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                                                } else {
                                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3Next
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3Next
                                    };
                                    match source_ml_source2_s147.clone() {
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3Next => {
                                            {
                                                let base_2 = if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::create_java_script_property_descriptor(&(kind), (current.clone()).0, (current.clone()).2)
                                                } else {
                                                    current.clone()
                                                };
                                                {
                                                    let descriptor_2 = self::apply_java_script_property_descriptor_fields(base_2.clone(), proposed.clone());
                                                    {
                                                        let conversion = ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original));
                                                        self::create_java_script_property_transition(&(if conversion {
                                                            self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116])
                                                        } else {
                                                            self::ml_string::Text::from_units(vec![117, 112, 100, 97, 116, 101])
                                                        }), &(self::ml_string::Text::from_units(vec![])), descriptor_2.clone(), self::read_java_script_property_descriptor_writes(if conversion {
                                                            descriptor_2.clone()
                                                        } else {
                                                            proposed.clone()
                                                        }))
                                                    }
                                                }
                                            }
                                        }
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin3Return(source_ml_source1_temporary_result3) => {
                                            source_ml_source1_temporary_result3.clone()
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_ordinary_object(objects: Vec<self::JavaScriptOrdinaryObject>, identity: self::ml::Big) -> f64 {
    {
        let low = 0f64;
        {
            let high = (objects.len() as f64);
            {
                let source_ml_source2_s151 = self::source_ml_source1_temporary_find_java_script_ordinary_object_loop5(objects, identity.clone(), low, high);
                match source_ml_source2_s151.clone() {
                    self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop5Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop5Return(source_ml_source1_temporary_result5) => {
                        source_ml_source1_temporary_result5
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_ordinary_symbol(symbols: Vec<self::ml::Big>, identity: self::ml::Big) -> bool {
    {
        let low = 0f64;
        {
            let high = (symbols.len() as f64);
            {
                let source_ml_source2_s152 = self::source_ml_source1_temporary_test_java_script_ordinary_symbol_loop6(symbols, identity.clone(), low, high);
                match source_ml_source2_s152.clone() {
                    self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop6Done => {
                        false
                    }
                    self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop6Return(source_ml_source1_temporary_result6) => {
                        source_ml_source1_temporary_result6
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_value_error(heap: self::JavaScriptOrdinaryHeap, value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, shared_constructor_field2) => {
            {
                let error = self::read_java_script_semantic_value_error(value.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    match value.clone() {
                        self::JavaScriptSemanticValue::Symbol(identity) => {
                            if self::test_java_script_ordinary_symbol(shared_constructor_field2, identity.clone()) {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f157, source_ml_source2_f158) => {
                            {
                                let at = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), source_ml_source2_f157.clone());
                                if (at < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101, 99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                                } else {
                                    {
                                        let object = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(at));
                                        if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (({
                                            let source_ml_source2_nullish_field_receiver200 = (object.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver200.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source2_nullish_field_present201 = ((source_ml_source2_nullish_field_receiver200.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source2_nullish_field_present201.clone() {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source2_data_property_value202, _) => {
                                                            source_ml_source2_data_property_value202
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                            }
                                        }) == source_ml_source2_f158)) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121, 32, 109, 117, 115, 116, 32, 97, 103, 114, 101, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121])
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_key_error(heap: self::JavaScriptOrdinaryHeap, key: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    if (match key.clone() {
        self::JavaScriptSemanticValue::Text(_) => {
            false
        }
        _ => {
            true
        }
    } && match key.clone() {
        self::JavaScriptSemanticValue::Symbol(_) => {
            false
        }
        _ => {
            true
        }
    }) {
        self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 101, 120, 116, 32, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 119, 105, 116, 104, 111, 117, 116, 32, 105, 109, 112, 108, 105, 99, 105, 116, 32, 99, 111, 101, 114, 99, 105, 111, 110])
    } else {
        self::read_java_script_ordinary_value_error(heap.clone(), key.clone())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_ordinary_property(properties: Vec<self::JavaScriptOrdinaryProperty>, key: self::JavaScriptSemanticValue) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source2_s217 = self::source_ml_source1_temporary_find_java_script_ordinary_property_loop7(properties, key.clone(), at);
            match source_ml_source2_s217.clone() {
                self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop7Result::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop7Done => {
                    (-1f64)
                }
                self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop7Result::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop7Return(source_ml_source1_temporary_result7) => {
                    source_ml_source1_temporary_result7
                }
            }
        }
    }
}
