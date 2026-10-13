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

#[test]
fn native_typing_documentation_patterns_preserve_shared_source_and_javascript_values() {
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/typescript-documentation-patterns.lino"
    ))
    .unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let input = case["source"].as_str().unwrap();
        if let Some(expected) = case["expectedError"].as_str() {
            let error = project_type_script_typing(input)
                .expect_err("the original annotation refusal remains explicit");
            assert_eq!(error.reason, expected, "{}", case["id"]);
            continue;
        }
        let code = project_type_script_typing(input).unwrap();
        assert_eq!(
            code,
            case["projectedSource"].as_str().unwrap(),
            "{}",
            case["id"]
        );
        for projection in case["projections"].as_array().unwrap() {
            for pattern in projection["parameterPatterns"].as_array().unwrap() {
                let start =
                    usize::try_from(pattern["sourceSpan"]["start"].as_u64().unwrap()).unwrap();
                let end = usize::try_from(pattern["sourceSpan"]["end"].as_u64().unwrap()).unwrap();
                let original = pattern["sourceText"].as_str().unwrap();
                assert_eq!(&input[start..end], original);
                assert!(code.contains(original));
                assert!(
                    !pattern["documentationText"]
                        .as_str()
                        .unwrap()
                        .contains("*/")
                );
            }
        }
        assert_eq!(code.matches("\r\n").count(), input.matches("\r\n").count());
        let script = format!(
            "{code}\nconsole.log(JSON.stringify({}));",
            case["invocation"].as_str().unwrap()
        );
        let output = std::process::Command::new("node")
            .args(["--eval", &script])
            .output()
            .expect("the JavaScript oracle is available in native CI");
        assert!(
            output.status.success(),
            "{}: {}",
            case["id"],
            String::from_utf8_lossy(&output.stderr)
        );
        let actual: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(actual, case["expectedValue"], "{}", case["id"]);
    }
}
