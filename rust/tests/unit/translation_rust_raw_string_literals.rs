//! Public raw-string paths and complete artifact execution, compiled only in CI.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::diagnostics::ErrorKind;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::ir::{Decl, Effect, LitValue, Node, Program};
use meta_language::translation::lexer::{TokenKind, tokenize};
use meta_language::translation::{Language, Span};
use meta_language::{
    BinaryLinksOptions, LinkId, LinkNetwork, LinkType, ParseConfiguration, SubstitutionRule,
    create_typed_program_links, create_typed_program_links_with_rust_runtime_shell_model,
    create_typed_program_links_with_rust_type_namespace_model, decode_link_network_binary,
    decode_typed_program_links, emit_typed_program_links, encode_link_network_binary,
    encode_typed_program_links, read_structured_data_lino, typed_program_links_schema,
};
use serde_json::{Value, json};
use std::fs;
use std::process::Command;

fn fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-raw-string-literals.lino"
    ))
    .unwrap()
}

fn runtime_fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-runtime-shell.lino"
    ))
    .unwrap()
}

fn literal(delimiters: usize, content: &str) -> String {
    let hashes = "#".repeat(delimiters);
    format!("r{hashes}\"{content}\"{hashes}")
}

fn case_literal(case: &Value) -> String {
    literal(
        usize::try_from(case["delimiters"].as_u64().unwrap()).unwrap(),
        case["content"].as_str().unwrap(),
    )
}

fn source_for(literal: &str) -> String {
    format!(
        "\u{feff}// 😀 original source\r\npub fn choose()->String{{String::from({literal})}}fn main(){{println!(\"{{}}\",choose());}}"
    )
}

fn offset(source: &str, spelling: &str) -> usize {
    source[..source.find(spelling).unwrap()]
        .encode_utf16()
        .count()
}

fn archives(network: &LinkNetwork) -> [LinkNetwork; 2] {
    [
        LinkNetwork::from_lino(&network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ]
}

fn value(program: &Program) -> &str {
    let Some(Decl::Fn(function)) = program.declaration("choose") else {
        panic!("actual choose declaration")
    };
    let Node::Lit {
        value: LitValue::Text(value),
    } = &function.body.node
    else {
        panic!("actual string value")
    };
    value
}

fn execute_javascript(code: &str, expected: &str) {
    let directory = TemporaryDirectory::new("raw-string-javascript");
    let file = directory.0.join("program.cjs");
    fs::write(&file, code).unwrap();
    let observed = run(Command::new("node").arg(file));
    assert!(
        observed.ok,
        "JavaScript execution failed\n{}\n{}",
        observed.stdout, observed.stderr
    );
    assert_eq!(observed.stdout, expected);
}

fn execute_runtime_javascript(code: &str, entry: &Value) {
    let directory = TemporaryDirectory::new("raw-runtime-javascript");
    let file = directory.0.join("observations.cjs");
    let script = format!(
        "const assert=require('node:assert/strict'),vm=require('node:vm');const output=[],context=vm.createContext({{console:{{log:value=>output.push(String(value))}}}});vm.runInContext({},context,{{timeout:1000}});assert.equal(output.join('\\n'),{});const calls=[];for(const name of {})context[name]=()=>{{calls.push(name);return null;}};vm.runInContext('main();',context,{{timeout:1000}});assert.deepEqual(calls,{});",
        serde_json::to_string(code).unwrap(),
        entry["output"],
        entry["names"],
        entry["calls"]
    );
    fs::write(&file, script).unwrap();
    let observed = run(Command::new("node").arg(file));
    assert!(
        observed.ok,
        "Runtime call observations failed\n{}\n{}",
        observed.stdout, observed.stderr
    );
}

fn execute_rust(code: &str, expected: &str) {
    let directory = TemporaryDirectory::new("raw-string-native");
    let source = directory.0.join("program.rs");
    let binary = directory
        .0
        .join(format!("program{}", std::env::consts::EXE_SUFFIX));
    // Write and execute the complete original or actual emitted artifact.
    // No function is extracted, rewritten or replaced by a test harness.
    fs::write(&source, code).unwrap();
    let mut build_command = Command::new("rustc");
    build_command.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        build_command.arg("-C").arg(format!("linker={linker}"));
    }
    let compiled = run(build_command.arg(source).arg("-o").arg(&binary));
    assert!(
        compiled.ok,
        "Rust compilation failed\n{}\n{}",
        compiled.stdout, compiled.stderr
    );
    let observed = run(&mut Command::new(&binary));
    assert!(
        observed.ok,
        "Rust execution failed\n{}\n{}",
        observed.stdout, observed.stderr
    );
    assert_eq!(observed.stdout, expected);
}

