// Generated from complete js/src/translation/javascript-arrays.js. Do not edit by hand.
use super::JavaScriptArrayCompletion;
use super::JavaScriptArrayDefinitionState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::JavaScriptSymbolRegistryEntry;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop94Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop100Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop116Result;
use super::construct_java_script_array_object;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::order_java_script_collection_keys;
use super::read_java_script_ordinary_value_error;
use super::source_ml_temporary_allocate_java_script_array_loop134;
use super::source_ml_temporary_allocate_java_script_array_loop135;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop93;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop94;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop96;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop97;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop99;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop100;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop101;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop109;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop110;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop111;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop116;
use super::test_java_script_array_length;
use super::test_java_script_ordinary_symbol;
use super::test_java_script_standard_array_intrinsic_operation;
use super::test_java_script_validated_heap_snapshot;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_heap_error(
    heap: self::JavaScriptOrdinaryHeap,
) -> self::ml_string::Text {
    if self::test_java_script_validated_heap_snapshot(heap.clone()) {
        self::ml_string::Text::from_units(vec![])
    } else if (({
        let ml_reference_array174 = match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                source_ml_source1_data_property_value5635,
                _,
                _,
                _,
                _,
                _,
            ) => source_ml_source1_data_property_value5635.clone(),
        };
        ml_reference_array174.borrow().len() as f64
    }) > 4096f64)
        || (({
            let ml_reference_array175 = match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    _,
                    source_ml_source1_data_property_value5637,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value5637.clone(),
            };
            ml_reference_array175.borrow().len() as f64
        }) > 16_384f64)
    {
        self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110,
            116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99,
            101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32,
            114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116,
        ])
    } else {
        {
            let previous = self::ml::Big::from_i128(0);
            {
                let source_ml_temporary_values77 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source1_data_property_value5638,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5638.clone(),
                };
                {
                    let source_ml_temporary_index77 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_s5639 =
                            self::source_ml_temporary_read_java_script_ordinary_heap_error_loop93(
                                heap.clone(),
                                previous,
                                source_ml_temporary_values77,
                                source_ml_temporary_index77,
                            );
                        match source_ml_source1_s5639 {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Done(previous_2) => {
                                    {
                                        let previous_3 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_temporary_values78 = match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value5640, _, _, _, _) => {
                                                    source_ml_source1_data_property_value5640.clone()
                                                }
                                            };
                                            {
                                                let source_ml_temporary_index78 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source1_s5641 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop94(heap.clone(), previous_3, source_ml_temporary_values78, source_ml_temporary_index78);
                                                    match source_ml_source1_s5641 {
                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop94Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop94Done => {
                                                            {
                                                                let source_ml_source1_s5660 = if !(match (match heap.clone() {
                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value5661, _, _) => {
                                                                        ((source_ml_source1_data_property_value5661.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                    }
                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || {
                                                                    let source_ml_source1_comparison_operand5663 = match heap.clone() {
                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, source_ml_source1_data_property_value5662, _, _, _) => {
                                                                            ((source_ml_source1_data_property_value5662.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_comparison_operand5664 = None::<()>;
                                                                        !(match (source_ml_source1_comparison_operand5663, source_ml_source1_comparison_operand5664) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                    }
                                                                } {
                                                                    if ({
                                                                        let source_ml_source1_comparison_operand5666 = match heap.clone() {
                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, source_ml_source1_data_property_value5665, _, _, _) => {
                                                                                ((source_ml_source1_data_property_value5665.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                            }
                                                                        };
                                                                        !(match (source_ml_source1_comparison_operand5666, self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 99, 108, 97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110, 32, 49])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                    } || !({
                                                                        let source_ml_source1_array_argument5669 = match heap.clone() {
                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value5668, _, _) => {
                                                                                ((source_ml_source1_data_property_value5668.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                            }
                                                                        };
                                                                        match source_ml_source1_array_argument5669 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }
                                                                    })) || (({
                                                                        let source_ml_source1_nullish_field_receiver5675 = (match heap.clone() {
                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value5674, _, _) => {
                                                                                ((source_ml_source1_data_property_value5674.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                            }
                                                                        },);
                                                                        if match (source_ml_source1_nullish_field_receiver5675.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                            {
                                                                                let source_ml_source1_nullish_field_present5676 = ((source_ml_source1_nullish_field_receiver5675.clone()).0).clone().expect("a proven present value");
                                                                                { let ml_reference_array176 = source_ml_source1_nullish_field_present5676; ml_reference_array176.borrow().len() as f64 }
                                                                            }
                                                                        } else {
                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
                                                                        }
                                                                    }) > 16_384f64) {
                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        {
                                                                            let registrations = (match heap.clone() {
                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value5677, _, _) => {
                                                                                    ((source_ml_source1_data_property_value5677.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                                }
                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistrationFact>::new())));
                                                                            {
                                                                                let prior = self::ml::Big::from_i128(0);
                                                                                {
                                                                                    let source_ml_temporary_index79 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let source_ml_source1_s5678 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop116(heap.clone(), prior, registrations, source_ml_temporary_index79);
                                                                                        match source_ml_source1_s5678 {
                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop116Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop116Done => {
                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95Next
                                                                                            }
                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop116Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop116Return(source_ml_temporary_result116) => {
                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95Return(source_ml_temporary_result116.clone())
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95Next
                                                                };
                                                                match source_ml_source1_s5660 {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95Next => {
                                                                        {
                                                                            let registered_keys = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                            {
                                                                                let source_ml_temporary_values81 = (match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value5679, _, _) => {
                                                                                        ((source_ml_source1_data_property_value5679.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                                    }
                                                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistrationFact>::new())));
                                                                                {
                                                                                    let source_ml_temporary_index81 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let source_ml_source1_s5680 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop96(registered_keys.clone(), source_ml_temporary_values81, source_ml_temporary_index81);
                                                                                        match source_ml_source1_s5680 {
                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop96Done => {
                                                                                                {
                                                                                                    let witnessed_keys = self::order_java_script_collection_keys(registered_keys.clone());
                                                                                                    {
                                                                                                        let witnessed_units = 0f64;
                                                                                                        {
                                                                                                            let previous_witness = None::<()>;
                                                                                                            {
                                                                                                                let source_ml_temporary_index82 = self::ml::Big::from_i128(0);
                                                                                                                {
                                                                                                                    let source_ml_source1_s5681 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop97(witnessed_units, None::<self::JavaScriptSemanticValue>, witnessed_keys, source_ml_temporary_index82);
                                                                                                                    match source_ml_source1_s5681 {
                                                                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Done => {
                                                                                                                            {
                                                                                                                                let source_ml_source1_s5703 = if !(match (match heap.clone() {
                                                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source1_data_property_value5704) => {
                                                                                                                                        ((source_ml_source1_data_property_value5704.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                                                    }
                                                                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || {
                                                                                                                                    let source_ml_source1_comparison_operand5706 = match heap.clone() {
                                                                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, source_ml_source1_data_property_value5705, _) => {
                                                                                                                                            ((source_ml_source1_data_property_value5705.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source1_comparison_operand5707 = None::<()>;
                                                                                                                                        !(match (source_ml_source1_comparison_operand5706, source_ml_source1_comparison_operand5707) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                                                                                    }
                                                                                                                                } {
                                                                                                                                    if ({
                                                                                                                                        let source_ml_source1_comparison_operand5709 = match heap.clone() {
                                                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, source_ml_source1_data_property_value5708, _) => {
                                                                                                                                                ((source_ml_source1_data_property_value5708.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        !(match (source_ml_source1_comparison_operand5709, self::ml_string::Text::from_units(vec![114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 49])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                                                                                    } || !({
                                                                                                                                        let source_ml_source1_array_argument5712 = match heap.clone() {
                                                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source1_data_property_value5711) => {
                                                                                                                                                ((source_ml_source1_data_property_value5711.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        match source_ml_source1_array_argument5712 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }
                                                                                                                                    })) || (({
                                                                                                                                        let source_ml_source1_nullish_field_receiver5718 = (match heap.clone() {
                                                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source1_data_property_value5717) => {
                                                                                                                                                ((source_ml_source1_data_property_value5717.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        },);
                                                                                                                                        if match (source_ml_source1_nullish_field_receiver5718.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                                            {
                                                                                                                                                let source_ml_source1_nullish_field_present5719 = ((source_ml_source1_nullish_field_receiver5718.clone()).0).clone().expect("a proven present value");
                                                                                                                                                { let ml_reference_array177 = source_ml_source1_nullish_field_present5719; ml_reference_array177.borrow().len() as f64 }
                                                                                                                                            }
                                                                                                                                        } else {
                                                                                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
                                                                                                                                        }
                                                                                                                                    }) > 16_384f64) {
                                                                                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98Return(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 115, 104, 97, 114, 101, 100, 32, 97, 103, 101, 110, 116, 32, 109, 111, 100, 101, 108]))
                                                                                                                                    } else {
                                                                                                                                        {
                                                                                                                                            let facts = (match heap.clone() {
                                                                                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value5720, _, _) => {
                                                                                                                                                    ((source_ml_source1_data_property_value5720.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                                                                                                }
                                                                                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistrationFact>::new())));
                                                                                                                                            {
                                                                                                                                                let registry = (match heap.clone() {
                                                                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source1_data_property_value5721) => {
                                                                                                                                                        ((source_ml_source1_data_property_value5721.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                                                                    }
                                                                                                                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistryEntry>::new())));
                                                                                                                                                {
                                                                                                                                                    let identities = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                                                                                    {
                                                                                                                                                        let previous_key = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                        {
                                                                                                                                                            let units = 0f64;
                                                                                                                                                            {
                                                                                                                                                                let source_ml_temporary_index83 = self::ml::Big::from_i128(0);
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source1_s5722 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop109(heap.clone(), facts.clone(), identities.clone(), previous_key, units, registry.clone(), source_ml_temporary_index83);
                                                                                                                                                                    match source_ml_source1_s5722 {
                                                                                                                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Done => {
                                                                                                                                                                            {
                                                                                                                                                                                let ordered = self::order_java_script_collection_keys(identities.clone());
                                                                                                                                                                                {
                                                                                                                                                                                    let previous_identity = None::<()>;
                                                                                                                                                                                    {
                                                                                                                                                                                        let source_ml_temporary_index88 = self::ml::Big::from_i128(0);
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source1_s5723 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop110(None::<self::JavaScriptSemanticValue>, ordered, source_ml_temporary_index88);
                                                                                                                                                                                            match source_ml_source1_s5723 {
                                                                                                                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Done => {
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_temporary_index89 = self::ml::Big::from_i128(0);
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source1_s5724 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop111(registry.clone(), facts.clone(), source_ml_temporary_index89);
                                                                                                                                                                                                            match source_ml_source1_s5724 {
                                                                                                                                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Done => {
                                                                                                                                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98Next
                                                                                                                                                                                                                }
                                                                                                                                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Return(source_ml_temporary_result111) => {
                                                                                                                                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98Return(source_ml_temporary_result111.clone())
                                                                                                                                                                                                                }
                                                                                                                                                                                                            }
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Return(source_ml_temporary_result112) => {
                                                                                                                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98Return(source_ml_temporary_result112.clone())
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(source_ml_temporary_result112_2) => {
                                                                                                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98Return(source_ml_temporary_result112_2.clone())
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
                                                                                                                                } else {
                                                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98Next
                                                                                                                                };
                                                                                                                                match source_ml_source1_s5703 {
                                                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98Next => {
                                                                                                                                        {
                                                                                                                                            let budget = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                                let source_ml_source1_field5725 = 0f64;
                                                                                                                                                {
                                                                                                                                                    let source_ml_source1_field5726 = 0f64;
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source1_field5727 = 0f64;
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source1_field5728 = 0f64;
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source1_field5729 = 0f64;
                                                                                                                                                                (source_ml_source1_field5725, source_ml_source1_field5727, source_ml_source1_field5728, source_ml_source1_field5729, source_ml_source1_field5726)
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }));
                                                                                                                                            {
                                                                                                                                                let source_ml_temporary_values91 = match heap.clone() {
                                                                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value5730, _, _, _, _, _) => {
                                                                                                                                                        source_ml_source1_data_property_value5730.clone()
                                                                                                                                                    }
                                                                                                                                                };
                                                                                                                                                {
                                                                                                                                                    let source_ml_temporary_index91 = self::ml::Big::from_i128(0);
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source1_s5731 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop99(heap.clone(), budget, source_ml_temporary_values91, source_ml_temporary_index91);
                                                                                                                                                        match source_ml_source1_s5731 {
                                                                                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Done => {
                                                                                                                                                                {
                                                                                                                                                                    let marks = std::rc::Rc::new(std::cell::RefCell::new(Vec::<std::rc::Rc<std::cell::RefCell<(f64,)>>>::new()));
                                                                                                                                                                    {
                                                                                                                                                                        let index = 0f64;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_s5732 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop100(heap.clone(), marks.clone(), index);
                                                                                                                                                                            match source_ml_source1_s5732 {
                                                                                                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop100Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop100Done => {
                                                                                                                                                                                    {
                                                                                                                                                                                        let first = 0f64;
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source1_s5733 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop101(heap.clone(), marks.clone(), first);
                                                                                                                                                                                            match source_ml_source1_s5733 {
                                                                                                                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Done => {
                                                                                                                                                                                                    self::ml_string::Text::from_units(vec![])
                                                                                                                                                                                                }
                                                                                                                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop101Return(source_ml_temporary_result101) => {
                                                                                                                                                                                                    source_ml_temporary_result101.clone()
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
                                                                                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(source_ml_temporary_result107) => {
                                                                                                                                                                source_ml_temporary_result107.clone()
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin98Return(source_ml_temporary_result108) => {
                                                                                                                                        source_ml_temporary_result108.clone()
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop97Return(source_ml_temporary_result114) => {
                                                                                                                            source_ml_temporary_result114.clone()
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
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin95Return(source_ml_temporary_result115) => {
                                                                        source_ml_temporary_result115.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop94Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop94Return(source_ml_temporary_result116_2) => {
                                                            source_ml_temporary_result116_2.clone()
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop93Return(source_ml_temporary_result116_3) => {
                                    source_ml_temporary_result116_3.clone()
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
pub fn replace_java_script_ordinary_object(
    heap: self::JavaScriptOrdinaryHeap,
    replacement: self::JavaScriptOrdinaryObject,
) -> self::JavaScriptOrdinaryHeap {
    {
        let objects = std::rc::Rc::new(std::cell::RefCell::new({
            let ml_values178 = {
                (match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source1_data_property_value5734,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5734.clone(),
                })
                .borrow()
                .clone()
            };
            let ml_callback178 = {
                let replacement = replacement.clone();
                std::rc::Rc::new(move |object: self::JavaScriptOrdinaryObject| -> self::JavaScriptOrdinaryObject { if (match object.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value5761, _, _, _, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value5761.clone()
            }
            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value5762, _, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value5762.clone()
            }
            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value5763, _, _, _, _) => {
                source_ml_source1_data_property_value5763.clone()
            }
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value5764, _, _, _, _, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value5764.clone()
            }
            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value5765, _, _, _, _, _) => {
                source_ml_source1_data_property_value5765.clone()
            }
            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value5766, _, _, _, _, _) => {
                source_ml_source1_data_property_value5766.clone()
            }
            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value5767, _, _, _, _, _) => {
                source_ml_source1_data_property_value5767.clone()
            }
            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value5768, _, _, _, _, _) => {
                source_ml_source1_data_property_value5768.clone()
            }
            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value5769, _, _, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value5769.clone()
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value5770, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value5770.clone()
            }
            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value5771, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value5771.clone()
            }
            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value5772, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value5772.clone()
            }
            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value5773, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value5773.clone()
            }
        }) == (match replacement.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value5774, _, _, _, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value5774.clone()
            }
            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value5775, _, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value5775.clone()
            }
            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value5776, _, _, _, _) => {
                source_ml_source1_data_property_value5776.clone()
            }
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value5777, _, _, _, _, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value5777.clone()
            }
            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value5778, _, _, _, _, _) => {
                source_ml_source1_data_property_value5778.clone()
            }
            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value5779, _, _, _, _, _) => {
                source_ml_source1_data_property_value5779.clone()
            }
            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value5780, _, _, _, _, _) => {
                source_ml_source1_data_property_value5780.clone()
            }
            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value5781, _, _, _, _, _) => {
                source_ml_source1_data_property_value5781.clone()
            }
            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value5782, _, _, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value5782.clone()
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value5783, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value5783.clone()
            }
            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value5784, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value5784.clone()
            }
            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value5785, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value5785.clone()
            }
            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value5786, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value5786.clone()
            }
        }) {
            replacement.clone()
        } else {
            object.clone()
        } })
            };
            ml_values178
                .iter()
                .cloned()
                .enumerate()
                .map(|(ml_index178, ml_value178)| ml_callback178(ml_value178))
                .collect::<Vec<_>>()
        }));
        {
            let source_ml_source1_nominal_field5788 = match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    source_ml_source1_data_property_value5789,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value5789.clone(),
            };
            {
                let source_ml_source1_nominal_field5790 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        _,
                        source_ml_source1_data_property_value5791,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5791.clone(),
                };
                {
                    let source_ml_source1_nominal_field5792 = match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            _,
                            _,
                            source_ml_source1_data_property_value5793,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value5793.clone(),
                    };
                    {
                        let source_ml_source1_nominal_field5794 = match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value5795,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value5795.clone(),
                        };
                        {
                            let source_ml_source1_nominal_field5796 = match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value5797,
                                    _,
                                ) => source_ml_source1_data_property_value5797.clone(),
                            };
                            {
                                let source_ml_source1_nominal_field5798 = match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value5799,
                                    ) => source_ml_source1_data_property_value5799.clone(),
                                };
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                    objects,
                                    source_ml_source1_nominal_field5790,
                                    source_ml_source1_nominal_field5792,
                                    source_ml_source1_nominal_field5794,
                                    source_ml_source1_nominal_field5796,
                                    source_ml_source1_nominal_field5798,
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
pub fn test_java_script_array_intrinsic_operation(operation: &self::ml_string::Text) -> bool {
    self::test_java_script_standard_array_intrinsic_operation(operation)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn allocate_java_script_array(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    prototype: self::JavaScriptSemanticValue,
    length: f64,
) -> self::JavaScriptArrayCompletion {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let ml_library_inequality_3 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_3 {
                self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), error.clone())
            } else if (((identity <= (self::ml::Big::from_i128(0)))
                || (identity > (self::ml::Big::from_i128(18_446_744_073_709_551_615))))
                || ((self::find_java_script_ordinary_object(
                    match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value17326,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17326.clone(),
                    },
                    identity.clone(),
                )) >= 0f64))
                || self::test_java_script_ordinary_symbol(
                    match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            _,
                            source_ml_source1_data_property_value17327,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17327.clone(),
                    },
                    identity.clone(),
                )
            {
                self::JavaScriptArrayCompletion::ArrayInvalid(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110,
                        32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 102, 114, 101, 115,
                        104, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32, 117, 110, 115, 105,
                        103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100,
                        101, 110, 116, 105, 116, 121,
                    ]),
                )
            } else {
                {
                    let actual = if length == 0f64 { 0f64 } else { length };
                    if (self::test_java_script_array_length(actual)) {
                        if match prototype.clone() {
                            self::JavaScriptSemanticValue::Null => false,
                            _ => true,
                        } && match prototype.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => false,
                            _ => true,
                        } {
                            self::JavaScriptArrayCompletion::ArrayInvalid(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105,
                                    111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116,
                                    115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 117, 108, 108, 32,
                                    111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111,
                                    116, 111, 116, 121, 112, 101,
                                ]),
                            )
                        } else {
                            {
                                let invalid = self::read_java_script_ordinary_value_error(
                                    heap.clone(),
                                    prototype.clone(),
                                );
                                {
                                    let ml_library_inequality_2 =
                                        invalid != (self::ml_string::Text::from_units(vec![]));
                                    if ml_library_inequality_2 {
                                        self::JavaScriptArrayCompletion::ArrayInvalid(
                                            Box::new(heap.clone()),
                                            invalid.clone(),
                                        )
                                    } else {
                                        {
                                            let count = 0f64;
                                            {
                                                let source_ml_temporary_values132 = match heap
                                                    .clone()
                                                {
                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                        source_ml_source1_data_property_value17346,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                    ) => source_ml_source1_data_property_value17346
                                                        .clone(),
                                                };
                                                {
                                                    let source_ml_temporary_index132 =
                                                        self::ml::Big::from_i128(0);
                                                    {
                                                        let count_2 = self::source_ml_temporary_allocate_java_script_array_loop134(count, source_ml_temporary_values132, source_ml_temporary_index132);
                                                        if (({
                                                            let ml_reference_array180 = match heap.clone() {
                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17348, _, _, _, _, _) => {
                                                            source_ml_source1_data_property_value17348.clone()
                                                        }
                                                    };
                                                            ml_reference_array180.borrow().len()
                                                                as f64
                                                        }) >= 4096f64)
                                                            || (count_2 >= 16_384f64)
                                                        {
                                                            self::JavaScriptArrayCompletion::ArrayResourceLimit(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 111, 98, 106, 101, 99, 116, 32, 111, 114, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 98, 117, 100, 103, 101, 116]), None::<(Option<self::JavaScriptArrayDefinitionState>,)>)
                                                        } else {
                                                            {
                                                                let object = self::construct_java_script_array_object(identity.clone(), prototype.clone(), actual);
                                                                {
                                                                    let at = 0f64;
                                                                    {
                                                                        let at_2 = self::source_ml_temporary_allocate_java_script_array_loop135(heap.clone(), identity.clone(), at);
                                                                        {
                                                                            let next = {
                                                                                let source_ml_source1_nominal_field17359 = match heap.clone() {
                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17360, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value17360.clone()
                                                                                }
                                                                            };
                                                                                {
                                                                                    let source_ml_source1_nominal_field17361 = match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value17362, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value17362.clone()
                                                                                    }
                                                                                };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field17363 = match heap.clone() {
                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, source_ml_source1_data_property_value17364, _, _, _) => {
                                                                                            source_ml_source1_data_property_value17364.clone()
                                                                                        }
                                                                                    };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field17365 = match heap.clone() {
                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value17366, _, _) => {
                                                                                                source_ml_source1_data_property_value17366.clone()
                                                                                            }
                                                                                        };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field17367 = match heap.clone() {
                                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, source_ml_source1_data_property_value17368, _) => {
                                                                                                    source_ml_source1_data_property_value17368.clone()
                                                                                                }
                                                                                            };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field17369 = match heap.clone() {
                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source1_data_property_value17370) => {
                                                                                                        source_ml_source1_data_property_value17370.clone()
                                                                                                    }
                                                                                                };
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_override17371 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                        let source_ml_source1_concat_value17355 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ (match heap.clone() {
                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17352, _, _, _, _, _) => {
                                                                                                                source_ml_source1_data_property_value17352.clone()
                                                                                                            }
                                                                                                        }).borrow().clone() }, 0f64, at_2)));
                                                                                                        {
                                                                                                            let source_ml_source1_concat_value17356 = std::rc::Rc::new(std::cell::RefCell::new(vec![object]));
                                                                                                            {
                                                                                                                let source_ml_source1_concat_value17357 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ (match heap.clone() {
                                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17354, _, _, _, _, _) => {
                                                                                                                        source_ml_source1_data_property_value17354.clone()
                                                                                                                    }
                                                                                                                }).borrow().clone() }, at_2, f64::INFINITY)));
                                                                                                                self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), { source_ml_source1_concat_value17355.borrow().clone() }), { source_ml_source1_concat_value17356.borrow().clone() }), { source_ml_source1_concat_value17357.borrow().clone() })
                                                                                                            }
                                                                                                        }
                                                                                                    }));
                                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_nominal_override17371, source_ml_source1_nominal_field17361, source_ml_source1_nominal_field17363, source_ml_source1_nominal_field17365, source_ml_source1_nominal_field17367, source_ml_source1_nominal_field17369)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            };
                                                                            {
                                                                                let problem = self::read_java_script_ordinary_heap_error(next.clone());
                                                                                {
                                                                                    let ml_library_inequality_1 = problem != (self::ml_string::Text::from_units(vec![])) ;
                                                                                    if ml_library_inequality_1 {
                                                                                self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), problem.clone())
                                                                            } else {
                                                                                self::JavaScriptArrayCompletion::ArrayNormal(Box::new(next.clone()), Box::new(self::JavaScriptSemanticValue::Object(identity.clone(), false)))
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
                    } else {
                        self::JavaScriptArrayCompletion::ArrayError(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                82, 97, 110, 103, 101, 69, 114, 114, 111, 114,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32,
                                108, 101, 110, 103, 116, 104,
                            ]),
                        )
                    }
                }
            }
        }
    }
}
