// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
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
use super::source_ml_source1_temporary_commit_java_script_ordinary_own_property_loop122;
use super::source_ml_source3_record_order;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn commit_java_script_ordinary_own_property(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
    key: self::JavaScriptSemanticValue,
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
) -> std::rc::Rc<
    std::cell::RefCell<(
        (
            self::ml_string::Text,
            self::JavaScriptOrdinaryHeap,
            Option<(Option<bool>,)>,
            bool,
            std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
        ),
        Vec<f64>,
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
                    source_ml_source2_data_property_value1730,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value1730.clone(),
                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value1731,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value1731.clone(),
                self::JavaScriptOrdinaryObject::ArrayObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value1732,
                ) => source_ml_source2_data_property_value1732.clone(),
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value1733,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value1733.clone(),
                self::JavaScriptOrdinaryObject::MapObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value1734,
                    _,
                ) => source_ml_source2_data_property_value1734.clone(),
                self::JavaScriptOrdinaryObject::SetObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value1735,
                    _,
                ) => source_ml_source2_data_property_value1735.clone(),
                self::JavaScriptOrdinaryObject::WeakMapObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value1736,
                    _,
                ) => source_ml_source2_data_property_value1736.clone(),
                self::JavaScriptOrdinaryObject::WeakSetObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value1737,
                    _,
                ) => source_ml_source2_data_property_value1737.clone(),
                self::JavaScriptOrdinaryObject::OrdinaryObject(
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value1738,
                ) => source_ml_source2_data_property_value1738.clone(),
                self::JavaScriptOrdinaryObject::HostErrorObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value1739,
                    _,
                    _,
                ) => source_ml_source2_data_property_value1739.clone(),
                self::JavaScriptOrdinaryObject::HostURLObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value1740,
                    _,
                    _,
                ) => source_ml_source2_data_property_value1740.clone(),
                self::JavaScriptOrdinaryObject::HostArrayObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value1741,
                    _,
                    _,
                ) => source_ml_source2_data_property_value1741.clone(),
                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value1742,
                    _,
                    _,
                ) => source_ml_source2_data_property_value1742.clone(),
            },
            key.clone(),
        );
        {
            let property = {
                let source_ml_source2_array_read_value1756 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1743,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value1743.clone(),
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1744,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value1744.clone(),
                    self::JavaScriptOrdinaryObject::ArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1745,
                    ) => source_ml_source2_data_property_value1745.clone(),
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1746,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value1746.clone(),
                    self::JavaScriptOrdinaryObject::MapObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1747,
                        _,
                    ) => source_ml_source2_data_property_value1747.clone(),
                    self::JavaScriptOrdinaryObject::SetObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1748,
                        _,
                    ) => source_ml_source2_data_property_value1748.clone(),
                    self::JavaScriptOrdinaryObject::WeakMapObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1749,
                        _,
                    ) => source_ml_source2_data_property_value1749.clone(),
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1750,
                        _,
                    ) => source_ml_source2_data_property_value1750.clone(),
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1751,
                    ) => source_ml_source2_data_property_value1751.clone(),
                    self::JavaScriptOrdinaryObject::HostErrorObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1752,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value1752.clone(),
                    self::JavaScriptOrdinaryObject::HostURLObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1753,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value1753.clone(),
                    self::JavaScriptOrdinaryObject::HostArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1754,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value1754.clone(),
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1755,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value1755.clone(),
                };
                {
                    let ml_reference_array231 = source_ml_source2_array_read_value1756.clone();
                    let ml_reference_position231 = at;
                    let ml_reference_projection231 = self::ml_array::optional_at(
                        &*ml_reference_array231.borrow(),
                        self::ml_array::number_index(ml_reference_position231),
                    );
                    ml_reference_projection231
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
                            source_ml_source2_data_property_value1758,
                        ) => source_ml_source2_data_property_value1758.clone(),
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
                                source_ml_source2_data_property_value1759,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value1759,
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                source_ml_source2_data_property_value1760,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value1760,
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                source_ml_source2_data_property_value1761,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value1761,
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                source_ml_source2_data_property_value1762,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value1762,
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                source_ml_source2_data_property_value1763,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value1763,
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                source_ml_source2_data_property_value1764,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value1764,
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                source_ml_source2_data_property_value1765,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value1765,
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                source_ml_source2_data_property_value1766,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value1766,
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                source_ml_source2_data_property_value1767,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value1767,
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                source_ml_source2_data_property_value1768,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value1768,
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                source_ml_source2_data_property_value1769,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value1769,
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                source_ml_source2_data_property_value1770,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value1770,
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                source_ml_source2_data_property_value1771,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value1771,
                        },
                        current.clone(),
                        proposed.clone(),
                    );
                    if (((({
                        let ml_reference_value232 = transition.borrow().clone();
                        ml_reference_value232
                    })
                    .0)
                        .1)
                        == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])))
                    {
                        std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_field1774 = (({
                                let ml_reference_value233 = transition.borrow().clone();
                                ml_reference_value233
                            })
                            .0)
                                .2;
                            {
                                let source_ml_source2_field1775 =
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                        self::ml_string::Text,
                                    >::new(
                                    )));
                                {
                                    let source_ml_source3_record_field147 =
                                        source_ml_source2_field1774.clone();
                                    {
                                        let source_ml_source3_record_field148 = heap.clone();
                                        {
                                            let source_ml_source3_record_field149 =
                                                None::<(Option<bool>,)>;
                                            {
                                                let source_ml_source3_record_field150 = false;
                                                {
                                                    let source_ml_source3_record_field151 =
                                                        source_ml_source2_field1775.clone();
                                                    ((source_ml_source3_record_field147.clone(), source_ml_source3_record_field148.clone(), source_ml_source3_record_field149.clone(), source_ml_source3_record_field150, source_ml_source3_record_field151.clone()), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64, 76f64, 105f64, 109f64, 105f64, 116f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64], vec![119f64, 114f64, 105f64, 116f64, 101f64, 115f64]], vec![1f64, 3f64, 0f64, 4f64], vec![true, true, match source_ml_source3_record_field149.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true]))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }))
                    } else {
                        if (((({
                            let ml_reference_value234 = transition.borrow().clone();
                            ml_reference_value234
                        })
                        .0)
                            .1)
                            == (self::ml_string::Text::from_units(vec![107, 101, 101, 112])))
                        {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_field1779 =
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                        self::ml_string::Text,
                                    >::new(
                                    )));
                                {
                                    let source_ml_source3_record_field152 =
                                        self::ml_string::Text::from_units(vec![]);
                                    {
                                        let source_ml_source3_record_field153 = heap.clone();
                                        {
                                            let source_ml_source3_record_field154 =
                                                None::<(Option<bool>,)>;
                                            {
                                                let source_ml_source3_record_field155 = true;
                                                {
                                                    let source_ml_source3_record_field156 =
                                                        source_ml_source2_field1779.clone();
                                                    ((source_ml_source3_record_field152.clone(), source_ml_source3_record_field153.clone(), source_ml_source3_record_field154.clone(), source_ml_source3_record_field155, source_ml_source3_record_field156.clone()), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64, 76f64, 105f64, 109f64, 105f64, 116f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64], vec![119f64, 114f64, 105f64, 116f64, 101f64, 115f64]], vec![1f64, 3f64, 0f64, 4f64], vec![true, true, match source_ml_source3_record_field154.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true]))
                                                }
                                            }
                                        }
                                    }
                                }
                            }))
                        } else {
                            if ((at < (0f64))
                                && (({
                                    let ml_reference_array235 = match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value1793, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1793.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value1794, _, _, _) => {
                                    source_ml_source2_data_property_value1794.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value1795) => {
                                    source_ml_source2_data_property_value1795.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value1796, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1796.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value1797, _) => {
                                    source_ml_source2_data_property_value1797.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value1798, _) => {
                                    source_ml_source2_data_property_value1798.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value1799, _) => {
                                    source_ml_source2_data_property_value1799.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value1800, _) => {
                                    source_ml_source2_data_property_value1800.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value1801) => {
                                    source_ml_source2_data_property_value1801.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value1802, _, _) => {
                                    source_ml_source2_data_property_value1802.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value1803, _, _) => {
                                    source_ml_source2_data_property_value1803.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value1804, _, _) => {
                                    source_ml_source2_data_property_value1804.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value1805, _, _) => {
                                    source_ml_source2_data_property_value1805.clone()
                                }
                            };
                                    let ml_reference_projection235 =
                                        ml_reference_array235.borrow().len() as f64;
                                    ml_reference_projection235
                                }) >= (4096f64)))
                            {
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_field1809 =
                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                            self::ml_string::Text,
                                        >::new(
                                        )));
                                    {
                                        let source_ml_source2_field1810 = Some((Some(true),));
                                        {
                                            let source_ml_source3_record_field157 =
                                                self::ml_string::Text::from_units(vec![
                                                    111, 114, 100, 105, 110, 97, 114, 121, 32, 112,
                                                    114, 111, 112, 101, 114, 116, 105, 101, 115,
                                                    32, 101, 120, 99, 101, 101, 100, 32, 116, 104,
                                                    101, 105, 114, 32, 101, 120, 112, 108, 105, 99,
                                                    105, 116, 32, 111, 114, 100, 101, 114, 101,
                                                    100, 32, 115, 116, 111, 114, 97, 103, 101, 32,
                                                    98, 117, 100, 103, 101, 116,
                                                ]);
                                            {
                                                let source_ml_source3_record_field158 =
                                                    heap.clone();
                                                {
                                                    let source_ml_source3_record_field159 =
                                                        source_ml_source2_field1810.clone();
                                                    {
                                                        let source_ml_source3_record_field160 =
                                                            false;
                                                        {
                                                            let source_ml_source3_record_field161 =
                                                                source_ml_source2_field1809.clone();
                                                            ((source_ml_source3_record_field157.clone(), source_ml_source3_record_field158.clone(), source_ml_source3_record_field159.clone(), source_ml_source3_record_field160, source_ml_source3_record_field161.clone()), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64, 76f64, 105f64, 109f64, 105f64, 116f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64], vec![119f64, 114f64, 105f64, 116f64, 101f64, 115f64]], vec![1f64, 3f64, 0f64, 4f64, 2f64], vec![true, true, match source_ml_source3_record_field159.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true]))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }))
                            } else {
                                {
                                    let count = 0f64;
                                    {
                                        let source_ml_source1_temporary_values121 = match heap
                                            .clone()
                                        {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                source_ml_source2_data_property_value1811,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source2_data_property_value1811.clone(),
                                        };
                                        {
                                            let source_ml_source1_temporary_index121 =
                                                self::ml::Big::from_i128(0);
                                            {
                                                let count_2 = self::source_ml_source1_temporary_commit_java_script_ordinary_own_property_loop122(count, source_ml_source1_temporary_values121.clone(), source_ml_source1_temporary_index121.clone());
                                                if ((at < (0f64)) && (count_2 >= (16384f64))) {
                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                        let source_ml_source2_field1815 =
                                                            std::rc::Rc::new(
                                                                std::cell::RefCell::new(Vec::<
                                                                    self::ml_string::Text,
                                                                >::new(
                                                                )),
                                                            );
                                                        {
                                                            let source_ml_source2_field1816 =
                                                                Some((Some(true),));
                                                            {
                                                                let source_ml_source3_record_field162 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]);
                                                                {
                                                                    let source_ml_source3_record_field163 =
                                                                        heap.clone();
                                                                    {
                                                                        let source_ml_source3_record_field164 = source_ml_source2_field1816.clone();
                                                                        {
                                                                            let source_ml_source3_record_field165 =
                                                                                false;
                                                                            {
                                                                                let source_ml_source3_record_field166 = source_ml_source2_field1815.clone();
                                                                                ((source_ml_source3_record_field162.clone(), source_ml_source3_record_field163.clone(), source_ml_source3_record_field164.clone(), source_ml_source3_record_field165, source_ml_source3_record_field166.clone()), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64, 76f64, 105f64, 109f64, 105f64, 116f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64], vec![119f64, 114f64, 105f64, 116f64, 101f64, 115f64]], vec![1f64, 3f64, 0f64, 4f64, 2f64], vec![true, true, match source_ml_source3_record_field164.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true]))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }))
                                                } else {
                                                    {
                                                        let updated = self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(key.clone()), (({ let ml_reference_value236 = transition.borrow().clone(); ml_reference_value236 }).0).0);
                                                        {
                                                            let properties = if (at < (0f64)) {
                                                                std::rc::Rc::new(
                                                                    std::cell::RefCell::new({
                                                                        let source_ml_source2_concat_value1830 = match object.clone() {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value1817, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value1817.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value1818, _, _, _) => {
                                                                            source_ml_source2_data_property_value1818.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value1819) => {
                                                                            source_ml_source2_data_property_value1819.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value1820, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value1820.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value1821, _) => {
                                                                            source_ml_source2_data_property_value1821.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value1822, _) => {
                                                                            source_ml_source2_data_property_value1822.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value1823, _) => {
                                                                            source_ml_source2_data_property_value1823.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value1824, _) => {
                                                                            source_ml_source2_data_property_value1824.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value1825) => {
                                                                            source_ml_source2_data_property_value1825.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value1826, _, _) => {
                                                                            source_ml_source2_data_property_value1826.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value1827, _, _) => {
                                                                            source_ml_source2_data_property_value1827.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value1828, _, _) => {
                                                                            source_ml_source2_data_property_value1828.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value1829, _, _) => {
                                                                            source_ml_source2_data_property_value1829.clone()
                                                                        }
                                                                    };
                                                                        {
                                                                            let source_ml_source2_concat_value1831 = std::rc::Rc::new(std::cell::RefCell::new(vec![updated.clone()]));
                                                                            self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryProperty>::new(), { let ml_reference_value237 = source_ml_source2_concat_value1830.borrow().clone(); ml_reference_value237 }), { let ml_reference_value238 = source_ml_source2_concat_value1831.borrow().clone(); ml_reference_value238 })
                                                                        }
                                                                    }),
                                                                )
                                                            } else {
                                                                std::rc::Rc::new(
                                                                    std::cell::RefCell::new({
                                                                        let ml_values239 = {
                                                                            let ml_reference_value240 = (match object.clone() {
                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value1832, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value1832.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value1833, _, _, _) => {
                                                                        source_ml_source2_data_property_value1833.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value1834) => {
                                                                        source_ml_source2_data_property_value1834.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value1835, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value1835.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value1836, _) => {
                                                                        source_ml_source2_data_property_value1836.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value1837, _) => {
                                                                        source_ml_source2_data_property_value1837.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value1838, _) => {
                                                                        source_ml_source2_data_property_value1838.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value1839, _) => {
                                                                        source_ml_source2_data_property_value1839.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value1840) => {
                                                                        source_ml_source2_data_property_value1840.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value1841, _, _) => {
                                                                        source_ml_source2_data_property_value1841.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value1842, _, _) => {
                                                                        source_ml_source2_data_property_value1842.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value1843, _, _) => {
                                                                        source_ml_source2_data_property_value1843.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value1844, _, _) => {
                                                                        source_ml_source2_data_property_value1844.clone()
                                                                    }
                                                                }).borrow().clone();
                                                                            ml_reference_value240
                                                                        };
                                                                        let ml_callback239 = {
                                                                            let at = at;
                                                                            let updated =
                                                                                updated.clone();
                                                                            std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> self::JavaScriptOrdinaryProperty { if (position == at) {
                                                                    updated.clone()
                                                                } else {
                                                                    entry.clone()
                                                                } })
                                                                        };
                                                                        ml_values239.iter().cloned().enumerate().map(|(ml_index239, ml_value239)| ml_callback239(ml_value239, (ml_index239 as f64))).collect::<Vec<_>>()
                                                                    }),
                                                                )
                                                            };
                                                            {
                                                                let replacement = match object.clone() {
                                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field1845, source_ml_source2_nominal_field1846, source_ml_source2_nominal_field1847, source_ml_source2_nominal_field1848, _, source_ml_source2_nominal_field1850, source_ml_source2_nominal_field1851, source_ml_source2_nominal_field1852, source_ml_source2_nominal_field1853, source_ml_source2_nominal_field1854) => {
                                                                        let source_ml_source2_nominal_field1846 = *source_ml_source2_nominal_field1846;
                                                                        let source_ml_source2_nominal_field1853 = *source_ml_source2_nominal_field1853;
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field1845.clone(), Box::new(source_ml_source2_nominal_field1846.clone()), source_ml_source2_nominal_field1847, source_ml_source2_nominal_field1848, properties.clone(), source_ml_source2_nominal_field1850.clone(), source_ml_source2_nominal_field1851, source_ml_source2_nominal_field1852.clone(), Box::new(source_ml_source2_nominal_field1853.clone()), source_ml_source2_nominal_field1854.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field1856, source_ml_source2_nominal_field1857, source_ml_source2_nominal_field1858, source_ml_source2_nominal_field1859, _, source_ml_source2_nominal_field1861, source_ml_source2_nominal_field1862, source_ml_source2_nominal_field1863) => {
                                                                        let source_ml_source2_nominal_field1857 = *source_ml_source2_nominal_field1857;
                                                                        let source_ml_source2_nominal_field1861 = *source_ml_source2_nominal_field1861;
                                                                        let source_ml_source2_nominal_field1862 = *source_ml_source2_nominal_field1862;
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field1856.clone(), Box::new(source_ml_source2_nominal_field1857.clone()), source_ml_source2_nominal_field1858, source_ml_source2_nominal_field1859, properties.clone(), Box::new(source_ml_source2_nominal_field1861.clone()), Box::new(source_ml_source2_nominal_field1862.clone()), source_ml_source2_nominal_field1863.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field1865, source_ml_source2_nominal_field1866, source_ml_source2_nominal_field1867, source_ml_source2_nominal_field1868, _) => {
                                                                        let source_ml_source2_nominal_field1866 = *source_ml_source2_nominal_field1866;
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field1865.clone(), Box::new(source_ml_source2_nominal_field1866.clone()), source_ml_source2_nominal_field1867, source_ml_source2_nominal_field1868, properties.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field1871, source_ml_source2_nominal_field1872, source_ml_source2_nominal_field1873, source_ml_source2_nominal_field1874, _, source_ml_source2_nominal_field1876, source_ml_source2_nominal_field1877, source_ml_source2_nominal_field1878, source_ml_source2_nominal_field1879, source_ml_source2_nominal_field1880, source_ml_source2_nominal_field1881) => {
                                                                        let source_ml_source2_nominal_field1872 = *source_ml_source2_nominal_field1872;
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field1871.clone(), Box::new(source_ml_source2_nominal_field1872.clone()), source_ml_source2_nominal_field1873, source_ml_source2_nominal_field1874, properties.clone(), source_ml_source2_nominal_field1876.clone(), source_ml_source2_nominal_field1877, source_ml_source2_nominal_field1878.clone(), source_ml_source2_nominal_field1879.clone(), source_ml_source2_nominal_field1880.clone(), source_ml_source2_nominal_field1881.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field1883, source_ml_source2_nominal_field1884, source_ml_source2_nominal_field1885, source_ml_source2_nominal_field1886, _, source_ml_source2_nominal_field1888) => {
                                                                        let source_ml_source2_nominal_field1884 = *source_ml_source2_nominal_field1884;
                                                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field1883.clone(), Box::new(source_ml_source2_nominal_field1884.clone()), source_ml_source2_nominal_field1885, source_ml_source2_nominal_field1886, properties.clone(), source_ml_source2_nominal_field1888.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field1890, source_ml_source2_nominal_field1891, source_ml_source2_nominal_field1892, source_ml_source2_nominal_field1893, _, source_ml_source2_nominal_field1895) => {
                                                                        let source_ml_source2_nominal_field1891 = *source_ml_source2_nominal_field1891;
                                                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field1890.clone(), Box::new(source_ml_source2_nominal_field1891.clone()), source_ml_source2_nominal_field1892, source_ml_source2_nominal_field1893, properties.clone(), source_ml_source2_nominal_field1895.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field1897, source_ml_source2_nominal_field1898, source_ml_source2_nominal_field1899, source_ml_source2_nominal_field1900, _, source_ml_source2_nominal_field1902) => {
                                                                        let source_ml_source2_nominal_field1898 = *source_ml_source2_nominal_field1898;
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field1897.clone(), Box::new(source_ml_source2_nominal_field1898.clone()), source_ml_source2_nominal_field1899, source_ml_source2_nominal_field1900, properties.clone(), source_ml_source2_nominal_field1902.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field1904, source_ml_source2_nominal_field1905, source_ml_source2_nominal_field1906, source_ml_source2_nominal_field1907, _, source_ml_source2_nominal_field1909) => {
                                                                        let source_ml_source2_nominal_field1905 = *source_ml_source2_nominal_field1905;
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field1904.clone(), Box::new(source_ml_source2_nominal_field1905.clone()), source_ml_source2_nominal_field1906, source_ml_source2_nominal_field1907, properties.clone(), source_ml_source2_nominal_field1909.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field1911, source_ml_source2_nominal_field1912, source_ml_source2_nominal_field1913, source_ml_source2_nominal_field1914, source_ml_source2_nominal_field1915, source_ml_source2_nominal_field1916, source_ml_source2_nominal_field1917, source_ml_source2_nominal_field1918, _) => {
                                                                        let source_ml_source2_nominal_field1912 = *source_ml_source2_nominal_field1912;
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field1911.clone(), Box::new(source_ml_source2_nominal_field1912.clone()), source_ml_source2_nominal_field1913, source_ml_source2_nominal_field1914.clone(), source_ml_source2_nominal_field1915.clone(), source_ml_source2_nominal_field1916.clone(), source_ml_source2_nominal_field1917.clone(), source_ml_source2_nominal_field1918, properties.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field1921, source_ml_source2_nominal_field1922, source_ml_source2_nominal_field1923, source_ml_source2_nominal_field1924, _, source_ml_source2_nominal_field1926, source_ml_source2_nominal_field1927) => {
                                                                        let source_ml_source2_nominal_field1922 = *source_ml_source2_nominal_field1922;
                                                                        let source_ml_source2_nominal_field1926 = *source_ml_source2_nominal_field1926;
                                                                        let source_ml_source2_nominal_field1927 = *source_ml_source2_nominal_field1927;
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field1921.clone(), Box::new(source_ml_source2_nominal_field1922.clone()), source_ml_source2_nominal_field1923, source_ml_source2_nominal_field1924, properties.clone(), Box::new(source_ml_source2_nominal_field1926.clone()), Box::new(source_ml_source2_nominal_field1927.clone()))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field1929, source_ml_source2_nominal_field1930, source_ml_source2_nominal_field1931, source_ml_source2_nominal_field1932, _, source_ml_source2_nominal_field1934, source_ml_source2_nominal_field1935) => {
                                                                        let source_ml_source2_nominal_field1930 = *source_ml_source2_nominal_field1930;
                                                                        let source_ml_source2_nominal_field1934 = *source_ml_source2_nominal_field1934;
                                                                        let source_ml_source2_nominal_field1935 = *source_ml_source2_nominal_field1935;
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field1929.clone(), Box::new(source_ml_source2_nominal_field1930.clone()), source_ml_source2_nominal_field1931, source_ml_source2_nominal_field1932, properties.clone(), Box::new(source_ml_source2_nominal_field1934.clone()), Box::new(source_ml_source2_nominal_field1935.clone()))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field1937, source_ml_source2_nominal_field1938, source_ml_source2_nominal_field1939, source_ml_source2_nominal_field1940, _, source_ml_source2_nominal_field1942, source_ml_source2_nominal_field1943) => {
                                                                        let source_ml_source2_nominal_field1938 = *source_ml_source2_nominal_field1938;
                                                                        let source_ml_source2_nominal_field1942 = *source_ml_source2_nominal_field1942;
                                                                        let source_ml_source2_nominal_field1943 = *source_ml_source2_nominal_field1943;
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field1937.clone(), Box::new(source_ml_source2_nominal_field1938.clone()), source_ml_source2_nominal_field1939, source_ml_source2_nominal_field1940, properties.clone(), Box::new(source_ml_source2_nominal_field1942.clone()), Box::new(source_ml_source2_nominal_field1943.clone()))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field1945, source_ml_source2_nominal_field1946, source_ml_source2_nominal_field1947, source_ml_source2_nominal_field1948, _, source_ml_source2_nominal_field1950, source_ml_source2_nominal_field1951) => {
                                                                        let source_ml_source2_nominal_field1946 = *source_ml_source2_nominal_field1946;
                                                                        let source_ml_source2_nominal_field1950 = *source_ml_source2_nominal_field1950;
                                                                        let source_ml_source2_nominal_field1951 = *source_ml_source2_nominal_field1951;
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field1945.clone(), Box::new(source_ml_source2_nominal_field1946.clone()), source_ml_source2_nominal_field1947, source_ml_source2_nominal_field1948, properties.clone(), Box::new(source_ml_source2_nominal_field1950.clone()), Box::new(source_ml_source2_nominal_field1951.clone()))
                                                                    }
                                                                };
                                                                std::rc::Rc::new(
                                                                    std::cell::RefCell::new({
                                                                        let source_ml_source2_field1953 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                        {
                                                                            let source_ml_source2_field1956 = (({ let ml_reference_value241 = transition.borrow().clone(); ml_reference_value241 }).0).3;
                                                                            {
                                                                                let source_ml_source3_record_field167 = self::ml_string::Text::from_units(vec![]);
                                                                                {
                                                                                    let source_ml_source3_record_field168 = source_ml_source2_field1953.clone();
                                                                                    {
                                                                                        let source_ml_source3_record_field169 = None::<(Option<bool>,)>;
                                                                                        {
                                                                                            let source_ml_source3_record_field170 =
                                                                                                true;
                                                                                            {
                                                                                                let source_ml_source3_record_field171 = source_ml_source2_field1956.clone();
                                                                                                ((source_ml_source3_record_field167.clone(), source_ml_source3_record_field168.clone(), source_ml_source3_record_field169.clone(), source_ml_source3_record_field170, source_ml_source3_record_field171.clone()), self::source_ml_source3_record_order::create_java_script_present_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![104f64, 101f64, 97f64, 112f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64, 76f64, 105f64, 109f64, 105f64, 116f64], vec![115f64, 117f64, 99f64, 99f64, 101f64, 115f64, 115f64], vec![119f64, 114f64, 105f64, 116f64, 101f64, 115f64]], vec![1f64, 3f64, 0f64, 4f64], vec![true, true, match source_ml_source3_record_field169.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }, true, true]))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }),
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
                    }
                }
            }
        }
    }
}
