//! Deferred source-derived binding/export policy parity controls.
#[path = "../../src/translation/source_library_binding_rules.rs"]
mod bindings;
#[path = "../../src/translation/source_library_export_rules.rs"]
mod exports;
#[test]
fn actual_binding_and_export_policy_retains_literal_source_roles() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/source-library-constant-functions.lino"
    ))
    .unwrap();
    for row in fixture["policies"].as_array().unwrap() {
        let keyword = row["keyword"].as_str().unwrap();
        let kind = row["kind"].as_str().unwrap();
        let name = row["name"].as_str().unwrap();
        let callable = row["callable"].as_bool().unwrap();
        assert_eq!(
            bindings::read_source_library_declaration_binding_kind(keyword, kind, name, callable),
            row["binding"].as_str().unwrap()
        );
        assert_eq!(
            exports::accept_source_library_function_export(keyword, kind, name, callable),
            row["exported"].as_bool().unwrap()
        );
    }
    for row in fixture["headNames"].as_array().unwrap() {
        assert_eq!(
            bindings::accept_java_script_source_function_head_name(row["kind"].as_str().unwrap()),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in fixture["expressionTerms"].as_array().unwrap() {
        assert_eq!(
            bindings::test_java_script_source_function_expression_term(
                row["term"].as_str().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in fixture["keywordRoles"].as_array().unwrap() {
        assert_eq!(
            bindings::test_java_script_source_keyword_role(
                row["identifier"].as_bool().unwrap(),
                row["keyword"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in fixture["functionScopeForms"].as_array().unwrap() {
        assert_eq!(
            bindings::read_java_script_source_function_scope_form(row["term"].as_str().unwrap()),
            row["expected"].as_str().unwrap()
        );
    }
    for row in fixture["simpleParameterLists"].as_array().unwrap() {
        assert_eq!(
            bindings::accept_java_script_source_simple_parameter_list(
                &row["terms"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|term| term.as_str().unwrap().to_string())
                    .collect::<Vec<_>>()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in fixture["variableRedeclarations"].as_array().unwrap() {
        assert_eq!(
            bindings::test_java_script_source_variable_redeclaration(
                row["kind"].as_str().unwrap(),
                row["existing"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in fixture["parameterVariableDependencies"].as_array().unwrap() {
        assert_eq!(
            bindings::test_java_script_source_parameter_variable_dependency(
                row["first"].as_str().unwrap(),
                row["second"].as_str().unwrap(),
                row["bodyParent"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in fixture["wordBindings"]
        .as_array()
        .unwrap()
        .iter()
        .chain(fixture["parameterEnvironments"].as_array().unwrap())
    {
        let program = meta_language::analyze_program(
            row["source"].as_str().unwrap(),
            "JavaScript",
            meta_language::ProgramProjectContext::default(),
        )
        .unwrap();
        let expected = row["bindings"].as_array().unwrap();
        assert_eq!(program.bindings().len(), expected.len());
        for (binding, expected) in program.bindings().iter().zip(expected) {
            assert_eq!(binding.name(), expected[0].as_str().unwrap());
            assert_eq!(binding.kind(), expected[1].as_str().unwrap());
            assert_eq!(
                binding.references().len(),
                expected[2].as_u64().unwrap() as usize
            );
        }
        assert_eq!(
            program
                .unresolved_references()
                .iter()
                .map(|reference| reference.name())
                .collect::<Vec<_>>(),
            row["unresolved"]
                .as_array()
                .unwrap()
                .iter()
                .map(|name| name.as_str().unwrap())
                .collect::<Vec<_>>()
        );
        if let Some(count) = row.get("parameterScopeCount") {
            assert_eq!(
                program
                    .bindings()
                    .iter()
                    .filter(|binding| binding.kind() == "parameter")
                    .map(|binding| binding.scope())
                    .collect::<std::collections::BTreeSet<_>>()
                    .len(),
                count.as_u64().unwrap() as usize
            );
        }
        if let Some(rename) = row.get("rename") {
            let binding = program
                .bindings()
                .iter()
                .find(|binding| {
                    binding.name() == rename["name"].as_str().unwrap()
                        && binding.kind() == rename["kind"].as_str().unwrap()
                })
                .unwrap();
            assert_eq!(
                program
                    .rename_binding(binding.id(), rename["replacement"].as_str().unwrap())
                    .unwrap()
                    .source(),
                rename["expected"].as_str().unwrap()
            );
        }
        for rename in row
            .get("dependentRenames")
            .and_then(|rows| rows.as_array())
            .into_iter()
            .flatten()
        {
            let binding = program
                .bindings()
                .iter()
                .find(|binding| {
                    binding.name() == rename["name"].as_str().unwrap()
                        && binding.kind() == rename["kind"].as_str().unwrap()
                })
                .unwrap();
            let error = program
                .rename_binding(binding.id(), "renamedValue")
                .unwrap_err();
            assert!(matches!(
                &error,
                meta_language::ProgramRepresentationError::BindingDependency { .. }
            ));
            assert!(error.to_string().contains("parameter/var initialization"));
        }
    }
    for row in fixture["wordRenames"].as_array().unwrap() {
        let program = meta_language::analyze_program(
            row["source"].as_str().unwrap(),
            "JavaScript",
            meta_language::ProgramProjectContext::default(),
        )
        .unwrap();
        let binding = program
            .bindings()
            .iter()
            .find(|binding| binding.name() == row["binding"].as_str().unwrap())
            .unwrap();
        assert_eq!(
            program
                .rename_binding(binding.id(), row["replacement"].as_str().unwrap())
                .unwrap()
                .source(),
            row["expected"].as_str().unwrap()
        );
    }
    for replacement in fixture["reservedRenames"].as_array().unwrap() {
        let program = meta_language::analyze_program(
            "export function inspect(value){return value;}",
            "JavaScript",
            meta_language::ProgramProjectContext::default(),
        )
        .unwrap();
        let binding = program
            .bindings()
            .iter()
            .find(|binding| binding.name() == "value")
            .unwrap();
        assert!(
            program
                .rename_binding(binding.id(), replacement.as_str().unwrap())
                .is_err()
        );
    }
    for row in fixture["declarationOffsets"].as_array().unwrap() {
        assert_eq!(
            bindings::read_source_library_function_declaration_offset(
                row["first"].as_str().unwrap(),
                row["second"].as_str().unwrap(),
                row["lineBreak"].as_bool().unwrap()
            ),
            row["expected"].as_i64().unwrap() as f64
        );
    }
    for row in fixture["anonymousBindings"]
        .as_array()
        .unwrap()
        .iter()
        .chain(fixture["namedBindings"].as_array().unwrap())
    {
        let program = meta_language::analyze_program(
            row["source"].as_str().unwrap(),
            "JavaScript",
            meta_language::ProgramProjectContext::default(),
        )
        .unwrap();
        let expected = row["bindings"].as_array().unwrap();
        assert_eq!(program.bindings().len(), expected.len());
        for (binding, expected) in program.bindings().iter().zip(expected) {
            assert_eq!(binding.name(), expected[0].as_str().unwrap());
            assert_eq!(binding.kind(), expected[1].as_str().unwrap());
            assert_eq!(
                binding.references().len(),
                expected[2].as_u64().unwrap() as usize
            );
            if binding.kind() == "parameter" {
                assert_ne!(binding.scope(), "scope:0");
            }
            if binding.kind() == "function" {
                assert_ne!(binding.scope(), "scope:0");
            }
        }
        if let Some(unresolved) = row.get("unresolved") {
            assert_eq!(
                program
                    .unresolved_references()
                    .iter()
                    .map(|reference| reference.name())
                    .collect::<Vec<_>>(),
                unresolved
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|name| name.as_str().unwrap())
                    .collect::<Vec<_>>()
            );
        }
    }
}
