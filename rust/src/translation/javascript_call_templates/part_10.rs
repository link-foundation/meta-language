// Generated from complete js/src/translation/javascript-call-templates.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptCallTemplate;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::JavaScriptSymbolRegistryEntry;
use super::MlTemporaryFindJavaScriptCallTemplateLoop143Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin106;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop107Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop108Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop112Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop121Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop122Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop127Result;
use super::ml;
use super::ml_source_exception;
use super::ml_string;
use super::order_java_script_collection_keys;
use super::source_ml_temporary_find_java_script_call_template_loop143;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop104;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop105;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop107;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop108;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop110;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop111;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop112;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop120;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop121;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop122;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop127;
use super::test_java_script_validated_heap_snapshot;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_heap_error(
    heap: self::JavaScriptOrdinaryHeap,
) -> self::ml_string::Text {
    if self::test_java_script_validated_heap_snapshot(heap.clone()) {
        self::ml_string::Text::from_units(vec![])
    } else if (({
        let ml_reference_array76 = match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                source_ml_source1_data_property_value5915,
                _,
                _,
                _,
                _,
                _,
            ) => source_ml_source1_data_property_value5915.clone(),
        };
        ml_reference_array76.borrow().len() as f64
    }) > 4096f64)
        || (({
            let ml_reference_array77 = match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    _,
                    source_ml_source1_data_property_value5917,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value5917.clone(),
            };
            ml_reference_array77.borrow().len() as f64
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
                let source_ml_temporary_values88 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source1_data_property_value5918,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5918.clone(),
                };
                {
                    let source_ml_temporary_index88 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_s5919 =
                            self::source_ml_temporary_read_java_script_ordinary_heap_error_loop104(
                                heap.clone(),
                                previous,
                                source_ml_temporary_values88,
                                source_ml_temporary_index88,
                            );
                        match source_ml_source1_s5919 {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop104Done(previous_2) => {
                                    {
                                        let previous_3 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_temporary_values89 = match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value5920, _, _, _, _) => {
                                                    source_ml_source1_data_property_value5920.clone()
                                                }
                                            };
                                            {
                                                let source_ml_temporary_index89 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source1_s5921 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop105(heap.clone(), previous_3, source_ml_temporary_values89, source_ml_temporary_index89);
                                                    match source_ml_source1_s5921 {
                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop105Done => {
                                                            {
                                                                let source_ml_source1_s5940 = if !(match (match heap.clone() {
                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value5941, _, _) => {
                                                                        ((source_ml_source1_data_property_value5941.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                    }
                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || {
                                                                    let source_ml_source1_comparison_operand5943 = match heap.clone() {
                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, source_ml_source1_data_property_value5942, _, _, _) => {
                                                                            ((source_ml_source1_data_property_value5942.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_comparison_operand5944 = None::<()>;
                                                                        !(match (source_ml_source1_comparison_operand5943, source_ml_source1_comparison_operand5944) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                    }
                                                                } {
                                                                    if ({
                                                                        let source_ml_source1_comparison_operand5946 = match heap.clone() {
                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, source_ml_source1_data_property_value5945, _, _, _) => {
                                                                                ((source_ml_source1_data_property_value5945.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                            }
                                                                        };
                                                                        !(match (source_ml_source1_comparison_operand5946, self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 99, 108, 97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110, 32, 49])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                    } || !({
                                                                        let source_ml_source1_array_argument5949 = match heap.clone() {
                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value5948, _, _) => {
                                                                                ((source_ml_source1_data_property_value5948.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                            }
                                                                        };
                                                                        match source_ml_source1_array_argument5949 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }
                                                                    })) || (({
                                                                        let source_ml_source1_nullish_field_receiver5955 = (match heap.clone() {
                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value5954, _, _) => {
                                                                                ((source_ml_source1_data_property_value5954.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                            }
                                                                        },);
                                                                        if match (source_ml_source1_nullish_field_receiver5955.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                            {
                                                                                let source_ml_source1_nullish_field_present5956 = ((source_ml_source1_nullish_field_receiver5955.clone()).0).clone().expect("a proven present value");
                                                                                { let ml_reference_array78 = source_ml_source1_nullish_field_present5956; ml_reference_array78.borrow().len() as f64 }
                                                                            }
                                                                        } else {
                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
                                                                        }
                                                                    }) > 16_384f64) {
                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin106::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin106Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        {
                                                                            let registrations = (match heap.clone() {
                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value5957, _, _) => {
                                                                                    ((source_ml_source1_data_property_value5957.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                                }
                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistrationFact>::new())));
                                                                            {
                                                                                let prior = self::ml::Big::from_i128(0);
                                                                                {
                                                                                    let source_ml_temporary_index90 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let source_ml_source1_s5958 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop127(heap.clone(), prior, registrations, source_ml_temporary_index90);
                                                                                        match source_ml_source1_s5958 {
                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop127Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop127Done => {
                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin106::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin106Next
                                                                                            }
                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop127Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop127Return(source_ml_temporary_result127) => {
                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin106::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin106Return(source_ml_temporary_result127.clone())
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin106::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin106Next
                                                                };
                                                                match source_ml_source1_s5940 {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin106::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin106Next => {
                                                                        {
                                                                            let registered_keys = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                            {
                                                                                let source_ml_temporary_values92 = (match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value5959, _, _) => {
                                                                                        ((source_ml_source1_data_property_value5959.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                                    }
                                                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistrationFact>::new())));
                                                                                {
                                                                                    let source_ml_temporary_index92 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let source_ml_source1_s5960 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop107(registered_keys.clone(), source_ml_temporary_values92, source_ml_temporary_index92);
                                                                                        match source_ml_source1_s5960 {
                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop107Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop107Done => {
                                                                                                {
                                                                                                    let witnessed_keys = self::order_java_script_collection_keys(registered_keys.clone());
                                                                                                    {
                                                                                                        let witnessed_units = 0f64;
                                                                                                        {
                                                                                                            let previous_witness = None::<()>;
                                                                                                            {
                                                                                                                let source_ml_temporary_index93 = self::ml::Big::from_i128(0);
                                                                                                                {
                                                                                                                    let source_ml_source1_s5961 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop108(witnessed_units, None::<self::JavaScriptSemanticValue>, witnessed_keys, source_ml_temporary_index93);
                                                                                                                    match source_ml_source1_s5961 {
                                                                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop108Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop108Done => {
                                                                                                                            {
                                                                                                                                let source_ml_source1_s5983 = if !(match (match heap.clone() {
                                                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source1_data_property_value5984) => {
                                                                                                                                        ((source_ml_source1_data_property_value5984.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                                                    }
                                                                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || {
                                                                                                                                    let source_ml_source1_comparison_operand5986 = match heap.clone() {
                                                                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, source_ml_source1_data_property_value5985, _) => {
                                                                                                                                            ((source_ml_source1_data_property_value5985.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source1_comparison_operand5987 = None::<()>;
                                                                                                                                        !(match (source_ml_source1_comparison_operand5986, source_ml_source1_comparison_operand5987) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                                                                                    }
                                                                                                                                } {
                                                                                                                                    if ({
                                                                                                                                        let source_ml_source1_comparison_operand5989 = match heap.clone() {
                                                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, source_ml_source1_data_property_value5988, _) => {
                                                                                                                                                ((source_ml_source1_data_property_value5988.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        !(match (source_ml_source1_comparison_operand5989, self::ml_string::Text::from_units(vec![114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 49])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                                                                                    } || !({
                                                                                                                                        let source_ml_source1_array_argument5992 = match heap.clone() {
                                                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source1_data_property_value5991) => {
                                                                                                                                                ((source_ml_source1_data_property_value5991.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        match source_ml_source1_array_argument5992 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }
                                                                                                                                    })) || (({
                                                                                                                                        let source_ml_source1_nullish_field_receiver5998 = (match heap.clone() {
                                                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source1_data_property_value5997) => {
                                                                                                                                                ((source_ml_source1_data_property_value5997.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        },);
                                                                                                                                        if match (source_ml_source1_nullish_field_receiver5998.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                                            {
                                                                                                                                                let source_ml_source1_nullish_field_present5999 = ((source_ml_source1_nullish_field_receiver5998.clone()).0).clone().expect("a proven present value");
                                                                                                                                                { let ml_reference_array79 = source_ml_source1_nullish_field_present5999; ml_reference_array79.borrow().len() as f64 }
                                                                                                                                            }
                                                                                                                                        } else {
                                                                                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
                                                                                                                                        }
                                                                                                                                    }) > 16_384f64) {
                                                                                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109Return(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 115, 104, 97, 114, 101, 100, 32, 97, 103, 101, 110, 116, 32, 109, 111, 100, 101, 108]))
                                                                                                                                    } else {
                                                                                                                                        {
                                                                                                                                            let facts = (match heap.clone() {
                                                                                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value6000, _, _) => {
                                                                                                                                                    ((source_ml_source1_data_property_value6000.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                                                                                                }
                                                                                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistrationFact>::new())));
                                                                                                                                            {
                                                                                                                                                let registry = (match heap.clone() {
                                                                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source1_data_property_value6001) => {
                                                                                                                                                        ((source_ml_source1_data_property_value6001.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                                                                    }
                                                                                                                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistryEntry>::new())));
                                                                                                                                                {
                                                                                                                                                    let identities = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                                                                                    {
                                                                                                                                                        let previous_key = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                        {
                                                                                                                                                            let units = 0f64;
                                                                                                                                                            {
                                                                                                                                                                let source_ml_temporary_index94 = self::ml::Big::from_i128(0);
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source1_s6002 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop120(heap.clone(), facts.clone(), identities.clone(), previous_key, units, registry.clone(), source_ml_temporary_index94);
                                                                                                                                                                    match source_ml_source1_s6002 {
                                                                                                                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Done => {
                                                                                                                                                                            {
                                                                                                                                                                                let ordered = self::order_java_script_collection_keys(identities.clone());
                                                                                                                                                                                {
                                                                                                                                                                                    let previous_identity = None::<()>;
                                                                                                                                                                                    {
                                                                                                                                                                                        let source_ml_temporary_index99 = self::ml::Big::from_i128(0);
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source1_s6003 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop121(None::<self::JavaScriptSemanticValue>, ordered, source_ml_temporary_index99);
                                                                                                                                                                                            match source_ml_source1_s6003 {
                                                                                                                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop121Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop121Done => {
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_temporary_index100 = self::ml::Big::from_i128(0);
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source1_s6004 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop122(registry.clone(), facts.clone(), source_ml_temporary_index100);
                                                                                                                                                                                                            match source_ml_source1_s6004 {
                                                                                                                                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop122Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop122Done => {
                                                                                                                                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109Next
                                                                                                                                                                                                                }
                                                                                                                                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop122Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop122Return(source_ml_temporary_result122) => {
                                                                                                                                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109Return(source_ml_temporary_result122.clone())
                                                                                                                                                                                                                }
                                                                                                                                                                                                            }
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop121Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop121Return(source_ml_temporary_result123) => {
                                                                                                                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109Return(source_ml_temporary_result123.clone())
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop120Return(source_ml_temporary_result123_2) => {
                                                                                                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109Return(source_ml_temporary_result123_2.clone())
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
                                                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109Next
                                                                                                                                };
                                                                                                                                match source_ml_source1_s5983 {
                                                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109Next => {
                                                                                                                                        {
                                                                                                                                            let budget = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                                let source_ml_source1_field6005 = 0f64;
                                                                                                                                                {
                                                                                                                                                    let source_ml_source1_field6006 = 0f64;
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source1_field6007 = 0f64;
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source1_field6008 = 0f64;
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source1_field6009 = 0f64;
                                                                                                                                                                (source_ml_source1_field6005, source_ml_source1_field6007, source_ml_source1_field6008, source_ml_source1_field6009, source_ml_source1_field6006)
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }));
                                                                                                                                            {
                                                                                                                                                let source_ml_temporary_values102 = match heap.clone() {
                                                                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value6010, _, _, _, _, _) => {
                                                                                                                                                        source_ml_source1_data_property_value6010.clone()
                                                                                                                                                    }
                                                                                                                                                };
                                                                                                                                                {
                                                                                                                                                    let source_ml_temporary_index102 = self::ml::Big::from_i128(0);
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source1_s6011 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop110(heap.clone(), budget, source_ml_temporary_values102, source_ml_temporary_index102);
                                                                                                                                                        match source_ml_source1_s6011 {
                                                                                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Done => {
                                                                                                                                                                {
                                                                                                                                                                    let marks = std::rc::Rc::new(std::cell::RefCell::new(Vec::<std::rc::Rc<std::cell::RefCell<(f64,)>>>::new()));
                                                                                                                                                                    {
                                                                                                                                                                        let index = 0f64;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_s6012 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop111(heap.clone(), marks.clone(), index);
                                                                                                                                                                            match source_ml_source1_s6012 {
                                                                                                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop111Done => {
                                                                                                                                                                                    {
                                                                                                                                                                                        let first = 0f64;
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source1_s6013 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop112(heap.clone(), marks.clone(), first);
                                                                                                                                                                                            match source_ml_source1_s6013 {
                                                                                                                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop112Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop112Done => {
                                                                                                                                                                                                    self::ml_string::Text::from_units(vec![])
                                                                                                                                                                                                }
                                                                                                                                                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop112Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop112Return(source_ml_temporary_result112) => {
                                                                                                                                                                                                    source_ml_temporary_result112.clone()
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
                                                                                                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop110Return(source_ml_temporary_result118) => {
                                                                                                                                                                source_ml_temporary_result118.clone()
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin109Return(source_ml_temporary_result119) => {
                                                                                                                                        source_ml_temporary_result119.clone()
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop108Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop108Return(source_ml_temporary_result125) => {
                                                                                                                            source_ml_temporary_result125.clone()
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
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin106::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin106Return(source_ml_temporary_result126) => {
                                                                        source_ml_temporary_result126.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(source_ml_temporary_result127_2) => {
                                                            source_ml_temporary_result127_2.clone()
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop104Return(source_ml_temporary_result127_3) => {
                                    source_ml_temporary_result127_3.clone()
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
pub fn find_java_script_call_template(
    program: self::JavaScriptCallProgram,
    owner: self::ml::Big,
    ordinal: f64,
) -> f64 {
    {
        let templates = (match program.clone() {
            self::JavaScriptCallProgram::CallProgram(
                _,
                _,
                _,
                source_ml_source1_data_property_value17605,
                _,
                _,
            ) => ((source_ml_source1_data_property_value17605
                .clone()
                .unwrap_or_else(|| {
                    (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>>,)
                }))
            .0)
                .clone(),
        })
        .clone()
        .unwrap_or_else(|| {
            std::rc::Rc::new(std::cell::RefCell::new(
                Vec::<self::JavaScriptCallTemplate>::new(),
            ))
        });
        {
            let low = 0f64;
            {
                let high = {
                    let ml_reference_array80 = templates.clone();
                    ml_reference_array80.borrow().len() as f64
                };
                {
                    let source_ml_source1_s17606 =
                        self::source_ml_temporary_find_java_script_call_template_loop143(
                            owner.clone(),
                            ordinal,
                            templates.clone(),
                            low,
                            high,
                        );
                    match source_ml_source1_s17606 {
                        self::MlTemporaryFindJavaScriptCallTemplateLoop143Result::MlTemporaryFindJavaScriptCallTemplateLoop143Done => {
                            -1f64
                        }
                        self::MlTemporaryFindJavaScriptCallTemplateLoop143Result::MlTemporaryFindJavaScriptCallTemplateLoop143Return(source_ml_temporary_result143) => {
                            source_ml_temporary_result143
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn project_java_script_call_template_function(
    template: self::JavaScriptCallTemplate,
    identity: self::ml::Big,
    captures: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    receiver: self::JavaScriptSemanticValue,
    new_target: Option<self::JavaScriptSemanticValue>,
) -> self::JavaScriptCallFunction {
    {
        let source_ml_temporary_constructor_field145 = match template.clone() {
            self::JavaScriptCallTemplate::CallTemplate(
                _,
                _,
                source_ml_source1_data_property_value17607,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => source_ml_source1_data_property_value17607.clone(),
        };
        {
            let source_ml_temporary_constructor_field146 = (match template.clone() {
                self::JavaScriptCallTemplate::CallTemplate(
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
                    source_ml_source1_data_property_value17608,
                    _,
                ) => ((source_ml_source1_data_property_value17608
                    .clone()
                    .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                .0)
                    .clone(),
            })
            .clone()
            .unwrap_or_else(|| {
                self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108])
            });
            {
                let source_ml_temporary_constructor_field148 = match template.clone() {
                    self::JavaScriptCallTemplate::CallTemplate(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value17609,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value17609,
                };
                {
                    let source_ml_temporary_constructor_field149 = match template.clone() {
                        self::JavaScriptCallTemplate::CallTemplate(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value17610,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17610,
                    };
                    {
                        let source_ml_temporary_constructor_field151 = match template.clone() {
                            self::JavaScriptCallTemplate::CallTemplate(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value17611,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value17611.clone(),
                        };
                        if !(match (new_target.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }) && (((match template.clone() {
                            self::JavaScriptCallTemplate::CallTemplate(
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
                                source_ml_source1_data_property_value17613,
                                _,
                            ) => ((source_ml_source1_data_property_value17613
                                .clone()
                                .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                            .0)
                                .clone(),
                        })
                        .clone()
                        .unwrap_or_else(|| {
                            self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108])
                        })) == (self::ml_string::Text::from_units(vec![
                            108, 101, 120, 105, 99, 97, 108,
                        ]))) {
                            {
                                let source_ml_source1_constructor_field17622 =
                                    Some(new_target.clone().expect("a proven present value"));
                                self::JavaScriptCallFunction::CallFunction(
                                    identity.clone(),
                                    source_ml_temporary_constructor_field145.clone(),
                                    source_ml_temporary_constructor_field146.clone(),
                                    Box::new(receiver.clone()),
                                    source_ml_temporary_constructor_field148,
                                    source_ml_temporary_constructor_field149,
                                    captures.clone(),
                                    source_ml_temporary_constructor_field151.clone(),
                                    Some(source_ml_source1_constructor_field17622),
                                    None::<(Option<self::JavaScriptCallSourceSpan>,)>,
                                )
                            }
                        } else {
                            self::JavaScriptCallFunction::CallFunction(
                                identity.clone(),
                                source_ml_temporary_constructor_field145.clone(),
                                source_ml_temporary_constructor_field146.clone(),
                                Box::new(receiver.clone()),
                                source_ml_temporary_constructor_field148,
                                source_ml_temporary_constructor_field149,
                                captures.clone(),
                                source_ml_temporary_constructor_field151.clone(),
                                None::<(Option<self::JavaScriptSemanticValue>,)>,
                                None::<(Option<self::JavaScriptCallSourceSpan>,)>,
                            )
                        }
                    }
                }
            }
        }
    }
}