#[test]
fn zero_through_255_hashes_close_exactly_without_interpreting_backslashes() {
    for delimiters in 0..=255 {
        let content = if delimiters == 0 {
            r"α😀\n\u{0}".to_owned()
        } else {
            format!("α😀\\n\"{}inside", "#".repeat(delimiters - 1))
        };
        let raw = literal(delimiters, &content);
        let source = format!("// 😀\n{raw}#");
        let tokens = tokenize(&source, Language::Rust).unwrap();
        let token = &tokens.tokens[0];
        assert_eq!(token.kind, TokenKind::String);
        assert_eq!(token.value, content);
        assert_eq!(token.raw, raw);
        assert_eq!(token.start, offset(&source, &raw));
        assert_eq!(token.end, token.start + raw.encode_utf16().count());
        assert_eq!(tokens.tokens[1].raw, "#");
        assert_eq!(tokens.tokens[1].start, token.end);
    }
    let tokens = tokenize("r#\"first\"#r#\"second\"#", Language::Rust).unwrap();
    let strings = tokens
        .tokens
        .iter()
        .filter(|token| token.kind == TokenKind::String)
        .map(|token| token.value.as_str())
        .collect::<Vec<_>>();
    assert_eq!(strings, ["first", "second"]);
}

#[test]
fn every_shared_raw_value_keeps_utf16_spans_and_cst_text_binary_bytes() {
    let data = fixture();
    assert_eq!(data["literalCases"].as_array().unwrap().len(), 5);
    for case in data["literalCases"].as_array().unwrap() {
        let raw = case_literal(case);
        let source = source_for(&raw);
        let tokens = tokenize(&source, Language::Rust).unwrap();
        let token = tokens.tokens.iter().find(|token| token.raw == raw).unwrap();
        assert_eq!(token.value, case["value"].as_str().unwrap());
        assert_eq!(token.start, offset(&source, &raw));
        assert_eq!(token.end, token.start + raw.encode_utf16().count());
        let byte_start = source.find(&raw).unwrap();
        assert_eq!(
            &source.as_bytes()[byte_start..byte_start + raw.len()],
            raw.as_bytes()
        );
        let cst = LinkNetwork::parse(&source, "Rust", ParseConfiguration::default());
        for restored in archives(&cst) {
            assert_eq!(restored.reconstruct_text().as_bytes(), source.as_bytes());
        }
    }
}

#[test]
fn public_raw_string_links_execute_and_reimport_complete_emitted_javascript() {
    let data = fixture();
    for case in data["literalCases"].as_array().unwrap() {
        let source = source_for(&case_literal(case));
        let expected = case["value"].as_str().unwrap();
        let packet = create_typed_program_links(&source, "Rust").unwrap();
        for network in archives(&packet.network) {
            assert!(network.links().all(|link| {
                !link
                    .metadata()
                    .definition()
                    .is_some_and(|text| text.contains(&source))
            }));
            let program = decode_typed_program_links(&network, packet.root).unwrap();
            assert_eq!(value(&program), expected);
            let definitions = emit_rust(&program).unwrap().definitions.join("\n\n");
            let replay = create_typed_program_links(
                &format!("{definitions}\nfn main(){{println!(\"{{}}\",choose());}}"),
                "Rust",
            )
            .unwrap();
            assert_eq!(
                value(&decode_typed_program_links(&replay.network, replay.root).unwrap()),
                expected
            );
            let javascript = emit_typed_program_links(&network, packet.root, "JavaScript").unwrap();
            execute_javascript(&javascript.code, &format!("{expected}\n"));
            let from_javascript =
                create_typed_program_links(&javascript.code, "JavaScript").unwrap();
            for restored in archives(&from_javascript.network) {
                execute_javascript(
                    &emit_typed_program_links(&restored, from_javascript.root, "JavaScript")
                        .unwrap()
                        .code,
                    &format!("{expected}\n"),
                );
                let rust =
                    emit_typed_program_links(&restored, from_javascript.root, "Rust").unwrap();
                assert!(
                    tokenize(&rust.code, Language::Rust)
                        .unwrap()
                        .tokens
                        .iter()
                        .any(|token| token.kind == TokenKind::String && token.value == expected)
                );
            }
        }
    }
}

