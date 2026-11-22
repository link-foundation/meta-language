// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCollectionCompletion;
use super::JavaScriptCollectionEntry;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryExecuteJavaScriptCollectionOperationLoop425Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::normalize_java_script_collection_key;
use super::read_java_script_collection_operation_kind;
use super::read_java_script_ordinary_heap_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_source1_temporary_execute_java_script_collection_operation_loop425;
use super::test_java_script_collection_key;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn replace_java_script_collection_entries(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
    entries: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>,
) -> self::JavaScriptOrdinaryHeap {
    if (match object.clone() {
        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => false,
        _ => true,
    } && match object.clone() {
        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => false,
        _ => true,
    }) {
        heap.clone()
    } else {
        {
            let identity = (((match object.clone() {
                self::JavaScriptOrdinaryObject::MapObject(
                    source_ml_source2_data_property_value17166,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => Some((source_ml_source2_data_property_value17166.clone(),)),
                self::JavaScriptOrdinaryObject::SetObject(
                    source_ml_source2_data_property_value17167,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => Some((source_ml_source2_data_property_value17167.clone(),)),
                _ => None::<(self::ml::Big,)>,
            })
            .clone()
            .expect("a proven present value"))
            .0)
                .clone();
            {
                let prototype = (((match object.clone() {
                    self::JavaScriptOrdinaryObject::MapObject(
                        _,
                        source_ml_source2_data_property_value17168,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value17168 =
                            *source_ml_source2_data_property_value17168;
                        Some((source_ml_source2_data_property_value17168.clone(),))
                    }
                    self::JavaScriptOrdinaryObject::SetObject(
                        _,
                        source_ml_source2_data_property_value17169,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value17169 =
                            *source_ml_source2_data_property_value17169;
                        Some((source_ml_source2_data_property_value17169.clone(),))
                    }
                    _ => None::<(self::JavaScriptSemanticValue,)>,
                })
                .clone()
                .expect("a proven present value"))
                .0)
                    .clone();
                {
                    let extensible = ((match object.clone() {
                        self::JavaScriptOrdinaryObject::MapObject(
                            _,
                            _,
                            source_ml_source2_data_property_value17170,
                            _,
                            _,
                            _,
                        ) => Some((source_ml_source2_data_property_value17170,)),
                        self::JavaScriptOrdinaryObject::SetObject(
                            _,
                            _,
                            source_ml_source2_data_property_value17171,
                            _,
                            _,
                            _,
                        ) => Some((source_ml_source2_data_property_value17171,)),
                        _ => None::<(bool,)>,
                    })
                    .clone()
                    .expect("a proven present value"))
                    .0;
                    {
                        let callable = ((match object.clone() {
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value17172,
                                _,
                                _,
                            ) => Some((source_ml_source2_data_property_value17172,)),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value17173,
                                _,
                                _,
                            ) => Some((source_ml_source2_data_property_value17173,)),
                            _ => None::<(bool,)>,
                        })
                        .clone()
                        .expect("a proven present value"))
                        .0;
                        {
                            let properties = (((match object.clone() {
                                self::JavaScriptOrdinaryObject::MapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value17174,
                                    _,
                                ) => Some((source_ml_source2_data_property_value17174.clone(),)),
                                self::JavaScriptOrdinaryObject::SetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value17175,
                                    _,
                                ) => Some((source_ml_source2_data_property_value17175.clone(),)),
                                _ => {
                                    None::<(
                                        std::rc::Rc<
                                            std::cell::RefCell<
                                                Vec<self::JavaScriptOrdinaryProperty>,
                                            >,
                                        >,
                                    )>
                                }
                            })
                            .clone()
                            .expect("a proven present value"))
                            .0)
                                .clone();
                            {
                                let replacement = if match object.clone() {
                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                        true
                                    }
                                    _ => false,
                                } {
                                    self::JavaScriptOrdinaryObject::MapObject(
                                        identity.clone(),
                                        Box::new(prototype.clone()),
                                        extensible,
                                        callable,
                                        properties.clone(),
                                        entries.clone(),
                                    )
                                } else {
                                    self::JavaScriptOrdinaryObject::SetObject(
                                        identity.clone(),
                                        Box::new(prototype.clone()),
                                        extensible,
                                        callable,
                                        properties.clone(),
                                        entries.clone(),
                                    )
                                };
                                self::replace_java_script_ordinary_object(
                                    heap.clone(),
                                    replacement.clone(),
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
pub fn read_java_script_collection_entries(
    object: self::JavaScriptOrdinaryObject,
    kind: &self::ml_string::Text,
) -> Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>> {
    match object.clone() {
        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, entries) => {
            if ((*kind) == (self::ml_string::Text::from_units(vec![77, 97, 112]))) {
                Some(entries.clone())
            } else {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>
            }
        }
        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, entries_2) => {
            if ((*kind) == (self::ml_string::Text::from_units(vec![83, 101, 116]))) {
                Some(entries_2.clone())
            } else {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>
            }
        }
        _ => None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>,
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_collection_operation(
    heap: self::JavaScriptOrdinaryHeap,
    receiver: self::JavaScriptSemanticValue,
    operation: &self::ml_string::Text,
    args: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCollectionCompletion {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptCollectionCompletion::CollectionInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else {
                {
                    let kind = self::read_java_script_collection_operation_kind(operation);
                    if (kind == (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCollectionCompletion::CollectionInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 111, 112, 101,
                                114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114,
                                101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108,
                                32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112,
                                101, 114, 97, 116, 105, 111, 110,
                            ]),
                        )
                    } else {
                        if (({
                            let ml_reference_array890 = args.clone();
                            let ml_reference_projection890 =
                                ml_reference_array890.borrow().len() as f64;
                            ml_reference_projection890
                        }) > (4096f64))
                        {
                            self::JavaScriptCollectionCompletion::CollectionResourceLimit(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 97, 114,
                                    103, 117, 109, 101, 110, 116, 115, 32, 101, 120, 99, 101, 101,
                                    100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 101, 99, 117,
                                    116, 105, 111, 110, 32, 98, 117, 100, 103, 101, 116,
                                ]),
                            )
                        } else {
                            {
                                let source_ml_source1_temporary_values424 =
                                    std::rc::Rc::new(std::cell::RefCell::new(
                                        self::ml_array::append(vec![receiver.clone()], {
                                            let ml_reference_value891 = args.borrow().clone();
                                            ml_reference_value891
                                        }),
                                    ));
                                {
                                    let source_ml_source1_temporary_index424 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s17192 = self::source_ml_source1_temporary_execute_java_script_collection_operation_loop425(heap.clone(), source_ml_source1_temporary_values424.clone(), source_ml_source1_temporary_index424.clone());
                                        match source_ml_source2_s17192.clone() {
                                                self::MlSource1TemporaryExecuteJavaScriptCollectionOperationLoop425Result::MlSource1TemporaryExecuteJavaScriptCollectionOperationLoop425Done => {
                                                    {
                                                        let object = match receiver.clone() {
                                                            self::JavaScriptSemanticValue::Object(identity, _) => {
                                                                {
                                                                    let source_ml_source2_array_read_value17196 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index17197 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                                                        { let ml_reference_array892 = source_ml_source2_array_read_value17196.clone(); let ml_reference_position892 = source_ml_source2_array_read_index17197; let ml_reference_projection892 = self::ml_array::optional_at(&*ml_reference_array892.borrow(), self::ml_array::number_index(ml_reference_position892)); ml_reference_projection892 }
                                                                    }
                                                                }
                                                            }
                                                            _ => {
                                                                None::<self::JavaScriptOrdinaryObject>
                                                            }
                                                        };
                                                        if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::JavaScriptCollectionCompletion::CollectionTypeError(Box::new(heap.clone()), operation.concat(&self::ml_string::Text::from_units(vec![32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])))
                                                        } else {
                                                            {
                                                                let stored = self::read_java_script_collection_entries(object.clone().expect("a proven present value"), &(kind));
                                                                if match (stored.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    self::JavaScriptCollectionCompletion::CollectionTypeError(Box::new(heap.clone()), operation.concat(&self::ml_string::Text::from_units(vec![32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])))
                                                                } else {
                                                                    {
                                                                        let key = self::normalize_java_script_collection_key(({
                                                                            let source_ml_source2_array_read_index17199 = 0f64;
                                                                            { let ml_reference_array893 = args.clone(); let ml_reference_position893 = source_ml_source2_array_read_index17199; let ml_reference_projection893 = self::ml_array::optional_at(&*ml_reference_array893.borrow(), self::ml_array::number_index(ml_reference_position893)); ml_reference_projection893 }
                                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                                                        {
                                                                            let at = { let ml_values894 = { let ml_reference_value895 = (stored.clone().expect("a proven present value")).borrow().clone(); ml_reference_value895 }; let ml_callback894 = { let key = key.clone(); std::rc::Rc::new(move |entry: self::JavaScriptCollectionEntry| -> bool { (match entry.clone() {
                                                                                self::JavaScriptCollectionEntry::CollectionEntry(_, _, source_ml_source2_data_property_value17200) => {
                                                                                    source_ml_source2_data_property_value17200
                                                                                }
                                                                            } && self::test_java_script_collection_key(match entry.clone() {
                                                                                self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source2_data_property_value17201, _, _) => {
                                                                                    let source_ml_source2_data_property_value17201 = *source_ml_source2_data_property_value17201;
                                                                                    source_ml_source2_data_property_value17201.clone()
                                                                                }
                                                                            }, key.clone())) }) }; ml_values894.iter().cloned().enumerate().position(|(ml_index894, ml_value894)| ml_callback894(ml_value894)).map_or(-1f64, |ml_position| ml_position as f64) };
                                                                            {
                                                                                let entry = {
                                                                                    let source_ml_source2_array_read_value17202 = stored.clone().expect("a proven present value");
                                                                                    { let ml_reference_array896 = source_ml_source2_array_read_value17202.clone(); let ml_reference_position896 = at; let ml_reference_projection896 = self::ml_array::optional_at(&*ml_reference_array896.borrow(), self::ml_array::number_index(ml_reference_position896)); ml_reference_projection896 }
                                                                                };
                                                                                if ((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 112, 46, 103, 101, 116]))) {
                                                                                    self::JavaScriptCollectionCompletion::CollectionNormal(Box::new(heap.clone()), Box::new(if match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                        self::JavaScriptSemanticValue::Undefined
                                                                                    } else {
                                                                                        match entry.clone().expect("a proven present value") {
                                                                                            self::JavaScriptCollectionEntry::CollectionEntry(_, source_ml_source2_data_property_value17204, _) => {
                                                                                                let source_ml_source2_data_property_value17204 = *source_ml_source2_data_property_value17204;
                                                                                                source_ml_source2_data_property_value17204.clone()
                                                                                            }
                                                                                        }
                                                                                    }))
                                                                                } else {
                                                                                    if (((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 112, 46, 104, 97, 115]))) || ((*operation) == (self::ml_string::Text::from_units(vec![83, 101, 116, 46, 104, 97, 115])))) {
                                                                                        self::JavaScriptCollectionCompletion::CollectionNormal(Box::new(heap.clone()), Box::new(self::JavaScriptSemanticValue::Boolean((at >= (0f64)))))
                                                                                    } else {
                                                                                        if (((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 112, 46, 115, 105, 122, 101]))) || ((*operation) == (self::ml_string::Text::from_units(vec![83, 101, 116, 46, 115, 105, 122, 101])))) {
                                                                                            self::JavaScriptCollectionCompletion::CollectionNormal(Box::new(heap.clone()), Box::new(self::JavaScriptSemanticValue::Number({ let ml_reference_array899 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values897 = { let ml_reference_value898 = (stored.clone().expect("a proven present value")).borrow().clone(); ml_reference_value898 }; let ml_callback897 = std::rc::Rc::new(move |entry_2: self::JavaScriptCollectionEntry| -> bool { match entry_2.clone() {
                                                                                                self::JavaScriptCollectionEntry::CollectionEntry(_, _, source_ml_source2_data_property_value17205) => {
                                                                                                    source_ml_source2_data_property_value17205
                                                                                                }
                                                                                            } }); ml_values897.iter().cloned().enumerate().filter_map(|(ml_index897, ml_value897)| if ml_callback897(ml_value897.clone()) { Some(ml_value897) } else { None }).collect::<Vec<_>>() })); let ml_reference_projection899 = ml_reference_array899.borrow().len() as f64; ml_reference_projection899 })))
                                                                                        } else {
                                                                                            if (((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 112, 46, 99, 108, 101, 97, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![83, 101, 116, 46, 99, 108, 101, 97, 114])))) {
                                                                                                self::JavaScriptCollectionCompletion::CollectionNormal(Box::new(self::replace_java_script_collection_entries(heap.clone(), object.clone().expect("a proven present value"), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values900 = { let ml_reference_value901 = (stored.clone().expect("a proven present value")).borrow().clone(); ml_reference_value901 }; let ml_callback900 = std::rc::Rc::new(move |entry_2: self::JavaScriptCollectionEntry| -> self::JavaScriptCollectionEntry { {
                                                                                                    let source_ml_source2_nominal_spread17206 = entry_2.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field17207 = match source_ml_source2_nominal_spread17206.clone() {
                                                                                                            self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source2_data_property_value17208, _, _) => {
                                                                                                                let source_ml_source2_data_property_value17208 = *source_ml_source2_data_property_value17208;
                                                                                                                source_ml_source2_data_property_value17208.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field17209 = match source_ml_source2_nominal_spread17206.clone() {
                                                                                                                self::JavaScriptCollectionEntry::CollectionEntry(_, source_ml_source2_data_property_value17210, _) => {
                                                                                                                    let source_ml_source2_data_property_value17210 = *source_ml_source2_data_property_value17210;
                                                                                                                    source_ml_source2_data_property_value17210.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field17211 = match source_ml_source2_nominal_spread17206.clone() {
                                                                                                                    self::JavaScriptCollectionEntry::CollectionEntry(_, _, source_ml_source2_data_property_value17212) => {
                                                                                                                        source_ml_source2_data_property_value17212
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override17213 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override17214 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_override17215 = false;
                                                                                                                            self::JavaScriptCollectionEntry::CollectionEntry(Box::new(source_ml_source2_nominal_override17213.clone()), Box::new(source_ml_source2_nominal_override17214.clone()), source_ml_source2_nominal_override17215)
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                } }); ml_values900.iter().cloned().enumerate().map(|(ml_index900, ml_value900)| ml_callback900(ml_value900)).collect::<Vec<_>>() })))), Box::new(self::JavaScriptSemanticValue::Undefined))
                                                                                            } else {
                                                                                                if (((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 112, 46, 100, 101, 108, 101, 116, 101]))) || ((*operation) == (self::ml_string::Text::from_units(vec![83, 101, 116, 46, 100, 101, 108, 101, 116, 101])))) {
                                                                                                    self::JavaScriptCollectionCompletion::CollectionNormal(Box::new(if (at < (0f64)) {
                                                                                                        heap.clone()
                                                                                                    } else {
                                                                                                        self::replace_java_script_collection_entries(heap.clone(), object.clone().expect("a proven present value"), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values902 = { let ml_reference_value903 = (stored.clone().expect("a proven present value")).borrow().clone(); ml_reference_value903 }; let ml_callback902 = { let at = at; std::rc::Rc::new(move |entry_2: self::JavaScriptCollectionEntry, index: f64| -> self::JavaScriptCollectionEntry { if (index == at) {
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_spread17216 = entry_2.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field17217 = match source_ml_source2_nominal_spread17216.clone() {
                                                                                                                        self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source2_data_property_value17218, _, _) => {
                                                                                                                            let source_ml_source2_data_property_value17218 = *source_ml_source2_data_property_value17218;
                                                                                                                            source_ml_source2_data_property_value17218.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field17219 = match source_ml_source2_nominal_spread17216.clone() {
                                                                                                                            self::JavaScriptCollectionEntry::CollectionEntry(_, source_ml_source2_data_property_value17220, _) => {
                                                                                                                                let source_ml_source2_data_property_value17220 = *source_ml_source2_data_property_value17220;
                                                                                                                                source_ml_source2_data_property_value17220.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field17221 = match source_ml_source2_nominal_spread17216.clone() {
                                                                                                                                self::JavaScriptCollectionEntry::CollectionEntry(_, _, source_ml_source2_data_property_value17222) => {
                                                                                                                                    source_ml_source2_data_property_value17222
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_override17223 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_override17224 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_override17225 = false;
                                                                                                                                        self::JavaScriptCollectionEntry::CollectionEntry(Box::new(source_ml_source2_nominal_override17223.clone()), Box::new(source_ml_source2_nominal_override17224.clone()), source_ml_source2_nominal_override17225)
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        } else {
                                                                                                            entry_2.clone()
                                                                                                        } }) }; ml_values902.iter().cloned().enumerate().map(|(ml_index902, ml_value902)| ml_callback902(ml_value902, (ml_index902 as f64))).collect::<Vec<_>>() })))
                                                                                                    }), Box::new(self::JavaScriptSemanticValue::Boolean((at >= (0f64)))))
                                                                                                } else {
                                                                                                    if ((at < (0f64)) && (({ let ml_reference_array904 = stored.clone().expect("a proven present value"); let ml_reference_projection904 = ml_reference_array904.borrow().len() as f64; ml_reference_projection904 }) >= (65536f64))) {
                                                                                                        self::JavaScriptCollectionCompletion::CollectionResourceLimit(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 115, 101, 114, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 105, 110, 97, 108, 32, 98, 117, 100, 103, 101, 116]))
                                                                                                    } else {
                                                                                                        {
                                                                                                            let value = if (kind == (self::ml_string::Text::from_units(vec![83, 101, 116]))) {
                                                                                                                key.clone()
                                                                                                            } else {
                                                                                                                ({
                                                                                                                    let source_ml_source2_array_read_index17227 = 1f64;
                                                                                                                    { let ml_reference_array905 = args.clone(); let ml_reference_position905 = source_ml_source2_array_read_index17227; let ml_reference_projection905 = self::ml_array::optional_at(&*ml_reference_array905.borrow(), self::ml_array::number_index(ml_reference_position905)); ml_reference_projection905 }
                                                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)
                                                                                                            };
                                                                                                            {
                                                                                                                let next = self::JavaScriptCollectionEntry::CollectionEntry(Box::new(key.clone()), Box::new(value.clone()), true);
                                                                                                                {
                                                                                                                    let entries = if (at < (0f64)) {
                                                                                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                            let source_ml_source2_concat_value17228 = stored.clone().expect("a proven present value");
                                                                                                                            {
                                                                                                                                let source_ml_source2_concat_value17229 = std::rc::Rc::new(std::cell::RefCell::new(vec![next.clone()]));
                                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCollectionEntry>::new(), { let ml_reference_value906 = source_ml_source2_concat_value17228.borrow().clone(); ml_reference_value906 }), { let ml_reference_value907 = source_ml_source2_concat_value17229.borrow().clone(); ml_reference_value907 })
                                                                                                                            }
                                                                                                                        }))
                                                                                                                    } else {
                                                                                                                        std::rc::Rc::new(std::cell::RefCell::new({ let ml_values908 = { let ml_reference_value909 = (stored.clone().expect("a proven present value")).borrow().clone(); ml_reference_value909 }; let ml_callback908 = { let at = at; let value = value.clone(); std::rc::Rc::new(move |current: self::JavaScriptCollectionEntry, index: f64| -> self::JavaScriptCollectionEntry { if (index == at) {
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_spread17230 = current.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field17231 = match source_ml_source2_nominal_spread17230.clone() {
                                                                                                                                        self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source2_data_property_value17232, _, _) => {
                                                                                                                                            let source_ml_source2_data_property_value17232 = *source_ml_source2_data_property_value17232;
                                                                                                                                            source_ml_source2_data_property_value17232.clone()
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field17233 = match source_ml_source2_nominal_spread17230.clone() {
                                                                                                                                            self::JavaScriptCollectionEntry::CollectionEntry(_, source_ml_source2_data_property_value17234, _) => {
                                                                                                                                                let source_ml_source2_data_property_value17234 = *source_ml_source2_data_property_value17234;
                                                                                                                                                source_ml_source2_data_property_value17234.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field17235 = match source_ml_source2_nominal_spread17230.clone() {
                                                                                                                                                self::JavaScriptCollectionEntry::CollectionEntry(_, _, source_ml_source2_data_property_value17236) => {
                                                                                                                                                    source_ml_source2_data_property_value17236
                                                                                                                                                }
                                                                                                                                            };
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_override17237 = value.clone();
                                                                                                                                                self::JavaScriptCollectionEntry::CollectionEntry(Box::new(source_ml_source2_nominal_field17231.clone()), Box::new(source_ml_source2_nominal_override17237.clone()), source_ml_source2_nominal_field17235)
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            current.clone()
                                                                                                                        } }) }; ml_values908.iter().cloned().enumerate().map(|(ml_index908, ml_value908)| ml_callback908(ml_value908, (ml_index908 as f64))).collect::<Vec<_>>() }))
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let updated = self::replace_java_script_collection_entries(heap.clone(), object.clone().expect("a proven present value"), entries.clone());
                                                                                                                        {
                                                                                                                            let limit = self::read_java_script_ordinary_heap_error(updated.clone());
                                                                                                                            if (limit != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                                self::JavaScriptCollectionCompletion::CollectionResourceLimit(Box::new(heap.clone()), limit.clone())
                                                                                                                            } else {
                                                                                                                                self::JavaScriptCollectionCompletion::CollectionNormal(Box::new(updated.clone()), Box::new(receiver.clone()))
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
                                                self::MlSource1TemporaryExecuteJavaScriptCollectionOperationLoop425Result::MlSource1TemporaryExecuteJavaScriptCollectionOperationLoop425Return(source_ml_source1_temporary_result425) => {
                                                    let source_ml_source1_temporary_result425 = *source_ml_source1_temporary_result425;
                                                    source_ml_source1_temporary_result425.clone()
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
