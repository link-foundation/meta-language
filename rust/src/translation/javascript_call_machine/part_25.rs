// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCompletion;
use super::JavaScriptArrayDefinitionState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryAcceptStringNumberInfinityLoop158Result;
use super::MlSource1TemporaryConvertStringNumberRationalJoin150;
use super::MlSource1TemporaryConvertStringNumberRationalJoin151;
use super::MlSource1TemporaryConvertStringNumberRationalLoop152Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop153Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop156Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop157Result;
use super::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result;
use super::MlSource1TemporaryReadStringNumberBinaryPowerLoop148Result;
use super::convert_java_script_array_unsigned_length;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_array_definition_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_source1_temporary_accept_string_number_infinity_loop158;
use super::source_ml_source1_temporary_convert_string_number_rational_loop152;
use super::source_ml_source1_temporary_convert_string_number_rational_loop153;
use super::source_ml_source1_temporary_convert_string_number_rational_loop154;
use super::source_ml_source1_temporary_convert_string_number_rational_loop155;
use super::source_ml_source1_temporary_convert_string_number_rational_loop156;
use super::source_ml_source1_temporary_convert_string_number_rational_loop157;
use super::source_ml_source1_temporary_read_java_script_indexed_property_presence_loop138;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop148;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop149;
use super::source_ml_source1_temporary_read_string_number_decimal_power_loop146;
use super::source_ml_source1_temporary_read_string_number_exact_mantissa_loop147;
use super::source_ml_source3_record_order;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_array_property_number(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayDefinitionState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptArrayCompletion {
    match state.clone() {
        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field6 = *shared_constructor_field6;
            {
                let error = self::read_java_script_array_definition_state_error(
                    heap.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArrayCompletion::ArrayInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    {
                        let invalid = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            value.clone(),
                        );
                        if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                            self::JavaScriptArrayCompletion::ArrayInvalid(
                                Box::new(heap.clone()),
                                invalid.clone(),
                            )
                        } else {
                            match value.clone() {
                                self::JavaScriptSemanticValue::Number(value_value) => {
                                    if (shared_constructor_field4
                                        == (self::ml_string::Text::from_units(vec![
                                            97, 112, 112, 108, 121,
                                        ])))
                                    {
                                        self::JavaScriptArrayCompletion::ArrayInvalid(
                                            Box::new(heap.clone()),
                                            self::ml_string::Text::from_units(vec![
                                                65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114,
                                                105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105,
                                                111, 110, 32, 114, 101, 115, 117, 109, 101, 115,
                                                32, 111, 110, 99, 101, 32, 119, 105, 116, 104, 32,
                                                105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                                                84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101,
                                                115, 117, 108, 116,
                                            ]),
                                        )
                                    } else {
                                        {
                                            let next = if (shared_constructor_field4
                                                == (self::ml_string::Text::from_units(vec![
                                                    102, 105, 114, 115, 116, 32, 110, 117, 109, 98,
                                                    101, 114,
                                                ]))) {
                                                {
                                                    let source_ml_source2_nominal_field3326 =
                                                        shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_nominal_field3328 =
                                                            shared_constructor_field2.clone();
                                                        {
                                                            let source_ml_source2_nominal_field3330 =
                                                                shared_constructor_field3.clone();
                                                            {
                                                                let source_ml_source2_nominal_field3332 =
                                                                    shared_constructor_field4
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field3334 =
                                                                        shared_constructor_field5;
                                                                    {
                                                                        let source_ml_source2_nominal_field3336 = shared_constructor_field6.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_override3339 = self::convert_java_script_array_unsigned_length(value_value);
                                                                            self::JavaScriptArrayDefinitionState::ArrayDefinitionState(source_ml_source2_nominal_field3326.clone(), Box::new(source_ml_source2_nominal_field3328.clone()), source_ml_source2_nominal_field3330.clone(), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 32, 110, 117, 109, 98, 101, 114]), source_ml_source2_nominal_override3339, Box::new(source_ml_source2_nominal_field3336.clone()))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                {
                                                    let source_ml_source2_nominal_field3341 =
                                                        shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_nominal_field3343 =
                                                            shared_constructor_field2.clone();
                                                        {
                                                            let source_ml_source2_nominal_field3345 =
                                                                shared_constructor_field3.clone();
                                                            {
                                                                let source_ml_source2_nominal_field3347 =
                                                                    shared_constructor_field4
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field3349 =
                                                                        shared_constructor_field5;
                                                                    {
                                                                        let source_ml_source2_nominal_field3351 = shared_constructor_field6.clone();
                                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(source_ml_source2_nominal_field3341.clone(), Box::new(source_ml_source2_nominal_field3343.clone()), source_ml_source2_nominal_field3345.clone(), self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]), source_ml_source2_nominal_field3349, Box::new(value.clone()))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            };
                                            self::JavaScriptArrayCompletion::ArrayDefinitionContinue(
                                                Box::new(heap.clone()),
                                                Box::new(next.clone()),
                                            )
                                        }
                                    }
                                }
                                _ => self::JavaScriptArrayCompletion::ArrayInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105,
                                        99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                                        32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 99,
                                        101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99,
                                        116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114,
                                        32, 114, 101, 115, 117, 108, 116,
                                    ]),
                                ),
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
pub fn test_java_script_array_value(
    heap: self::JavaScriptOrdinaryHeap,
    value: self::JavaScriptSemanticValue,
) -> std::rc::Rc<std::cell::RefCell<((bool, self::ml_string::Text), Vec<f64>)>> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source3_record_field367 = false;
                    {
                        let source_ml_source3_record_field368 = error.clone();
                        ((source_ml_source3_record_field367, source_ml_source3_record_field368.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![97f64, 114f64, 114f64, 97f64, 121f64], vec![101f64, 114f64, 114f64, 111f64, 114f64]], vec![0f64, 1f64]))
                    }
                }))
            } else {
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source3_record_field369 = false;
                            {
                                let source_ml_source3_record_field370 = invalid.clone();
                                ((source_ml_source3_record_field369, source_ml_source3_record_field370.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![97f64, 114f64, 114f64, 97f64, 121f64], vec![101f64, 114f64, 114f64, 111f64, 114f64]], vec![0f64, 1f64]))
                            }
                        }))
                    } else {
                        match value.clone() {
                            self::JavaScriptSemanticValue::Object(identity, _) => {
                                let object = {
                                    let source_ml_source2_array_read_value3362 =
                                        shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_array_read_index3363 =
                                            self::find_java_script_ordinary_object(
                                                shared_constructor_field1.clone(),
                                                identity.clone(),
                                            );
                                        {
                                            let ml_reference_array304 =
                                                source_ml_source2_array_read_value3362.clone();
                                            let ml_reference_position304 =
                                                source_ml_source2_array_read_index3363;
                                            let ml_reference_projection304 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array304.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position304,
                                                    ),
                                                );
                                            ml_reference_projection304
                                        }
                                    }
                                };
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_field3376 =
                                        (!(match (object.clone(), None::<()>) {
                                            (Some(_ml_left), Some(_ml_right)) => false,
                                            (None, None) => true,
                                            _ => false,
                                        }) && (match object
                                            .clone()
                                            .expect("a proven present value")
                                        {
                                            self::JavaScriptOrdinaryObject::ArrayObject(
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => true,
                                            _ => false,
                                        } || match object
                                            .clone()
                                            .expect("a proven present value")
                                        {
                                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => true,
                                            _ => false,
                                        }));
                                    {
                                        let source_ml_source3_record_field371 =
                                            source_ml_source2_field3376;
                                        {
                                            let source_ml_source3_record_field372 =
                                                self::ml_string::Text::from_units(vec![]);
                                            ((source_ml_source3_record_field371, source_ml_source3_record_field372.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![97f64, 114f64, 114f64, 97f64, 121f64], vec![101f64, 114f64, 114f64, 111f64, 114f64]], vec![0f64, 1f64]))
                                        }
                                    }
                                }))
                            }
                            _ => std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source3_record_field373 = false;
                                {
                                    let source_ml_source3_record_field374 =
                                        self::ml_string::Text::from_units(vec![]);
                                    ((source_ml_source3_record_field373, source_ml_source3_record_field374.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![97f64, 114f64, 114f64, 97f64, 121f64], vec![101f64, 114f64, 114f64, 111f64, 114f64]], vec![0f64, 1f64]))
                                }
                            })),
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_like_length(value: f64) -> f64 {
    if (value.is_nan() || (value <= (0f64))) {
        0f64
    } else {
        self::ml_math::min(value.floor(), 9007199254740991f64)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_array_search_operation(operation: &self::ml_string::Text) -> bool {
    self::ml_array::includes(
        &{
            let ml_reference_value305 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                self::ml_string::Text::from_units(vec![
                    65, 114, 114, 97, 121, 46, 105, 110, 99, 108, 117, 100, 101, 115,
                ]),
                self::ml_string::Text::from_units(vec![
                    65, 114, 114, 97, 121, 46, 105, 110, 100, 101, 120, 79, 102,
                ]),
                self::ml_string::Text::from_units(vec![
                    65, 114, 114, 97, 121, 46, 108, 97, 115, 116, 73, 110, 100, 101, 120, 79, 102,
                ]),
                self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 97, 116]),
            ])))
            .borrow()
            .clone();
            ml_reference_value305
        },
        operation,
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_search_index(
    operation: &self::ml_string::Text,
    length: f64,
    present: bool,
    value: f64,
) -> f64 {
    if (((*operation)
        == (self::ml_string::Text::from_units(vec![
            65, 114, 114, 97, 121, 46, 108, 97, 115, 116, 73, 110, 100, 101, 120, 79, 102,
        ])))
        && !present)
    {
        (length - 1f64)
    } else {
        {
            let integer = if (value.is_nan() || (value == (0f64))) {
                0f64
            } else {
                value.trunc()
            };
            if ((*operation)
                == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 97, 116])))
            {
                {
                    let index = if (integer < (0f64)) {
                        (length + integer)
                    } else {
                        integer
                    };
                    if ((index < (0f64)) || (index >= length)) {
                        (-1f64)
                    } else {
                        index
                    }
                }
            } else {
                if ((*operation)
                    == (self::ml_string::Text::from_units(vec![
                        65, 114, 114, 97, 121, 46, 108, 97, 115, 116, 73, 110, 100, 101, 120, 79,
                        102,
                    ])))
                {
                    if (integer == (f64::NEG_INFINITY)) {
                        (-1f64)
                    } else {
                        if (integer >= (0f64)) {
                            self::ml_math::min(integer, (length - 1f64))
                        } else {
                            self::ml_math::max((length + integer), (-1f64))
                        }
                    }
                } else {
                    if (integer == (f64::INFINITY)) {
                        length
                    } else {
                        if (integer >= (0f64)) {
                            self::ml_math::min(integer, length)
                        } else {
                            self::ml_math::max((length + integer), 0f64)
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_indexed_property_presence(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
) -> std::rc::Rc<std::cell::RefCell<((self::ml_string::Text, bool), Vec<f64>)>> {
    {
        let depth = 0f64;
        {
            let source_ml_source2_s3380 = self::source_ml_source1_temporary_read_java_script_indexed_property_presence_loop138(heap.clone(), key.clone(), identity.clone(), depth);
            match source_ml_source2_s3380.clone() {
                self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Done => {
                    std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source3_record_field375 = self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 111, 107, 117, 112, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]);
                        {
                            let source_ml_source3_record_field376 = false;
                            ((source_ml_source3_record_field375.clone(), source_ml_source3_record_field376), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![112f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64]))
                        }
                    }))
                }
                self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Return(source_ml_source1_temporary_result138) => {
                    source_ml_source1_temporary_result138.clone()
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_whitespace_unit(unit: f64) -> bool {
    ((((((((((((unit >= (9f64)) && (unit <= (13f64))) || (unit == (32f64)))
        || (unit == (160f64)))
        || (unit == (5760f64)))
        || ((unit >= (8192f64)) && (unit <= (8202f64))))
        || (unit == (8232f64)))
        || (unit == (8233f64)))
        || (unit == (8239f64)))
        || (unit == (8287f64)))
        || (unit == (12288f64)))
        || (unit == (65279f64)))
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
                let value_2 =
                    self::source_ml_source1_temporary_read_string_number_decimal_power_loop146(
                        exponent,
                        value.clone(),
                        base.clone(),
                    );
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
                let result_2 =
                    self::source_ml_source1_temporary_read_string_number_exact_mantissa_loop147(
                        value.clone(),
                        result,
                        scale,
                    );
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
            let source_ml_source2_s3385 =
                self::source_ml_source1_temporary_read_string_number_binary_power_loop148(
                    exponent, result,
                );
            match source_ml_source2_s3385.clone() {
                self::MlSource1TemporaryReadStringNumberBinaryPowerLoop148Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop148Done(exponent_2, result_2) => {
                    {
                        let result_3 = self::source_ml_source1_temporary_read_string_number_binary_power_loop149(exponent_2, result_2);
                        result_3
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_string_number_rational(numerator: self::ml::Big, denominator: self::ml::Big) -> f64 {
    if ((numerator < (self::ml::Big::from_i128(0)))
        || (denominator <= (self::ml::Big::from_i128(0))))
    {
        f64::NAN
    } else {
        if (numerator == (self::ml::Big::from_i128(0))) {
            0f64
        } else {
            {
                let exponent = 0f64;
                {
                    let source_ml_source2_s3388 = if (numerator >= denominator) {
                        {
                            let source_ml_source2_s3389 = self::source_ml_source1_temporary_convert_string_number_rational_loop156(numerator.clone(), exponent, denominator.clone());
                            match source_ml_source2_s3389.clone() {
                                self::MlSource1TemporaryConvertStringNumberRationalLoop156Result::MlSource1TemporaryConvertStringNumberRationalLoop156Done(exponent_2) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin150::MlSource1TemporaryConvertStringNumberRationalJoin150Next(exponent_2)
                                }
                                self::MlSource1TemporaryConvertStringNumberRationalLoop156Result::MlSource1TemporaryConvertStringNumberRationalLoop156Return(source_ml_source1_temporary_result156) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin150::MlSource1TemporaryConvertStringNumberRationalJoin150Return(source_ml_source1_temporary_result156)
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_source2_s3390 = self::source_ml_source1_temporary_convert_string_number_rational_loop157(denominator.clone(), exponent, numerator.clone());
                            match source_ml_source2_s3390.clone() {
                                self::MlSource1TemporaryConvertStringNumberRationalLoop157Result::MlSource1TemporaryConvertStringNumberRationalLoop157Done(exponent_3) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin150::MlSource1TemporaryConvertStringNumberRationalJoin150Next(exponent_3)
                                }
                                self::MlSource1TemporaryConvertStringNumberRationalLoop157Result::MlSource1TemporaryConvertStringNumberRationalLoop157Return(source_ml_source1_temporary_result157) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin150::MlSource1TemporaryConvertStringNumberRationalJoin150Return(source_ml_source1_temporary_result157)
                                }
                            }
                        }
                    };
                    match source_ml_source2_s3388.clone() {
                        self::MlSource1TemporaryConvertStringNumberRationalJoin150::MlSource1TemporaryConvertStringNumberRationalJoin150Next(exponent_4) => {
                            {
                                let shift = if (exponent_4 >= ((-1022f64))) {
                                    (52f64 - exponent_4)
                                } else {
                                    1074f64
                                };
                                {
                                    let source_ml_source2_s3391 = if (shift >= (0f64)) {
                                        {
                                            let index = 0f64;
                                            {
                                                let numerator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop154(numerator.clone(), shift, index);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin151::MlSource1TemporaryConvertStringNumberRationalJoin151Next(numerator_2.clone(), denominator.clone())
                                            }
                                        }
                                    } else {
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let denominator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop155(denominator.clone(), shift, index_2);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin151::MlSource1TemporaryConvertStringNumberRationalJoin151Next(numerator.clone(), denominator_2.clone())
                                            }
                                        }
                                    };
                                    match source_ml_source2_s3391.clone() {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin151::MlSource1TemporaryConvertStringNumberRationalJoin151Next(numerator_3, denominator_3) => {
                                            {
                                                let quotient = self::ml::Big::from_i128(0);
                                                {
                                                    let remainder = numerator_3.clone();
                                                    {
                                                        let scale = denominator_3.clone();
                                                        {
                                                            let weight = self::ml::Big::from_i128(1);
                                                            {
                                                                let source_ml_source2_s3392 = self::source_ml_source1_temporary_convert_string_number_rational_loop152(remainder.clone(), scale.clone(), weight.clone());
                                                                match source_ml_source2_s3392.clone() {
                                                                    self::MlSource1TemporaryConvertStringNumberRationalLoop152Result::MlSource1TemporaryConvertStringNumberRationalLoop152Done(scale_2, weight_2) => {
                                                                        {
                                                                            let source_ml_source2_s3393 = self::source_ml_source1_temporary_convert_string_number_rational_loop153(quotient.clone(), remainder.clone(), scale_2.clone(), weight_2.clone());
                                                                            match source_ml_source2_s3393.clone() {
                                                                                self::MlSource1TemporaryConvertStringNumberRationalLoop153Result::MlSource1TemporaryConvertStringNumberRationalLoop153Done(quotient_2, remainder_2) => {
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
                        self::MlSource1TemporaryConvertStringNumberRationalJoin150::MlSource1TemporaryConvertStringNumberRationalJoin150Return(source_ml_source1_temporary_result155) => {
                            source_ml_source1_temporary_result155
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_string_number_infinity(
    units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    start: f64,
    end: f64,
) -> bool {
    if ((end - start) != (8f64)) {
        false
    } else {
        {
            let expected = std::rc::Rc::new(std::cell::RefCell::new(vec![
                73f64, 110f64, 102f64, 105f64, 110f64, 105f64, 116f64, 121f64,
            ]));
            {
                let index = 0f64;
                {
                    let source_ml_source2_s3394 =
                        self::source_ml_source1_temporary_accept_string_number_infinity_loop158(
                            units.clone(),
                            start,
                            expected.clone(),
                            index,
                        );
                    match source_ml_source2_s3394.clone() {
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop158Result::MlSource1TemporaryAcceptStringNumberInfinityLoop158Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop158Result::MlSource1TemporaryAcceptStringNumberInfinityLoop158Return(source_ml_source1_temporary_result158) => {
                            source_ml_source1_temporary_result158
                        }
                    }
                }
            }
        }
    }
}