#[test]
fn all_original_and_actual_emitted_raw_programs_execute_their_own_main() {
    for case in fixture()["literalCases"].as_array().unwrap() {
        let source = source_for(&case_literal(case));
        let expected = format!("{}\n", case["value"].as_str().unwrap());
        execute_rust(&source, &expected);
        let packet = create_typed_program_links(&source, "Rust").unwrap();
        let binary = archives(&packet.network).into_iter().nth(1).unwrap();
        execute_rust(
            &emit_typed_program_links(&binary, packet.root, "Rust")
                .unwrap()
                .code,
            &expected,
        );
    }
}

#[test]
fn source_free_raw_value_edits_change_both_targets_and_round_trip_archives() {
    let data = fixture();
    let source = source_for(&case_literal(&data["literalCases"][1]));
    let packet = create_typed_program_links(&source, "Rust").unwrap();
    let binary = archives(&packet.network).into_iter().nth(1).unwrap();
    let mut program = decode_typed_program_links(&binary, packet.root).unwrap();
    let Some(Decl::Fn(function)) = program
        .declarations
        .iter_mut()
        .find(|item| item.full_name() == "choose")
    else {
        panic!("choose")
    };
    let expected = "edited\\n\r\n\0\"#α😀";
    function.body.node = Node::Lit {
        value: LitValue::Text(expected.to_owned()),
    };
    let saved = encode_typed_program_links(&program).unwrap();
    for network in archives(&saved.network) {
        let restored = decode_typed_program_links(&network, saved.root).unwrap();
        assert_eq!(value(&restored), expected);
        assert!(network.links().all(|link| {
            !link
                .metadata()
                .definition()
                .is_some_and(|text| text.contains(&source))
        }));
        execute_javascript(
            &emit_typed_program_links(&network, saved.root, "JavaScript")
                .unwrap()
                .code,
            &format!("{expected}\n"),
        );
        let definitions = emit_rust(&restored).unwrap().definitions.join("\n\n");
        let replay = create_typed_program_links(
            &format!("{definitions}\nfn main(){{println!(\"{{}}\",choose());}}"),
            "Rust",
        )
        .unwrap();
        assert_eq!(
            value(&decode_typed_program_links(&replay.network, replay.root).unwrap()),
            expected
        );
    }
    execute_rust(
        &emit_typed_program_links(&saved.network, saved.root, "Rust")
            .unwrap()
            .code,
        &format!("{expected}\n"),
    );
}

#[test]
fn malformed_raw_tokens_and_isolated_cr_refuse_without_widening_other_prefixes() {
    let data = fixture();
    assert_eq!(data["malformed"].as_array().unwrap().len(), 5);
    for entry in data["malformed"].as_array().unwrap() {
        let literal = entry.as_str().unwrap();
        let error = tokenize(literal, Language::Rust).unwrap_err();
        assert_eq!(error.kind, ErrorKind::Syntax);
        assert!(error.reason.contains("Rust raw string"), "{error}");
        assert!(create_typed_program_links(&source_for(literal), "Rust").is_err());
    }
    for entry in data["isolatedCarriageReturns"].as_array().unwrap() {
        let raw = literal(2, entry.as_str().unwrap());
        let source = source_for(&raw);
        let start = offset(&source, &raw) + 4 + entry.as_str().unwrap().find('\r').unwrap();
        let error = tokenize(&source, Language::Rust).unwrap_err();
        assert_eq!(error.reason, "isolated carriage return in a Rust string");
        assert_eq!(error.span, Some(Span::new(start, start + 1)));
        assert!(create_typed_program_links(&source, "Rust").is_err());
    }
    for entry in data["unchangedPrefixes"].as_array().unwrap() {
        let source = entry.as_str().unwrap();
        assert_eq!(
            tokenize(source, Language::Rust).unwrap().tokens[0].kind,
            TokenKind::Identifier
        );
        assert!(create_typed_program_links(&source_for(source), "Rust").is_err());
    }
    for source in ["r\"value\"suffix", "r#\"value\"##"] {
        assert!(create_typed_program_links(&source_for(source), "Rust").is_err());
    }
    for language in [Language::JavaScript, Language::Lean, Language::Rocq] {
        let tokens = tokenize("r\"value\"", language).unwrap();
        assert_eq!(tokens.tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens.tokens[0].value, "r");
        assert_eq!(tokens.tokens[1].raw, "\"value\"");
    }
    assert!(
        create_typed_program_links_with_rust_type_namespace_model(
            &source_for("r\"value\""),
            "Rust",
            "resolved Rust type namespace 1",
            None,
            None
        )
        .is_err()
    );
}

