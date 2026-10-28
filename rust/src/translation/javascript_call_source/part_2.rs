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
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value333) => {
            source_ml_source2_data_property_value333.clone()
        }
    }) != "") {
        builder.clone()
    } else {
        if ((((match &builder { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value335, _, _, _, _, _) => source_ml_source2_data_property_value335.as_slice() }).len() as f64)) >= (65536f64)) {
            {
                let source_ml_source2_nominal_spread336 = builder.clone();
                {
                    let source_ml_source2_nominal_field337 = match source_ml_source2_nominal_spread336.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value338, _, _, _, _, _) => {
                            source_ml_source2_data_property_value338
                        }
                    };
                    {
                        let source_ml_source2_nominal_field339 = match source_ml_source2_nominal_spread336.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value340, _, _, _, _) => {
                                source_ml_source2_data_property_value340
                            }
                        };
                        {
                            let source_ml_source2_nominal_field341 = match source_ml_source2_nominal_spread336.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value342, _, _, _) => {
                                    source_ml_source2_data_property_value342
                                }
                            };
                            {
                                let source_ml_source2_nominal_field343 = match source_ml_source2_nominal_spread336.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value344, _, _) => {
                                        source_ml_source2_data_property_value344
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field345 = match source_ml_source2_nominal_spread336.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value346, _) => {
                                            source_ml_source2_data_property_value346
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field347 = match source_ml_source2_nominal_spread336.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value348) => {
                                                source_ml_source2_data_property_value348.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override349 = String::from("source lowering exceeds its instruction budget");
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field337, source_ml_source2_nominal_field339, source_ml_source2_nominal_field341, source_ml_source2_nominal_field343, source_ml_source2_nominal_field345, source_ml_source2_nominal_override349.clone())
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
                let source_ml_source2_nominal_spread353 = builder.clone();
                {
                    let source_ml_source2_nominal_field354 = match source_ml_source2_nominal_spread353.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value355, _, _, _, _, _) => {
                            source_ml_source2_data_property_value355
                        }
                    };
                    {
                        let source_ml_source2_nominal_field356 = match source_ml_source2_nominal_spread353.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value357, _, _, _, _) => {
                                source_ml_source2_data_property_value357
                            }
                        };
                        {
                            let source_ml_source2_nominal_field358 = match source_ml_source2_nominal_spread353.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value359, _, _, _) => {
                                    source_ml_source2_data_property_value359
                                }
                            };
                            {
                                let source_ml_source2_nominal_field360 = match source_ml_source2_nominal_spread353.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value361, _, _) => {
                                        source_ml_source2_data_property_value361
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field362 = match source_ml_source2_nominal_spread353.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value363, _) => {
                                            source_ml_source2_data_property_value363
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field364 = match source_ml_source2_nominal_spread353.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value365) => {
                                                source_ml_source2_data_property_value365.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override366 = {
                                                let source_ml_source2_concat_value351 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value350, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value350
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_concat_value352 = vec![instruction.clone()];
                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallInstruction>::new(), source_ml_source2_concat_value351), source_ml_source2_concat_value352)
                                                }
                                            };
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override366, source_ml_source2_nominal_field356, source_ml_source2_nominal_field358, source_ml_source2_nominal_field360, source_ml_source2_nominal_field362, source_ml_source2_nominal_field364.clone())
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
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value368) => {
            source_ml_source2_data_property_value368.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value370, _, _, _, _) => {
                source_ml_source2_data_property_value370
            }
        }) >= (4096f64)) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its register budget")
        } else {
            self::JavaScriptCallSourceResult::SourceResult(Box::new({
                let source_ml_source2_nominal_spread372 = builder.clone();
                {
                    let source_ml_source2_nominal_field373 = match source_ml_source2_nominal_spread372.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value374, _, _, _, _, _) => {
                            source_ml_source2_data_property_value374
                        }
                    };
                    {
                        let source_ml_source2_nominal_field375 = match source_ml_source2_nominal_spread372.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value376, _, _, _, _) => {
                                source_ml_source2_data_property_value376
                            }
                        };
                        {
                            let source_ml_source2_nominal_field377 = match source_ml_source2_nominal_spread372.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value378, _, _, _) => {
                                    source_ml_source2_data_property_value378
                                }
                            };
                            {
                                let source_ml_source2_nominal_field379 = match source_ml_source2_nominal_spread372.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value380, _, _) => {
                                        source_ml_source2_data_property_value380
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field381 = match source_ml_source2_nominal_spread372.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value382, _) => {
                                            source_ml_source2_data_property_value382
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field383 = match source_ml_source2_nominal_spread372.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value384) => {
                                                source_ml_source2_data_property_value384.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override385 = (match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value371, _, _, _, _) => {
                                                    source_ml_source2_data_property_value371
                                                }
                                            } + 1f64);
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field373, source_ml_source2_nominal_override385, source_ml_source2_nominal_field377, source_ml_source2_nominal_field379, source_ml_source2_nominal_field381, source_ml_source2_nominal_field383.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }), match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value386, _, _, _, _) => {
                    source_ml_source2_data_property_value386
                }
            })
        }
    }
}
