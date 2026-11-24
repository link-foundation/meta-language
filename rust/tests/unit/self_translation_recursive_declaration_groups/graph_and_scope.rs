//! Complete original graph and deferred-scope controls.
use super::*;

// Independent finite transitive closure is deliberately unlike the production
// generated two-pass walk. It is only the native test oracle, never host policy.
fn graph_case(mask: u32) -> (Vec<usize>, Vec<usize>, Vec<usize>, [usize; 3]) {
    let mut adjacency = [[false; 3]; 3];
    for (source, row) in adjacency.iter_mut().enumerate() {
        for (target, edge) in row.iter_mut().enumerate() {
            *edge = mask & (1 << (source * 3 + target)) != 0;
        }
    }
    let mut reachable = adjacency;
    for (index, row) in reachable.iter_mut().enumerate() {
        row[index] = true;
    }
    for middle in [0, 1, 2] {
        for source in [0, 1, 2] {
            for target in [0, 1, 2] {
                let next = reachable[source][middle] && reachable[middle][target];
                reachable[source][target] |= next;
            }
        }
    }
    let expected = std::array::from_fn(|source| {
        (0..3)
            .find(|target| reachable[source][*target] && reachable[*target][source])
            .unwrap()
    });
    let (mut starts, mut counts, mut targets) = (Vec::new(), Vec::new(), Vec::new());
    for row in adjacency {
        starts.push(targets.len());
        for (target, present) in row.iter().enumerate() {
            if *present {
                targets.push(target);
            }
        }
        counts.push(targets.len() - *starts.last().unwrap());
    }
    (starts, counts, targets, expected)
}
#[test]
fn the_complete_original_shared_algorithm_and_every_three_node_graph_execute_through_both_semantic_archives_in_ci()
 {
    versions();
    let value = policy();
    let closure = &value["sourceClosure"];
    for input in closure["inputs"].as_array().unwrap() {
        let source = fs::read(root().join(input["path"].as_str().unwrap())).unwrap();
        assert_eq!(
            u64::try_from(source.len()).unwrap(),
            input["bytes"].as_u64().unwrap()
        );
        assert_eq!(
            super::issue_195_translation_pairs::sha256(&source),
            input["sourceSha256"].as_str().unwrap()
        );
    }
    let mut source = closure["source"].as_str().unwrap().to_owned();
    let mut expected = String::new();
    for mask in 0..512 {
        let (starts, counts, targets, leaders) = graph_case(mask);
        source.push_str(&format!("\nconst case{mask}=readDeclarationDependencyComponents({starts:?},{counts:?},{targets:?});console.log(case{mask}[0],case{mask}[1],case{mask}[2]);\n"));
        expected.push_str(&format!("{} {} {}\n", leaders[0], leaders[1], leaders[2]));
    }
    source.push_str("\nconsole.log(acceptDeclarationModuleScope(['comment','named function declaration','named import','primitive literal binding'],false));\n");
    expected.push_str("true\n");
    for form in [
        "var declaration",
        "assignment",
        "eval",
        "namespace import",
        "const binding list",
        "class declaration",
        "unknown module form",
    ] {
        let literal = serde_json::to_string(form).unwrap();
        source.push_str(&format!("\nconsole.log(acceptDeclarationModuleScope(['named function declaration',{literal}],false));console.log(acceptDeclarationModuleScope(['named function declaration',{literal}],true));\n"));
        expected.push_str("false\ntrue\n");
    }
    source.push_str("\nconsole.log(acceptDeclarationModuleScope(['empty class declaration'],false));console.log(acceptDeclarationSignatureBinding(true,1));\n");
    expected.push_str("true\ntrue\n");
    for count in [0, 2] {
        source.push_str(&format!("\nconsole.log(acceptDeclarationSignatureBinding(true,{count}));console.log(acceptDeclarationSignatureBinding(false,{count}));\n"));
        expected.push_str("false\nfalse\n");
    }
    source.push_str("\nconsole.log(acceptDeclarationSignatureBinding(false,1));\n");
    expected.push_str("false\n");
    observe_complete_javascript(&source, &expected, "recursive-original-graphs");
    let packet = create_typed_program_links(&source, "JavaScript").unwrap();
    let program = decode_typed_program_links(&packet.network, packet.root).unwrap();
    for restored in archives(&packet.network) {
        assert_eq!(
            decode_typed_program_links(&restored, packet.root).unwrap(),
            program
        );
        compile_complete_and_observe(
            &emit_typed_program_links(&restored, packet.root, "Rust")
                .unwrap()
                .code,
            &expected,
            "recursive-dependency-graphs",
        );
        let code = emit_typed_program_links(&restored, packet.root, "JavaScript")
            .unwrap()
            .code;
        observe_complete_javascript(&code, &expected, "recursive-javascript-graphs");
    }
}

#[test]
fn deferred_class_and_function_value_scope_uses_actual_shared_cst_facts_and_keeps_every_original_sibling_oracle()
 {
    versions();
    for fixture in policy()["deferredModuleFixtures"].as_array().unwrap() {
        let source = fixture["source"].as_str().unwrap();
        let actual = self_translate(source, "JavaScript", "Rust").unwrap();
        let javascript = javascript_bridge(source, "JavaScript", false);
        assert_eq!(
            actual.code,
            javascript["translation"]["code"].as_str().unwrap(),
            "{}",
            fixture["id"]
        );
        let items=actual.items.iter().map(|item|json!({"term":item.term,"start":item.start,"end":item.end,"status":item.status,"reason":item.reason})).collect::<Vec<_>>();
        assert_eq!(
            json!(items),
            javascript["translation"]["items"],
            "{}",
            fixture["id"]
        );
        let statuses = actual
            .items
            .iter()
            .filter(|item| item.term == "function_declaration" || item.term == "export_statement")
            .map(|item| item.status)
            .collect::<Vec<_>>();
        assert_eq!(
            json!(statuses),
            fixture["functionStatuses"],
            "{}",
            fixture["id"]
        );
        let signatures =
            self_translation_signatures(source, "JavaScript", &Default::default()).unwrap();
        assert_eq!(
            serde_json::to_value(&signatures).unwrap(),
            javascript["signatures"]
        );
        assert_eq!(
            json!(
                signatures
                    .iter()
                    .map(meta_language::SelfTranslationSignature::name)
                    .collect::<Vec<_>>()
            ),
            fixture["signatures"],
            "{}",
            fixture["id"]
        );
        assert_eq!(
            self_translate(&actual.code, "Rust", "JavaScript")
                .unwrap()
                .code,
            source
        );
        observe_javascript(
            source,
            &fixture["calls"],
            fixture["expectedLines"].as_str().unwrap(),
            false,
        );
        compile_and_observe(
            &actual.code,
            &fixture["rustCalls"],
            fixture["expectedLines"].as_str().unwrap(),
            "deferred-definition-scope",
        );
        for restored in archives(&LinkNetwork::parse(
            source,
            "JavaScript",
            ParseConfiguration::default(),
        )) {
            assert_eq!(restored.reconstruct_text(), source);
        }
    }
}
