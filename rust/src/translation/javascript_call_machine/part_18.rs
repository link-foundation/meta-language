// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::accepts_java_script_ordinary_own_property_definition;
use super::commit_java_script_ordinary_own_property;
use super::define_java_script_array_own_property;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_source3_record_order;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn define_java_script_ordinary_property(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
    proposed: std::rc::Rc<
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
) -> std::rc::Rc<
    std::cell::RefCell<(
        (
            self::ml_string::Text,
            self::JavaScriptOrdinaryHeap,
            Option<(Option<bool>,)>,
            bool,
            std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
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
                    let error = if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                        heap_error.clone()
                    } else {
                        key_error.clone()
                    };
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_field1700 =
                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                    self::ml_string::Text,
                                >::new(
                                )));
                            {
                                let source_ml_source3_record_field117 = error.clone();
                                {
                                    let source_ml_source3_record_field118 = heap.clone();
                                    {
                                        let source_ml_source3_record_field119 =
                                            None::<(Option<bool>,)>;
                                        {
                                            let source_ml_source3_record_field120 = false;
                                            {
                                                let source_ml_source3_record_field121 =
                                                    source_ml_source2_field1700.clone();
                                                ((source_ml_source3_record_field117.clone(), source_ml_source3_record_field118.clone(), source_ml_source3_record_field119.clone(), source_ml_source3_record_field120, source_ml_source3_record_field121.clone()), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64, 76f64, 105f64, 109f64, 105f64, 116f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64], vec![119f64, 114f64, 105f64, 116f64, 101f64, 115f64]], vec![1f64, 3f64, 0f64, 4f64], vec![true, true, match source_ml_source3_record_field119.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true]))
                                            }
                                        }
                                    }
                                }
                            }
                        }))
                    } else {
                        if ((({
                            let ml_reference_value224 = proposed.borrow().clone();
                            ml_reference_value224
                        })
                        .0)
                            .9
                            && ((self::read_java_script_ordinary_value_error(
                                heap.clone(),
                                (({
                                    let ml_reference_value225 = proposed.borrow().clone();
                                    ml_reference_value225
                                })
                                .0)
                                    .8,
                            )) != (self::ml_string::Text::from_units(vec![]))))
                        {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_field1704 =
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                        self::ml_string::Text,
                                    >::new(
                                    )));
                                {
                                    let source_ml_source3_record_field122 =
                                        self::ml_string::Text::from_units(vec![
                                            112, 114, 111, 112, 111, 115, 101, 100, 32, 112, 114,
                                            111, 112, 101, 114, 116, 121, 32, 118, 97, 108, 117,
                                            101, 115, 32, 109, 117, 115, 116, 32, 98, 101, 108,
                                            111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97,
                                            99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108,
                                            32, 99, 111, 110, 116, 101, 120, 116,
                                        ]);
                                    {
                                        let source_ml_source3_record_field123 = heap.clone();
                                        {
                                            let source_ml_source3_record_field124 =
                                                None::<(Option<bool>,)>;
                                            {
                                                let source_ml_source3_record_field125 = false;
                                                {
                                                    let source_ml_source3_record_field126 =
                                                        source_ml_source2_field1704.clone();
                                                    ((source_ml_source3_record_field122.clone(), source_ml_source3_record_field123.clone(), source_ml_source3_record_field124.clone(), source_ml_source3_record_field125, source_ml_source3_record_field126.clone()), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64, 76f64, 105f64, 109f64, 105f64, 116f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64], vec![119f64, 114f64, 105f64, 116f64, 101f64, 115f64]], vec![1f64, 3f64, 0f64, 4f64], vec![true, true, match source_ml_source3_record_field124.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true]))
                                                }
                                            }
                                        }
                                    }
                                }
                            }))
                        } else {
                            if ((({
                                let ml_reference_value226 = proposed.borrow().clone();
                                ml_reference_value226
                            })
                            .0)
                                .5
                                && ((self::read_java_script_ordinary_value_error(
                                    heap.clone(),
                                    (({
                                        let ml_reference_value227 = proposed.borrow().clone();
                                        ml_reference_value227
                                    })
                                    .0)
                                        .4,
                                )) != (self::ml_string::Text::from_units(vec![]))))
                            {
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_field1708 =
                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                            self::ml_string::Text,
                                        >::new(
                                        )));
                                    {
                                        let source_ml_source3_record_field127 =
                                            self::ml_string::Text::from_units(vec![
                                                112, 114, 111, 112, 111, 115, 101, 100, 32, 103,
                                                101, 116, 116, 101, 114, 115, 32, 109, 117, 115,
                                                116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111,
                                                32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108,
                                                32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110,
                                                116, 101, 120, 116,
                                            ]);
                                        {
                                            let source_ml_source3_record_field128 = heap.clone();
                                            {
                                                let source_ml_source3_record_field129 =
                                                    None::<(Option<bool>,)>;
                                                {
                                                    let source_ml_source3_record_field130 = false;
                                                    {
                                                        let source_ml_source3_record_field131 =
                                                            source_ml_source2_field1708.clone();
                                                        ((source_ml_source3_record_field127.clone(), source_ml_source3_record_field128.clone(), source_ml_source3_record_field129.clone(), source_ml_source3_record_field130, source_ml_source3_record_field131.clone()), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64, 76f64, 105f64, 109f64, 105f64, 116f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64], vec![119f64, 114f64, 105f64, 116f64, 101f64, 115f64]], vec![1f64, 3f64, 0f64, 4f64], vec![true, true, match source_ml_source3_record_field129.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true]))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }))
                            } else {
                                if ((({
                                    let ml_reference_value228 = proposed.borrow().clone();
                                    ml_reference_value228
                                })
                                .0)
                                    .7
                                    && ((self::read_java_script_ordinary_value_error(
                                        heap.clone(),
                                        (({
                                            let ml_reference_value229 = proposed.borrow().clone();
                                            ml_reference_value229
                                        })
                                        .0)
                                            .6,
                                    )) != (self::ml_string::Text::from_units(vec![]))))
                                {
                                    std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_field1712 =
                                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                                self::ml_string::Text,
                                            >::new(
                                            )));
                                        {
                                            let source_ml_source3_record_field132 =
                                                self::ml_string::Text::from_units(vec![
                                                    112, 114, 111, 112, 111, 115, 101, 100, 32,
                                                    115, 101, 116, 116, 101, 114, 115, 32, 109,
                                                    117, 115, 116, 32, 98, 101, 108, 111, 110, 103,
                                                    32, 116, 111, 32, 116, 104, 101, 32, 97, 99,
                                                    116, 117, 97, 108, 32, 103, 108, 111, 98, 97,
                                                    108, 32, 99, 111, 110, 116, 101, 120, 116,
                                                ]);
                                            {
                                                let source_ml_source3_record_field133 =
                                                    heap.clone();
                                                {
                                                    let source_ml_source3_record_field134 =
                                                        None::<(Option<bool>,)>;
                                                    {
                                                        let source_ml_source3_record_field135 =
                                                            false;
                                                        {
                                                            let source_ml_source3_record_field136 =
                                                                source_ml_source2_field1712.clone();
                                                            ((source_ml_source3_record_field132.clone(), source_ml_source3_record_field133.clone(), source_ml_source3_record_field134.clone(), source_ml_source3_record_field135, source_ml_source3_record_field136.clone()), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64, 76f64, 105f64, 109f64, 105f64, 116f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64], vec![119f64, 114f64, 105f64, 116f64, 101f64, 115f64]], vec![1f64, 3f64, 0f64, 4f64], vec![true, true, match source_ml_source3_record_field134.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true]))
                                                        }
                                                    }
                                                }
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
                                                let source_ml_source2_array_read_value1715 =
                                                    shared_constructor_field1.clone();
                                                {
                                                    let ml_reference_array230 =
                                                        source_ml_source2_array_read_value1715
                                                            .clone();
                                                    let ml_reference_position230 = index;
                                                    let ml_reference_projection230 =
                                                        self::ml_array::optional_at(
                                                            &*ml_reference_array230.borrow(),
                                                            self::ml_array::number_index(
                                                                ml_reference_position230,
                                                            ),
                                                        );
                                                    ml_reference_projection230
                                                }
                                            };
                                            if match (object.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } {
                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source2_field1720 =
                                                        std::rc::Rc::new(std::cell::RefCell::new(
                                                            Vec::<self::ml_string::Text>::new(),
                                                        ));
                                                    {
                                                        let source_ml_source3_record_field137 =
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    68, 101, 102, 105, 110, 101,
                                                                    79, 119, 110, 80, 114, 111,
                                                                    112, 101, 114, 116, 121, 32,
                                                                    110, 101, 101, 100, 115, 32,
                                                                    105, 116, 115, 32, 97, 99, 116,
                                                                    117, 97, 108, 32, 79, 98, 106,
                                                                    101, 99, 116, 32, 105, 100,
                                                                    101, 110, 116, 105, 116, 121,
                                                                ],
                                                            );
                                                        {
                                                            let source_ml_source3_record_field138 =
                                                                heap.clone();
                                                            {
                                                                let source_ml_source3_record_field139 =
                                                                    None::<(Option<bool>,)>;
                                                                {
                                                                    let source_ml_source3_record_field140 =
                                                                        false;
                                                                    {
                                                                        let source_ml_source3_record_field141 = source_ml_source2_field1720.clone();
                                                                        ((source_ml_source3_record_field137.clone(), source_ml_source3_record_field138.clone(), source_ml_source3_record_field139.clone(), source_ml_source3_record_field140, source_ml_source3_record_field141.clone()), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64, 76f64, 105f64, 109f64, 105f64, 116f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64], vec![119f64, 114f64, 105f64, 116f64, 101f64, 115f64]], vec![1f64, 3f64, 0f64, 4f64], vec![true, true, match source_ml_source3_record_field139.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true]))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }))
                                            } else {
                                                match object.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                            self::define_java_script_array_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
                                                        }
                                                        _ => {
                                                            if !(self::accepts_java_script_ordinary_own_property_definition(object.clone().expect("a proven present value"))) {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source2_field1729 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                                    {
                                                                        let source_ml_source3_record_field142 = self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]);
                                                                        {
                                                                            let source_ml_source3_record_field143 = heap.clone();
                                                                            {
                                                                                let source_ml_source3_record_field144 = None::<(Option<bool>,)>;
                                                                                {
                                                                                    let source_ml_source3_record_field145 = false;
                                                                                    {
                                                                                        let source_ml_source3_record_field146 = source_ml_source2_field1729.clone();
                                                                                        ((source_ml_source3_record_field142.clone(), source_ml_source3_record_field143.clone(), source_ml_source3_record_field144.clone(), source_ml_source3_record_field145, source_ml_source3_record_field146.clone()), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64, 76f64, 105f64, 109f64, 105f64, 116f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64], vec![119f64, 114f64, 105f64, 116f64, 101f64, 115f64]], vec![1f64, 3f64, 0f64, 4f64], vec![true, true, match source_ml_source3_record_field144.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true]))
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }))
                                                            } else {
                                                                self::commit_java_script_ordinary_own_property(heap.clone(), object.clone().expect("a proven present value"), key.clone(), proposed.clone())
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
