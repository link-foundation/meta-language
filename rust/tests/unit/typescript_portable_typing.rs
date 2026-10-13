use meta_language::project_type_script_typing;

#[test]
fn native_typing_projection_matches_shared_javascript_annotation_spans() {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../parity/fixtures/typescript-erasure.lino"),
    )
    .expect("shared TypeScript erasure fixture is available in the test checkout");
    let fixture = meta_language::read_structured_data_lino(&source).unwrap();
    for case in fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|case| case["typing"].is_string())
    {
        assert_eq!(
            project_type_script_typing(case["source"].as_str().unwrap()).unwrap(),
            case["typing"].as_str().unwrap(),
            "{}",
            case["id"]
        );
    }
}

#[test]
fn native_typing_projection_preserves_shared_source_refusals() {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../parity/fixtures/typescript-erasure.lino"),
    )
    .expect("shared TypeScript erasure fixture is available in the test checkout");
    let fixture = meta_language::read_structured_data_lino(&source).unwrap();
    for case in fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|case| case["error"].is_object())
    {
        let error = project_type_script_typing(case["source"].as_str().unwrap())
            .expect_err("unsupported runtime constructs retain their source obligation");
        assert_eq!(
            error.reason,
            case["error"]["reason"].as_str().unwrap(),
            "{}",
            case["id"]
        );
    }
}
