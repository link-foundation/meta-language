// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryInsertJavaScriptOrdinaryObjectJoin128;
use super::MlSource1TemporaryReadJavaScriptOrdinaryOwnKeysLoop125Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_source1_temporary_insert_java_script_ordinary_object_loop129;
use super::source_ml_source1_temporary_read_java_script_ordinary_own_keys_loop125;
use super::source_ml_source1_temporary_register_java_script_ordinary_symbol_loop131;
use super::source_ml_source3_record_order;
use super::test_java_script_ordinary_symbol;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_own_keys(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
) -> std::rc::Rc<
    std::cell::RefCell<(
        (
            self::ml_string::Text,
            std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
        ),
        Vec<f64>,
    )>,
> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source2_field2877 =
                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                            self::JavaScriptSemanticValue,
                        >::new()));
                    {
                        let source_ml_source3_record_field326 = error.clone();
                        {
                            let source_ml_source3_record_field327 =
                                source_ml_source2_field2877.clone();
                            ((source_ml_source3_record_field326.clone(), source_ml_source3_record_field327.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![107f64, 101f64, 121f64, 115f64]], vec![1f64, 0f64]))
                        }
                    }
                }))
            } else {
                {
                    let index = self::find_java_script_ordinary_object(
                        shared_constructor_field1.clone(),
                        identity.clone(),
                    );
                    {
                        let object = {
                            let source_ml_source2_array_read_value2881 =
                                shared_constructor_field1.clone();
                            {
                                let ml_reference_array263 =
                                    source_ml_source2_array_read_value2881.clone();
                                let ml_reference_position263 = index;
                                let ml_reference_projection263 = self::ml_array::optional_at(
                                    &*ml_reference_array263.borrow(),
                                    self::ml_array::number_index(ml_reference_position263),
                                );
                                ml_reference_projection263
                            }
                        };
                        if match (object.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_field2883 =
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                        self::JavaScriptSemanticValue,
                                    >::new(
                                    )));
                                {
                                    let source_ml_source3_record_field328 =
                                        self::ml_string::Text::from_units(vec![
                                            79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121,
                                            75, 101, 121, 115, 32, 110, 101, 101, 100, 115, 32,
                                            105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79,
                                            98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116,
                                            105, 116, 121,
                                        ]);
                                    {
                                        let source_ml_source3_record_field329 =
                                            source_ml_source2_field2883.clone();
                                        ((source_ml_source3_record_field328.clone(), source_ml_source3_record_field329.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![107f64, 101f64, 121f64, 115f64]], vec![1f64, 0f64]))
                                    }
                                }
                            }))
                        } else {
                            if ((((((((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => false,
                                _ => true,
                            } && match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => false,
                                _ => true,
                            }) && match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => false,
                                _ => true,
                            }) && match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                    false
                                }
                                _ => true,
                            }) && match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                    false
                                }
                                _ => true,
                            }) && match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                    false
                                }
                                _ => true,
                            }) && match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                    false
                                }
                                _ => true,
                            }) && match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => false,
                                _ => true,
                            }) && match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => false,
                                _ => true,
                            }) {
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_field2952 =
                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                            self::JavaScriptSemanticValue,
                                        >::new(
                                        )));
                                    {
                                        let source_ml_source3_record_field330 =
                                            self::ml_string::Text::from_units(vec![
                                                111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111,
                                                114, 32, 111, 119, 110, 32, 107, 101, 121, 115, 32,
                                                114, 101, 113, 117, 105, 114, 101, 32, 116, 104,
                                                101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32,
                                                104, 111, 115, 116, 32, 112, 114, 111, 116, 111,
                                                99, 111, 108,
                                            ]);
                                        {
                                            let source_ml_source3_record_field331 =
                                                source_ml_source2_field2952.clone();
                                            ((source_ml_source3_record_field330.clone(), source_ml_source3_record_field331.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![107f64, 101f64, 121f64, 115f64]], vec![1f64, 0f64]))
                                        }
                                    }
                                }))
                            } else {
                                {
                                    let numeric = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                        self::JavaScriptSemanticValue,
                                    >::new(
                                    )));
                                    {
                                        let positions = std::rc::Rc::new(std::cell::RefCell::new(
                                            Vec::<f64>::new(),
                                        ));
                                        {
                                            let text =
                                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                                    self::JavaScriptSemanticValue,
                                                >::new(
                                                )));
                                            {
                                                let symbols =
                                                    std::rc::Rc::new(std::cell::RefCell::new(
                                                        Vec::<self::JavaScriptSemanticValue>::new(),
                                                    ));
                                                {
                                                    let source_ml_source1_temporary_values124 = (((match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value2954, _, _, _, _, _) => {
                                                                Some((source_ml_source2_data_property_value2954.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value2955, _, _, _) => {
                                                                Some((source_ml_source2_data_property_value2955.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value2956) => {
                                                                Some((source_ml_source2_data_property_value2956.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value2957, _, _, _, _, _, _) => {
                                                                Some((source_ml_source2_data_property_value2957.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value2958, _) => {
                                                                Some((source_ml_source2_data_property_value2958.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value2959, _) => {
                                                                Some((source_ml_source2_data_property_value2959.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value2960, _) => {
                                                                Some((source_ml_source2_data_property_value2960.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value2961, _) => {
                                                                Some((source_ml_source2_data_property_value2961.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value2962) => {
                                                                Some((source_ml_source2_data_property_value2962.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone();
                                                    {
                                                        let source_ml_source1_temporary_index124 =
                                                            self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source2_s2963 = self::source_ml_source1_temporary_read_java_script_ordinary_own_keys_loop125(numeric.clone(), positions.clone(), text.clone(), symbols.clone(), source_ml_source1_temporary_values124.clone(), source_ml_source1_temporary_index124.clone());
                                                            match source_ml_source2_s2963.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryOwnKeysLoop125Result::MlSource1TemporaryReadJavaScriptOrdinaryOwnKeysLoop125Done(numeric_2, text_2, symbols_2) => {
                                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                                            let source_ml_source2_field2967 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), { let ml_reference_value264 = numeric_2.borrow().clone(); ml_reference_value264 }), { let ml_reference_value265 = text_2.borrow().clone(); ml_reference_value265 }), { let ml_reference_value266 = symbols_2.borrow().clone(); ml_reference_value266 })));
                                                                            {
                                                                                let source_ml_source3_record_field332 = self::ml_string::Text::from_units(vec![]);
                                                                                {
                                                                                    let source_ml_source3_record_field333 = source_ml_source2_field2967.clone();
                                                                                    ((source_ml_source3_record_field332.clone(), source_ml_source3_record_field333.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![107f64, 101f64, 121f64, 115f64]], vec![1f64, 0f64]))
                                                                                }
                                                                            }
                                                                        }))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn insert_java_script_ordinary_object(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    prototype: self::JavaScriptSemanticValue,
) -> std::rc::Rc<
    std::cell::RefCell<(
        (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool),
        Vec<f64>,
    )>,
> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
        ) => {
            let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source3_record_field334 = heap_error.clone();
                    {
                        let source_ml_source3_record_field335 = heap.clone();
                        {
                            let source_ml_source3_record_field336 = false;
                            ((source_ml_source3_record_field334.clone(), source_ml_source3_record_field335.clone(), source_ml_source3_record_field336), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                        }
                    }
                }))
            } else {
                if (({
                    let ml_reference_array267 = shared_constructor_field1.clone();
                    let ml_reference_projection267 = ml_reference_array267.borrow().len() as f64;
                    ml_reference_projection267
                }) >= (4096f64))
                {
                    std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source3_record_field337 =
                            self::ml_string::Text::from_units(vec![
                                111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32,
                                105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99,
                                116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105,
                                116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101,
                                115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116,
                            ]);
                        {
                            let source_ml_source3_record_field338 = heap.clone();
                            {
                                let source_ml_source3_record_field339 = false;
                                ((source_ml_source3_record_field337.clone(), source_ml_source3_record_field338.clone(), source_ml_source3_record_field339), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                            }
                        }
                    }))
                } else {
                    if ((((identity <= (self::ml::Big::from_i128(0)))
                        || (identity > (self::ml::Big::from_i128(18446744073709551615))))
                        || ((self::find_java_script_ordinary_object(
                            shared_constructor_field1.clone(),
                            identity.clone(),
                        )) >= (0f64)))
                        || self::test_java_script_ordinary_symbol(
                            shared_constructor_field2.clone(),
                            identity.clone(),
                        ))
                    {
                        std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source3_record_field340 =
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 102, 114, 101, 115, 104, 32, 111, 114, 100, 105, 110,
                                    97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 113,
                                    117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 100, 105, 115,
                                    116, 105, 110, 99, 116, 32, 101, 120, 116, 101, 114, 110, 97,
                                    108, 108, 121, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100,
                                    32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103,
                                    108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116,
                                    121,
                                ]);
                            {
                                let source_ml_source3_record_field341 = heap.clone();
                                {
                                    let source_ml_source3_record_field342 = false;
                                    ((source_ml_source3_record_field340.clone(), source_ml_source3_record_field341.clone(), source_ml_source3_record_field342), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                }
                            }
                        }))
                    } else {
                        if (match prototype.clone() {
                            self::JavaScriptSemanticValue::Null => false,
                            _ => true,
                        } && match prototype.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => false,
                            _ => true,
                        }) {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source3_record_field343 =
                                    self::ml_string::Text::from_units(vec![
                                        97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32,
                                        112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117,
                                        115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111,
                                        114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32,
                                        79, 98, 106, 101, 99, 116,
                                    ]);
                                {
                                    let source_ml_source3_record_field344 = heap.clone();
                                    {
                                        let source_ml_source3_record_field345 = false;
                                        ((source_ml_source3_record_field343.clone(), source_ml_source3_record_field344.clone(), source_ml_source3_record_field345), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                    }
                                }
                            }))
                        } else {
                            {
                                let error = self::read_java_script_ordinary_value_error(
                                    heap.clone(),
                                    prototype.clone(),
                                );
                                if (error != (self::ml_string::Text::from_units(vec![]))) {
                                    std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source3_record_field346 = error.clone();
                                        {
                                            let source_ml_source3_record_field347 = heap.clone();
                                            {
                                                let source_ml_source3_record_field348 = false;
                                                ((source_ml_source3_record_field346.clone(), source_ml_source3_record_field347.clone(), source_ml_source3_record_field348), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                            }
                                        }
                                    }))
                                } else {
                                    {
                                        let source_ml_source2_s3066 = match prototype.clone() {
                                                self::JavaScriptSemanticValue::Object(identity_2, _) => {
                                                    {
                                                        let parent = {
                                                            let source_ml_source2_array_read_value3070 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source2_array_read_index3071 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity_2.clone());
                                                                { let ml_reference_array268 = source_ml_source2_array_read_value3070.clone(); let ml_reference_position268 = source_ml_source2_array_read_index3071; let ml_reference_projection268 = self::ml_array::optional_at(&*ml_reference_array268.borrow(), self::ml_array::number_index(ml_reference_position268)); ml_reference_projection268 }
                                                            }
                                                        };
                                                        if (((((((((!(match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && match parent.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }) && match parent.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }) && match parent.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }) && match parent.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }) && match parent.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }) && match parent.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }) && match parent.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }) && match parent.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }) && match parent.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }) {
                                                            self::MlSource1TemporaryInsertJavaScriptOrdinaryObjectJoin128::MlSource1TemporaryInsertJavaScriptOrdinaryObjectJoin128Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                                let source_ml_source3_record_field349 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 119, 105, 116, 104, 32, 97, 110, 32, 111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 104, 111, 115, 116, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 112, 114, 111, 116, 111, 99, 111, 108]);
                                                                {
                                                                    let source_ml_source3_record_field350 = heap.clone();
                                                                    {
                                                                        let source_ml_source3_record_field351 = false;
                                                                        ((source_ml_source3_record_field349.clone(), source_ml_source3_record_field350.clone(), source_ml_source3_record_field351), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                                                    }
                                                                }
                                                            })))
                                                        } else {
                                                            self::MlSource1TemporaryInsertJavaScriptOrdinaryObjectJoin128::MlSource1TemporaryInsertJavaScriptOrdinaryObjectJoin128Next
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    self::MlSource1TemporaryInsertJavaScriptOrdinaryObjectJoin128::MlSource1TemporaryInsertJavaScriptOrdinaryObjectJoin128Next
                                                }
                                            };
                                        match source_ml_source2_s3066.clone() {
                                                self::MlSource1TemporaryInsertJavaScriptOrdinaryObjectJoin128::MlSource1TemporaryInsertJavaScriptOrdinaryObjectJoin128Next => {
                                                    {
                                                        let at = 0f64;
                                                        {
                                                            let source_ml_source1_temporary_values127 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source1_temporary_index127 = self::ml::Big::from_i128(0);
                                                                {
                                                                    let at_2 = self::source_ml_source1_temporary_insert_java_script_ordinary_object_loop129(identity.clone(), at, source_ml_source1_temporary_values127.clone(), source_ml_source1_temporary_index127.clone());
                                                                    {
                                                                        let object = {
                                                                            let source_ml_source2_constructor_field3147 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(identity.clone(), Box::new(prototype.clone()), true, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, false, source_ml_source2_constructor_field3147.clone())
                                                                        };
                                                                        {
                                                                            let objects = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                let source_ml_source2_concat_value3152 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value269 = shared_constructor_field1.borrow().clone(); ml_reference_value269 }, 0f64, at_2)));
                                                                                {
                                                                                    let source_ml_source2_concat_value3153 = std::rc::Rc::new(std::cell::RefCell::new(vec![object.clone()]));
                                                                                    {
                                                                                        let source_ml_source2_concat_value3154 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value270 = shared_constructor_field1.borrow().clone(); ml_reference_value270 }, at_2, f64::INFINITY)));
                                                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), { let ml_reference_value271 = source_ml_source2_concat_value3152.borrow().clone(); ml_reference_value271 }), { let ml_reference_value272 = source_ml_source2_concat_value3153.borrow().clone(); ml_reference_value272 }), { let ml_reference_value273 = source_ml_source2_concat_value3154.borrow().clone(); ml_reference_value273 })
                                                                                    }
                                                                                }
                                                                            }));
                                                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                                                let source_ml_source2_field3169 = {
                                                                                    let source_ml_source2_nominal_field3156 = shared_constructor_field1.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field3158 = shared_constructor_field2.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field3160 = shared_constructor_field3.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field3162 = shared_constructor_field4.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field3164 = shared_constructor_field5.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field3166 = shared_constructor_field6.clone();
                                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(objects.clone(), source_ml_source2_nominal_field3158.clone(), source_ml_source2_nominal_field3160.clone(), source_ml_source2_nominal_field3162.clone(), source_ml_source2_nominal_field3164.clone(), source_ml_source2_nominal_field3166.clone())
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source3_record_field352 = self::ml_string::Text::from_units(vec![]);
                                                                                    {
                                                                                        let source_ml_source3_record_field353 = source_ml_source2_field3169.clone();
                                                                                        {
                                                                                            let source_ml_source3_record_field354 = true;
                                                                                            ((source_ml_source3_record_field352.clone(), source_ml_source3_record_field353.clone(), source_ml_source3_record_field354), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryInsertJavaScriptOrdinaryObjectJoin128::MlSource1TemporaryInsertJavaScriptOrdinaryObjectJoin128Return(source_ml_source1_temporary_result129) => {
                                                    source_ml_source1_temporary_result129.clone()
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
pub fn register_java_script_ordinary_symbol(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
) -> std::rc::Rc<
    std::cell::RefCell<(
        (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool),
        Vec<f64>,
    )>,
> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
        ) => {
            let error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source3_record_field355 = error.clone();
                    {
                        let source_ml_source3_record_field356 = heap.clone();
                        {
                            let source_ml_source3_record_field357 = false;
                            ((source_ml_source3_record_field355.clone(), source_ml_source3_record_field356.clone(), source_ml_source3_record_field357), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                        }
                    }
                }))
            } else {
                if (({
                    let ml_reference_array274 = shared_constructor_field2.clone();
                    let ml_reference_projection274 = ml_reference_array274.borrow().len() as f64;
                    ml_reference_projection274
                }) >= (16384f64))
                {
                    std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source3_record_field358 =
                            self::ml_string::Text::from_units(vec![
                                111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32,
                                105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99,
                                116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105,
                                116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101,
                                115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116,
                            ]);
                        {
                            let source_ml_source3_record_field359 = heap.clone();
                            {
                                let source_ml_source3_record_field360 = false;
                                ((source_ml_source3_record_field358.clone(), source_ml_source3_record_field359.clone(), source_ml_source3_record_field360), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                            }
                        }
                    }))
                } else {
                    if ((((identity <= (self::ml::Big::from_i128(0)))
                        || (identity > (self::ml::Big::from_i128(18446744073709551615))))
                        || ((self::find_java_script_ordinary_object(
                            shared_constructor_field1.clone(),
                            identity.clone(),
                        )) >= (0f64)))
                        || self::test_java_script_ordinary_symbol(
                            shared_constructor_field2.clone(),
                            identity.clone(),
                        ))
                    {
                        std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source3_record_field361 =
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 102, 114, 101, 115, 104, 32, 83, 121, 109, 98, 111,
                                    108, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116,
                                    115, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 101, 120,
                                    116, 101, 114, 110, 97, 108, 108, 121, 32, 97, 108, 108, 111,
                                    99, 97, 116, 101, 100, 32, 117, 110, 115, 105, 103, 110, 101,
                                    100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101,
                                    110, 116, 105, 116, 121,
                                ]);
                            {
                                let source_ml_source3_record_field362 = heap.clone();
                                {
                                    let source_ml_source3_record_field363 = false;
                                    ((source_ml_source3_record_field361.clone(), source_ml_source3_record_field362.clone(), source_ml_source3_record_field363), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                }
                            }
                        }))
                    } else {
                        {
                            let at = 0f64;
                            {
                                let source_ml_source1_temporary_values130 =
                                    shared_constructor_field2.clone();
                                {
                                    let source_ml_source1_temporary_index130 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let at_2 = self::source_ml_source1_temporary_register_java_script_ordinary_symbol_loop131(identity.clone(), at, source_ml_source1_temporary_values130.clone(), source_ml_source1_temporary_index130.clone());
                                        {
                                            let symbols = std::rc::Rc::new(
                                                std::cell::RefCell::new({
                                                    let source_ml_source2_concat_value3191 =
                                                        std::rc::Rc::new(std::cell::RefCell::new(
                                                            self::ml_array::slice(
                                                                &{
                                                                    let ml_reference_value275 =
                                                                        shared_constructor_field2
                                                                            .borrow()
                                                                            .clone();
                                                                    ml_reference_value275
                                                                },
                                                                0f64,
                                                                at_2,
                                                            ),
                                                        ));
                                                    {
                                                        let source_ml_source2_concat_value3192 =
                                                            std::rc::Rc::new(
                                                                std::cell::RefCell::new(vec![
                                                                    identity.clone(),
                                                                ]),
                                                            );
                                                        {
                                                            let source_ml_source2_concat_value3193 =
                                                                std::rc::Rc::new(
                                                                    std::cell::RefCell::new(
                                                                        self::ml_array::slice(
                                                                            &{
                                                                                let ml_reference_value276 = shared_constructor_field2.borrow().clone();
                                                                                ml_reference_value276
                                                                            },
                                                                            at_2,
                                                                            f64::INFINITY,
                                                                        ),
                                                                    ),
                                                                );
                                                            self::ml_array::append(
                                                                self::ml_array::append(
                                                                    self::ml_array::append(
                                                                        Vec::<self::ml::Big>::new(),
                                                                        {
                                                                            let ml_reference_value277 = source_ml_source2_concat_value3191.borrow().clone();
                                                                            ml_reference_value277
                                                                        },
                                                                    ),
                                                                    {
                                                                        let ml_reference_value278 = source_ml_source2_concat_value3192.borrow().clone();
                                                                        ml_reference_value278
                                                                    },
                                                                ),
                                                                {
                                                                    let ml_reference_value279 = source_ml_source2_concat_value3193.borrow().clone();
                                                                    ml_reference_value279
                                                                },
                                                            )
                                                        }
                                                    }
                                                }),
                                            );
                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source2_field3208 = {
                                                    let source_ml_source2_nominal_field3195 =
                                                        shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_nominal_field3197 =
                                                            shared_constructor_field2.clone();
                                                        {
                                                            let source_ml_source2_nominal_field3199 =
                                                                shared_constructor_field3.clone();
                                                            {
                                                                let source_ml_source2_nominal_field3201 =
                                                                    shared_constructor_field4
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field3203 =
                                                                        shared_constructor_field5
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field3205 = shared_constructor_field6.clone();
                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_nominal_field3195.clone(), symbols.clone(), source_ml_source2_nominal_field3199.clone(), source_ml_source2_nominal_field3201.clone(), source_ml_source2_nominal_field3203.clone(), source_ml_source2_nominal_field3205.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                {
                                                    let source_ml_source3_record_field364 =
                                                        self::ml_string::Text::from_units(vec![]);
                                                    {
                                                        let source_ml_source3_record_field365 =
                                                            source_ml_source2_field3208.clone();
                                                        {
                                                            let source_ml_source3_record_field366 =
                                                                true;
                                                            ((source_ml_source3_record_field364.clone(), source_ml_source3_record_field365.clone(), source_ml_source3_record_field366), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![1f64, 2f64, 0f64]))
                                                        }
                                                    }
                                                }
                                            }))
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
pub fn test_java_script_standard_array_creation_operation(
    operation: &self::ml_string::Text,
) -> bool {
    (((*operation)
        == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 111, 102])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                65, 114, 114, 97, 121, 46, 102, 114, 111, 109,
            ]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_standard_collection_for_each_operation(
    operation: &self::ml_string::Text,
) -> bool {
    (((*operation)
        == (self::ml_string::Text::from_units(vec![
            77, 97, 112, 46, 102, 111, 114, 69, 97, 99, 104,
        ])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                83, 101, 116, 46, 102, 111, 114, 69, 97, 99, 104,
            ]))))
}
