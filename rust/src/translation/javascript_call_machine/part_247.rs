// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCollectionForEachTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::JavaScriptSymbolRegistryEntry;
use super::JavaScriptSymbolRegistryTransition;
use super::MlOptional;
use super::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1155Result;
use super::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1156Result;
use super::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1157Result;
use super::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1158Result;
use super::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1159Result;
use super::compare_java_script_collection_keys;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_symbol_registration_fact;
use super::source_ml_source1_temporary_order_java_script_symbol_registry_entries_loop1158;
use super::source_ml_source1_temporary_order_java_script_symbol_registry_entries_loop1159;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_symbol_registry_entries_loop1157(
    mut entries: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut target: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut middle: f64,
    mut end: f64,
    mut left: f64,
    mut right: f64,
) -> self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1157Result {
    loop {
        return if ((left < middle) && (right < end)) {
            {
                let first = {
                    let ml_reference_array2988 = source.clone();
                    let ml_reference_position2988 = left;
                    let ml_reference_projection2988 = self::ml_array::optional_at(
                        &*ml_reference_array2988.borrow(),
                        self::ml_array::number_index(ml_reference_position2988),
                    );
                    ml_reference_projection2988
                };
                {
                    let second = {
                        let ml_reference_array2989 = source.clone();
                        let ml_reference_position2989 = right;
                        let ml_reference_projection2989 = self::ml_array::optional_at(
                            &*ml_reference_array2989.borrow(),
                            self::ml_array::number_index(ml_reference_position2989),
                        );
                        ml_reference_projection2989
                    };
                    if (match (first.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } || match (second.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1157Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1157Return(entries.clone())
                    } else {
                        if ((self::compare_java_script_collection_keys(
                            match first.clone().expect("a proven present value") {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    _,
                                    source_ml_source2_data_property_value41718,
                                ) => {
                                    let source_ml_source2_data_property_value41718 =
                                        *source_ml_source2_data_property_value41718;
                                    source_ml_source2_data_property_value41718.clone()
                                }
                            },
                            match second.clone().expect("a proven present value") {
                                self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                    _,
                                    source_ml_source2_data_property_value41719,
                                ) => {
                                    let source_ml_source2_data_property_value41719 =
                                        *source_ml_source2_data_property_value41719;
                                    source_ml_source2_data_property_value41719.clone()
                                }
                            },
                        )) <= (0f64))
                        {
                            {
                                let source_ml_source1_temporary_order_java_script_symbol_registry_entries_ignored1162 = {
                                    let ml_method_owner2990 = target.clone();
                                    let ml_method_argument2990_0 =
                                        first.clone().expect("a proven present value");
                                    let ml_inserted = vec![ml_method_argument2990_0];
                                    let mut ml_method_values2990 = ml_method_owner2990.borrow_mut();
                                    let ml_length = ml_method_values2990
                                        .len()
                                        .checked_add(ml_inserted.len())
                                        .filter(|length| *length <= 4294967295usize)
                                        .expect("Invalid array length");
                                    ml_method_values2990.extend(ml_inserted);
                                    ml_length as f64
                                };
                                {
                                    let left_2 = (left + 1f64);
                                    {
                                        left = left_2;
                                        continue;
                                    }
                                }
                            }
                        } else {
                            {
                                let source_ml_source1_temporary_order_java_script_symbol_registry_entries_ignored1163 = {
                                    let ml_method_owner2991 = target.clone();
                                    let ml_method_argument2991_0 =
                                        second.clone().expect("a proven present value");
                                    let ml_inserted = vec![ml_method_argument2991_0];
                                    let mut ml_method_values2991 = ml_method_owner2991.borrow_mut();
                                    let ml_length = ml_method_values2991
                                        .len()
                                        .checked_add(ml_inserted.len())
                                        .filter(|length| *length <= 4294967295usize)
                                        .expect("Invalid array length");
                                    ml_method_values2991.extend(ml_inserted);
                                    ml_length as f64
                                };
                                {
                                    let right_2 = (right + 1f64);
                                    {
                                        right = right_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1157Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1157Done(left, right)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_symbol_registry_entries_loop1156(
    mut entries: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut width: f64,
    mut target: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut start: f64,
) -> self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1156Result {
    loop {
        return if (start
            < ({
                let ml_reference_array2992 = source.clone();
                let ml_reference_projection2992 = ml_reference_array2992.borrow().len() as f64;
                ml_reference_projection2992
            })) {
            {
                let middle = self::ml_math::min((start + width), {
                    let ml_reference_array2993 = source.clone();
                    let ml_reference_projection2993 = ml_reference_array2993.borrow().len() as f64;
                    ml_reference_projection2993
                });
                {
                    let end = self::ml_math::min((start + (width * 2f64)), {
                        let ml_reference_array2994 = source.clone();
                        let ml_reference_projection2994 =
                            ml_reference_array2994.borrow().len() as f64;
                        ml_reference_projection2994
                    });
                    {
                        let left = start;
                        {
                            let right = middle;
                            {
                                let source_ml_source2_s41720 = self::source_ml_source1_temporary_order_java_script_symbol_registry_entries_loop1157(entries.clone(), source.clone(), target.clone(), middle, end, left, right);
                                match source_ml_source2_s41720.clone() {
                                    self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1157Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1157Done(left_2, right_2) => {
                                        {
                                            let source_ml_source2_s41721 = self::source_ml_source1_temporary_order_java_script_symbol_registry_entries_loop1158(entries.clone(), source.clone(), target.clone(), middle, left_2);
                                            match source_ml_source2_s41721.clone() {
                                                self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1158Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1158Done => {
                                                    {
                                                        let source_ml_source2_s41722 = self::source_ml_source1_temporary_order_java_script_symbol_registry_entries_loop1159(entries.clone(), source.clone(), target.clone(), end, right_2);
                                                        match source_ml_source2_s41722.clone() {
                                                            self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1159Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1159Done => {
                                                                {
                                                                    let start_2 = (start + (width * 2f64));
                                                                    {
                                                                        start = start_2;
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1159Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1159Return(source_ml_source1_temporary_result1159) => {
                                                                self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1156Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1156Return(source_ml_source1_temporary_result1159.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1158Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1158Return(source_ml_source1_temporary_result1160) => {
                                                    self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1156Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1156Return(source_ml_source1_temporary_result1160.clone())
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1157Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1157Return(source_ml_source1_temporary_result1161) => {
                                        self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1156Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1156Return(source_ml_source1_temporary_result1161.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1156Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1156Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_symbol_registry_entries_loop1155(
    mut entries: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut width: f64,
) -> self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1155Result {
    loop {
        return if (width
            < ({
                let ml_reference_array2995 = source.clone();
                let ml_reference_projection2995 = ml_reference_array2995.borrow().len() as f64;
                ml_reference_projection2995
            })) {
            {
                let target = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                    self::JavaScriptSymbolRegistryEntry,
                >::new()));
                {
                    let start = 0f64;
                    {
                        let source_ml_source2_s41723 = self::source_ml_source1_temporary_order_java_script_symbol_registry_entries_loop1156(entries.clone(), source.clone(), width, target.clone(), start);
                        match source_ml_source2_s41723.clone() {
                            self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1156Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1156Done => {
                                {
                                    let width_2 = (width * 2f64);
                                    {
                                        (source, width) = (target.clone(), width_2);
                                        continue;
                                    }
                                }
                            }
                            self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1156Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1156Return(source_ml_source1_temporary_result1156) => {
                                self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1155Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1155Return(source_ml_source1_temporary_result1156.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1155Result::MlSource1TemporaryOrderJavaScriptSymbolRegistryEntriesLoop1155Done(source.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryInitializeJavaScriptSymbolRegistryDirectoryLoop1165Result {
    MlSource1TemporaryInitializeJavaScriptSymbolRegistryDirectoryLoop1165Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_initialize_java_script_symbol_registry_directory_loop1165(
    mut entries: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut source_ml_source1_temporary_values1164: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>,
    >,
    mut source_ml_source1_temporary_index1164: self::ml::Big,
) -> self::MlSource1TemporaryInitializeJavaScriptSymbolRegistryDirectoryLoop1165Result {
    loop {
        return if (source_ml_source1_temporary_index1164
            < ({
                let ml_reference_array2996 = source_ml_source1_temporary_values1164.clone();
                let ml_reference_projection2996 =
                    self::ml::Big::from_u128(ml_reference_array2996.borrow().len() as u128);
                ml_reference_projection2996
            })) {
            {
                let fact = {
                    let ml_reference_array2997 = source_ml_source1_temporary_values1164.clone();
                    let ml_reference_position2997 = source_ml_source1_temporary_index1164.clone();
                    let ml_reference_projection2997 = self::ml_array::at(
                        &*ml_reference_array2997.borrow(),
                        ml_reference_position2997.to_index(),
                    );
                    ml_reference_projection2997
                };
                {
                    let key = match fact.clone() {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            _,
                            _,
                            source_ml_source2_data_property_value41726,
                        ) => ((source_ml_source2_data_property_value41726
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                        .0)
                            .clone(),
                    };
                    if !(match (key.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        {
                            let source_ml_source1_temporary_initialize_java_script_symbol_registry_directory_ignored1166 = {
                                let ml_method_owner2998 = entries.clone();
                                let ml_method_argument2998_0 = self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(match fact.clone() {
                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source2_data_property_value41727, _, _) => {
                                    source_ml_source2_data_property_value41727.clone()
                                }
                            }, Box::new(key.clone().expect("a proven present value")));
                                let ml_inserted = vec![ml_method_argument2998_0];
                                let mut ml_method_values2998 = ml_method_owner2998.borrow_mut();
                                let ml_length = ml_method_values2998
                                    .len()
                                    .checked_add(ml_inserted.len())
                                    .filter(|length| *length <= 4294967295usize)
                                    .expect("Invalid array length");
                                ml_method_values2998.extend(ml_inserted);
                                ml_length as f64
                            };
                            {
                                let source_ml_source1_temporary_index1164_2 =
                                    source_ml_source1_temporary_index1164
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index1164 =
                                        source_ml_source1_temporary_index1164_2.clone();
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_source1_temporary_index1164_3 =
                                source_ml_source1_temporary_index1164
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1164 =
                                    source_ml_source1_temporary_index1164_3.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryInitializeJavaScriptSymbolRegistryDirectoryLoop1165Result::MlSource1TemporaryInitializeJavaScriptSymbolRegistryDirectoryLoop1165Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptSymbolRegistryKeyLoop1167Result {
    MlSource1TemporaryFindJavaScriptSymbolRegistryKeyLoop1167Done(f64),
    MlSource1TemporaryFindJavaScriptSymbolRegistryKeyLoop1167Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_symbol_registry_key_loop1167(
    mut key: self::JavaScriptSemanticValue,
    mut entries: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>,
    mut low: f64,
    mut high: f64,
) -> self::MlSource1TemporaryFindJavaScriptSymbolRegistryKeyLoop1167Result {
    loop {
        return if (low < high) {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let entry = {
                        let ml_reference_array2999 = entries.clone();
                        let ml_reference_position2999 = middle;
                        let ml_reference_projection2999 = self::ml_array::optional_at(
                            &*ml_reference_array2999.borrow(),
                            self::ml_array::number_index(ml_reference_position2999),
                        );
                        ml_reference_projection2999
                    };
                    if match (entry.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlSource1TemporaryFindJavaScriptSymbolRegistryKeyLoop1167Result::MlSource1TemporaryFindJavaScriptSymbolRegistryKeyLoop1167Return((-1f64))
                    } else {
                        {
                            let order = self::compare_java_script_collection_keys(
                                match entry.clone().expect("a proven present value") {
                                    self::JavaScriptSymbolRegistryEntry::SymbolRegistryEntry(
                                        _,
                                        source_ml_source2_data_property_value41730,
                                    ) => {
                                        let source_ml_source2_data_property_value41730 =
                                            *source_ml_source2_data_property_value41730;
                                        source_ml_source2_data_property_value41730.clone()
                                    }
                                },
                                key.clone(),
                            );
                            if (order == (0f64)) {
                                self::MlSource1TemporaryFindJavaScriptSymbolRegistryKeyLoop1167Result::MlSource1TemporaryFindJavaScriptSymbolRegistryKeyLoop1167Return(middle)
                            } else {
                                if (order < (0f64)) {
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
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptSymbolRegistryKeyLoop1167Result::MlSource1TemporaryFindJavaScriptSymbolRegistryKeyLoop1167Done(low)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_symbol_registration_position_loop1168(
    mut facts: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>,
    mut identity: self::ml::Big,
    mut low: f64,
    mut high: f64,
) -> f64 {
    loop {
        return if (low < high) {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let fact = {
                        let ml_reference_array3000 = facts.clone();
                        let ml_reference_position3000 = middle;
                        let ml_reference_projection3000 = self::ml_array::optional_at(
                            &*ml_reference_array3000.borrow(),
                            self::ml_array::number_index(ml_reference_position3000),
                        );
                        ml_reference_projection3000
                    };
                    if (!(match (fact.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) && ((match fact.clone().expect("a proven present value") {
                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                            source_ml_source2_data_property_value41734,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value41734.clone(),
                    }) < identity))
                    {
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
        } else {
            low
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptSymbolRegistryOperationLoop1173Result {
    MlSource1TemporaryExecuteJavaScriptSymbolRegistryOperationLoop1173Done,
    MlSource1TemporaryExecuteJavaScriptSymbolRegistryOperationLoop1173Return(
        Box<self::JavaScriptSymbolRegistryTransition>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_symbol_registry_operation_loop1173(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1172: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    mut source_ml_source1_temporary_index1172: self::ml::Big,
) -> self::MlSource1TemporaryExecuteJavaScriptSymbolRegistryOperationLoop1173Result {
    loop {
        return if (source_ml_source1_temporary_index1172
            < ({
                let ml_reference_array3001 = source_ml_source1_temporary_values1172.clone();
                let ml_reference_projection3001 =
                    self::ml::Big::from_u128(ml_reference_array3001.borrow().len() as u128);
                ml_reference_projection3001
            })) {
            {
                let identity = {
                    let ml_reference_array3002 = source_ml_source1_temporary_values1172.clone();
                    let ml_reference_position3002 = source_ml_source1_temporary_index1172.clone();
                    let ml_reference_projection3002 = self::ml_array::at(
                        &*ml_reference_array3002.borrow(),
                        ml_reference_position3002.to_index(),
                    );
                    ml_reference_projection3002
                };
                {
                    let fact = self::read_java_script_symbol_registration_fact(
                        heap.clone(),
                        identity.clone(),
                    );
                    if (match (fact.clone(), None::<()>) {
                        (self::MlOptional::Value(_ml_left), Some(_ml_right)) => false,
                        (self::MlOptional::Null, None) => false,
                        (self::MlOptional::Undefined, None) => true,
                        _ => false,
                    } || ({
                        let source_ml_source2_nullish_field_receiver41737 = (fact.clone(),);
                        if match (source_ml_source2_nullish_field_receiver41737.clone()).0 {
                            self::MlOptional::Value(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            self::MlOptional::Null => false,
                            self::MlOptional::Undefined => false,
                        } {
                            {
                                let source_ml_source2_nullish_field_present41738 =
                                    match (source_ml_source2_nullish_field_receiver41737.clone()).0
                                    {
                                        self::MlOptional::Value(ml_present) => ml_present,
                                        _ => unreachable!("a proven present value"),
                                    };
                                match source_ml_source2_nullish_field_present41738.clone() {
                                    self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, source_ml_source2_data_property_value41739, _) => {
                                        source_ml_source2_data_property_value41739
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units:
                                    (if ((match (source_ml_source2_nullish_field_receiver41737
                                        .clone())
                                    .0
                                    {
                                        self::MlOptional::Value(ml_present) => {
                                            let _ = ml_present;
                                            self::ml_string::Text::from_units(vec![
                                                111, 98, 106, 101, 99, 116,
                                            ])
                                        }
                                        self::MlOptional::Null => {
                                            self::ml_string::Text::from_units(vec![
                                                111, 98, 106, 101, 99, 116,
                                            ])
                                        }
                                        self::MlOptional::Undefined => {
                                            self::ml_string::Text::from_units(vec![
                                                117, 110, 100, 101, 102, 105, 110, 101, 100,
                                            ])
                                        }
                                    }) == (self::ml_string::Text::from_units(vec![
                                        111, 98, 106, 101, 99, 116,
                                    ]))) {
                                        self::ml_string::Text::from_units(vec![
                                            67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32,
                                            112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32,
                                            111, 102, 32, 110, 117, 108, 108, 32, 40, 114, 101, 97,
                                            100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115,
                                            116, 101, 114, 101, 100, 39, 41,
                                        ])
                                    } else {
                                        self::ml_string::Text::from_units(vec![
                                            67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32,
                                            112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32,
                                            111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101,
                                            100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39,
                                            114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 39,
                                            41,
                                        ])
                                    })
                                    .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    } && match (
                        {
                            let source_ml_source2_nullish_field_receiver41740 = (fact.clone(),);
                            if match (source_ml_source2_nullish_field_receiver41740.clone()).0 {
                                self::MlOptional::Value(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                self::MlOptional::Null => false,
                                self::MlOptional::Undefined => false,
                            } {
                                {
                                    let source_ml_source2_nullish_field_present41741 =
                                        match (source_ml_source2_nullish_field_receiver41740
                                            .clone())
                                        .0
                                        {
                                            self::MlOptional::Value(ml_present) => ml_present,
                                            _ => unreachable!("a proven present value"),
                                        };
                                    match source_ml_source2_nullish_field_present41741.clone() {
                                    self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, _, source_ml_source2_data_property_value41742) => {
                                        ((source_ml_source2_data_property_value41742.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                    }
                                }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units:
                                        (if ((match (source_ml_source2_nullish_field_receiver41740
                                            .clone())
                                        .0
                                        {
                                            self::MlOptional::Value(ml_present) => {
                                                let _ = ml_present;
                                                self::ml_string::Text::from_units(vec![
                                                    111, 98, 106, 101, 99, 116,
                                                ])
                                            }
                                            self::MlOptional::Null => {
                                                self::ml_string::Text::from_units(vec![
                                                    111, 98, 106, 101, 99, 116,
                                                ])
                                            }
                                            self::MlOptional::Undefined => {
                                                self::ml_string::Text::from_units(vec![
                                                    117, 110, 100, 101, 102, 105, 110, 101, 100,
                                                ])
                                            }
                                        }) == (self::ml_string::Text::from_units(vec![
                                            111, 98, 106, 101, 99, 116,
                                        ]))) {
                                            self::ml_string::Text::from_units(vec![
                                                67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100,
                                                32, 112, 114, 111, 112, 101, 114, 116, 105, 101,
                                                115, 32, 111, 102, 32, 110, 117, 108, 108, 32, 40,
                                                114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101,
                                                121, 39, 41,
                                            ])
                                        } else {
                                            self::ml_string::Text::from_units(vec![
                                                67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100,
                                                32, 112, 114, 111, 112, 101, 114, 116, 105, 101,
                                                115, 32, 111, 102, 32, 117, 110, 100, 101, 102,
                                                105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105,
                                                110, 103, 32, 39, 107, 101, 121, 39, 41,
                                            ])
                                        })
                                        .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    })) {
                        self::MlSource1TemporaryExecuteJavaScriptSymbolRegistryOperationLoop1173Result::MlSource1TemporaryExecuteJavaScriptSymbolRegistryOperationLoop1173Return(Box::new(self::JavaScriptSymbolRegistryTransition::SymbolRegistryInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![110, 101, 119, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 107, 101, 121, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 109, 112, 111, 114, 116, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 47, 107, 101, 121, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))))
                    } else {
                        {
                            let source_ml_source1_temporary_index1172_2 =
                                source_ml_source1_temporary_index1172
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1172 =
                                    source_ml_source1_temporary_index1172_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptSymbolRegistryOperationLoop1173Result::MlSource1TemporaryExecuteJavaScriptSymbolRegistryOperationLoop1173Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryImportJavaScriptSymbolRegistryWitnessJoin1177 {
    MlSource1TemporaryImportJavaScriptSymbolRegistryWitnessJoin1177Next(
        Box<self::JavaScriptOrdinaryHeap>,
    ),
    MlSource1TemporaryImportJavaScriptSymbolRegistryWitnessJoin1177Return(
        Box<self::JavaScriptSymbolRegistryTransition>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1180Result {
    MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1180Done,
    MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1180Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_collection_for_each_state_error_loop1180(
    mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut expected: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1180Result {
    loop {
        return if (at
            < ({
                let ml_reference_array3003 = expected.clone();
                let ml_reference_projection3003 = ml_reference_array3003.borrow().len() as f64;
                ml_reference_projection3003
            })) {
            if !(self::test_java_script_semantic_same_value(
                ({
                    let ml_reference_array3004 = arguments_.clone();
                    let ml_reference_position3004 = at;
                    let ml_reference_projection3004 = self::ml_array::optional_at(
                        &*ml_reference_array3004.borrow(),
                        self::ml_array::number_index(ml_reference_position3004),
                    );
                    ml_reference_projection3004
                })
                .clone()
                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
                ({
                    let ml_reference_array3005 = expected.clone();
                    let ml_reference_position3005 = at;
                    let ml_reference_projection3005 = self::ml_array::optional_at(
                        &*ml_reference_array3005.borrow(),
                        self::ml_array::number_index(ml_reference_position3005),
                    );
                    ml_reference_projection3005
                })
                .clone()
                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
            )) {
                self::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1180Result::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1180Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 102, 111, 114, 69, 97, 99, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 101, 114, 101, 100, 32, 118, 97, 108, 117, 101, 32, 107, 101, 121, 32, 97, 110, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1180Result::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1180Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1179Result {
    MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1179Done,
    MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1179Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_collection_for_each_state_error_loop1179(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1178: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1178: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1179Result {
    loop {
        return if (source_ml_source1_temporary_index1178
            < ({
                let ml_reference_array3006 = source_ml_source1_temporary_values1178.clone();
                let ml_reference_projection3006 =
                    self::ml::Big::from_u128(ml_reference_array3006.borrow().len() as u128);
                ml_reference_projection3006
            })) {
            {
                let value = {
                    let ml_reference_array3007 = source_ml_source1_temporary_values1178.clone();
                    let ml_reference_position3007 = source_ml_source1_temporary_index1178.clone();
                    let ml_reference_projection3007 = self::ml_array::at(
                        &*ml_reference_array3007.borrow(),
                        ml_reference_position3007.to_index(),
                    );
                    ml_reference_projection3007
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1179Result::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1179Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1178_2 =
                                source_ml_source1_temporary_index1178
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1178 =
                                    source_ml_source1_temporary_index1178_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1179Result::MlSource1TemporaryReadJavaScriptCollectionForEachStateErrorLoop1179Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryBeginJavaScriptCollectionForEachLoop1182Result {
    MlSource1TemporaryBeginJavaScriptCollectionForEachLoop1182Done,
    MlSource1TemporaryBeginJavaScriptCollectionForEachLoop1182Return(
        Box<self::JavaScriptCollectionForEachTransition>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_begin_java_script_collection_for_each_loop1182(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1181: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1181: self::ml::Big,
) -> self::MlSource1TemporaryBeginJavaScriptCollectionForEachLoop1182Result {
    loop {
        return if (source_ml_source1_temporary_index1181
            < ({
                let ml_reference_array3008 = source_ml_source1_temporary_values1181.clone();
                let ml_reference_projection3008 =
                    self::ml::Big::from_u128(ml_reference_array3008.borrow().len() as u128);
                ml_reference_projection3008
            })) {
            {
                let value = {
                    let ml_reference_array3009 = source_ml_source1_temporary_values1181.clone();
                    let ml_reference_position3009 = source_ml_source1_temporary_index1181.clone();
                    let ml_reference_projection3009 = self::ml_array::at(
                        &*ml_reference_array3009.borrow(),
                        ml_reference_position3009.to_index(),
                    );
                    ml_reference_projection3009
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryBeginJavaScriptCollectionForEachLoop1182Result::MlSource1TemporaryBeginJavaScriptCollectionForEachLoop1182Return(Box::new(self::JavaScriptCollectionForEachTransition::CollectionForEachInvalid(Box::new(heap.clone()), invalid.clone())))
                    } else {
                        {
                            let source_ml_source1_temporary_index1181_2 =
                                source_ml_source1_temporary_index1181
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1181 =
                                    source_ml_source1_temporary_index1181_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryBeginJavaScriptCollectionForEachLoop1182Result::MlSource1TemporaryBeginJavaScriptCollectionForEachLoop1182Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189 {
    MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189Next,
    MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1189Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1190Result {
    MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1190Done,
    MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1190Return(self::ml_string::Text),
}
