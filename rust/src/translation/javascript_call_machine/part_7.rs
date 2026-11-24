// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptSemanticValue;
use super::ml_string;
use super::read_java_script_property_descriptor_kind;
use super::read_java_script_semantic_value_error;
use super::source_ml_source3_record_order;
use super::test_java_script_semantic_accessor;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_property_descriptors_same_value(
    left: std::rc::Rc<
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
    right: std::rc::Rc<
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
) -> bool {
    !((((((((((((((({
        let ml_reference_value12 = left.borrow().clone();
        ml_reference_value12
    })
    .0)
        .1)
        != ((({
            let ml_reference_value13 = right.borrow().clone();
            ml_reference_value13
        })
        .0)
            .1))
        || ((({
            let ml_reference_value14 = left.borrow().clone();
            ml_reference_value14
        })
        .0)
            .1
            && (((({
                let ml_reference_value15 = left.borrow().clone();
                ml_reference_value15
            })
            .0)
                .0)
                != ((({
                    let ml_reference_value16 = right.borrow().clone();
                    ml_reference_value16
                })
                .0)
                    .0))))
        || (((({
            let ml_reference_value17 = left.borrow().clone();
            ml_reference_value17
        })
        .0)
            .3)
            != ((({
                let ml_reference_value18 = right.borrow().clone();
                ml_reference_value18
            })
            .0)
                .3)))
        || ((({
            let ml_reference_value19 = left.borrow().clone();
            ml_reference_value19
        })
        .0)
            .3
            && (((({
                let ml_reference_value20 = left.borrow().clone();
                ml_reference_value20
            })
            .0)
                .2)
                != ((({
                    let ml_reference_value21 = right.borrow().clone();
                    ml_reference_value21
                })
                .0)
                    .2))))
        || (((({
            let ml_reference_value22 = left.borrow().clone();
            ml_reference_value22
        })
        .0)
            .11)
            != ((({
                let ml_reference_value23 = right.borrow().clone();
                ml_reference_value23
            })
            .0)
                .11)))
        || ((({
            let ml_reference_value24 = left.borrow().clone();
            ml_reference_value24
        })
        .0)
            .11
            && (((({
                let ml_reference_value25 = left.borrow().clone();
                ml_reference_value25
            })
            .0)
                .10)
                != ((({
                    let ml_reference_value26 = right.borrow().clone();
                    ml_reference_value26
                })
                .0)
                    .10))))
        || (((({
            let ml_reference_value27 = left.borrow().clone();
            ml_reference_value27
        })
        .0)
            .9)
            != ((({
                let ml_reference_value28 = right.borrow().clone();
                ml_reference_value28
            })
            .0)
                .9)))
        || ((({
            let ml_reference_value29 = left.borrow().clone();
            ml_reference_value29
        })
        .0)
            .9
            && !(self::test_java_script_semantic_same_value(
                (({
                    let ml_reference_value30 = left.borrow().clone();
                    ml_reference_value30
                })
                .0)
                    .8,
                (({
                    let ml_reference_value31 = right.borrow().clone();
                    ml_reference_value31
                })
                .0)
                    .8,
            ))))
        || (((({
            let ml_reference_value32 = left.borrow().clone();
            ml_reference_value32
        })
        .0)
            .5)
            != ((({
                let ml_reference_value33 = right.borrow().clone();
                ml_reference_value33
            })
            .0)
                .5)))
        || ((({
            let ml_reference_value34 = left.borrow().clone();
            ml_reference_value34
        })
        .0)
            .5
            && !(self::test_java_script_semantic_same_value(
                (({
                    let ml_reference_value35 = left.borrow().clone();
                    ml_reference_value35
                })
                .0)
                    .4,
                (({
                    let ml_reference_value36 = right.borrow().clone();
                    ml_reference_value36
                })
                .0)
                    .4,
            ))))
        || (((({
            let ml_reference_value37 = left.borrow().clone();
            ml_reference_value37
        })
        .0)
            .7)
            != ((({
                let ml_reference_value38 = right.borrow().clone();
                ml_reference_value38
            })
            .0)
                .7)))
        || ((({
            let ml_reference_value39 = left.borrow().clone();
            ml_reference_value39
        })
        .0)
            .7
            && !(self::test_java_script_semantic_same_value(
                (({
                    let ml_reference_value40 = left.borrow().clone();
                    ml_reference_value40
                })
                .0)
                    .6,
                (({
                    let ml_reference_value41 = right.borrow().clone();
                    ml_reference_value41
                })
                .0)
                    .6,
            ))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_descriptor_error(
    descriptor: std::rc::Rc<
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
) -> self::ml_string::Text {
    if ((self::read_java_script_property_descriptor_kind(descriptor.clone()))
        == (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100])))
    {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
    } else {
        if ((({
            let ml_reference_value42 = descriptor.borrow().clone();
            ml_reference_value42
        })
        .0)
            .9
            && ((self::read_java_script_semantic_value_error(
                (({
                    let ml_reference_value43 = descriptor.borrow().clone();
                    ml_reference_value43
                })
                .0)
                    .8,
            )) != (self::ml_string::Text::from_units(vec![]))))
        {
            self::ml_string::Text::from_units(vec![
                73, 110, 118, 97, 108, 105, 100, 83, 101, 109, 97, 110, 116, 105, 99, 86, 97, 108,
                117, 101,
            ])
        } else {
            if ((({
                let ml_reference_value44 = descriptor.borrow().clone();
                ml_reference_value44
            })
            .0)
                .5
                && (((self::read_java_script_semantic_value_error(
                    (({
                        let ml_reference_value45 = descriptor.borrow().clone();
                        ml_reference_value45
                    })
                    .0)
                        .4,
                )) != (self::ml_string::Text::from_units(vec![])))
                    || !(self::test_java_script_semantic_accessor(
                        (({
                            let ml_reference_value46 = descriptor.borrow().clone();
                            ml_reference_value46
                        })
                        .0)
                            .4,
                    ))))
            {
                self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
            } else {
                if ((({
                    let ml_reference_value47 = descriptor.borrow().clone();
                    ml_reference_value47
                })
                .0)
                    .7
                    && (((self::read_java_script_semantic_value_error(
                        (({
                            let ml_reference_value48 = descriptor.borrow().clone();
                            ml_reference_value48
                        })
                        .0)
                            .6,
                    )) != (self::ml_string::Text::from_units(vec![])))
                        || !(self::test_java_script_semantic_accessor(
                            (({
                                let ml_reference_value49 = descriptor.borrow().clone();
                                ml_reference_value49
                            })
                            .0)
                                .6,
                        ))))
                {
                    self::ml_string::Text::from_units(vec![
                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                    ])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_complete_java_script_property_descriptor(
    descriptor: std::rc::Rc<
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
) -> bool {
    if ((!((({
        let ml_reference_value50 = descriptor.borrow().clone();
        ml_reference_value50
    })
    .0)
        .1)
        || !((({
            let ml_reference_value51 = descriptor.borrow().clone();
            ml_reference_value51
        })
        .0)
            .3))
        || ((self::read_java_script_property_descriptor_error(descriptor.clone()))
            != (self::ml_string::Text::from_units(vec![]))))
    {
        false
    } else {
        {
            let kind = self::read_java_script_property_descriptor_kind(descriptor.clone());
            if (kind == (self::ml_string::Text::from_units(vec![100, 97, 116, 97]))) {
                ((({
                    let ml_reference_value52 = descriptor.borrow().clone();
                    ml_reference_value52
                })
                .0)
                    .9
                    && (({
                        let ml_reference_value53 = descriptor.borrow().clone();
                        ml_reference_value53
                    })
                    .0)
                        .11)
            } else {
                (((kind
                    == (self::ml_string::Text::from_units(vec![
                        97, 99, 99, 101, 115, 115, 111, 114,
                    ])))
                    && (({
                        let ml_reference_value54 = descriptor.borrow().clone();
                        ml_reference_value54
                    })
                    .0)
                        .5)
                    && (({
                        let ml_reference_value55 = descriptor.borrow().clone();
                        ml_reference_value55
                    })
                    .0)
                        .7)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_property_descriptor(
    kind: &self::ml_string::Text,
    configurable: bool,
    enumerable: bool,
) -> std::rc::Rc<
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
> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source2_field54 = ((*kind)
            != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
        {
            let source_ml_source2_field55 = self::JavaScriptSemanticValue::Undefined;
            {
                let source_ml_source2_field56 = ((*kind)
                    != (self::ml_string::Text::from_units(vec![
                        97, 99, 99, 101, 115, 115, 111, 114,
                    ])));
                {
                    let source_ml_source2_field58 = ((*kind)
                        == (self::ml_string::Text::from_units(vec![
                            97, 99, 99, 101, 115, 115, 111, 114,
                        ])));
                    {
                        let source_ml_source2_field59 = self::JavaScriptSemanticValue::Undefined;
                        {
                            let source_ml_source2_field60 = ((*kind)
                                == (self::ml_string::Text::from_units(vec![
                                    97, 99, 99, 101, 115, 115, 111, 114,
                                ])));
                            {
                                let source_ml_source2_field61 =
                                    self::JavaScriptSemanticValue::Undefined;
                                {
                                    let source_ml_source3_record_field1 = configurable;
                                    {
                                        let source_ml_source3_record_field2 = true;
                                        {
                                            let source_ml_source3_record_field3 = enumerable;
                                            {
                                                let source_ml_source3_record_field4 = true;
                                                {
                                                    let source_ml_source3_record_field5 =
                                                        source_ml_source2_field59.clone();
                                                    {
                                                        let source_ml_source3_record_field6 =
                                                            source_ml_source2_field58;
                                                        {
                                                            let source_ml_source3_record_field7 =
                                                                source_ml_source2_field61.clone();
                                                            {
                                                                let source_ml_source3_record_field8 =
                                                                    source_ml_source2_field60;
                                                                {
                                                                    let source_ml_source3_record_field9 =
                                                                        source_ml_source2_field55
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source3_record_field10 = source_ml_source2_field54;
                                                                        {
                                                                            let source_ml_source3_record_field11 =
                                                                                false;
                                                                            {
                                                                                let source_ml_source3_record_field12 = source_ml_source2_field56;
                                                                                ((source_ml_source3_record_field1, source_ml_source3_record_field2, source_ml_source3_record_field3, source_ml_source3_record_field4, source_ml_source3_record_field5.clone(), source_ml_source3_record_field6, source_ml_source3_record_field7.clone(), source_ml_source3_record_field8, source_ml_source3_record_field9.clone(), source_ml_source3_record_field10, source_ml_source3_record_field11, source_ml_source3_record_field12), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64, 3f64, 2f64, 9f64, 8f64, 11f64, 10f64, 5f64, 4f64, 7f64, 6f64]))
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
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_property_descriptor_fields(
    base: std::rc::Rc<
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
> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source2_field63 = if (({
            let ml_reference_value56 = proposed.borrow().clone();
            ml_reference_value56
        })
        .0)
            .1
        {
            (({
                let ml_reference_value57 = proposed.borrow().clone();
                ml_reference_value57
            })
            .0)
                .0
        } else {
            (({
                let ml_reference_value58 = base.borrow().clone();
                ml_reference_value58
            })
            .0)
                .0
        };
        {
            let source_ml_source2_field65 = if (({
                let ml_reference_value59 = proposed.borrow().clone();
                ml_reference_value59
            })
            .0)
                .3
            {
                (({
                    let ml_reference_value60 = proposed.borrow().clone();
                    ml_reference_value60
                })
                .0)
                    .2
            } else {
                (({
                    let ml_reference_value61 = base.borrow().clone();
                    ml_reference_value61
                })
                .0)
                    .2
            };
            {
                let source_ml_source2_field66 = (({
                    let ml_reference_value62 = base.borrow().clone();
                    ml_reference_value62
                })
                .0)
                    .9;
                {
                    let source_ml_source2_field67 = if ((({
                        let ml_reference_value63 = base.borrow().clone();
                        ml_reference_value63
                    })
                    .0)
                        .9
                        && (({
                            let ml_reference_value64 = proposed.borrow().clone();
                            ml_reference_value64
                        })
                        .0)
                            .9)
                    {
                        (({
                            let ml_reference_value65 = proposed.borrow().clone();
                            ml_reference_value65
                        })
                        .0)
                            .8
                    } else {
                        (({
                            let ml_reference_value66 = base.borrow().clone();
                            ml_reference_value66
                        })
                        .0)
                            .8
                    };
                    {
                        let source_ml_source2_field68 = (({
                            let ml_reference_value67 = base.borrow().clone();
                            ml_reference_value67
                        })
                        .0)
                            .11;
                        {
                            let source_ml_source2_field69 = if ((({
                                let ml_reference_value68 = base.borrow().clone();
                                ml_reference_value68
                            })
                            .0)
                                .11
                                && (({
                                    let ml_reference_value69 = proposed.borrow().clone();
                                    ml_reference_value69
                                })
                                .0)
                                    .11)
                            {
                                (({
                                    let ml_reference_value70 = proposed.borrow().clone();
                                    ml_reference_value70
                                })
                                .0)
                                    .10
                            } else {
                                (({
                                    let ml_reference_value71 = base.borrow().clone();
                                    ml_reference_value71
                                })
                                .0)
                                    .10
                            };
                            {
                                let source_ml_source2_field70 = (({
                                    let ml_reference_value72 = base.borrow().clone();
                                    ml_reference_value72
                                })
                                .0)
                                    .5;
                                {
                                    let source_ml_source2_field71 = if ((({
                                        let ml_reference_value73 = base.borrow().clone();
                                        ml_reference_value73
                                    })
                                    .0)
                                        .5
                                        && (({
                                            let ml_reference_value74 = proposed.borrow().clone();
                                            ml_reference_value74
                                        })
                                        .0)
                                            .5)
                                    {
                                        (({
                                            let ml_reference_value75 = proposed.borrow().clone();
                                            ml_reference_value75
                                        })
                                        .0)
                                            .4
                                    } else {
                                        (({
                                            let ml_reference_value76 = base.borrow().clone();
                                            ml_reference_value76
                                        })
                                        .0)
                                            .4
                                    };
                                    {
                                        let source_ml_source2_field72 = (({
                                            let ml_reference_value77 = base.borrow().clone();
                                            ml_reference_value77
                                        })
                                        .0)
                                            .7;
                                        {
                                            let source_ml_source2_field73 = if ((({
                                                let ml_reference_value78 = base.borrow().clone();
                                                ml_reference_value78
                                            })
                                            .0)
                                                .7
                                                && (({
                                                    let ml_reference_value79 =
                                                        proposed.borrow().clone();
                                                    ml_reference_value79
                                                })
                                                .0)
                                                    .7)
                                            {
                                                (({
                                                    let ml_reference_value80 =
                                                        proposed.borrow().clone();
                                                    ml_reference_value80
                                                })
                                                .0)
                                                    .6
                                            } else {
                                                (({
                                                    let ml_reference_value81 =
                                                        base.borrow().clone();
                                                    ml_reference_value81
                                                })
                                                .0)
                                                    .6
                                            };
                                            {
                                                let source_ml_source3_record_field13 =
                                                    source_ml_source2_field63;
                                                {
                                                    let source_ml_source3_record_field14 = true;
                                                    {
                                                        let source_ml_source3_record_field15 =
                                                            source_ml_source2_field65;
                                                        {
                                                            let source_ml_source3_record_field16 =
                                                                true;
                                                            {
                                                                let source_ml_source3_record_field17 =
                                                                    source_ml_source2_field71
                                                                        .clone();
                                                                {
                                                                    let source_ml_source3_record_field18 =
                                                                        source_ml_source2_field70;
                                                                    {
                                                                        let source_ml_source3_record_field19 = source_ml_source2_field73.clone();
                                                                        {
                                                                            let source_ml_source3_record_field20 = source_ml_source2_field72;
                                                                            {
                                                                                let source_ml_source3_record_field21 = source_ml_source2_field67.clone();
                                                                                {
                                                                                    let source_ml_source3_record_field22 = source_ml_source2_field66;
                                                                                    {
                                                                                        let source_ml_source3_record_field23 = source_ml_source2_field69;
                                                                                        {
                                                                                            let source_ml_source3_record_field24 = source_ml_source2_field68;
                                                                                            ((source_ml_source3_record_field13, source_ml_source3_record_field14, source_ml_source3_record_field15, source_ml_source3_record_field16, source_ml_source3_record_field17.clone(), source_ml_source3_record_field18, source_ml_source3_record_field19.clone(), source_ml_source3_record_field20, source_ml_source3_record_field21.clone(), source_ml_source3_record_field22, source_ml_source3_record_field23, source_ml_source3_record_field24), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64, 3f64, 2f64, 9f64, 8f64, 11f64, 10f64, 5f64, 4f64, 7f64, 6f64]))
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
}
