// Generated from complete js/src/translation/javascript-array-assignment.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::JavaScriptSymbolRegistryEntry;
use super::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop94Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop100Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop116Result;
use super::create_java_script_property_descriptor;
use super::ml;
use super::ml_source_exception;
use super::ml_string;
use super::order_java_script_collection_keys;
use super::source_ml_source1_temporary_locate_java_script_ordinary_property_loop118;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop93;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop94;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop96;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop97;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop99;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop100;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop101;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop109;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop110;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop111;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop116;
use super::test_java_script_validated_heap_snapshot;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_heap_error(
    heap: self::JavaScriptOrdinaryHeap,
) -> self::ml_string::Text {
    if self::test_java_script_validated_heap_snapshot(heap.clone()) {
        self::ml_string::Text::from_units(vec![])
    } else if (({
        let ml_reference_array171 = match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                source_ml_source2_data_property_value5635,
                _,
                _,
                _,
                _,
                _,
            ) => source_ml_source2_data_property_value5635.clone(),
        };
        ml_reference_array171.borrow().len() as f64
    }) > 4096f64)
        || (({
            let ml_reference_array172 = match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    _,
                    source_ml_source2_data_property_value5637,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value5637.clone(),
            };
            ml_reference_array172.borrow().len() as f64
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
                let source_ml_source1_temporary_values77 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source2_data_property_value5638,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value5638.clone(),
                };
                {
                    let source_ml_source1_temporary_index77 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s5639 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop93(heap.clone(), previous, source_ml_source1_temporary_values77, source_ml_source1_temporary_index77);
                        match source_ml_source2_s5639 {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Done(previous_2) => {
                                    {
                                        let previous_3 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source1_temporary_values78 = match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value5640, _, _, _, _) => {
                                                    source_ml_source2_data_property_value5640.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source1_temporary_index78 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s5641 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop94(heap.clone(), previous_3, source_ml_source1_temporary_values78, source_ml_source1_temporary_index78);
                                                    match source_ml_source2_s5641 {
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop94Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop94Done => {
                                                            {
                                                                let source_ml_source2_s5660 = if !(match (match heap.clone() {
                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source2_data_property_value5661, _, _) => {
                                                                        ((source_ml_source2_data_property_value5661.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                    }
                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || {
                                                                    let source_ml_source2_comparison_operand5663 = match heap.clone() {
                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, source_ml_source2_data_property_value5662, _, _, _) => {
                                                                            ((source_ml_source2_data_property_value5662.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_comparison_operand5664 = None::<()>;
                                                                        !(match (source_ml_source2_comparison_operand5663, source_ml_source2_comparison_operand5664) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                    }
                                                                } {
                                                                    if ({
                                                                        let source_ml_source2_comparison_operand5666 = match heap.clone() {
                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, source_ml_source2_data_property_value5665, _, _, _) => {
                                                                                ((source_ml_source2_data_property_value5665.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                            }
                                                                        };
                                                                        !(match (source_ml_source2_comparison_operand5666, self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 99, 108, 97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110, 32, 49])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                    } || !({
                                                                        let source_ml_source2_array_argument5669 = match heap.clone() {
                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source2_data_property_value5668, _, _) => {
                                                                                ((source_ml_source2_data_property_value5668.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                            }
                                                                        };
                                                                        match source_ml_source2_array_argument5669 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }
                                                                    })) || (({
                                                                        let source_ml_source2_nullish_field_receiver5675 = (match heap.clone() {
                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source2_data_property_value5674, _, _) => {
                                                                                ((source_ml_source2_data_property_value5674.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                            }
                                                                        },);
                                                                        if match (source_ml_source2_nullish_field_receiver5675.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                            {
                                                                                let source_ml_source2_nullish_field_present5676 = ((source_ml_source2_nullish_field_receiver5675.clone()).0).clone().expect("a proven present value");
                                                                                { let ml_reference_array173 = source_ml_source2_nullish_field_present5676; ml_reference_array173.borrow().len() as f64 }
                                                                            }
                                                                        } else {
                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
                                                                        }
                                                                    }) > 16_384f64) {
                                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        {
                                                                            let registrations = (match heap.clone() {
                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source2_data_property_value5677, _, _) => {
                                                                                    ((source_ml_source2_data_property_value5677.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                                }
                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistrationFact>::new())));
                                                                            {
                                                                                let prior = self::ml::Big::from_i128(0);
                                                                                {
                                                                                    let source_ml_source1_temporary_index79 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let source_ml_source2_s5678 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop116(heap.clone(), prior, registrations, source_ml_source1_temporary_index79);
                                                                                        match source_ml_source2_s5678 {
                                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop116Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop116Done => {
                                                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95Next
                                                                                            }
                                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop116Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop116Return(source_ml_source1_temporary_result116) => {
                                                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95Return(source_ml_source1_temporary_result116.clone())
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95Next
                                                                };
                                                                match source_ml_source2_s5660 {
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95Next => {
                                                                        {
                                                                            let registered_keys = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                            {
                                                                                let source_ml_source1_temporary_values81 = (match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source2_data_property_value5679, _, _) => {
                                                                                        ((source_ml_source2_data_property_value5679.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                                    }
                                                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistrationFact>::new())));
                                                                                {
                                                                                    let source_ml_source1_temporary_index81 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let source_ml_source2_s5680 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop96(registered_keys.clone(), source_ml_source1_temporary_values81, source_ml_source1_temporary_index81);
                                                                                        match source_ml_source2_s5680 {
                                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop96Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop96Done => {
                                                                                                {
                                                                                                    let witnessed_keys = self::order_java_script_collection_keys(registered_keys.clone());
                                                                                                    {
                                                                                                        let witnessed_units = 0f64;
                                                                                                        {
                                                                                                            let previous_witness = None::<()>;
                                                                                                            {
                                                                                                                let source_ml_source1_temporary_index82 = self::ml::Big::from_i128(0);
                                                                                                                {
                                                                                                                    let source_ml_source2_s5681 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop97(witnessed_units, None::<self::JavaScriptSemanticValue>, witnessed_keys, source_ml_source1_temporary_index82);
                                                                                                                    match source_ml_source2_s5681 {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop97Done => {
                                                                                                                            {
                                                                                                                                let source_ml_source2_s5703 = if !(match (match heap.clone() {
                                                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source2_data_property_value5704) => {
                                                                                                                                        ((source_ml_source2_data_property_value5704.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                                                    }
                                                                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || {
                                                                                                                                    let source_ml_source2_comparison_operand5706 = match heap.clone() {
                                                                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, source_ml_source2_data_property_value5705, _) => {
                                                                                                                                            ((source_ml_source2_data_property_value5705.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_comparison_operand5707 = None::<()>;
                                                                                                                                        !(match (source_ml_source2_comparison_operand5706, source_ml_source2_comparison_operand5707) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                                                                                    }
                                                                                                                                } {
                                                                                                                                    if ({
                                                                                                                                        let source_ml_source2_comparison_operand5709 = match heap.clone() {
                                                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, source_ml_source2_data_property_value5708, _) => {
                                                                                                                                                ((source_ml_source2_data_property_value5708.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        !(match (source_ml_source2_comparison_operand5709, self::ml_string::Text::from_units(vec![114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 49])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                                                                                    } || !({
                                                                                                                                        let source_ml_source2_array_argument5712 = match heap.clone() {
                                                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source2_data_property_value5711) => {
                                                                                                                                                ((source_ml_source2_data_property_value5711.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        match source_ml_source2_array_argument5712 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }
                                                                                                                                    })) || (({
                                                                                                                                        let source_ml_source2_nullish_field_receiver5718 = (match heap.clone() {
                                                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source2_data_property_value5717) => {
                                                                                                                                                ((source_ml_source2_data_property_value5717.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        },);
                                                                                                                                        if match (source_ml_source2_nullish_field_receiver5718.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nullish_field_present5719 = ((source_ml_source2_nullish_field_receiver5718.clone()).0).clone().expect("a proven present value");
                                                                                                                                                { let ml_reference_array174 = source_ml_source2_nullish_field_present5719; ml_reference_array174.borrow().len() as f64 }
                                                                                                                                            }
                                                                                                                                        } else {
                                                                                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
                                                                                                                                        }
                                                                                                                                    }) > 16_384f64) {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98Return(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 115, 104, 97, 114, 101, 100, 32, 97, 103, 101, 110, 116, 32, 109, 111, 100, 101, 108]))
                                                                                                                                    } else {
                                                                                                                                        {
                                                                                                                                            let facts = (match heap.clone() {
                                                                                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source2_data_property_value5720, _, _) => {
                                                                                                                                                    ((source_ml_source2_data_property_value5720.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                                                                                                }
                                                                                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistrationFact>::new())));
                                                                                                                                            {
                                                                                                                                                let registry = (match heap.clone() {
                                                                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source2_data_property_value5721) => {
                                                                                                                                                        ((source_ml_source2_data_property_value5721.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                                                                    }
                                                                                                                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistryEntry>::new())));
                                                                                                                                                {
                                                                                                                                                    let identities = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                                                                                    {
                                                                                                                                                        let previous_key = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                        {
                                                                                                                                                            let units = 0f64;
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source1_temporary_index83 = self::ml::Big::from_i128(0);
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_s5722 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop109(heap.clone(), facts.clone(), identities.clone(), previous_key, units, registry.clone(), source_ml_source1_temporary_index83);
                                                                                                                                                                    match source_ml_source2_s5722 {
                                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Done => {
                                                                                                                                                                            {
                                                                                                                                                                                let ordered = self::order_java_script_collection_keys(identities.clone());
                                                                                                                                                                                {
                                                                                                                                                                                    let previous_identity = None::<()>;
                                                                                                                                                                                    {
                                                                                                                                                                                        let source_ml_source1_temporary_index88 = self::ml::Big::from_i128(0);
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source2_s5723 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop110(None::<self::JavaScriptSemanticValue>, ordered, source_ml_source1_temporary_index88);
                                                                                                                                                                                            match source_ml_source2_s5723 {
                                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop110Done => {
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source1_temporary_index89 = self::ml::Big::from_i128(0);
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source2_s5724 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop111(registry.clone(), facts.clone(), source_ml_source1_temporary_index89);
                                                                                                                                                                                                            match source_ml_source2_s5724 {
                                                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop111Done => {
                                                                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98Next
                                                                                                                                                                                                                }
                                                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop111Return(source_ml_source1_temporary_result111) => {
                                                                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98Return(source_ml_source1_temporary_result111.clone())
                                                                                                                                                                                                                }
                                                                                                                                                                                                            }
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop110Return(source_ml_source1_temporary_result112) => {
                                                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98Return(source_ml_source1_temporary_result112.clone())
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Return(source_ml_source1_temporary_result112_2) => {
                                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98Return(source_ml_source1_temporary_result112_2.clone())
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
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98Next
                                                                                                                                };
                                                                                                                                match source_ml_source2_s5703 {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98Next => {
                                                                                                                                        {
                                                                                                                                            let budget = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                                let source_ml_source2_field5725 = 0f64;
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_field5726 = 0f64;
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_field5727 = 0f64;
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_field5728 = 0f64;
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_field5729 = 0f64;
                                                                                                                                                                (source_ml_source2_field5725, source_ml_source2_field5727, source_ml_source2_field5728, source_ml_source2_field5729, source_ml_source2_field5726)
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }));
                                                                                                                                            {
                                                                                                                                                let source_ml_source1_temporary_values91 = match heap.clone() {
                                                                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value5730, _, _, _, _, _) => {
                                                                                                                                                        source_ml_source2_data_property_value5730.clone()
                                                                                                                                                    }
                                                                                                                                                };
                                                                                                                                                {
                                                                                                                                                    let source_ml_source1_temporary_index91 = self::ml::Big::from_i128(0);
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_s5731 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop99(heap.clone(), budget, source_ml_source1_temporary_values91, source_ml_source1_temporary_index91);
                                                                                                                                                        match source_ml_source2_s5731 {
                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Done => {
                                                                                                                                                                {
                                                                                                                                                                    let marks = std::rc::Rc::new(std::cell::RefCell::new(Vec::<std::rc::Rc<std::cell::RefCell<(f64,)>>>::new()));
                                                                                                                                                                    {
                                                                                                                                                                        let index = 0f64;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_s5732 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop100(heap.clone(), marks.clone(), index);
                                                                                                                                                                            match source_ml_source2_s5732 {
                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop100Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop100Done => {
                                                                                                                                                                                    {
                                                                                                                                                                                        let first = 0f64;
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source2_s5733 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop101(heap.clone(), marks.clone(), first);
                                                                                                                                                                                            match source_ml_source2_s5733 {
                                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop101Done => {
                                                                                                                                                                                                    self::ml_string::Text::from_units(vec![])
                                                                                                                                                                                                }
                                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop101Return(source_ml_source1_temporary_result101) => {
                                                                                                                                                                                                    source_ml_source1_temporary_result101.clone()
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
                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Return(source_ml_source1_temporary_result107) => {
                                                                                                                                                                source_ml_source1_temporary_result107.clone()
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98Return(source_ml_source1_temporary_result108) => {
                                                                                                                                        source_ml_source1_temporary_result108.clone()
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop97Return(source_ml_source1_temporary_result114) => {
                                                                                                                            source_ml_source1_temporary_result114.clone()
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
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95Return(source_ml_source1_temporary_result115) => {
                                                                        source_ml_source1_temporary_result115.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop94Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop94Return(source_ml_source1_temporary_result116_2) => {
                                                            source_ml_source1_temporary_result116_2.clone()
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Return(source_ml_source1_temporary_result116_3) => {
                                    source_ml_source1_temporary_result116_3.clone()
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
            let ml_values175 = {
                (match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source2_data_property_value5734,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value5734.clone(),
                })
                .borrow()
                .clone()
            };
            let ml_callback175 = {
                let replacement = replacement.clone();
                std::rc::Rc::new(move |object: self::JavaScriptOrdinaryObject| -> self::JavaScriptOrdinaryObject { if (match object.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value5761, _, _, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value5761.clone()
            }
            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value5762, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value5762.clone()
            }
            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value5763, _, _, _, _) => {
                source_ml_source2_data_property_value5763.clone()
            }
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value5764, _, _, _, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value5764.clone()
            }
            self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value5765, _, _, _, _, _) => {
                source_ml_source2_data_property_value5765.clone()
            }
            self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value5766, _, _, _, _, _) => {
                source_ml_source2_data_property_value5766.clone()
            }
            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value5767, _, _, _, _, _) => {
                source_ml_source2_data_property_value5767.clone()
            }
            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value5768, _, _, _, _, _) => {
                source_ml_source2_data_property_value5768.clone()
            }
            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value5769, _, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value5769.clone()
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value5770, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value5770.clone()
            }
            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value5771, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value5771.clone()
            }
            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value5772, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value5772.clone()
            }
            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value5773, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value5773.clone()
            }
        }) == (match replacement.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value5774, _, _, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value5774.clone()
            }
            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value5775, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value5775.clone()
            }
            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value5776, _, _, _, _) => {
                source_ml_source2_data_property_value5776.clone()
            }
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value5777, _, _, _, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value5777.clone()
            }
            self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value5778, _, _, _, _, _) => {
                source_ml_source2_data_property_value5778.clone()
            }
            self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value5779, _, _, _, _, _) => {
                source_ml_source2_data_property_value5779.clone()
            }
            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value5780, _, _, _, _, _) => {
                source_ml_source2_data_property_value5780.clone()
            }
            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value5781, _, _, _, _, _) => {
                source_ml_source2_data_property_value5781.clone()
            }
            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value5782, _, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value5782.clone()
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value5783, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value5783.clone()
            }
            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value5784, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value5784.clone()
            }
            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value5785, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value5785.clone()
            }
            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value5786, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value5786.clone()
            }
        }) {
            replacement.clone()
        } else {
            object.clone()
        } })
            };
            ml_values175
                .iter()
                .cloned()
                .enumerate()
                .map(|(ml_index175, ml_value175)| ml_callback175(ml_value175))
                .collect::<Vec<_>>()
        }));
        {
            let source_ml_source2_nominal_field5788 = match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    source_ml_source2_data_property_value5789,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value5789.clone(),
            };
            {
                let source_ml_source2_nominal_field5790 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        _,
                        source_ml_source2_data_property_value5791,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value5791.clone(),
                };
                {
                    let source_ml_source2_nominal_field5792 = match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            _,
                            _,
                            source_ml_source2_data_property_value5793,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value5793.clone(),
                    };
                    {
                        let source_ml_source2_nominal_field5794 = match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value5795,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value5795.clone(),
                        };
                        {
                            let source_ml_source2_nominal_field5796 = match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value5797,
                                    _,
                                ) => source_ml_source2_data_property_value5797.clone(),
                            };
                            {
                                let source_ml_source2_nominal_field5798 = match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value5799,
                                    ) => source_ml_source2_data_property_value5799.clone(),
                                };
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                    objects,
                                    source_ml_source2_nominal_field5790,
                                    source_ml_source2_nominal_field5792,
                                    source_ml_source2_nominal_field5794,
                                    source_ml_source2_nominal_field5796,
                                    source_ml_source2_nominal_field5798,
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
pub fn locate_java_script_ordinary_property(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
) -> std::rc::Rc<
    std::cell::RefCell<(
        std::rc::Rc<
            std::cell::RefCell<(
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
            )>,
        >,
        bool,
        self::ml::Big,
    )>,
> {
    {
        let depth = 0f64;
        {
            let source_ml_source2_s5801 =
                self::source_ml_source1_temporary_locate_java_script_ordinary_property_loop118(
                    heap.clone(),
                    key.clone(),
                    identity.clone(),
                    depth,
                );
            match source_ml_source2_s5801 {
                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Done => {
                    std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_field5803 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_field5804 = self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false);
                            (source_ml_source2_field5804, false, source_ml_source2_field5803)
                        }
                    }))
                }
                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Return(source_ml_source1_temporary_result118) => {
                    source_ml_source1_temporary_result118.clone()
                }
            }
        }
    }
}
