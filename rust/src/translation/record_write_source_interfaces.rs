// Generated from actual checked source function signatures and target mappings.
// UTF8 source/schema fragments only; runtime text retains its UTF16 interface.
use crate::translation::record_write_rules as implementation;
pub(crate) fn accept_record_field_assignment_operator(source_argument_0: &str) -> bool {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    implementation::accept_record_field_assignment_operator(&native_argument_0)
}
pub(crate) fn read_record_field_write_identity(
    source_argument_0: &str,
    source_argument_1: &str,
) -> String {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    let native_argument_1 = implementation::ml_string::Text::from_text(source_argument_1);
    let value =
        implementation::read_record_field_write_identity(&native_argument_0, &native_argument_1);
    String::from_utf16(&value.0)
        .expect("a native source operation must preserve Unicode scalar spelling")
}
pub(crate) fn read_record_field_write_operator(source_argument_0: &str) -> String {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    let value = implementation::read_record_field_write_operator(&native_argument_0);
    String::from_utf16(&value.0)
        .expect("a native source operation must preserve Unicode scalar spelling")
}
pub(crate) fn read_record_field_write_name(source_argument_0: &str) -> String {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    let value = implementation::read_record_field_write_name(&native_argument_0);
    String::from_utf16(&value.0)
        .expect("a native source operation must preserve Unicode scalar spelling")
}
pub(crate) fn read_record_field_write_argument_count(source_argument_0: &str) -> f64 {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    implementation::read_record_field_write_argument_count(&native_argument_0)
}
pub(crate) fn read_record_field_write_suffix(
    source_argument_0: f64,
    source_argument_1: &str,
    source_argument_2: &[String],
) -> f64 {
    let native_argument_1 = implementation::ml_string::Text::from_text(source_argument_1);
    let native_argument_2 = source_argument_2
        .iter()
        .map(|value| implementation::ml_string::Text::from_text(value))
        .collect::<Vec<_>>();
    implementation::read_record_field_write_suffix(
        source_argument_0,
        &native_argument_1,
        &native_argument_2,
    )
}
pub(crate) fn accept_linked_record_field_write(
    source_argument_0: &str,
    source_argument_1: &str,
    source_argument_2: &str,
    source_argument_3: bool,
) -> bool {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    let native_argument_1 = implementation::ml_string::Text::from_text(source_argument_1);
    let native_argument_2 = implementation::ml_string::Text::from_text(source_argument_2);
    implementation::accept_linked_record_field_write(
        &native_argument_0,
        &native_argument_1,
        &native_argument_2,
        source_argument_3,
    )
}
pub(crate) fn read_record_field_write_refusal(
    source_argument_0: &str,
    source_argument_1: &str,
    source_argument_2: bool,
    source_argument_3: bool,
    source_argument_4: bool,
    source_argument_5: &str,
) -> String {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    let native_argument_1 = implementation::ml_string::Text::from_text(source_argument_1);
    let native_argument_5 = implementation::ml_string::Text::from_text(source_argument_5);
    let value = implementation::read_record_field_write_refusal(
        &native_argument_0,
        &native_argument_1,
        source_argument_2,
        source_argument_3,
        source_argument_4,
        &native_argument_5,
    );
    String::from_utf16(&value.0)
        .expect("a native source operation must preserve Unicode scalar spelling")
}
pub(crate) fn read_record_compound_binary_operation(
    source_argument_0: &str,
    source_argument_1: &str,
    source_argument_2: &str,
) -> String {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    let native_argument_1 = implementation::ml_string::Text::from_text(source_argument_1);
    let native_argument_2 = implementation::ml_string::Text::from_text(source_argument_2);
    let value = implementation::read_record_compound_binary_operation(
        &native_argument_0,
        &native_argument_1,
        &native_argument_2,
    );
    String::from_utf16(&value.0)
        .expect("a native source operation must preserve Unicode scalar spelling")
}
pub(crate) fn read_record_field_source_write_refusal(
    source_argument_0: &str,
    source_argument_1: &str,
    source_argument_2: &str,
    source_argument_3: bool,
    source_argument_4: bool,
    source_argument_5: bool,
    source_argument_6: &str,
) -> String {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    let native_argument_1 = implementation::ml_string::Text::from_text(source_argument_1);
    let native_argument_2 = implementation::ml_string::Text::from_text(source_argument_2);
    let native_argument_6 = implementation::ml_string::Text::from_text(source_argument_6);
    let value = implementation::read_record_field_source_write_refusal(
        &native_argument_0,
        &native_argument_1,
        &native_argument_2,
        source_argument_3,
        source_argument_4,
        source_argument_5,
        &native_argument_6,
    );
    String::from_utf16(&value.0)
        .expect("a native source operation must preserve Unicode scalar spelling")
}
pub(crate) fn render_record_field_write(
    source_argument_0: &str,
    source_argument_1: &str,
    source_argument_2: &str,
    source_argument_3: &str,
    source_argument_4: &str,
    source_argument_5: &[String],
    source_argument_6: &str,
) -> String {
    let native_argument_0 = implementation::ml_string::Text::from_text(source_argument_0);
    let native_argument_1 = implementation::ml_string::Text::from_text(source_argument_1);
    let native_argument_2 = implementation::ml_string::Text::from_text(source_argument_2);
    let native_argument_3 = implementation::ml_string::Text::from_text(source_argument_3);
    let native_argument_4 = implementation::ml_string::Text::from_text(source_argument_4);
    let native_argument_5 = source_argument_5
        .iter()
        .map(|value| implementation::ml_string::Text::from_text(value))
        .collect::<Vec<_>>();
    let native_argument_6 = implementation::ml_string::Text::from_text(source_argument_6);
    let value = implementation::render_record_field_write(
        &native_argument_0,
        &native_argument_1,
        &native_argument_2,
        &native_argument_3,
        &native_argument_4,
        &native_argument_5,
        &native_argument_6,
    );
    String::from_utf16(&value.0)
        .expect("a native source operation must preserve Unicode scalar spelling")
}
pub(crate) fn read_record_field_write_statement() -> String {
    let value = implementation::read_record_field_write_statement();
    String::from_utf16(&value.0)
        .expect("a native source operation must preserve Unicode scalar spelling")
}
