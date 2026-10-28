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
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value328) => {
            source_ml_source2_data_property_value328.clone()
        }
    }) != "") {
        builder.clone()
    } else {
        if ((((match &builder { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value330, _, _, _, _, _) => source_ml_source2_data_property_value330.as_slice() }).len() as f64)) >= (65536f64)) {
            {
                let source_ml_source2_nominal_spread331 = builder.clone();
                {
                    let source_ml_source2_nominal_field332 = match source_ml_source2_nominal_spread331.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value333, _, _, _, _, _) => {
                            source_ml_source2_data_property_value333
                        }
                    };
                    {
                        let source_ml_source2_nominal_field334 = match source_ml_source2_nominal_spread331.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value335, _, _, _, _) => {
                                source_ml_source2_data_property_value335
                            }
                        };
                        {
                            let source_ml_source2_nominal_field336 = match source_ml_source2_nominal_spread331.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value337, _, _, _) => {
                                    source_ml_source2_data_property_value337
                                }
                            };
                            {
                                let source_ml_source2_nominal_field338 = match source_ml_source2_nominal_spread331.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value339, _, _) => {
                                        source_ml_source2_data_property_value339
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field340 = match source_ml_source2_nominal_spread331.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value341, _) => {
                                            source_ml_source2_data_property_value341
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field342 = match source_ml_source2_nominal_spread331.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value343) => {
                                                source_ml_source2_data_property_value343.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override344 = String::from("source lowering exceeds its instruction budget");
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field332, source_ml_source2_nominal_field334, source_ml_source2_nominal_field336, source_ml_source2_nominal_field338, source_ml_source2_nominal_field340, source_ml_source2_nominal_override344.clone())
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
                let source_ml_source2_nominal_spread348 = builder.clone();
                {
                    let source_ml_source2_nominal_field349 = match source_ml_source2_nominal_spread348.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value350, _, _, _, _, _) => {
                            source_ml_source2_data_property_value350
                        }
                    };
                    {
                        let source_ml_source2_nominal_field351 = match source_ml_source2_nominal_spread348.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value352, _, _, _, _) => {
                                source_ml_source2_data_property_value352
                            }
                        };
                        {
                            let source_ml_source2_nominal_field353 = match source_ml_source2_nominal_spread348.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value354, _, _, _) => {
                                    source_ml_source2_data_property_value354
                                }
                            };
                            {
                                let source_ml_source2_nominal_field355 = match source_ml_source2_nominal_spread348.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value356, _, _) => {
                                        source_ml_source2_data_property_value356
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field357 = match source_ml_source2_nominal_spread348.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value358, _) => {
                                            source_ml_source2_data_property_value358
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field359 = match source_ml_source2_nominal_spread348.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value360) => {
                                                source_ml_source2_data_property_value360.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override361 = {
                                                let source_ml_source2_concat_value346 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value345, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value345
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_concat_value347 = vec![instruction.clone()];
                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallInstruction>::new(), source_ml_source2_concat_value346), source_ml_source2_concat_value347)
                                                }
                                            };
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override361, source_ml_source2_nominal_field351, source_ml_source2_nominal_field353, source_ml_source2_nominal_field355, source_ml_source2_nominal_field357, source_ml_source2_nominal_field359.clone())
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
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value363) => {
            source_ml_source2_data_property_value363.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value365, _, _, _, _) => {
                source_ml_source2_data_property_value365
            }
        }) >= (4096f64)) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its register budget")
        } else {
            self::JavaScriptCallSourceResult::SourceResult(Box::new({
                let source_ml_source2_nominal_spread367 = builder.clone();
                {
                    let source_ml_source2_nominal_field368 = match source_ml_source2_nominal_spread367.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value369, _, _, _, _, _) => {
                            source_ml_source2_data_property_value369
                        }
                    };
                    {
                        let source_ml_source2_nominal_field370 = match source_ml_source2_nominal_spread367.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value371, _, _, _, _) => {
                                source_ml_source2_data_property_value371
                            }
                        };
                        {
                            let source_ml_source2_nominal_field372 = match source_ml_source2_nominal_spread367.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value373, _, _, _) => {
                                    source_ml_source2_data_property_value373
                                }
                            };
                            {
                                let source_ml_source2_nominal_field374 = match source_ml_source2_nominal_spread367.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value375, _, _) => {
                                        source_ml_source2_data_property_value375
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field376 = match source_ml_source2_nominal_spread367.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value377, _) => {
                                            source_ml_source2_data_property_value377
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field378 = match source_ml_source2_nominal_spread367.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value379) => {
                                                source_ml_source2_data_property_value379.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override380 = (match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value366, _, _, _, _) => {
                                                    source_ml_source2_data_property_value366
                                                }
                                            } + 1f64);
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field368, source_ml_source2_nominal_override380, source_ml_source2_nominal_field372, source_ml_source2_nominal_field374, source_ml_source2_nominal_field376, source_ml_source2_nominal_field378.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }), match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value381, _, _, _, _) => {
                    source_ml_source2_data_property_value381
                }
            })
        }
    }
}
