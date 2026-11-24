// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::create_java_script_ordinary_set_descriptor;
use super::define_java_script_ordinary_property;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::locate_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_source3_record_order;
use super::test_java_script_array_length_key;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_ordinary_set_definition(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
    value: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
    defer_array_length: bool,
) -> std::rc::Rc<
    std::cell::RefCell<(
        (
            self::ml_string::Text,
            std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
            self::JavaScriptSemanticValue,
            Option<(
                Option<
                    std::rc::Rc<
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
                >,
            )>,
            self::ml_string::Text,
            self::JavaScriptOrdinaryHeap,
            self::JavaScriptSemanticValue,
            bool,
        ),
        Vec<f64>,
    )>,
> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
            {
                let key_error =
                    self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
                {
                    let value_error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    {
                        let receiver_error = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            receiver.clone(),
                        );
                        {
                            let error = if (heap_error
                                != (self::ml_string::Text::from_units(vec![])))
                            {
                                heap_error.clone()
                            } else {
                                if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                    key_error.clone()
                                } else {
                                    if (value_error != (self::ml_string::Text::from_units(vec![])))
                                    {
                                        value_error.clone()
                                    } else {
                                        if (receiver_error
                                            != (self::ml_string::Text::from_units(vec![])))
                                        {
                                            receiver_error.clone()
                                        } else {
                                            if ((self::find_java_script_ordinary_object(
                                                shared_constructor_field1.clone(),
                                                identity.clone(),
                                            )) < (0f64))
                                            {
                                                self::ml_string::Text::from_units(vec![
                                                    97, 110, 32, 111, 114, 100, 105, 110, 97, 114,
                                                    121, 32, 83, 101, 116, 32, 110, 101, 101, 100,
                                                    115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                                    97, 108, 32, 79, 98, 106, 101, 99, 116, 32,
                                                    105, 100, 101, 110, 116, 105, 116, 121,
                                                ])
                                            } else {
                                                self::ml_string::Text::from_units(vec![])
                                            }
                                        }
                                    }
                                }
                            };
                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_field2629 =
                                        self::JavaScriptSemanticValue::Undefined;
                                    {
                                        let source_ml_source2_field2631 =
                                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                                self::JavaScriptSemanticValue,
                                            >::new(
                                            )));
                                        {
                                            let source_ml_source3_record_field238 =
                                                self::ml_string::Text::from_units(vec![
                                                    101, 114, 114, 111, 114,
                                                ]);
                                            {
                                                let source_ml_source3_record_field239 =
                                                    source_ml_source2_field2631.clone();
                                                {
                                                    let source_ml_source3_record_field240 =
                                                        source_ml_source2_field2629.clone();
                                                    {
                                                        let source_ml_source3_record_field241 = None::<(Option<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>,)>;
                                                        {
                                                            let source_ml_source3_record_field242 =
                                                                error.clone();
                                                            {
                                                                let source_ml_source3_record_field243 =
                                                                    heap.clone();
                                                                {
                                                                    let source_ml_source3_record_field244 =
                                                                        receiver.clone();
                                                                    {
                                                                        let source_ml_source3_record_field245 =
                                                                            false;
                                                                        ((source_ml_source3_record_field238.clone(), source_ml_source3_record_field239.clone(), source_ml_source3_record_field240.clone(), source_ml_source3_record_field241.clone(), source_ml_source3_record_field242.clone(), source_ml_source3_record_field243.clone(), source_ml_source3_record_field244.clone(), source_ml_source3_record_field245), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![97f64, 99f64, 116f64, 105f64, 111f64, 110f64], vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![99f64, 97f64, 108f64, 108f64, 101f64, 101f64], vec![100f64, 101f64, 102f64, 105f64, 110f64, 105f64, 116f64, 105f64, 111f64, 110f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![5f64, 0f64, 4f64, 7f64, 2f64, 6f64, 1f64], vec![true, true, true, match source_ml_source3_record_field241.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true, true, true]))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }))
                            } else {
                                {
                                    let target = {
                                        let source_ml_source2_array_read_value2634 =
                                            shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_array_read_index2635 =
                                                self::find_java_script_ordinary_object(
                                                    shared_constructor_field1.clone(),
                                                    identity.clone(),
                                                );
                                            {
                                                let ml_reference_array249 =
                                                    source_ml_source2_array_read_value2634.clone();
                                                let ml_reference_position249 =
                                                    source_ml_source2_array_read_index2635;
                                                let ml_reference_projection249 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array249.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position249,
                                                        ),
                                                    );
                                                ml_reference_projection249
                                            }
                                        }
                                    };
                                    if (((((((((!(match (target.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && match target.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match target.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match target.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match target.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match target.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match target.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match target.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match target.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match target.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) {
                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source2_field2711 = (((match target.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, source_ml_source2_data_property_value2707) => {
                                                        let source_ml_source2_data_property_value2707 = *source_ml_source2_data_property_value2707;
                                                        Some((source_ml_source2_data_property_value2707.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, source_ml_source2_data_property_value2708) => {
                                                        let source_ml_source2_data_property_value2708 = *source_ml_source2_data_property_value2708;
                                                        Some((source_ml_source2_data_property_value2708.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, source_ml_source2_data_property_value2709) => {
                                                        let source_ml_source2_data_property_value2709 = *source_ml_source2_data_property_value2709;
                                                        Some((source_ml_source2_data_property_value2709.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, source_ml_source2_data_property_value2710) => {
                                                        let source_ml_source2_data_property_value2710 = *source_ml_source2_data_property_value2710;
                                                        Some((source_ml_source2_data_property_value2710.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(self::JavaScriptSemanticValue,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone();
                                                {
                                                    let source_ml_source2_field2712 = self::JavaScriptSemanticValue::Object(identity.clone(), false);
                                                    {
                                                        let source_ml_source2_field2713 = std::rc::Rc::new(std::cell::RefCell::new(vec![key.clone(), value.clone(), receiver.clone()]));
                                                        {
                                                            let source_ml_source3_record_field246 = self::ml_string::Text::from_units(vec![104, 111, 115, 116, 32, 99, 97, 108, 108]);
                                                            {
                                                                let source_ml_source3_record_field247 = source_ml_source2_field2713.clone();
                                                                {
                                                                    let source_ml_source3_record_field248 = source_ml_source2_field2711.clone();
                                                                    {
                                                                        let source_ml_source3_record_field249 = None::<(Option<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>,)>;
                                                                        {
                                                                            let source_ml_source3_record_field250 = self::ml_string::Text::from_units(vec![]);
                                                                            {
                                                                                let source_ml_source3_record_field251 = heap.clone();
                                                                                {
                                                                                    let source_ml_source3_record_field252 = source_ml_source2_field2712.clone();
                                                                                    {
                                                                                        let source_ml_source3_record_field253 = false;
                                                                                        ((source_ml_source3_record_field246.clone(), source_ml_source3_record_field247.clone(), source_ml_source3_record_field248.clone(), source_ml_source3_record_field249.clone(), source_ml_source3_record_field250.clone(), source_ml_source3_record_field251.clone(), source_ml_source3_record_field252.clone(), source_ml_source3_record_field253), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![97f64, 99f64, 116f64, 105f64, 111f64, 110f64], vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![99f64, 97f64, 108f64, 108f64, 101f64, 101f64], vec![100f64, 101f64, 102f64, 105f64, 110f64, 105f64, 116f64, 105f64, 111f64, 110f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![5f64, 0f64, 4f64, 7f64, 2f64, 6f64, 1f64], vec![true, true, true, match source_ml_source3_record_field249.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true, true, true]))
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
                                            }))
                                        } else {
                                            {
                                                let located = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                                                {
                                                    let own = if (({ let ml_reference_value250 = located.borrow().clone(); ml_reference_value250 }).0).1 {
                                                        (({ let ml_reference_value251 = located.borrow().clone(); ml_reference_value251 }).0).0
                                                    } else {
                                                        self::create_java_script_ordinary_set_descriptor(self::JavaScriptSemanticValue::Undefined, true)
                                                    };
                                                    if (({ let ml_reference_value252 = own.borrow().clone(); ml_reference_value252 }).0).5 {
                                                        {
                                                            let setter = (({ let ml_reference_value253 = own.borrow().clone(); ml_reference_value253 }).0).6;
                                                            match setter.clone() {
                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source2_field2718 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field2720 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                            {
                                                                                let source_ml_source3_record_field254 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                                {
                                                                                    let source_ml_source3_record_field255 = source_ml_source2_field2720.clone();
                                                                                    {
                                                                                        let source_ml_source3_record_field256 = source_ml_source2_field2718.clone();
                                                                                        {
                                                                                            let source_ml_source3_record_field257 = None::<(Option<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>,)>;
                                                                                            {
                                                                                                let source_ml_source3_record_field258 = self::ml_string::Text::from_units(vec![]);
                                                                                                {
                                                                                                    let source_ml_source3_record_field259 = heap.clone();
                                                                                                    {
                                                                                                        let source_ml_source3_record_field260 = receiver.clone();
                                                                                                        {
                                                                                                            let source_ml_source3_record_field261 = false;
                                                                                                            ((source_ml_source3_record_field254.clone(), source_ml_source3_record_field255.clone(), source_ml_source3_record_field256.clone(), source_ml_source3_record_field257.clone(), source_ml_source3_record_field258.clone(), source_ml_source3_record_field259.clone(), source_ml_source3_record_field260.clone(), source_ml_source3_record_field261), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![97f64, 99f64, 116f64, 105f64, 111f64, 110f64], vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![99f64, 97f64, 108f64, 108f64, 101f64, 101f64], vec![100f64, 101f64, 102f64, 105f64, 110f64, 105f64, 116f64, 105f64, 111f64, 110f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![5f64, 0f64, 4f64, 7f64, 2f64, 6f64, 1f64], vec![true, true, true, match source_ml_source3_record_field257.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true, true, true]))
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }))
                                                                }
                                                                _ => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source2_field2727 = std::rc::Rc::new(std::cell::RefCell::new(vec![value.clone()]));
                                                                        {
                                                                            let source_ml_source3_record_field262 = self::ml_string::Text::from_units(vec![99, 97, 108, 108]);
                                                                            {
                                                                                let source_ml_source3_record_field263 = source_ml_source2_field2727.clone();
                                                                                {
                                                                                    let source_ml_source3_record_field264 = setter.clone();
                                                                                    {
                                                                                        let source_ml_source3_record_field265 = None::<(Option<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>,)>;
                                                                                        {
                                                                                            let source_ml_source3_record_field266 = self::ml_string::Text::from_units(vec![]);
                                                                                            {
                                                                                                let source_ml_source3_record_field267 = heap.clone();
                                                                                                {
                                                                                                    let source_ml_source3_record_field268 = receiver.clone();
                                                                                                    {
                                                                                                        let source_ml_source3_record_field269 = false;
                                                                                                        ((source_ml_source3_record_field262.clone(), source_ml_source3_record_field263.clone(), source_ml_source3_record_field264.clone(), source_ml_source3_record_field265.clone(), source_ml_source3_record_field266.clone(), source_ml_source3_record_field267.clone(), source_ml_source3_record_field268.clone(), source_ml_source3_record_field269), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![97f64, 99f64, 116f64, 105f64, 111f64, 110f64], vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![99f64, 97f64, 108f64, 108f64, 101f64, 101f64], vec![100f64, 101f64, 102f64, 105f64, 110f64, 105f64, 116f64, 105f64, 111f64, 110f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![5f64, 0f64, 4f64, 7f64, 2f64, 6f64, 1f64], vec![true, true, true, match source_ml_source3_record_field265.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true, true, true]))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if !((({ let ml_reference_value254 = own.borrow().clone(); ml_reference_value254 }).0).10) {
                                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                                let source_ml_source2_field2732 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source2_field2734 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                    {
                                                                        let source_ml_source3_record_field270 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                        {
                                                                            let source_ml_source3_record_field271 = source_ml_source2_field2734.clone();
                                                                            {
                                                                                let source_ml_source3_record_field272 = source_ml_source2_field2732.clone();
                                                                                {
                                                                                    let source_ml_source3_record_field273 = None::<(Option<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>,)>;
                                                                                    {
                                                                                        let source_ml_source3_record_field274 = self::ml_string::Text::from_units(vec![]);
                                                                                        {
                                                                                            let source_ml_source3_record_field275 = heap.clone();
                                                                                            {
                                                                                                let source_ml_source3_record_field276 = receiver.clone();
                                                                                                {
                                                                                                    let source_ml_source3_record_field277 = false;
                                                                                                    ((source_ml_source3_record_field270.clone(), source_ml_source3_record_field271.clone(), source_ml_source3_record_field272.clone(), source_ml_source3_record_field273.clone(), source_ml_source3_record_field274.clone(), source_ml_source3_record_field275.clone(), source_ml_source3_record_field276.clone(), source_ml_source3_record_field277), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![97f64, 99f64, 116f64, 105f64, 111f64, 110f64], vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![99f64, 97f64, 108f64, 108f64, 101f64, 101f64], vec![100f64, 101f64, 102f64, 105f64, 110f64, 105f64, 116f64, 105f64, 111f64, 110f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![5f64, 0f64, 4f64, 7f64, 2f64, 6f64, 1f64], vec![true, true, true, match source_ml_source3_record_field273.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true, true, true]))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }))
                                                        } else {
                                                            match receiver.clone() {
                                                                self::JavaScriptSemanticValue::Object(identity_2, _) => {
                                                                    {
                                                                        let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity_2.clone());
                                                                        {
                                                                            let object = {
                                                                                let source_ml_source2_array_read_value2738 = shared_constructor_field1.clone();
                                                                                { let ml_reference_array255 = source_ml_source2_array_read_value2738.clone(); let ml_reference_position255 = index; let ml_reference_projection255 = self::ml_array::optional_at(&*ml_reference_array255.borrow(), self::ml_array::number_index(ml_reference_position255)); ml_reference_projection255 }
                                                                            };
                                                                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                                    let source_ml_source2_field2744 = self::JavaScriptSemanticValue::Undefined;
                                                                                    {
                                                                                        let source_ml_source2_field2746 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                        {
                                                                                            let source_ml_source3_record_field278 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                                                                            {
                                                                                                let source_ml_source3_record_field279 = source_ml_source2_field2746.clone();
                                                                                                {
                                                                                                    let source_ml_source3_record_field280 = source_ml_source2_field2744.clone();
                                                                                                    {
                                                                                                        let source_ml_source3_record_field281 = None::<(Option<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>,)>;
                                                                                                        {
                                                                                                            let source_ml_source3_record_field282 = self::ml_string::Text::from_units(vec![83, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 79, 98, 106, 101, 99, 116]);
                                                                                                            {
                                                                                                                let source_ml_source3_record_field283 = heap.clone();
                                                                                                                {
                                                                                                                    let source_ml_source3_record_field284 = receiver.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source3_record_field285 = false;
                                                                                                                        ((source_ml_source3_record_field278.clone(), source_ml_source3_record_field279.clone(), source_ml_source3_record_field280.clone(), source_ml_source3_record_field281.clone(), source_ml_source3_record_field282.clone(), source_ml_source3_record_field283.clone(), source_ml_source3_record_field284.clone(), source_ml_source3_record_field285), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![97f64, 99f64, 116f64, 105f64, 111f64, 110f64], vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![99f64, 97f64, 108f64, 108f64, 101f64, 101f64], vec![100f64, 101f64, 102f64, 105f64, 110f64, 105f64, 116f64, 105f64, 111f64, 110f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![5f64, 0f64, 4f64, 7f64, 2f64, 6f64, 1f64], vec![true, true, true, match source_ml_source3_record_field281.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true, true, true]))
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }))
                                                                            } else {
                                                                                if ((((((((match object.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                                        false
                                                                                    }
                                                                                    _ => {
                                                                                        true
                                                                                    }
                                                                                } && match object.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                                                        false
                                                                                    }
                                                                                    _ => {
                                                                                        true
                                                                                    }
                                                                                }) && match object.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                                        false
                                                                                    }
                                                                                    _ => {
                                                                                        true
                                                                                    }
                                                                                }) && match object.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                                                        false
                                                                                    }
                                                                                    _ => {
                                                                                        true
                                                                                    }
                                                                                }) && match object.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                                        false
                                                                                    }
                                                                                    _ => {
                                                                                        true
                                                                                    }
                                                                                }) && match object.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                                                        false
                                                                                    }
                                                                                    _ => {
                                                                                        true
                                                                                    }
                                                                                }) && match object.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                                                        false
                                                                                    }
                                                                                    _ => {
                                                                                        true
                                                                                    }
                                                                                }) && match object.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                                                        false
                                                                                    }
                                                                                    _ => {
                                                                                        true
                                                                                    }
                                                                                }) && match object.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                                        false
                                                                                    }
                                                                                    _ => {
                                                                                        true
                                                                                    }
                                                                                }) {
                                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                                        let source_ml_source2_field2818 = self::JavaScriptSemanticValue::Undefined;
                                                                                        {
                                                                                            let source_ml_source2_field2820 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                            {
                                                                                                let source_ml_source3_record_field286 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                                                                                {
                                                                                                    let source_ml_source3_record_field287 = source_ml_source2_field2820.clone();
                                                                                                    {
                                                                                                        let source_ml_source3_record_field288 = source_ml_source2_field2818.clone();
                                                                                                        {
                                                                                                            let source_ml_source3_record_field289 = None::<(Option<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>,)>;
                                                                                                            {
                                                                                                                let source_ml_source3_record_field290 = self::ml_string::Text::from_units(vec![97, 32, 98, 111, 114, 114, 111, 119, 101, 100, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 97, 110, 32, 111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 112, 114, 111, 116, 111, 99, 111, 108]);
                                                                                                                {
                                                                                                                    let source_ml_source3_record_field291 = heap.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source3_record_field292 = receiver.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source3_record_field293 = false;
                                                                                                                            ((source_ml_source3_record_field286.clone(), source_ml_source3_record_field287.clone(), source_ml_source3_record_field288.clone(), source_ml_source3_record_field289.clone(), source_ml_source3_record_field290.clone(), source_ml_source3_record_field291.clone(), source_ml_source3_record_field292.clone(), source_ml_source3_record_field293), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![97f64, 99f64, 116f64, 105f64, 111f64, 110f64], vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![99f64, 97f64, 108f64, 108f64, 101f64, 101f64], vec![100f64, 101f64, 102f64, 105f64, 110f64, 105f64, 116f64, 105f64, 111f64, 110f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![5f64, 0f64, 4f64, 7f64, 2f64, 6f64, 1f64], vec![true, true, true, match source_ml_source3_record_field289.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true, true, true]))
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }))
                                                                                } else {
                                                                                    {
                                                                                        let at = self::find_java_script_ordinary_property((((match object.clone().expect("a proven present value") {
                                                                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value2821, _, _, _, _, _) => {
                                                                                                Some((source_ml_source2_data_property_value2821.clone(),))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value2822, _, _, _) => {
                                                                                                Some((source_ml_source2_data_property_value2822.clone(),))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value2823) => {
                                                                                                Some((source_ml_source2_data_property_value2823.clone(),))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value2824, _, _, _, _, _, _) => {
                                                                                                Some((source_ml_source2_data_property_value2824.clone(),))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value2825, _) => {
                                                                                                Some((source_ml_source2_data_property_value2825.clone(),))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value2826, _) => {
                                                                                                Some((source_ml_source2_data_property_value2826.clone(),))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value2827, _) => {
                                                                                                Some((source_ml_source2_data_property_value2827.clone(),))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value2828, _) => {
                                                                                                Some((source_ml_source2_data_property_value2828.clone(),))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value2829) => {
                                                                                                Some((source_ml_source2_data_property_value2829.clone(),))
                                                                                            }
                                                                                            _ => {
                                                                                                None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,)>
                                                                                            }
                                                                                        }).clone().expect("a proven present value")).0).clone(), key.clone());
                                                                                        {
                                                                                            let property = {
                                                                                                let source_ml_source2_array_read_value2839 = (((match object.clone().expect("a proven present value") {
                                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value2830, _, _, _, _, _) => {
                                                                                                        Some((source_ml_source2_data_property_value2830.clone(),))
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value2831, _, _, _) => {
                                                                                                        Some((source_ml_source2_data_property_value2831.clone(),))
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value2832) => {
                                                                                                        Some((source_ml_source2_data_property_value2832.clone(),))
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value2833, _, _, _, _, _, _) => {
                                                                                                        Some((source_ml_source2_data_property_value2833.clone(),))
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value2834, _) => {
                                                                                                        Some((source_ml_source2_data_property_value2834.clone(),))
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value2835, _) => {
                                                                                                        Some((source_ml_source2_data_property_value2835.clone(),))
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value2836, _) => {
                                                                                                        Some((source_ml_source2_data_property_value2836.clone(),))
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value2837, _) => {
                                                                                                        Some((source_ml_source2_data_property_value2837.clone(),))
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value2838) => {
                                                                                                        Some((source_ml_source2_data_property_value2838.clone(),))
                                                                                                    }
                                                                                                    _ => {
                                                                                                        None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,)>
                                                                                                    }
                                                                                                }).clone().expect("a proven present value")).0).clone();
                                                                                                { let ml_reference_array256 = source_ml_source2_array_read_value2839.clone(); let ml_reference_position256 = at; let ml_reference_projection256 = self::ml_array::optional_at(&*ml_reference_array256.borrow(), self::ml_array::number_index(ml_reference_position256)); ml_reference_projection256 }
                                                                                            };
                                                                                            if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (!((({ let ml_reference_value257 = (match property.clone().expect("a proven present value") {
                                                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value2841) => {
                                                                                                    source_ml_source2_data_property_value2841.clone()
                                                                                                }
                                                                                            }).borrow().clone(); ml_reference_value257 }).0).9) || !((({ let ml_reference_value258 = (match property.clone().expect("a proven present value") {
                                                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value2842) => {
                                                                                                    source_ml_source2_data_property_value2842.clone()
                                                                                                }
                                                                                            }).borrow().clone(); ml_reference_value258 }).0).10))) {
                                                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                    let source_ml_source2_field2847 = self::JavaScriptSemanticValue::Undefined;
                                                                                                    {
                                                                                                        let source_ml_source2_field2849 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                                        {
                                                                                                            let source_ml_source3_record_field294 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                                                            {
                                                                                                                let source_ml_source3_record_field295 = source_ml_source2_field2849.clone();
                                                                                                                {
                                                                                                                    let source_ml_source3_record_field296 = source_ml_source2_field2847.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source3_record_field297 = None::<(Option<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>,)>;
                                                                                                                        {
                                                                                                                            let source_ml_source3_record_field298 = self::ml_string::Text::from_units(vec![]);
                                                                                                                            {
                                                                                                                                let source_ml_source3_record_field299 = heap.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source3_record_field300 = receiver.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source3_record_field301 = false;
                                                                                                                                        ((source_ml_source3_record_field294.clone(), source_ml_source3_record_field295.clone(), source_ml_source3_record_field296.clone(), source_ml_source3_record_field297.clone(), source_ml_source3_record_field298.clone(), source_ml_source3_record_field299.clone(), source_ml_source3_record_field300.clone(), source_ml_source3_record_field301), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![97f64, 99f64, 116f64, 105f64, 111f64, 110f64], vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![99f64, 97f64, 108f64, 108f64, 101f64, 101f64], vec![100f64, 101f64, 102f64, 105f64, 110f64, 105f64, 116f64, 105f64, 111f64, 110f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![5f64, 0f64, 4f64, 7f64, 2f64, 6f64, 1f64], vec![true, true, true, match source_ml_source3_record_field297.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true, true, true]))
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }))
                                                                                            } else {
                                                                                                {
                                                                                                    let proposed = self::create_java_script_ordinary_set_descriptor(value.clone(), match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false });
                                                                                                    if ((defer_array_length && match object.clone().expect("a proven present value") {
                                                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                                                            true
                                                                                                        }
                                                                                                        _ => {
                                                                                                            false
                                                                                                        }
                                                                                                    }) && self::test_java_script_array_length_key(key.clone())) {
                                                                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                            let source_ml_source2_field2859 = self::JavaScriptSemanticValue::Undefined;
                                                                                                            {
                                                                                                                let source_ml_source2_field2861 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                                                {
                                                                                                                    let source_ml_source2_field2862 = Some((Some(proposed.clone()),));
                                                                                                                    {
                                                                                                                        let source_ml_source3_record_field302 = self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]);
                                                                                                                        {
                                                                                                                            let source_ml_source3_record_field303 = source_ml_source2_field2861.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source3_record_field304 = source_ml_source2_field2859.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source3_record_field305 = source_ml_source2_field2862.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source3_record_field306 = self::ml_string::Text::from_units(vec![]);
                                                                                                                                        {
                                                                                                                                            let source_ml_source3_record_field307 = heap.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source3_record_field308 = receiver.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source3_record_field309 = false;
                                                                                                                                                    ((source_ml_source3_record_field302.clone(), source_ml_source3_record_field303.clone(), source_ml_source3_record_field304.clone(), source_ml_source3_record_field305.clone(), source_ml_source3_record_field306.clone(), source_ml_source3_record_field307.clone(), source_ml_source3_record_field308.clone(), source_ml_source3_record_field309), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![97f64, 99f64, 116f64, 105f64, 111f64, 110f64], vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![99f64, 97f64, 108f64, 108f64, 101f64, 101f64], vec![100f64, 101f64, 102f64, 105f64, 110f64, 105f64, 116f64, 105f64, 111f64, 110f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![5f64, 0f64, 4f64, 7f64, 2f64, 6f64, 1f64, 3f64], vec![true, true, true, match source_ml_source3_record_field305.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true, true, true]))
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
                                                                                                        }))
                                                                                                    } else {
                                                                                                        {
                                                                                                            let transition = self::define_java_script_ordinary_property(heap.clone(), identity_2.clone(), key.clone(), proposed.clone());
                                                                                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                let source_ml_source2_field2863 = (({ let ml_reference_value259 = transition.borrow().clone(); ml_reference_value259 }).0).1;
                                                                                                                {
                                                                                                                    let source_ml_source2_field2864 = if (((({ let ml_reference_value260 = transition.borrow().clone(); ml_reference_value260 }).0).0) == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                        self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101])
                                                                                                                    } else {
                                                                                                                        self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_field2865 = (({ let ml_reference_value261 = transition.borrow().clone(); ml_reference_value261 }).0).0;
                                                                                                                        {
                                                                                                                            let source_ml_source2_field2866 = (({ let ml_reference_value262 = transition.borrow().clone(); ml_reference_value262 }).0).3;
                                                                                                                            {
                                                                                                                                let source_ml_source2_field2867 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                {
                                                                                                                                    let source_ml_source2_field2869 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                                                                    {
                                                                                                                                        let source_ml_source3_record_field310 = source_ml_source2_field2864.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source3_record_field311 = source_ml_source2_field2869.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source3_record_field312 = source_ml_source2_field2867.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source3_record_field313 = None::<(Option<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>,)>;
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source3_record_field314 = source_ml_source2_field2865.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source3_record_field315 = source_ml_source2_field2863.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source3_record_field316 = receiver.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source3_record_field317 = source_ml_source2_field2866;
                                                                                                                                                                    ((source_ml_source3_record_field310.clone(), source_ml_source3_record_field311.clone(), source_ml_source3_record_field312.clone(), source_ml_source3_record_field313.clone(), source_ml_source3_record_field314.clone(), source_ml_source3_record_field315.clone(), source_ml_source3_record_field316.clone(), source_ml_source3_record_field317), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![97f64, 99f64, 116f64, 105f64, 111f64, 110f64], vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![99f64, 97f64, 108f64, 108f64, 101f64, 101f64], vec![100f64, 101f64, 102f64, 105f64, 110f64, 105f64, 116f64, 105f64, 111f64, 110f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![5f64, 0f64, 4f64, 7f64, 2f64, 6f64, 1f64], vec![true, true, true, match source_ml_source3_record_field313.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true, true, true]))
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
                                                                _ => {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source2_field2874 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_field2876 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                            {
                                                                                let source_ml_source3_record_field318 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                                {
                                                                                    let source_ml_source3_record_field319 = source_ml_source2_field2876.clone();
                                                                                    {
                                                                                        let source_ml_source3_record_field320 = source_ml_source2_field2874.clone();
                                                                                        {
                                                                                            let source_ml_source3_record_field321 = None::<(Option<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>,)>;
                                                                                            {
                                                                                                let source_ml_source3_record_field322 = self::ml_string::Text::from_units(vec![]);
                                                                                                {
                                                                                                    let source_ml_source3_record_field323 = heap.clone();
                                                                                                    {
                                                                                                        let source_ml_source3_record_field324 = receiver.clone();
                                                                                                        {
                                                                                                            let source_ml_source3_record_field325 = false;
                                                                                                            ((source_ml_source3_record_field318.clone(), source_ml_source3_record_field319.clone(), source_ml_source3_record_field320.clone(), source_ml_source3_record_field321.clone(), source_ml_source3_record_field322.clone(), source_ml_source3_record_field323.clone(), source_ml_source3_record_field324.clone(), source_ml_source3_record_field325), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![97f64, 99f64, 116f64, 105f64, 111f64, 110f64], vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![99f64, 97f64, 108f64, 108f64, 101f64, 101f64], vec![100f64, 101f64, 102f64, 105f64, 110f64, 105f64, 116f64, 105f64, 111f64, 110f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64]], vec![5f64, 0f64, 4f64, 7f64, 2f64, 6f64, 1f64], vec![true, true, true, match source_ml_source3_record_field321.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true, true, true]))
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
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
                }
            }
        }
    }
}
