// Generated from complete js/src/translation/javascript-descriptor-conversion.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::create_java_script_property_descriptor;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::plan_java_script_property_definition;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_temporary_define_java_script_ordinary_property_loop20;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn define_java_script_ordinary_property(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
    proposed: (
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
) -> (String, self::JavaScriptOrdinaryHeap, bool, Vec<String>) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
            {
                let error = if !(heap_error).is_empty() {
                    heap_error.clone()
                } else {
                    key_error
                };
                {
                    let ml_library_inequality_1 = error != "";
                    if ml_library_inequality_1 {
                        {
                            let source_ml_source1_field313 = heap.clone();
                            {
                                let source_ml_source1_field314 = false;
                                {
                                    let source_ml_source1_field315 = error.clone();
                                    {
                                        let source_ml_source1_field316 = Vec::<String>::new();
                                        (
                                            source_ml_source1_field315,
                                            source_ml_source1_field313,
                                            source_ml_source1_field314,
                                            source_ml_source1_field316,
                                        )
                                    }
                                }
                            }
                        }
                    } else if (proposed.clone()).9
                        && (!(self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            (proposed.clone()).8,
                        ))
                        .is_empty())
                    {
                        {
                            let source_ml_source1_field317 = heap.clone();
                            {
                                let source_ml_source1_field318 = false;
                                {
                                    let source_ml_source1_field319 = String::from(
                                        "proposed property values must belong to the actual global context",
                                    );
                                    {
                                        let source_ml_source1_field320 = Vec::<String>::new();
                                        (
                                            source_ml_source1_field319,
                                            source_ml_source1_field317,
                                            source_ml_source1_field318,
                                            source_ml_source1_field320,
                                        )
                                    }
                                }
                            }
                        }
                    } else if (proposed.clone()).5
                        && (!(self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            (proposed.clone()).4,
                        ))
                        .is_empty())
                    {
                        {
                            let source_ml_source1_field321 = heap.clone();
                            {
                                let source_ml_source1_field322 = false;
                                {
                                    let source_ml_source1_field323 = String::from(
                                        "proposed getters must belong to the actual global context",
                                    );
                                    {
                                        let source_ml_source1_field324 = Vec::<String>::new();
                                        (
                                            source_ml_source1_field323,
                                            source_ml_source1_field321,
                                            source_ml_source1_field322,
                                            source_ml_source1_field324,
                                        )
                                    }
                                }
                            }
                        }
                    } else if (proposed.clone()).7
                        && (!(self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            (proposed.clone()).6,
                        ))
                        .is_empty())
                    {
                        {
                            let source_ml_source1_field325 = heap.clone();
                            {
                                let source_ml_source1_field326 = false;
                                {
                                    let source_ml_source1_field327 = String::from(
                                        "proposed setters must belong to the actual global context",
                                    );
                                    {
                                        let source_ml_source1_field328 = Vec::<String>::new();
                                        (
                                            source_ml_source1_field327,
                                            source_ml_source1_field325,
                                            source_ml_source1_field326,
                                            source_ml_source1_field328,
                                        )
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(
                                match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                        source_ml_source1_data_property_value329,
                                        _,
                                    ) => source_ml_source1_data_property_value329,
                                },
                                identity.clone(),
                            );
                            {
                                let object = self::ml_array::optional_at(
                                    (match &heap {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                            source_ml_source1_data_property_value330,
                                            _,
                                        ) => source_ml_source1_data_property_value330.as_slice(),
                                    }),
                                    self::ml_array::number_index(index),
                                );
                                if match (object.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    {
                                        let source_ml_source1_field331 = heap.clone();
                                        {
                                            let source_ml_source1_field332 = false;
                                            {
                                                let source_ml_source1_field333 = String::from(
                                                    "DefineOwnProperty needs its actual Object identity",
                                                );
                                                {
                                                    let source_ml_source1_field334 =
                                                        Vec::<String>::new();
                                                    (
                                                        source_ml_source1_field333,
                                                        source_ml_source1_field331,
                                                        source_ml_source1_field332,
                                                        source_ml_source1_field334,
                                                    )
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let at = self::find_java_script_ordinary_property(
                                            match object.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    source_ml_source1_data_property_value335,
                                                ) => source_ml_source1_data_property_value335,
                                            },
                                            key.clone(),
                                        );
                                        {
                                            let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value336) => {
                                                            source_ml_source1_data_property_value336
                                                        }
                                                    }, self::ml_array::number_index(at));
                                            {
                                                let current = if match (
                                                    property.clone(),
                                                    None::<()>,
                                                ) {
                                                    (Some(_ml_left), Some(_ml_right)) => false,
                                                    (None, None) => true,
                                                    _ => false,
                                                } {
                                                    self::create_java_script_property_descriptor(
                                                        "data", false, false,
                                                    )
                                                } else {
                                                    match property.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value337) => {
                                                                    source_ml_source1_data_property_value337.clone()
                                                                }
                                                            }
                                                };
                                                {
                                                    let transition = self::plan_java_script_property_definition(!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }), match object.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value338, _, _) => {
                                                                    source_ml_source1_data_property_value338
                                                                }
                                                            }, current, proposed.clone());
                                                    if ((transition.clone()).1) == "reject" {
                                                                {
                                                                    let source_ml_source1_field339 = heap.clone();
                                                                    {
                                                                        let source_ml_source1_field340 = false;
                                                                        {
                                                                            let source_ml_source1_field341 = (transition.clone()).2;
                                                                            {
                                                                                let source_ml_source1_field342 = Vec::<String>::new();
                                                                                (source_ml_source1_field341, source_ml_source1_field339, source_ml_source1_field340, source_ml_source1_field342)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else if ((transition.clone()).1) == "keep" {
                                                                    {
                                                                        let source_ml_source1_field343 = heap.clone();
                                                                        {
                                                                            let source_ml_source1_field344 = true;
                                                                            {
                                                                                let source_ml_source1_field345 = String::new();
                                                                                {
                                                                                    let source_ml_source1_field346 = Vec::<String>::new();
                                                                                    (source_ml_source1_field345, source_ml_source1_field343, source_ml_source1_field344, source_ml_source1_field346)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else if (at < 0f64) && (((match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value348) => {
                                                                            source_ml_source1_data_property_value348
                                                                        }
                                                                    }).len() as f64) >= 4096f64) {
                                                                        {
                                                                            let source_ml_source1_field349 = heap.clone();
                                                                            {
                                                                                let source_ml_source1_field350 = false;
                                                                                {
                                                                                    let source_ml_source1_field351 = String::from("ordinary properties exceed their explicit ordered storage budget");
                                                                                    {
                                                                                        let source_ml_source1_field352 = Vec::<String>::new();
                                                                                        (source_ml_source1_field351, source_ml_source1_field349, source_ml_source1_field350, source_ml_source1_field352)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let count = 0f64;
                                                                            {
                                                                                let source_ml_temporary_values19 = match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value353, _) => {
                                                                                        source_ml_source1_data_property_value353
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_temporary_index19 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let count_2 = self::source_ml_temporary_define_java_script_ordinary_property_loop20(count, source_ml_temporary_values19, source_ml_temporary_index19);
                                                                                        if (at < 0f64) && (count_2 >= 16_384f64) {
                                                                                            {
                                                                                                let source_ml_source1_field354 = heap.clone();
                                                                                                {
                                                                                                    let source_ml_source1_field355 = false;
                                                                                                    {
                                                                                                        let source_ml_source1_field356 = String::from("ordinary properties exceed their explicit ordered storage budget");
                                                                                                        {
                                                                                                            let source_ml_source1_field357 = Vec::<String>::new();
                                                                                                            (source_ml_source1_field356, source_ml_source1_field354, source_ml_source1_field355, source_ml_source1_field357)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            {
                                                                                                let updated = self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(key.clone()), (transition.clone()).0);
                                                                                                {
                                                                                                    let properties = if at < 0f64 {
                                                                                                        {
                                                                                                            let source_ml_source1_concat_value359 = match object.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value358) => {
                                                                                                                    source_ml_source1_data_property_value358
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_concat_value360 = vec![updated.clone()];
                                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryProperty>::new(), source_ml_source1_concat_value359), source_ml_source1_concat_value360)
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        { let ml_values2 = match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value361) => {
                                                                                                                source_ml_source1_data_property_value361
                                                                                                            }
                                                                                                        }; let ml_callback2 = { let at = at; let updated = updated.clone(); std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> self::JavaScriptOrdinaryProperty { if position == at {
                                                                                                            updated.clone()
                                                                                                        } else {
                                                                                                            entry.clone()
                                                                                                        } }) }; ml_values2.iter().cloned().enumerate().map(|(ml_index2, ml_value2)| ml_callback2(ml_value2, (ml_index2 as f64))).collect::<Vec<_>>() }
                                                                                                    };
                                                                                                    {
                                                                                                        let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value362, _, _, _, _) => {
                                                                                                                source_ml_source1_data_property_value362.clone()
                                                                                                            }
                                                                                                        }, Box::new(match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value363, _, _, _) => {
                                                                                                                let source_ml_source1_data_property_value363 = *source_ml_source1_data_property_value363;
                                                                                                                source_ml_source1_data_property_value363.clone()
                                                                                                            }
                                                                                                        }), match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value364, _, _) => {
                                                                                                                source_ml_source1_data_property_value364
                                                                                                            }
                                                                                                        }, match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value365, _) => {
                                                                                                                source_ml_source1_data_property_value365
                                                                                                            }
                                                                                                        }, properties);
                                                                                                        {
                                                                                                            let source_ml_source1_field366 = self::replace_java_script_ordinary_object(heap.clone(), replacement);
                                                                                                            {
                                                                                                                let source_ml_source1_field367 = true;
                                                                                                                {
                                                                                                                    let source_ml_source1_field368 = String::new();
                                                                                                                    {
                                                                                                                        let source_ml_source1_field369 = (transition.clone()).3;
                                                                                                                        (source_ml_source1_field368, source_ml_source1_field366, source_ml_source1_field367, source_ml_source1_field369)
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn delete_java_script_ordinary_property(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
) -> (String, self::JavaScriptOrdinaryHeap, bool) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
            {
                let error = if !(heap_error).is_empty() {
                    heap_error.clone()
                } else {
                    key_error
                };
                {
                    let ml_library_inequality_1 = error != "";
                    if ml_library_inequality_1 {
                        {
                            let source_ml_source1_field370 = heap.clone();
                            {
                                let source_ml_source1_field371 = false;
                                {
                                    let source_ml_source1_field372 = error.clone();
                                    (
                                        source_ml_source1_field372,
                                        source_ml_source1_field370,
                                        source_ml_source1_field371,
                                    )
                                }
                            }
                        }
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(
                                match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                        source_ml_source1_data_property_value373,
                                        _,
                                    ) => source_ml_source1_data_property_value373,
                                },
                                identity.clone(),
                            );
                            {
                                let object = self::ml_array::optional_at(
                                    (match &heap {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                            source_ml_source1_data_property_value374,
                                            _,
                                        ) => source_ml_source1_data_property_value374.as_slice(),
                                    }),
                                    self::ml_array::number_index(index),
                                );
                                if match (object.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    {
                                        let source_ml_source1_field375 = heap.clone();
                                        {
                                            let source_ml_source1_field376 = false;
                                            {
                                                let source_ml_source1_field377 = String::from(
                                                    "Delete needs its actual Object identity",
                                                );
                                                (
                                                    source_ml_source1_field377,
                                                    source_ml_source1_field375,
                                                    source_ml_source1_field376,
                                                )
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let at = self::find_java_script_ordinary_property(
                                            match object.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    source_ml_source1_data_property_value378,
                                                ) => source_ml_source1_data_property_value378,
                                            },
                                            key.clone(),
                                        );
                                        {
                                            let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value379) => {
                                                source_ml_source1_data_property_value379
                                            }
                                        }, self::ml_array::number_index(at));
                                            if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            {
                                                let source_ml_source1_field380 = heap.clone();
                                                {
                                                    let source_ml_source1_field381 = true;
                                                    {
                                                        let source_ml_source1_field382 = String::new();
                                                        (source_ml_source1_field382, source_ml_source1_field380, source_ml_source1_field381)
                                                    }
                                                }
                                            }
                                        } else if ((match property.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value383) => {
                                                    source_ml_source1_data_property_value383.clone()
                                                }
                                            }).0) {
                                                {
                                                    let properties = { let ml_values3 = match object.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value387) => {
                                                            source_ml_source1_data_property_value387
                                                        }
                                                    }; let ml_callback3 = { let at = at; std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> bool { position != at }) }; ml_values3.iter().cloned().enumerate().filter_map(|(ml_index3, ml_value3)| if ml_callback3(ml_value3.clone(), (ml_index3 as f64)) { Some(ml_value3) } else { None }).collect::<Vec<_>>() };
                                                    {
                                                        let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value388, _, _, _, _) => {
                                                                source_ml_source1_data_property_value388.clone()
                                                            }
                                                        }, Box::new(match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value389, _, _, _) => {
                                                                let source_ml_source1_data_property_value389 = *source_ml_source1_data_property_value389;
                                                                source_ml_source1_data_property_value389.clone()
                                                            }
                                                        }), match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value390, _, _) => {
                                                                source_ml_source1_data_property_value390
                                                            }
                                                        }, match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value391, _) => {
                                                                source_ml_source1_data_property_value391
                                                            }
                                                        }, properties);
                                                        {
                                                            let source_ml_source1_field392 = self::replace_java_script_ordinary_object(heap.clone(), replacement);
                                                            {
                                                                let source_ml_source1_field393 = true;
                                                                {
                                                                    let source_ml_source1_field394 = String::new();
                                                                    (source_ml_source1_field394, source_ml_source1_field392, source_ml_source1_field393)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                {
                                                    let source_ml_source1_field384 = heap.clone();
                                                    {
                                                        let source_ml_source1_field385 = false;
                                                        {
                                                            let source_ml_source1_field386 = String::new();
                                                            (source_ml_source1_field386, source_ml_source1_field384, source_ml_source1_field385)
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn prevent_java_script_ordinary_extensions(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
) -> (String, self::JavaScriptOrdinaryHeap, bool) {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if !(error).is_empty() {
            {
                let source_ml_source1_field395 = heap.clone();
                {
                    let source_ml_source1_field396 = false;
                    {
                        let source_ml_source1_field397 = error.clone();
                        (
                            source_ml_source1_field397,
                            source_ml_source1_field395,
                            source_ml_source1_field396,
                        )
                    }
                }
            }
        } else {
            {
                let index = self::find_java_script_ordinary_object(
                    match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value398,
                            _,
                        ) => source_ml_source1_data_property_value398,
                    },
                    identity.clone(),
                );
                {
                    let object = self::ml_array::optional_at(
                        (match &heap {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source1_data_property_value399,
                                _,
                            ) => source_ml_source1_data_property_value399.as_slice(),
                        }),
                        self::ml_array::number_index(index),
                    );
                    if match (object.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        {
                            let source_ml_source1_field400 = heap.clone();
                            {
                                let source_ml_source1_field401 = false;
                                {
                                    let source_ml_source1_field402 = String::from(
                                        "PreventExtensions needs its actual Object identity",
                                    );
                                    (
                                        source_ml_source1_field402,
                                        source_ml_source1_field400,
                                        source_ml_source1_field401,
                                    )
                                }
                            }
                        }
                    } else {
                        {
                            let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(
                                match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        source_ml_source1_data_property_value403,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value403.clone(),
                                },
                                Box::new(match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        _,
                                        source_ml_source1_data_property_value404,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source1_data_property_value404 =
                                            *source_ml_source1_data_property_value404;
                                        source_ml_source1_data_property_value404.clone()
                                    }
                                }),
                                false,
                                match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value405,
                                        _,
                                    ) => source_ml_source1_data_property_value405,
                                },
                                match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value406,
                                    ) => source_ml_source1_data_property_value406,
                                },
                            );
                            {
                                let source_ml_source1_field407 =
                                    self::replace_java_script_ordinary_object(
                                        heap.clone(),
                                        replacement,
                                    );
                                {
                                    let source_ml_source1_field408 = true;
                                    {
                                        let source_ml_source1_field409 = String::new();
                                        (
                                            source_ml_source1_field409,
                                            source_ml_source1_field407,
                                            source_ml_source1_field408,
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
}
