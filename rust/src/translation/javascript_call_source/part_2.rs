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
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value339) => {
            source_ml_source2_data_property_value339.clone()
        }
    }) != "") {
        builder.clone()
    } else {
        if ((((match &builder { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value341, _, _, _, _, _) => source_ml_source2_data_property_value341.as_slice() }).len() as f64)) >= (65536f64)) {
            {
                let source_ml_source2_nominal_spread342 = builder.clone();
                {
                    let source_ml_source2_nominal_field343 = match source_ml_source2_nominal_spread342.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value344, _, _, _, _, _) => {
                            source_ml_source2_data_property_value344
                        }
                    };
                    {
                        let source_ml_source2_nominal_field345 = match source_ml_source2_nominal_spread342.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value346, _, _, _, _) => {
                                source_ml_source2_data_property_value346
                            }
                        };
                        {
                            let source_ml_source2_nominal_field347 = match source_ml_source2_nominal_spread342.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value348, _, _, _) => {
                                    source_ml_source2_data_property_value348
                                }
                            };
                            {
                                let source_ml_source2_nominal_field349 = match source_ml_source2_nominal_spread342.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value350, _, _) => {
                                        source_ml_source2_data_property_value350
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field351 = match source_ml_source2_nominal_spread342.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value352, _) => {
                                            source_ml_source2_data_property_value352
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field353 = match source_ml_source2_nominal_spread342.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value354) => {
                                                source_ml_source2_data_property_value354.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override355 = String::from("source lowering exceeds its instruction budget");
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field343, source_ml_source2_nominal_field345, source_ml_source2_nominal_field347, source_ml_source2_nominal_field349, source_ml_source2_nominal_field351, source_ml_source2_nominal_override355.clone())
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
                let source_ml_source2_nominal_spread359 = builder.clone();
                {
                    let source_ml_source2_nominal_field360 = match source_ml_source2_nominal_spread359.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value361, _, _, _, _, _) => {
                            source_ml_source2_data_property_value361
                        }
                    };
                    {
                        let source_ml_source2_nominal_field362 = match source_ml_source2_nominal_spread359.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value363, _, _, _, _) => {
                                source_ml_source2_data_property_value363
                            }
                        };
                        {
                            let source_ml_source2_nominal_field364 = match source_ml_source2_nominal_spread359.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value365, _, _, _) => {
                                    source_ml_source2_data_property_value365
                                }
                            };
                            {
                                let source_ml_source2_nominal_field366 = match source_ml_source2_nominal_spread359.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value367, _, _) => {
                                        source_ml_source2_data_property_value367
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field368 = match source_ml_source2_nominal_spread359.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value369, _) => {
                                            source_ml_source2_data_property_value369
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field370 = match source_ml_source2_nominal_spread359.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value371) => {
                                                source_ml_source2_data_property_value371.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override372 = {
                                                let source_ml_source2_concat_value357 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value356, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value356
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_concat_value358 = vec![instruction.clone()];
                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallInstruction>::new(), source_ml_source2_concat_value357), source_ml_source2_concat_value358)
                                                }
                                            };
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override372, source_ml_source2_nominal_field362, source_ml_source2_nominal_field364, source_ml_source2_nominal_field366, source_ml_source2_nominal_field368, source_ml_source2_nominal_field370.clone())
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
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value374) => {
            source_ml_source2_data_property_value374.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value376, _, _, _, _) => {
                source_ml_source2_data_property_value376
            }
        }) >= (4096f64)) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its register budget")
        } else {
            self::JavaScriptCallSourceResult::SourceResult(Box::new({
                let source_ml_source2_nominal_spread378 = builder.clone();
                {
                    let source_ml_source2_nominal_field379 = match source_ml_source2_nominal_spread378.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value380, _, _, _, _, _) => {
                            source_ml_source2_data_property_value380
                        }
                    };
                    {
                        let source_ml_source2_nominal_field381 = match source_ml_source2_nominal_spread378.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value382, _, _, _, _) => {
                                source_ml_source2_data_property_value382
                            }
                        };
                        {
                            let source_ml_source2_nominal_field383 = match source_ml_source2_nominal_spread378.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value384, _, _, _) => {
                                    source_ml_source2_data_property_value384
                                }
                            };
                            {
                                let source_ml_source2_nominal_field385 = match source_ml_source2_nominal_spread378.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value386, _, _) => {
                                        source_ml_source2_data_property_value386
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field387 = match source_ml_source2_nominal_spread378.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value388, _) => {
                                            source_ml_source2_data_property_value388
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field389 = match source_ml_source2_nominal_spread378.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value390) => {
                                                source_ml_source2_data_property_value390.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override391 = (match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value377, _, _, _, _) => {
                                                    source_ml_source2_data_property_value377
                                                }
                                            } + 1f64);
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field379, source_ml_source2_nominal_override391, source_ml_source2_nominal_field383, source_ml_source2_nominal_field385, source_ml_source2_nominal_field387, source_ml_source2_nominal_field389.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }), match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value392, _, _, _, _) => {
                    source_ml_source2_data_property_value392
                }
            })
        }
    }
}