#[test]
fn shared_invalid_surrogate_vectors_remain_outside_the_native_str_input_domain() {
    // The JS host alone can construct these unpaired UTF-16 source strings.
    // Native public import accepts &str, so no private raw kernel is exposed
    // to manufacture a second input domain. These are domain checks, not
    // observations of native raw-decoder rejection on otherwise valid input.
    let data = fixture();
    assert_eq!(data["nonScalarUnits"].as_array().unwrap().len(), 5);
    for units in data["nonScalarUnits"].as_array().unwrap() {
        let units = units
            .as_array()
            .unwrap()
            .iter()
            .map(|unit| u16::try_from(unit.as_u64().unwrap()).unwrap())
            .collect::<Vec<_>>();
        assert!(String::from_utf16(&units).is_err());
    }
}

fn field(network: &LinkNetwork, parent: LinkId, name: &str) -> Option<(LinkId, LinkId)> {
    let schema = typed_program_links_schema();
    let label = schema["fields"][name].as_str().map_or_else(
        || {
            name.chars()
                .flat_map(|character| {
                    if character.is_ascii_uppercase() {
                        vec![' ', character.to_ascii_lowercase()]
                    } else {
                        vec![character]
                    }
                })
                .collect::<String>()
        },
        str::to_owned,
    );
    network.links().find_map(|link| match link.references() {
        [owner, key, child]
            if *owner == parent
                && link.metadata().link_type() == Some(LinkType::Field)
                && network.link(*key).and_then(|key| key.metadata().term())
                    == Some(label.as_str()) =>
        {
            Some((link.id(), *child))
        }
        _ => None,
    })
}

fn scalar_field<'a>(network: &'a LinkNetwork, parent: LinkId, name: &str) -> &'a str {
    network
        .link(field(network, parent, name).unwrap().1)
        .unwrap()
        .metadata()
        .definition()
        .unwrap()
}

fn operations(network: &LinkNetwork, root: LinkId) -> Vec<LinkId> {
    let mut result = Vec::new();
    let mut pending = vec![root];
    while let Some(node) = pending.pop() {
        if scalar_field(network, node, "kind") == "source operation" {
            assert!(!result.contains(&node));
            result.push(node);
            let parts = field(network, node, "parts").unwrap().1;
            pending.extend_from_slice(network.link(parts).unwrap().references());
        }
    }
    result
}

fn runtime_calls(program: &Program) -> Vec<&str> {
    program
        .main
        .as_ref()
        .unwrap()
        .effects
        .iter()
        .map(|effect| {
            let Effect::Let { value, .. } = effect else {
                panic!("owned worker binding")
            };
            let Node::Call { func, .. } = &value.node else {
                panic!("owned worker call")
            };
            func.as_str()
        })
        .collect()
}

