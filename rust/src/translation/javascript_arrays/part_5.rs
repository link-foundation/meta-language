// Generated from complete js/src/translation/javascript-arrays.js. Do not edit by hand.
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::find_java_script_array_property;
use super::ml_array;
use super::ml_math;
use super::read_java_script_array_length_key;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_length(object: self::JavaScriptOrdinaryObject) -> f64 {
    {
        let property = {
            let source_ml_source1_array_read_value688 = match object.clone() {
                self::JavaScriptOrdinaryObject::ClosureObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value662,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value662.clone(),
                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value663,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value663.clone(),
                self::JavaScriptOrdinaryObject::ArrayObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value664,
                ) => source_ml_source1_data_property_value664.clone(),
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value665,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value665.clone(),
                self::JavaScriptOrdinaryObject::MapObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value666,
                    _,
                ) => source_ml_source1_data_property_value666.clone(),
                self::JavaScriptOrdinaryObject::SetObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value667,
                    _,
                ) => source_ml_source1_data_property_value667.clone(),
                self::JavaScriptOrdinaryObject::WeakMapObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value668,
                    _,
                ) => source_ml_source1_data_property_value668.clone(),
                self::JavaScriptOrdinaryObject::WeakSetObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value669,
                    _,
                ) => source_ml_source1_data_property_value669.clone(),
                self::JavaScriptOrdinaryObject::OrdinaryObject(
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value670,
                ) => source_ml_source1_data_property_value670.clone(),
                self::JavaScriptOrdinaryObject::HostErrorObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value671,
                    _,
                    _,
                ) => source_ml_source1_data_property_value671.clone(),
                self::JavaScriptOrdinaryObject::HostURLObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value672,
                    _,
                    _,
                ) => source_ml_source1_data_property_value672.clone(),
                self::JavaScriptOrdinaryObject::HostArrayObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value673,
                    _,
                    _,
                ) => source_ml_source1_data_property_value673.clone(),
                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value674,
                    _,
                    _,
                ) => source_ml_source1_data_property_value674.clone(),
            };
            {
                let source_ml_source1_array_read_index689 = self::find_java_script_array_property(
                    match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value675,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value675.clone(),
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value676,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value676.clone(),
                        self::JavaScriptOrdinaryObject::ArrayObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value677,
                        ) => source_ml_source1_data_property_value677.clone(),
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value678,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value678.clone(),
                        self::JavaScriptOrdinaryObject::MapObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value679,
                            _,
                        ) => source_ml_source1_data_property_value679.clone(),
                        self::JavaScriptOrdinaryObject::SetObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value680,
                            _,
                        ) => source_ml_source1_data_property_value680.clone(),
                        self::JavaScriptOrdinaryObject::WeakMapObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value681,
                            _,
                        ) => source_ml_source1_data_property_value681.clone(),
                        self::JavaScriptOrdinaryObject::WeakSetObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value682,
                            _,
                        ) => source_ml_source1_data_property_value682.clone(),
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value683,
                        ) => source_ml_source1_data_property_value683.clone(),
                        self::JavaScriptOrdinaryObject::HostErrorObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value684,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value684.clone(),
                        self::JavaScriptOrdinaryObject::HostURLObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value685,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value685.clone(),
                        self::JavaScriptOrdinaryObject::HostArrayObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value686,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value686.clone(),
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value687,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value687.clone(),
                    },
                    self::read_java_script_array_length_key(),
                );
                {
                    let ml_reference_array101 = source_ml_source1_array_read_value688;
                    let ml_reference_position101 = source_ml_source1_array_read_index689;
                    self::ml_array::optional_at(
                        &*ml_reference_array101.borrow(),
                        self::ml_array::number_index(ml_reference_position101),
                    )
                }
            }
        };
        if match (property.clone(), None::<()>) {
            (Some(_ml_left), Some(_ml_right)) => false,
            (None, None) => true,
            _ => false,
        } {
            -1f64
        } else {
            self::read_java_script_array_length_value(
                ({
                    (match property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            _,
                            source_ml_source1_data_property_value690,
                        ) => source_ml_source1_data_property_value690.clone(),
                    })
                    .borrow()
                    .clone()
                })
                .8,
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_length_value(value: self::JavaScriptSemanticValue) -> f64 {
    match value.clone() {
        self::JavaScriptSemanticValue::Number(value_value) => value_value,
        _ => -1f64,
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_array_length(length: f64) -> bool {
    ((self::ml_math::is_integer(length) && (length >= 0f64)) && (length <= 4_294_967_295f64))
        && !({
            let source_ml_source1_same_value_operand700 = -0f64;
            ((((length).partial_cmp(&(source_ml_source1_same_value_operand700))
                == Some(std::cmp::Ordering::Equal))
                && ((length != 0f64)
                    || ((1f64 / length) == (1f64 / source_ml_source1_same_value_operand700))))
                || ((length != length)
                    && (source_ml_source1_same_value_operand700
                        != source_ml_source1_same_value_operand700)))
        })
}
