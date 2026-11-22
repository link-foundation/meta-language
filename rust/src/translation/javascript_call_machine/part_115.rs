// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCompletion;
use super::JavaScriptObjectEnumerationState;
use super::JavaScriptObjectEnumerationTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::accept_java_script_object_enumeration_value;
use super::allocate_java_script_array;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_object_enumeration_state_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_unit;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_object_enumeration_read(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectEnumerationState,
) -> self::JavaScriptObjectEnumerationTransition {
    match state.clone() {
        self::JavaScriptObjectEnumerationState::ObjectEnumeration(
            shared_constructor_field2,
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
            shared_constructor_field13,
            shared_constructor_field14,
        ) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field11 = *shared_constructor_field11;
            let shared_constructor_field13 = *shared_constructor_field13;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    if (shared_constructor_field7
                        == ({
                            let ml_reference_array1170 = shared_constructor_field5.clone();
                            let ml_reference_projection1170 =
                                ml_reference_array1170.borrow().len() as f64;
                            ml_reference_projection1170
                        }))
                    {
                        self::JavaScriptObjectEnumerationTransition::ObjectEnumerationContinue(
                            Box::new(heap.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field23094 =
                                    shared_constructor_field2.clone();
                                {
                                    let source_ml_source2_nominal_field23096 =
                                        shared_constructor_field3.clone();
                                    {
                                        let source_ml_source2_nominal_field23098 =
                                            shared_constructor_field4.clone();
                                        {
                                            let source_ml_source2_nominal_field23100 =
                                                shared_constructor_field5.clone();
                                            {
                                                let source_ml_source2_nominal_field23102 =
                                                    shared_constructor_field6.clone();
                                                {
                                                    let source_ml_source2_nominal_field23104 =
                                                        shared_constructor_field7;
                                                    {
                                                        let source_ml_source2_nominal_field23106 =
                                                            shared_constructor_field8.clone();
                                                        {
                                                            let source_ml_source2_nominal_field23108 =
                                                                shared_constructor_field9.clone();
                                                            {
                                                                let source_ml_source2_nominal_field23110 =
                                                                    shared_constructor_field10
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field23112 =
                                                                        shared_constructor_field11
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field23114 = shared_constructor_field12;
                                                                        {
                                                                            let source_ml_source2_nominal_field23116 = shared_constructor_field13.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field23118 = shared_constructor_field14;
                                                                                self::JavaScriptObjectEnumerationState::ObjectEnumeration(source_ml_source2_nominal_field23094.clone(), Box::new(source_ml_source2_nominal_field23096.clone()), Box::new(source_ml_source2_nominal_field23098.clone()), source_ml_source2_nominal_field23100.clone(), source_ml_source2_nominal_field23102.clone(), source_ml_source2_nominal_field23104, self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), Box::new(source_ml_source2_nominal_field23108.clone()), Box::new(source_ml_source2_nominal_field23110.clone()), Box::new(source_ml_source2_nominal_field23112.clone()), source_ml_source2_nominal_field23114, Box::new(source_ml_source2_nominal_field23116.clone()), source_ml_source2_nominal_field23118)
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
                            }),
                        )
                    } else {
                        {
                            let key = {
                                let source_ml_source2_array_read_value23123 =
                                    shared_constructor_field5.clone();
                                {
                                    let source_ml_source2_array_read_index23124 =
                                        shared_constructor_field7;
                                    {
                                        let ml_reference_array1171 =
                                            source_ml_source2_array_read_value23123.clone();
                                        let ml_reference_position1171 =
                                            source_ml_source2_array_read_index23124;
                                        let ml_reference_projection1171 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array1171.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position1171,
                                                ),
                                            );
                                        ml_reference_projection1171
                                    }
                                }
                            };
                            if match (key.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 111, 119, 110, 32, 107, 101, 121]))
                            } else {
                                match key.clone().expect("a proven present value") {
                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                        self::JavaScriptObjectEnumerationTransition::ObjectEnumerationContinue(Box::new(heap.clone()), Box::new({
                                            let source_ml_source2_nominal_field23128 = shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field23130 = shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field23132 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field23134 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field23136 = shared_constructor_field6.clone();
                                                            {
                                                                let source_ml_source2_nominal_field23138 = shared_constructor_field7;
                                                                {
                                                                    let source_ml_source2_nominal_field23140 = shared_constructor_field8.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field23142 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field23144 = shared_constructor_field10.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field23146 = shared_constructor_field11.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field23148 = shared_constructor_field12;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field23150 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field23152 = shared_constructor_field14;
                                                                                            {
                                                                                                let source_ml_source2_nominal_override23154 = (shared_constructor_field7 + 1f64);
                                                                                                self::JavaScriptObjectEnumerationState::ObjectEnumeration(source_ml_source2_nominal_field23128.clone(), Box::new(source_ml_source2_nominal_field23130.clone()), Box::new(source_ml_source2_nominal_field23132.clone()), source_ml_source2_nominal_field23134.clone(), source_ml_source2_nominal_field23136.clone(), source_ml_source2_nominal_override23154, source_ml_source2_nominal_field23140.clone(), Box::new(source_ml_source2_nominal_field23142.clone()), Box::new(source_ml_source2_nominal_field23144.clone()), Box::new(source_ml_source2_nominal_field23146.clone()), source_ml_source2_nominal_field23148, Box::new(source_ml_source2_nominal_field23150.clone()), source_ml_source2_nominal_field23152)
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
                                        }))
                                    }
                                    _ => {
                                        if {
                                            let source_ml_source2_s23156 = shared_constructor_field3.clone();
                                            match source_ml_source2_s23156.clone() {
                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                    true
                                                }
                                                _ => {
                                                    false
                                                }
                                            }
                                        } {
                                            {
                                                let source = {
                                                    let source_ml_source2_array_read_value23164 = shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_array_read_index23165 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), (((match shared_constructor_field3.clone() {
                                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value23163, _) => {
                                                                Some((source_ml_source2_data_property_value23163.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::ml::Big,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone());
                                                        { let ml_reference_array1172 = source_ml_source2_array_read_value23164.clone(); let ml_reference_position1172 = source_ml_source2_array_read_index23165; let ml_reference_projection1172 = self::ml_array::optional_at(&*ml_reference_array1172.borrow(), self::ml_array::number_index(ml_reference_position1172)); ml_reference_projection1172 }
                                                    }
                                                };
                                                {
                                                    let property = {
                                                        let source_ml_source2_optional_chain_receiver23166 = (source.clone(),);
                                                        match if match (source_ml_source2_optional_chain_receiver23166.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                            Some({
                                                                let source_ml_source1_temporary_optional_chain1060 = ((source_ml_source2_optional_chain_receiver23166.clone()).0).clone().expect("a proven present value");
                                                                {
                                                                    let source_ml_source2_array_read_value23195 = match source_ml_source1_temporary_optional_chain1060.clone() {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value23167, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value23167.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value23168, _, _, _) => {
                                                                            source_ml_source2_data_property_value23168.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value23169) => {
                                                                            source_ml_source2_data_property_value23169.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value23170, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value23170.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value23171, _) => {
                                                                            source_ml_source2_data_property_value23171.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value23172, _) => {
                                                                            source_ml_source2_data_property_value23172.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value23173, _) => {
                                                                            source_ml_source2_data_property_value23173.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value23174, _) => {
                                                                            source_ml_source2_data_property_value23174.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value23175) => {
                                                                            source_ml_source2_data_property_value23175.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value23176, _, _) => {
                                                                            source_ml_source2_data_property_value23176.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value23177, _, _) => {
                                                                            source_ml_source2_data_property_value23177.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value23178, _, _) => {
                                                                            source_ml_source2_data_property_value23178.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value23179, _, _) => {
                                                                            source_ml_source2_data_property_value23179.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_array_read_index23196 = self::find_java_script_ordinary_property({
                                                                            let source_ml_source2_nullish_field_receiver23180 = (source.clone(),);
                                                                            if match (source_ml_source2_nullish_field_receiver23180.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source2_nullish_field_present23181 = ((source_ml_source2_nullish_field_receiver23180.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source2_nullish_field_present23181.clone() {
                                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value23182, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value23182.clone()
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value23183, _, _, _) => {
                                                                                            source_ml_source2_data_property_value23183.clone()
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value23184) => {
                                                                                            source_ml_source2_data_property_value23184.clone()
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value23185, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value23185.clone()
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value23186, _) => {
                                                                                            source_ml_source2_data_property_value23186.clone()
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value23187, _) => {
                                                                                            source_ml_source2_data_property_value23187.clone()
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value23188, _) => {
                                                                                            source_ml_source2_data_property_value23188.clone()
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value23189, _) => {
                                                                                            source_ml_source2_data_property_value23189.clone()
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value23190) => {
                                                                                            source_ml_source2_data_property_value23190.clone()
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value23191, _, _) => {
                                                                                            source_ml_source2_data_property_value23191.clone()
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value23192, _, _) => {
                                                                                            source_ml_source2_data_property_value23192.clone()
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value23193, _, _) => {
                                                                                            source_ml_source2_data_property_value23193.clone()
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value23194, _, _) => {
                                                                                            source_ml_source2_data_property_value23194.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }, key.clone().expect("a proven present value"));
                                                                        { let ml_reference_array1173 = source_ml_source2_array_read_value23195.clone(); let ml_reference_position1173 = source_ml_source2_array_read_index23196; let ml_reference_projection1173 = self::ml_array::optional_at(&*ml_reference_array1173.borrow(), self::ml_array::number_index(ml_reference_position1173)); ml_reference_projection1173 }
                                                                    }
                                                                }
                                                            })
                                                        } else {
                                                            None::<self::JavaScriptOrdinaryProperty>
                                                        } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                    };
                                                    if (match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !((({ let ml_reference_value1174 = (match property.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value23197) => {
                                                            source_ml_source2_data_property_value23197.clone()
                                                        }
                                                    }).borrow().clone(); ml_reference_value1174 }).0).2)) {
                                                        self::JavaScriptObjectEnumerationTransition::ObjectEnumerationContinue(Box::new(heap.clone()), Box::new({
                                                            let source_ml_source2_nominal_field23200 = shared_constructor_field2.clone();
                                                            {
                                                                let source_ml_source2_nominal_field23202 = shared_constructor_field3.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field23204 = shared_constructor_field4.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field23206 = shared_constructor_field5.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field23208 = shared_constructor_field6.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field23210 = shared_constructor_field7;
                                                                                {
                                                                                    let source_ml_source2_nominal_field23212 = shared_constructor_field8.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field23214 = shared_constructor_field9.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field23216 = shared_constructor_field10.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field23218 = shared_constructor_field11.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field23220 = shared_constructor_field12;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field23222 = shared_constructor_field13.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field23224 = shared_constructor_field14;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override23226 = (shared_constructor_field7 + 1f64);
                                                                                                                self::JavaScriptObjectEnumerationState::ObjectEnumeration(source_ml_source2_nominal_field23200.clone(), Box::new(source_ml_source2_nominal_field23202.clone()), Box::new(source_ml_source2_nominal_field23204.clone()), source_ml_source2_nominal_field23206.clone(), source_ml_source2_nominal_field23208.clone(), source_ml_source2_nominal_override23226, source_ml_source2_nominal_field23212.clone(), Box::new(source_ml_source2_nominal_field23214.clone()), Box::new(source_ml_source2_nominal_field23216.clone()), Box::new(source_ml_source2_nominal_field23218.clone()), source_ml_source2_nominal_field23220, Box::new(source_ml_source2_nominal_field23222.clone()), source_ml_source2_nominal_field23224)
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
                                                        }))
                                                    } else {
                                                        if (shared_constructor_field2 == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 107, 101, 121, 115]))) {
                                                            self::accept_java_script_object_enumeration_value(heap.clone(), state.clone(), key.clone().expect("a proven present value"))
                                                        } else {
                                                            {
                                                                let get = self::plan_java_script_ordinary_get(heap.clone(), (((match shared_constructor_field3.clone() {
                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value23230, _) => {
                                                                        Some((source_ml_source2_data_property_value23230.clone(),))
                                                                    }
                                                                    _ => {
                                                                        None::<(self::ml::Big,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0).clone(), key.clone().expect("a proven present value"), shared_constructor_field3.clone());
                                                                if (((({ let ml_reference_value1175 = get.borrow().clone(); ml_reference_value1175 }).0).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                                    self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(Box::new(heap.clone()), (({ let ml_reference_value1176 = get.borrow().clone(); ml_reference_value1176 }).0).3)
                                                                } else {
                                                                    if (((({ let ml_reference_value1177 = get.borrow().clone(); ml_reference_value1177 }).0).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                        self::JavaScriptObjectEnumerationTransition::ObjectEnumerationCall(Box::new(heap.clone()), Box::new({
                                                                            let source_ml_source2_nominal_field23233 = shared_constructor_field2.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field23235 = shared_constructor_field3.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field23237 = shared_constructor_field4.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field23239 = shared_constructor_field5.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field23241 = shared_constructor_field6.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field23243 = shared_constructor_field7;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field23245 = shared_constructor_field8.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field23247 = shared_constructor_field9.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field23249 = shared_constructor_field10.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field23251 = shared_constructor_field11.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field23253 = shared_constructor_field12;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field23255 = shared_constructor_field13.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field23257 = shared_constructor_field14;
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_override23260 = (({ let ml_reference_value1178 = get.borrow().clone(); ml_reference_value1178 }).0).2;
                                                                                                                                self::JavaScriptObjectEnumerationState::ObjectEnumeration(source_ml_source2_nominal_field23233.clone(), Box::new(source_ml_source2_nominal_field23235.clone()), Box::new(source_ml_source2_nominal_field23237.clone()), source_ml_source2_nominal_field23239.clone(), source_ml_source2_nominal_field23241.clone(), source_ml_source2_nominal_field23243, self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]), Box::new(source_ml_source2_nominal_field23247.clone()), Box::new(source_ml_source2_nominal_override23260.clone()), Box::new(source_ml_source2_nominal_field23251.clone()), source_ml_source2_nominal_field23253, Box::new(source_ml_source2_nominal_field23255.clone()), source_ml_source2_nominal_field23257)
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
                                                                        }), Box::new((({ let ml_reference_value1179 = get.borrow().clone(); ml_reference_value1179 }).0).2), Box::new((({ let ml_reference_value1180 = get.borrow().clone(); ml_reference_value1180 }).0).4), (({ let ml_reference_value1181 = get.borrow().clone(); ml_reference_value1181 }).0).1)
                                                                    } else {
                                                                        if (((({ let ml_reference_value1182 = get.borrow().clone(); ml_reference_value1182 }).0).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                            self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 71, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 103, 101, 116, 116, 101, 114]))
                                                                        } else {
                                                                            self::accept_java_script_object_enumeration_value(heap.clone(), state.clone(), (({ let ml_reference_value1183 = get.borrow().clone(); ml_reference_value1183 }).0).5)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if ({
                                                let source_ml_source2_s23262 = shared_constructor_field3.clone();
                                                match source_ml_source2_s23262.clone() {
                                                    self::JavaScriptSemanticValue::Text(_) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }
                                            } && {
                                                let source_ml_source2_s23266 = shared_constructor_field3.clone();
                                                match source_ml_source2_s23266.clone() {
                                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }
                                            }) {
                                                self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 105, 110, 118, 101, 110, 116, 32, 111, 119, 110, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115]))
                                            } else {
                                                self::accept_java_script_object_enumeration_value(heap.clone(), state.clone(), if (shared_constructor_field2 == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 107, 101, 121, 115]))) {
                                                    key.clone().expect("a proven present value")
                                                } else {
                                                    self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![self::read_java_script_semantic_text_unit(shared_constructor_field3.clone(), shared_constructor_field7)])))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_object_enumeration_allocation(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectEnumerationState,
    identity: self::ml::Big,
) -> self::JavaScriptObjectEnumerationTransition {
    match state.clone() {
        self::JavaScriptObjectEnumerationState::ObjectEnumeration(
            shared_constructor_field1,
            shared_constructor_field2,
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
            shared_constructor_field13,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field12 = *shared_constructor_field12;
            {
                let error = self::read_java_script_object_enumeration_state_error(
                    heap.clone(),
                    state.clone(),
                );
                if ((error != (self::ml_string::Text::from_units(vec![])))
                    || ((shared_constructor_field7
                        != (self::ml_string::Text::from_units(vec![
                            97, 108, 108, 111, 99, 97, 116, 101,
                        ])))
                        && (shared_constructor_field7
                            != (self::ml_string::Text::from_units(vec![
                                112, 97, 105, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111,
                                110,
                            ])))))
                {
                    self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(
                        Box::new(heap.clone()),
                        if !error.0.is_empty() {
                            error.clone()
                        } else {
                            self::ml_string::Text::from_units(vec![
                                79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97,
                                116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111,
                                110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32,
                                97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103,
                                32, 112, 104, 97, 115, 101,
                            ])
                        },
                    )
                } else {
                    {
                        let allocated = self::allocate_java_script_array(
                            heap.clone(),
                            identity.clone(),
                            shared_constructor_field3.clone(),
                            if (shared_constructor_field7
                                == (self::ml_string::Text::from_units(vec![
                                    97, 108, 108, 111, 99, 97, 116, 101,
                                ])))
                            {
                                {
                                    let ml_reference_array1184 = shared_constructor_field5.clone();
                                    let ml_reference_projection1184 =
                                        ml_reference_array1184.borrow().len() as f64;
                                    ml_reference_projection1184
                                }
                            } else {
                                2f64
                            },
                        );
                        match allocated.clone() {
                            self::JavaScriptArrayCompletion::ArrayNormal(heap_2, value) => {
                                let heap_2 = *heap_2;
                                let value = *value;
                                self::JavaScriptObjectEnumerationTransition::ObjectEnumerationContinue(Box::new(heap_2.clone()), Box::new(if (shared_constructor_field7 == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]))) {
                                    {
                                        let source_ml_source2_nominal_field23293 = shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field23295 = shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field23297 = shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field23299 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field23301 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field23303 = shared_constructor_field6;
                                                            {
                                                                let source_ml_source2_nominal_field23305 = shared_constructor_field7.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field23307 = shared_constructor_field8.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field23309 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field23311 = shared_constructor_field10.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field23313 = shared_constructor_field11;
                                                                                {
                                                                                    let source_ml_source2_nominal_field23315 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field23317 = shared_constructor_field13;
                                                                                        self::JavaScriptObjectEnumerationState::ObjectEnumeration(source_ml_source2_nominal_field23293.clone(), Box::new(source_ml_source2_nominal_field23295.clone()), Box::new(source_ml_source2_nominal_field23297.clone()), source_ml_source2_nominal_field23299.clone(), source_ml_source2_nominal_field23301.clone(), source_ml_source2_nominal_field23303, self::ml_string::Text::from_units(vec![111, 117, 116, 112, 117, 116, 32, 119, 114, 105, 116, 101]), Box::new(source_ml_source2_nominal_field23307.clone()), Box::new(source_ml_source2_nominal_field23309.clone()), Box::new(source_ml_source2_nominal_field23311.clone()), source_ml_source2_nominal_field23313, Box::new(value.clone()), source_ml_source2_nominal_field23317)
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
                                    {
                                        let source_ml_source2_nominal_field23322 = shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field23324 = shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field23326 = shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field23328 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field23330 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field23332 = shared_constructor_field6;
                                                            {
                                                                let source_ml_source2_nominal_field23334 = shared_constructor_field7.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field23336 = shared_constructor_field8.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field23338 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field23340 = shared_constructor_field10.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field23342 = shared_constructor_field11;
                                                                                {
                                                                                    let source_ml_source2_nominal_field23344 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field23346 = shared_constructor_field13;
                                                                                        self::JavaScriptObjectEnumerationState::ObjectEnumeration(source_ml_source2_nominal_field23322.clone(), Box::new(source_ml_source2_nominal_field23324.clone()), Box::new(source_ml_source2_nominal_field23326.clone()), source_ml_source2_nominal_field23328.clone(), source_ml_source2_nominal_field23330.clone(), source_ml_source2_nominal_field23332, self::ml_string::Text::from_units(vec![112, 97, 105, 114, 32, 119, 114, 105, 116, 101]), Box::new(source_ml_source2_nominal_field23336.clone()), Box::new(source_ml_source2_nominal_field23338.clone()), Box::new(value.clone()), source_ml_source2_nominal_field23342, Box::new(source_ml_source2_nominal_field23344.clone()), source_ml_source2_nominal_field23346)
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
                                }))
                            }
                            self::JavaScriptArrayCompletion::ArrayPropertyComplete(
                                heap_3,
                                _,
                                _,
                            ) => {
                                let heap_3 = *heap_3;
                                self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(Box::new(heap_3.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 65, 114, 114, 97, 121]))
                            }
                            self::JavaScriptArrayCompletion::ArrayNumberRequest(heap_4, _, _) => {
                                let heap_4 = *heap_4;
                                self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(Box::new(heap_4.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 65, 114, 114, 97, 121]))
                            }
                            self::JavaScriptArrayCompletion::ArrayDefinitionContinue(heap_5, _) => {
                                let heap_5 = *heap_5;
                                self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(Box::new(heap_5.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 65, 114, 114, 97, 121]))
                            }
                            self::JavaScriptArrayCompletion::ArrayInvalid(heap_6, reason) => {
                                let heap_6 = *heap_6;
                                self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(Box::new(heap_6.clone()), reason.clone())
                            }
                            self::JavaScriptArrayCompletion::ArrayError(heap_7, _, reason_2) => {
                                let heap_7 = *heap_7;
                                self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(Box::new(heap_7.clone()), reason_2.clone())
                            }
                            self::JavaScriptArrayCompletion::ArrayResourceLimit(
                                heap_8,
                                reason_3,
                                _,
                            ) => {
                                let heap_8 = *heap_8;
                                self::JavaScriptObjectEnumerationTransition::ObjectEnumerationResourceLimit(Box::new(heap_8.clone()), Box::new(state.clone()), reason_3.clone())
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
pub fn resume_java_script_object_enumeration_getter(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectEnumerationState,
    completion: self::JavaScriptSemanticCompletion,
) -> self::JavaScriptObjectEnumerationTransition {
    {
        let error = {
            let source_ml_source2_logical23352 =
                self::read_java_script_object_enumeration_state_error(heap.clone(), state.clone());
            if !source_ml_source2_logical23352.0.is_empty() {
                source_ml_source2_logical23352.clone()
            } else {
                self::read_java_script_ordinary_value_error(
                    heap.clone(),
                    match completion.clone() {
                        self::JavaScriptSemanticCompletion::NormalCompletion(
                            source_ml_source2_data_property_value23350,
                        ) => {
                            let source_ml_source2_data_property_value23350 =
                                *source_ml_source2_data_property_value23350;
                            source_ml_source2_data_property_value23350.clone()
                        }
                        self::JavaScriptSemanticCompletion::ThrowCompletion(
                            source_ml_source2_data_property_value23351,
                        ) => {
                            let source_ml_source2_data_property_value23351 =
                                *source_ml_source2_data_property_value23351;
                            source_ml_source2_data_property_value23351.clone()
                        }
                    },
                )
            }
        };
        if ((error != (self::ml_string::Text::from_units(vec![])))
            || ((match state.clone() {
                self::JavaScriptObjectEnumerationState::ObjectEnumeration(
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value23354,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value23354.clone(),
            }) != (self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]))))
        {
            self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(
                Box::new(heap.clone()),
                if !error.0.is_empty() {
                    error.clone()
                } else {
                    self::ml_string::Text::from_units(vec![
                        79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105,
                        111, 110, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 114,
                        101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                        108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 103, 101, 116, 116,
                        101, 114,
                    ])
                },
            )
        } else {
            match completion.clone() {
                self::JavaScriptSemanticCompletion::NormalCompletion(value) => {
                    let value = *value;
                    self::accept_java_script_object_enumeration_value(
                        heap.clone(),
                        state.clone(),
                        value.clone(),
                    )
                }
                self::JavaScriptSemanticCompletion::ThrowCompletion(value_2) => {
                    let value_2 = *value_2;
                    self::JavaScriptObjectEnumerationTransition::ObjectEnumerationThrow(
                        Box::new(heap.clone()),
                        Box::new(value_2.clone()),
                    )
                }
            }
        }
    }
}
