// Generated from complete js/src/translation/javascript-record-order-rules.js. Do not edit by hand.
use super::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result;
use super::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Result;
use super::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result;
use super::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result;
use super::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result;
use super::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result;
use super::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result;
use super::accept_record_field_name;
use super::compare_record_field_names;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::read_required_number_fact;
use super::read_required_record_name;
use super::source_ml_temporary_accept_java_script_record_order_names_loop23;
use super::source_ml_temporary_accept_java_script_record_order_permutation_loop24;
use super::source_ml_temporary_accept_java_script_record_property_order_loop29;
use super::source_ml_temporary_create_java_script_record_property_order_loop26;
use super::source_ml_temporary_create_java_script_record_property_order_loop28;
use super::source_ml_temporary_merge_java_script_record_property_order_loop32;
use super::source_ml_temporary_read_java_script_literal_property_index_loop22;
use super::source_ml_temporary_read_java_script_record_order_merge_error_loop30;
use super::source_ml_temporary_read_java_script_record_order_rebase_error_loop34;
use super::source_ml_temporary_read_java_script_record_property_keys_loop36;
use super::source_ml_temporary_read_record_type_owner_presence_loop17;
use super::source_ml_temporary_rebase_java_script_record_property_order_loop35;
use super::source_ml_temporary_record_type_fact_elements_loop16;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn record_type_fact_elements(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    owner: f64,
) -> Vec<f64> {
    {
        let values = Vec::<f64>::new();
        if owner < 0f64 {
            values
        } else {
            {
                let start = self::read_required_number_fact(starts, owner);
                {
                    let end = start + self::read_required_number_fact(counts, owner);
                    {
                        let edge = start;
                        {
                            self::source_ml_temporary_record_type_fact_elements_loop16(
                                targets, values, end, edge,
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_record_type_owner_presence(
    texts: &[String],
    numbers: &[f64],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    bindings: &[f64],
    root: f64,
    owner: f64,
) -> bool {
    {
        let pending = vec![root];
        {
            let seen = Vec::<f64>::new();
            {
                let position = 0f64;
                {
                    let source_ml_source1_s9 =
                        self::source_ml_temporary_read_record_type_owner_presence_loop17(
                            texts, numbers, starts, counts, labels, targets, bindings, owner,
                            pending, seen, position,
                        );
                    match source_ml_source1_s9 {
                        self::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result::MlTemporaryReadRecordTypeOwnerPresenceLoop17Done => {
                            false
                        }
                        self::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result::MlTemporaryReadRecordTypeOwnerPresenceLoop17Return(source_ml_temporary_result17) => {
                            source_ml_temporary_result17
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_literal_property_index(key: &[f64]) -> f64 {
    if ((key.len() as f64) == 0f64) || ((key.len() as f64) > 10f64) {
        -1f64
    } else {
        if ((key.len() as f64) > 1f64) && {
            let source_ml_source1_comparison_operand10 =
                self::ml_array::optional_at(key, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand11 = 48f64;
                match (
                    source_ml_source1_comparison_operand10,
                    source_ml_source1_comparison_operand11,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        } {
            -1f64
        } else {
            {
                let value = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s12 = self::source_ml_temporary_read_java_script_literal_property_index_loop22(key, value, at);
                        match source_ml_source1_s12 {
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Done(value_2) => {
                                if value_2 <= 4_294_967_294f64 {
                                    value_2
                                } else {
                                    -1f64
                                }
                            }
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Return(source_ml_temporary_result22) => {
                                source_ml_temporary_result22
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
pub fn read_java_script_literal_property_form(kind: &str, index: f64) -> String {
    if (((kind == "array") || (kind == "tuple")) || (kind == "string")) && (index >= 0f64) {
        String::from("index")
    } else {
        String::from("field")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_string_index_number_form(kind: &str, element: &str) -> String {
    if kind == "float" {
        String::from("identity")
    } else if (kind == "int") || (kind == "nat") {
        String::from("convert")
    } else if (kind == "unit") || ((kind == "option") && (element == "unit")) {
        String::from("absent")
    } else if (kind == "option")
        && (((element == "float") || (element == "int")) || (element == "nat"))
    {
        String::from("optional")
    } else {
        String::from("refuse")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_java_script_record_order_name() -> Vec<f64> {
    (self::ml_source_exception::SourceException {
        class_name: String::from("TypeError"),
        message_units: (String::from("an ordered record needs every actual own field name"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_java_script_record_order_position() -> f64 {
    (self::ml_source_exception::SourceException {
        class_name: String::from("TypeError"),
        message_units: (String::from("an ordered record needs every actual field position"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_name(names: &[Vec<f64>], index: f64) -> Vec<f64> {
    (self::ml_array::optional_at(names, self::ml_array::number_index(index)))
        .unwrap_or_else(self::refuse_java_script_record_order_name)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_position(positions: &[f64], index: f64) -> f64 {
    (self::ml_array::optional_at(positions, self::ml_array::number_index(index)))
        .unwrap_or_else(self::refuse_java_script_record_order_position)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_record_order_names(names: &[Vec<f64>]) -> bool {
    {
        let index = 0f64;
        {
            let source_ml_source1_s13 =
                self::source_ml_temporary_accept_java_script_record_order_names_loop23(
                    names, index,
                );
            match source_ml_source1_s13 {
                self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Done => {
                    true
                }
                self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Return(source_ml_temporary_result23) => {
                    source_ml_temporary_result23
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_record_order_permutation(count: f64, order: &[f64]) -> bool {
    if ((!(self::ml_math::is_safe_integer(count)) || (count < 0f64))
        || ((count == 0f64) && ((1f64 / count) < 0f64)))
        || ((order.len() as f64) != count)
    {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s14 =
                    self::source_ml_temporary_accept_java_script_record_order_permutation_loop24(
                        count, order, index,
                    );
                match source_ml_source1_s14 {
                    self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Done => {
                        true
                    }
                    self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Return(source_ml_temporary_result24) => {
                        source_ml_temporary_result24
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_creation_error(
    names: &[Vec<f64>],
    insertion: &[f64],
) -> String {
    if !(self::accept_java_script_record_order_names(names))
        || !(self::accept_java_script_record_order_permutation((names.len() as f64), insertion))
    {
        String::from("invalid ordered record field sequence")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_record_property_order(names: &[Vec<f64>], insertion: &[f64]) -> Vec<f64> {
    {
        let error = self::read_java_script_record_order_creation_error(names, insertion);
        if error.is_empty() {
            {
                let numeric = Vec::<f64>::new();
                {
                    let field = 0f64;
                    {
                        let numeric_2 = self::source_ml_temporary_create_java_script_record_property_order_loop26(names, numeric, field);
                        {
                            let result = numeric_2;
                            {
                                let index = 0f64;
                                {
                                    self::source_ml_temporary_create_java_script_record_property_order_loop28(names, insertion, result, index)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (error).encode_utf16().collect(),
                message_present: true,
            })
            .raise()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_record_property_order(names: &[Vec<f64>], order: &[f64]) -> bool {
    if !(self::accept_java_script_record_order_names(names))
        || !(self::accept_java_script_record_order_permutation((names.len() as f64), order))
    {
        false
    } else {
        {
            let expected = self::create_java_script_record_property_order(names, order);
            {
                let index = 0f64;
                {
                    let source_ml_source1_s15 =
                        self::source_ml_temporary_accept_java_script_record_property_order_loop29(
                            order,
                            &(expected),
                            index,
                        );
                    match source_ml_source1_s15 {
                        self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Done => {
                            true
                        }
                        self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Return(source_ml_temporary_result29) => {
                            source_ml_temporary_result29
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_merge_error(
    names: &[Vec<f64>],
    chunks: &[Vec<f64>],
) -> String {
    if (self::accept_java_script_record_order_names(names)) {
        {
            let insertion = Vec::<f64>::new();
            {
                let chunk = 0f64;
                {
                    let source_ml_source1_s16 =
                        self::source_ml_temporary_read_java_script_record_order_merge_error_loop30(
                            names, chunks, insertion, chunk,
                        );
                    match source_ml_source1_s16 {
                        self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Done(insertion_2) => {
                            self::read_java_script_record_order_creation_error(names, &(insertion_2))
                        }
                        self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Return(source_ml_temporary_result30) => {
                            source_ml_temporary_result30
                        }
                    }
                }
            }
        }
    } else {
        String::from("invalid ordered record field names")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn merge_java_script_record_property_order(
    names: &[Vec<f64>],
    chunks: &[Vec<f64>],
) -> Vec<f64> {
    {
        let error = self::read_java_script_record_order_merge_error(names, chunks);
        if error.is_empty() {
            {
                let insertion = Vec::<f64>::new();
                {
                    let chunk = 0f64;
                    {
                        let insertion_2 = self::source_ml_temporary_merge_java_script_record_property_order_loop32(chunks, insertion, chunk);
                        self::create_java_script_record_property_order(names, &(insertion_2))
                    }
                }
            }
        } else {
            (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (error).encode_utf16().collect(),
                message_present: true,
            })
            .raise()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_rebase_error(
    target: &[Vec<f64>],
    source: &[Vec<f64>],
    order: &[f64],
) -> String {
    if !(self::accept_java_script_record_order_names(target))
        || !(self::accept_java_script_record_property_order(source, order))
    {
        String::from("invalid ordered record spread source")
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s17 =
                    self::source_ml_temporary_read_java_script_record_order_rebase_error_loop34(
                        target, source, index,
                    );
                match source_ml_source1_s17 {
                    self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Done => {
                        String::new()
                    }
                    self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Return(source_ml_temporary_result34) => {
                        source_ml_temporary_result34
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn rebase_java_script_record_property_order(
    target: &[Vec<f64>],
    source: &[Vec<f64>],
    order: &[f64],
) -> Vec<f64> {
    {
        let error = self::read_java_script_record_order_rebase_error(target, source, order);
        if error.is_empty() {
            {
                let result = Vec::<f64>::new();
                {
                    let index = 0f64;
                    {
                        self::source_ml_temporary_rebase_java_script_record_property_order_loop35(
                            target, source, order, result, index,
                        )
                    }
                }
            }
        } else {
            (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (error).encode_utf16().collect(),
                message_present: true,
            })
            .raise()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_key_error(names: &[Vec<f64>], order: &[f64]) -> String {
    if self::accept_java_script_record_property_order(names, order) {
        String::new()
    } else {
        String::from("invalid ordered record property order")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_property_keys(names: &[Vec<f64>], order: &[f64]) -> Vec<Vec<f64>> {
    {
        let error = self::read_java_script_record_order_key_error(names, order);
        if error.is_empty() {
            {
                let result = Vec::<Vec<f64>>::new();
                {
                    let index = 0f64;
                    {
                        self::source_ml_temporary_read_java_script_record_property_keys_loop36(
                            names, order, result, index,
                        )
                    }
                }
            }
        } else {
            (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (error).encode_utf16().collect(),
                message_present: true,
            })
            .raise()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_value_order_loop1(
    mut count: f64,
    mut reverse: bool,
    mut order: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < count {
            {
                let order_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![if reverse {
                            (count - index) - 1f64
                        } else {
                            index
                        }],
                        order,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (order, index) = (order_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { order };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRecordProjectionErrorLoop2Result {
    MlTemporaryReadRecordProjectionErrorLoop2Done,
    MlTemporaryReadRecordProjectionErrorLoop2Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_projection_error_loop2(
    names: &[Vec<f64>],
    entries: &[bool],
    mut index: f64,
) -> self::MlTemporaryReadRecordProjectionErrorLoop2Result {
    loop {
        return if index < (names.len() as f64) {
            if ((self::ml_array::optional_at(entries, self::ml_array::number_index(index)))
                .unwrap_or_default())
            {
                if (self::accept_record_field_name(
                    &(self::read_required_record_name(names, index)),
                )) {
                    if (index > 0f64)
                        && ((self::compare_record_field_names(
                            &(self::read_required_record_name(names, (index - 1f64))),
                            &(self::read_required_record_name(names, index)),
                        )) >= 0f64)
                    {
                        self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Return(String::from("checked record projection needs unique fields in UTF16 order"))
                    } else {
                        {
                            let index_2 = index + 1f64;
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                } else {
                    self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Return(String::from("checked record projection has an invalid UTF16 field name"))
                }
            } else {
                self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Return(String::from("checked record projection has a malformed field entry"))
            }
        } else {
            self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRecordFieldNameLoop3Result {
    MlTemporaryAcceptRecordFieldNameLoop3Done,
    MlTemporaryAcceptRecordFieldNameLoop3Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_record_field_name_loop3(
    name: &[f64],
    mut index: f64,
) -> self::MlTemporaryAcceptRecordFieldNameLoop3Result {
    loop {
        return if index < (name.len() as f64) {
            {
                let unit = self::ml_array::optional_at(name, self::ml_array::number_index(index));
                if (!((unit).is_some_and(self::ml_math::is_integer)) || {
                    let source_ml_source1_comparison_operand18 = unit;
                    {
                        let source_ml_source1_comparison_operand19 = 0f64;
                        if match (source_ml_source1_comparison_operand18.clone(), None::<f64>) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        } {
                            (f64::NAN) < source_ml_source1_comparison_operand19
                        } else {
                            (source_ml_source1_comparison_operand18
                                .clone()
                                .expect("a proven present value"))
                                < source_ml_source1_comparison_operand19
                        }
                    }
                }) || {
                    let source_ml_source1_comparison_operand20 = unit;
                    {
                        let source_ml_source1_comparison_operand21 = 65_535f64;
                        if match (source_ml_source1_comparison_operand20.clone(), None::<f64>) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        } {
                            (f64::NAN) > source_ml_source1_comparison_operand21
                        } else {
                            (source_ml_source1_comparison_operand20
                                .clone()
                                .expect("a proven present value"))
                                > source_ml_source1_comparison_operand21
                        }
                    }
                } {
                    self::MlTemporaryAcceptRecordFieldNameLoop3Result::MlTemporaryAcceptRecordFieldNameLoop3Return(false)
                } else {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptRecordFieldNameLoop3Result::MlTemporaryAcceptRecordFieldNameLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareRecordFieldNamesLoop4Result {
    MlTemporaryCompareRecordFieldNamesLoop4Done,
    MlTemporaryCompareRecordFieldNamesLoop4Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compare_record_field_names_loop4(
    left: &[f64],
    right: &[f64],
    mut end: f64,
    mut index: f64,
) -> self::MlTemporaryCompareRecordFieldNamesLoop4Result {
    loop {
        return if index < end {
            if (self::read_required_number_fact(left, index))
                < (self::read_required_number_fact(right, index))
            {
                self::MlTemporaryCompareRecordFieldNamesLoop4Result::MlTemporaryCompareRecordFieldNamesLoop4Return(-1f64)
            } else if (self::read_required_number_fact(left, index))
                > (self::read_required_number_fact(right, index))
            {
                self::MlTemporaryCompareRecordFieldNamesLoop4Result::MlTemporaryCompareRecordFieldNamesLoop4Return(1f64)
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryCompareRecordFieldNamesLoop4Result::MlTemporaryCompareRecordFieldNamesLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRecordFieldPositionLoop6Result {
    MlTemporaryReadRecordFieldPositionLoop6Done,
    MlTemporaryReadRecordFieldPositionLoop6Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_field_position_loop6(
    fields: &[Vec<f64>],
    name: &[f64],
    mut index: f64,
) -> self::MlTemporaryReadRecordFieldPositionLoop6Result {
    loop {
        return if index < (fields.len() as f64) {
            {
                let comparison = self::compare_record_field_names(
                    &(self::read_required_record_name(fields, index)),
                    name,
                );
                if ((comparison).partial_cmp(&0f64) != Some(std::cmp::Ordering::Less))
                    && ((comparison).partial_cmp(&0f64) != Some(std::cmp::Ordering::Greater))
                {
                    self::MlTemporaryReadRecordFieldPositionLoop6Result::MlTemporaryReadRecordFieldPositionLoop6Return(index)
                } else {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRecordFieldPositionLoop6Result::MlTemporaryReadRecordFieldPositionLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRecordFieldPositionLoop5Result {
    MlTemporaryReadRecordFieldPositionLoop5Done,
    MlTemporaryReadRecordFieldPositionLoop5Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_field_position_loop5(
    fields: &[Vec<f64>],
    mut index: f64,
) -> self::MlTemporaryReadRecordFieldPositionLoop5Result {
    loop {
        return if index < (fields.len() as f64) {
            if (self::accept_record_field_name(&(self::read_required_record_name(fields, index)))) {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            {
                self::MlTemporaryReadRecordFieldPositionLoop5Result::MlTemporaryReadRecordFieldPositionLoop5Return(-1f64)
            }
        } else {
            self::MlTemporaryReadRecordFieldPositionLoop5Result::MlTemporaryReadRecordFieldPositionLoop5Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_sorted_field_positions_loop10(
    fields: &[Vec<f64>],
    sorted: &[f64],
    mut merged: Vec<f64>,
    mut middle: f64,
    mut end: f64,
    mut left: f64,
    mut right: f64,
) -> Vec<f64> {
    loop {
        if (left < middle) || (right < end) {
            if (right >= end)
                || ((left < middle)
                    && ((self::compare_record_field_names(
                        &(self::read_required_record_name(
                            fields,
                            self::read_required_number_fact(sorted, left),
                        )),
                        &(self::read_required_record_name(
                            fields,
                            self::read_required_number_fact(sorted, right),
                        )),
                    )) <= 0f64))
            {
                {
                    let merged_2 = {
                        let (ml_append_right, ml_append_left) =
                            (vec![self::read_required_number_fact(sorted, left)], merged);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let left_2 = left + 1f64;
                        {
                            (merged, left) = (merged_2, left_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let merged_3 = {
                        let (ml_append_right, ml_append_left) =
                            (vec![self::read_required_number_fact(sorted, right)], merged);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let right_2 = right + 1f64;
                        {
                            (merged, right) = (merged_3, right_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { merged };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_sorted_field_positions_loop9(
    fields: &[Vec<f64>],
    sorted: &[f64],
    mut width: f64,
    mut merged: Vec<f64>,
    mut start: f64,
) -> Vec<f64> {
    loop {
        if start < (sorted.len() as f64) {
            {
                let middle = if (start + width) < (sorted.len() as f64) {
                    start + width
                } else {
                    sorted.len() as f64
                };
                {
                    let end = if (middle + width) < (sorted.len() as f64) {
                        middle + width
                    } else {
                        sorted.len() as f64
                    };
                    {
                        let left = start;
                        {
                            let right = middle;
                            {
                                let merged_2 = self::source_ml_temporary_read_record_sorted_field_positions_loop10(fields, sorted, merged, middle, end, left, right);
                                {
                                    let start_2 = start + (width * 2f64);
                                    {
                                        (merged, start) = (merged_2, start_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { merged };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_sorted_field_positions_loop8(
    fields: &[Vec<f64>],
    mut sorted: Vec<f64>,
    mut width: f64,
) -> Vec<f64> {
    loop {
        if width < (sorted.len() as f64) {
            {
                let merged = Vec::<f64>::new();
                {
                    let start = 0f64;
                    {
                        let merged_2 =
                            self::source_ml_temporary_read_record_sorted_field_positions_loop9(
                                fields,
                                &(sorted),
                                width,
                                merged,
                                start,
                            );
                        {
                            let sorted_2 = merged_2;
                            {
                                let width_2 = width * 2f64;
                                {
                                    (sorted, width) = (sorted_2, width_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { sorted };
    }
}
