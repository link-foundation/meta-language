// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayPropertyPlan;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryPlanJavaScriptArrayLengthDefinitionLoop27Result;
use super::find_java_script_array_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_array_ordinary_definition;
use super::read_java_script_array_length;
use super::read_java_script_array_length_key;
use super::source_ml_source1_temporary_plan_java_script_array_length_definition_loop26;
use super::source_ml_source1_temporary_plan_java_script_array_length_definition_loop27;
use super::source_ml_source3_record_order;
use super::test_java_script_array_length;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_array_length_definition(
    object: self::JavaScriptOrdinaryObject,
    proposed: std::rc::Rc<
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
    unsigned_length: f64,
    number_length: f64,
) -> self::JavaScriptArrayPropertyPlan {
    if !(self::test_java_script_array_length(unsigned_length)) {
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
    } else {
        if (unsigned_length != number_length) {
            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                Box::new(object.clone()),
                false,
                self::ml_string::Text::from_units(vec![
                    73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32, 108, 101, 110,
                    103, 116, 104,
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
                                source_ml_source2_data_property_value416,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value416.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value417,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value417.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value418,
                            ) => source_ml_source2_data_property_value418.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value419,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value419.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value420,
                                _,
                            ) => source_ml_source2_data_property_value420.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value421,
                                _,
                            ) => source_ml_source2_data_property_value421.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value422,
                                _,
                            ) => source_ml_source2_data_property_value422.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value423,
                                _,
                            ) => source_ml_source2_data_property_value423.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value424,
                            ) => source_ml_source2_data_property_value424.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value425,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value425.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value426,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value426.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value427,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value427.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value428,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value428.clone(),
                        },
                        key.clone(),
                    );
                    {
                        let old = {
                            let source_ml_source2_array_read_value442 = match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value429,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value429.clone(),
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value430,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value430.clone(),
                                self::JavaScriptOrdinaryObject::ArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value431,
                                ) => source_ml_source2_data_property_value431.clone(),
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value432,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value432.clone(),
                                self::JavaScriptOrdinaryObject::MapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value433,
                                    _,
                                ) => source_ml_source2_data_property_value433.clone(),
                                self::JavaScriptOrdinaryObject::SetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value434,
                                    _,
                                ) => source_ml_source2_data_property_value434.clone(),
                                self::JavaScriptOrdinaryObject::WeakMapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value435,
                                    _,
                                ) => source_ml_source2_data_property_value435.clone(),
                                self::JavaScriptOrdinaryObject::WeakSetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value436,
                                    _,
                                ) => source_ml_source2_data_property_value436.clone(),
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value437,
                                ) => source_ml_source2_data_property_value437.clone(),
                                self::JavaScriptOrdinaryObject::HostErrorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value438,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value438.clone(),
                                self::JavaScriptOrdinaryObject::HostURLObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value439,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value439.clone(),
                                self::JavaScriptOrdinaryObject::HostArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value440,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value440.clone(),
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value441,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value441.clone(),
                            };
                            {
                                let ml_reference_array153 =
                                    source_ml_source2_array_read_value442.clone();
                                let ml_reference_position153 = at;
                                let ml_reference_projection153 = self::ml_array::optional_at(
                                    &*ml_reference_array153.borrow(),
                                    self::ml_array::number_index(ml_reference_position153),
                                );
                                ml_reference_projection153
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
                                    116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114, 114,
                                    97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 100, 101, 115,
                                    99, 114, 105, 112, 116, 111, 114, 32, 109, 117, 115, 116, 32,
                                    114, 101, 109, 97, 105, 110, 32, 112, 114, 101, 115, 101, 110,
                                    116,
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
                                let length = self::read_java_script_array_length(object.clone());
                                {
                                    let descriptor = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_spread444 = {
                                            let ml_reference_value154 = proposed.borrow().clone();
                                            ml_reference_value154
                                        };
                                        {
                                            let source_ml_source2_spread_field445 =
                                                ((source_ml_source2_spread444.clone()).0).0;
                                            {
                                                let source_ml_source2_spread_field446 =
                                                    ((source_ml_source2_spread444.clone()).0).1;
                                                {
                                                    let source_ml_source2_spread_field447 =
                                                        ((source_ml_source2_spread444.clone()).0).2;
                                                    {
                                                        let source_ml_source2_spread_field448 =
                                                            ((source_ml_source2_spread444.clone())
                                                                .0)
                                                                .3;
                                                        {
                                                            let source_ml_source2_spread_field449 =
                                                                ((source_ml_source2_spread444
                                                                    .clone())
                                                                .0)
                                                                    .4;
                                                            {
                                                                let source_ml_source2_spread_field450 =
                                                                    ((source_ml_source2_spread444
                                                                        .clone())
                                                                    .0)
                                                                        .5;
                                                                {
                                                                    let source_ml_source2_spread_field451 = ((source_ml_source2_spread444.clone()).0).6;
                                                                    {
                                                                        let source_ml_source2_spread_field452 = ((source_ml_source2_spread444.clone()).0).7;
                                                                        {
                                                                            let source_ml_source2_spread_field453 = ((source_ml_source2_spread444.clone()).0).8;
                                                                            {
                                                                                let source_ml_source2_spread_field454 = ((source_ml_source2_spread444.clone()).0).9;
                                                                                {
                                                                                    let source_ml_source2_spread_field455 = ((source_ml_source2_spread444.clone()).0).10;
                                                                                    {
                                                                                        let source_ml_source2_spread_field456 = ((source_ml_source2_spread444.clone()).0).11;
                                                                                        {
                                                                                            let source_ml_source2_field458 = self::JavaScriptSemanticValue::Number(unsigned_length);
                                                                                            {
                                                                                                let source_ml_source3_record_field29 = source_ml_source2_spread_field445;
                                                                                                {
                                                                                                    let source_ml_source3_record_field30 = source_ml_source2_spread_field446;
                                                                                                    {
                                                                                                        let source_ml_source3_record_field31 = source_ml_source2_spread_field447;
                                                                                                        {
                                                                                                            let source_ml_source3_record_field32 = source_ml_source2_spread_field448;
                                                                                                            {
                                                                                                                let source_ml_source3_record_field33 = source_ml_source2_spread_field449.clone();
                                                                                                                {
                                                                                                                    let source_ml_source3_record_field34 = source_ml_source2_spread_field450;
                                                                                                                    {
                                                                                                                        let source_ml_source3_record_field35 = source_ml_source2_spread_field451.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source3_record_field36 = source_ml_source2_spread_field452;
                                                                                                                            {
                                                                                                                                let source_ml_source3_record_field37 = source_ml_source2_field458.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source3_record_field38 = true;
                                                                                                                                    {
                                                                                                                                        let source_ml_source3_record_field39 = source_ml_source2_spread_field455;
                                                                                                                                        {
                                                                                                                                            let source_ml_source3_record_field40 = source_ml_source2_spread_field456;
                                                                                                                                            ((source_ml_source3_record_field29, source_ml_source3_record_field30, source_ml_source3_record_field31, source_ml_source3_record_field32, source_ml_source3_record_field33.clone(), source_ml_source3_record_field34, source_ml_source3_record_field35.clone(), source_ml_source3_record_field36, source_ml_source3_record_field37.clone(), source_ml_source3_record_field38, source_ml_source3_record_field39, source_ml_source3_record_field40), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], self::source_ml_source3_record_order::merge_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![self::source_ml_source3_record_order::rebase_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], (source_ml_source2_spread444.clone()).1.clone()), vec![9f64], vec![8f64]])))
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
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }));
                                    if (unsigned_length >= length) {
                                        self::plan_java_script_array_ordinary_definition(
                                            object.clone(),
                                            key.clone(),
                                            descriptor.clone(),
                                        )
                                    } else {
                                        if !((({ let ml_reference_value155 = (match old.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value459) => {
                                                source_ml_source2_data_property_value459.clone()
                                            }
                                        }).borrow().clone(); ml_reference_value155 }).0).10) {
                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                                        } else {
                                            {
                                                let new_writable = (!((({ let ml_reference_value156 = descriptor.borrow().clone(); ml_reference_value156 }).0).11) || (({ let ml_reference_value157 = descriptor.borrow().clone(); ml_reference_value157 }).0).10);
                                                {
                                                    let temporary = if new_writable {
                                                        descriptor.clone()
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_spread460 = { let ml_reference_value158 = descriptor.borrow().clone(); ml_reference_value158 };
                                                            {
                                                                let source_ml_source2_spread_field461 = ((source_ml_source2_spread460.clone()).0).0;
                                                                {
                                                                    let source_ml_source2_spread_field462 = ((source_ml_source2_spread460.clone()).0).1;
                                                                    {
                                                                        let source_ml_source2_spread_field463 = ((source_ml_source2_spread460.clone()).0).2;
                                                                        {
                                                                            let source_ml_source2_spread_field464 = ((source_ml_source2_spread460.clone()).0).3;
                                                                            {
                                                                                let source_ml_source2_spread_field465 = ((source_ml_source2_spread460.clone()).0).4;
                                                                                {
                                                                                    let source_ml_source2_spread_field466 = ((source_ml_source2_spread460.clone()).0).5;
                                                                                    {
                                                                                        let source_ml_source2_spread_field467 = ((source_ml_source2_spread460.clone()).0).6;
                                                                                        {
                                                                                            let source_ml_source2_spread_field468 = ((source_ml_source2_spread460.clone()).0).7;
                                                                                            {
                                                                                                let source_ml_source2_spread_field469 = ((source_ml_source2_spread460.clone()).0).8;
                                                                                                {
                                                                                                    let source_ml_source2_spread_field470 = ((source_ml_source2_spread460.clone()).0).9;
                                                                                                    {
                                                                                                        let source_ml_source2_spread_field471 = ((source_ml_source2_spread460.clone()).0).10;
                                                                                                        {
                                                                                                            let source_ml_source2_spread_field472 = ((source_ml_source2_spread460.clone()).0).11;
                                                                                                            {
                                                                                                                let source_ml_source3_record_field41 = source_ml_source2_spread_field461;
                                                                                                                {
                                                                                                                    let source_ml_source3_record_field42 = source_ml_source2_spread_field462;
                                                                                                                    {
                                                                                                                        let source_ml_source3_record_field43 = source_ml_source2_spread_field463;
                                                                                                                        {
                                                                                                                            let source_ml_source3_record_field44 = source_ml_source2_spread_field464;
                                                                                                                            {
                                                                                                                                let source_ml_source3_record_field45 = source_ml_source2_spread_field465.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source3_record_field46 = source_ml_source2_spread_field466;
                                                                                                                                    {
                                                                                                                                        let source_ml_source3_record_field47 = source_ml_source2_spread_field467.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source3_record_field48 = source_ml_source2_spread_field468;
                                                                                                                                            {
                                                                                                                                                let source_ml_source3_record_field49 = source_ml_source2_spread_field469.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source3_record_field50 = source_ml_source2_spread_field470;
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source3_record_field51 = true;
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source3_record_field52 = source_ml_source2_spread_field472;
                                                                                                                                                            ((source_ml_source3_record_field41, source_ml_source3_record_field42, source_ml_source3_record_field43, source_ml_source3_record_field44, source_ml_source3_record_field45.clone(), source_ml_source3_record_field46, source_ml_source3_record_field47.clone(), source_ml_source3_record_field48, source_ml_source3_record_field49.clone(), source_ml_source3_record_field50, source_ml_source3_record_field51, source_ml_source3_record_field52), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], self::source_ml_source3_record_order::merge_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![self::source_ml_source3_record_order::rebase_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], (source_ml_source2_spread460.clone()).1.clone()), vec![10f64]])))
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
                                                        let initial = self::plan_java_script_array_ordinary_definition(object.clone(), key.clone(), temporary.clone());
                                                        match initial.clone() {
                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(shared_constructor_field1, shared_constructor_field2, _, _, shared_constructor_field3) => {
                                                                let shared_constructor_field1 = *shared_constructor_field1;
                                                                if !shared_constructor_field2 {
                                                                    initial.clone()
                                                                } else {
                                                                    {
                                                                        let blocked = (-1f64);
                                                                        {
                                                                            let source_ml_source1_temporary_values24 = match object.clone() {
                                                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value475, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value475.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value476, _, _, _) => {
                                                                                    source_ml_source2_data_property_value476.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value477) => {
                                                                                    source_ml_source2_data_property_value477.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value478, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value478.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value479, _) => {
                                                                                    source_ml_source2_data_property_value479.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value480, _) => {
                                                                                    source_ml_source2_data_property_value480.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value481, _) => {
                                                                                    source_ml_source2_data_property_value481.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value482, _) => {
                                                                                    source_ml_source2_data_property_value482.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value483) => {
                                                                                    source_ml_source2_data_property_value483.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value484, _, _) => {
                                                                                    source_ml_source2_data_property_value484.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value485, _, _) => {
                                                                                    source_ml_source2_data_property_value485.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value486, _, _) => {
                                                                                    source_ml_source2_data_property_value486.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value487, _, _) => {
                                                                                    source_ml_source2_data_property_value487.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_temporary_index24 = self::ml::Big::from_i128(0);
                                                                                {
                                                                                    let blocked_2 = self::source_ml_source1_temporary_plan_java_script_array_length_definition_loop26(unsigned_length, blocked, source_ml_source1_temporary_values24.clone(), source_ml_source1_temporary_index24.clone());
                                                                                    {
                                                                                        let final_length = if (blocked_2 < (0f64)) {
                                                                                            unsigned_length
                                                                                        } else {
                                                                                            (blocked_2 + 1f64)
                                                                                        };
                                                                                        {
                                                                                            let properties = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                                                                            {
                                                                                                let source_ml_source1_temporary_values25 = match shared_constructor_field1.clone() {
                                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value489, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value489.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value490, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value490.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value491) => {
                                                                                                        source_ml_source2_data_property_value491.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value492, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value492.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value493, _) => {
                                                                                                        source_ml_source2_data_property_value493.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value494, _) => {
                                                                                                        source_ml_source2_data_property_value494.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value495, _) => {
                                                                                                        source_ml_source2_data_property_value495.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value496, _) => {
                                                                                                        source_ml_source2_data_property_value496.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value497) => {
                                                                                                        source_ml_source2_data_property_value497.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value498, _, _) => {
                                                                                                        source_ml_source2_data_property_value498.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value499, _, _) => {
                                                                                                        source_ml_source2_data_property_value499.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value500, _, _) => {
                                                                                                        source_ml_source2_data_property_value500.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value501, _, _) => {
                                                                                                        source_ml_source2_data_property_value501.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_temporary_index25 = self::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let source_ml_source2_s502 = self::source_ml_source1_temporary_plan_java_script_array_length_definition_loop27(new_writable, final_length, properties.clone(), source_ml_source1_temporary_values25.clone(), source_ml_source1_temporary_index25.clone());
                                                                                                        match source_ml_source2_s502.clone() {
                                                                                                            self::MlSource1TemporaryPlanJavaScriptArrayLengthDefinitionLoop27Result::MlSource1TemporaryPlanJavaScriptArrayLengthDefinitionLoop27Done => {
                                                                                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(match shared_constructor_field1.clone() {
                                                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field504, source_ml_source2_nominal_field505, source_ml_source2_nominal_field506, source_ml_source2_nominal_field507, _, source_ml_source2_nominal_field509, source_ml_source2_nominal_field510, source_ml_source2_nominal_field511, source_ml_source2_nominal_field512, source_ml_source2_nominal_field513) => {
                                                                                                                        let source_ml_source2_nominal_field505 = *source_ml_source2_nominal_field505;
                                                                                                                        let source_ml_source2_nominal_field512 = *source_ml_source2_nominal_field512;
                                                                                                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field504.clone(), Box::new(source_ml_source2_nominal_field505.clone()), source_ml_source2_nominal_field506, source_ml_source2_nominal_field507, properties.clone(), source_ml_source2_nominal_field509.clone(), source_ml_source2_nominal_field510, source_ml_source2_nominal_field511.clone(), Box::new(source_ml_source2_nominal_field512.clone()), source_ml_source2_nominal_field513.clone())
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field515, source_ml_source2_nominal_field516, source_ml_source2_nominal_field517, source_ml_source2_nominal_field518, _, source_ml_source2_nominal_field520, source_ml_source2_nominal_field521, source_ml_source2_nominal_field522) => {
                                                                                                                        let source_ml_source2_nominal_field516 = *source_ml_source2_nominal_field516;
                                                                                                                        let source_ml_source2_nominal_field520 = *source_ml_source2_nominal_field520;
                                                                                                                        let source_ml_source2_nominal_field521 = *source_ml_source2_nominal_field521;
                                                                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field515.clone(), Box::new(source_ml_source2_nominal_field516.clone()), source_ml_source2_nominal_field517, source_ml_source2_nominal_field518, properties.clone(), Box::new(source_ml_source2_nominal_field520.clone()), Box::new(source_ml_source2_nominal_field521.clone()), source_ml_source2_nominal_field522.clone())
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field524, source_ml_source2_nominal_field525, source_ml_source2_nominal_field526, source_ml_source2_nominal_field527, _) => {
                                                                                                                        let source_ml_source2_nominal_field525 = *source_ml_source2_nominal_field525;
                                                                                                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field524.clone(), Box::new(source_ml_source2_nominal_field525.clone()), source_ml_source2_nominal_field526, source_ml_source2_nominal_field527, properties.clone())
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field530, source_ml_source2_nominal_field531, source_ml_source2_nominal_field532, source_ml_source2_nominal_field533, _, source_ml_source2_nominal_field535, source_ml_source2_nominal_field536, source_ml_source2_nominal_field537, source_ml_source2_nominal_field538, source_ml_source2_nominal_field539, source_ml_source2_nominal_field540) => {
                                                                                                                        let source_ml_source2_nominal_field531 = *source_ml_source2_nominal_field531;
                                                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field530.clone(), Box::new(source_ml_source2_nominal_field531.clone()), source_ml_source2_nominal_field532, source_ml_source2_nominal_field533, properties.clone(), source_ml_source2_nominal_field535.clone(), source_ml_source2_nominal_field536, source_ml_source2_nominal_field537.clone(), source_ml_source2_nominal_field538.clone(), source_ml_source2_nominal_field539.clone(), source_ml_source2_nominal_field540.clone())
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field542, source_ml_source2_nominal_field543, source_ml_source2_nominal_field544, source_ml_source2_nominal_field545, _, source_ml_source2_nominal_field547) => {
                                                                                                                        let source_ml_source2_nominal_field543 = *source_ml_source2_nominal_field543;
                                                                                                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field542.clone(), Box::new(source_ml_source2_nominal_field543.clone()), source_ml_source2_nominal_field544, source_ml_source2_nominal_field545, properties.clone(), source_ml_source2_nominal_field547.clone())
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field549, source_ml_source2_nominal_field550, source_ml_source2_nominal_field551, source_ml_source2_nominal_field552, _, source_ml_source2_nominal_field554) => {
                                                                                                                        let source_ml_source2_nominal_field550 = *source_ml_source2_nominal_field550;
                                                                                                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field549.clone(), Box::new(source_ml_source2_nominal_field550.clone()), source_ml_source2_nominal_field551, source_ml_source2_nominal_field552, properties.clone(), source_ml_source2_nominal_field554.clone())
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field556, source_ml_source2_nominal_field557, source_ml_source2_nominal_field558, source_ml_source2_nominal_field559, _, source_ml_source2_nominal_field561) => {
                                                                                                                        let source_ml_source2_nominal_field557 = *source_ml_source2_nominal_field557;
                                                                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field556.clone(), Box::new(source_ml_source2_nominal_field557.clone()), source_ml_source2_nominal_field558, source_ml_source2_nominal_field559, properties.clone(), source_ml_source2_nominal_field561.clone())
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field563, source_ml_source2_nominal_field564, source_ml_source2_nominal_field565, source_ml_source2_nominal_field566, _, source_ml_source2_nominal_field568) => {
                                                                                                                        let source_ml_source2_nominal_field564 = *source_ml_source2_nominal_field564;
                                                                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field563.clone(), Box::new(source_ml_source2_nominal_field564.clone()), source_ml_source2_nominal_field565, source_ml_source2_nominal_field566, properties.clone(), source_ml_source2_nominal_field568.clone())
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field570, source_ml_source2_nominal_field571, source_ml_source2_nominal_field572, source_ml_source2_nominal_field573, source_ml_source2_nominal_field574, source_ml_source2_nominal_field575, source_ml_source2_nominal_field576, source_ml_source2_nominal_field577, _) => {
                                                                                                                        let source_ml_source2_nominal_field571 = *source_ml_source2_nominal_field571;
                                                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field570.clone(), Box::new(source_ml_source2_nominal_field571.clone()), source_ml_source2_nominal_field572, source_ml_source2_nominal_field573.clone(), source_ml_source2_nominal_field574.clone(), source_ml_source2_nominal_field575.clone(), source_ml_source2_nominal_field576.clone(), source_ml_source2_nominal_field577, properties.clone())
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field580, source_ml_source2_nominal_field581, source_ml_source2_nominal_field582, source_ml_source2_nominal_field583, _, source_ml_source2_nominal_field585, source_ml_source2_nominal_field586) => {
                                                                                                                        let source_ml_source2_nominal_field581 = *source_ml_source2_nominal_field581;
                                                                                                                        let source_ml_source2_nominal_field585 = *source_ml_source2_nominal_field585;
                                                                                                                        let source_ml_source2_nominal_field586 = *source_ml_source2_nominal_field586;
                                                                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field580.clone(), Box::new(source_ml_source2_nominal_field581.clone()), source_ml_source2_nominal_field582, source_ml_source2_nominal_field583, properties.clone(), Box::new(source_ml_source2_nominal_field585.clone()), Box::new(source_ml_source2_nominal_field586.clone()))
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field588, source_ml_source2_nominal_field589, source_ml_source2_nominal_field590, source_ml_source2_nominal_field591, _, source_ml_source2_nominal_field593, source_ml_source2_nominal_field594) => {
                                                                                                                        let source_ml_source2_nominal_field589 = *source_ml_source2_nominal_field589;
                                                                                                                        let source_ml_source2_nominal_field593 = *source_ml_source2_nominal_field593;
                                                                                                                        let source_ml_source2_nominal_field594 = *source_ml_source2_nominal_field594;
                                                                                                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field588.clone(), Box::new(source_ml_source2_nominal_field589.clone()), source_ml_source2_nominal_field590, source_ml_source2_nominal_field591, properties.clone(), Box::new(source_ml_source2_nominal_field593.clone()), Box::new(source_ml_source2_nominal_field594.clone()))
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field596, source_ml_source2_nominal_field597, source_ml_source2_nominal_field598, source_ml_source2_nominal_field599, _, source_ml_source2_nominal_field601, source_ml_source2_nominal_field602) => {
                                                                                                                        let source_ml_source2_nominal_field597 = *source_ml_source2_nominal_field597;
                                                                                                                        let source_ml_source2_nominal_field601 = *source_ml_source2_nominal_field601;
                                                                                                                        let source_ml_source2_nominal_field602 = *source_ml_source2_nominal_field602;
                                                                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field596.clone(), Box::new(source_ml_source2_nominal_field597.clone()), source_ml_source2_nominal_field598, source_ml_source2_nominal_field599, properties.clone(), Box::new(source_ml_source2_nominal_field601.clone()), Box::new(source_ml_source2_nominal_field602.clone()))
                                                                                                                    }
                                                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field604, source_ml_source2_nominal_field605, source_ml_source2_nominal_field606, source_ml_source2_nominal_field607, _, source_ml_source2_nominal_field609, source_ml_source2_nominal_field610) => {
                                                                                                                        let source_ml_source2_nominal_field605 = *source_ml_source2_nominal_field605;
                                                                                                                        let source_ml_source2_nominal_field609 = *source_ml_source2_nominal_field609;
                                                                                                                        let source_ml_source2_nominal_field610 = *source_ml_source2_nominal_field610;
                                                                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field604.clone(), Box::new(source_ml_source2_nominal_field605.clone()), source_ml_source2_nominal_field606, source_ml_source2_nominal_field607, properties.clone(), Box::new(source_ml_source2_nominal_field609.clone()), Box::new(source_ml_source2_nominal_field610.clone()))
                                                                                                                    }
                                                                                                                }), (blocked_2 < (0f64)), self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                    let source_ml_source2_concat_value615 = shared_constructor_field3.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_concat_value616 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 105, 110, 100, 105, 99, 101, 115]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104])]));
                                                                                                                        {
                                                                                                                            let source_ml_source2_concat_value617 = if (!new_writable && !(self::ml_array::includes(&{ let ml_reference_value159 = shared_constructor_field3.borrow().clone(); ml_reference_value159 }, &self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101]), 0f64, false, |a, b| a == b))) {
                                                                                                                                std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])]))
                                                                                                                            } else {
                                                                                                                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))
                                                                                                                            };
                                                                                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value160 = source_ml_source2_concat_value615.borrow().clone(); ml_reference_value160 }), { let ml_reference_value161 = source_ml_source2_concat_value616.borrow().clone(); ml_reference_value161 }), { let ml_reference_value162 = source_ml_source2_concat_value617.borrow().clone(); ml_reference_value162 })
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
