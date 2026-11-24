// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop227Result;
use super::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Result;
use super::convert_java_script_semantic_boolean;
use super::create_java_script_empty_descriptor;
use super::ml_math;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_descriptor_error;
use super::source_ml_source1_temporary_read_java_script_descriptor_conversion_state_error_loop227;
use super::source_ml_source1_temporary_step_java_script_descriptor_conversion_loop228;
use super::source_ml_source3_record_order;
use super::test_java_script_semantic_accessor;
use super::test_java_script_standard_array_transform_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_descriptor_field_presence(
    descriptor: std::rc::Rc<
        std::cell::RefCell<(
            (
                bool,
                bool,
                bool,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                bool,
                bool,
            ),
            Vec<f64>,
        )>,
    >,
    index: f64,
) -> bool {
    if (index == (0f64)) {
        (({
            let ml_reference_value381 = descriptor.borrow().clone();
            ml_reference_value381
        })
        .0)
            .3
    } else {
        if (index == (1f64)) {
            (({
                let ml_reference_value382 = descriptor.borrow().clone();
                ml_reference_value382
            })
            .0)
                .1
        } else {
            if (index == (2f64)) {
                (({
                    let ml_reference_value383 = descriptor.borrow().clone();
                    ml_reference_value383
                })
                .0)
                    .9
            } else {
                if (index == (3f64)) {
                    (({
                        let ml_reference_value384 = descriptor.borrow().clone();
                        ml_reference_value384
                    })
                    .0)
                        .11
                } else {
                    if (index == (4f64)) {
                        (({
                            let ml_reference_value385 = descriptor.borrow().clone();
                            ml_reference_value385
                        })
                        .0)
                            .5
                    } else {
                        if (index == (5f64)) {
                            (({
                                let ml_reference_value386 = descriptor.borrow().clone();
                                ml_reference_value386
                            })
                            .0)
                                .7
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_descriptor_conversion_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptDescriptorConversionState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                    heap_error.clone()
                } else {
                    if {
                        let source_ml_source2_s4287 = shared_constructor_field1.clone();
                        match source_ml_source2_s4287.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => false,
                            _ => true,
                        }
                    } {
                        self::ml_string::Text::from_units(vec![
                            100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118,
                            101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32,
                            105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99,
                            116, 32, 114, 101, 99, 101, 105, 118, 101, 114,
                        ])
                    } else {
                        {
                            let receiver_error = self::read_java_script_ordinary_value_error(
                                heap.clone(),
                                shared_constructor_field1.clone(),
                            );
                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                receiver_error.clone()
                            } else {
                                if ((((!(self::ml_math::is_safe_integer(
                                    shared_constructor_field2,
                                )) || (shared_constructor_field2 < (0f64)))
                                    || (shared_constructor_field2 > (6f64)))
                                    || ((shared_constructor_field2 == (0f64))
                                        && ((1f64 / shared_constructor_field2) < (0f64))))
                                    || (shared_constructor_field4
                                        && (shared_constructor_field2 == (6f64))))
                                {
                                    self::ml_string::Text::from_units(vec![
                                        100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99,
                                        111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101,
                                        101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99,
                                        116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 102, 105,
                                        101, 108, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97,
                                        116, 105, 111, 110,
                                    ])
                                } else {
                                    {
                                        let index = shared_constructor_field2;
                                        {
                                            let source_ml_source2_s4305 = self::source_ml_source1_temporary_read_java_script_descriptor_conversion_state_error_loop227(state.clone(), index);
                                            match source_ml_source2_s4305.clone() {
                                                self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop227Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop227Done => {
                                                    {
                                                        let descriptor = shared_constructor_field3.clone();
                                                        if ((({ let ml_reference_value387 = descriptor.borrow().clone(); ml_reference_value387 }).0).9 && ((self::read_java_script_ordinary_value_error(heap.clone(), (({ let ml_reference_value388 = descriptor.borrow().clone(); ml_reference_value388 }).0).8)) != (self::ml_string::Text::from_units(vec![])))) {
                                                            self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 118, 97, 108, 117, 101, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116])
                                                        } else {
                                                            if ((({ let ml_reference_value389 = descriptor.borrow().clone(); ml_reference_value389 }).0).5 && (((self::read_java_script_ordinary_value_error(heap.clone(), (({ let ml_reference_value390 = descriptor.borrow().clone(); ml_reference_value390 }).0).4)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((({ let ml_reference_value391 = descriptor.borrow().clone(); ml_reference_value391 }).0).4)))) {
                                                                self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 103, 101, 116, 116, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121])
                                                            } else {
                                                                if ((({ let ml_reference_value392 = descriptor.borrow().clone(); ml_reference_value392 }).0).7 && (((self::read_java_script_ordinary_value_error(heap.clone(), (({ let ml_reference_value393 = descriptor.borrow().clone(); ml_reference_value393 }).0).6)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((({ let ml_reference_value394 = descriptor.borrow().clone(); ml_reference_value394 }).0).6)))) {
                                                                    self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 101, 116, 116, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![])
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop227Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop227Return(source_ml_source1_temporary_result227) => {
                                                    source_ml_source1_temporary_result227.clone()
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_descriptor_field(
    state: self::JavaScriptDescriptorConversionState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptDescriptorConversionTransition {
    match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            _,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            if (((shared_constructor_field2 == (4f64)) || (shared_constructor_field2 == (5f64)))
                && !(self::test_java_script_semantic_accessor(value.clone())))
            {
                self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
                    self::ml_string::Text::from_units(vec![
                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 99, 99,
                        101, 115, 115, 111, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97,
                        108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105,
                        110, 101, 100,
                    ]),
                )
            } else {
                {
                    let before = shared_constructor_field3.clone();
                    {
                        let enumerable = (shared_constructor_field2 == (0f64));
                        {
                            let configurable = (shared_constructor_field2 == (1f64));
                            {
                                let data_value = (shared_constructor_field2 == (2f64));
                                {
                                    let writable = (shared_constructor_field2 == (3f64));
                                    {
                                        let getter = (shared_constructor_field2 == (4f64));
                                        {
                                            let setter = !((((enumerable || configurable)
                                                || data_value)
                                                || writable)
                                                || getter);
                                            {
                                                let flag =
                                                    if ((enumerable || configurable) || writable) {
                                                        self::convert_java_script_semantic_boolean(
                                                            value.clone(),
                                                        )
                                                    } else {
                                                        false
                                                    };
                                                {
                                                    let descriptor = std::rc::Rc::new(
                                                        std::cell::RefCell::new({
                                                            let source_ml_source2_spread4322 = {
                                                                let ml_reference_value395 =
                                                                    before.borrow().clone();
                                                                ml_reference_value395
                                                            };
                                                            {
                                                                let source_ml_source2_spread_field4323 =
                                                                    ((source_ml_source2_spread4322
                                                                        .clone())
                                                                    .0)
                                                                        .0;
                                                                {
                                                                    let source_ml_source2_spread_field4324 = ((source_ml_source2_spread4322.clone()).0).1;
                                                                    {
                                                                        let source_ml_source2_spread_field4325 = ((source_ml_source2_spread4322.clone()).0).2;
                                                                        {
                                                                            let source_ml_source2_spread_field4326 = ((source_ml_source2_spread4322.clone()).0).3;
                                                                            {
                                                                                let source_ml_source2_spread_field4327 = ((source_ml_source2_spread4322.clone()).0).4;
                                                                                {
                                                                                    let source_ml_source2_spread_field4328 = ((source_ml_source2_spread4322.clone()).0).5;
                                                                                    {
                                                                                        let source_ml_source2_spread_field4329 = ((source_ml_source2_spread4322.clone()).0).6;
                                                                                        {
                                                                                            let source_ml_source2_spread_field4330 = ((source_ml_source2_spread4322.clone()).0).7;
                                                                                            {
                                                                                                let source_ml_source2_spread_field4331 = ((source_ml_source2_spread4322.clone()).0).8;
                                                                                                {
                                                                                                    let source_ml_source2_spread_field4332 = ((source_ml_source2_spread4322.clone()).0).9;
                                                                                                    {
                                                                                                        let source_ml_source2_spread_field4333 = ((source_ml_source2_spread4322.clone()).0).10;
                                                                                                        {
                                                                                                            let source_ml_source2_spread_field4334 = ((source_ml_source2_spread4322.clone()).0).11;
                                                                                                            {
                                                                                                                let source_ml_source2_field4335 = (enumerable || (({ let ml_reference_value396 = before.borrow().clone(); ml_reference_value396 }).0).3);
                                                                                                                {
                                                                                                                    let source_ml_source2_field4336 = if enumerable {
                                                                                                                    flag
                                                                                                                } else {
                                                                                                                    (({ let ml_reference_value397 = before.borrow().clone(); ml_reference_value397 }).0).2
                                                                                                                };
                                                                                                                    {
                                                                                                                        let source_ml_source2_field4337 = (configurable || (({ let ml_reference_value398 = before.borrow().clone(); ml_reference_value398 }).0).1);
                                                                                                                        {
                                                                                                                            let source_ml_source2_field4338 = if configurable {
                                                                                                                            flag
                                                                                                                        } else {
                                                                                                                            (({ let ml_reference_value399 = before.borrow().clone(); ml_reference_value399 }).0).0
                                                                                                                        };
                                                                                                                            {
                                                                                                                                let source_ml_source2_field4339 = (data_value || (({ let ml_reference_value400 = before.borrow().clone(); ml_reference_value400 }).0).9);
                                                                                                                                {
                                                                                                                                    let source_ml_source2_field4340 = if data_value {
                                                                                                                                    value.clone()
                                                                                                                                } else {
                                                                                                                                    (({ let ml_reference_value401 = before.borrow().clone(); ml_reference_value401 }).0).8
                                                                                                                                };
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_field4341 = (writable || (({ let ml_reference_value402 = before.borrow().clone(); ml_reference_value402 }).0).11);
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_field4342 = if writable {
                                                                                                                                            flag
                                                                                                                                        } else {
                                                                                                                                            (({ let ml_reference_value403 = before.borrow().clone(); ml_reference_value403 }).0).10
                                                                                                                                        };
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_field4343 = (getter || (({ let ml_reference_value404 = before.borrow().clone(); ml_reference_value404 }).0).5);
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_field4344 = if getter {
                                                                                                                                                    value.clone()
                                                                                                                                                } else {
                                                                                                                                                    (({ let ml_reference_value405 = before.borrow().clone(); ml_reference_value405 }).0).4
                                                                                                                                                };
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_field4345 = (setter || (({ let ml_reference_value406 = before.borrow().clone(); ml_reference_value406 }).0).7);
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_field4346 = if setter {
                                                                                                                                                            value.clone()
                                                                                                                                                        } else {
                                                                                                                                                            (({ let ml_reference_value407 = before.borrow().clone(); ml_reference_value407 }).0).6
                                                                                                                                                        };
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source3_record_field410 = source_ml_source2_field4338;
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source3_record_field411 = source_ml_source2_field4337;
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source3_record_field412 = source_ml_source2_field4336;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source3_record_field413 = source_ml_source2_field4335;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source3_record_field414 = source_ml_source2_field4344.clone();
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source3_record_field415 = source_ml_source2_field4343;
                                                                                                                                                                                    {
                                                                                                                                                                                        let source_ml_source3_record_field416 = source_ml_source2_field4346.clone();
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source3_record_field417 = source_ml_source2_field4345;
                                                                                                                                                                                            {
                                                                                                                                                                                                let source_ml_source3_record_field418 = source_ml_source2_field4340.clone();
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_source3_record_field419 = source_ml_source2_field4339;
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source3_record_field420 = source_ml_source2_field4342;
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source3_record_field421 = source_ml_source2_field4341;
                                                                                                                                                                                                            ((source_ml_source3_record_field410, source_ml_source3_record_field411, source_ml_source3_record_field412, source_ml_source3_record_field413, source_ml_source3_record_field414.clone(), source_ml_source3_record_field415, source_ml_source3_record_field416.clone(), source_ml_source3_record_field417, source_ml_source3_record_field418.clone(), source_ml_source3_record_field419, source_ml_source3_record_field420, source_ml_source3_record_field421), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], self::source_ml_source3_record_order::merge_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![self::source_ml_source3_record_order::rebase_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], (source_ml_source2_spread4322.clone()).1.clone()), vec![3f64], vec![2f64], vec![1f64], vec![0f64], vec![9f64], vec![8f64], vec![11f64], vec![10f64], vec![5f64], vec![4f64], vec![7f64], vec![6f64]])))
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
                                                        }),
                                                    );
                                                    self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(Box::new(self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(shared_constructor_field1.clone()), (shared_constructor_field2 + 1f64), descriptor.clone(), false)))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_descriptor_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptDescriptorConversionState,
) -> self::JavaScriptDescriptorConversionTransition {
    {
        let error =
            self::read_java_script_descriptor_conversion_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
        } else {
            if match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value4349,
                ) => source_ml_source2_data_property_value4349,
            } {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(
                    self::ml_string::Text::from_units(vec![
                        97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 100, 101, 115, 99, 114, 105,
                        112, 116, 111, 114, 32, 71, 101, 116, 32, 109, 117, 115, 116, 32, 98, 101,
                        32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 98, 101, 102, 111, 114,
                        101, 32, 105, 116, 115, 32, 110, 101, 120, 116, 32, 115, 116, 101, 112,
                    ]),
                )
            } else {
                {
                    let current = state.clone();
                    {
                        let source_ml_source2_s4350 = self::source_ml_source1_temporary_step_java_script_descriptor_conversion_loop228(heap.clone(), current.clone());
                        match source_ml_source2_s4350.clone() {
                            self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Done(current_2) => {
                                let current_2 = *current_2;
                                match current_2.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, shared_constructor_field1, _) => {
                                        if ((self::read_java_script_property_descriptor_error(shared_constructor_field1.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 97, 110, 110, 111, 116, 32, 99, 111, 110, 116, 97, 105, 110, 32, 98, 111, 116, 104, 32, 100, 97, 116, 97, 32, 97, 110, 100, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115]))
                                        } else {
                                            self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(shared_constructor_field1.clone())
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop228Return(source_ml_source1_temporary_result228) => {
                                let source_ml_source1_temporary_result228 = *source_ml_source1_temporary_result228;
                                source_ml_source1_temporary_result228.clone()
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
pub fn begin_java_script_descriptor_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    receiver: self::JavaScriptSemanticValue,
) -> self::JavaScriptDescriptorConversionTransition {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(heap_error.clone())
        } else {
            {
                let error =
                    self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
                } else {
                    match receiver.clone() {
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            self::step_java_script_descriptor_conversion(
                                heap.clone(),
                                self::JavaScriptDescriptorConversionState::DescriptorConversion(
                                    Box::new(receiver.clone()),
                                    0f64,
                                    self::create_java_script_empty_descriptor(),
                                    false,
                                ),
                            )
                        }
                        _ => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101,
                                    115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114,
                                    99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32,
                                    79, 98, 106, 101, 99, 116,
                                ]),
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_descriptor_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptDescriptorConversionState,
    completion: self::JavaScriptSemanticCompletion,
) -> self::JavaScriptDescriptorConversionTransition {
    {
        let error =
            self::read_java_script_descriptor_conversion_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
        } else {
            if !(match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value4356,
                ) => source_ml_source2_data_property_value4356,
            }) {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(
                    self::ml_string::Text::from_units(vec![
                        100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 114, 101, 115, 117,
                        109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116,
                        115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103,
                        32, 71, 101, 116,
                    ]),
                )
            } else {
                {
                    let value_error = self::read_java_script_ordinary_value_error(
                        heap.clone(),
                        match completion.clone() {
                            self::JavaScriptSemanticCompletion::NormalCompletion(
                                source_ml_source2_data_property_value4357,
                            ) => {
                                let source_ml_source2_data_property_value4357 =
                                    *source_ml_source2_data_property_value4357;
                                source_ml_source2_data_property_value4357.clone()
                            }
                            self::JavaScriptSemanticCompletion::ThrowCompletion(
                                source_ml_source2_data_property_value4358,
                            ) => {
                                let source_ml_source2_data_property_value4358 =
                                    *source_ml_source2_data_property_value4358;
                                source_ml_source2_data_property_value4358.clone()
                            }
                        },
                    );
                    if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(
                            value_error.clone(),
                        )
                    } else {
                        match completion.clone() {
                            self::JavaScriptSemanticCompletion::NormalCompletion(value) => {
                                let value = *value;
                                {
                                    let applied = self::apply_java_script_descriptor_field(
                                        state.clone(),
                                        value.clone(),
                                    );
                                    match applied.clone() {
                                        self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(state_2) => {
                                            let state_2 = *state_2;
                                            self::step_java_script_descriptor_conversion(heap.clone(), state_2.clone())
                                        }
                                        _ => {
                                            applied.clone()
                                        }
                                    }
                                }
                            }
                            self::JavaScriptSemanticCompletion::ThrowCompletion(value_2) => {
                                let value_2 = *value_2;
                                self::JavaScriptDescriptorConversionTransition::DescriptorThrow(
                                    Box::new(value_2.clone()),
                                )
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
pub fn test_java_script_array_transform_operation(operation: &self::ml_string::Text) -> bool {
    self::test_java_script_standard_array_transform_operation(operation)
}
