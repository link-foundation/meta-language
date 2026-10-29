// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::ml_array;
use super::refuse_java_script_call_source;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn append_java_script_call_source_instruction(builder: self::JavaScriptCallSourceBuilder, instruction: self::JavaScriptCallInstruction) -> self::JavaScriptCallSourceBuilder {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value376) => {
            source_ml_source2_data_property_value376.clone()
        }
    }) != "") {
        builder.clone()
    } else {
        if ((((match &builder { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value378, _, _, _, _, _) => source_ml_source2_data_property_value378.as_slice() }).len() as f64)) >= (65536f64)) {
            {
                let source_ml_source2_nominal_spread379 = builder.clone();
                {
                    let source_ml_source2_nominal_field380 = match source_ml_source2_nominal_spread379.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value381, _, _, _, _, _) => {
                            source_ml_source2_data_property_value381
                        }
                    };
                    {
                        let source_ml_source2_nominal_field382 = match source_ml_source2_nominal_spread379.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value383, _, _, _, _) => {
                                source_ml_source2_data_property_value383
                            }
                        };
                        {
                            let source_ml_source2_nominal_field384 = match source_ml_source2_nominal_spread379.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value385, _, _, _) => {
                                    source_ml_source2_data_property_value385
                                }
                            };
                            {
                                let source_ml_source2_nominal_field386 = match source_ml_source2_nominal_spread379.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value387, _, _) => {
                                        source_ml_source2_data_property_value387
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field388 = match source_ml_source2_nominal_spread379.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value389, _) => {
                                            source_ml_source2_data_property_value389
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field390 = match source_ml_source2_nominal_spread379.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value391) => {
                                                source_ml_source2_data_property_value391.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override392 = String::from("source lowering exceeds its instruction budget");
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field380, source_ml_source2_nominal_field382, source_ml_source2_nominal_field384, source_ml_source2_nominal_field386, source_ml_source2_nominal_field388, source_ml_source2_nominal_override392.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            {
                let source_ml_source2_nominal_spread396 = builder.clone();
                {
                    let source_ml_source2_nominal_field397 = match source_ml_source2_nominal_spread396.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value398, _, _, _, _, _) => {
                            source_ml_source2_data_property_value398
                        }
                    };
                    {
                        let source_ml_source2_nominal_field399 = match source_ml_source2_nominal_spread396.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value400, _, _, _, _) => {
                                source_ml_source2_data_property_value400
                            }
                        };
                        {
                            let source_ml_source2_nominal_field401 = match source_ml_source2_nominal_spread396.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value402, _, _, _) => {
                                    source_ml_source2_data_property_value402
                                }
                            };
                            {
                                let source_ml_source2_nominal_field403 = match source_ml_source2_nominal_spread396.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value404, _, _) => {
                                        source_ml_source2_data_property_value404
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field405 = match source_ml_source2_nominal_spread396.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value406, _) => {
                                            source_ml_source2_data_property_value406
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field407 = match source_ml_source2_nominal_spread396.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value408) => {
                                                source_ml_source2_data_property_value408.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override409 = {
                                                let source_ml_source2_concat_value394 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value393, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value393
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_concat_value395 = vec![instruction.clone()];
                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallInstruction>::new(), source_ml_source2_concat_value394), source_ml_source2_concat_value395)
                                                }
                                            };
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override409, source_ml_source2_nominal_field399, source_ml_source2_nominal_field401, source_ml_source2_nominal_field403, source_ml_source2_nominal_field405, source_ml_source2_nominal_field407.clone())
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
pub fn allocate_java_script_call_source_register(builder: self::JavaScriptCallSourceBuilder) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value411) => {
            source_ml_source2_data_property_value411.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value413, _, _, _, _) => {
                source_ml_source2_data_property_value413
            }
        }) >= (4096f64)) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its register budget")
        } else {
            self::JavaScriptCallSourceResult::SourceResult(Box::new({
                let source_ml_source2_nominal_spread415 = builder.clone();
                {
                    let source_ml_source2_nominal_field416 = match source_ml_source2_nominal_spread415.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value417, _, _, _, _, _) => {
                            source_ml_source2_data_property_value417
                        }
                    };
                    {
                        let source_ml_source2_nominal_field418 = match source_ml_source2_nominal_spread415.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value419, _, _, _, _) => {
                                source_ml_source2_data_property_value419
                            }
                        };
                        {
                            let source_ml_source2_nominal_field420 = match source_ml_source2_nominal_spread415.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value421, _, _, _) => {
                                    source_ml_source2_data_property_value421
                                }
                            };
                            {
                                let source_ml_source2_nominal_field422 = match source_ml_source2_nominal_spread415.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value423, _, _) => {
                                        source_ml_source2_data_property_value423
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field424 = match source_ml_source2_nominal_spread415.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value425, _) => {
                                            source_ml_source2_data_property_value425
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field426 = match source_ml_source2_nominal_spread415.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value427) => {
                                                source_ml_source2_data_property_value427.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override428 = (match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value414, _, _, _, _) => {
                                                    source_ml_source2_data_property_value414
                                                }
                                            } + 1f64);
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field416, source_ml_source2_nominal_override428, source_ml_source2_nominal_field420, source_ml_source2_nominal_field422, source_ml_source2_nominal_field424, source_ml_source2_nominal_field426.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }), match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value429, _, _, _, _) => {
                    source_ml_source2_data_property_value429
                }
            })
        }
    }
}
