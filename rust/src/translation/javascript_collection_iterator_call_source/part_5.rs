// Generated from complete js/src/translation/javascript-collection-iterator-call-source.js. Do not edit by hand.
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSourceUnit;
use super::JavaScriptCallTemplate;
use super::JavaScriptCollectionIteratorCallCompilation;
use super::JavaScriptCollectionIteratorCallableDirectory;
use super::JavaScriptCollectionIteratorMethodOwner;
use super::JavaScriptSemanticValue;
use super::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Result;
use super::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop16Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::order_java_script_call_code_functions;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_compile_java_script_collection_iterator_call_directory_loop15;
use super::source_ml_temporary_compile_java_script_collection_iterator_call_directory_loop16;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_code_destination(instruction: self::JavaScriptCallInstruction) -> f64 {
    match instruction.clone() {
        self::JavaScriptCallInstruction::Constant(destination, source_ml_source1_f386) => {
            let source_ml_source1_f386 = *source_ml_source1_f386;
            destination
        }
        self::JavaScriptCallInstruction::Copy(destination_2, source_ml_source1_f387) => {
            destination_2
        }
        self::JavaScriptCallInstruction::Argument(destination_3, source_ml_source1_f388) => {
            destination_3
        }
        self::JavaScriptCallInstruction::ArgumentCount(destination_4) => destination_4,
        self::JavaScriptCallInstruction::ArgumentAt(destination_5, source_ml_source1_f389) => {
            destination_5
        }
        self::JavaScriptCallInstruction::FunctionBind(destination_6) => destination_6,
        self::JavaScriptCallInstruction::BoundFunctionCall(destination_7) => destination_7,
        self::JavaScriptCallInstruction::FunctionToString(destination_8) => destination_8,
        self::JavaScriptCallInstruction::PushFinally(source_ml_source1_f390) => -1f64,
        self::JavaScriptCallInstruction::FinishFinally => -1f64,
        self::JavaScriptCallInstruction::CompleteJump(
            source_ml_source1_f391,
            source_ml_source1_f392,
        ) => -1f64,
        self::JavaScriptCallInstruction::Jump(source_ml_source1_f393) => -1f64,
        self::JavaScriptCallInstruction::Branch(
            source_ml_source1_f394,
            source_ml_source1_f395,
            source_ml_source1_f396,
        ) => -1f64,
        self::JavaScriptCallInstruction::Call(
            destination_9,
            source_ml_source1_f397,
            source_ml_source1_f398,
            source_ml_source1_f399,
        ) => destination_9,
        self::JavaScriptCallInstruction::ArgumentListOpen(source_ml_source1_f400) => -1f64,
        self::JavaScriptCallInstruction::ArgumentListAppend(
            source_ml_source1_f401,
            source_ml_source1_f402,
        ) => -1f64,
        self::JavaScriptCallInstruction::ArgumentListCall(
            destination_10,
            source_ml_source1_f403,
            source_ml_source1_f404,
            source_ml_source1_f405,
            source_ml_source1_f406,
        ) => destination_10,
        self::JavaScriptCallInstruction::Construct(
            destination_11,
            source_ml_source1_f407,
            source_ml_source1_f408,
            source_ml_source1_f409,
        ) => destination_11,
        self::JavaScriptCallInstruction::ConstructorCheck(
            destination_12,
            source_ml_source1_f410,
            source_ml_source1_f411,
        ) => destination_12,
        self::JavaScriptCallInstruction::ReadNewTarget(destination_13) => destination_13,
        self::JavaScriptCallInstruction::ArrayJoinEnter(destination_14, source_ml_source1_f412) => {
            destination_14
        }
        self::JavaScriptCallInstruction::ArrayJoinLeave(source_ml_source1_f413) => -1f64,
        self::JavaScriptCallInstruction::ConstructErrorClass(
            destination_15,
            source_ml_source1_f414,
            source_ml_source1_f415,
        ) => destination_15,
        self::JavaScriptCallInstruction::CallClassConstructor(
            destination_16,
            source_ml_source1_f416,
            source_ml_source1_f417,
            source_ml_source1_f418,
        ) => destination_16,
        self::JavaScriptCallInstruction::ConstructCollection(
            destination_17,
            source_ml_source1_f419,
            source_ml_source1_f420,
        ) => destination_17,
        self::JavaScriptCallInstruction::CollectionAllocate(
            destination_18,
            source_ml_source1_f421,
            source_ml_source1_f422,
        ) => destination_18,
        self::JavaScriptCallInstruction::CollectionIteratorAllocate(
            destination_19,
            source_ml_source1_f423,
            source_ml_source1_f424,
            source_ml_source1_f425,
            source_ml_source1_f426,
        ) => destination_19,
        self::JavaScriptCallInstruction::ArgumentRest(
            destination_20,
            source_ml_source1_f427,
            source_ml_source1_f428,
            source_ml_source1_f429,
        ) => destination_20,
        self::JavaScriptCallInstruction::IteratorProtocol(
            destination_21,
            source_ml_source1_f430,
            source_ml_source1_f431,
            source_ml_source1_f432,
            source_ml_source1_f433,
            source_ml_source1_f434,
            source_ml_source1_f435,
            source_ml_source1_f436,
        ) => destination_21,
        self::JavaScriptCallInstruction::CollectionIteratorResult(
            destination_22,
            source_ml_source1_f437,
            source_ml_source1_f438,
            source_ml_source1_f439,
        ) => destination_22,
        self::JavaScriptCallInstruction::CollectionOperation(
            destination_23,
            source_ml_source1_f440,
        ) => destination_23,
        self::JavaScriptCallInstruction::PrimitiveMethod(
            destination_24,
            source_ml_source1_f441,
        ) => destination_24,
        self::JavaScriptCallInstruction::CollectionForEach(
            destination_25,
            source_ml_source1_f442,
        ) => destination_25,
        self::JavaScriptCallInstruction::ArrayConcatenation(
            destination_26,
            source_ml_source1_f443,
            source_ml_source1_f444,
            source_ml_source1_f445,
        ) => {
            let source_ml_source1_f445 = *source_ml_source1_f445;
            destination_26
        }
        self::JavaScriptCallInstruction::ArraySlice(
            destination_27,
            source_ml_source1_f446,
            source_ml_source1_f447,
        ) => {
            let source_ml_source1_f447 = *source_ml_source1_f447;
            destination_27
        }
        self::JavaScriptCallInstruction::ArrayCreation(
            destination_28,
            source_ml_source1_f448,
            source_ml_source1_f449,
            source_ml_source1_f450,
        ) => destination_28,
        self::JavaScriptCallInstruction::ArrayCallback(
            destination_29,
            source_ml_source1_f451,
            source_ml_source1_f452,
        ) => destination_29,
        self::JavaScriptCallInstruction::ArraySearch(
            destination_30,
            source_ml_source1_f453,
            source_ml_source1_f454,
        ) => destination_30,
        self::JavaScriptCallInstruction::ArrayTransform(
            destination_31,
            source_ml_source1_f455,
            source_ml_source1_f456,
            source_ml_source1_f457,
        ) => {
            let source_ml_source1_f456 = *source_ml_source1_f456;
            destination_31
        }
        self::JavaScriptCallInstruction::ObjectEnumeration(
            destination_32,
            source_ml_source1_f458,
            source_ml_source1_f459,
        ) => destination_32,
        self::JavaScriptCallInstruction::PropertyDefinitions(
            destination_33,
            source_ml_source1_f460,
            source_ml_source1_f461,
            source_ml_source1_f462,
        ) => destination_33,
        self::JavaScriptCallInstruction::PropertyReflection(
            destination_34,
            source_ml_source1_f463,
            source_ml_source1_f464,
            source_ml_source1_f465,
            source_ml_source1_f466,
            source_ml_source1_f467,
        ) => destination_34,
        self::JavaScriptCallInstruction::InstanceOf(
            destination_35,
            source_ml_source1_f468,
            source_ml_source1_f469,
            source_ml_source1_f470,
            source_ml_source1_f471,
        ) => destination_35,
        self::JavaScriptCallInstruction::ObjectObservation(
            destination_36,
            source_ml_source1_f472,
            source_ml_source1_f473,
            source_ml_source1_f474,
        ) => destination_36,
        self::JavaScriptCallInstruction::CollectionCopy(
            destination_37,
            source_ml_source1_f475,
            source_ml_source1_f476,
            source_ml_source1_f477,
            source_ml_source1_f478,
            source_ml_source1_f479,
            source_ml_source1_f480,
            source_ml_source1_f481,
        ) => destination_37,
        self::JavaScriptCallInstruction::ConstructErrorValue(
            destination_38,
            source_ml_source1_f482,
            source_ml_source1_f483,
            source_ml_source1_f484,
            source_ml_source1_f485,
            source_ml_source1_f486,
            source_ml_source1_f487,
            source_ml_source1_f488,
            source_ml_source1_f489,
        ) => destination_38,
        self::JavaScriptCallInstruction::ReadDerivedReceiver(destination_39) => destination_39,
        self::JavaScriptCallInstruction::InvokeErrorIntrinsic(destination_40) => destination_40,
        self::JavaScriptCallInstruction::InvokeHostCapability(destination_41) => destination_41,
        self::JavaScriptCallInstruction::Return(source_ml_source1_f490) => -1f64,
        self::JavaScriptCallInstruction::Throw(source_ml_source1_f491) => -1f64,
        self::JavaScriptCallInstruction::PushHandler(destination_42, source_ml_source1_f492) => {
            destination_42
        }
        self::JavaScriptCallInstruction::PopHandler => -1f64,
        self::JavaScriptCallInstruction::NumberBinary(
            destination_43,
            source_ml_source1_f493,
            source_ml_source1_f494,
            source_ml_source1_f495,
        ) => destination_43,
        self::JavaScriptCallInstruction::SameValue(
            destination_44,
            source_ml_source1_f496,
            source_ml_source1_f497,
        ) => destination_44,
        self::JavaScriptCallInstruction::StrictEqual(
            destination_45,
            source_ml_source1_f498,
            source_ml_source1_f499,
        ) => destination_45,
        self::JavaScriptCallInstruction::RequireWrite(source_ml_source1_f500) => -1f64,
        self::JavaScriptCallInstruction::Primitive(
            destination_46,
            source_ml_source1_f501,
            source_ml_source1_f502,
        ) => destination_46,
        self::JavaScriptCallInstruction::OrdinaryAllocate(
            destination_47,
            source_ml_source1_f503,
        ) => destination_47,
        self::JavaScriptCallInstruction::LiteralPrototype(
            destination_48,
            source_ml_source1_f504,
            source_ml_source1_f505,
        ) => destination_48,
        self::JavaScriptCallInstruction::LiteralAccessor(
            destination_49,
            source_ml_source1_f506,
            source_ml_source1_f507,
            source_ml_source1_f508,
            source_ml_source1_f509,
            source_ml_source1_f510,
        ) => destination_49,
        self::JavaScriptCallInstruction::ObjectFromEntries(
            destination_50,
            source_ml_source1_f511,
            source_ml_source1_f512,
            source_ml_source1_f513,
        ) => destination_50,
        self::JavaScriptCallInstruction::PrimitiveWrapper(
            destination_51,
            source_ml_source1_f514,
            source_ml_source1_f515,
        ) => destination_51,
        self::JavaScriptCallInstruction::SymbolAllocate(destination_52, source_ml_source1_f516) => {
            destination_52
        }
        self::JavaScriptCallInstruction::SymbolDescription(
            destination_53,
            source_ml_source1_f517,
        ) => destination_53,
        self::JavaScriptCallInstruction::SymbolRegistry(
            destination_54,
            source_ml_source1_f518,
            source_ml_source1_f519,
        ) => destination_54,
        self::JavaScriptCallInstruction::FunctionName(
            destination_55,
            source_ml_source1_f520,
            source_ml_source1_f521,
            source_ml_source1_f522,
            source_ml_source1_f523,
        ) => destination_55,
        self::JavaScriptCallInstruction::ArrayAllocate(
            destination_56,
            source_ml_source1_f524,
            source_ml_source1_f525,
        ) => destination_56,
        self::JavaScriptCallInstruction::ClosureAllocate(
            destination_57,
            source_ml_source1_f526,
            source_ml_source1_f527,
            source_ml_source1_f528,
            source_ml_source1_f529,
        ) => destination_57,
        self::JavaScriptCallInstruction::ArraySpread(
            destination_58,
            source_ml_source1_f530,
            source_ml_source1_f531,
            source_ml_source1_f532,
            source_ml_source1_f533,
            source_ml_source1_f534,
        ) => destination_58,
        self::JavaScriptCallInstruction::ObjectSpread(
            destination_59,
            source_ml_source1_f535,
            source_ml_source1_f536,
            source_ml_source1_f537,
            source_ml_source1_f538,
            source_ml_source1_f539,
        ) => destination_59,
        self::JavaScriptCallInstruction::OrdinaryDataProperty(
            destination_60,
            source_ml_source1_f540,
            source_ml_source1_f541,
            source_ml_source1_f542,
            source_ml_source1_f543,
        ) => destination_60,
        self::JavaScriptCallInstruction::OrdinaryGet(
            destination_61,
            source_ml_source1_f544,
            source_ml_source1_f545,
            source_ml_source1_f546,
        ) => destination_61,
        self::JavaScriptCallInstruction::OrdinaryDelete(
            destination_62,
            source_ml_source1_f547,
            source_ml_source1_f548,
        ) => destination_62,
        self::JavaScriptCallInstruction::OrdinaryHas(
            destination_63,
            source_ml_source1_f549,
            source_ml_source1_f550,
        ) => destination_63,
        self::JavaScriptCallInstruction::OrdinarySet(
            destination_64,
            source_ml_source1_f551,
            source_ml_source1_f552,
            source_ml_source1_f553,
            source_ml_source1_f554,
        ) => destination_64,
        self::JavaScriptCallInstruction::OrdinaryDefine(
            destination_65,
            source_ml_source1_f555,
            source_ml_source1_f556,
            source_ml_source1_f557,
            source_ml_source1_f558,
        ) => destination_65,
        self::JavaScriptCallInstruction::PropertyKey(
            destination_66,
            source_ml_source1_f559,
            source_ml_source1_f560,
        ) => destination_66,
        self::JavaScriptCallInstruction::PropertyDefinition(
            destination_67,
            source_ml_source1_f561,
            source_ml_source1_f562,
            source_ml_source1_f563,
            source_ml_source1_f564,
            source_ml_source1_f565,
        ) => destination_67,
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_constant_error(
    value: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    self::read_java_script_semantic_value_error(value.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_strict_equal(
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
) -> bool {
    if ((self::read_java_script_semantic_value_error(left.clone()))
        != (self::ml_string::Text::from_units(vec![])))
        || ((self::read_java_script_semantic_value_error(right.clone()))
            != (self::ml_string::Text::from_units(vec![])))
    {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                115, 116, 114, 105, 99, 116, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 114,
                101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32,
                115, 101, 109, 97, 110, 116, 105, 99, 32, 111, 112, 101, 114, 97, 110, 100, 115,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        match left.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Null => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f566) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Number(left_value) => match right.clone() {
                self::JavaScriptSemanticValue::Undefined => false,
                self::JavaScriptSemanticValue::Null => false,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f574) => false,
                self::JavaScriptSemanticValue::Number(value) => left_value == value,
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f575) => false,
                self::JavaScriptSemanticValue::Text(source_ml_source1_f576) => false,
                self::JavaScriptSemanticValue::SourceText(
                    source_ml_source1_f577,
                    source_ml_source1_f578,
                ) => false,
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f579) => false,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f580,
                    source_ml_source1_f581,
                ) => false,
            },
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f567) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f568) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::SourceText(
                source_ml_source1_f569,
                source_ml_source1_f570,
            ) => self::test_java_script_semantic_same_value(left.clone(), right.clone()),
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f571) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Object(
                source_ml_source1_f572,
                source_ml_source1_f573,
            ) => self::test_java_script_semantic_same_value(left.clone(), right.clone()),
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_collection_iterator_intrinsic_mode(mode: &self::ml_string::Text) -> bool {
    (*mode)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111,
            114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99,
        ]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn normalize_java_script_collection_iterator_operation(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*operation)
        == (self::ml_string::Text::from_units(vec![
            83, 116, 114, 105, 110, 103, 46, 105, 116, 101, 114, 97, 116, 111, 114,
        ]))
    {
        self::ml_string::Text::from_units(vec![
            83, 116, 114, 105, 110, 103, 46, 118, 97, 108, 117, 101, 115,
        ])
    } else if (*operation)
        == (self::ml_string::Text::from_units(vec![83, 101, 116, 46, 107, 101, 121, 115]))
    {
        self::ml_string::Text::from_units(vec![83, 101, 116, 46, 118, 97, 108, 117, 101, 115])
    } else {
        operation.clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_iterator_operation_kind(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![
                83, 116, 114, 105, 110, 103, 46, 105, 116, 101, 114, 97, 116, 111, 114,
            ]),
            self::ml_string::Text::from_units(vec![
                83, 116, 114, 105, 110, 103, 46, 118, 97, 108, 117, 101, 115,
            ]),
            self::ml_string::Text::from_units(vec![
                83, 116, 114, 105, 110, 103, 73, 116, 101, 114, 97, 116, 111, 114, 46, 110, 101,
                120, 116,
            ]),
        ],
        operation,
        0f64,
        false,
        |a, b| a == b,
    ) {
        self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])
    } else if self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 107, 101, 121, 115]),
            self::ml_string::Text::from_units(vec![
                65, 114, 114, 97, 121, 46, 118, 97, 108, 117, 101, 115,
            ]),
            self::ml_string::Text::from_units(vec![
                65, 114, 114, 97, 121, 46, 101, 110, 116, 114, 105, 101, 115,
            ]),
            self::ml_string::Text::from_units(vec![
                65, 114, 114, 97, 121, 73, 116, 101, 114, 97, 116, 111, 114, 46, 110, 101, 120, 116,
            ]),
        ],
        operation,
        0f64,
        false,
        |a, b| a == b,
    ) {
        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121])
    } else if self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![77, 97, 112, 46, 107, 101, 121, 115]),
            self::ml_string::Text::from_units(vec![77, 97, 112, 46, 118, 97, 108, 117, 101, 115]),
            self::ml_string::Text::from_units(vec![
                77, 97, 112, 46, 101, 110, 116, 114, 105, 101, 115,
            ]),
            self::ml_string::Text::from_units(vec![
                77, 97, 112, 73, 116, 101, 114, 97, 116, 111, 114, 46, 110, 101, 120, 116,
            ]),
        ],
        operation,
        0f64,
        false,
        |a, b| a == b,
    ) {
        self::ml_string::Text::from_units(vec![77, 97, 112])
    } else if self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![83, 101, 116, 46, 107, 101, 121, 115]),
            self::ml_string::Text::from_units(vec![83, 101, 116, 46, 118, 97, 108, 117, 101, 115]),
            self::ml_string::Text::from_units(vec![
                83, 101, 116, 46, 101, 110, 116, 114, 105, 101, 115,
            ]),
            self::ml_string::Text::from_units(vec![
                83, 101, 116, 73, 116, 101, 114, 97, 116, 111, 114, 46, 110, 101, 120, 116,
            ]),
        ],
        operation,
        0f64,
        false,
        |a, b| a == b,
    ) {
        self::ml_string::Text::from_units(vec![83, 101, 116])
    } else if (*operation)
        == (self::ml_string::Text::from_units(vec![
            73, 116, 101, 114, 97, 116, 111, 114, 46, 115, 101, 108, 102,
        ]))
    {
        self::ml_string::Text::from_units(vec![115, 101, 108, 102])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_iterator_operation_strategy(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let actual = self::normalize_java_script_collection_iterator_operation(operation);
        if actual.ends_with(&self::ml_string::Text::from_units(vec![
            46, 107, 101, 121, 115,
        ])) {
            self::ml_string::Text::from_units(vec![107, 101, 121, 115])
        } else if actual.ends_with(&self::ml_string::Text::from_units(vec![
            46, 118, 97, 108, 117, 101, 115,
        ])) {
            self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115])
        } else if actual.ends_with(&self::ml_string::Text::from_units(vec![
            46, 101, 110, 116, 114, 105, 101, 115,
        ])) {
            self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_collection_iterator_method_alias(
    left: self::JavaScriptCollectionIteratorMethodOwner,
    right: self::JavaScriptCollectionIteratorMethodOwner,
) -> bool {
    {
        let operation = self::normalize_java_script_collection_iterator_operation(
            &(match left.clone() {
                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(
                    _,
                    _,
                    source_ml_source1_data_property_value582,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value582.clone(),
            }),
        );
        if (((match left.clone() {
            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value585, _, _, _, _, _) => {
                source_ml_source1_data_property_value585.clone()
            }
        }) != (match right.clone() {
            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value586, _, _, _, _, _) => {
                source_ml_source1_data_property_value586.clone()
            }
        })) || ((match left.clone() {
            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, source_ml_source1_data_property_value589, _, _, _, _) => {
                source_ml_source1_data_property_value589.clone()
            }
        }) != (match right.clone() {
            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, source_ml_source1_data_property_value590, _, _, _, _) => {
                source_ml_source1_data_property_value590.clone()
            }
        }))) || (operation != (self::normalize_java_script_collection_iterator_operation(&(match right.clone() {
            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, source_ml_source1_data_property_value592, _, _, _) => {
                source_ml_source1_data_property_value592.clone()
            }
        })))) {
            false
        } else if operation == (self::ml_string::Text::from_units(vec![73, 116, 101, 114, 97, 116, 111, 114, 46, 115, 101, 108, 102])) {
                true
            } else {
let ml_library_inequality_1 = (self::read_java_script_collection_iterator_operation_strategy(&(operation))) != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                    self::test_java_script_semantic_same_value(match left.clone() {
                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value593, _, _) => {
                            let source_ml_source1_data_property_value593 = *source_ml_source1_data_property_value593;
                            source_ml_source1_data_property_value593.clone()
                        }
                    }, match right.clone() {
                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value594, _, _) => {
                            let source_ml_source1_data_property_value594 = *source_ml_source1_data_property_value594;
                            source_ml_source1_data_property_value594.clone()
                        }
                    })
                } else {
                    self::test_java_script_semantic_same_value(match left.clone() {
                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, source_ml_source1_data_property_value595, _) => {
                            let source_ml_source1_data_property_value595 = *source_ml_source1_data_property_value595;
                            source_ml_source1_data_property_value595.clone()
                        }
                    }, match right.clone() {
                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, source_ml_source1_data_property_value596, _) => {
                            let source_ml_source1_data_property_value596 = *source_ml_source1_data_property_value596;
                            source_ml_source1_data_property_value596.clone()
                        }
                    }) && self::test_java_script_semantic_same_value(match left.clone() {
                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, _, source_ml_source1_data_property_value597) => {
                            let source_ml_source1_data_property_value597 = *source_ml_source1_data_property_value597;
                            source_ml_source1_data_property_value597.clone()
                        }
                    }, match right.clone() {
                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, _, source_ml_source1_data_property_value598) => {
                            let source_ml_source1_data_property_value598 = *source_ml_source1_data_property_value598;
                            source_ml_source1_data_property_value598.clone()
                        }
                    })
                }
}
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_collection_iterator_call_directory(
    directory: self::JavaScriptCollectionIteratorCallableDirectory,
) -> self::JavaScriptCollectionIteratorCallCompilation {
    {
        let empty = {
            let source_ml_source1_constructor_field599 = Vec::<self::JavaScriptCallFunction>::new();
            {
                let source_ml_source1_constructor_field601 = match directory.clone() {
                    self::JavaScriptCollectionIteratorCallableDirectory::CollectionIteratorCallableDirectory(source_ml_source1_data_property_value600, _, _) => {
                        source_ml_source1_data_property_value600
                    }
                };
                {
                    let source_ml_source1_constructor_field603 = match directory.clone() {
                        self::JavaScriptCollectionIteratorCallableDirectory::CollectionIteratorCallableDirectory(_, source_ml_source1_data_property_value602, _) => {
                            let source_ml_source1_data_property_value602 = *source_ml_source1_data_property_value602;
                            source_ml_source1_data_property_value602.clone()
                        }
                    };
                    self::JavaScriptCallProgram::CallProgram(
                        source_ml_source1_constructor_field599,
                        source_ml_source1_constructor_field601,
                        Box::new(source_ml_source1_constructor_field603),
                        None::<(Option<Vec<self::JavaScriptCallTemplate>>,)>,
                        None::<(Option<self::JavaScriptArraySpeciesDirectory>,)>,
                        None::<(Option<Vec<self::JavaScriptCallSourceUnit>>,)>,
                    )
                }
            }
        };
        if (((match &directory { self::JavaScriptCollectionIteratorCallableDirectory::CollectionIteratorCallableDirectory(_, _, source_ml_source1_data_property_value605) => source_ml_source1_data_property_value605.as_slice() }).len() as f64) > 64f64) || (((match &directory { self::JavaScriptCollectionIteratorCallableDirectory::CollectionIteratorCallableDirectory(source_ml_source1_data_property_value607, _, _) => source_ml_source1_data_property_value607.as_slice() }).len() as f64) > 4096f64) {
            self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 100, 105, 114, 101, 99, 116, 111, 114, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 119, 110, 101, 114, 32, 98, 117, 100, 103, 101, 116]))
        } else {
            {
                let functions = Vec::<self::JavaScriptCallFunction>::new();
                {
                    let owners = Vec::<self::JavaScriptCollectionIteratorMethodOwner>::new();
                    {
                        let source_ml_temporary_values13 = match directory.clone() {
                            self::JavaScriptCollectionIteratorCallableDirectory::CollectionIteratorCallableDirectory(_, _, source_ml_source1_data_property_value608) => {
                                source_ml_source1_data_property_value608
                            }
                        };
                        {
                            let source_ml_temporary_index13 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source1_s609 = self::source_ml_temporary_compile_java_script_collection_iterator_call_directory_loop15(directory.clone(), empty.clone(), functions, owners, source_ml_temporary_values13, source_ml_temporary_index13);
                                match source_ml_source1_s609 {
                                    self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Done(functions_2, owners_2) => {
                                        {
                                            let ordered = self::order_java_script_call_code_functions(functions_2);
                                            {
                                                let program = {
                                                    let source_ml_source1_nominal_spread610 = empty.clone();
                                                    {
                                                        let source_ml_source1_nominal_field611 = match source_ml_source1_nominal_spread610.clone() {
                                                            self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value612, _, _, _, _, _) => {
                                                                source_ml_source1_data_property_value612
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_field613 = match source_ml_source1_nominal_spread610.clone() {
                                                                self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value614, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value614
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field615 = match source_ml_source1_nominal_spread610.clone() {
                                                                    self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source1_data_property_value616, _, _, _) => {
                                                                        let source_ml_source1_data_property_value616 = *source_ml_source1_data_property_value616;
                                                                        source_ml_source1_data_property_value616.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_field617 = match source_ml_source1_nominal_spread610.clone() {
                                                                        self::JavaScriptCallProgram::CallProgram(_, _, _, source_ml_source1_data_property_value618, _, _) => {
                                                                            source_ml_source1_data_property_value618.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_nominal_field619 = match source_ml_source1_nominal_spread610.clone() {
                                                                            self::JavaScriptCallProgram::CallProgram(_, _, _, _, source_ml_source1_data_property_value620, _) => {
                                                                                source_ml_source1_data_property_value620.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_nominal_field621 = match source_ml_source1_nominal_spread610.clone() {
                                                                                self::JavaScriptCallProgram::CallProgram(_, _, _, _, _, source_ml_source1_data_property_value622) => {
                                                                                    source_ml_source1_data_property_value622.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_override623 = ordered;
                                                                                self::JavaScriptCallProgram::CallProgram(source_ml_source1_nominal_override623, source_ml_source1_nominal_field613, Box::new(source_ml_source1_nominal_field615), source_ml_source1_nominal_field617, source_ml_source1_nominal_field619, source_ml_source1_nominal_field621)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                {
                                                    let source_ml_temporary_values14 = owners_2;
                                                    {
                                                        let source_ml_temporary_index14 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source1_s624 = self::source_ml_temporary_compile_java_script_collection_iterator_call_directory_loop16(empty.clone(), program.clone(), source_ml_temporary_values14, source_ml_temporary_index14);
                                                            match source_ml_source1_s624 {
                                                                self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop16Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop16Done => {
                                                                    self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(program.clone()), self::ml_string::Text::from_units(vec![]))
                                                                }
                                                                self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop16Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop16Return(source_ml_temporary_result16) => {
                                                                    let source_ml_temporary_result16 = *source_ml_temporary_result16;
                                                                    source_ml_temporary_result16.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Return(source_ml_temporary_result16_2) => {
                                        let source_ml_temporary_result16_2 = *source_ml_temporary_result16_2;
                                        source_ml_temporary_result16_2.clone()
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(
    units: &[f64],
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if at < (units.len() as f64) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done,
    MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(bool),
}
