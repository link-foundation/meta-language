// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCollectionEntry;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71;
use super::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72;
use super::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73;
use super::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result;
use super::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63;
use super::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result;
use super::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Result;
use super::ml;
use super::ml_string;
use super::order_java_script_collection_keys;
use super::read_java_script_ordinary_value_error;
use super::source_ml_source1_temporary_read_java_script_ordinary_object_descriptor_error_loop75;
use super::source_ml_source1_temporary_read_java_script_ordinary_prototype_chain_error_loop76;
use super::source_ml_source1_temporary_read_java_script_strong_collection_object_error_loop65;
use super::source_ml_source1_temporary_read_java_script_strong_collection_object_error_loop66;
use super::test_complete_java_script_property_descriptor;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_strong_collection_object_error(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
    budget: std::rc::Rc<std::cell::RefCell<((f64, f64, f64, f64, f64), Vec<f64>)>>,
) -> self::ml_string::Text {
    {
        let source_ml_source2_s1224 = if (match object.clone() {
            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => true,
            _ => false,
        } || match object.clone() {
            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => true,
            _ => false,
        }) {
            {
                let entries = (((match object.clone() {
                    self::JavaScriptOrdinaryObject::MapObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1237,
                    ) => Some((source_ml_source2_data_property_value1237.clone(),)),
                    self::JavaScriptOrdinaryObject::SetObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1238,
                    ) => Some((source_ml_source2_data_property_value1238.clone(),)),
                    _ => {
                        None::<(
                            std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>,
                        )>
                    }
                })
                .clone()
                .expect("a proven present value"))
                .0)
                    .clone();
                if ((((match object.clone() {
                    self::JavaScriptOrdinaryObject::MapObject(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1239,
                        _,
                        _,
                    ) => Some((source_ml_source2_data_property_value1239,)),
                    self::JavaScriptOrdinaryObject::SetObject(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1240,
                        _,
                        _,
                    ) => Some((source_ml_source2_data_property_value1240,)),
                    _ => None::<(bool,)>,
                })
                .clone()
                .expect("a proven present value"))
                .0 || match (entries.clone(), None::<()>) {
                    (_ml_left, Some(_ml_right)) => false,
                    _ => false,
                }) || (({
                    let ml_reference_array194 = entries.clone();
                    let ml_reference_projection194 = ml_reference_array194.borrow().len() as f64;
                    ml_reference_projection194
                }) > (65536f64)))
                {
                    self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 111, 98, 106, 101, 99, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_read_java_script_strong_collection_object_error_written64 = {
                            let ml_field_owner196 = budget.clone();
                            let ml_field_previous196 = ml_field_owner196.borrow().0.1;
                            let ml_field_value196 = {
                                let ml_reference_array196 = entries.clone();
                                let ml_reference_projection196 =
                                    ml_reference_array196.borrow().len() as f64;
                                ml_reference_projection196
                            };
                            ml_field_owner196.borrow_mut().0.1 =
                                ml_field_previous196 + ml_field_value196;
                            ()
                        };
                        if (((({
                            let ml_reference_value197 = budget.borrow().clone();
                            ml_reference_value197
                        })
                        .0)
                            .1)
                            > (1048576f64))
                        {
                            self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 110, 116, 114, 121, 32, 104, 105, 115, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 104, 101, 97, 112, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            {
                                let keys = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                    self::JavaScriptSemanticValue,
                                >::new(
                                )));
                                {
                                    let source_ml_source1_temporary_index62 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s1241 = self::source_ml_source1_temporary_read_java_script_strong_collection_object_error_loop65(heap.clone(), object.clone(), budget.clone(), keys.clone(), entries.clone(), source_ml_source1_temporary_index62.clone());
                                        match source_ml_source2_s1241.clone() {
                                            self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Done => {
                                                {
                                                    let ordered_keys = self::order_java_script_collection_keys(keys.clone());
                                                    {
                                                        let at = 1f64;
                                                        {
                                                            let source_ml_source2_s1242 = self::source_ml_source1_temporary_read_java_script_strong_collection_object_error_loop66(ordered_keys.clone(), at);
                                                            match source_ml_source2_s1242.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Result::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Done => {
                                                                    self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Next
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Result::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Return(source_ml_source1_temporary_result66) => {
                                                                    self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Return(source_ml_source1_temporary_result66.clone())
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return(source_ml_source1_temporary_result66_2) => {
                                                self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Return(source_ml_source1_temporary_result66_2.clone())
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
            self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Next
        };
        match source_ml_source2_s1224.clone() {
            self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Return(source_ml_source1_temporary_result63) => {
                source_ml_source1_temporary_result63.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_descriptor_value_error(
    heap: self::JavaScriptOrdinaryHeap,
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
) -> self::ml_string::Text {
    if !(self::test_complete_java_script_property_descriptor(descriptor.clone())) {
        self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 114,
            101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118,
            97, 108, 105, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99,
            114, 105, 112, 116, 111, 114, 115,
        ])
    } else {
        {
            let source_ml_source2_s1243 = if (({
                let ml_reference_value198 = descriptor.borrow().clone();
                ml_reference_value198
            })
            .0)
                .9
            {
                {
                    let error = self::read_java_script_ordinary_value_error(
                        heap.clone(),
                        (({
                            let ml_reference_value199 = descriptor.borrow().clone();
                            ml_reference_value199
                        })
                        .0)
                            .8,
                    );
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71Return(error.clone())
                    } else {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71Next
                    }
                }
            } else {
                self::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71Next
            };
            match source_ml_source2_s1243.clone() {
                self::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71Next => {
                    {
                        let source_ml_source2_s1244 = if (({ let ml_reference_value200 = descriptor.borrow().clone(); ml_reference_value200 }).0).5 {
                            {
                                let error_2 = self::read_java_script_ordinary_value_error(heap.clone(), (({ let ml_reference_value201 = descriptor.borrow().clone(); ml_reference_value201 }).0).4);
                                if (error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72Return(error_2.clone())
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72Next
                                }
                            }
                        } else {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72Next
                        };
                        match source_ml_source2_s1244.clone() {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72Next => {
                                {
                                    let source_ml_source2_s1245 = if (({ let ml_reference_value202 = descriptor.borrow().clone(); ml_reference_value202 }).0).7 {
                                        {
                                            let error_3 = self::read_java_script_ordinary_value_error(heap.clone(), (({ let ml_reference_value203 = descriptor.borrow().clone(); ml_reference_value203 }).0).6);
                                            if (error_3 != (self::ml_string::Text::from_units(vec![]))) {
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73Return(error_3.clone())
                                            } else {
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73Next
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73Next
                                    };
                                    match source_ml_source2_s1245.clone() {
                                        self::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73Next => {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                                        self::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73Return(source_ml_source1_temporary_result73) => {
                                            source_ml_source1_temporary_result73.clone()
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72Return(source_ml_source1_temporary_result73_2) => {
                                source_ml_source1_temporary_result73_2.clone()
                            }
                        }
                    }
                }
                self::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71::MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71Return(source_ml_source1_temporary_result73_3) => {
                    source_ml_source1_temporary_result73_3.clone()
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_object_descriptor_error(
    heap: self::JavaScriptOrdinaryHeap,
    prototype: self::JavaScriptSemanticValue,
    properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    budget: std::rc::Rc<std::cell::RefCell<((f64, f64, f64, f64, f64), Vec<f64>)>>,
) -> self::ml_string::Text {
    if (match prototype.clone() {
        self::JavaScriptSemanticValue::Null => false,
        _ => true,
    } && match prototype.clone() {
        self::JavaScriptSemanticValue::Object(_, _) => false,
        _ => true,
    }) {
        self::ml_string::Text::from_units(vec![
            97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116,
            121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111,
            114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116,
        ])
    } else {
        {
            let prototype_error =
                self::read_java_script_ordinary_value_error(heap.clone(), prototype.clone());
            if (prototype_error != (self::ml_string::Text::from_units(vec![]))) {
                prototype_error.clone()
            } else {
                {
                    let source_ml_source1_temporary_read_java_script_ordinary_object_descriptor_error_written74 = {
                        let ml_field_owner205 = budget.clone();
                        let ml_field_previous205 = ml_field_owner205.borrow().0.4;
                        let ml_field_value205 = {
                            let ml_reference_array205 = properties.clone();
                            let ml_reference_projection205 =
                                ml_reference_array205.borrow().len() as f64;
                            ml_reference_projection205
                        };
                        ml_field_owner205.borrow_mut().0.4 =
                            ml_field_previous205 + ml_field_value205;
                        ()
                    };
                    if ((((({
                        let ml_reference_value206 = budget.borrow().clone();
                        ml_reference_value206
                    })
                    .0)
                        .4)
                        > (16384f64))
                        || (({
                            let ml_reference_array207 = properties.clone();
                            let ml_reference_projection207 =
                                ml_reference_array207.borrow().len() as f64;
                            ml_reference_projection207
                        }) > (4096f64)))
                    {
                        self::ml_string::Text::from_units(vec![
                            111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101,
                            114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104,
                            101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114,
                            100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98,
                            117, 100, 103, 101, 116,
                        ])
                    } else {
                        {
                            let at = 0f64;
                            {
                                let source_ml_source2_s1248 = self::source_ml_source1_temporary_read_java_script_ordinary_object_descriptor_error_loop75(heap.clone(), properties.clone(), at);
                                match source_ml_source2_s1248.clone() {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Done => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Return(source_ml_source1_temporary_result75) => {
                                        source_ml_source1_temporary_result75.clone()
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
pub fn read_java_script_ordinary_prototype_chain_error(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
) -> self::ml_string::Text {
    {
        let prototype = match object.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(
                _,
                source_ml_source2_data_property_value1249,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value1249 =
                    *source_ml_source2_data_property_value1249;
                source_ml_source2_data_property_value1249.clone()
            }
            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                _,
                source_ml_source2_data_property_value1250,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value1250 =
                    *source_ml_source2_data_property_value1250;
                source_ml_source2_data_property_value1250.clone()
            }
            self::JavaScriptOrdinaryObject::ArrayObject(
                _,
                source_ml_source2_data_property_value1251,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value1251 =
                    *source_ml_source2_data_property_value1251;
                source_ml_source2_data_property_value1251.clone()
            }
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                _,
                source_ml_source2_data_property_value1252,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value1252 =
                    *source_ml_source2_data_property_value1252;
                source_ml_source2_data_property_value1252.clone()
            }
            self::JavaScriptOrdinaryObject::MapObject(
                _,
                source_ml_source2_data_property_value1253,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value1253 =
                    *source_ml_source2_data_property_value1253;
                source_ml_source2_data_property_value1253.clone()
            }
            self::JavaScriptOrdinaryObject::SetObject(
                _,
                source_ml_source2_data_property_value1254,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value1254 =
                    *source_ml_source2_data_property_value1254;
                source_ml_source2_data_property_value1254.clone()
            }
            self::JavaScriptOrdinaryObject::WeakMapObject(
                _,
                source_ml_source2_data_property_value1255,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value1255 =
                    *source_ml_source2_data_property_value1255;
                source_ml_source2_data_property_value1255.clone()
            }
            self::JavaScriptOrdinaryObject::WeakSetObject(
                _,
                source_ml_source2_data_property_value1256,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value1256 =
                    *source_ml_source2_data_property_value1256;
                source_ml_source2_data_property_value1256.clone()
            }
            self::JavaScriptOrdinaryObject::OrdinaryObject(
                _,
                source_ml_source2_data_property_value1257,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value1257 =
                    *source_ml_source2_data_property_value1257;
                source_ml_source2_data_property_value1257.clone()
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(
                _,
                source_ml_source2_data_property_value1258,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value1258 =
                    *source_ml_source2_data_property_value1258;
                source_ml_source2_data_property_value1258.clone()
            }
            self::JavaScriptOrdinaryObject::HostURLObject(
                _,
                source_ml_source2_data_property_value1259,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value1259 =
                    *source_ml_source2_data_property_value1259;
                source_ml_source2_data_property_value1259.clone()
            }
            self::JavaScriptOrdinaryObject::HostArrayObject(
                _,
                source_ml_source2_data_property_value1260,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value1260 =
                    *source_ml_source2_data_property_value1260;
                source_ml_source2_data_property_value1260.clone()
            }
            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                _,
                source_ml_source2_data_property_value1261,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value1261 =
                    *source_ml_source2_data_property_value1261;
                source_ml_source2_data_property_value1261.clone()
            }
        };
        {
            let depth = 0f64;
            {
                let source_ml_source2_s1262 = self::source_ml_source1_temporary_read_java_script_ordinary_prototype_chain_error_loop76(heap.clone(), object.clone(), prototype.clone(), depth);
                match source_ml_source2_s1262.clone() {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done => {
                        self::ml_string::Text::from_units(vec![])
                    }
                    self::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Return(source_ml_source1_temporary_result76) => {
                        source_ml_source1_temporary_result76.clone()
                    }
                }
            }
        }
    }
}
