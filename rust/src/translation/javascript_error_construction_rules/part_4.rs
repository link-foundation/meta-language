// Generated from complete js/src/translation/javascript-error-construction-rules.js. Do not edit by hand.
use super::JavaScriptErrorAllocationDirectory;
use super::JavaScriptErrorAllocationIdentity;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result;
use super::find_java_script_heap_directory_object;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_heap_directory_value_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_heap_directory_object_loop8(mut objects: Vec<self::JavaScriptOrdinaryObject>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let object = self::ml_array::optional_at(&objects, self::ml_array::number_index(middle));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Return((-1f64))
                    } else {
                        if ((match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value819, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value819.clone()
                            }
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value820, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value820.clone()
                            }
                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value821, _, _, _, _) => {
                                source_ml_source2_data_property_value821.clone()
                            }
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value822, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value822.clone()
                            }
                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value823, _, _, _, _, _) => {
                                source_ml_source2_data_property_value823.clone()
                            }
                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value824, _, _, _, _, _) => {
                                source_ml_source2_data_property_value824.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value825, _, _, _, _, _) => {
                                source_ml_source2_data_property_value825.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value826, _, _, _, _, _) => {
                                source_ml_source2_data_property_value826.clone()
                            }
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value827, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value827.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value828, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value828.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value829, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value829.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value830, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value830.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value831, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value831.clone()
                            }
                        }) == identity) {
                            self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(middle)
                        } else {
                            if ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value845, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value845.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value846, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value846.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value847, _, _, _, _) => {
                                    source_ml_source2_data_property_value847.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value848, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value848.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value849, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value849.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value850, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value850.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value851, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value851.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value852, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value852.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value853, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value853.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value854, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value854.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value855, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value855.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value856, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value856.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value857, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value857.clone()
                                }
                            }) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    high = middle;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result {
    MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Done,
    MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_java_script_heap_directory_symbol_loop9(mut symbols: Vec<self::ml::Big>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let symbol = self::ml_array::optional_at(&symbols, self::ml_array::number_index(middle));
                    if {
                        let source_ml_source2_comparison_operand859 = None::<()>;
                        match (symbol.clone(), source_ml_source2_comparison_operand859.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                    } {
                        self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(false)
                    } else {
                        if ((symbol.clone().expect("a proven present value")) == identity) {
                            self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(true)
                        } else {
                            if ((symbol.clone().expect("a proven present value")) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    high = middle;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result {
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Done,
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop13(mut heap: self::JavaScriptOrdinaryHeap, mut before: self::JavaScriptErrorAllocationDirectory, mut previous: self::ml::Big, mut source_ml_source1_temporary_values11: Vec<self::ml::Big>, mut source_ml_source1_temporary_index11: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result {
    loop {
        return if (source_ml_source1_temporary_index11 < (self::ml::Big::from_u128(source_ml_source1_temporary_values11.len() as u128))) {
            {
                let symbol = self::ml_array::at(&source_ml_source1_temporary_values11, source_ml_source1_temporary_index11.to_index());
                if (((symbol <= previous) || (symbol > (self::ml::Big::from_i128(18446744073709551615)))) || { let ml_values3 = match before.clone() {
                    self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(source_ml_source2_data_property_value860, _) => {
                        source_ml_source2_data_property_value860
                    }
                }; let ml_callback3 = { let symbol = symbol.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorAllocationIdentity| -> bool { ((match row.clone() {
                    self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(source_ml_source2_data_property_value862, _, _) => {
                        source_ml_source2_data_property_value862.clone()
                    }
                }) == symbol) }) }; ml_values3.iter().cloned().enumerate().any(|(ml_index3, ml_value3)| ml_callback3(ml_value3)) }) {
                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Return(self::ml_string::Text::from_units(vec![112, 114, 105, 111, 114, 32, 83, 121, 109, 98, 111, 108, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    if !(self::ml_array::includes((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value864, _, _, _, _) => source_ml_source2_data_property_value864.as_slice() }), &symbol.clone(), 0f64, false, |a, b| a == b)) {
                        self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Return(self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 109, 111, 118, 101, 32, 111, 114, 32, 114, 101, 116, 121, 112, 101, 32, 112, 114, 105, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                    } else {
                        {
                            let source_ml_source1_temporary_index11_2 = source_ml_source1_temporary_index11.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_source1_temporary_index11) = (symbol.clone(), source_ml_source1_temporary_index11_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result {
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Done(self::ml::Big),
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop12(mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut source_ml_source1_temporary_values10: Vec<self::JavaScriptErrorAllocationIdentity>, mut source_ml_source1_temporary_index10: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field4, _, _, _, _, _) => {
                if (source_ml_source1_temporary_index10 < (self::ml::Big::from_u128(source_ml_source1_temporary_values10.len() as u128))) {
                    {
                        let row = self::ml_array::at(&source_ml_source1_temporary_values10, source_ml_source1_temporary_index10.to_index());
                        match row.clone() {
                            self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                if ((shared_constructor_field1 <= previous) || (shared_constructor_field1 > (self::ml::Big::from_i128(18446744073709551615)))) {
                                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Return(self::ml_string::Text::from_units(vec![112, 114, 105, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 115, 116, 114, 105, 99, 116, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 111, 114, 100, 101, 114]))
                                } else {
                                    {
                                        let previous_2 = shared_constructor_field1.clone();
                                        {
                                            let object = self::ml_array::optional_at(&shared_constructor_field4, self::ml_array::number_index(self::find_java_script_heap_directory_object(shared_constructor_field4, shared_constructor_field1.clone())));
                                            if ((match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || (({
                                                let source_ml_source2_nullish_field_receiver876 = (object.clone(),);
                                                if match (source_ml_source2_nullish_field_receiver876.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    {
                                                        let source_ml_source2_nullish_field_present877 = ((source_ml_source2_nullish_field_receiver876.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source2_nullish_field_present877.clone() {
                                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                                self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116])
                                                            }
                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116])
                                                            }
                                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                            }
                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                                self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])
                                                            }
                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                                self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
                                                            }
                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116])
                                                            }
                                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                                self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116])
                                                            }
                                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                                self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116])
                                                            }
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])
                                                            }
                                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                                self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116])
                                                            }
                                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                                self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116])
                                                            }
                                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                                self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                            }
                                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                                self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 36, 39, 41])).0, message_present: true }).raise()
                                                }
                                            }) != shared_constructor_field2)) || (({
                                                let source_ml_source2_nullish_field_receiver895 = (object.clone(),);
                                                if match (source_ml_source2_nullish_field_receiver895.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    {
                                                        let source_ml_source2_nullish_field_present896 = ((source_ml_source2_nullish_field_receiver895.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source2_nullish_field_present896.clone() {
                                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value897, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value897
                                                            }
                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value898, _, _, _, _) => {
                                                                source_ml_source2_data_property_value898
                                                            }
                                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value899, _) => {
                                                                source_ml_source2_data_property_value899
                                                            }
                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value900, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value900
                                                            }
                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value901, _, _) => {
                                                                source_ml_source2_data_property_value901
                                                            }
                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value902, _, _) => {
                                                                source_ml_source2_data_property_value902
                                                            }
                                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value903, _, _) => {
                                                                source_ml_source2_data_property_value903
                                                            }
                                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value904, _, _) => {
                                                                source_ml_source2_data_property_value904
                                                            }
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value905, _) => {
                                                                source_ml_source2_data_property_value905
                                                            }
                                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value906, _, _, _) => {
                                                                source_ml_source2_data_property_value906
                                                            }
                                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value907, _, _, _) => {
                                                                source_ml_source2_data_property_value907
                                                            }
                                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value908, _, _, _) => {
                                                                source_ml_source2_data_property_value908
                                                            }
                                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value909, _, _, _) => {
                                                                source_ml_source2_data_property_value909
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                                }
                                            }) != shared_constructor_field3)) {
                                                self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Return(self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 109, 111, 118, 101, 32, 111, 114, 32, 114, 101, 116, 121, 112, 101, 32, 112, 114, 105, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                            } else {
                                                {
                                                    let source_ml_source1_temporary_index10_2 = source_ml_source1_temporary_index10.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        (previous, source_ml_source1_temporary_index10) = (previous_2.clone(), source_ml_source1_temporary_index10_2.clone());
                                                        continue;
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
                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Done(previous.clone())
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result {
    MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Done,
    MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_error_construction_operands_error_loop15(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_source1_temporary_values14: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index14: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result {
    loop {
        return if (source_ml_source1_temporary_index14 < (self::ml::Big::from_u128(source_ml_source1_temporary_values14.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values14, source_ml_source1_temporary_index14.to_index());
                {
                    let error = self::read_java_script_heap_directory_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index14_2 = source_ml_source1_temporary_index14.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index14 = source_ml_source1_temporary_index14_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Done
        };
    }
}
