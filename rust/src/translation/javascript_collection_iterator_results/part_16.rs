// Generated from complete js/src/translation/javascript-collection-iterator-results.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::create_java_script_property_descriptor;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_property_definition;
use super::replace_java_script_ordinary_object;
use super::source_ml_temporary_commit_java_script_ordinary_own_property_loop122;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn commit_java_script_ordinary_own_property(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
    key: self::JavaScriptSemanticValue,
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
) -> std::rc::Rc<
    std::cell::RefCell<(
        self::ml_string::Text,
        self::JavaScriptOrdinaryHeap,
        Option<(Option<bool>,)>,
        bool,
        std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
    )>,
> {
    {
        let at = self::find_java_script_ordinary_property(
            match object.clone() {
                self::JavaScriptOrdinaryObject::ClosureObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value7862,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value7862.clone(),
                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value7863,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value7863.clone(),
                self::JavaScriptOrdinaryObject::ArrayObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value7864,
                ) => source_ml_source1_data_property_value7864.clone(),
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value7865,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value7865.clone(),
                self::JavaScriptOrdinaryObject::MapObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value7866,
                    _,
                ) => source_ml_source1_data_property_value7866.clone(),
                self::JavaScriptOrdinaryObject::SetObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value7867,
                    _,
                ) => source_ml_source1_data_property_value7867.clone(),
                self::JavaScriptOrdinaryObject::WeakMapObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value7868,
                    _,
                ) => source_ml_source1_data_property_value7868.clone(),
                self::JavaScriptOrdinaryObject::WeakSetObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value7869,
                    _,
                ) => source_ml_source1_data_property_value7869.clone(),
                self::JavaScriptOrdinaryObject::OrdinaryObject(
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value7870,
                ) => source_ml_source1_data_property_value7870.clone(),
                self::JavaScriptOrdinaryObject::HostErrorObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value7871,
                    _,
                    _,
                ) => source_ml_source1_data_property_value7871.clone(),
                self::JavaScriptOrdinaryObject::HostURLObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value7872,
                    _,
                    _,
                ) => source_ml_source1_data_property_value7872.clone(),
                self::JavaScriptOrdinaryObject::HostArrayObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value7873,
                    _,
                    _,
                ) => source_ml_source1_data_property_value7873.clone(),
                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value7874,
                    _,
                    _,
                ) => source_ml_source1_data_property_value7874.clone(),
            },
            key.clone(),
        );
        {
            let property = {
                let source_ml_source1_array_read_value7888 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value7875,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value7875.clone(),
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value7876,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value7876.clone(),
                    self::JavaScriptOrdinaryObject::ArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value7877,
                    ) => source_ml_source1_data_property_value7877.clone(),
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value7878,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value7878.clone(),
                    self::JavaScriptOrdinaryObject::MapObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value7879,
                        _,
                    ) => source_ml_source1_data_property_value7879.clone(),
                    self::JavaScriptOrdinaryObject::SetObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value7880,
                        _,
                    ) => source_ml_source1_data_property_value7880.clone(),
                    self::JavaScriptOrdinaryObject::WeakMapObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value7881,
                        _,
                    ) => source_ml_source1_data_property_value7881.clone(),
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value7882,
                        _,
                    ) => source_ml_source1_data_property_value7882.clone(),
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value7883,
                    ) => source_ml_source1_data_property_value7883.clone(),
                    self::JavaScriptOrdinaryObject::HostErrorObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value7884,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value7884.clone(),
                    self::JavaScriptOrdinaryObject::HostURLObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value7885,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value7885.clone(),
                    self::JavaScriptOrdinaryObject::HostArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value7886,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value7886.clone(),
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value7887,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value7887.clone(),
                };
                {
                    let ml_reference_array189 = source_ml_source1_array_read_value7888;
                    let ml_reference_position189 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array189.borrow(),
                        self::ml_array::number_index(ml_reference_position189),
                    )
                }
            };
            {
                let current = if match (property.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::create_java_script_property_descriptor(
                        &(self::ml_string::Text::from_units(vec![100, 97, 116, 97])),
                        false,
                        false,
                    )
                } else {
                    match property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            _,
                            source_ml_source1_data_property_value7890,
                        ) => source_ml_source1_data_property_value7890.clone(),
                    }
                };
                {
                    let transition = self::plan_java_script_property_definition(
                        !(match (property.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }),
                        match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                source_ml_source1_data_property_value7891,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value7891,
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                source_ml_source1_data_property_value7892,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value7892,
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                source_ml_source1_data_property_value7893,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value7893,
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                source_ml_source1_data_property_value7894,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value7894,
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                source_ml_source1_data_property_value7895,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value7895,
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                source_ml_source1_data_property_value7896,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value7896,
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                source_ml_source1_data_property_value7897,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value7897,
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                source_ml_source1_data_property_value7898,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value7898,
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                source_ml_source1_data_property_value7899,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value7899,
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                source_ml_source1_data_property_value7900,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value7900,
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                source_ml_source1_data_property_value7901,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value7901,
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                source_ml_source1_data_property_value7902,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value7902,
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                source_ml_source1_data_property_value7903,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value7903,
                        },
                        current,
                        proposed.clone(),
                    );
                    if (({ transition.borrow().clone() }).1)
                        == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))
                    {
                        std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source1_field7906 = ({ transition.borrow().clone() }).2;
                            {
                                let source_ml_source1_field7907 =
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                        self::ml_string::Text,
                                    >::new(
                                    )));
                                (
                                    source_ml_source1_field7906,
                                    heap.clone(),
                                    None::<(Option<bool>,)>,
                                    false,
                                    source_ml_source1_field7907,
                                )
                            }
                        }))
                    } else if (({ transition.borrow().clone() }).1)
                        == (self::ml_string::Text::from_units(vec![107, 101, 101, 112]))
                    {
                        std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source1_field7911 =
                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                    self::ml_string::Text,
                                >::new(
                                )));
                            (
                                self::ml_string::Text::from_units(vec![]),
                                heap.clone(),
                                None::<(Option<bool>,)>,
                                true,
                                source_ml_source1_field7911,
                            )
                        }))
                    } else if (at < 0f64)
                        && (({
                            let ml_reference_array193 = match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value7925,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value7925.clone(),
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value7926,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value7926.clone(),
                                self::JavaScriptOrdinaryObject::ArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value7927,
                                ) => source_ml_source1_data_property_value7927.clone(),
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value7928,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value7928.clone(),
                                self::JavaScriptOrdinaryObject::MapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value7929,
                                    _,
                                ) => source_ml_source1_data_property_value7929.clone(),
                                self::JavaScriptOrdinaryObject::SetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value7930,
                                    _,
                                ) => source_ml_source1_data_property_value7930.clone(),
                                self::JavaScriptOrdinaryObject::WeakMapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value7931,
                                    _,
                                ) => source_ml_source1_data_property_value7931.clone(),
                                self::JavaScriptOrdinaryObject::WeakSetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value7932,
                                    _,
                                ) => source_ml_source1_data_property_value7932.clone(),
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value7933,
                                ) => source_ml_source1_data_property_value7933.clone(),
                                self::JavaScriptOrdinaryObject::HostErrorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value7934,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value7934.clone(),
                                self::JavaScriptOrdinaryObject::HostURLObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value7935,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value7935.clone(),
                                self::JavaScriptOrdinaryObject::HostArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value7936,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value7936.clone(),
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value7937,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value7937.clone(),
                            };
                            ml_reference_array193.borrow().len() as f64
                        }) >= 4096f64)
                    {
                        std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source1_field7941 =
                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                    self::ml_string::Text,
                                >::new(
                                )));
                            {
                                let source_ml_source1_field7942 = Some(Some(true));
                                (
                                    self::ml_string::Text::from_units(vec![
                                        111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111,
                                        112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101,
                                        101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112,
                                        108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101,
                                        100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117,
                                        100, 103, 101, 116,
                                    ]),
                                    heap.clone(),
                                    source_ml_source1_field7942,
                                    false,
                                    source_ml_source1_field7941,
                                )
                            }
                        }))
                    } else {
                        {
                            let count = 0f64;
                            {
                                let source_ml_temporary_values121 = match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                        source_ml_source1_data_property_value7943,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value7943.clone(),
                                };
                                {
                                    let source_ml_temporary_index121 = self::ml::Big::from_i128(0);
                                    {
                                        let count_2 = self::source_ml_temporary_commit_java_script_ordinary_own_property_loop122(count, source_ml_temporary_values121, source_ml_temporary_index121);
                                        if (at < 0f64) && (count_2 >= 16_384f64) {
                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source1_field7947 =
                                                    std::rc::Rc::new(std::cell::RefCell::new(
                                                        Vec::<self::ml_string::Text>::new(),
                                                    ));
                                                {
                                                    let source_ml_source1_field7948 =
                                                        Some(Some(true));
                                                    (
                                                        self::ml_string::Text::from_units(vec![
                                                            111, 114, 100, 105, 110, 97, 114, 121,
                                                            32, 112, 114, 111, 112, 101, 114, 116,
                                                            105, 101, 115, 32, 101, 120, 99, 101,
                                                            101, 100, 32, 116, 104, 101, 105, 114,
                                                            32, 101, 120, 112, 108, 105, 99, 105,
                                                            116, 32, 111, 114, 100, 101, 114, 101,
                                                            100, 32, 115, 116, 111, 114, 97, 103,
                                                            101, 32, 98, 117, 100, 103, 101, 116,
                                                        ]),
                                                        heap.clone(),
                                                        source_ml_source1_field7948,
                                                        false,
                                                        source_ml_source1_field7947,
                                                    )
                                                }
                                            }))
                                        } else {
                                            {
                                                let updated = self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(key.clone()), ({ transition.borrow().clone() }).0);
                                                {
                                                    let properties = if at < 0f64 {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_concat_value7962 = match object.clone() {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value7949, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value7949.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value7950, _, _, _) => {
                                                                            source_ml_source1_data_property_value7950.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value7951) => {
                                                                            source_ml_source1_data_property_value7951.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value7952, _, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value7952.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value7953, _) => {
                                                                            source_ml_source1_data_property_value7953.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value7954, _) => {
                                                                            source_ml_source1_data_property_value7954.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value7955, _) => {
                                                                            source_ml_source1_data_property_value7955.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value7956, _) => {
                                                                            source_ml_source1_data_property_value7956.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value7957) => {
                                                                            source_ml_source1_data_property_value7957.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value7958, _, _) => {
                                                                            source_ml_source1_data_property_value7958.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value7959, _, _) => {
                                                                            source_ml_source1_data_property_value7959.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value7960, _, _) => {
                                                                            source_ml_source1_data_property_value7960.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value7961, _, _) => {
                                                                            source_ml_source1_data_property_value7961.clone()
                                                                        }
                                                                    };
                                                            {
                                                                let source_ml_source1_concat_value7963 =
                                                                    std::rc::Rc::new(
                                                                        std::cell::RefCell::new(
                                                                            vec![updated.clone()],
                                                                        ),
                                                                    );
                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryProperty>::new(), { source_ml_source1_concat_value7962.borrow().clone() }), { source_ml_source1_concat_value7963.borrow().clone() })
                                                            }
                                                        }))
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let ml_values197 = {
                                                                (match object.clone() {
                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value7964, _, _, _, _, _) => {
                                                                        source_ml_source1_data_property_value7964.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value7965, _, _, _) => {
                                                                        source_ml_source1_data_property_value7965.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value7966) => {
                                                                        source_ml_source1_data_property_value7966.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value7967, _, _, _, _, _, _) => {
                                                                        source_ml_source1_data_property_value7967.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value7968, _) => {
                                                                        source_ml_source1_data_property_value7968.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value7969, _) => {
                                                                        source_ml_source1_data_property_value7969.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value7970, _) => {
                                                                        source_ml_source1_data_property_value7970.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value7971, _) => {
                                                                        source_ml_source1_data_property_value7971.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value7972) => {
                                                                        source_ml_source1_data_property_value7972.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value7973, _, _) => {
                                                                        source_ml_source1_data_property_value7973.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value7974, _, _) => {
                                                                        source_ml_source1_data_property_value7974.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value7975, _, _) => {
                                                                        source_ml_source1_data_property_value7975.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value7976, _, _) => {
                                                                        source_ml_source1_data_property_value7976.clone()
                                                                    }
                                                                }).borrow().clone()
                                                            };
                                                            let ml_callback197 = {
                                                                let at = at;
                                                                let updated = updated.clone();
                                                                std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> self::JavaScriptOrdinaryProperty { if position == at {
                                                                    updated.clone()
                                                                } else {
                                                                    entry.clone()
                                                                } })
                                                            };
                                                            ml_values197
                                                                .iter()
                                                                .cloned()
                                                                .enumerate()
                                                                .map(
                                                                    |(ml_index197, ml_value197)| {
                                                                        ml_callback197(
                                                                            ml_value197,
                                                                            (ml_index197 as f64),
                                                                        )
                                                                    },
                                                                )
                                                                .collect::<Vec<_>>()
                                                        }))
                                                    };
                                                    {
                                                        let replacement = match object.clone() {
                                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field7977, source_ml_source1_nominal_field7978, source_ml_source1_nominal_field7979, source_ml_source1_nominal_field7980, _, source_ml_source1_nominal_field7982, source_ml_source1_nominal_field7983, source_ml_source1_nominal_field7984, source_ml_source1_nominal_field7985, source_ml_source1_nominal_field7986) => {
                                                                        let source_ml_source1_nominal_field7978 = *source_ml_source1_nominal_field7978;
                                                                        let source_ml_source1_nominal_field7985 = *source_ml_source1_nominal_field7985;
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field7977.clone(), Box::new(source_ml_source1_nominal_field7978.clone()), source_ml_source1_nominal_field7979, source_ml_source1_nominal_field7980, properties.clone(), source_ml_source1_nominal_field7982.clone(), source_ml_source1_nominal_field7983, source_ml_source1_nominal_field7984.clone(), Box::new(source_ml_source1_nominal_field7985.clone()), source_ml_source1_nominal_field7986.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field7988, source_ml_source1_nominal_field7989, source_ml_source1_nominal_field7990, source_ml_source1_nominal_field7991, _, source_ml_source1_nominal_field7993, source_ml_source1_nominal_field7994, source_ml_source1_nominal_field7995) => {
                                                                        let source_ml_source1_nominal_field7989 = *source_ml_source1_nominal_field7989;
                                                                        let source_ml_source1_nominal_field7993 = *source_ml_source1_nominal_field7993;
                                                                        let source_ml_source1_nominal_field7994 = *source_ml_source1_nominal_field7994;
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field7988.clone(), Box::new(source_ml_source1_nominal_field7989.clone()), source_ml_source1_nominal_field7990, source_ml_source1_nominal_field7991, properties.clone(), Box::new(source_ml_source1_nominal_field7993.clone()), Box::new(source_ml_source1_nominal_field7994.clone()), source_ml_source1_nominal_field7995.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field7997, source_ml_source1_nominal_field7998, source_ml_source1_nominal_field7999, source_ml_source1_nominal_field8000, _) => {
                                                                        let source_ml_source1_nominal_field7998 = *source_ml_source1_nominal_field7998;
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field7997.clone(), Box::new(source_ml_source1_nominal_field7998.clone()), source_ml_source1_nominal_field7999, source_ml_source1_nominal_field8000, properties.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field8003, source_ml_source1_nominal_field8004, source_ml_source1_nominal_field8005, source_ml_source1_nominal_field8006, _, source_ml_source1_nominal_field8008, source_ml_source1_nominal_field8009, source_ml_source1_nominal_field8010, source_ml_source1_nominal_field8011, source_ml_source1_nominal_field8012, source_ml_source1_nominal_field8013) => {
                                                                        let source_ml_source1_nominal_field8004 = *source_ml_source1_nominal_field8004;
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field8003.clone(), Box::new(source_ml_source1_nominal_field8004.clone()), source_ml_source1_nominal_field8005, source_ml_source1_nominal_field8006, properties.clone(), source_ml_source1_nominal_field8008.clone(), source_ml_source1_nominal_field8009, source_ml_source1_nominal_field8010.clone(), source_ml_source1_nominal_field8011.clone(), source_ml_source1_nominal_field8012.clone(), source_ml_source1_nominal_field8013.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field8015, source_ml_source1_nominal_field8016, source_ml_source1_nominal_field8017, source_ml_source1_nominal_field8018, _, source_ml_source1_nominal_field8020) => {
                                                                        let source_ml_source1_nominal_field8016 = *source_ml_source1_nominal_field8016;
                                                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field8015.clone(), Box::new(source_ml_source1_nominal_field8016.clone()), source_ml_source1_nominal_field8017, source_ml_source1_nominal_field8018, properties.clone(), source_ml_source1_nominal_field8020.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field8022, source_ml_source1_nominal_field8023, source_ml_source1_nominal_field8024, source_ml_source1_nominal_field8025, _, source_ml_source1_nominal_field8027) => {
                                                                        let source_ml_source1_nominal_field8023 = *source_ml_source1_nominal_field8023;
                                                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field8022.clone(), Box::new(source_ml_source1_nominal_field8023.clone()), source_ml_source1_nominal_field8024, source_ml_source1_nominal_field8025, properties.clone(), source_ml_source1_nominal_field8027.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field8029, source_ml_source1_nominal_field8030, source_ml_source1_nominal_field8031, source_ml_source1_nominal_field8032, _, source_ml_source1_nominal_field8034) => {
                                                                        let source_ml_source1_nominal_field8030 = *source_ml_source1_nominal_field8030;
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field8029.clone(), Box::new(source_ml_source1_nominal_field8030.clone()), source_ml_source1_nominal_field8031, source_ml_source1_nominal_field8032, properties.clone(), source_ml_source1_nominal_field8034.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field8036, source_ml_source1_nominal_field8037, source_ml_source1_nominal_field8038, source_ml_source1_nominal_field8039, _, source_ml_source1_nominal_field8041) => {
                                                                        let source_ml_source1_nominal_field8037 = *source_ml_source1_nominal_field8037;
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field8036.clone(), Box::new(source_ml_source1_nominal_field8037.clone()), source_ml_source1_nominal_field8038, source_ml_source1_nominal_field8039, properties.clone(), source_ml_source1_nominal_field8041.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field8043, source_ml_source1_nominal_field8044, source_ml_source1_nominal_field8045, source_ml_source1_nominal_field8046, source_ml_source1_nominal_field8047, source_ml_source1_nominal_field8048, source_ml_source1_nominal_field8049, source_ml_source1_nominal_field8050, _) => {
                                                                        let source_ml_source1_nominal_field8044 = *source_ml_source1_nominal_field8044;
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field8043.clone(), Box::new(source_ml_source1_nominal_field8044.clone()), source_ml_source1_nominal_field8045, source_ml_source1_nominal_field8046.clone(), source_ml_source1_nominal_field8047.clone(), source_ml_source1_nominal_field8048.clone(), source_ml_source1_nominal_field8049.clone(), source_ml_source1_nominal_field8050, properties.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field8053, source_ml_source1_nominal_field8054, source_ml_source1_nominal_field8055, source_ml_source1_nominal_field8056, _, source_ml_source1_nominal_field8058, source_ml_source1_nominal_field8059) => {
                                                                        let source_ml_source1_nominal_field8054 = *source_ml_source1_nominal_field8054;
                                                                        let source_ml_source1_nominal_field8058 = *source_ml_source1_nominal_field8058;
                                                                        let source_ml_source1_nominal_field8059 = *source_ml_source1_nominal_field8059;
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field8053.clone(), Box::new(source_ml_source1_nominal_field8054.clone()), source_ml_source1_nominal_field8055, source_ml_source1_nominal_field8056, properties.clone(), Box::new(source_ml_source1_nominal_field8058.clone()), Box::new(source_ml_source1_nominal_field8059.clone()))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field8061, source_ml_source1_nominal_field8062, source_ml_source1_nominal_field8063, source_ml_source1_nominal_field8064, _, source_ml_source1_nominal_field8066, source_ml_source1_nominal_field8067) => {
                                                                        let source_ml_source1_nominal_field8062 = *source_ml_source1_nominal_field8062;
                                                                        let source_ml_source1_nominal_field8066 = *source_ml_source1_nominal_field8066;
                                                                        let source_ml_source1_nominal_field8067 = *source_ml_source1_nominal_field8067;
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field8061.clone(), Box::new(source_ml_source1_nominal_field8062.clone()), source_ml_source1_nominal_field8063, source_ml_source1_nominal_field8064, properties.clone(), Box::new(source_ml_source1_nominal_field8066.clone()), Box::new(source_ml_source1_nominal_field8067.clone()))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field8069, source_ml_source1_nominal_field8070, source_ml_source1_nominal_field8071, source_ml_source1_nominal_field8072, _, source_ml_source1_nominal_field8074, source_ml_source1_nominal_field8075) => {
                                                                        let source_ml_source1_nominal_field8070 = *source_ml_source1_nominal_field8070;
                                                                        let source_ml_source1_nominal_field8074 = *source_ml_source1_nominal_field8074;
                                                                        let source_ml_source1_nominal_field8075 = *source_ml_source1_nominal_field8075;
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field8069.clone(), Box::new(source_ml_source1_nominal_field8070.clone()), source_ml_source1_nominal_field8071, source_ml_source1_nominal_field8072, properties.clone(), Box::new(source_ml_source1_nominal_field8074.clone()), Box::new(source_ml_source1_nominal_field8075.clone()))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field8077, source_ml_source1_nominal_field8078, source_ml_source1_nominal_field8079, source_ml_source1_nominal_field8080, _, source_ml_source1_nominal_field8082, source_ml_source1_nominal_field8083) => {
                                                                        let source_ml_source1_nominal_field8078 = *source_ml_source1_nominal_field8078;
                                                                        let source_ml_source1_nominal_field8082 = *source_ml_source1_nominal_field8082;
                                                                        let source_ml_source1_nominal_field8083 = *source_ml_source1_nominal_field8083;
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field8077.clone(), Box::new(source_ml_source1_nominal_field8078.clone()), source_ml_source1_nominal_field8079, source_ml_source1_nominal_field8080, properties.clone(), Box::new(source_ml_source1_nominal_field8082.clone()), Box::new(source_ml_source1_nominal_field8083.clone()))
                                                                    }
                                                                };
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_field8085 = self::replace_java_script_ordinary_object(heap.clone(), replacement);
                                                            {
                                                                let source_ml_source1_field8088 =
                                                                    ({
                                                                        transition.borrow().clone()
                                                                    })
                                                                    .3;
                                                                (self::ml_string::Text::from_units(vec![]), source_ml_source1_field8085, None::<(Option<bool>,)>, true, source_ml_source1_field8088)
                                                            }
                                                        }))
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