fn check_runtime_owners(network: &LinkNetwork, root: LinkId, source: &str, raws: &[&str]) {
    let program = decode_typed_program_links(network, root).unwrap();
    let proof = program.source_runtime_shell.as_ref().unwrap();
    assert_eq!(
        proof["span"]["end"].as_u64().unwrap(),
        u64::try_from(source.encode_utf16().count()).unwrap()
    );
    let tokens = proof["tokens"].as_array().unwrap();
    assert_eq!(tokens.last().unwrap()["span"]["end"], proof["span"]["end"]);
    let body = network.link(root).unwrap().references()[0];
    let proof_id = field(network, body, "sourceRuntimeShell").unwrap().1;
    let tokens_id = field(network, proof_id, "tokens").unwrap().1;
    let token_ids = network.link(tokens_id).unwrap().references();
    assert_eq!(tokens.len(), token_ids.len());
    let mapping = field(network, body, "declarations").unwrap().1;
    let declarations = network
        .link(mapping)
        .unwrap()
        .references()
        .iter()
        .map(|pair| network.link(*pair).unwrap().references()[1])
        .collect::<Vec<_>>();
    let nodes = operations(network, field(network, proof_id, "operations").unwrap().1);
    for raw in raws {
        let start = offset(source, raw);
        let index = tokens
            .iter()
            .position(|token| {
                token["kind"] == "string"
                    && token["span"]["start"].as_u64() == Some(u64::try_from(start).unwrap())
            })
            .unwrap();
        assert_eq!(
            tokens[index]["span"]["end"].as_u64().unwrap(),
            u64::try_from(start + raw.encode_utf16().count()).unwrap()
        );
        let role = if raw.contains("--ml-check-theorems") {
            "standard argument flag comparison"
        } else {
            "standard thread success requirement"
        };
        let owner = nodes
            .iter()
            .find(|node| scalar_field(network, **node, "op") == role)
            .unwrap();
        let parts = field(network, *owner, "parts").unwrap().1;
        assert!(
            network
                .link(parts)
                .unwrap()
                .references()
                .contains(&token_ids[index])
        );
    }
    let mut declaration_count = 0;
    let mut binding_count = 0;
    for token in token_ids {
        if let Some((_, declaration)) = field(network, *token, "declaration") {
            assert!(declarations.contains(&declaration));
            declaration_count += 1;
        }
        if let Some((_, binding)) = field(network, *token, "binding") {
            assert!(token_ids.contains(&binding));
            binding_count += 1;
        }
    }
    assert!(declaration_count > 0 && binding_count > 0);
    let assumptions = field(network, proof_id, "assumptions").unwrap().1;
    let assumptions = network.link(assumptions).unwrap().references();
    assert_eq!(assumptions.len(), 4);
    for assumption in assumptions {
        assert!(nodes.contains(&field(network, *assumption, "operationNode").unwrap().1));
    }
}

