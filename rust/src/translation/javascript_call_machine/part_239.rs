// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptCallFunction;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructionCall;
use super::JavaScriptConstructionState;
use super::JavaScriptConstructionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_executable_call_function;
use super::read_java_script_ordinary_value_error;
use super::source_ml_source3_record_order;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_count_java_script_leading_zero_bits_loop958(
    mut integer: f64,
    mut count: f64,
) -> f64 {
    loop {
        return if (integer > (0f64)) {
            {
                let integer_2 = (integer / 2f64).floor();
                {
                    let count_2 = (count - 1f64);
                    {
                        (integer, count) = (integer_2, count_2);
                        continue;
                    }
                }
            }
        } else {
            count
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_round_java_script_binary32_loop959(
    mut magnitude: f64,
    mut lower: f64,
    mut spacing: f64,
) -> f64 {
    loop {
        return if (magnitude >= (lower * 2f64)) {
            {
                let lower_2 = (lower * 2f64);
                {
                    let spacing_2 = (spacing * 2f64);
                    {
                        (lower, spacing) = (lower_2, spacing_2);
                        continue;
                    }
                }
            }
        } else {
            spacing
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop962Result {
    MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop962Done(
        Box<self::JavaScriptSemanticValue>,
        Box<self::JavaScriptSemanticValue>,
    ),
    MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop962Return(
        std::rc::Rc<
            std::cell::RefCell<(
                (
                    std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
                    self::ml_string::Text,
                    self::JavaScriptSemanticValue,
                    bool,
                    self::JavaScriptSemanticValue,
                ),
                Vec<f64>,
            )>,
        >,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop964Result {
    MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop964Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_bound_construction_operands_loop964(
    mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut source_ml_source1_temporary_values961: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index961: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop964Result {
    loop {
        return if (source_ml_source1_temporary_index961
            < ({
                let ml_reference_array2854 = source_ml_source1_temporary_values961.clone();
                let ml_reference_projection2854 =
                    self::ml::Big::from_u128(ml_reference_array2854.borrow().len() as u128);
                ml_reference_projection2854
            })) {
            {
                let value = {
                    let ml_reference_array2855 = source_ml_source1_temporary_values961.clone();
                    let ml_reference_position2855 = source_ml_source1_temporary_index961.clone();
                    let ml_reference_projection2855 = self::ml_array::at(
                        &*ml_reference_array2855.borrow(),
                        ml_reference_position2855.to_index(),
                    );
                    ml_reference_projection2855
                };
                {
                    let source_ml_source1_temporary_read_java_script_bound_construction_operands_ignored965 = {
                        let ml_method_owner2856 = arguments_.clone();
                        let ml_method_argument2856_0 = value.clone();
                        let ml_inserted = vec![ml_method_argument2856_0];
                        let mut ml_method_values2856 = ml_method_owner2856.borrow_mut();
                        let ml_length = ml_method_values2856
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4294967295usize)
                            .expect("Invalid array length");
                        ml_method_values2856.extend(ml_inserted);
                        ml_length as f64
                    };
                    {
                        let source_ml_source1_temporary_index961_2 =
                            source_ml_source1_temporary_index961.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index961 =
                                source_ml_source1_temporary_index961_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop964Result::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop964Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop963Result {
    MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop963Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop966Result {
    MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop966Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_bound_construction_operands_loop966(
    mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut source_ml_source1_temporary_values960: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index960: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop966Result {
    loop {
        return if (source_ml_source1_temporary_index960
            < ({
                let ml_reference_array2857 = source_ml_source1_temporary_values960.clone();
                let ml_reference_projection2857 =
                    self::ml::Big::from_u128(ml_reference_array2857.borrow().len() as u128);
                ml_reference_projection2857
            })) {
            {
                let value = {
                    let ml_reference_array2858 = source_ml_source1_temporary_values960.clone();
                    let ml_reference_position2858 = source_ml_source1_temporary_index960.clone();
                    let ml_reference_projection2858 = self::ml_array::at(
                        &*ml_reference_array2858.borrow(),
                        ml_reference_position2858.to_index(),
                    );
                    ml_reference_projection2858
                };
                {
                    let source_ml_source1_temporary_read_java_script_bound_construction_operands_ignored967 = {
                        let ml_method_owner2859 = arguments_.clone();
                        let ml_method_argument2859_0 = value.clone();
                        let ml_inserted = vec![ml_method_argument2859_0];
                        let mut ml_method_values2859 = ml_method_owner2859.borrow_mut();
                        let ml_length = ml_method_values2859
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4294967295usize)
                            .expect("Invalid array length");
                        ml_method_values2859.extend(ml_inserted);
                        ml_length as f64
                    };
                    {
                        let source_ml_source1_temporary_index960_2 =
                            source_ml_source1_temporary_index960.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index960 =
                                source_ml_source1_temporary_index960_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop966Result::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop966Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_bound_construction_operands_loop963(
    mut prefixes: std::rc::Rc<
        std::cell::RefCell<
            Vec<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>,
        >,
    >,
    mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop963Result {
    loop {
        return if (index >= (0f64)) {
            {
                let source_ml_source1_temporary_values960 = ({
                    let ml_reference_array2860 = prefixes.clone();
                    let ml_reference_position2860 = index;
                    let ml_reference_projection2860 = self::ml_array::optional_at(
                        &*ml_reference_array2860.borrow(),
                        self::ml_array::number_index(ml_reference_position2860),
                    );
                    ml_reference_projection2860
                })
                .clone()
                .unwrap_or_else(|| {
                    std::rc::Rc::new(std::cell::RefCell::new(
                        Vec::<self::JavaScriptSemanticValue>::new(),
                    ))
                });
                {
                    let source_ml_source1_temporary_index960 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s41198 = self::source_ml_source1_temporary_read_java_script_bound_construction_operands_loop966(arguments_.clone(), source_ml_source1_temporary_values960.clone(), source_ml_source1_temporary_index960.clone());
                        match source_ml_source2_s41198.clone() {
                            self::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop966Result::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop966Done => {
                                {
                                    let index_2 = (index - 1f64);
                                    {
                                        index = index_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop963Result::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop963Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_bound_construction_operands_loop962(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut state: self::JavaScriptConstructionState,
    mut target: self::JavaScriptSemanticValue,
    mut new_target: self::JavaScriptSemanticValue,
    mut total: f64,
    mut prefixes: std::rc::Rc<
        std::cell::RefCell<
            Vec<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>,
        >,
    >,
    mut depth: f64,
) -> self::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop962Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                shared_constructor_field1,
                _,
                _,
                _,
                _,
                _,
            ) => {
                if (depth
                    <= ({
                        let ml_reference_array2861 = shared_constructor_field1.clone();
                        let ml_reference_projection2861 =
                            ml_reference_array2861.borrow().len() as f64;
                        ml_reference_projection2861
                    }))
                {
                    match target.clone() {
                        self::JavaScriptSemanticValue::Object(identity, _) => {
                            {
                                let object = {
                                    let source_ml_source2_array_read_value41204 = shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_array_read_index41205 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                        { let ml_reference_array2862 = source_ml_source2_array_read_value41204.clone(); let ml_reference_position2862 = source_ml_source2_array_read_index41205; let ml_reference_projection2862 = self::ml_array::optional_at(&*ml_reference_array2862.borrow(), self::ml_array::number_index(ml_reference_position2862)); ml_reference_projection2862 }
                                    }
                                };
                                if (match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value41206, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value41206
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value41207, _, _, _, _) => {
                                        source_ml_source2_data_property_value41207
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value41208, _) => {
                                        source_ml_source2_data_property_value41208
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value41209, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value41209
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value41210, _, _) => {
                                        source_ml_source2_data_property_value41210
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value41211, _, _) => {
                                        source_ml_source2_data_property_value41211
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value41212, _, _) => {
                                        source_ml_source2_data_property_value41212
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value41213, _, _) => {
                                        source_ml_source2_data_property_value41213
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value41214, _) => {
                                        source_ml_source2_data_property_value41214
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value41215, _, _, _) => {
                                        source_ml_source2_data_property_value41215
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value41216, _, _, _) => {
                                        source_ml_source2_data_property_value41216
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value41217, _, _, _) => {
                                        source_ml_source2_data_property_value41217
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value41218, _, _, _) => {
                                        source_ml_source2_data_property_value41218
                                    }
                                })) {
                                    self::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop962Result::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop962Return(std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_field41221 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                        {
                                            let source_ml_source3_record_field538 = source_ml_source2_field41221.clone();
                                            {
                                                let source_ml_source3_record_field539 = self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 116, 97, 114, 103, 101, 116, 32, 99, 104, 97, 105, 110]);
                                                {
                                                    let source_ml_source3_record_field540 = new_target.clone();
                                                    {
                                                        let source_ml_source3_record_field541 = false;
                                                        {
                                                            let source_ml_source3_record_field542 = target.clone();
                                                            ((source_ml_source3_record_field538.clone(), source_ml_source3_record_field539.clone(), source_ml_source3_record_field540.clone(), source_ml_source3_record_field541, source_ml_source3_record_field542.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![110f64, 101f64, 119f64, 84f64, 97f64, 114f64, 103f64, 101f64, 116f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64], vec![116f64, 97f64, 114f64, 103f64, 101f64, 116f64]], vec![4f64, 2f64, 0f64, 1f64, 3f64]))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    })))
                                } else {
                                    match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, bound_target, _, bound_arguments) => {
                                            let bound_target = *bound_target;
                                            {
                                                let total_2 = (total + { let ml_reference_array2863 = bound_arguments.clone(); let ml_reference_projection2863 = ml_reference_array2863.borrow().len() as f64; ml_reference_projection2863 });
                                                if (total_2 > (1048576f64)) {
                                                    self::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop962Result::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop962Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                        let source_ml_source2_field41232 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                        {
                                                            let source_ml_source3_record_field543 = source_ml_source2_field41232.clone();
                                                            {
                                                                let source_ml_source3_record_field544 = self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 115, 104, 97, 114, 101, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 115, 116, 111, 114, 97, 103, 101]);
                                                                {
                                                                    let source_ml_source3_record_field545 = new_target.clone();
                                                                    {
                                                                        let source_ml_source3_record_field546 = true;
                                                                        {
                                                                            let source_ml_source3_record_field547 = target.clone();
                                                                            ((source_ml_source3_record_field543.clone(), source_ml_source3_record_field544.clone(), source_ml_source3_record_field545.clone(), source_ml_source3_record_field546, source_ml_source3_record_field547.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![110f64, 101f64, 119f64, 84f64, 97f64, 114f64, 103f64, 101f64, 116f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64], vec![116f64, 97f64, 114f64, 103f64, 101f64, 116f64]], vec![4f64, 2f64, 0f64, 1f64, 3f64]))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    })))
                                                } else {
                                                    {
                                                        let source_ml_source1_temporary_read_java_script_bound_construction_operands_ignored968 = { let ml_method_owner2864 = prefixes.clone(); let ml_method_argument2864_0 = bound_arguments.clone(); let ml_inserted = vec![ml_method_argument2864_0]; let mut ml_method_values2864 = ml_method_owner2864.borrow_mut(); let ml_length = ml_method_values2864.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values2864.extend(ml_inserted); ml_length as f64 };
                                                        {
                                                            let new_target_2 = if self::test_java_script_semantic_same_value(target.clone(), new_target.clone()) {
                                                                bound_target.clone()
                                                            } else {
                                                                new_target.clone()
                                                            };
                                                            {
                                                                let depth_2 = (depth + 1f64);
                                                                {
                                                                    (target, new_target, total, depth) = (bound_target.clone(), new_target_2.clone(), total_2, depth_2);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            {
                                                let arguments_ = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                {
                                                    let index = ({ let ml_reference_array2865 = prefixes.clone(); let ml_reference_projection2865 = ml_reference_array2865.borrow().len() as f64; ml_reference_projection2865 } - 1f64);
                                                    {
                                                        let source_ml_source2_s41235 = self::source_ml_source1_temporary_read_java_script_bound_construction_operands_loop963(prefixes.clone(), arguments_.clone(), index);
                                                        match source_ml_source2_s41235.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop963Result::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop963Done => {
                                                                {
                                                                    let source_ml_source1_temporary_values961 = match state.clone() {
                                                                        self::JavaScriptConstructionState::ConstructionState(_, _, source_ml_source2_data_property_value41236, _, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value41236.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_temporary_index961 = self::ml::Big::from_i128(0);
                                                                        {
                                                                            let source_ml_source2_s41237 = self::source_ml_source1_temporary_read_java_script_bound_construction_operands_loop964(arguments_.clone(), source_ml_source1_temporary_values961.clone(), source_ml_source1_temporary_index961.clone());
                                                                            match source_ml_source2_s41237.clone() {
                                                                                self::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop964Result::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop964Done => {
                                                                                    self::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop962Result::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop962Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                                                        let source_ml_source3_record_field548 = arguments_.clone();
                                                                                        {
                                                                                            let source_ml_source3_record_field549 = self::ml_string::Text::from_units(vec![]);
                                                                                            {
                                                                                                let source_ml_source3_record_field550 = new_target.clone();
                                                                                                {
                                                                                                    let source_ml_source3_record_field551 = false;
                                                                                                    {
                                                                                                        let source_ml_source3_record_field552 = target.clone();
                                                                                                        ((source_ml_source3_record_field548.clone(), source_ml_source3_record_field549.clone(), source_ml_source3_record_field550.clone(), source_ml_source3_record_field551, source_ml_source3_record_field552.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![110f64, 101f64, 119f64, 84f64, 97f64, 114f64, 103f64, 101f64, 116f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64], vec![116f64, 97f64, 114f64, 103f64, 101f64, 116f64]], vec![4f64, 2f64, 0f64, 1f64, 3f64]))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    })))
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
                        _ => {
                            self::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop962Result::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop962Return(std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_field41245 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                {
                                    let source_ml_source3_record_field553 = source_ml_source2_field41245.clone();
                                    {
                                        let source_ml_source3_record_field554 = self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 116, 97, 114, 103, 101, 116, 32, 99, 104, 97, 105, 110]);
                                        {
                                            let source_ml_source3_record_field555 = new_target.clone();
                                            {
                                                let source_ml_source3_record_field556 = false;
                                                {
                                                    let source_ml_source3_record_field557 = target.clone();
                                                    ((source_ml_source3_record_field553.clone(), source_ml_source3_record_field554.clone(), source_ml_source3_record_field555.clone(), source_ml_source3_record_field556, source_ml_source3_record_field557.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![110f64, 101f64, 119f64, 84f64, 97f64, 114f64, 103f64, 101f64, 116f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64], vec![116f64, 97f64, 114f64, 103f64, 101f64, 116f64]], vec![4f64, 2f64, 0f64, 1f64, 3f64]))
                                                }
                                            }
                                        }
                                    }
                                }
                            })))
                        }
                    }
                } else {
                    self::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop962Result::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop962Done(Box::new(target.clone()), Box::new(new_target.clone()))
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop975Result {
    MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop975Done,
    MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop975Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_construction_state_error_loop975(
    mut program: self::JavaScriptCallProgram,
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values973: std::rc::Rc<
        std::cell::RefCell<Vec<Option<self::JavaScriptArrayConstructorFact>>>,
    >,
    mut source_ml_source1_temporary_index973: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop975Result {
    loop {
        return if (source_ml_source1_temporary_index973
            < ({
                let ml_reference_array2866 = source_ml_source1_temporary_values973.clone();
                let ml_reference_projection2866 =
                    self::ml::Big::from_u128(ml_reference_array2866.borrow().len() as u128);
                ml_reference_projection2866
            })) {
            {
                let fact = {
                    let ml_reference_array2867 = source_ml_source1_temporary_values973.clone();
                    let ml_reference_position2867 = source_ml_source1_temporary_index973.clone();
                    let ml_reference_projection2867 = self::ml_array::at(
                        &*ml_reference_array2867.borrow(),
                        ml_reference_position2867.to_index(),
                    );
                    ml_reference_projection2867
                };
                if !(match (fact.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) {
                    {
                        let fn_ = self::read_java_script_executable_call_function(
                            program.clone(),
                            heap.clone(),
                            match fact.clone().expect("a proven present value") {
                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                    source_ml_source2_data_property_value41250,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value41250.clone(),
                            },
                        );
                        if (!(match (fn_.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }) && ((match fn_.clone().expect("a proven present value") {
                            self::JavaScriptCallFunction::CallFunction(
                                _,
                                source_ml_source2_data_property_value41253,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41253.clone(),
                        }) != (match fact.clone().expect("a proven present value") {
                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                _,
                                _,
                                source_ml_source2_data_property_value41254,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41254.clone(),
                        }))) {
                            self::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop975Result::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop975Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 97, 108, 109, 32, 114, 101, 99, 111, 114, 100, 101, 100, 32, 98, 121, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))
                        } else {
                            {
                                let source_ml_source1_temporary_index973_2 =
                                    source_ml_source1_temporary_index973
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index973 =
                                        source_ml_source1_temporary_index973_2.clone();
                                    continue;
                                }
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index973_3 =
                            source_ml_source1_temporary_index973.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index973 =
                                source_ml_source1_temporary_index973_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop975Result::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop975Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop974Result {
    MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop974Done,
    MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop974Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_construction_state_error_loop974(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values972: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index972: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop974Result {
    loop {
        return if (source_ml_source1_temporary_index972
            < ({
                let ml_reference_array2868 = source_ml_source1_temporary_values972.clone();
                let ml_reference_projection2868 =
                    self::ml::Big::from_u128(ml_reference_array2868.borrow().len() as u128);
                ml_reference_projection2868
            })) {
            {
                let value = {
                    let ml_reference_array2869 = source_ml_source1_temporary_values972.clone();
                    let ml_reference_position2869 = source_ml_source1_temporary_index972.clone();
                    let ml_reference_projection2869 = self::ml_array::at(
                        &*ml_reference_array2869.borrow(),
                        ml_reference_position2869.to_index(),
                    );
                    ml_reference_projection2869
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop974Result::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop974Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index972_2 =
                                source_ml_source1_temporary_index972
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index972 =
                                    source_ml_source1_temporary_index972_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop974Result::MlSource1TemporaryReadJavaScriptConstructionStateErrorLoop974Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980 {
    MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980Next,
    MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop982Result {
    MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop982Done,
    MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop982Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_construction_completion_state_error_loop982(
    mut state: self::JavaScriptConstructionState,
    mut call: self::JavaScriptConstructionCall,
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop982Result {
    loop {
        return match state.clone() {
            self::JavaScriptConstructionState::ConstructionState(
                _,
                _,
                shared_constructor_field1,
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
            ) => {
                if (index
                    < ({
                        let ml_reference_array2870 = shared_constructor_field1.clone();
                        let ml_reference_projection2870 =
                            ml_reference_array2870.borrow().len() as f64;
                        ml_reference_projection2870
                    }))
                {
                    {
                        let selected = {
                            let source_ml_source2_array_read_value41260 = match call.clone() {
                                self::JavaScriptConstructionCall::ConstructionCallRecord(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value41259,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value41259.clone(),
                            };
                            {
                                let ml_reference_array2871 =
                                    source_ml_source2_array_read_value41260.clone();
                                let ml_reference_position2871 = index;
                                let ml_reference_projection2871 = self::ml_array::optional_at(
                                    &*ml_reference_array2871.borrow(),
                                    self::ml_array::number_index(ml_reference_position2871),
                                );
                                ml_reference_projection2871
                            }
                        };
                        {
                            let actual = {
                                let source_ml_source2_array_read_value41263 =
                                    shared_constructor_field1.clone();
                                {
                                    let ml_reference_array2872 =
                                        source_ml_source2_array_read_value41263.clone();
                                    let ml_reference_position2872 = index;
                                    let ml_reference_projection2872 = self::ml_array::optional_at(
                                        &*ml_reference_array2872.borrow(),
                                        self::ml_array::number_index(ml_reference_position2872),
                                    );
                                    ml_reference_projection2872
                                }
                            };
                            if ((match (selected.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } || match (actual.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            }) || !(self::test_java_script_semantic_same_value(
                                selected.clone().expect("a proven present value"),
                                actual.clone().expect("a proven present value"),
                            ))) {
                                self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop982Result::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop982Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 98, 111, 100, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 111, 114, 100, 101, 114, 101, 100, 32, 118, 97, 108, 117, 101, 115]))
                            } else {
                                {
                                    let index_2 = (index + 1f64);
                                    {
                                        index = index_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop982Result::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop982Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop981Result {
    MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop981Done,
    MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop981Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_construction_completion_state_error_loop981(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values976: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index976: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop981Result {
    loop {
        return if (source_ml_source1_temporary_index976
            < ({
                let ml_reference_array2873 = source_ml_source1_temporary_values976.clone();
                let ml_reference_projection2873 =
                    self::ml::Big::from_u128(ml_reference_array2873.borrow().len() as u128);
                ml_reference_projection2873
            })) {
            {
                let value = {
                    let ml_reference_array2874 = source_ml_source1_temporary_values976.clone();
                    let ml_reference_position2874 = source_ml_source1_temporary_index976.clone();
                    let ml_reference_projection2874 = self::ml_array::at(
                        &*ml_reference_array2874.borrow(),
                        ml_reference_position2874.to_index(),
                    );
                    ml_reference_projection2874
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop981Result::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop981Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index976_2 =
                                source_ml_source1_temporary_index976
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index976 =
                                    source_ml_source1_temporary_index976_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop981Result::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop981Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979 {
    MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979Next,
    MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin978 {
    MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin978Next,
    MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin978Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin977 {
    MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin977Next,
    MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin977Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryStepJavaScriptConstructionJoin985 {
    MlSource1TemporaryStepJavaScriptConstructionJoin985Next,
    MlSource1TemporaryStepJavaScriptConstructionJoin985Return(
        Box<self::JavaScriptConstructionTransition>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987 {
    MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987Next,
    MlSource1TemporaryStepJavaScriptConstructionAllocationJoin987Return(
        Box<self::JavaScriptConstructionTransition>,
    ),
}
