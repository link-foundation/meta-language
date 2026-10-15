// Generated from the shared owned local state JavaScript kernel. Do not edit.
/// Read a shared owned local text state decision.
#[must_use]
pub fn accept_rust_owned_local_state_model(model: &str) -> bool {
    model.is_empty()
        || model == "closed owned text local state 1"
        || model == "canonical owned text local copies 1"
}

/// Read a shared owned local text state decision.
#[must_use]
pub fn accept_rust_owned_local_value_type(kind: &str, bits: &str, signed: bool) -> bool {
    kind == "string" || kind == "bool" || kind == "fixed" && bits == "32" && signed
}

/// Read a shared owned local text state decision.
#[must_use]
pub fn accept_rust_owned_local_expression(kind: &str) -> bool {
    kind == "lit"
        || kind == "var"
        || kind == "tuple"
        || kind == "tupleIndex"
        || kind == "let"
        || kind == "if"
        || kind == "binary"
        || kind == "toString"
}

/// Read a shared owned local text state decision.
#[must_use]
pub fn accept_rust_owned_local_binary_operation(operation: &str) -> bool {
    operation == "eq"
}

/// Read a shared owned local text state decision.
#[must_use]
pub fn accept_rust_owned_local_source_operation(operation: &str) -> bool {
    operation != "borrow" && operation != "dereference" && operation != "box construction"
}

/// Read a shared owned local text state decision.
#[must_use]
pub fn accept_rust_owned_local_container(kind: &str, dense: bool, unrepresented: bool) -> bool {
    kind == "array" && dense && !unrepresented
        || kind == "map" && !unrepresented
        || kind == "record" && !unrepresented
}

/// Read a shared owned local text state decision.
#[must_use]
pub const fn accept_rust_owned_local_record_value(
    boolean_required: bool,
    boolean_value: bool,
) -> bool {
    !boolean_required || boolean_value
}

/// Read a shared owned local text state decision.
#[must_use]
pub const fn accept_rust_owned_local_source_span(
    present: bool,
    record_shape: bool,
    offset_shape: bool,
) -> bool {
    !present || record_shape && offset_shape
}

/// Read a shared owned local text state decision.
#[must_use]
pub fn accept_rust_owned_local_text_units(units: &[f64]) -> bool {
    {
        let index = 0f64;
        {
            let ml_s1 = crate::translation::rust::owned_local_state_rules::ml_accept_rust_owned_local_text_units_loop1(units, index);
            match ml_s1 {
                crate::translation::rust::owned_local_state_rules::MlAcceptRustOwnedLocalTextUnitsLoop1Result::MlAcceptRustOwnedLocalTextUnitsLoop1Done => {
                    true
                }
                crate::translation::rust::owned_local_state_rules::MlAcceptRustOwnedLocalTextUnitsLoop1Result::MlAcceptRustOwnedLocalTextUnitsLoop1Return(ml_result1) => {
                    ml_result1
                }
            }
        }
    }
}

/// Read a shared owned local text state decision.
#[must_use]
pub fn render_rust_owned_local_text_input() -> String {
    String::from(
        "function ml_ownedTextInput(text){\n if(typeof text !== \"string\")return false;\n for(let index=0;index<text.length;index++){\n  const unit=text.charCodeAt(index);\n  if(unit>=55296&&unit<=56319){\n   index++;\n   const next=text.charCodeAt(index);\n   if(!(next>=56320&&next<=57343))return false;\n  }else if(unit>=56320&&unit<=57343)return false;\n }\n return true;\n}",
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptRustOwnedLocalTextUnitsLoop1Result {
    MlAcceptRustOwnedLocalTextUnitsLoop1Done,
    MlAcceptRustOwnedLocalTextUnitsLoop1Return(bool),
}

/// Read a shared owned local text state decision.
#[must_use]
pub fn ml_accept_rust_owned_local_text_units_loop1(
    units: &[f64],
    mut index: f64,
) -> crate::translation::rust::owned_local_state_rules::MlAcceptRustOwnedLocalTextUnitsLoop1Result {
    loop {
        return if index
            < serde_json::Number::from(units.len())
                .as_f64()
                .expect("a UTF16 unit count has a finite Number length")
        {
            {
                let unit = crate::translation::frontend_rules::ml_array::optional_at(
                    units,
                    crate::translation::frontend_rules::ml_array::number_index(index),
                )
                .unwrap_or(-1f64);
                if (55_296f64..=56_319f64).contains(&unit) {
                    {
                        let index_2 = index + 1f64;
                        {
                            let next = crate::translation::frontend_rules::ml_array::optional_at(
                                units,
                                crate::translation::frontend_rules::ml_array::number_index(index_2),
                            )
                            .unwrap_or(-1f64);
                            if (56_320f64..=57_343f64).contains(&next) {
                                {
                                    let index_3 = index_2 + 1f64;
                                    {
                                        index = index_3;
                                        continue;
                                    }
                                }
                            }
                            crate::translation::rust::owned_local_state_rules::MlAcceptRustOwnedLocalTextUnitsLoop1Result::MlAcceptRustOwnedLocalTextUnitsLoop1Return(false)
                        }
                    }
                } else if (56_320f64..=57_343f64).contains(&unit) {
                    crate::translation::rust::owned_local_state_rules::MlAcceptRustOwnedLocalTextUnitsLoop1Result::MlAcceptRustOwnedLocalTextUnitsLoop1Return(false)
                } else {
                    {
                        let index_4 = index + 1f64;
                        {
                            index = index_4;
                            continue;
                        }
                    }
                }
            }
        } else {
            crate::translation::rust::owned_local_state_rules::MlAcceptRustOwnedLocalTextUnitsLoop1Result::MlAcceptRustOwnedLocalTextUnitsLoop1Done
        };
    }
}