fn clone_record(network: &mut LinkNetwork, original: LinkId) -> LinkId {
    let link = network.link(original).unwrap();
    let metadata = link.metadata().clone();
    let references = link.references().to_vec();
    let fields = network
        .links()
        .filter_map(|link| match link.references() {
            [owner, label, child]
                if *owner == original && link.metadata().link_type() == Some(LinkType::Field) =>
            {
                Some((
                    network
                        .link(*label)
                        .unwrap()
                        .metadata()
                        .term()
                        .unwrap()
                        .to_owned(),
                    *child,
                ))
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    let cloned = network.insert_dynamic_link(&references, metadata);
    for (label, child) in fields {
        network.insert_field(cloned, &label, child);
    }
    cloned
}

fn forge_owner(network: &mut LinkNetwork, root: LinkId, role: &str) {
    let body = network.link(root).unwrap().references()[0];
    let proof = field(network, body, "sourceRuntimeShell").unwrap().1;
    let relation = if role == "operationNode" {
        let assumptions = field(network, proof, "assumptions").unwrap().1;
        let first = network.link(assumptions).unwrap().references()[0];
        field(network, first, role).unwrap()
    } else {
        let tokens = field(network, proof, "tokens").unwrap().1;
        network
            .link(tokens)
            .unwrap()
            .references()
            .iter()
            .find_map(|token| field(network, *token, role))
            .unwrap()
    };
    let references: [LinkId; 3] = network
        .link(relation.0)
        .unwrap()
        .references()
        .try_into()
        .unwrap();
    let cloned = clone_record(network, relation.1);
    assert_eq!(
        network
            .apply_substitution(&SubstitutionRule::new(
                references,
                [references[0], references[1], cloned]
            ))
            .updated()
            .len(),
        1
    );
}

#[test]
fn raw_fixed_runtime_literals_retain_owned_operations_and_reimport_complete_actual_artifacts() {
    let data = runtime_fixture();
    let model = data["model"].as_str().unwrap();
    let entry = data["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["id"] == "ordered-repeated")
        .unwrap();
    assert_eq!(entry["output"], "");
    let expected_calls = entry["calls"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect::<Vec<_>>();
    let replacements = [
        ("\"--ml-check-theorems\"", "r\"--ml-check-theorems\""),
        (
            "\"spawn the program thread\"",
            "r#\"spawn the program thread\"#",
        ),
    ];
    for selected in [
        vec![replacements[0]],
        vec![replacements[1]],
        replacements.to_vec(),
    ] {
        let mut source = entry["fullEmittedRust"].as_str().unwrap().to_owned();
        for (before, after) in &selected {
            assert!(source.contains(before));
            source = source.replacen(before, after, 1);
        }
        let packet =
            create_typed_program_links_with_rust_runtime_shell_model(&source, "Rust", model, None)
                .unwrap();
        let raws = selected.iter().map(|(_, after)| *after).collect::<Vec<_>>();
        execute_rust(&source, "");
        for network in archives(&packet.network) {
            check_runtime_owners(&network, packet.root, &source, &raws);
            let program = decode_typed_program_links(&network, packet.root).unwrap();
            assert_eq!(runtime_calls(&program), expected_calls);
            execute_runtime_javascript(
                &emit_typed_program_links(&network, packet.root, "JavaScript")
                    .unwrap()
                    .code,
                entry,
            );
            let rust = emit_typed_program_links(&network, packet.root, "Rust").unwrap();
            assert_eq!(rust.assumptions.len(), 4);
            let replay = create_typed_program_links_with_rust_runtime_shell_model(
                &rust.code, "Rust", model, None,
            )
            .unwrap();
            for restored in archives(&replay.network) {
                let rechecked = decode_typed_program_links(&restored, replay.root).unwrap();
                assert_eq!(runtime_calls(&rechecked), expected_calls);
                assert_eq!(
                    rechecked.source_runtime_shell.as_ref().unwrap()["span"]["end"]
                        .as_u64()
                        .unwrap(),
                    u64::try_from(rust.code.encode_utf16().count()).unwrap()
                );
                assert!(encode_typed_program_links(&rechecked).is_ok());
            }
        }
        execute_rust(
            &emit_typed_program_links(&packet.network, packet.root, "Rust")
                .unwrap()
                .code,
            "",
        );
        for (before, after) in [
            ("--ml-check-theorems", "--changed-check-theorems"),
            ("spawn the program thread", "changed thread expectation"),
        ] {
            let changed = source.replacen(before, after, 1);
            assert_ne!(changed, source);
            assert!(
                create_typed_program_links_with_rust_runtime_shell_model(
                    &changed, "Rust", model, None
                )
                .is_err()
            );
        }
        let mut changed = decode_typed_program_links(&packet.network, packet.root).unwrap();
        let token = changed.source_runtime_shell.as_mut().unwrap()["tokens"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|token| token["kind"] == "string")
            .unwrap();
        token["value"] = json!("changed fact");
        assert!(encode_typed_program_links(&changed).is_err());
        assert!(emit_rust(&changed).is_err());
        for role in ["declaration", "binding", "operationNode"] {
            let mut forged = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
            forge_owner(&mut forged, packet.root, role);
            for restored in archives(&forged) {
                assert!(decode_typed_program_links(&restored, packet.root).is_err());
                for target in ["Rust", "JavaScript"] {
                    assert!(emit_typed_program_links(&restored, packet.root, target).is_err());
                }
            }
        }
    }
}

// Opaque character token controls. Native execution belongs exclusively to CI.
#[test]
fn opaque_rust_character_tokens_preserve_string_values_and_absolute_utf16_spans() {
    let prefix = "// 😀 original UTF16 source\r\n";
    for (literal, kind, expected) in [
        ("'{'", TokenKind::Character, "123"),
        ("'}'", TokenKind::Character, "125"),
        ("'\"'", TokenKind::Character, "34"),
        ("'α'", TokenKind::Character, "945"),
        ("'😀'", TokenKind::Character, "128512"),
        (r"'\u{1_F600___}'", TokenKind::Character, "128512"),
        (r"'\x7F'", TokenKind::Character, "127"),
        (r"'\''", TokenKind::Character, "39"),
        ("b'}'", TokenKind::ByteCharacter, "125"),
        (r"b'\xFF'", TokenKind::ByteCharacter, "255"),
    ] {
        let source = format!("{prefix}{literal} + following");
        let result = tokenize(&source, Language::Rust).unwrap();
        let token = &result.tokens[0];
        assert_eq!(token.kind, kind, "{literal}");
        assert_eq!(token.value, expected, "{literal}");
        assert_eq!(token.raw, literal);
        assert_eq!(
            token.span(),
            Span::new(
                prefix.encode_utf16().count(),
                prefix.encode_utf16().count() + literal.encode_utf16().count()
            )
        );
        assert_eq!(result.comments[0].text, prefix.strip_suffix('\n').unwrap());
        assert_eq!(
            result
                .tokens
                .iter()
                .skip(1)
                .map(|token| token.raw.as_str())
                .collect::<Vec<_>>(),
            ["+", "following", ""]
        );
    }
}

#[test]
fn character_tokens_protect_original_delimiters_comments_and_complete_owned_rust_source() {
    let source =
        "// 😀\nfn main(){ let a='{'; let b=b'}'; let c='/'; let d='*'; let e='\"'; } // outside";
    let result = tokenize(source, Language::Rust).unwrap();
    assert_eq!(
        result
            .tokens
            .iter()
            .filter(|token| matches!(token.kind, TokenKind::Character | TokenKind::ByteCharacter))
            .map(|token| token.raw.as_str())
            .collect::<Vec<_>>(),
        ["'{'", "b'}'", "'/'", "'*'", "'\"'"]
    );
    assert_eq!(
        result
            .tokens
            .iter()
            .filter(
                |token| token.kind == TokenKind::Punct && matches!(token.value.as_str(), "{" | "}")
            )
            .map(|token| token.value.as_str())
            .collect::<Vec<_>>(),
        ["{", "}"]
    );
    assert_eq!(
        result
            .comments
            .iter()
            .map(|comment| comment.text.as_str())
            .collect::<Vec<_>>(),
        ["// 😀", "// outside"]
    );
    let original = "fn a(){m!('\"')} //\"\ntype A = _;";
    let result = tokenize(original, Language::Rust).unwrap();
    assert_eq!(
        result
            .tokens
            .iter()
            .filter(|token| token.kind == TokenKind::Character)
            .map(|token| token.raw.as_str())
            .collect::<Vec<_>>(),
        ["'\"'"]
    );
    assert_eq!(result.comments[0].text, "//\"");
    assert_eq!(
        result
            .tokens
            .iter()
            .rev()
            .take(6)
            .map(|token| token.raw.as_str())
            .collect::<Vec<_>>(),
        ["", ";", "_", "=", "A", "type"]
    );
    let owned = include_str!("../../src/translation/js_number.rs");
    let result = tokenize(owned, Language::Rust).unwrap();
    assert_eq!(
        result
            .tokens
            .iter()
            .filter(|token| matches!(token.kind, TokenKind::Character | TokenKind::ByteCharacter))
            .map(|token| (token.kind, token.value.as_str(), token.raw.as_str()))
            .collect::<Vec<_>>(),
        [
            (TokenKind::Character, "53", "'5'"),
            (TokenKind::ByteCharacter, "48", "b'0'"),
            (TokenKind::ByteCharacter, "48", "b'0'"),
            (TokenKind::Character, "48", "'0'"),
            (TokenKind::Character, "45", "'-'"),
            (TokenKind::Character, "43", "'+'"),
            (TokenKind::ByteCharacter, "57", "b'9'"),
            (TokenKind::ByteCharacter, "48", "b'0'"),
            (TokenKind::Character, "101", "'e'"),
            (TokenKind::Character, "46", "'.'"),
            (TokenKind::Character, "48", "'0'"),
        ]
    );
    let original = meta_language::translation::lexer::Source::new(owned);
    for token in &result.tokens {
        assert_eq!(original.slice(token.start, token.end), token.raw);
    }
    assert_eq!(
        result.tokens.last().unwrap().end,
        owned.encode_utf16().count()
    );
}

#[test]
fn rust_character_selection_preserves_lifetimes_labels_strings_comments_and_foreign_paths() {
    let lifetime = "fn keep<'a>(value: &'a str) -> &'a str { value }";
    let label = "fn run() { 'outer: loop { break 'outer; } }";
    for (source, expected) in [
        (
            lifetime,
            vec![
                "fn", "keep", "<", "'", "a", ">", "(", "value", ":", "&", "'", "a", "str", ")",
                "->", "&", "'", "a", "str", "{", "value", "}", "",
            ],
        ),
        (
            label,
            vec![
                "fn", "run", "(", ")", "{", "'", "outer", ":", "loop", "{", "break", "'", "outer",
                ";", "}", "}", "",
            ],
        ),
    ] {
        let result = tokenize(source, Language::Rust).unwrap();
        assert_eq!(
            result
                .tokens
                .iter()
                .map(|token| token.raw.as_str())
                .collect::<Vec<_>>(),
            expected
        );
        assert!(
            result.tokens.iter().all(|token| !matches!(
                token.kind,
                TokenKind::Character | TokenKind::ByteCharacter
            ))
        );
        let mut end = 0;
        for token in result
            .tokens
            .iter()
            .filter(|token| token.kind != TokenKind::Eof)
        {
            let start = end + source[end..].find(&token.raw).unwrap();
            assert_eq!(token.span(), Span::new(start, start + token.raw.len()));
            end = start + token.raw.len();
        }
        assert!(result.comments.is_empty());
    }
    let source = "\"'{' b'x'\" r##\"'}' \" // /* b'y'\"## // 'z'\n/* b'w' */ value";
    let result = tokenize(source, Language::Rust).unwrap();
    assert_eq!(
        result
            .tokens
            .iter()
            .map(|token| (token.kind, token.value.as_str()))
            .collect::<Vec<_>>(),
        [
            (TokenKind::String, "'{' b'x'"),
            (TokenKind::String, "'}' \" // /* b'y'"),
            (TokenKind::Identifier, "value"),
            (TokenKind::Eof, ""),
        ]
    );
    assert_eq!(
        result
            .comments
            .iter()
            .map(|comment| comment.text.as_str())
            .collect::<Vec<_>>(),
        ["// 'z'", "/* b'w' */"]
    );
    for language in [Language::Lean, Language::Rocq] {
        let result = tokenize("'a' b'a' x'", language).unwrap();
        assert_eq!(
            result
                .tokens
                .iter()
                .map(|token| (token.kind, token.value.as_str()))
                .collect::<Vec<_>>(),
            [
                (TokenKind::Punct, "'"),
                (TokenKind::Identifier, "a'"),
                (TokenKind::Identifier, "b'a'"),
                (TokenKind::Identifier, "x'"),
                (TokenKind::Eof, ""),
            ]
        );
    }
    let result = tokenize("'a' b'a' x", Language::JavaScript).unwrap();
    assert_eq!(
        result
            .tokens
            .iter()
            .map(|token| (token.kind, token.value.as_str()))
            .collect::<Vec<_>>(),
        [
            (TokenKind::String, "a"),
            (TokenKind::Identifier, "b"),
            (TokenKind::String, "a"),
            (TokenKind::Identifier, "x"),
            (TokenKind::Eof, ""),
        ]
    );
}

#[test]
fn rust_character_suffixes_and_typed_semantics_remain_explicitly_unsupported() {
    for literal in ["'a'word", "b'a'u8", "'a'_suffix", "'a'λ"] {
        let source = format!("// 😀\n{literal}");
        let error = tokenize(&source, Language::Rust).unwrap_err();
        let suffix = &literal[literal.rfind('\'').unwrap() + 1..];
        let start = offset(&source, suffix);
        assert_eq!(error.kind, ErrorKind::Unsupported);
        assert_eq!(
            error.reason,
            "Rust character literal suffix is outside the supported lexical profile"
        );
        assert_eq!(error.span, Some(Span::new(start, start + 1)));
    }
    for literal in ["'{'", "b'}'", r"'\u{1F600}'", r"b'\xFF'"] {
        let source = format!("fn value()->u32{{{literal}}}");
        let error = meta_language::translation::rust::parse_rust(&source).unwrap_err();
        if literal.starts_with('b') {
            assert_eq!(error.kind, ErrorKind::Syntax);
            assert_eq!(error.reason, "a Rust program needs fn main");
            let end = source.encode_utf16().count();
            assert_eq!(error.span, Some(Span::new(end, end)));
        } else {
            assert_eq!(error.kind, ErrorKind::Unsupported);
            assert_eq!(
                error.reason,
                "character literal: characters are outside the portable core"
            );
            assert_eq!(
                error.span,
                Some(Span::new(
                    offset(&source, literal),
                    offset(&source, literal) + literal.encode_utf16().count()
                ))
            );
        }
        let source = format!("fn choose(x:u8)->u8{{match x{{{literal}=>1,_=>0}}}}");
        let error = meta_language::translation::rust::parse_rust(&source).unwrap_err();
        assert_eq!(error.kind, ErrorKind::Unsupported);
        assert_eq!(
            error.reason,
            "character pattern: characters are outside the portable core"
        );
    }
    for literal in ["''", "b'é'", r"b'\u{41}'", r"'\x80'"] {
        let error = tokenize(literal, Language::Rust).unwrap_err();
        assert_eq!(error.kind, ErrorKind::Syntax);
        let span = error.span.unwrap();
        assert!(span.start <= span.end && span.end <= literal.encode_utf16().count());
    }
    let error = tokenize(r"'\u{D800}'", Language::Rust).unwrap_err();
    assert_eq!(error.kind, ErrorKind::Unsupported);
    assert_eq!(
        error.reason,
        "Rust character literal is not a Unicode scalar value"
    );
}
