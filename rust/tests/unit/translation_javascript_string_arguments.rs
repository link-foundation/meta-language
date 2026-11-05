//! Deferred native policy and real frontend emission controls. No local execution.
#[path = "../../src/translation/javascript_string_argument_rules.rs"]
mod native;
use meta_language::read_structured_data_lino;
#[test]
fn native_argument_plan_retains_every_declared_source_coordinate() {
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/javascript-string-arguments.lino"
    ))
    .unwrap();
    for row in fixture["rows"].as_array().unwrap() {
        let actual = native::plan_java_script_string_arguments(row["count"].as_f64().unwrap());
        let expected = row["expected"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(actual, expected, "{}", row["name"]);
    }
}
#[test]
fn native_frontend_reads_and_emits_every_original_string_argument_case() {
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/javascript-call-source.lino"
    ))
    .unwrap();
    let mut count = 0;
    for row in fixture["rows"].as_array().unwrap() {
        if !row["name"].as_str().unwrap().starts_with("String call ") {
            continue;
        }
        count += 1;
        let surface = meta_language::translation::javascript::parse_javascript(
            row["source"].as_str().unwrap(),
        )
        .unwrap();
        let program = meta_language::translation::check::check_program(&surface).unwrap();
        meta_language::translation::emit_javascript::emit_javascript(&program).unwrap();
        meta_language::translation::emit_rust::emit_rust(&program).unwrap();
    }
    assert_eq!(
        count, 5,
        "all actual new frontend originals must remain included"
    );
}
