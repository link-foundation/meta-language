#[test]
fn external_scanners_and_semantic_actions_are_executable_link_definitions() {
    let fixture = fixture();
    let suite = Suite::new(&fixture);
    let by_id = |wanted: &str| {
        features(&fixture)
            .iter()
            .find(|feature| feature["id"] == wanted)
            .expect("feature")
    };
    for id in ["layout", "modes", "predicates", "actions"] {
        let grammar = native(text(by_id(id), "listing"));
        let operations: Vec<&FeatureForm> = grammar
            .declarations()
            .scanners
            .iter()
            .flat_map(|scanner| scanner.operations.iter())
            .chain(
                grammar
                    .rules()
                    .iter()
                    .flat_map(|rule| rule.attributes.action.iter().flatten()),
            )
            .collect();
        assert!(
            !operations.is_empty(),
            "{id} has scanner or action operations"
        );
        for operation in operations {
            assert!(
                operation_form(&operation.head).is_some(),
                "{id}: {}",
                operation.head
            );
        }
    }

    // The layout scanner and the sum action are links of the links form, and
    // the grammar read back from those links runs them.
    let layout = by_id("layout");
    let layout_links = render_grammar_links(&native(text(layout, "listing")));
    let scanner_link = Regex::new(
        r"(?m)^\(scanner layout \(tokens newline indent dedent\) \(operations \(if \(valid newline\)",
    )
    .expect("pattern");
    assert!(scanner_link.is_match(&layout_links), "{layout_links}");
    let actions = by_id("actions");
    let action_links = render_grammar_links(&native(text(actions, "listing")));
    assert!(
        action_links.contains("(action (setAttribute value (sumOf terms value))"),
        "{action_links}"
    );
    let run_links = |links: &str, feature: &Value, input: &Value| {
        let grammar = parse_grammar_links(links).expect("the links parse");
        outcome(&suite.parser(&grammar, feature), &json!({ "input": input }))
    };
    let nested = &layout["positive"][0];
    let summed = &actions["positive"][0];
    assert_eq!(
        run_links(&layout_links, layout, &nested["input"]),
        expected(nested)
    );
    assert_eq!(
        run_links(&action_links, actions, &summed["input"]),
        expected(summed)
    );

    // Changing one operation link changes what the parser does: a scanner that
    // never emits indent cannot open a block, and a lower action limit fails
    // the sum.
    assert!(text(nested, "tree").contains("(indent \"\")"));
    let without_indent = layout_links.replacen(
        "(push indents (variable pending)) (emit indent)",
        "(push indents (variable pending)) fail",
        1,
    );
    assert_ne!(without_indent, layout_links);
    assert_eq!(
        run_links(&without_indent, layout, &nested["input"])["rejection"]["reason"],
        "syntax"
    );
    assert!(text(summed, "tree").starts_with("(sum {value=6}"));
    let lower_limit = action_links.replacen("(integer 100)", "(integer 2)", 1);
    assert_ne!(lower_limit, action_links);
    assert_eq!(
        run_links(&lower_limit, actions, &summed["input"])["rejection"]["reason"],
        "syntax"
    );

    // An operation outside its context, or one naming a token the scanner does
    // not declare, is rejected when the grammar loads.
    let out_of_context = "start s\nrule s = normal literal(\"a\") action(emit(s))\n";
    assert_eq!(
        suite.load_failure(out_of_context, &json!({})),
        json!({ "reason": "operation" })
    );
    let undeclared = text(layout, "listing").replacen("emit(dedent)", "emit(outdent)", 1);
    assert_eq!(
        suite.load_failure(&undeclared, layout),
        json!({ "reason": "operation" })
    );

    // The executor interprets those links; it runs no host code: no process,
    // no dynamic library and no unsafe code in its sources.
    let runtime = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/grammar/feature_runtime");
    let host_code = Regex::new(r"\bunsafe\b|process::Command|libloading|dlopen").expect("pattern");
    let mut files = 0;
    // The runtime's submodules sit in directories of their own.
    let mut directories = vec![runtime];
    while let Some(directory) = directories.pop() {
        for entry in std::fs::read_dir(&directory).expect("the runtime directory reads") {
            let path = entry.expect("entry").path();
            if path.is_dir() {
                directories.push(path);
                continue;
            }
            let source = std::fs::read_to_string(
                super::project_data::resolve_project_data_path(&path)
                    .expect("one project data storage format"),
            )
            .expect("the source reads");
            assert!(!host_code.is_match(&source), "{}", path.display());
            files += 1;
        }
    }
    assert!(files > 0);
    observe(
        &["scannersAndActionsExecutableAsLinks"],
        "external_scanners_and_semantic_actions_are_executable_link_definitions",
    );
}

#[test]
fn node_kinds_no_rule_defines_keep_their_source_names_in_the_links_form() {
    let links = "(grammar (start comment))\n\
        (kind documentation_comment (source-names (tree-sitter doc_comment)))\n\
        (rule comment normal (literal %2F%2F))\n";
    let grammar = parse_grammar_links(links).expect("the kind links parse");
    let [kind] = grammar.kinds() else {
        panic!("one kind: {:?}", grammar.kinds());
    };
    assert_eq!(kind.name, "documentation_comment");
    assert_eq!(
        kind.source_names
            .iter()
            .map(|name| (name.source.as_str(), name.name.as_str()))
            .collect::<Vec<_>>(),
        [("tree-sitter", "doc_comment")]
    );
    assert_eq!(render_grammar_links(&grammar), links);
    // The kinds come before the rules, once each, with their source names.
    for (malformed, detail) in [
        (
            "(grammar)\n(rule a normal empty)\n(kind b (source-names (t c)))\n",
            "unexpected link kind",
        ),
        (
            "(grammar)\n(kind b (source-names (t c)))\n(kind b (source-names (t c)))\n(rule a normal empty)\n",
            "kind b is given twice",
        ),
        (
            "(grammar)\n(kind b (doc x))\n(rule a normal empty)\n",
            "expected (source-names ...), not doc",
        ),
        (
            "(grammar)\n(kind b (source-names))\n(rule a normal empty)\n",
            "source-names lists at least one source name",
        ),
    ] {
        let error = parse_grammar_links(malformed).expect_err(malformed);
        assert!(error.to_string().ends_with(detail), "{error}");
    }
}
