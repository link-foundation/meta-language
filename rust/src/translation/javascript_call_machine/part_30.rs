// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesCall;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptConstructorRealm;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop214Result;
use super::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop215Result;
use super::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop216Result;
use super::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Result;
use super::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorJoin224;
use super::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop223Result;
use super::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop225Result;
use super::find_java_script_array_constructor_fact;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::order_java_script_collection_keys;
use super::read_java_script_ordinary_heap_error;
use super::source_ml_source1_temporary_read_java_script_array_species_directory_error_loop214;
use super::source_ml_source1_temporary_read_java_script_array_species_directory_error_loop215;
use super::source_ml_source1_temporary_read_java_script_array_species_directory_error_loop216;
use super::source_ml_source1_temporary_read_java_script_array_species_directory_error_loop217;
use super::source_ml_source1_temporary_read_java_script_array_species_state_error_loop223;
use super::source_ml_source1_temporary_read_java_script_array_species_state_error_loop225;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_constructor_realm_fallback(
    root: self::JavaScriptConstructorRealm,
    index: f64,
) -> Option<self::JavaScriptSemanticValue> {
    if (index == (0f64)) {
        match root.clone() {
            self::JavaScriptConstructorRealm::ConstructorRealm(
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value3935,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => ((source_ml_source2_data_property_value3935
                .clone()
                .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
            .0)
                .clone(),
        }
    } else {
        if (index == (1f64)) {
            match root.clone() {
                self::JavaScriptConstructorRealm::ConstructorRealm(
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value3936,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => ((source_ml_source2_data_property_value3936
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                .0)
                    .clone(),
            }
        } else {
            if (index == (2f64)) {
                match root.clone() {
                    self::JavaScriptConstructorRealm::ConstructorRealm(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value3937,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => ((source_ml_source2_data_property_value3937
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                    .0)
                        .clone(),
                }
            } else {
                if (index == (3f64)) {
                    match root.clone() {
                        self::JavaScriptConstructorRealm::ConstructorRealm(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value3938,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => ((source_ml_source2_data_property_value3938
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                        .0)
                            .clone(),
                    }
                } else {
                    if (index == (4f64)) {
                        match root.clone() {
                            self::JavaScriptConstructorRealm::ConstructorRealm(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value3939,
                                _,
                                _,
                                _,
                                _,
                            ) => ((source_ml_source2_data_property_value3939
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                            .0)
                                .clone(),
                        }
                    } else {
                        if (index == (5f64)) {
                            match root.clone() {
                                self::JavaScriptConstructorRealm::ConstructorRealm(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value3940,
                                    _,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value3940
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                                .0)
                                    .clone(),
                            }
                        } else {
                            if (index == (6f64)) {
                                match root.clone() {
                                    self::JavaScriptConstructorRealm::ConstructorRealm(
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
                                        source_ml_source2_data_property_value3941,
                                        _,
                                        _,
                                    ) => ((source_ml_source2_data_property_value3941
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptSemanticValue>,)
                                        }))
                                    .0)
                                        .clone(),
                                }
                            } else {
                                if (index == (7f64)) {
                                    match root.clone() {
                                        self::JavaScriptConstructorRealm::ConstructorRealm(
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
                                            _,
                                            source_ml_source2_data_property_value3942,
                                            _,
                                        ) => ((source_ml_source2_data_property_value3942
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<self::JavaScriptSemanticValue>,)
                                            }))
                                        .0)
                                            .clone(),
                                    }
                                } else {
                                    if (index == (8f64)) {
                                        match root.clone() {
                                            self::JavaScriptConstructorRealm::ConstructorRealm(
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
                                                _,
                                                _,
                                                source_ml_source2_data_property_value3943,
                                            ) => ((source_ml_source2_data_property_value3943
                                                .clone()
                                                .unwrap_or_else(|| {
                                                    (None::<self::JavaScriptSemanticValue>,)
                                                }))
                                            .0)
                                                .clone(),
                                        }
                                    } else {
                                        None::<self::JavaScriptSemanticValue>
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
pub fn read_java_script_array_species_directory_error(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
) -> self::ml_string::Text {
    match directory.clone() {
        self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            _,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            {
                let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                    heap_error.clone()
                } else {
                    if ((((({
                        let source_ml_source2_type_query_operand3947 =
                            shared_constructor_field1.clone();
                        self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                    }) != (self::ml_string::Text::from_units(vec![
                        98, 105, 103, 105, 110, 116,
                    ]))) || (shared_constructor_field1 <= (self::ml::Big::from_i128(0))))
                        || (shared_constructor_field1
                            > (self::ml::Big::from_i128(18446744073709551615))))
                        || (({
                            let ml_reference_array358 = shared_constructor_field5.clone();
                            let ml_reference_projection358 =
                                ml_reference_array358.borrow().len() as f64;
                            ml_reference_projection358
                        }) > (4096f64)))
                    {
                        self::ml_string::Text::from_units(vec![
                            115, 112, 101, 99, 105, 101, 115, 32, 97, 117, 116, 104, 111, 114, 105,
                            116, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32,
                            97, 99, 116, 117, 97, 108, 32, 117, 110, 115, 105, 103, 110, 101, 100,
                            54, 52, 32, 114, 101, 97, 108, 109, 32, 97, 110, 100, 32, 98, 111, 117,
                            110, 100, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111,
                            114, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121,
                        ])
                    } else {
                        {
                            let source_ml_source1_temporary_values209 =
                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    shared_constructor_field2.clone(),
                                    shared_constructor_field3.clone(),
                                    shared_constructor_field4.clone(),
                                ]));
                            {
                                let source_ml_source1_temporary_index209 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s3957 = self::source_ml_source1_temporary_read_java_script_array_species_directory_error_loop214(heap.clone(), source_ml_source1_temporary_values209.clone(), source_ml_source1_temporary_index209.clone());
                                    match source_ml_source2_s3957.clone() {
                                        self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop214Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop214Done => {
                                            if (((({
                                                let source_ml_source2_s3959 = shared_constructor_field2.clone();
                                                match source_ml_source2_s3959.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }
                                            } || !(((match shared_constructor_field2.clone() {
                                                self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value3964) => {
                                                    Some((source_ml_source2_data_property_value3964,))
                                                }
                                                _ => {
                                                    None::<(bool,)>
                                                }
                                            }).clone().expect("a proven present value")).0)) || {
                                                let source_ml_source2_s3966 = shared_constructor_field3.clone();
                                                match source_ml_source2_s3966.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }
                                            }) || ((match shared_constructor_field3.clone() {
                                                self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value3971) => {
                                                    Some((source_ml_source2_data_property_value3971,))
                                                }
                                                _ => {
                                                    None::<(bool,)>
                                                }
                                            }).clone().expect("a proven present value")).0) || {
                                                let source_ml_source2_s3973 = shared_constructor_field4.clone();
                                                match source_ml_source2_s3973.clone() {
                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }
                                            }) {
                                                self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114, 114, 97, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 47, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 97, 110, 100, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115])
                                            } else {
                                                {
                                                    let intrinsic_realms = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                    {
                                                        let prior = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source1_temporary_values210 = shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source1_temporary_index210 = self::ml::Big::from_i128(0);
                                                                {
                                                                    let source_ml_source2_s3977 = self::source_ml_source1_temporary_read_java_script_array_species_directory_error_loop215(heap.clone(), intrinsic_realms.clone(), prior.clone(), source_ml_source1_temporary_values210.clone(), source_ml_source1_temporary_index210.clone());
                                                                    match source_ml_source2_s3977.clone() {
                                                                        self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop215Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop215Done => {
                                                                            {
                                                                                let previous_intrinsic_realm = self::ml::Big::from_i128(0);
                                                                                {
                                                                                    let source_ml_source1_temporary_values211 = self::order_java_script_collection_keys(intrinsic_realms.clone());
                                                                                    {
                                                                                        let source_ml_source1_temporary_index211 = self::ml::Big::from_i128(0);
                                                                                        {
                                                                                            let source_ml_source2_s3978 = self::source_ml_source1_temporary_read_java_script_array_species_directory_error_loop216(previous_intrinsic_realm.clone(), source_ml_source1_temporary_values211.clone(), source_ml_source1_temporary_index211.clone());
                                                                                            match source_ml_source2_s3978.clone() {
                                                                                                self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop216Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop216Done => {
                                                                                                    {
                                                                                                        let previous_realm = self::ml::Big::from_i128(0);
                                                                                                        {
                                                                                                            let source_ml_source1_temporary_values212 = (match directory.clone() {
                                                                                                                self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(_, _, _, _, _, source_ml_source2_data_property_value3979) => {
                                                                                                                    ((source_ml_source2_data_property_value3979.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptConstructorRealm>>>>,))).0).clone()
                                                                                                                }
                                                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptConstructorRealm>::new())));
                                                                                                            {
                                                                                                                let source_ml_source1_temporary_index212 = self::ml::Big::from_i128(0);
                                                                                                                {
                                                                                                                    let source_ml_source2_s3980 = self::source_ml_source1_temporary_read_java_script_array_species_directory_error_loop217(heap.clone(), directory.clone(), previous_realm.clone(), source_ml_source1_temporary_values212.clone(), source_ml_source1_temporary_index212.clone());
                                                                                                                    match source_ml_source2_s3980.clone() {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Done => {
                                                                                                                            {
                                                                                                                                let root = {
                                                                                                                                    let source_ml_source2_array_read_value3985 = shared_constructor_field5.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_array_read_index3986 = self::find_java_script_array_constructor_fact(shared_constructor_field5.clone(), (((match shared_constructor_field2.clone() {
                                                                                                                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value3984, _) => {
                                                                                                                                                Some((source_ml_source2_data_property_value3984.clone(),))
                                                                                                                                            }
                                                                                                                                            _ => {
                                                                                                                                                None::<(self::ml::Big,)>
                                                                                                                                            }
                                                                                                                                        }).clone().expect("a proven present value")).0).clone());
                                                                                                                                        { let ml_reference_array359 = source_ml_source2_array_read_value3985.clone(); let ml_reference_position359 = source_ml_source2_array_read_index3986; let ml_reference_projection359 = self::ml_array::optional_at(&*ml_reference_array359.borrow(), self::ml_array::number_index(ml_reference_position359)); ml_reference_projection359 }
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                if (((match (root.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(match root.clone().expect("a proven present value") {
                                                                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value3987, _, _, _) => {
                                                                                                                                        source_ml_source2_data_property_value3987
                                                                                                                                    }
                                                                                                                                })) || !(match root.clone().expect("a proven present value") {
                                                                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, source_ml_source2_data_property_value3988, _) => {
                                                                                                                                        source_ml_source2_data_property_value3988
                                                                                                                                    }
                                                                                                                                })) || ((match root.clone().expect("a proven present value") {
                                                                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, source_ml_source2_data_property_value3991, _, _) => {
                                                                                                                                        source_ml_source2_data_property_value3991.clone()
                                                                                                                                    }
                                                                                                                                }) != shared_constructor_field1)) {
                                                                                                                                    self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 117, 114, 114, 101, 110, 116, 45, 114, 101, 97, 108, 109, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 65, 114, 114, 97, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 102, 97, 99, 116])
                                                                                                                                } else {
                                                                                                                                    self::ml_string::Text::from_units(vec![])
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                        self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop217Return(source_ml_source1_temporary_result217) => {
                                                                                                                            source_ml_source1_temporary_result217.clone()
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop216Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop216Return(source_ml_source1_temporary_result219) => {
                                                                                                    source_ml_source1_temporary_result219.clone()
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop215Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop215Return(source_ml_source1_temporary_result219_2) => {
                                                                            source_ml_source1_temporary_result219_2.clone()
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop214Result::MlSource1TemporaryReadJavaScriptArraySpeciesDirectoryErrorLoop214Return(source_ml_source1_temporary_result220) => {
                                            source_ml_source1_temporary_result220.clone()
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
pub fn read_java_script_array_species_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArraySpeciesState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptArraySpeciesState::ArraySpeciesState(
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
            _,
        ) => {
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field11 = *shared_constructor_field11;
            match directory.clone() {
                self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(
                    shared_constructor_field1,
                    shared_constructor_field2,
                    shared_constructor_field3,
                    shared_constructor_field4,
                    _,
                    _,
                ) => {
                    let shared_constructor_field2 = *shared_constructor_field2;
                    let shared_constructor_field3 = *shared_constructor_field3;
                    let shared_constructor_field4 = *shared_constructor_field4;
                    {
                        let error = self::read_java_script_array_species_directory_error(
                            heap.clone(),
                            directory.clone(),
                        );
                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                            error.clone()
                        } else {
                            if ((((shared_constructor_field7 != shared_constructor_field1)
                                || !(self::test_java_script_semantic_same_value(
                                    shared_constructor_field8.clone(),
                                    shared_constructor_field2.clone(),
                                )))
                                || !(self::test_java_script_semantic_same_value(
                                    shared_constructor_field9.clone(),
                                    shared_constructor_field3.clone(),
                                )))
                                || !(self::test_java_script_semantic_same_value(
                                    shared_constructor_field10.clone(),
                                    shared_constructor_field4.clone(),
                                )))
                            {
                                self::ml_string::Text::from_units(vec![
                                    115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 116, 105,
                                    110, 117, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111,
                                    116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 105, 116, 115,
                                    32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 44,
                                    32, 114, 111, 111, 116, 115, 32, 111, 114, 32, 83, 121, 109,
                                    98, 111, 108,
                                ])
                            } else {
                                if (((({
                                    let source_ml_source2_s4004 = shared_constructor_field5.clone();
                                    match source_ml_source2_s4004.clone() {
                                        self::JavaScriptSemanticValue::Object(_, _) => false,
                                        _ => true,
                                    }
                                } || !(self::ml_math::is_safe_integer(
                                    shared_constructor_field6,
                                ))) || (shared_constructor_field6 < (0f64)))
                                    || {
                                        let source_ml_source2_same_value_operand4012 =
                                            shared_constructor_field6;
                                        {
                                            let source_ml_source2_same_value_operand4013 = (-0f64);
                                            (((source_ml_source2_same_value_operand4012 == source_ml_source2_same_value_operand4013) && ((source_ml_source2_same_value_operand4012 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand4012)) == ((1f64 / source_ml_source2_same_value_operand4013))))) || ((source_ml_source2_same_value_operand4012 != source_ml_source2_same_value_operand4012) && (source_ml_source2_same_value_operand4013 != source_ml_source2_same_value_operand4013)))
                                        }
                                    })
                                    || !(self::ml_array::includes(
                                        &{
                                            let ml_reference_value360 =
                                                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                    self::ml_string::Text::from_units(vec![
                                                        98, 114, 97, 110, 100,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        99, 111, 110, 115, 116, 114, 117, 99, 116,
                                                        111, 114,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        99, 111, 110, 115, 116, 114, 117, 99, 116,
                                                        111, 114, 32, 99, 108, 97, 115, 115, 105,
                                                        102, 105, 99, 97, 116, 105, 111, 110,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        115, 112, 101, 99, 105, 101, 115,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        115, 112, 101, 99, 105, 101, 115, 32, 99,
                                                        108, 97, 115, 115, 105, 102, 105, 99, 97,
                                                        116, 105, 111, 110,
                                                    ]),
                                                ])))
                                                .borrow()
                                                .clone();
                                            ml_reference_value360
                                        },
                                        &shared_constructor_field12.clone(),
                                        0f64,
                                        false,
                                        |a, b| a == b,
                                    )))
                                {
                                    self::ml_string::Text::from_units(vec![
                                        115, 112, 101, 99, 105, 101, 115, 32, 115, 101, 108, 101,
                                        99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110,
                                        115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                                        79, 98, 106, 101, 99, 116, 44, 32, 110, 111, 110, 110, 101,
                                        103, 97, 116, 105, 118, 101, 32, 108, 101, 110, 103, 116,
                                        104, 32, 97, 110, 100, 32, 112, 104, 97, 115, 101,
                                    ])
                                } else {
                                    {
                                        let source_ml_source1_temporary_values221 =
                                            std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                shared_constructor_field5.clone(),
                                                shared_constructor_field11.clone(),
                                            ]));
                                        {
                                            let source_ml_source1_temporary_index221 =
                                                self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s4017 = self::source_ml_source1_temporary_read_java_script_array_species_state_error_loop223(heap.clone(), source_ml_source1_temporary_values221.clone(), source_ml_source1_temporary_index221.clone());
                                                match source_ml_source2_s4017.clone() {
                                                    self::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop223Result::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop223Done => {
                                                        {
                                                            let source_ml_source2_s4041 = if !(match (match state.clone() {
                                                                self::JavaScriptArraySpeciesState::ArraySpeciesState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value4042) => {
                                                                    ((source_ml_source2_data_property_value4042.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesCall>,))).0).clone()
                                                                }
                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                if (((!(self::ml_array::includes(&{ let ml_reference_value361 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115])]))).borrow().clone(); ml_reference_value361 }, &shared_constructor_field12.clone(), 0f64, false, |a, b| a == b)) || {
                                                                    let source_ml_source2_s4046 = match (match state.clone() {
                                                                        self::JavaScriptArraySpeciesState::ArraySpeciesState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value4049) => {
                                                                            ((source_ml_source2_data_property_value4049.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesCall>,))).0).clone()
                                                                        }
                                                                    }).clone().expect("a proven present value") {
                                                                        self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(source_ml_source2_data_property_value4050, _, _) => {
                                                                            let source_ml_source2_data_property_value4050 = *source_ml_source2_data_property_value4050;
                                                                            source_ml_source2_data_property_value4050.clone()
                                                                        }
                                                                    };
                                                                    match source_ml_source2_s4046.clone() {
                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    }
                                                                }) || !(((match match (match state.clone() {
                                                                    self::JavaScriptArraySpeciesState::ArraySpeciesState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value4051) => {
                                                                        ((source_ml_source2_data_property_value4051.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesCall>,))).0).clone()
                                                                    }
                                                                }).clone().expect("a proven present value") {
                                                                    self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(source_ml_source2_data_property_value4052, _, _) => {
                                                                        let source_ml_source2_data_property_value4052 = *source_ml_source2_data_property_value4052;
                                                                        source_ml_source2_data_property_value4052.clone()
                                                                    }
                                                                } {
                                                                    self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value4053) => {
                                                                        Some((source_ml_source2_data_property_value4053,))
                                                                    }
                                                                    _ => {
                                                                        None::<(bool,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0)) || (({ let ml_reference_array362 = match (match state.clone() {
                                                                    self::JavaScriptArraySpeciesState::ArraySpeciesState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value4056) => {
                                                                        ((source_ml_source2_data_property_value4056.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesCall>,))).0).clone()
                                                                    }
                                                                }).clone().expect("a proven present value") {
                                                                    self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(_, _, source_ml_source2_data_property_value4057) => {
                                                                        source_ml_source2_data_property_value4057.clone()
                                                                    }
                                                                }; let ml_reference_projection362 = ml_reference_array362.borrow().len() as f64; ml_reference_projection362 }) > (3f64))) {
                                                                    self::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorJoin224::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorJoin224Return(self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 115, 101, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 103, 101, 116, 116, 101, 114, 32, 99, 97, 108, 108]))
                                                                } else {
                                                                    {
                                                                        let source_ml_source1_temporary_values222 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![match (match state.clone() {
                                                                            self::JavaScriptArraySpeciesState::ArraySpeciesState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value4058) => {
                                                                                ((source_ml_source2_data_property_value4058.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesCall>,))).0).clone()
                                                                            }
                                                                        }).clone().expect("a proven present value") {
                                                                            self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(source_ml_source2_data_property_value4059, _, _) => {
                                                                                let source_ml_source2_data_property_value4059 = *source_ml_source2_data_property_value4059;
                                                                                source_ml_source2_data_property_value4059.clone()
                                                                            }
                                                                        }, match (match state.clone() {
                                                                            self::JavaScriptArraySpeciesState::ArraySpeciesState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value4060) => {
                                                                                ((source_ml_source2_data_property_value4060.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesCall>,))).0).clone()
                                                                            }
                                                                        }).clone().expect("a proven present value") {
                                                                            self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(_, source_ml_source2_data_property_value4061, _) => {
                                                                                let source_ml_source2_data_property_value4061 = *source_ml_source2_data_property_value4061;
                                                                                source_ml_source2_data_property_value4061.clone()
                                                                            }
                                                                        }], { let ml_reference_value363 = (match (match state.clone() {
                                                                            self::JavaScriptArraySpeciesState::ArraySpeciesState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value4062) => {
                                                                                ((source_ml_source2_data_property_value4062.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesCall>,))).0).clone()
                                                                            }
                                                                        }).clone().expect("a proven present value") {
                                                                            self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(_, _, source_ml_source2_data_property_value4063) => {
                                                                                source_ml_source2_data_property_value4063.clone()
                                                                            }
                                                                        }).borrow().clone(); ml_reference_value363 })));
                                                                        {
                                                                            let source_ml_source1_temporary_index222 = self::ml::Big::from_i128(0);
                                                                            {
                                                                                let source_ml_source2_s4064 = self::source_ml_source1_temporary_read_java_script_array_species_state_error_loop225(heap.clone(), source_ml_source1_temporary_values222.clone(), source_ml_source1_temporary_index222.clone());
                                                                                match source_ml_source2_s4064.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop225Result::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop225Done => {
                                                                                        self::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorJoin224::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorJoin224Next
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop225Result::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop225Return(source_ml_source1_temporary_result225) => {
                                                                                        self::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorJoin224::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorJoin224Return(source_ml_source1_temporary_result225.clone())
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                self::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorJoin224::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorJoin224Next
                                                            };
                                                            match source_ml_source2_s4041.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorJoin224::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorJoin224Next => {
                                                                    if ((shared_constructor_field12 == (self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]))) && {
                                                                        let source_ml_source2_s4068 = shared_constructor_field11.clone();
                                                                        match source_ml_source2_s4068.clone() {
                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                false
                                                                            }
                                                                            _ => {
                                                                                true
                                                                            }
                                                                        }
                                                                    }) {
                                                                        self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 71, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![])
                                                                    }
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorJoin224::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorJoin224Return(source_ml_source1_temporary_result224) => {
                                                                    source_ml_source1_temporary_result224.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop223Result::MlSource1TemporaryReadJavaScriptArraySpeciesStateErrorLoop223Return(source_ml_source1_temporary_result225_2) => {
                                                        source_ml_source1_temporary_result225_2.clone()
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
