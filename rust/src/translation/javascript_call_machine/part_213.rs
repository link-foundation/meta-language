// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallIteratorOperation;
use super::JavaScriptCallIteratorRegion;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptIteratorProtocolTransition;
use super::JavaScriptIteratorRegionsUpdate;
use super::JavaScriptSemanticValue;
use super::ml_array;
use super::ml_string;
use super::replace_java_script_call_frame;
use super::request_java_script_call_error;
use super::throw_java_script_call_value;
use super::update_java_script_call_iterator_protocol_regions;
use super::write_java_script_call_iterator_result;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_call_iterator_protocol(
    current: self::JavaScriptCallMachine,
    frame: self::JavaScriptCallFrame,
    pending: self::JavaScriptCallIteratorOperation,
    transition: self::JavaScriptIteratorProtocolTransition,
) -> self::JavaScriptCallMachineTransition {
    match pending.clone() {
        self::JavaScriptCallIteratorOperation::IteratorOperation(
            shared_constructor_field13,
            shared_constructor_field14,
            shared_constructor_field15,
            shared_constructor_field16,
            shared_constructor_field17,
        ) => match frame.clone() {
            self::JavaScriptCallFrame::CallFrame(
                shared_constructor_field3,
                shared_constructor_field4,
                shared_constructor_field5,
                shared_constructor_field6,
                shared_constructor_field7,
                shared_constructor_field8,
                shared_constructor_field9,
                shared_constructor_field10,
                shared_constructor_field11,
                shared_constructor_field12,
            ) => {
                let shared_constructor_field8 = *shared_constructor_field8;
                match current.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field2) => {
                            match transition.clone() {
                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(transition_heap, _) => {
                                    let transition_heap = *transition_heap;
                                    {
                                        let update = self::update_java_script_call_iterator_protocol_regions((match frame.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, source_ml_source2_data_property_value37689, _, _, _) => {
                                                ((source_ml_source2_data_property_value37689.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>>,))).0).clone()
                                            }
                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallIteratorRegion>::new()))), shared_constructor_field15, &(shared_constructor_field16), transition.clone());
                                        match update.clone() {
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions) => {
                                                {
                                                    let next = self::replace_java_script_call_frame(transition_heap.clone(), shared_constructor_field2.clone(), {
                                                        let source_ml_source2_nominal_field37694 = shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field37696 = shared_constructor_field4;
                                                            {
                                                                let source_ml_source2_nominal_field37698 = shared_constructor_field5.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field37700 = shared_constructor_field6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field37702 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field37704 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field37706 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field37708 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field37710 = shared_constructor_field11.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field37712 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_override37715 = self::JavaScriptCallPending::CallIdle;
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field37694.clone(), source_ml_source2_nominal_field37696, source_ml_source2_nominal_field37698.clone(), source_ml_source2_nominal_field37700.clone(), source_ml_source2_nominal_field37702.clone(), Box::new(source_ml_source2_nominal_override37715.clone()), Some((Some(regions.clone()),)), source_ml_source2_nominal_field37708.clone(), source_ml_source2_nominal_field37710.clone(), source_ml_source2_nominal_field37712.clone())
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
                                                    });
                                                    match next.clone() {
                                                        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
                                                            {
                                                                let result = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let done = (shared_constructor_field16 == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101])));
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(transition_heap.clone(), shared_constructor_field1.clone(), self::write_java_script_call_iterator_result(({
                                                                        let source_ml_source2_array_read_value37746 = shared_constructor_field1.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index37747 = ({ let ml_reference_array2457 = shared_constructor_field1.clone(); let ml_reference_projection2457 = ml_reference_array2457.borrow().len() as f64; ml_reference_projection2457 } - 1f64);
                                                                            { let ml_reference_array2458 = source_ml_source2_array_read_value37746.clone(); let ml_reference_position2458 = source_ml_source2_array_read_index37747; let ml_reference_projection2458 = self::ml_array::optional_at(&*ml_reference_array2458.borrow(), self::ml_array::number_index(ml_reference_position2458)); ml_reference_projection2458 }
                                                                        }
                                                                    }).clone().unwrap_or_else(|| frame.clone()), shared_constructor_field13, shared_constructor_field14, result.clone(), done))))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsInvalid(reason) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(transition_heap.clone()), reason.clone())
                                            }
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsResourceLimit(reason_2) => {
                                                self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(current.clone()), reason_2.clone())
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolCall(transition_heap_2, _, _, _, _) => {
                                    let transition_heap_2 = *transition_heap_2;
                                    {
                                        let update_2 = self::update_java_script_call_iterator_protocol_regions((match frame.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, source_ml_source2_data_property_value37750, _, _, _) => {
                                                ((source_ml_source2_data_property_value37750.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>>,))).0).clone()
                                            }
                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallIteratorRegion>::new()))), shared_constructor_field15, &(shared_constructor_field16), transition.clone());
                                        match update_2.clone() {
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions_2) => {
                                                {
                                                    let next_2 = self::replace_java_script_call_frame(transition_heap_2.clone(), shared_constructor_field2.clone(), {
                                                        let source_ml_source2_nominal_field37755 = shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field37757 = shared_constructor_field4;
                                                            {
                                                                let source_ml_source2_nominal_field37759 = shared_constructor_field5.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field37761 = shared_constructor_field6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field37763 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field37765 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field37767 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field37769 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field37771 = shared_constructor_field11.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field37773 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_override37776 = self::JavaScriptCallPending::CallIdle;
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field37755.clone(), source_ml_source2_nominal_field37757, source_ml_source2_nominal_field37759.clone(), source_ml_source2_nominal_field37761.clone(), source_ml_source2_nominal_field37763.clone(), Box::new(source_ml_source2_nominal_override37776.clone()), Some((Some(regions_2.clone()),)), source_ml_source2_nominal_field37769.clone(), source_ml_source2_nominal_field37771.clone(), source_ml_source2_nominal_field37773.clone())
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
                                                    });
                                                    match next_2.clone() {
                                                        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1_2) => {
                                                            {
                                                                let result_2 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let done_2 = (shared_constructor_field16 == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101])));
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(transition_heap_2.clone(), shared_constructor_field1_2.clone(), self::write_java_script_call_iterator_result(({
                                                                        let source_ml_source2_array_read_value37807 = shared_constructor_field1_2.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index37808 = ({ let ml_reference_array2459 = shared_constructor_field1_2.clone(); let ml_reference_projection2459 = ml_reference_array2459.borrow().len() as f64; ml_reference_projection2459 } - 1f64);
                                                                            { let ml_reference_array2460 = source_ml_source2_array_read_value37807.clone(); let ml_reference_position2460 = source_ml_source2_array_read_index37808; let ml_reference_projection2460 = self::ml_array::optional_at(&*ml_reference_array2460.borrow(), self::ml_array::number_index(ml_reference_position2460)); ml_reference_projection2460 }
                                                                        }
                                                                    }).clone().unwrap_or_else(|| frame.clone()), shared_constructor_field13, shared_constructor_field14, result_2.clone(), done_2))))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsInvalid(reason_3) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(transition_heap_2.clone()), reason_3.clone())
                                            }
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsResourceLimit(reason_4) => {
                                                self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(current.clone()), reason_4.clone())
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolPrimitiveGet(transition_heap_3, _, _, _) => {
                                    let transition_heap_3 = *transition_heap_3;
                                    {
                                        let update_3 = self::update_java_script_call_iterator_protocol_regions((match frame.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, source_ml_source2_data_property_value37811, _, _, _) => {
                                                ((source_ml_source2_data_property_value37811.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>>,))).0).clone()
                                            }
                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallIteratorRegion>::new()))), shared_constructor_field15, &(shared_constructor_field16), transition.clone());
                                        match update_3.clone() {
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions_3) => {
                                                {
                                                    let next_3 = self::replace_java_script_call_frame(transition_heap_3.clone(), shared_constructor_field2.clone(), {
                                                        let source_ml_source2_nominal_field37816 = shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field37818 = shared_constructor_field4;
                                                            {
                                                                let source_ml_source2_nominal_field37820 = shared_constructor_field5.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field37822 = shared_constructor_field6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field37824 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field37826 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field37828 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field37830 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field37832 = shared_constructor_field11.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field37834 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_override37837 = self::JavaScriptCallPending::CallIdle;
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field37816.clone(), source_ml_source2_nominal_field37818, source_ml_source2_nominal_field37820.clone(), source_ml_source2_nominal_field37822.clone(), source_ml_source2_nominal_field37824.clone(), Box::new(source_ml_source2_nominal_override37837.clone()), Some((Some(regions_3.clone()),)), source_ml_source2_nominal_field37830.clone(), source_ml_source2_nominal_field37832.clone(), source_ml_source2_nominal_field37834.clone())
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
                                                    });
                                                    match next_3.clone() {
                                                        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1_3) => {
                                                            {
                                                                let result_3 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let done_3 = (shared_constructor_field16 == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101])));
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(transition_heap_3.clone(), shared_constructor_field1_3.clone(), self::write_java_script_call_iterator_result(({
                                                                        let source_ml_source2_array_read_value37868 = shared_constructor_field1_3.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index37869 = ({ let ml_reference_array2461 = shared_constructor_field1_3.clone(); let ml_reference_projection2461 = ml_reference_array2461.borrow().len() as f64; ml_reference_projection2461 } - 1f64);
                                                                            { let ml_reference_array2462 = source_ml_source2_array_read_value37868.clone(); let ml_reference_position2462 = source_ml_source2_array_read_index37869; let ml_reference_projection2462 = self::ml_array::optional_at(&*ml_reference_array2462.borrow(), self::ml_array::number_index(ml_reference_position2462)); ml_reference_projection2462 }
                                                                        }
                                                                    }).clone().unwrap_or_else(|| frame.clone()), shared_constructor_field13, shared_constructor_field14, result_3.clone(), done_3))))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsInvalid(reason_5) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(transition_heap_3.clone()), reason_5.clone())
                                            }
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsResourceLimit(reason_6) => {
                                                self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(current.clone()), reason_6.clone())
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolAcquired(transition_heap_4, _) => {
                                    let transition_heap_4 = *transition_heap_4;
                                    {
                                        let update_4 = self::update_java_script_call_iterator_protocol_regions((match frame.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, source_ml_source2_data_property_value37872, _, _, _) => {
                                                ((source_ml_source2_data_property_value37872.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>>,))).0).clone()
                                            }
                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallIteratorRegion>::new()))), shared_constructor_field15, &(shared_constructor_field16), transition.clone());
                                        match update_4.clone() {
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions_4) => {
                                                {
                                                    let next_4 = self::replace_java_script_call_frame(transition_heap_4.clone(), shared_constructor_field2.clone(), {
                                                        let source_ml_source2_nominal_field37877 = shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field37879 = shared_constructor_field4;
                                                            {
                                                                let source_ml_source2_nominal_field37881 = shared_constructor_field5.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field37883 = shared_constructor_field6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field37885 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field37887 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field37889 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field37891 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field37893 = shared_constructor_field11.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field37895 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_override37898 = self::JavaScriptCallPending::CallIdle;
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field37877.clone(), source_ml_source2_nominal_field37879, source_ml_source2_nominal_field37881.clone(), source_ml_source2_nominal_field37883.clone(), source_ml_source2_nominal_field37885.clone(), Box::new(source_ml_source2_nominal_override37898.clone()), Some((Some(regions_4.clone()),)), source_ml_source2_nominal_field37891.clone(), source_ml_source2_nominal_field37893.clone(), source_ml_source2_nominal_field37895.clone())
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
                                                    });
                                                    match next_4.clone() {
                                                        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1_4) => {
                                                            {
                                                                let result_4 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let done_4 = (shared_constructor_field16 == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101])));
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(transition_heap_4.clone(), shared_constructor_field1_4.clone(), self::write_java_script_call_iterator_result(({
                                                                        let source_ml_source2_array_read_value37929 = shared_constructor_field1_4.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index37930 = ({ let ml_reference_array2463 = shared_constructor_field1_4.clone(); let ml_reference_projection2463 = ml_reference_array2463.borrow().len() as f64; ml_reference_projection2463 } - 1f64);
                                                                            { let ml_reference_array2464 = source_ml_source2_array_read_value37929.clone(); let ml_reference_position2464 = source_ml_source2_array_read_index37930; let ml_reference_projection2464 = self::ml_array::optional_at(&*ml_reference_array2464.borrow(), self::ml_array::number_index(ml_reference_position2464)); ml_reference_projection2464 }
                                                                        }
                                                                    }).clone().unwrap_or_else(|| frame.clone()), shared_constructor_field13, shared_constructor_field14, result_4.clone(), done_4))))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsInvalid(reason_7) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(transition_heap_4.clone()), reason_7.clone())
                                            }
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsResourceLimit(reason_8) => {
                                                self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(current.clone()), reason_8.clone())
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolStep(transition_heap_5, _, source_ml_source2_f37680, source_ml_source2_f37681) => {
                                    let transition_heap_5 = *transition_heap_5;
                                    let source_ml_source2_f37681 = *source_ml_source2_f37681;
                                    {
                                        let update_5 = self::update_java_script_call_iterator_protocol_regions((match frame.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, source_ml_source2_data_property_value37933, _, _, _) => {
                                                ((source_ml_source2_data_property_value37933.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>>,))).0).clone()
                                            }
                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallIteratorRegion>::new()))), shared_constructor_field15, &(shared_constructor_field16), transition.clone());
                                        match update_5.clone() {
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions_5) => {
                                                {
                                                    let next_5 = self::replace_java_script_call_frame(transition_heap_5.clone(), shared_constructor_field2.clone(), {
                                                        let source_ml_source2_nominal_field37938 = shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field37940 = shared_constructor_field4;
                                                            {
                                                                let source_ml_source2_nominal_field37942 = shared_constructor_field5.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field37944 = shared_constructor_field6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field37946 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field37948 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field37950 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field37952 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field37954 = shared_constructor_field11.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field37956 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_override37959 = self::JavaScriptCallPending::CallIdle;
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field37938.clone(), source_ml_source2_nominal_field37940, source_ml_source2_nominal_field37942.clone(), source_ml_source2_nominal_field37944.clone(), source_ml_source2_nominal_field37946.clone(), Box::new(source_ml_source2_nominal_override37959.clone()), Some((Some(regions_5.clone()),)), source_ml_source2_nominal_field37952.clone(), source_ml_source2_nominal_field37954.clone(), source_ml_source2_nominal_field37956.clone())
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
                                                    });
                                                    match next_5.clone() {
                                                        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1_5) => {
                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(transition_heap_5.clone(), shared_constructor_field1_5.clone(), self::write_java_script_call_iterator_result(({
                                                                let source_ml_source2_array_read_value37988 = shared_constructor_field1_5.clone();
                                                                {
                                                                    let source_ml_source2_array_read_index37989 = ({ let ml_reference_array2465 = shared_constructor_field1_5.clone(); let ml_reference_projection2465 = ml_reference_array2465.borrow().len() as f64; ml_reference_projection2465 } - 1f64);
                                                                    { let ml_reference_array2466 = source_ml_source2_array_read_value37988.clone(); let ml_reference_position2466 = source_ml_source2_array_read_index37989; let ml_reference_projection2466 = self::ml_array::optional_at(&*ml_reference_array2466.borrow(), self::ml_array::number_index(ml_reference_position2466)); ml_reference_projection2466 }
                                                                }
                                                            }).clone().unwrap_or_else(|| frame.clone()), shared_constructor_field13, shared_constructor_field14, source_ml_source2_f37681.clone(), source_ml_source2_f37680))))
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsInvalid(reason_9) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(transition_heap_5.clone()), reason_9.clone())
                                            }
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsResourceLimit(reason_10) => {
                                                self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(current.clone()), reason_10.clone())
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolClosed(transition_heap_6, _, _) => {
                                    let transition_heap_6 = *transition_heap_6;
                                    {
                                        let update_6 = self::update_java_script_call_iterator_protocol_regions((match frame.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, source_ml_source2_data_property_value37992, _, _, _) => {
                                                ((source_ml_source2_data_property_value37992.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>>,))).0).clone()
                                            }
                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallIteratorRegion>::new()))), shared_constructor_field15, &(shared_constructor_field16), transition.clone());
                                        match update_6.clone() {
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions_6) => {
                                                {
                                                    let next_6 = self::replace_java_script_call_frame(transition_heap_6.clone(), shared_constructor_field2.clone(), {
                                                        let source_ml_source2_nominal_field37997 = shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field37999 = shared_constructor_field4;
                                                            {
                                                                let source_ml_source2_nominal_field38001 = shared_constructor_field5.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field38003 = shared_constructor_field6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field38005 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field38007 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field38009 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field38011 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field38013 = shared_constructor_field11.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field38015 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_override38018 = self::JavaScriptCallPending::CallIdle;
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field37997.clone(), source_ml_source2_nominal_field37999, source_ml_source2_nominal_field38001.clone(), source_ml_source2_nominal_field38003.clone(), source_ml_source2_nominal_field38005.clone(), Box::new(source_ml_source2_nominal_override38018.clone()), Some((Some(regions_6.clone()),)), source_ml_source2_nominal_field38011.clone(), source_ml_source2_nominal_field38013.clone(), source_ml_source2_nominal_field38015.clone())
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
                                                    });
                                                    match next_6.clone() {
                                                        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1_6) => {
                                                            {
                                                                let result_5 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let done_5 = (shared_constructor_field16 == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101])));
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(transition_heap_6.clone(), shared_constructor_field1_6.clone(), self::write_java_script_call_iterator_result(({
                                                                        let source_ml_source2_array_read_value38049 = shared_constructor_field1_6.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index38050 = ({ let ml_reference_array2467 = shared_constructor_field1_6.clone(); let ml_reference_projection2467 = ml_reference_array2467.borrow().len() as f64; ml_reference_projection2467 } - 1f64);
                                                                            { let ml_reference_array2468 = source_ml_source2_array_read_value38049.clone(); let ml_reference_position2468 = source_ml_source2_array_read_index38050; let ml_reference_projection2468 = self::ml_array::optional_at(&*ml_reference_array2468.borrow(), self::ml_array::number_index(ml_reference_position2468)); ml_reference_projection2468 }
                                                                        }
                                                                    }).clone().unwrap_or_else(|| frame.clone()), shared_constructor_field13, shared_constructor_field14, result_5.clone(), done_5))))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsInvalid(reason_11) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(transition_heap_6.clone()), reason_11.clone())
                                            }
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsResourceLimit(reason_12) => {
                                                self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(current.clone()), reason_12.clone())
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolThrow(transition_heap_7, _, source_ml_source2_f37685) => {
                                    let transition_heap_7 = *transition_heap_7;
                                    let source_ml_source2_f37685 = *source_ml_source2_f37685;
                                    {
                                        let update_7 = self::update_java_script_call_iterator_protocol_regions((match frame.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, source_ml_source2_data_property_value38053, _, _, _) => {
                                                ((source_ml_source2_data_property_value38053.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>>,))).0).clone()
                                            }
                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallIteratorRegion>::new()))), shared_constructor_field15, &(shared_constructor_field16), transition.clone());
                                        match update_7.clone() {
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions_7) => {
                                                {
                                                    let next_7 = self::replace_java_script_call_frame(transition_heap_7.clone(), shared_constructor_field2.clone(), {
                                                        let source_ml_source2_nominal_field38058 = shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field38060 = shared_constructor_field4;
                                                            {
                                                                let source_ml_source2_nominal_field38062 = shared_constructor_field5.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field38064 = shared_constructor_field6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field38066 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field38068 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field38070 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field38072 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field38074 = shared_constructor_field11.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field38076 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_override38079 = self::JavaScriptCallPending::CallIdle;
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field38058.clone(), source_ml_source2_nominal_field38060, source_ml_source2_nominal_field38062.clone(), source_ml_source2_nominal_field38064.clone(), source_ml_source2_nominal_field38066.clone(), Box::new(source_ml_source2_nominal_override38079.clone()), Some((Some(regions_7.clone()),)), source_ml_source2_nominal_field38072.clone(), source_ml_source2_nominal_field38074.clone(), source_ml_source2_nominal_field38076.clone())
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
                                                    });
                                                    self::throw_java_script_call_value(next_7.clone(), source_ml_source2_f37685.clone())
                                                }
                                            }
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsInvalid(reason_13) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(transition_heap_7.clone()), reason_13.clone())
                                            }
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsResourceLimit(reason_14) => {
                                                self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(current.clone()), reason_14.clone())
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolError(transition_heap_8, _, source_ml_source2_f37687, source_ml_source2_f37688) => {
                                    let transition_heap_8 = *transition_heap_8;
                                    {
                                        let update_8 = self::update_java_script_call_iterator_protocol_regions((match frame.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, source_ml_source2_data_property_value38099, _, _, _) => {
                                                ((source_ml_source2_data_property_value38099.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>>,))).0).clone()
                                            }
                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallIteratorRegion>::new()))), shared_constructor_field15, &(shared_constructor_field16), transition.clone());
                                        match update_8.clone() {
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions_8) => {
                                                {
                                                    let next_8 = self::replace_java_script_call_frame(transition_heap_8.clone(), shared_constructor_field2.clone(), {
                                                        let source_ml_source2_nominal_field38104 = shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field38106 = shared_constructor_field4;
                                                            {
                                                                let source_ml_source2_nominal_field38108 = shared_constructor_field5.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field38110 = shared_constructor_field6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field38112 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field38114 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field38116 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field38118 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field38120 = shared_constructor_field11.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field38122 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_override38125 = self::JavaScriptCallPending::CallIdle;
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field38104.clone(), source_ml_source2_nominal_field38106, source_ml_source2_nominal_field38108.clone(), source_ml_source2_nominal_field38110.clone(), source_ml_source2_nominal_field38112.clone(), Box::new(source_ml_source2_nominal_override38125.clone()), Some((Some(regions_8.clone()),)), source_ml_source2_nominal_field38118.clone(), source_ml_source2_nominal_field38120.clone(), source_ml_source2_nominal_field38122.clone())
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
                                                    });
                                                    self::request_java_script_call_error(next_8.clone(), shared_constructor_field17.clone(), &(source_ml_source2_f37687), &(source_ml_source2_f37688))
                                                }
                                            }
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsInvalid(reason_15) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(transition_heap_8.clone()), reason_15.clone())
                                            }
                                            self::JavaScriptIteratorRegionsUpdate::IteratorRegionsResourceLimit(reason_16) => {
                                                self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(current.clone()), reason_16.clone())
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(heap, reason_17) => {
                                    let heap = *heap;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), reason_17.clone())
                                }
                            }
                        }
                    }
            }
        },
    }
}
