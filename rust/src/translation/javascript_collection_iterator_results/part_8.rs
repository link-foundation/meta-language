// Generated from complete js/src/translation/javascript-collection-iterator-results.js. Do not edit by hand.
use super::JavaScriptArrayPropertyPlan;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop27Result;
use super::find_java_script_array_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_array_ordinary_definition;
use super::read_java_script_array_length;
use super::read_java_script_array_length_key;
use super::source_ml_temporary_plan_java_script_array_length_definition_loop26;
use super::source_ml_temporary_plan_java_script_array_length_definition_loop27;
use super::test_java_script_array_length;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_java_script_array_length_definition(
    object: self::JavaScriptOrdinaryObject,
    proposed: std::rc::Rc<
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
    unsigned_length: f64,
    number_length: f64,
) -> self::JavaScriptArrayPropertyPlan {
    if (self::test_java_script_array_length(unsigned_length)) {
        {
            let ml_library_inequality_1 = unsigned_length != number_length;
            if ml_library_inequality_1 {
                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                    Box::new(object.clone()),
                    false,
                    self::ml_string::Text::from_units(vec![
                        73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32, 108, 101,
                        110, 103, 116, 104,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        82, 97, 110, 103, 101, 69, 114, 114, 111, 114,
                    ]),
                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())),
                )
            } else {
                {
                    let key = self::read_java_script_array_length_key();
                    {
                        let at = self::find_java_script_array_property(
                            match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value953,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value953.clone(),
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value954,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value954.clone(),
                                self::JavaScriptOrdinaryObject::ArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value955,
                                ) => source_ml_source1_data_property_value955.clone(),
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value956,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value956.clone(),
                                self::JavaScriptOrdinaryObject::MapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value957,
                                    _,
                                ) => source_ml_source1_data_property_value957.clone(),
                                self::JavaScriptOrdinaryObject::SetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value958,
                                    _,
                                ) => source_ml_source1_data_property_value958.clone(),
                                self::JavaScriptOrdinaryObject::WeakMapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value959,
                                    _,
                                ) => source_ml_source1_data_property_value959.clone(),
                                self::JavaScriptOrdinaryObject::WeakSetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value960,
                                    _,
                                ) => source_ml_source1_data_property_value960.clone(),
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value961,
                                ) => source_ml_source1_data_property_value961.clone(),
                                self::JavaScriptOrdinaryObject::HostErrorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value962,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value962.clone(),
                                self::JavaScriptOrdinaryObject::HostURLObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value963,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value963.clone(),
                                self::JavaScriptOrdinaryObject::HostArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value964,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value964.clone(),
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value965,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value965.clone(),
                            },
                            key.clone(),
                        );
                        {
                            let old = {
                                let source_ml_source1_array_read_value979 = match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value966,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value966.clone(),
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value967,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value967.clone(),
                                    self::JavaScriptOrdinaryObject::ArrayObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value968,
                                    ) => source_ml_source1_data_property_value968.clone(),
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value969,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value969.clone(),
                                    self::JavaScriptOrdinaryObject::MapObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value970,
                                        _,
                                    ) => source_ml_source1_data_property_value970.clone(),
                                    self::JavaScriptOrdinaryObject::SetObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value971,
                                        _,
                                    ) => source_ml_source1_data_property_value971.clone(),
                                    self::JavaScriptOrdinaryObject::WeakMapObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value972,
                                        _,
                                    ) => source_ml_source1_data_property_value972.clone(),
                                    self::JavaScriptOrdinaryObject::WeakSetObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value973,
                                        _,
                                    ) => source_ml_source1_data_property_value973.clone(),
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value974,
                                    ) => source_ml_source1_data_property_value974.clone(),
                                    self::JavaScriptOrdinaryObject::HostErrorObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value975,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value975.clone(),
                                    self::JavaScriptOrdinaryObject::HostURLObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value976,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value976.clone(),
                                    self::JavaScriptOrdinaryObject::HostArrayObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value977,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value977.clone(),
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value978,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value978.clone(),
                                };
                                {
                                    let ml_reference_array121 =
                                        source_ml_source1_array_read_value979;
                                    let ml_reference_position121 = at;
                                    self::ml_array::optional_at(
                                        &*ml_reference_array121.borrow(),
                                        self::ml_array::number_index(ml_reference_position121),
                                    )
                                }
                            };
                            if match (old.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                                    Box::new(object.clone()),
                                    false,
                                    self::ml_string::Text::from_units(vec![
                                        116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114,
                                        114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 100,
                                        101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 109, 117,
                                        115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 112, 114,
                                        101, 115, 101, 110, 116,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        105, 110, 118, 97, 108, 105, 100,
                                    ]),
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                        self::ml_string::Text,
                                    >::new(
                                    ))),
                                )
                            } else {
                                {
                                    let length =
                                        self::read_java_script_array_length(object.clone());
                                    {
                                        let descriptor = std::rc::Rc::new(std::cell::RefCell::new(
                                            {
                                                let source_ml_source1_spread981 =
                                                    { proposed.borrow().clone() };
                                                {
                                                    let source_ml_source1_spread_field982 =
                                                        (source_ml_source1_spread981.clone()).0;
                                                    {
                                                        let source_ml_source1_spread_field983 =
                                                            (source_ml_source1_spread981.clone()).1;
                                                        {
                                                            let source_ml_source1_spread_field984 =
                                                                (source_ml_source1_spread981
                                                                    .clone())
                                                                .2;
                                                            {
                                                                let source_ml_source1_spread_field985 =
                                                                    (source_ml_source1_spread981
                                                                        .clone())
                                                                    .3;
                                                                {
                                                                    let source_ml_source1_spread_field986 = (source_ml_source1_spread981.clone()).4;
                                                                    {
                                                                        let source_ml_source1_spread_field987 = (source_ml_source1_spread981.clone()).5;
                                                                        {
                                                                            let source_ml_source1_spread_field988 = (source_ml_source1_spread981.clone()).6;
                                                                            {
                                                                                let source_ml_source1_spread_field989 = (source_ml_source1_spread981.clone()).7;
                                                                                {
                                                                                    let source_ml_source1_spread_field990 = (source_ml_source1_spread981.clone()).8;
                                                                                    {
                                                                                        let source_ml_source1_spread_field991 = (source_ml_source1_spread981.clone()).9;
                                                                                        {
                                                                                            let source_ml_source1_spread_field992 = (source_ml_source1_spread981.clone()).10;
                                                                                            {
                                                                                                let source_ml_source1_spread_field993 = (source_ml_source1_spread981.clone()).11;
                                                                                                {
                                                                                                    let source_ml_source1_field995 = self::JavaScriptSemanticValue::Number(unsigned_length);
                                                                                                    (source_ml_source1_spread_field982, source_ml_source1_spread_field983, source_ml_source1_spread_field984, source_ml_source1_spread_field985, source_ml_source1_spread_field986, source_ml_source1_spread_field987, source_ml_source1_spread_field988, source_ml_source1_spread_field989, source_ml_source1_field995, true, source_ml_source1_spread_field992, source_ml_source1_spread_field993)
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
                                            },
                                        ));
                                        if unsigned_length >= length {
                                        self::plan_java_script_array_ordinary_definition(object.clone(), key.clone(), descriptor.clone())
                                    } else if (({ (match old.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value996) => {
                                                source_ml_source1_data_property_value996.clone()
                                            }
                                        }).borrow().clone() }).10) {
                                            {
                                                let new_writable = !(({ descriptor.borrow().clone() }).11) || ({ descriptor.borrow().clone() }).10;
                                                {
                                                    let temporary = if new_writable {
                                                        descriptor.clone()
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_spread997 = { descriptor.borrow().clone() };
                                                            {
                                                                let source_ml_source1_spread_field998 = (source_ml_source1_spread997.clone()).0;
                                                                {
                                                                    let source_ml_source1_spread_field999 = (source_ml_source1_spread997.clone()).1;
                                                                    {
                                                                        let source_ml_source1_spread_field1000 = (source_ml_source1_spread997.clone()).2;
                                                                        {
                                                                            let source_ml_source1_spread_field1001 = (source_ml_source1_spread997.clone()).3;
                                                                            {
                                                                                let source_ml_source1_spread_field1002 = (source_ml_source1_spread997.clone()).4;
                                                                                {
                                                                                    let source_ml_source1_spread_field1003 = (source_ml_source1_spread997.clone()).5;
                                                                                    {
                                                                                        let source_ml_source1_spread_field1004 = (source_ml_source1_spread997.clone()).6;
                                                                                        {
                                                                                            let source_ml_source1_spread_field1005 = (source_ml_source1_spread997.clone()).7;
                                                                                            {
                                                                                                let source_ml_source1_spread_field1006 = (source_ml_source1_spread997.clone()).8;
                                                                                                {
                                                                                                    let source_ml_source1_spread_field1007 = (source_ml_source1_spread997.clone()).9;
                                                                                                    {
                                                                                                        let source_ml_source1_spread_field1008 = (source_ml_source1_spread997.clone()).10;
                                                                                                        {
                                                                                                            let source_ml_source1_spread_field1009 = (source_ml_source1_spread997.clone()).11;
                                                                                                            (source_ml_source1_spread_field998, source_ml_source1_spread_field999, source_ml_source1_spread_field1000, source_ml_source1_spread_field1001, source_ml_source1_spread_field1002, source_ml_source1_spread_field1003, source_ml_source1_spread_field1004, source_ml_source1_spread_field1005, source_ml_source1_spread_field1006, source_ml_source1_spread_field1007, true, source_ml_source1_spread_field1009)
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
                                                    };
                                                    {
                                                        let initial = self::plan_java_script_array_ordinary_definition(object.clone(), key.clone(), temporary);
                                                        if (match initial.clone() {
                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, source_ml_source1_data_property_value1011, _, _, _) => {
                                                                source_ml_source1_data_property_value1011
                                                            }
                                                        }) {
                                                            {
                                                                let blocked = -1f64;
                                                                {
                                                                    let source_ml_temporary_values24 = match object.clone() {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value1012, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value1012.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value1013, _, _, _) => {
                                                                            source_ml_source1_data_property_value1013.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value1014) => {
                                                                            source_ml_source1_data_property_value1014.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value1015, _, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value1015.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value1016, _) => {
                                                                            source_ml_source1_data_property_value1016.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value1017, _) => {
                                                                            source_ml_source1_data_property_value1017.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value1018, _) => {
                                                                            source_ml_source1_data_property_value1018.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value1019, _) => {
                                                                            source_ml_source1_data_property_value1019.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value1020) => {
                                                                            source_ml_source1_data_property_value1020.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value1021, _, _) => {
                                                                            source_ml_source1_data_property_value1021.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value1022, _, _) => {
                                                                            source_ml_source1_data_property_value1022.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value1023, _, _) => {
                                                                            source_ml_source1_data_property_value1023.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value1024, _, _) => {
                                                                            source_ml_source1_data_property_value1024.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_temporary_index24 = self::ml::Big::from_i128(0);
                                                                        {
                                                                            let blocked_2 = self::source_ml_temporary_plan_java_script_array_length_definition_loop26(unsigned_length, blocked, source_ml_temporary_values24, source_ml_temporary_index24);
                                                                            {
                                                                                let final_length = if blocked_2 < 0f64 {
                                                                                    unsigned_length
                                                                                } else {
                                                                                    blocked_2 + 1f64
                                                                                };
                                                                                {
                                                                                    let properties = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                                                                    {
                                                                                        let source_ml_temporary_values25 = match match initial.clone() {
                                                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value1025, _, _, _, _) => {
                                                                                                let source_ml_source1_data_property_value1025 = *source_ml_source1_data_property_value1025;
                                                                                                source_ml_source1_data_property_value1025.clone()
                                                                                            }
                                                                                        } {
                                                                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value1026, _, _, _, _, _) => {
                                                                                                source_ml_source1_data_property_value1026.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value1027, _, _, _) => {
                                                                                                source_ml_source1_data_property_value1027.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value1028) => {
                                                                                                source_ml_source1_data_property_value1028.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value1029, _, _, _, _, _, _) => {
                                                                                                source_ml_source1_data_property_value1029.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value1030, _) => {
                                                                                                source_ml_source1_data_property_value1030.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value1031, _) => {
                                                                                                source_ml_source1_data_property_value1031.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value1032, _) => {
                                                                                                source_ml_source1_data_property_value1032.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value1033, _) => {
                                                                                                source_ml_source1_data_property_value1033.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value1034) => {
                                                                                                source_ml_source1_data_property_value1034.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value1035, _, _) => {
                                                                                                source_ml_source1_data_property_value1035.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value1036, _, _) => {
                                                                                                source_ml_source1_data_property_value1036.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value1037, _, _) => {
                                                                                                source_ml_source1_data_property_value1037.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value1038, _, _) => {
                                                                                                source_ml_source1_data_property_value1038.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_temporary_index25 = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let source_ml_source1_s1039 = self::source_ml_temporary_plan_java_script_array_length_definition_loop27(new_writable, final_length, properties.clone(), source_ml_temporary_values25, source_ml_temporary_index25);
                                                                                                match source_ml_source1_s1039 {
                                                                                                    self::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop27Result::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop27Done => {
                                                                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(match match initial.clone() {
                                                                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value1040, _, _, _, _) => {
                                                                                                                let source_ml_source1_data_property_value1040 = *source_ml_source1_data_property_value1040;
                                                                                                                source_ml_source1_data_property_value1040.clone()
                                                                                                            }
                                                                                                        } {
                                                                                                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field1041, source_ml_source1_nominal_field1042, source_ml_source1_nominal_field1043, source_ml_source1_nominal_field1044, _, source_ml_source1_nominal_field1046, source_ml_source1_nominal_field1047, source_ml_source1_nominal_field1048, source_ml_source1_nominal_field1049, source_ml_source1_nominal_field1050) => {
                                                                                                                let source_ml_source1_nominal_field1042 = *source_ml_source1_nominal_field1042;
                                                                                                                let source_ml_source1_nominal_field1049 = *source_ml_source1_nominal_field1049;
                                                                                                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field1041.clone(), Box::new(source_ml_source1_nominal_field1042.clone()), source_ml_source1_nominal_field1043, source_ml_source1_nominal_field1044, properties.clone(), source_ml_source1_nominal_field1046.clone(), source_ml_source1_nominal_field1047, source_ml_source1_nominal_field1048.clone(), Box::new(source_ml_source1_nominal_field1049.clone()), source_ml_source1_nominal_field1050.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field1052, source_ml_source1_nominal_field1053, source_ml_source1_nominal_field1054, source_ml_source1_nominal_field1055, _, source_ml_source1_nominal_field1057, source_ml_source1_nominal_field1058, source_ml_source1_nominal_field1059) => {
                                                                                                                let source_ml_source1_nominal_field1053 = *source_ml_source1_nominal_field1053;
                                                                                                                let source_ml_source1_nominal_field1057 = *source_ml_source1_nominal_field1057;
                                                                                                                let source_ml_source1_nominal_field1058 = *source_ml_source1_nominal_field1058;
                                                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field1052.clone(), Box::new(source_ml_source1_nominal_field1053.clone()), source_ml_source1_nominal_field1054, source_ml_source1_nominal_field1055, properties.clone(), Box::new(source_ml_source1_nominal_field1057.clone()), Box::new(source_ml_source1_nominal_field1058.clone()), source_ml_source1_nominal_field1059.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field1061, source_ml_source1_nominal_field1062, source_ml_source1_nominal_field1063, source_ml_source1_nominal_field1064, _) => {
                                                                                                                let source_ml_source1_nominal_field1062 = *source_ml_source1_nominal_field1062;
                                                                                                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field1061.clone(), Box::new(source_ml_source1_nominal_field1062.clone()), source_ml_source1_nominal_field1063, source_ml_source1_nominal_field1064, properties.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field1067, source_ml_source1_nominal_field1068, source_ml_source1_nominal_field1069, source_ml_source1_nominal_field1070, _, source_ml_source1_nominal_field1072, source_ml_source1_nominal_field1073, source_ml_source1_nominal_field1074, source_ml_source1_nominal_field1075, source_ml_source1_nominal_field1076, source_ml_source1_nominal_field1077) => {
                                                                                                                let source_ml_source1_nominal_field1068 = *source_ml_source1_nominal_field1068;
                                                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field1067.clone(), Box::new(source_ml_source1_nominal_field1068.clone()), source_ml_source1_nominal_field1069, source_ml_source1_nominal_field1070, properties.clone(), source_ml_source1_nominal_field1072.clone(), source_ml_source1_nominal_field1073, source_ml_source1_nominal_field1074.clone(), source_ml_source1_nominal_field1075.clone(), source_ml_source1_nominal_field1076.clone(), source_ml_source1_nominal_field1077.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field1079, source_ml_source1_nominal_field1080, source_ml_source1_nominal_field1081, source_ml_source1_nominal_field1082, _, source_ml_source1_nominal_field1084) => {
                                                                                                                let source_ml_source1_nominal_field1080 = *source_ml_source1_nominal_field1080;
                                                                                                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field1079.clone(), Box::new(source_ml_source1_nominal_field1080.clone()), source_ml_source1_nominal_field1081, source_ml_source1_nominal_field1082, properties.clone(), source_ml_source1_nominal_field1084.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field1086, source_ml_source1_nominal_field1087, source_ml_source1_nominal_field1088, source_ml_source1_nominal_field1089, _, source_ml_source1_nominal_field1091) => {
                                                                                                                let source_ml_source1_nominal_field1087 = *source_ml_source1_nominal_field1087;
                                                                                                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field1086.clone(), Box::new(source_ml_source1_nominal_field1087.clone()), source_ml_source1_nominal_field1088, source_ml_source1_nominal_field1089, properties.clone(), source_ml_source1_nominal_field1091.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field1093, source_ml_source1_nominal_field1094, source_ml_source1_nominal_field1095, source_ml_source1_nominal_field1096, _, source_ml_source1_nominal_field1098) => {
                                                                                                                let source_ml_source1_nominal_field1094 = *source_ml_source1_nominal_field1094;
                                                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field1093.clone(), Box::new(source_ml_source1_nominal_field1094.clone()), source_ml_source1_nominal_field1095, source_ml_source1_nominal_field1096, properties.clone(), source_ml_source1_nominal_field1098.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field1100, source_ml_source1_nominal_field1101, source_ml_source1_nominal_field1102, source_ml_source1_nominal_field1103, _, source_ml_source1_nominal_field1105) => {
                                                                                                                let source_ml_source1_nominal_field1101 = *source_ml_source1_nominal_field1101;
                                                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field1100.clone(), Box::new(source_ml_source1_nominal_field1101.clone()), source_ml_source1_nominal_field1102, source_ml_source1_nominal_field1103, properties.clone(), source_ml_source1_nominal_field1105.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field1107, source_ml_source1_nominal_field1108, source_ml_source1_nominal_field1109, source_ml_source1_nominal_field1110, source_ml_source1_nominal_field1111, source_ml_source1_nominal_field1112, source_ml_source1_nominal_field1113, source_ml_source1_nominal_field1114, _) => {
                                                                                                                let source_ml_source1_nominal_field1108 = *source_ml_source1_nominal_field1108;
                                                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field1107.clone(), Box::new(source_ml_source1_nominal_field1108.clone()), source_ml_source1_nominal_field1109, source_ml_source1_nominal_field1110.clone(), source_ml_source1_nominal_field1111.clone(), source_ml_source1_nominal_field1112.clone(), source_ml_source1_nominal_field1113.clone(), source_ml_source1_nominal_field1114, properties.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field1117, source_ml_source1_nominal_field1118, source_ml_source1_nominal_field1119, source_ml_source1_nominal_field1120, _, source_ml_source1_nominal_field1122, source_ml_source1_nominal_field1123) => {
                                                                                                                let source_ml_source1_nominal_field1118 = *source_ml_source1_nominal_field1118;
                                                                                                                let source_ml_source1_nominal_field1122 = *source_ml_source1_nominal_field1122;
                                                                                                                let source_ml_source1_nominal_field1123 = *source_ml_source1_nominal_field1123;
                                                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field1117.clone(), Box::new(source_ml_source1_nominal_field1118.clone()), source_ml_source1_nominal_field1119, source_ml_source1_nominal_field1120, properties.clone(), Box::new(source_ml_source1_nominal_field1122.clone()), Box::new(source_ml_source1_nominal_field1123.clone()))
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field1125, source_ml_source1_nominal_field1126, source_ml_source1_nominal_field1127, source_ml_source1_nominal_field1128, _, source_ml_source1_nominal_field1130, source_ml_source1_nominal_field1131) => {
                                                                                                                let source_ml_source1_nominal_field1126 = *source_ml_source1_nominal_field1126;
                                                                                                                let source_ml_source1_nominal_field1130 = *source_ml_source1_nominal_field1130;
                                                                                                                let source_ml_source1_nominal_field1131 = *source_ml_source1_nominal_field1131;
                                                                                                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field1125.clone(), Box::new(source_ml_source1_nominal_field1126.clone()), source_ml_source1_nominal_field1127, source_ml_source1_nominal_field1128, properties.clone(), Box::new(source_ml_source1_nominal_field1130.clone()), Box::new(source_ml_source1_nominal_field1131.clone()))
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field1133, source_ml_source1_nominal_field1134, source_ml_source1_nominal_field1135, source_ml_source1_nominal_field1136, _, source_ml_source1_nominal_field1138, source_ml_source1_nominal_field1139) => {
                                                                                                                let source_ml_source1_nominal_field1134 = *source_ml_source1_nominal_field1134;
                                                                                                                let source_ml_source1_nominal_field1138 = *source_ml_source1_nominal_field1138;
                                                                                                                let source_ml_source1_nominal_field1139 = *source_ml_source1_nominal_field1139;
                                                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field1133.clone(), Box::new(source_ml_source1_nominal_field1134.clone()), source_ml_source1_nominal_field1135, source_ml_source1_nominal_field1136, properties.clone(), Box::new(source_ml_source1_nominal_field1138.clone()), Box::new(source_ml_source1_nominal_field1139.clone()))
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field1141, source_ml_source1_nominal_field1142, source_ml_source1_nominal_field1143, source_ml_source1_nominal_field1144, _, source_ml_source1_nominal_field1146, source_ml_source1_nominal_field1147) => {
                                                                                                                let source_ml_source1_nominal_field1142 = *source_ml_source1_nominal_field1142;
                                                                                                                let source_ml_source1_nominal_field1146 = *source_ml_source1_nominal_field1146;
                                                                                                                let source_ml_source1_nominal_field1147 = *source_ml_source1_nominal_field1147;
                                                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field1141.clone(), Box::new(source_ml_source1_nominal_field1142.clone()), source_ml_source1_nominal_field1143, source_ml_source1_nominal_field1144, properties.clone(), Box::new(source_ml_source1_nominal_field1146.clone()), Box::new(source_ml_source1_nominal_field1147.clone()))
                                                                                                            }
                                                                                                        }), (blocked_2 < 0f64), self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                            let source_ml_source1_concat_value1152 = match initial.clone() {
                                                                                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, _, source_ml_source1_data_property_value1149) => {
                                                                                                                    source_ml_source1_data_property_value1149.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_concat_value1153 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 105, 110, 100, 105, 99, 101, 115]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104])]));
                                                                                                                {
                                                                                                                    let source_ml_source1_concat_value1154 = if !new_writable && !(self::ml_array::includes(&{ (match initial.clone() {
                                                                                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, _, source_ml_source1_data_property_value1151) => {
                                                                                                                            source_ml_source1_data_property_value1151.clone()
                                                                                                                        }
                                                                                                                    }).borrow().clone() }, &self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101]), 0f64, false, |a, b| a == b)) {
                                                                                                                        std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])]))
                                                                                                                    } else {
                                                                                                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))
                                                                                                                    };
                                                                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { source_ml_source1_concat_value1152.borrow().clone() }), { source_ml_source1_concat_value1153.borrow().clone() }), { source_ml_source1_concat_value1154.borrow().clone() })
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
                                                        } else {
                                                            initial.clone()
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
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
        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
            Box::new(object.clone()),
            false,
            self::ml_string::Text::from_units(vec![
                97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 116,
                105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115,
                32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 115, 105, 103, 110,
                101, 100, 51, 50, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
            ]),
            self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]),
            std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())),
        )
    }
}
