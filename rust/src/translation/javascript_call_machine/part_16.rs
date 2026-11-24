// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
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
use super::source_ml_source3_record_order;
use super::test_java_script_validated_heap_snapshot;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_heap_error(
    heap: self::JavaScriptOrdinaryHeap,
) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
            shared_constructor_field1,
            shared_constructor_field2,
            _,
            _,
            _,
            _,
        ) => {
            if self::test_java_script_validated_heap_snapshot(heap.clone()) {
                self::ml_string::Text::from_units(vec![])
            } else {
                if ((({
                    let ml_reference_array208 = shared_constructor_field1.clone();
                    let ml_reference_projection208 = ml_reference_array208.borrow().len() as f64;
                    ml_reference_projection208
                }) > (4096f64))
                    || (({
                        let ml_reference_array209 = shared_constructor_field2.clone();
                        let ml_reference_projection209 =
                            ml_reference_array209.borrow().len() as f64;
                        ml_reference_projection209
                    }) > (16384f64)))
                {
                    self::ml_string::Text::from_units(vec![
                        111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100,
                        101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114,
                        121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120,
                        112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32,
                        98, 117, 100, 103, 101, 116,
                    ])
                } else {
                    {
                        let previous = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source1_temporary_values77 =
                                shared_constructor_field1.clone();
                            {
                                let source_ml_source1_temporary_index77 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s1268 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop93(heap.clone(), previous.clone(), source_ml_source1_temporary_values77.clone(), source_ml_source1_temporary_index77.clone());
                                    match source_ml_source2_s1268.clone() {
                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop93Done(previous_2) => {
                                            {
                                                let previous_3 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source1_temporary_values78 = shared_constructor_field2.clone();
                                                    {
                                                        let source_ml_source1_temporary_index78 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source2_s1270 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop94(heap.clone(), previous_3.clone(), source_ml_source1_temporary_values78.clone(), source_ml_source1_temporary_index78.clone());
                                                            match source_ml_source2_s1270.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop94Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop94Done => {
                                                                    {
                                                                        let source_ml_source2_s1289 = if (!(match (match heap.clone() {
                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source2_data_property_value1290, _, _) => {
                                                                                ((source_ml_source2_data_property_value1290.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                            }
                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || {
                                                                            let source_ml_source2_comparison_operand1292 = match heap.clone() {
                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, source_ml_source2_data_property_value1291, _, _, _) => {
                                                                                    ((source_ml_source2_data_property_value1291.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_comparison_operand1293 = None::<()>;
                                                                                !(match (source_ml_source2_comparison_operand1292.clone(), source_ml_source2_comparison_operand1293.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                            }
                                                                        }) {
                                                                            if (({
                                                                                let source_ml_source2_comparison_operand1295 = match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, source_ml_source2_data_property_value1294, _, _, _) => {
                                                                                        ((source_ml_source2_data_property_value1294.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                    }
                                                                                };
                                                                                !(match (source_ml_source2_comparison_operand1295.clone(), self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 99, 108, 97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110, 32, 49])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                            } || !({
                                                                                let source_ml_source2_array_argument1298 = match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source2_data_property_value1297, _, _) => {
                                                                                        ((source_ml_source2_data_property_value1297.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                                    }
                                                                                };
                                                                                match source_ml_source2_array_argument1298.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }
                                                                            })) || (({
                                                                                let source_ml_source2_nullish_field_receiver1304 = (match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source2_data_property_value1303, _, _) => {
                                                                                        ((source_ml_source2_data_property_value1303.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                                    }
                                                                                },);
                                                                                if match (source_ml_source2_nullish_field_receiver1304.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                    {
                                                                                        let source_ml_source2_nullish_field_present1305 = ((source_ml_source2_nullish_field_receiver1304.clone()).0).clone().expect("a proven present value");
                                                                                        { let ml_reference_array210 = source_ml_source2_nullish_field_present1305.clone(); let ml_reference_projection210 = ml_reference_array210.borrow().len() as f64; ml_reference_projection210 }
                                                                                    }
                                                                                } else {
                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
                                                                                }
                                                                            }) > (16384f64))) {
                                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 98, 117, 100, 103, 101, 116]))
                                                                            } else {
                                                                                {
                                                                                    let registrations = (match heap.clone() {
                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source2_data_property_value1306, _, _) => {
                                                                                            ((source_ml_source2_data_property_value1306.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                                        }
                                                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistrationFact>::new())));
                                                                                    {
                                                                                        let prior = self::ml::Big::from_i128(0);
                                                                                        {
                                                                                            let source_ml_source1_temporary_index79 = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let source_ml_source2_s1307 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop116(heap.clone(), prior.clone(), registrations.clone(), source_ml_source1_temporary_index79.clone());
                                                                                                match source_ml_source2_s1307.clone() {
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
                                                                        match source_ml_source2_s1289.clone() {
                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin95Next => {
                                                                                {
                                                                                    let registered_keys = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                    {
                                                                                        let source_ml_source1_temporary_values81 = (match heap.clone() {
                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source2_data_property_value1308, _, _) => {
                                                                                                ((source_ml_source2_data_property_value1308.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                                            }
                                                                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistrationFact>::new())));
                                                                                        {
                                                                                            let source_ml_source1_temporary_index81 = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let source_ml_source2_s1309 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop96(registered_keys.clone(), source_ml_source1_temporary_values81.clone(), source_ml_source1_temporary_index81.clone());
                                                                                                match source_ml_source2_s1309.clone() {
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
                                                                                                                            let source_ml_source2_s1310 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop97(witnessed_units, None::<self::JavaScriptSemanticValue>, witnessed_keys.clone(), source_ml_source1_temporary_index82.clone());
                                                                                                                            match source_ml_source2_s1310.clone() {
                                                                                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop97Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop97Done => {
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_s1332 = if (!(match (match heap.clone() {
                                                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source2_data_property_value1333) => {
                                                                                                                                                ((source_ml_source2_data_property_value1333.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || {
                                                                                                                                            let source_ml_source2_comparison_operand1335 = match heap.clone() {
                                                                                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, source_ml_source2_data_property_value1334, _) => {
                                                                                                                                                    ((source_ml_source2_data_property_value1334.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                                }
                                                                                                                                            };
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_comparison_operand1336 = None::<()>;
                                                                                                                                                !(match (source_ml_source2_comparison_operand1335.clone(), source_ml_source2_comparison_operand1336.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                                                                                            }
                                                                                                                                        }) {
                                                                                                                                            if (({
                                                                                                                                                let source_ml_source2_comparison_operand1338 = match heap.clone() {
                                                                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, source_ml_source2_data_property_value1337, _) => {
                                                                                                                                                        ((source_ml_source2_data_property_value1337.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                                    }
                                                                                                                                                };
                                                                                                                                                !(match (source_ml_source2_comparison_operand1338.clone(), self::ml_string::Text::from_units(vec![114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 49])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                                                                                            } || !({
                                                                                                                                                let source_ml_source2_array_argument1341 = match heap.clone() {
                                                                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source2_data_property_value1340) => {
                                                                                                                                                        ((source_ml_source2_data_property_value1340.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                                                                    }
                                                                                                                                                };
                                                                                                                                                match source_ml_source2_array_argument1341.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }
                                                                                                                                            })) || (({
                                                                                                                                                let source_ml_source2_nullish_field_receiver1347 = (match heap.clone() {
                                                                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source2_data_property_value1346) => {
                                                                                                                                                        ((source_ml_source2_data_property_value1346.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
                                                                                                                                                    }
                                                                                                                                                },);
                                                                                                                                                if match (source_ml_source2_nullish_field_receiver1347.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nullish_field_present1348 = ((source_ml_source2_nullish_field_receiver1347.clone()).0).clone().expect("a proven present value");
                                                                                                                                                        { let ml_reference_array211 = source_ml_source2_nullish_field_present1348.clone(); let ml_reference_projection211 = ml_reference_array211.borrow().len() as f64; ml_reference_projection211 }
                                                                                                                                                    }
                                                                                                                                                } else {
                                                                                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
                                                                                                                                                }
                                                                                                                                            }) > (16384f64))) {
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98Return(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 115, 104, 97, 114, 101, 100, 32, 97, 103, 101, 110, 116, 32, 109, 111, 100, 101, 108]))
                                                                                                                                            } else {
                                                                                                                                                {
                                                                                                                                                    let facts = (match heap.clone() {
                                                                                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source2_data_property_value1349, _, _) => {
                                                                                                                                                            ((source_ml_source2_data_property_value1349.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                                                                                                                                                        }
                                                                                                                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistrationFact>::new())));
                                                                                                                                                    {
                                                                                                                                                        let registry = (match heap.clone() {
                                                                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source2_data_property_value1350) => {
                                                                                                                                                                ((source_ml_source2_data_property_value1350.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,))).0).clone()
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
                                                                                                                                                                            let source_ml_source2_s1351 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop109(heap.clone(), facts.clone(), identities.clone(), previous_key.clone(), units, registry.clone(), source_ml_source1_temporary_index83.clone());
                                                                                                                                                                            match source_ml_source2_s1351.clone() {
                                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop109Done => {
                                                                                                                                                                                    {
                                                                                                                                                                                        let ordered = self::order_java_script_collection_keys(identities.clone());
                                                                                                                                                                                        {
                                                                                                                                                                                            let previous_identity = None::<()>;
                                                                                                                                                                                            {
                                                                                                                                                                                                let source_ml_source1_temporary_index88 = self::ml::Big::from_i128(0);
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_source2_s1352 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop110(None::<self::JavaScriptSemanticValue>, ordered.clone(), source_ml_source1_temporary_index88.clone());
                                                                                                                                                                                                    match source_ml_source2_s1352.clone() {
                                                                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop110Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop110Done => {
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let source_ml_source1_temporary_index89 = self::ml::Big::from_i128(0);
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    let source_ml_source2_s1353 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop111(registry.clone(), facts.clone(), source_ml_source1_temporary_index89.clone());
                                                                                                                                                                                                                    match source_ml_source2_s1353.clone() {
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
                                                                                                                                        match source_ml_source2_s1332.clone() {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin98Next => {
                                                                                                                                                {
                                                                                                                                                    let budget = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                                        let source_ml_source2_field1354 = 0f64;
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_field1355 = 0f64;
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_field1356 = 0f64;
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_field1357 = 0f64;
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_field1358 = 0f64;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source3_record_field53 = source_ml_source2_field1354;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source3_record_field54 = source_ml_source2_field1356;
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source3_record_field55 = source_ml_source2_field1357;
                                                                                                                                                                                    {
                                                                                                                                                                                        let source_ml_source3_record_field56 = source_ml_source2_field1358;
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source3_record_field57 = source_ml_source2_field1355;
                                                                                                                                                                                            ((source_ml_source3_record_field53, source_ml_source3_record_field54, source_ml_source3_record_field55, source_ml_source3_record_field56, source_ml_source3_record_field57), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![99f64, 108f64, 111f64, 115f64, 117f64, 114f64, 101f64, 67f64, 97f64, 112f64, 116f64, 117f64, 114f64, 101f64, 67f64, 111f64, 117f64, 110f64, 116f64], vec![99f64, 111f64, 108f64, 108f64, 101f64, 99f64, 116f64, 105f64, 111f64, 110f64, 69f64, 110f64, 116f64, 114f64, 121f64, 67f64, 111f64, 117f64, 110f64, 116f64], vec![99f64, 111f64, 108f64, 108f64, 101f64, 99f64, 116f64, 105f64, 111f64, 110f64, 75f64, 101f64, 121f64, 85f64, 110f64, 105f64, 116f64, 115f64], vec![99f64, 111f64, 108f64, 108f64, 101f64, 99f64, 116f64, 105f64, 111f64, 110f64, 83f64, 116f64, 111f64, 114f64, 101f64, 100f64, 85f64, 110f64, 105f64, 116f64, 115f64], vec![112f64, 114f64, 111f64, 112f64, 101f64, 114f64, 116f64, 121f64, 67f64, 111f64, 117f64, 110f64, 116f64]], vec![0f64, 4f64, 1f64, 2f64, 3f64]))
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }));
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source1_temporary_values91 = shared_constructor_field1.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source1_temporary_index91 = self::ml::Big::from_i128(0);
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_s1360 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop99(heap.clone(), budget.clone(), source_ml_source1_temporary_values91.clone(), source_ml_source1_temporary_index91.clone());
                                                                                                                                                                match source_ml_source2_s1360.clone() {
                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop99Done => {
                                                                                                                                                                        {
                                                                                                                                                                            let marks = std::rc::Rc::new(std::cell::RefCell::new(Vec::<std::rc::Rc<std::cell::RefCell<((f64,), Vec<f64>)>>>::new()));
                                                                                                                                                                            {
                                                                                                                                                                                let index = 0f64;
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source2_s1361 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop100(heap.clone(), marks.clone(), index);
                                                                                                                                                                                    match source_ml_source2_s1361.clone() {
                                                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop100Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop100Done => {
                                                                                                                                                                                            {
                                                                                                                                                                                                let first = 0f64;
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_source2_s1362 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop101(heap.clone(), marks.clone(), first);
                                                                                                                                                                                                    match source_ml_source2_s1362.clone() {
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
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn replace_java_script_ordinary_object(
    heap: self::JavaScriptOrdinaryHeap,
    replacement: self::JavaScriptOrdinaryObject,
) -> self::JavaScriptOrdinaryHeap {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
        ) => {
            let objects = std::rc::Rc::new(std::cell::RefCell::new({
                let ml_values212 = {
                    let ml_reference_value213 = shared_constructor_field1.borrow().clone();
                    ml_reference_value213
                };
                let ml_callback212 = {
                    let replacement = replacement.clone();
                    std::rc::Rc::new(move |object: self::JavaScriptOrdinaryObject| -> self::JavaScriptOrdinaryObject { if ((match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value1390, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1390.clone()
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value1391, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1391.clone()
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value1392, _, _, _, _) => {
                        source_ml_source2_data_property_value1392.clone()
                    }
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value1393, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1393.clone()
                    }
                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value1394, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1394.clone()
                    }
                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value1395, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1395.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value1396, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1396.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value1397, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1397.clone()
                    }
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value1398, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1398.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value1399, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1399.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value1400, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1400.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value1401, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1401.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value1402, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1402.clone()
                    }
                }) == (match replacement.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value1403, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1403.clone()
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value1404, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1404.clone()
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value1405, _, _, _, _) => {
                        source_ml_source2_data_property_value1405.clone()
                    }
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value1406, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1406.clone()
                    }
                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value1407, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1407.clone()
                    }
                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value1408, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1408.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value1409, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1409.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value1410, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1410.clone()
                    }
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value1411, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1411.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value1412, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1412.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value1413, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1413.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value1414, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1414.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value1415, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value1415.clone()
                    }
                })) {
                    replacement.clone()
                } else {
                    object.clone()
                } })
                };
                ml_values212
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(ml_index212, ml_value212)| ml_callback212(ml_value212))
                    .collect::<Vec<_>>()
            }));
            {
                let source_ml_source2_nominal_field1417 = shared_constructor_field1.clone();
                {
                    let source_ml_source2_nominal_field1419 = shared_constructor_field2.clone();
                    {
                        let source_ml_source2_nominal_field1421 = shared_constructor_field3.clone();
                        {
                            let source_ml_source2_nominal_field1423 =
                                shared_constructor_field4.clone();
                            {
                                let source_ml_source2_nominal_field1425 =
                                    shared_constructor_field5.clone();
                                {
                                    let source_ml_source2_nominal_field1427 =
                                        shared_constructor_field6.clone();
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                        objects.clone(),
                                        source_ml_source2_nominal_field1419.clone(),
                                        source_ml_source2_nominal_field1421.clone(),
                                        source_ml_source2_nominal_field1423.clone(),
                                        source_ml_source2_nominal_field1425.clone(),
                                        source_ml_source2_nominal_field1427.clone(),
                                    )
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
pub fn locate_java_script_ordinary_property(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
) -> std::rc::Rc<
    std::cell::RefCell<(
        (
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
            bool,
            self::ml::Big,
        ),
        Vec<f64>,
    )>,
> {
    {
        let depth = 0f64;
        {
            let source_ml_source2_s1430 =
                self::source_ml_source1_temporary_locate_java_script_ordinary_property_loop118(
                    heap.clone(),
                    key.clone(),
                    identity.clone(),
                    depth,
                );
            match source_ml_source2_s1430.clone() {
                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop118Done => {
                    std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_field1432 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_field1433 = self::create_java_script_property_descriptor(&(self::ml_string::Text::from_units(vec![100, 97, 116, 97])), false, false);
                            {
                                let source_ml_source3_record_field58 = source_ml_source2_field1433.clone();
                                {
                                    let source_ml_source3_record_field59 = false;
                                    {
                                        let source_ml_source3_record_field60 = source_ml_source2_field1432.clone();
                                        ((source_ml_source3_record_field58.clone(), source_ml_source3_record_field59, source_ml_source3_record_field60.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![100f64, 101f64, 115f64, 99f64, 114f64, 105f64, 112f64, 116f64, 111f64, 114f64], vec![102f64, 111f64, 117f64, 110f64, 100f64], vec![111f64, 119f64, 110f64, 101f64, 114f64]], vec![1f64, 2f64, 0f64]))
                                    }
                                }
                            }
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
