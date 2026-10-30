// Generated from actual checked source function signatures and target mappings.
// UTF8 source/schema fragments only; runtime text retains its UTF16 interface.
use crate::translation::rust_record_rules as implementation;
pub(crate) fn plan_rust_record_fields(
    source_argument_0: &[String],
    source_argument_1: &[String],
) -> Vec<f64> {
    let native_argument_0 = source_argument_0
        .iter()
        .map(|value| implementation::ml_string::Text::from_text(value))
        .collect::<Vec<_>>();
    let native_argument_1 = source_argument_1
        .iter()
        .map(|value| implementation::ml_string::Text::from_text(value))
        .collect::<Vec<_>>();
    implementation::plan_rust_record_fields(&native_argument_0, &native_argument_1)
}
pub(crate) fn read_rust_record_field_position(
    source_argument_0: &[String],
    source_argument_1: &str,
) -> f64 {
    let native_argument_0 = source_argument_0
        .iter()
        .map(|value| implementation::ml_string::Text::from_text(value))
        .collect::<Vec<_>>();
    let native_argument_1 = implementation::ml_string::Text::from_text(source_argument_1);
    implementation::read_rust_record_field_position(&native_argument_0, &native_argument_1)
}
pub(crate) fn plan_rust_record_pattern_fields(
    source_argument_0: &[String],
    source_argument_1: &[String],
    source_argument_2: bool,
) -> Vec<f64> {
    let native_argument_0 = source_argument_0
        .iter()
        .map(|value| implementation::ml_string::Text::from_text(value))
        .collect::<Vec<_>>();
    let native_argument_1 = source_argument_1
        .iter()
        .map(|value| implementation::ml_string::Text::from_text(value))
        .collect::<Vec<_>>();
    implementation::plan_rust_record_pattern_fields(
        &native_argument_0,
        &native_argument_1,
        source_argument_2,
    )
}
pub(crate) fn plan_rust_record_update_fields(
    source_argument_0: &[String],
    source_argument_1: &[String],
) -> Vec<f64> {
    let native_argument_0 = source_argument_0
        .iter()
        .map(|value| implementation::ml_string::Text::from_text(value))
        .collect::<Vec<_>>();
    let native_argument_1 = source_argument_1
        .iter()
        .map(|value| implementation::ml_string::Text::from_text(value))
        .collect::<Vec<_>>();
    implementation::plan_rust_record_update_fields(&native_argument_0, &native_argument_1)
}
pub(crate) fn accept_rust_record_update_owner(
    source_argument_0: bool,
    source_argument_1: bool,
    source_argument_2: &str,
    source_argument_3: &str,
    source_argument_4: f64,
    source_argument_5: bool,
) -> bool {
    let native_argument_2 = implementation::ml_string::Text::from_text(source_argument_2);
    let native_argument_3 = implementation::ml_string::Text::from_text(source_argument_3);
    implementation::accept_rust_record_update_owner(
        source_argument_0,
        source_argument_1,
        &native_argument_2,
        &native_argument_3,
        source_argument_4,
        source_argument_5,
    )
}
pub(crate) fn accept_rust_irrefutable_pattern_node(
    source_argument_0: &str,
    source_argument_1: bool,
    source_argument_2: f64,
    source_argument_3: f64,
    source_argument_4: f64,
) -> bool {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    implementation::accept_rust_irrefutable_pattern_node(
        &native_argument_0,
        source_argument_1,
        source_argument_2,
        source_argument_3,
        source_argument_4,
    )
}
pub(crate) fn plan_tuple_pattern_fields(
    source_argument_0: f64,
    source_argument_1: f64,
    source_argument_2: f64,
) -> Vec<f64> {
    implementation::plan_tuple_pattern_fields(
        source_argument_0,
        source_argument_1,
        source_argument_2,
    )
}
pub(crate) fn read_rust_let_else_initializer_error(
    source_argument_0: &[String],
    source_argument_1: &[String],
) -> String {
    let native_argument_0 = source_argument_0
        .iter()
        .map(|value| implementation::ml_string::Text::from_text(value))
        .collect::<Vec<_>>();
    let native_argument_1 = source_argument_1
        .iter()
        .map(|value| implementation::ml_string::Text::from_text(value))
        .collect::<Vec<_>>();
    let value = implementation::read_rust_let_else_initializer_error(
        &native_argument_0,
        &native_argument_1,
    );
    String::from_utf16(&value.0)
        .expect("a native source operation must preserve Unicode scalar spelling")
}
pub(crate) fn read_rust_record_binding_names(
    source_argument_0: &[String],
    source_argument_1: f64,
) -> Vec<String> {
    let native_argument_0 = source_argument_0
        .iter()
        .map(|value| implementation::ml_string::Text::from_text(value))
        .collect::<Vec<_>>();
    implementation::read_rust_record_binding_names(&native_argument_0, source_argument_1)
        .into_iter()
        .map(|value| {
            String::from_utf16(&value.0)
                .expect("a native source operation must preserve Unicode scalar spelling")
        })
        .collect::<Vec<_>>()
}
pub(crate) fn read_rust_record_constructor_name(
    source_argument_0: &str,
    source_argument_1: bool,
    source_argument_2: &[String],
) -> String {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    let native_argument_2 = source_argument_2
        .iter()
        .map(|value| implementation::ml_string::Text::from_text(value))
        .collect::<Vec<_>>();
    let value = implementation::read_rust_record_constructor_name(
        &native_argument_0,
        source_argument_1,
        &native_argument_2,
    );
    String::from_utf16(&value.0)
        .expect("a native source operation must preserve Unicode scalar spelling")
}
pub(crate) fn accept_rust_record_source_name(source_argument_0: &str) -> bool {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    implementation::accept_rust_record_source_name(&native_argument_0)
}
pub(crate) fn accept_rust_keyword_name(source_argument_0: &str) -> bool {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    implementation::accept_rust_keyword_name(&native_argument_0)
}
pub(crate) fn accept_rust_raw_identifier_name(source_argument_0: &str) -> bool {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    implementation::accept_rust_raw_identifier_name(&native_argument_0)
}
pub(crate) fn accept_rust_source_keyword_token(
    source_argument_0: &str,
    source_argument_1: &str,
) -> bool {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    let native_argument_1 = implementation::ml_string::Text::from_text(source_argument_1);
    implementation::accept_rust_source_keyword_token(&native_argument_0, &native_argument_1)
}
pub(crate) fn read_rust_raw_identifier_error(source_argument_0: &str) -> String {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    let value = implementation::read_rust_raw_identifier_error(&native_argument_0);
    String::from_utf16(&value.0)
        .expect("a native source operation must preserve Unicode scalar spelling")
}
pub(crate) fn render_rust_constructor_field_name(source_argument_0: &str) -> String {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    let value = implementation::render_rust_constructor_field_name(&native_argument_0);
    String::from_utf16(&value.0)
        .expect("a native source operation must preserve Unicode scalar spelling")
}
pub(crate) fn accept_rust_named_constructor_fields(
    source_argument_0: &str,
    source_argument_1: &[String],
) -> bool {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    let native_argument_1 = source_argument_1
        .iter()
        .map(|value| implementation::ml_string::Text::from_text(value))
        .collect::<Vec<_>>();
    implementation::accept_rust_named_constructor_fields(&native_argument_0, &native_argument_1)
}
pub(crate) fn render_rust_constructor_fields(
    source_argument_0: &str,
    source_argument_1: &str,
    source_argument_2: &[String],
    source_argument_3: &[String],
) -> String {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    let native_argument_1 = implementation::ml_string::Text::from_text(source_argument_1);
    let native_argument_2 = source_argument_2
        .iter()
        .map(|value| implementation::ml_string::Text::from_text(value))
        .collect::<Vec<_>>();
    let native_argument_3 = source_argument_3
        .iter()
        .map(|value| implementation::ml_string::Text::from_text(value))
        .collect::<Vec<_>>();
    let value = implementation::render_rust_constructor_fields(
        &native_argument_0,
        &native_argument_1,
        &native_argument_2,
        &native_argument_3,
    );
    String::from_utf16(&value.0)
        .expect("a native source operation must preserve Unicode scalar spelling")
}
pub(crate) fn render_rust_function_call(
    source_argument_0: &str,
    source_argument_1: &[String],
) -> String {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    let native_argument_1 = source_argument_1
        .iter()
        .map(|value| implementation::ml_string::Text::from_text(value))
        .collect::<Vec<_>>();
    let value = implementation::render_rust_function_call(&native_argument_0, &native_argument_1);
    String::from_utf16(&value.0)
        .expect("a native source operation must preserve Unicode scalar spelling")
}
pub(crate) fn render_rust_data_field_encoding(source_argument_0: bool) -> String {
    let value = implementation::render_rust_data_field_encoding(source_argument_0);
    String::from_utf16(&value.0)
        .expect("a native source operation must preserve Unicode scalar spelling")
}
