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
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value357) => {
            source_ml_source2_data_property_value357.clone()
        }
    }) != "") {
        builder.clone()
    } else {
        if ((((match &builder { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value359, _, _, _, _, _) => source_ml_source2_data_property_value359.as_slice() }).len() as f64)) >= (65536f64)) {
            {
                let source_ml_source2_nominal_spread360 = builder.clone();
                {
                    let source_ml_source2_nominal_field361 = match source_ml_source2_nominal_spread360.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value362, _, _, _, _, _) => {
                            source_ml_source2_data_property_value362
                        }
                    };
                    {
                        let source_ml_source2_nominal_field363 = match source_ml_source2_nominal_spread360.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value364, _, _, _, _) => {
                                source_ml_source2_data_property_value364
                            }
                        };
                        {
                            let source_ml_source2_nominal_field365 = match source_ml_source2_nominal_spread360.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value366, _, _, _) => {
                                    source_ml_source2_data_property_value366
                                }
                            };
                            {
                                let source_ml_source2_nominal_field367 = match source_ml_source2_nominal_spread360.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value368, _, _) => {
                                        source_ml_source2_data_property_value368
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field369 = match source_ml_source2_nominal_spread360.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value370, _) => {
                                            source_ml_source2_data_property_value370
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field371 = match source_ml_source2_nominal_spread360.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value372) => {
                                                source_ml_source2_data_property_value372.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override373 = String::from("source lowering exceeds its instruction budget");
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field361, source_ml_source2_nominal_field363, source_ml_source2_nominal_field365, source_ml_source2_nominal_field367, source_ml_source2_nominal_field369, source_ml_source2_nominal_override373.clone())
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
                let source_ml_source2_nominal_spread377 = builder.clone();
                {
                    let source_ml_source2_nominal_field378 = match source_ml_source2_nominal_spread377.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value379, _, _, _, _, _) => {
                            source_ml_source2_data_property_value379
                        }
                    };
                    {
                        let source_ml_source2_nominal_field380 = match source_ml_source2_nominal_spread377.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value381, _, _, _, _) => {
                                source_ml_source2_data_property_value381
                            }
                        };
                        {
                            let source_ml_source2_nominal_field382 = match source_ml_source2_nominal_spread377.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value383, _, _, _) => {
                                    source_ml_source2_data_property_value383
                                }
                            };
                            {
                                let source_ml_source2_nominal_field384 = match source_ml_source2_nominal_spread377.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value385, _, _) => {
                                        source_ml_source2_data_property_value385
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field386 = match source_ml_source2_nominal_spread377.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value387, _) => {
                                            source_ml_source2_data_property_value387
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field388 = match source_ml_source2_nominal_spread377.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value389) => {
                                                source_ml_source2_data_property_value389.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override390 = {
                                                let source_ml_source2_concat_value375 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value374, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value374
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_concat_value376 = vec![instruction.clone()];
                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallInstruction>::new(), source_ml_source2_concat_value375), source_ml_source2_concat_value376)
                                                }
                                            };
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override390, source_ml_source2_nominal_field380, source_ml_source2_nominal_field382, source_ml_source2_nominal_field384, source_ml_source2_nominal_field386, source_ml_source2_nominal_field388.clone())
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
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value392) => {
            source_ml_source2_data_property_value392.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value394, _, _, _, _) => {
                source_ml_source2_data_property_value394
            }
        }) >= (4096f64)) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its register budget")
        } else {
            self::JavaScriptCallSourceResult::SourceResult(Box::new({
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
                                            let source_ml_source2_nominal_override409 = (match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value395, _, _, _, _) => {
                                                    source_ml_source2_data_property_value395
                                                }
                                            } + 1f64);
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field397, source_ml_source2_nominal_override409, source_ml_source2_nominal_field401, source_ml_source2_nominal_field403, source_ml_source2_nominal_field405, source_ml_source2_nominal_field407.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }), match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value410, _, _, _, _) => {
                    source_ml_source2_data_property_value410
                }
            })
        }
    }
}
