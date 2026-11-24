// Generated from complete js/src/translation/javascript-collection-call-source.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptPrimitiveWrapperRoots;
use super::JavaScriptSemanticValue;
use super::MlOptional;
use super::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332;
use super::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin335;
use super::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin351;
use super::MlTemporaryCompileJavaScriptObjectCallFunctionLoop304Result;
use super::MlTemporaryCompileJavaScriptObjectCallFunctionLoop305Result;
use super::MlTemporaryCompileJavaScriptObjectCallFunctionLoop310Result;
use super::MlTemporaryCompileJavaScriptStringConversionFunctionJoin323;
use super::MlTemporaryCompileJavaScriptStringConversionFunctionJoin329;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::source_ml_temporary_compile_java_script_object_call_function_loop304;
use super::source_ml_temporary_compile_java_script_object_call_function_loop305;
use super::source_ml_temporary_compile_java_script_object_call_function_loop310;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_object_call_function(identity: self::ml::Big, realm: self::ml::Big, roots: Option<self::JavaScriptPrimitiveWrapperRoots>) -> Option<self::JavaScriptCallFunction> {
    if {
        let source_ml_source1_comparison_operand1542 = {
            let source_ml_source1_optional_chain_receiver1541 = (roots.clone(),);
            if match (source_ml_source1_optional_chain_receiver1541.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                Some({
                    let source_ml_temporary_optional_chain299 = ((source_ml_source1_optional_chain_receiver1541.clone()).0).clone().expect("a proven present value");
                    match source_ml_temporary_optional_chain299.clone() {
                        self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, _, _, _, _, _) => {
                            self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 87, 114, 97, 112, 112, 101, 114, 82, 111, 111, 116, 115])
                        }
                    }
                })
            } else {
                None::<self::ml_string::Text>
            }
        };
        !(match (source_ml_source1_comparison_operand1542.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 87, 114, 97, 112, 112, 101, 114, 82, 111, 111, 116, 115])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
    } {
        None::<self::JavaScriptCallFunction>
    } else {
        {
            let choices = std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new({
                let source_ml_source1_field1548 = {
                    let source_ml_source1_nullish_field_receiver1545 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver1545.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present1546 = ((source_ml_source1_nullish_field_receiver1545.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present1546.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, source_ml_source1_data_property_value1547, _, _, _, _) => {
                                    let source_ml_source1_data_property_value1547 = *source_ml_source1_data_property_value1547;
                                    source_ml_source1_data_property_value1547.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 115, 116, 114, 105, 110, 103, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                };
                (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]), source_ml_source1_field1548.clone())
            })), std::rc::Rc::new(std::cell::RefCell::new({
                let source_ml_source1_field1553 = {
                    let source_ml_source1_nullish_field_receiver1550 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver1550.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present1551 = ((source_ml_source1_nullish_field_receiver1550.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present1551.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, _, source_ml_source1_data_property_value1552, _, _, _) => {
                                    let source_ml_source1_data_property_value1552 = *source_ml_source1_data_property_value1552;
                                    source_ml_source1_data_property_value1552.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 110, 117, 109, 98, 101, 114, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                };
                (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]), source_ml_source1_field1553.clone())
            })), std::rc::Rc::new(std::cell::RefCell::new({
                let source_ml_source1_field1558 = {
                    let source_ml_source1_nullish_field_receiver1555 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver1555.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present1556 = ((source_ml_source1_nullish_field_receiver1555.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present1556.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, _, _, source_ml_source1_data_property_value1557, _, _) => {
                                    let source_ml_source1_data_property_value1557 = *source_ml_source1_data_property_value1557;
                                    source_ml_source1_data_property_value1557.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 98, 111, 111, 108, 101, 97, 110, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                };
                (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]), source_ml_source1_field1558.clone())
            })), std::rc::Rc::new(std::cell::RefCell::new({
                let source_ml_source1_field1563 = {
                    let source_ml_source1_nullish_field_receiver1560 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver1560.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present1561 = ((source_ml_source1_nullish_field_receiver1560.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present1561.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, _, _, _, source_ml_source1_data_property_value1562, _) => {
                                    let source_ml_source1_data_property_value1562 = *source_ml_source1_data_property_value1562;
                                    source_ml_source1_data_property_value1562.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 110, 116, 101, 103, 101, 114, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                };
                (self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116]), source_ml_source1_field1563.clone())
            })), std::rc::Rc::new(std::cell::RefCell::new({
                let source_ml_source1_field1568 = {
                    let source_ml_source1_nullish_field_receiver1565 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver1565.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present1566 = ((source_ml_source1_nullish_field_receiver1565.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present1566.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, _, _, _, _, source_ml_source1_data_property_value1567) => {
                                    let source_ml_source1_data_property_value1567 = *source_ml_source1_data_property_value1567;
                                    source_ml_source1_data_property_value1567.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 115, 121, 109, 98, 111, 108, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                };
                (self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108]), source_ml_source1_field1568.clone())
            }))]));
            {
                let source_ml_temporary_values300 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::MlOptional::Value({
                    let source_ml_source1_nullish_field_receiver1569 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver1569.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present1570 = ((source_ml_source1_nullish_field_receiver1569.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present1570.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(source_ml_source1_data_property_value1571, _, _, _, _, _) => {
                                    let source_ml_source1_data_property_value1571 = *source_ml_source1_data_property_value1571;
                                    source_ml_source1_data_property_value1571.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 111, 98, 106, 101, 99, 116, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                }), self::MlOptional::Value({
                    let source_ml_source1_nullish_field_receiver1572 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver1572.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present1573 = ((source_ml_source1_nullish_field_receiver1572.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present1573.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, source_ml_source1_data_property_value1574, _, _, _, _) => {
                                    let source_ml_source1_data_property_value1574 = *source_ml_source1_data_property_value1574;
                                    source_ml_source1_data_property_value1574.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 115, 116, 114, 105, 110, 103, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                }), self::MlOptional::Value({
                    let source_ml_source1_nullish_field_receiver1575 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver1575.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present1576 = ((source_ml_source1_nullish_field_receiver1575.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present1576.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, _, source_ml_source1_data_property_value1577, _, _, _) => {
                                    let source_ml_source1_data_property_value1577 = *source_ml_source1_data_property_value1577;
                                    source_ml_source1_data_property_value1577.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 110, 117, 109, 98, 101, 114, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                }), self::MlOptional::Value({
                    let source_ml_source1_nullish_field_receiver1578 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver1578.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present1579 = ((source_ml_source1_nullish_field_receiver1578.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present1579.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, _, _, source_ml_source1_data_property_value1580, _, _) => {
                                    let source_ml_source1_data_property_value1580 = *source_ml_source1_data_property_value1580;
                                    source_ml_source1_data_property_value1580.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 98, 111, 111, 108, 101, 97, 110, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                }), self::MlOptional::Value({
                    let source_ml_source1_nullish_field_receiver1581 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver1581.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present1582 = ((source_ml_source1_nullish_field_receiver1581.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present1582.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, _, _, _, source_ml_source1_data_property_value1583, _) => {
                                    let source_ml_source1_data_property_value1583 = *source_ml_source1_data_property_value1583;
                                    source_ml_source1_data_property_value1583.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 110, 116, 101, 103, 101, 114, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                }), self::MlOptional::Value({
                    let source_ml_source1_nullish_field_receiver1584 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver1584.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present1585 = ((source_ml_source1_nullish_field_receiver1584.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present1585.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, _, _, _, _, source_ml_source1_data_property_value1586) => {
                                    let source_ml_source1_data_property_value1586 = *source_ml_source1_data_property_value1586;
                                    source_ml_source1_data_property_value1586.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 115, 121, 109, 98, 111, 108, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                })]));
                {
                    let source_ml_temporary_index300 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_s1587 = self::source_ml_temporary_compile_java_script_object_call_function_loop304(source_ml_temporary_values300.clone(), source_ml_temporary_index300.clone());
                        match source_ml_source1_s1587.clone() {
                            self::MlTemporaryCompileJavaScriptObjectCallFunctionLoop304Result::MlTemporaryCompileJavaScriptObjectCallFunctionLoop304Done => {
                                {
                                    let code = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Argument(1f64, 0f64), self::JavaScriptCallInstruction::Primitive(3f64, self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 111, 98, 106, 101, 99, 116, 32, 118, 97, 108, 117, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))), self::JavaScriptCallInstruction::Branch(3f64, 0f64, 3f64), self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 121, 112, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])))]));
                                    {
                                        let jumps = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                        {
                                            let source_ml_temporary_index302 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source1_s1588 = self::source_ml_temporary_compile_java_script_object_call_function_loop305(code.clone(), jumps.clone(), choices.clone(), source_ml_temporary_index302.clone());
                                                match source_ml_source1_s1588.clone() {
                                                    self::MlTemporaryCompileJavaScriptObjectCallFunctionLoop305Result::MlTemporaryCompileJavaScriptObjectCallFunctionLoop305Done => {
                                                        {
                                                            let source_ml_temporary_compile_java_script_object_call_function_ignored306 = { let ml_method_owner378 = code.clone(); let ml_method_argument378_0 = self::JavaScriptCallInstruction::Constant(5f64, Box::new({
                                                                let source_ml_source1_nullish_field_receiver1589 = (roots.clone(),);
                                                                if match (source_ml_source1_nullish_field_receiver1589.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                    {
                                                                        let source_ml_source1_nullish_field_present1590 = ((source_ml_source1_nullish_field_receiver1589.clone()).0).clone().expect("a proven present value");
                                                                        match source_ml_source1_nullish_field_present1590.clone() {
                                                                            self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(source_ml_source1_data_property_value1591, _, _, _, _, _) => {
                                                                                let source_ml_source1_data_property_value1591 = *source_ml_source1_data_property_value1591;
                                                                                source_ml_source1_data_property_value1591.clone()
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 111, 98, 106, 101, 99, 116, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                                                                }
                                                            })); let ml_inserted = vec![ml_method_argument378_0]; let mut ml_method_values378 = ml_method_owner378.borrow_mut(); let ml_length = ml_method_values378.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values378.extend(ml_inserted); ml_length as f64 };
                                                            {
                                                                let allocation = { let ml_reference_array379 = code.clone(); let ml_reference_projection379 = ml_reference_array379.borrow().len() as f64; ml_reference_projection379 };
                                                                {
                                                                    let source_ml_temporary_compile_java_script_object_call_function_ignored307 = { let ml_method_owner380 = code.clone(); let ml_method_argument380_0 = self::JavaScriptCallInstruction::PrimitiveWrapper(2f64, 1f64, 5f64); let ml_method_argument380_1 = self::JavaScriptCallInstruction::Return(2f64); let ml_inserted = vec![ml_method_argument380_0, ml_method_argument380_1]; let mut ml_method_values380 = ml_method_owner380.borrow_mut(); let ml_length = ml_method_values380.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values380.extend(ml_inserted); ml_length as f64 };
                                                                    {
                                                                        let unchanged = { let ml_reference_array381 = code.clone(); let ml_reference_projection381 = ml_reference_array381.borrow().len() as f64; ml_reference_projection381 };
                                                                        {
                                                                            let source_ml_temporary_compile_java_script_object_call_function_ignored308 = { let ml_method_owner382 = code.clone(); let ml_method_argument382_0 = self::JavaScriptCallInstruction::Return(1f64); let ml_inserted = vec![ml_method_argument382_0]; let mut ml_method_values382 = ml_method_owner382.borrow_mut(); let ml_length = ml_method_values382.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values382.extend(ml_inserted); ml_length as f64 };
                                                                            {
                                                                                let source_ml_temporary_compile_java_script_object_call_function_ignored309 = { let ml_array_owner383 = code.clone(); let ml_array_argument383_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(3f64, unchanged, 3f64); let ml_array_argument383_1: f64 = 2f64; let ml_array_argument383_2: f64 = (2f64 + 1f64); let ml_array_length = ml_array_owner383.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument383_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument383_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner383.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument383_0); } } ml_array_owner383 };
                                                                                {
                                                                                    let source_ml_temporary_index303 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let source_ml_source1_s1592 = self::source_ml_temporary_compile_java_script_object_call_function_loop310(code.clone(), allocation, jumps.clone(), source_ml_temporary_index303.clone());
                                                                                        match source_ml_source1_s1592.clone() {
                                                                                            self::MlTemporaryCompileJavaScriptObjectCallFunctionLoop310Result::MlTemporaryCompileJavaScriptObjectCallFunctionLoop310Done => {
                                                                                                Some({
                                                                                                    let source_ml_source1_constructor_field1596 = self::JavaScriptSemanticValue::Undefined;
                                                                                                    {
                                                                                                        let source_ml_source1_constructor_field1597 = 0f64;
                                                                                                        {
                                                                                                            let source_ml_source1_constructor_field1598 = 6f64;
                                                                                                            {
                                                                                                                let source_ml_source1_constructor_field1599 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                                                self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field1596.clone()), source_ml_source1_constructor_field1597, source_ml_source1_constructor_field1598, source_ml_source1_constructor_field1599.clone(), code.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                })
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
                            self::MlTemporaryCompileJavaScriptObjectCallFunctionLoop304Result::MlTemporaryCompileJavaScriptObjectCallFunctionLoop304Return(source_ml_temporary_result317) => {
                                source_ml_temporary_result317.clone()
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
pub fn test_java_script_string_conversion_operation(operation: &self::ml_string::Text) -> bool {
    self::ml_array::includes(&{ let ml_reference_value384 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 46, 116, 111, 83, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 46, 118, 97, 108, 117, 101, 79, 102]), self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 46, 116, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101]), self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 46, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110])]))).borrow().clone(); ml_reference_value384 }, operation, 0f64, false, |a, b| a == b)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_string_conversion_function(identity: self::ml::Big, realm: self::ml::Big, operation: &self::ml_string::Text) -> Option<self::JavaScriptCallFunction> {
    if !(self::test_java_script_string_conversion_operation(operation)) {
        None::<self::JavaScriptCallFunction>
    } else {
        {
            let code = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new()));
            {
                let source_ml_source1_s1674 = if ((*operation) == (self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108]))) {
                    {
                        let source_ml_temporary_compile_java_script_string_conversion_function_ignored324 = { let ml_method_owner385 = code.clone(); let ml_method_argument385_0 = self::JavaScriptCallInstruction::Argument(1f64, 0f64); let ml_method_argument385_1 = self::JavaScriptCallInstruction::Constant(2f64, Box::new(self::JavaScriptSemanticValue::Undefined)); let ml_method_argument385_2 = self::JavaScriptCallInstruction::StrictEqual(3f64, 1f64, 2f64); let ml_method_argument385_3 = self::JavaScriptCallInstruction::Branch(3f64, 5f64, 4f64); let ml_method_argument385_4 = self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_method_argument385_5 = self::JavaScriptCallInstruction::SymbolAllocate(2f64, 1f64); let ml_method_argument385_6 = self::JavaScriptCallInstruction::Return(2f64); let ml_inserted = vec![ml_method_argument385_0, ml_method_argument385_1, ml_method_argument385_2, ml_method_argument385_3, ml_method_argument385_4, ml_method_argument385_5, ml_method_argument385_6]; let mut ml_method_values385 = ml_method_owner385.borrow_mut(); let ml_length = ml_method_values385.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values385.extend(ml_inserted); ml_length as f64 };
                        self::MlTemporaryCompileJavaScriptStringConversionFunctionJoin323::MlTemporaryCompileJavaScriptStringConversionFunctionJoin323Next
                    }
                } else {
                    if ((*operation) == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))) {
                        {
                            let source_ml_temporary_compile_java_script_string_conversion_function_ignored325 = { let ml_method_owner386 = code.clone(); let ml_method_argument386_0 = self::JavaScriptCallInstruction::ArgumentCount(1f64); let ml_method_argument386_1 = self::JavaScriptCallInstruction::Constant(2f64, Box::new(self::JavaScriptSemanticValue::Number(0f64))); let ml_method_argument386_2 = self::JavaScriptCallInstruction::NumberBinary(3f64, 1f64, 2f64, self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])); let ml_method_argument386_3 = self::JavaScriptCallInstruction::Branch(3f64, 14f64, 4f64); let ml_method_argument386_4 = self::JavaScriptCallInstruction::Argument(1f64, 0f64); let ml_method_argument386_5 = self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 121, 112, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_method_argument386_6 = self::JavaScriptCallInstruction::Constant(3f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![115f64, 121f64, 109f64, 98f64, 111f64, 108f64]))))); let ml_method_argument386_7 = self::JavaScriptCallInstruction::StrictEqual(4f64, 2f64, 3f64); let ml_method_argument386_8 = self::JavaScriptCallInstruction::Branch(4f64, 11f64, 9f64); let ml_method_argument386_9 = self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_method_argument386_10 = self::JavaScriptCallInstruction::Return(1f64); let ml_method_argument386_11 = self::JavaScriptCallInstruction::SymbolDescription(2f64, 1f64); let ml_method_argument386_12 = self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 32, 83, 121, 109, 98, 111, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 105, 118, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64]))); let ml_method_argument386_13 = self::JavaScriptCallInstruction::Return(1f64); let ml_method_argument386_14 = self::JavaScriptCallInstruction::Constant(1f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))))); let ml_method_argument386_15 = self::JavaScriptCallInstruction::Return(1f64); let ml_inserted = vec![ml_method_argument386_0, ml_method_argument386_1, ml_method_argument386_2, ml_method_argument386_3, ml_method_argument386_4, ml_method_argument386_5, ml_method_argument386_6, ml_method_argument386_7, ml_method_argument386_8, ml_method_argument386_9, ml_method_argument386_10, ml_method_argument386_11, ml_method_argument386_12, ml_method_argument386_13, ml_method_argument386_14, ml_method_argument386_15]; let mut ml_method_values386 = ml_method_owner386.borrow_mut(); let ml_length = ml_method_values386.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values386.extend(ml_inserted); ml_length as f64 };
                            self::MlTemporaryCompileJavaScriptStringConversionFunctionJoin323::MlTemporaryCompileJavaScriptStringConversionFunctionJoin323Next
                        }
                    } else {
                        {
                            let source_ml_temporary_compile_java_script_string_conversion_function_ignored326 = { let ml_method_owner387 = code.clone(); let ml_method_argument387_0 = self::JavaScriptCallInstruction::ObjectObservation(1f64, self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 46, 118, 97, 108, 117, 101, 79, 102]), 0f64, 0f64); let ml_inserted = vec![ml_method_argument387_0]; let mut ml_method_values387 = ml_method_owner387.borrow_mut(); let ml_length = ml_method_values387.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values387.extend(ml_inserted); ml_length as f64 };
                            if (((*operation) == (self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 46, 118, 97, 108, 117, 101, 79, 102]))) || ((*operation) == (self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 46, 116, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101])))) {
                                {
                                    let source_ml_temporary_compile_java_script_string_conversion_function_ignored327 = { let ml_method_owner388 = code.clone(); let ml_method_argument388_0 = self::JavaScriptCallInstruction::Return(1f64); let ml_inserted = vec![ml_method_argument388_0]; let mut ml_method_values388 = ml_method_owner388.borrow_mut(); let ml_length = ml_method_values388.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values388.extend(ml_inserted); ml_length as f64 };
                                    self::MlTemporaryCompileJavaScriptStringConversionFunctionJoin323::MlTemporaryCompileJavaScriptStringConversionFunctionJoin323Next
                                }
                            } else {
                                {
                                    let source_ml_temporary_compile_java_script_string_conversion_function_ignored328 = { let ml_method_owner389 = code.clone(); let ml_method_argument389_0 = self::JavaScriptCallInstruction::SymbolDescription(2f64, 1f64); let ml_inserted = vec![ml_method_argument389_0]; let mut ml_method_values389 = ml_method_owner389.borrow_mut(); let ml_length = ml_method_values389.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values389.extend(ml_inserted); ml_length as f64 };
                                    {
                                        let source_ml_source1_s1675 = if ((*operation) == (self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 46, 116, 111, 83, 116, 114, 105, 110, 103]))) {
                                            {
                                                let source_ml_temporary_compile_java_script_string_conversion_function_ignored331 = { let ml_method_owner390 = code.clone(); let ml_method_argument390_0 = self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 32, 83, 121, 109, 98, 111, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 105, 118, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64]))); let ml_inserted = vec![ml_method_argument390_0]; let mut ml_method_values390 = ml_method_owner390.borrow_mut(); let ml_length = ml_method_values390.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values390.extend(ml_inserted); ml_length as f64 };
                                                self::MlTemporaryCompileJavaScriptStringConversionFunctionJoin329::MlTemporaryCompileJavaScriptStringConversionFunctionJoin329Next
                                            }
                                        } else {
                                            self::MlTemporaryCompileJavaScriptStringConversionFunctionJoin329::MlTemporaryCompileJavaScriptStringConversionFunctionJoin329Next
                                        };
                                        match source_ml_source1_s1675.clone() {
                                            self::MlTemporaryCompileJavaScriptStringConversionFunctionJoin329::MlTemporaryCompileJavaScriptStringConversionFunctionJoin329Next => {
                                                {
                                                    let source_ml_temporary_compile_java_script_string_conversion_function_ignored330 = { let ml_method_owner391 = code.clone(); let ml_method_argument391_0 = self::JavaScriptCallInstruction::Return(2f64); let ml_inserted = vec![ml_method_argument391_0]; let mut ml_method_values391 = ml_method_owner391.borrow_mut(); let ml_length = ml_method_values391.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values391.extend(ml_inserted); ml_length as f64 };
                                                    self::MlTemporaryCompileJavaScriptStringConversionFunctionJoin323::MlTemporaryCompileJavaScriptStringConversionFunctionJoin323Next
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                };
                match source_ml_source1_s1674.clone() {
                    self::MlTemporaryCompileJavaScriptStringConversionFunctionJoin323::MlTemporaryCompileJavaScriptStringConversionFunctionJoin323Next => {
                        Some({
                            let source_ml_source1_constructor_field1679 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_constructor_field1680 = 0f64;
                                {
                                    let source_ml_source1_constructor_field1681 = 5f64;
                                    {
                                        let source_ml_source1_constructor_field1682 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                        self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field1679.clone()), source_ml_source1_constructor_field1680, source_ml_source1_constructor_field1681, source_ml_source1_constructor_field1682.clone(), code.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                    }
                                }
                            }
                        })
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_predicate_names() -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![105, 115, 78, 97, 78]), self::ml_string::Text::from_units(vec![105, 115, 70, 105, 110, 105, 116, 101]), self::ml_string::Text::from_units(vec![105, 115, 73, 110, 116, 101, 103, 101, 114]), self::ml_string::Text::from_units(vec![105, 115, 83, 97, 102, 101, 73, 110, 116, 101, 103, 101, 114])]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_math_method_names() -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![102, 114, 111, 117, 110, 100]), self::ml_string::Text::from_units(vec![99, 108, 122, 51, 50]), self::ml_string::Text::from_units(vec![105, 109, 117, 108]), self::ml_string::Text::from_units(vec![97, 98, 115]), self::ml_string::Text::from_units(vec![102, 108, 111, 111, 114]), self::ml_string::Text::from_units(vec![99, 101, 105, 108]), self::ml_string::Text::from_units(vec![116, 114, 117, 110, 99]), self::ml_string::Text::from_units(vec![114, 111, 117, 110, 100]), self::ml_string::Text::from_units(vec![115, 105, 103, 110]), self::ml_string::Text::from_units(vec![115, 113, 114, 116]), self::ml_string::Text::from_units(vec![112, 111, 119]), self::ml_string::Text::from_units(vec![109, 105, 110]), self::ml_string::Text::from_units(vec![109, 97, 120])]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_numeric_intrinsic_operation(operation: self::ml_string::Text) -> bool {
    ((self::ml_array::includes(&{ let ml_reference_value392 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![66, 111, 111, 108, 101, 97, 110]), self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116]), self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 46, 97, 115, 73, 110, 116, 78]), self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 46, 97, 115, 85, 105, 110, 116, 78]), self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 46, 116, 111, 83, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 46, 112, 97, 114, 115, 101, 73, 110, 116]), self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 46, 112, 97, 114, 115, 101, 70, 108, 111, 97, 116]), self::ml_string::Text::from_units(vec![105, 115, 78, 97, 78]), self::ml_string::Text::from_units(vec![105, 115, 70, 105, 110, 105, 116, 101])]))).borrow().clone(); ml_reference_value392 }, &operation.clone(), 0f64, false, |a, b| a == b) || { let ml_values393 = { let ml_reference_value394 = (self::read_java_script_number_predicate_names()).borrow().clone(); ml_reference_value394 }; let ml_callback393 = { let operation = operation.clone(); std::rc::Rc::new(move |name: self::ml_string::Text| -> bool { (operation == ((self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 46])).concat(&name))) }) }; ml_values393.iter().cloned().enumerate().any(|(ml_index393, ml_value393)| ml_callback393(ml_value393)) }) || { let ml_values395 = { let ml_reference_value396 = (self::read_java_script_math_method_names()).borrow().clone(); ml_reference_value396 }; let ml_callback395 = { let operation = operation.clone(); std::rc::Rc::new(move |name: self::ml_string::Text| -> bool { (operation == ((self::ml_string::Text::from_units(vec![77, 97, 116, 104, 46])).concat(&name))) }) }; ml_values395.iter().cloned().enumerate().any(|(ml_index395, ml_value395)| ml_callback395(ml_value395)) })
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_numeric_intrinsic_function(identity: self::ml::Big, realm: self::ml::Big, operation: &self::ml_string::Text) -> Option<self::JavaScriptCallFunction> {
    if !(self::test_java_script_numeric_intrinsic_operation(operation.clone())) {
        None::<self::JavaScriptCallFunction>
    } else {
        {
            let code = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new()));
            {
                let source_ml_source1_s1686 = if ((*operation) == (self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 46, 116, 111, 83, 116, 114, 105, 110, 103]))) {
                    {
                        let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored333 = { let ml_method_owner397 = code.clone(); let ml_method_argument397_0 = self::JavaScriptCallInstruction::ObjectObservation(1f64, self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 46, 118, 97, 108, 117, 101, 79, 102]), 0f64, 0f64); let ml_method_argument397_1 = self::JavaScriptCallInstruction::Argument(2f64, 0f64); let ml_method_argument397_2 = self::JavaScriptCallInstruction::Constant(3f64, Box::new(self::JavaScriptSemanticValue::Undefined)); let ml_method_argument397_3 = self::JavaScriptCallInstruction::StrictEqual(4f64, 2f64, 3f64); let ml_method_argument397_4 = self::JavaScriptCallInstruction::Branch(4f64, 5f64, 7f64); let ml_method_argument397_5 = self::JavaScriptCallInstruction::Constant(2f64, Box::new(self::JavaScriptSemanticValue::Number(10f64))); let ml_method_argument397_6 = self::JavaScriptCallInstruction::Jump(8f64); let ml_method_argument397_7 = self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64]))); let ml_method_argument397_8 = self::JavaScriptCallInstruction::Primitive(5f64, self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 97, 100, 105, 120, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64]))); let ml_method_argument397_9 = self::JavaScriptCallInstruction::Return(5f64); let ml_inserted = vec![ml_method_argument397_0, ml_method_argument397_1, ml_method_argument397_2, ml_method_argument397_3, ml_method_argument397_4, ml_method_argument397_5, ml_method_argument397_6, ml_method_argument397_7, ml_method_argument397_8, ml_method_argument397_9]; let mut ml_method_values397 = ml_method_owner397.borrow_mut(); let ml_length = ml_method_values397.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values397.extend(ml_inserted); ml_length as f64 };
                        self::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332Next
                    }
                } else {
                    if (((*operation) == (self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 46, 112, 97, 114, 115, 101, 73, 110, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 46, 112, 97, 114, 115, 101, 70, 108, 111, 97, 116])))) {
                        {
                            let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored334 = { let ml_method_owner398 = code.clone(); let ml_method_argument398_0 = self::JavaScriptCallInstruction::Argument(1f64, 0f64); let ml_method_argument398_1 = self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_inserted = vec![ml_method_argument398_0, ml_method_argument398_1]; let mut ml_method_values398 = ml_method_owner398.borrow_mut(); let ml_length = ml_method_values398.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values398.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_source1_s1687 = if ((*operation) == (self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 46, 112, 97, 114, 115, 101, 73, 110, 116]))) {
                                    {
                                        let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored337 = { let ml_method_owner399 = code.clone(); let ml_method_argument399_0 = self::JavaScriptCallInstruction::Argument(2f64, 1f64); let ml_method_argument399_1 = self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64]))); let ml_method_argument399_2 = self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![112, 97, 114, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 116, 101, 120, 116, 32, 112, 114, 101, 102, 105, 120]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64]))); let ml_inserted = vec![ml_method_argument399_0, ml_method_argument399_1, ml_method_argument399_2]; let mut ml_method_values399 = ml_method_owner399.borrow_mut(); let ml_length = ml_method_values399.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values399.extend(ml_inserted); ml_length as f64 };
                                        self::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin335::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin335Next
                                    }
                                } else {
                                    {
                                        let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored338 = { let ml_method_owner400 = code.clone(); let ml_method_argument400_0 = self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![112, 97, 114, 115, 101, 32, 102, 108, 111, 97, 116, 32, 116, 101, 120, 116, 32, 112, 114, 101, 102, 105, 120]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_inserted = vec![ml_method_argument400_0]; let mut ml_method_values400 = ml_method_owner400.borrow_mut(); let ml_length = ml_method_values400.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values400.extend(ml_inserted); ml_length as f64 };
                                        self::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin335::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin335Next
                                    }
                                };
                                match source_ml_source1_s1687.clone() {
                                    self::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin335::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin335Next => {
                                        {
                                            let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored336 = { let ml_method_owner401 = code.clone(); let ml_method_argument401_0 = self::JavaScriptCallInstruction::Return(1f64); let ml_inserted = vec![ml_method_argument401_0]; let mut ml_method_values401 = ml_method_owner401.borrow_mut(); let ml_length = ml_method_values401.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values401.extend(ml_inserted); ml_length as f64 };
                                            self::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332Next
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if (((*operation) == (self::ml_string::Text::from_units(vec![105, 115, 78, 97, 78]))) || ((*operation) == (self::ml_string::Text::from_units(vec![105, 115, 70, 105, 110, 105, 116, 101])))) {
                            {
                                let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored339 = { let ml_method_owner402 = code.clone(); let ml_method_argument402_0 = self::JavaScriptCallInstruction::Argument(1f64, 0f64); let ml_method_argument402_1 = self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_method_argument402_2 = self::JavaScriptCallInstruction::Primitive(1f64, if ((*operation) == (self::ml_string::Text::from_units(vec![105, 115, 78, 97, 78]))) {
                                    self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 78, 97, 78, 32, 110, 117, 109, 98, 101, 114])
                                } else {
                                    self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 102, 105, 110, 105, 116, 101, 32, 110, 117, 109, 98, 101, 114])
                                }, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_method_argument402_3 = self::JavaScriptCallInstruction::Return(1f64); let ml_inserted = vec![ml_method_argument402_0, ml_method_argument402_1, ml_method_argument402_2, ml_method_argument402_3]; let mut ml_method_values402 = ml_method_owner402.borrow_mut(); let ml_length = ml_method_values402.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values402.extend(ml_inserted); ml_length as f64 };
                                self::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332Next
                            }
                        } else {
                            if ((*operation) == (self::ml_string::Text::from_units(vec![66, 111, 111, 108, 101, 97, 110]))) {
                                {
                                    let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored340 = { let ml_method_owner403 = code.clone(); let ml_method_argument403_0 = self::JavaScriptCallInstruction::Argument(1f64, 0f64); let ml_method_argument403_1 = self::JavaScriptCallInstruction::Branch(1f64, 2f64, 4f64); let ml_method_argument403_2 = self::JavaScriptCallInstruction::Constant(2f64, Box::new(self::JavaScriptSemanticValue::Boolean(true))); let ml_method_argument403_3 = self::JavaScriptCallInstruction::Return(2f64); let ml_method_argument403_4 = self::JavaScriptCallInstruction::Constant(2f64, Box::new(self::JavaScriptSemanticValue::Boolean(false))); let ml_method_argument403_5 = self::JavaScriptCallInstruction::Return(2f64); let ml_inserted = vec![ml_method_argument403_0, ml_method_argument403_1, ml_method_argument403_2, ml_method_argument403_3, ml_method_argument403_4, ml_method_argument403_5]; let mut ml_method_values403 = ml_method_owner403.borrow_mut(); let ml_length = ml_method_values403.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values403.extend(ml_inserted); ml_length as f64 };
                                    self::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332Next
                                }
                            } else {
                                if ((*operation) == (self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116]))) {
                                    {
                                        let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored341 = { let ml_method_owner404 = code.clone(); let ml_method_argument404_0 = self::JavaScriptCallInstruction::Argument(1f64, 0f64); let ml_method_argument404_1 = self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_method_argument404_2 = self::JavaScriptCallInstruction::Return(1f64); let ml_inserted = vec![ml_method_argument404_0, ml_method_argument404_1, ml_method_argument404_2]; let mut ml_method_values404 = ml_method_owner404.borrow_mut(); let ml_length = ml_method_values404.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values404.extend(ml_inserted); ml_length as f64 };
                                        self::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332Next
                                    }
                                } else {
                                    if (((*operation) == (self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 46, 97, 115, 73, 110, 116, 78]))) || ((*operation) == (self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 46, 97, 115, 85, 105, 110, 116, 78])))) {
                                        {
                                            let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored342 = { let ml_method_owner405 = code.clone(); let ml_method_argument405_0 = self::JavaScriptCallInstruction::Argument(1f64, 0f64); let ml_method_argument405_1 = self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_method_argument405_2 = self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 105, 110, 100, 101, 120]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_method_argument405_3 = self::JavaScriptCallInstruction::Argument(2f64, 1f64); let ml_method_argument405_4 = self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 101, 103, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64]))); let ml_method_argument405_5 = self::JavaScriptCallInstruction::Primitive(3f64, if ((*operation) == (self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 46, 97, 115, 73, 110, 116, 78]))) {
                                                self::ml_string::Text::from_units(vec![119, 114, 97, 112, 32, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114])
                                            } else {
                                                self::ml_string::Text::from_units(vec![119, 114, 97, 112, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114])
                                            }, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64]))); let ml_method_argument405_6 = self::JavaScriptCallInstruction::Return(3f64); let ml_inserted = vec![ml_method_argument405_0, ml_method_argument405_1, ml_method_argument405_2, ml_method_argument405_3, ml_method_argument405_4, ml_method_argument405_5, ml_method_argument405_6]; let mut ml_method_values405 = ml_method_owner405.borrow_mut(); let ml_length = ml_method_values405.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values405.extend(ml_inserted); ml_length as f64 };
                                            self::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332Next
                                        }
                                    } else {
                                        if ((*operation) == (self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114]))) {
                                            {
                                                let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored343 = { let ml_method_owner406 = code.clone(); let ml_method_argument406_0 = self::JavaScriptCallInstruction::ArgumentCount(1f64); let ml_method_argument406_1 = self::JavaScriptCallInstruction::Constant(2f64, Box::new(self::JavaScriptSemanticValue::Number(0f64))); let ml_method_argument406_2 = self::JavaScriptCallInstruction::NumberBinary(3f64, 1f64, 2f64, self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])); let ml_method_argument406_3 = self::JavaScriptCallInstruction::Branch(3f64, 7f64, 4f64); let ml_method_argument406_4 = self::JavaScriptCallInstruction::Argument(1f64, 0f64); let ml_method_argument406_5 = self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_method_argument406_6 = self::JavaScriptCallInstruction::Return(1f64); let ml_method_argument406_7 = self::JavaScriptCallInstruction::Return(2f64); let ml_inserted = vec![ml_method_argument406_0, ml_method_argument406_1, ml_method_argument406_2, ml_method_argument406_3, ml_method_argument406_4, ml_method_argument406_5, ml_method_argument406_6, ml_method_argument406_7]; let mut ml_method_values406 = ml_method_owner406.borrow_mut(); let ml_length = ml_method_values406.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values406.extend(ml_inserted); ml_length as f64 };
                                                self::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332Next
                                            }
                                        } else {
                                            if operation.starts_with(&self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 46])) {
                                                {
                                                    let primitive = if ((*operation) == (self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 46, 105, 115, 78, 97, 78]))) {
                                                        self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 78, 97, 78, 32, 110, 117, 109, 98, 101, 114])
                                                    } else {
                                                        if ((*operation) == (self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 46, 105, 115, 70, 105, 110, 105, 116, 101]))) {
                                                            self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 102, 105, 110, 105, 116, 101, 32, 110, 117, 109, 98, 101, 114])
                                                        } else {
                                                            if ((*operation) == (self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 46, 105, 115, 73, 110, 116, 101, 103, 101, 114]))) {
                                                                self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110, 117, 109, 98, 101, 114])
                                                            } else {
                                                                self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 115, 97, 102, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110, 117, 109, 98, 101, 114])
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored344 = { let ml_method_owner407 = code.clone(); let ml_method_argument407_0 = self::JavaScriptCallInstruction::Argument(1f64, 0f64); let ml_method_argument407_1 = self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 121, 112, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_method_argument407_2 = self::JavaScriptCallInstruction::Constant(3f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![110f64, 117f64, 109f64, 98f64, 101f64, 114f64]))))); let ml_method_argument407_3 = self::JavaScriptCallInstruction::StrictEqual(4f64, 2f64, 3f64); let ml_method_argument407_4 = self::JavaScriptCallInstruction::Branch(4f64, 5f64, 7f64); let ml_method_argument407_5 = self::JavaScriptCallInstruction::Primitive(5f64, primitive.clone(), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_method_argument407_6 = self::JavaScriptCallInstruction::Return(5f64); let ml_method_argument407_7 = self::JavaScriptCallInstruction::Constant(5f64, Box::new(self::JavaScriptSemanticValue::Boolean(false))); let ml_method_argument407_8 = self::JavaScriptCallInstruction::Return(5f64); let ml_inserted = vec![ml_method_argument407_0, ml_method_argument407_1, ml_method_argument407_2, ml_method_argument407_3, ml_method_argument407_4, ml_method_argument407_5, ml_method_argument407_6, ml_method_argument407_7, ml_method_argument407_8]; let mut ml_method_values407 = ml_method_owner407.borrow_mut(); let ml_length = ml_method_values407.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values407.extend(ml_inserted); ml_length as f64 };
                                                        self::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332Next
                                                    }
                                                }
                                            } else {
                                                if (((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 116, 104, 46, 109, 105, 110]))) || ((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 116, 104, 46, 109, 97, 120])))) {
                                                    {
                                                        let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored345 = { let ml_method_owner408 = code.clone(); let ml_method_argument408_0 = self::JavaScriptCallInstruction::Constant(1f64, Box::new(self::JavaScriptSemanticValue::Number(if ((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 116, 104, 46, 109, 105, 110]))) {
                                                            f64::INFINITY
                                                        } else {
                                                            f64::NEG_INFINITY
                                                        }))); let ml_method_argument408_1 = self::JavaScriptCallInstruction::ArgumentCount(5f64); let ml_method_argument408_2 = self::JavaScriptCallInstruction::Constant(4f64, Box::new(self::JavaScriptSemanticValue::Number(0f64))); let ml_method_argument408_3 = self::JavaScriptCallInstruction::Constant(8f64, Box::new(self::JavaScriptSemanticValue::Number(1f64))); let ml_inserted = vec![ml_method_argument408_0, ml_method_argument408_1, ml_method_argument408_2, ml_method_argument408_3]; let mut ml_method_values408 = ml_method_owner408.borrow_mut(); let ml_length = ml_method_values408.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values408.extend(ml_inserted); ml_length as f64 };
                                                        {
                                                            let loop_ = { let ml_reference_array409 = code.clone(); let ml_reference_projection409 = ml_reference_array409.borrow().len() as f64; ml_reference_projection409 };
                                                            {
                                                                let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored346 = { let ml_method_owner410 = code.clone(); let ml_method_argument410_0 = self::JavaScriptCallInstruction::NumberBinary(6f64, 4f64, 5f64, self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110])); let ml_inserted = vec![ml_method_argument410_0]; let mut ml_method_values410 = ml_method_owner410.borrow_mut(); let ml_length = ml_method_values410.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values410.extend(ml_inserted); ml_length as f64 };
                                                                {
                                                                    let branch = { let ml_reference_array411 = code.clone(); let ml_reference_projection411 = ml_reference_array411.borrow().len() as f64; ml_reference_projection411 };
                                                                    {
                                                                        let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored347 = { let ml_method_owner412 = code.clone(); let ml_method_argument412_0 = self::JavaScriptCallInstruction::Branch(6f64, (branch + 1f64), 0f64); let ml_method_argument412_1 = self::JavaScriptCallInstruction::ArgumentAt(7f64, 4f64); let ml_method_argument412_2 = self::JavaScriptCallInstruction::Primitive(7f64, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![7f64]))); let ml_method_argument412_3 = self::JavaScriptCallInstruction::Primitive(1f64, if ((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 116, 104, 46, 109, 105, 110]))) {
                                                                            self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 105, 110, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114])
                                                                        } else {
                                                                            self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 97, 120, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114])
                                                                        }, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 7f64]))); let ml_method_argument412_4 = self::JavaScriptCallInstruction::NumberBinary(4f64, 4f64, 8f64, self::ml_string::Text::from_units(vec![97, 100, 100])); let ml_method_argument412_5 = self::JavaScriptCallInstruction::Jump(loop_); let ml_inserted = vec![ml_method_argument412_0, ml_method_argument412_1, ml_method_argument412_2, ml_method_argument412_3, ml_method_argument412_4, ml_method_argument412_5]; let mut ml_method_values412 = ml_method_owner412.borrow_mut(); let ml_length = ml_method_values412.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values412.extend(ml_inserted); ml_length as f64 };
                                                                        {
                                                                            let end = { let ml_reference_array413 = code.clone(); let ml_reference_projection413 = ml_reference_array413.borrow().len() as f64; ml_reference_projection413 };
                                                                            {
                                                                                let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored348 = { let ml_array_owner414 = code.clone(); let ml_array_argument414_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(6f64, (branch + 1f64), end); let ml_array_argument414_1: f64 = branch; let ml_array_argument414_2: f64 = (branch + 1f64); let ml_array_length = ml_array_owner414.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument414_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument414_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner414.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument414_0); } } ml_array_owner414 };
                                                                                {
                                                                                    let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored349 = { let ml_method_owner415 = code.clone(); let ml_method_argument415_0 = self::JavaScriptCallInstruction::Return(1f64); let ml_inserted = vec![ml_method_argument415_0]; let mut ml_method_values415 = ml_method_owner415.borrow_mut(); let ml_length = ml_method_values415.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values415.extend(ml_inserted); ml_length as f64 };
                                                                                    self::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332Next
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored350 = { let ml_method_owner416 = code.clone(); let ml_method_argument416_0 = self::JavaScriptCallInstruction::Argument(1f64, 0f64); let ml_method_argument416_1 = self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_inserted = vec![ml_method_argument416_0, ml_method_argument416_1]; let mut ml_method_values416 = ml_method_owner416.borrow_mut(); let ml_length = ml_method_values416.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values416.extend(ml_inserted); ml_length as f64 };
                                                        {
                                                            let source_ml_source1_s1688 = if (((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 116, 104, 46, 112, 111, 119]))) || ((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 116, 104, 46, 105, 109, 117, 108])))) {
                                                                {
                                                                    let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored353 = { let ml_method_owner417 = code.clone(); let ml_method_argument417_0 = self::JavaScriptCallInstruction::Argument(2f64, 1f64); let ml_method_argument417_1 = self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64]))); let ml_method_argument417_2 = self::JavaScriptCallInstruction::Primitive(1f64, if ((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 116, 104, 46, 112, 111, 119]))) {
                                                                        self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 111, 119, 101, 114])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115])
                                                                    }, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64]))); let ml_inserted = vec![ml_method_argument417_0, ml_method_argument417_1, ml_method_argument417_2]; let mut ml_method_values417 = ml_method_owner417.borrow_mut(); let ml_length = ml_method_values417.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values417.extend(ml_inserted); ml_length as f64 };
                                                                    self::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin351::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin351Next
                                                                }
                                                            } else {
                                                                {
                                                                    let primitive_2 = if ((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 116, 104, 46, 102, 114, 111, 117, 110, 100]))) {
                                                                        self::ml_string::Text::from_units(vec![114, 111, 117, 110, 100, 32, 98, 105, 110, 97, 114, 121, 51, 50, 32, 110, 117, 109, 98, 101, 114])
                                                                    } else {
                                                                        if ((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 116, 104, 46, 99, 108, 122, 51, 50]))) {
                                                                            self::ml_string::Text::from_units(vec![99, 111, 117, 110, 116, 32, 108, 101, 97, 100, 105, 110, 103, 32, 122, 101, 114, 111, 32, 98, 105, 116, 115])
                                                                        } else {
                                                                            if ((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 116, 104, 46, 97, 98, 115]))) {
                                                                                self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 110, 117, 109, 98, 101, 114])
                                                                            } else {
                                                                                if ((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 116, 104, 46, 102, 108, 111, 111, 114]))) {
                                                                                    self::ml_string::Text::from_units(vec![102, 108, 111, 111, 114, 32, 110, 117, 109, 98, 101, 114])
                                                                                } else {
                                                                                    if ((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 116, 104, 46, 99, 101, 105, 108]))) {
                                                                                        self::ml_string::Text::from_units(vec![99, 101, 105, 108, 32, 110, 117, 109, 98, 101, 114])
                                                                                    } else {
                                                                                        if ((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 116, 104, 46, 116, 114, 117, 110, 99]))) {
                                                                                            self::ml_string::Text::from_units(vec![116, 114, 117, 110, 99, 97, 116, 101, 32, 110, 117, 109, 98, 101, 114])
                                                                                        } else {
                                                                                            if ((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 116, 104, 46, 114, 111, 117, 110, 100]))) {
                                                                                                self::ml_string::Text::from_units(vec![114, 111, 117, 110, 100, 32, 110, 117, 109, 98, 101, 114])
                                                                                            } else {
                                                                                                if ((*operation) == (self::ml_string::Text::from_units(vec![77, 97, 116, 104, 46, 115, 105, 103, 110]))) {
                                                                                                    self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 110, 117, 109, 98, 101, 114, 32, 115, 105, 103, 110])
                                                                                                } else {
                                                                                                    self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 113, 117, 97, 114, 101, 32, 114, 111, 111, 116])
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored354 = { let ml_method_owner418 = code.clone(); let ml_method_argument418_0 = self::JavaScriptCallInstruction::Primitive(1f64, primitive_2.clone(), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_inserted = vec![ml_method_argument418_0]; let mut ml_method_values418 = ml_method_owner418.borrow_mut(); let ml_length = ml_method_values418.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values418.extend(ml_inserted); ml_length as f64 };
                                                                        self::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin351::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin351Next
                                                                    }
                                                                }
                                                            };
                                                            match source_ml_source1_s1688.clone() {
                                                                self::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin351::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin351Next => {
                                                                    {
                                                                        let source_ml_temporary_compile_java_script_numeric_intrinsic_function_ignored352 = { let ml_method_owner419 = code.clone(); let ml_method_argument419_0 = self::JavaScriptCallInstruction::Return(1f64); let ml_inserted = vec![ml_method_argument419_0]; let mut ml_method_values419 = ml_method_owner419.borrow_mut(); let ml_length = ml_method_values419.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values419.extend(ml_inserted); ml_length as f64 };
                                                                        self::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332Next
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
                };
                match source_ml_source1_s1686.clone() {
                    self::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332::MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332Next => {
                        Some({
                            let source_ml_source1_constructor_field1692 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_constructor_field1693 = 0f64;
                                {
                                    let source_ml_source1_constructor_field1694 = 10f64;
                                    {
                                        let source_ml_source1_constructor_field1695 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                        self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field1692.clone()), source_ml_source1_constructor_field1693, source_ml_source1_constructor_field1694, source_ml_source1_constructor_field1695.clone(), code.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                    }
                                }
                            }
                        })
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_string_method_names() -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![116, 111, 76, 111, 119, 101, 114, 67, 97, 115, 101]), self::ml_string::Text::from_units(vec![116, 111, 85, 112, 112, 101, 114, 67, 97, 115, 101]), self::ml_string::Text::from_units(vec![105, 115, 87, 101, 108, 108, 70, 111, 114, 109, 101, 100]), self::ml_string::Text::from_units(vec![116, 111, 87, 101, 108, 108, 70, 111, 114, 109, 101, 100]), self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 79, 102]), self::ml_string::Text::from_units(vec![116, 114, 105, 109]), self::ml_string::Text::from_units(vec![116, 114, 105, 109, 83, 116, 97, 114, 116]), self::ml_string::Text::from_units(vec![116, 114, 105, 109, 69, 110, 100]), self::ml_string::Text::from_units(vec![99, 104, 97, 114, 65, 116]), self::ml_string::Text::from_units(vec![99, 104, 97, 114, 67, 111, 100, 101, 65, 116]), self::ml_string::Text::from_units(vec![99, 111, 100, 101, 80, 111, 105, 110, 116, 65, 116]), self::ml_string::Text::from_units(vec![97, 116]), self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101]), self::ml_string::Text::from_units(vec![115, 117, 98, 115, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![115, 117, 98, 115, 116, 114]), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 79, 102]), self::ml_string::Text::from_units(vec![108, 97, 115, 116, 73, 110, 100, 101, 120, 79, 102]), self::ml_string::Text::from_units(vec![115, 116, 97, 114, 116, 115, 87, 105, 116, 104]), self::ml_string::Text::from_units(vec![101, 110, 100, 115, 87, 105, 116, 104]), self::ml_string::Text::from_units(vec![105, 110, 99, 108, 117, 100, 101, 115]), self::ml_string::Text::from_units(vec![114, 101, 112, 101, 97, 116]), self::ml_string::Text::from_units(vec![112, 97, 100, 83, 116, 97, 114, 116]), self::ml_string::Text::from_units(vec![112, 97, 100, 69, 110, 100]), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116])]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_string_static_method_names() -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![102, 114, 111, 109, 67, 104, 97, 114, 67, 111, 100, 101]), self::ml_string::Text::from_units(vec![102, 114, 111, 109, 67, 111, 100, 101, 80, 111, 105, 110, 116])]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_string_method_operation(operation: self::ml_string::Text) -> bool {
    { let ml_values420 = { let ml_reference_value421 = (std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value422 = (self::read_java_script_string_method_names()).borrow().clone(); ml_reference_value422 }, { let ml_reference_value423 = (self::read_java_script_string_static_method_names()).borrow().clone(); ml_reference_value423 })))).borrow().clone(); ml_reference_value421 }; let ml_callback420 = { let operation = operation.clone(); std::rc::Rc::new(move |name: self::ml_string::Text| -> bool { (operation == ((self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46])).concat(&name))) }) }; ml_values420.iter().cloned().enumerate().any(|(ml_index420, ml_value420)| ml_callback420(ml_value420)) }
}
