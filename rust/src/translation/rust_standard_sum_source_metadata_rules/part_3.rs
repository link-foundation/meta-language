// Generated from complete js/src/translation/rust-standard-sum-source-metadata-rules.js. Do not edit by hand.
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_rust_standard_sum_source_materialization_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_metadata_keys(form: &str) -> Vec<String> {
    if form == "origin" {
        vec![
            String::from("type"),
            String::from("path"),
            String::from("wrappers"),
            String::from("sourceOwner"),
            String::from("expandedSourceOwner"),
            String::from("fields"),
            String::from("callableParameters"),
            String::from("callableResult"),
            String::from("floatSourceModel"),
            String::from("textSourceModel"),
            String::from("arraySourceModel"),
            String::from("sourceLifetimes"),
            String::from("namespaceSpelling"),
            String::from("standardSum"),
        ]
    } else if form == "owner" {
        vec![
            String::from("kind"),
            String::from("path"),
            String::from("arguments"),
        ]
    } else if form == "sum" {
        vec![String::from("owner"), String::from("arguments")]
    } else if form == "source position" {
        vec![
            String::from("type"),
            String::from("wrappers"),
            String::from("lifetimes"),
            String::from("owner"),
        ]
    } else if form == "array element position" {
        vec![String::from("arrayElementPosition")]
    } else if form == "identity value" {
        vec![
            String::from("type"),
            String::from("owner"),
            String::from("wrappers"),
            String::from("fields"),
            String::from("float"),
            String::from("text"),
        ]
    } else if form == "identity sum" {
        vec![
            String::from("owner"),
            String::from("arguments"),
            String::from("wrappers"),
        ]
    } else if form == "identity header" {
        vec![
            String::from("type"),
            String::from("path"),
            String::from("owner"),
            String::from("wrappers"),
        ]
    } else if form == "fixed" {
        vec![
            String::from("kind"),
            String::from("bits"),
            String::from("signed"),
        ]
    } else if form == "named" {
        vec![String::from("kind"), String::from("path")]
    } else if form == "data" {
        vec![String::from("kind"), String::from("name")]
    } else if (form == "array") || (form == "set") {
        vec![
            String::from("kind"),
            String::from("element"),
            String::from("reference"),
        ]
    } else if form == "cell" {
        vec![String::from("kind"), String::from("element")]
    } else if form == "option" {
        vec![
            String::from("kind"),
            String::from("element"),
            String::from("absence"),
        ]
    } else if form == "absence" {
        vec![String::from("kind"), String::from("absence")]
    } else if form == "tuple" {
        vec![String::from("kind"), String::from("elements")]
    } else if form == "union" {
        vec![String::from("kind"), String::from("alternatives")]
    } else if form == "record" {
        vec![
            String::from("kind"),
            String::from("fields"),
            String::from("ordered"),
            String::from("open"),
            String::from("reference"),
        ]
    } else if form == "record field" {
        vec![String::from("name"), String::from("type")]
    } else if form == "map" {
        vec![
            String::from("kind"),
            String::from("key"),
            String::from("value"),
            String::from("reference"),
        ]
    } else if form == "function" {
        vec![
            String::from("kind"),
            String::from("params"),
            String::from("ret"),
        ]
    } else if form == "primitive" {
        vec![String::from("kind")]
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_metadata_type_form(kind: &str) -> String {
    if self::ml_array::includes(
        &vec![
            String::from("fixed"),
            String::from("named"),
            String::from("data"),
            String::from("array"),
            String::from("cell"),
            String::from("option"),
            String::from("absence"),
            String::from("tuple"),
            String::from("union"),
            String::from("record"),
            String::from("map"),
            String::from("set"),
            String::from("function"),
        ],
        &kind.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) {
        kind.to_owned()
    } else if self::ml_array::includes(
        &vec![
            String::from("nat"),
            String::from("int"),
            String::from("bool"),
            String::from("string"),
            String::from("unit"),
            String::from("float"),
            String::from("literal"),
            String::from("hypothesis"),
            String::from("output"),
        ],
        &kind.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    ) {
        String::from("primitive")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_metadata_record_cost(form: &str) -> Vec<f64> {
    {
        let keys = self::read_rust_standard_sum_source_metadata_keys(form);
        if (keys.len() as f64) == 0f64 {
            Vec::<f64>::new()
        } else {
            {
                let units = 0f64;
                {
                    let at = 0f64;
                    {
                        let units_2 = self::source_ml_source1_temporary_read_rust_standard_sum_source_metadata_record_cost_loop6(&(keys), units, at);
                        vec![1f64, units_2]
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_metadata_count_error(
    nodes: f64,
    units: f64,
    depth: f64,
) -> String {
    {
        let error =
            self::read_rust_standard_sum_source_materialization_error(0f64, 0f64, nodes, units);
        if (((!(error).is_empty()) || !(self::ml_math::is_integer(depth))) || (depth < 0f64))
            || (depth > 256f64)
        {
            String::from(
                "the complete standard sum source metadata exceeds its finite node, UTF16 or depth budget",
            )
        } else {
            String::new()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_metadata_reservation(
    nodes: f64,
    units: f64,
    cost_nodes: f64,
    cost_units: f64,
) -> Vec<f64> {
    {
        let additional_nodes = cost_nodes * 16f64;
        {
            let additional_units = cost_units * 16f64;
            if (self::read_rust_standard_sum_source_materialization_error(
                nodes,
                units,
                additional_nodes,
                additional_units,
            )) == ""
            {
                vec![(nodes + additional_nodes), (units + additional_units)]
            } else {
                Vec::<f64>::new()
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result {
    MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Done(f64),
    MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_sum_source_depth_loop2(
    mut depth: f64,
    source_ml_source1_temporary_values1: &[f64],
    mut source_ml_source1_temporary_index1: self::ml::Big,
) -> self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result {
    loop {
        return if source_ml_source1_temporary_index1
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values1.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_source1_temporary_values1,
                    source_ml_source1_temporary_index1.to_index(),
                );
                if (!(self::ml_math::is_integer(child)) || (child < 0f64)) || (child > 16f64) {
                    self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Return(-1f64)
                } else {
                    if child > depth {
                        {
                            let depth_2 = child;
                            {
                                let source_ml_source1_temporary_index1_2 =
                                    source_ml_source1_temporary_index1
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (depth, source_ml_source1_temporary_index1) =
                                        (depth_2, source_ml_source1_temporary_index1_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let source_ml_source1_temporary_index1_3 =
                                source_ml_source1_temporary_index1
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1 =
                                    source_ml_source1_temporary_index1_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Done(depth)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result {
    MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Done,
    MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_sum_source_materialization_error_loop4(
    source_ml_source1_temporary_values3: &[f64],
    mut source_ml_source1_temporary_index3: self::ml::Big,
) -> self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result {
    loop {
        return if source_ml_source1_temporary_index3
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values3.len() as u128))
        {
            {
                let value = self::ml_array::at(
                    source_ml_source1_temporary_values3,
                    source_ml_source1_temporary_index3.to_index(),
                );
                if !(self::ml_math::is_integer(value)) || (value < 0f64) {
                    self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Return(String::from("invalid standard sum source materialization count"))
                } else {
                    {
                        let source_ml_source1_temporary_index3_2 =
                            source_ml_source1_temporary_index3.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index3 =
                                source_ml_source1_temporary_index3_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result {
    MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Done,
    MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_rust_standard_sum_source_arguments_loop5(
    identities: &[String],
    direct_copies: &[bool],
    mut count: f64,
    mut at: f64,
) -> self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result {
    loop {
        return if at < count {
            if (((self::ml_array::optional_at(identities, self::ml_array::number_index(at)))
                .unwrap_or_else(|| String::new()))
                == "")
                || {
                    let source_ml_source2_comparison_operand22 = self::ml_array::optional_at(
                        direct_copies,
                        self::ml_array::number_index(at),
                    );
                    {
                        let source_ml_source2_comparison_operand23 = true;
                        !(match (
                            source_ml_source2_comparison_operand22,
                            source_ml_source2_comparison_operand23,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }
                }
            {
                self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Return(false)
            } else {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_sum_source_metadata_record_cost_loop6(
    keys: &[String],
    mut units: f64,
    mut at: f64,
) -> f64 {
    loop {
        if at < (keys.len() as f64) {
            {
                let units_2 = units
                    + (((self::ml_array::optional_at(keys, self::ml_array::number_index(at)))
                        .unwrap_or_else(|| String::new()))
                    .encode_utf16()
                    .count() as f64);
                {
                    let at_2 = at + 1f64;
                    {
                        (units, at) = (units_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { units };
    }
}
