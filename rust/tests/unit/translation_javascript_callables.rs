//! Actual callable sources and owned source-free binding edits; run in CI.

use meta_language::translation::emit_lean::emit_lean;
use meta_language::translation::emit_rocq::emit_rocq;
use meta_language::{
    BinaryLinksOptions, LinkId, LinkNetwork, LinkType, TypedProgramLinks,
    create_typed_program_links, decode_link_network_binary, decode_typed_program_links,
    emit_typed_program_links, encode_link_network_binary, percent_encode_links_text,
    read_typed_program_binding_identities, typed_program_links_schema,
};
use std::fs;
use std::process::Command;

fn portable_emission(network: &LinkNetwork, root: LinkId, target: &str) -> Result<String, String> {
    match target {
        "JavaScript" | "Rust" => emit_typed_program_links(network, root, target)
            .map(|emitted| emitted.code)
            .map_err(|failure| failure.to_string()),
        "Lean" | "Rocq" => {
            let program =
                decode_typed_program_links(network, root).map_err(|failure| failure.to_string())?;
            let emitted = if target == "Lean" {
                emit_lean(&program)
            } else {
                emit_rocq(&program)
            };
            emitted
                .map(|emitted| emitted.text)
                .map_err(|failure| failure.to_string())
        }
        _ => unreachable!("the test names every existing standard target explicitly"),
    }
}

/// Compile and execute whole emitted artifacts only in the native CI suite.
fn observe_rust(source: &str, expected: &str, abort_message: Option<&str>, name: &str) {
    let directory = std::env::temp_dir().join(format!(
        "meta-language-callable-execution-{}-{name}",
        std::process::id()
    ));
    fs::create_dir_all(&directory).unwrap();
    let file = directory.join("program.rs");
    let executable = directory.join("program");
    fs::write(&file, source).unwrap();
    let compiled = Command::new("rustc")
        .arg("--edition=2024")
        .arg(&file)
        .arg("-o")
        .arg(&executable)
        .output()
        .unwrap();
    assert!(
        compiled.status.success(),
        "{}; source retained at {}",
        String::from_utf8_lossy(&compiled.stderr),
        file.display()
    );
    assert_eq!(fs::read_to_string(&file).unwrap(), source);
    let output = Command::new(&executable).output().unwrap();
    fs::write(directory.join("stdout.txt"), &output.stdout).unwrap();
    fs::write(directory.join("stderr.txt"), &output.stderr).unwrap();
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        expected,
        "artifact retained at {}",
        file.display()
    );
    if let Some(message) = abort_message {
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains(message));
    } else {
        assert!(
            output.status.success(),
            "{}; artifact retained at {}",
            String::from_utf8_lossy(&output.stderr),
            file.display()
        );
    }
    fs::remove_dir_all(directory).unwrap();
}

fn observe(source: &str) -> String {
    let result = Command::new("node")
        .args(["--input-type=module", "-e", source])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    String::from_utf8(result.stdout).unwrap()
}

fn reload(saved: &TypedProgramLinks) -> [LinkNetwork; 2] {
    let binary = encode_link_network_binary(&saved.network, BinaryLinksOptions::default()).unwrap();
    [
        LinkNetwork::from_lino(&saved.network.to_lino()).unwrap(),
        decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
    ]
}

fn label(name: &str) -> String {
    typed_program_links_schema()["fields"][name]
        .as_str()
        .unwrap_or(name)
        .to_owned()
}

fn relation(network: &LinkNetwork, parent: LinkId, name: &str) -> (LinkId, [LinkId; 3]) {
    let name = label(name);
    network
        .links()
        .find_map(|link| match link.references() {
            [owner, field, value]
                if *owner == parent
                    && link.metadata().link_type() == Some(LinkType::Field)
                    && network.link(*field).and_then(|link| link.metadata().term())
                        == Some(name.as_str()) =>
            {
                Some((link.id(), [*owner, *field, *value]))
            }
            _ => None,
        })
        .unwrap()
}

fn field(network: &LinkNetwork, parent: LinkId, name: &str) -> LinkId {
    relation(network, parent, name).1[2]
}

fn constructs(network: &LinkNetwork, kind: &str) -> Vec<LinkId> {
    let name = label("k");
    network
        .links()
        .filter_map(|link| match link.references() {
            [owner, field, value]
                if link.metadata().link_type() == Some(LinkType::Field)
                    && network.link(*field).and_then(|link| link.metadata().term())
                        == Some(name.as_str())
                    && network
                        .link(*value)
                        .and_then(|link| link.metadata().definition())
                        == Some(kind) =>
            {
                Some(*owner)
            }
            _ => None,
        })
        .collect()
}

fn edit_references(network: &LinkNetwork, id: LinkId, references: &[LinkId]) -> LinkNetwork {
    let text = network
        .to_lino()
        .lines()
        .map(|line| {
            if line.starts_with(&format!("({id}: ")) {
                let metadata = &line[line.find("(meta:").unwrap()..];
                format!(
                    "({id}: {} {metadata}",
                    references
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    LinkNetwork::from_lino(&text).unwrap()
}

fn edit_definition(network: &LinkNetwork, id: LinkId, value: &str) -> LinkNetwork {
    let before = network.link(id).unwrap().metadata().definition().unwrap();
    let marker = format!("(def: {})", percent_encode_links_text(before));
    let replacement = format!("(def: {})", percent_encode_links_text(value));
    let text = network
        .to_lino()
        .lines()
        .map(|line| {
            if line.starts_with(&format!("({id}: ")) {
                line.replacen(&marker, &replacement, 1)
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    LinkNetwork::from_lino(&text).unwrap()
}

#[test]
fn native_callable_sources_execute_from_both_formats_with_shared_and_independent_cells() {
    for (source, expected) in [
        (
            "function choose(base){const outer=()=>base;const inner=base=>base+1;return outer()+inner(2);}console.log(choose(5));",
            "8\n",
        ),
        (
            "function increment(value){return value+1;}function use(){const selected=increment;return selected(4);}console.log(use());",
            "5\n",
        ),
        (
            "function counter(){let n=0;return ()=>{n=n+1;return n;};}const next=counter();console.log(next(),next());",
            "1 2\n",
        ),
        (
            "function counter(){let n=0;return ()=>{n=n+1;return n;};}const first=counter();const second=counter();console.log(first(),first(),second());",
            "1 2 1\n",
        ),
    ] {
        assert_eq!(observe(source), expected);
        let saved = create_typed_program_links(source, "JavaScript").unwrap();
        for network in reload(&saved) {
            let checked = decode_typed_program_links(&network, saved.root).unwrap();
            assert_eq!(checked.source_language.as_str(), "JavaScript");
            assert_eq!(
                observe(
                    &emit_typed_program_links(&network, saved.root, "JavaScript")
                        .unwrap()
                        .code
                ),
                expected
            );
            assert!(
                emit_typed_program_links(&network, saved.root, "Rust")
                    .unwrap()
                    .code
                    .contains("std::rc::Rc::new")
            );
            assert!(network.links().all(|link| {
                link.metadata()
                    .definition()
                    .is_none_or(|value| !value.contains(source))
            }));
            if source.contains("counter") {
                for kind in ["cellAllocate", "cellRead", "cellWrite"] {
                    assert!(!constructs(&network, kind).is_empty(), "{kind}");
                }
            } else {
                for target in ["Lean", "Rocq"] {
                    assert!(
                        !portable_emission(&network, saved.root, target)
                            .unwrap()
                            .is_empty()
                    );
                }
            }
        }
    }
}

#[test]
fn native_callable_parameter_edits_preserve_identity_and_reject_cross_scope_captures() {
    let saved = create_typed_program_links("function choose(base){const outer=()=>base;const inner=base=>base+1;return outer()+inner(2);}console.log(choose(5));", "JavaScript").unwrap();
    for network in reload(&saved) {
        let lambdas = constructs(&network, "lambda");
        let inner = *lambdas
            .iter()
            .find(|lambda| {
                network
                    .link(field(&network, **lambda, "params"))
                    .unwrap()
                    .references()
                    .len()
                    == 1
            })
            .unwrap();
        let parameter = network
            .link(field(&network, inner, "params"))
            .unwrap()
            .references()[0];
        assert!(
            read_typed_program_binding_identities(&network, saved.root)
                .unwrap()
                .contains(&parameter)
        );
        let name = field(&network, parameter, "name");
        let renamed = edit_definition(&network, name, "selected");
        assert_eq!(
            observe(
                &emit_typed_program_links(&renamed, saved.root, "JavaScript")
                    .unwrap()
                    .code
            ),
            "8\n"
        );
        assert!(
            emit_typed_program_links(&renamed, saved.root, "Rust")
                .unwrap()
                .code
                .contains("selected: f64")
        );
        let outer = *lambdas
            .iter()
            .find(|lambda| {
                network
                    .link(field(&network, **lambda, "params"))
                    .unwrap()
                    .references()
                    .is_empty()
            })
            .unwrap();
        let variable = field(&network, outer, "body");
        let (reference, [owner, label, _]) = relation(&network, variable, "binding reference");
        let invalid = edit_references(&network, reference, &[owner, label, parameter]);
        assert!(
            decode_typed_program_links(&invalid, saved.root)
                .unwrap_err()
                .message
                .contains("outside its scope")
        );
    }
}

#[test]
fn native_callable_arity_and_multiple_parameter_owners_remain_checked_after_reload() {
    let saved = create_typed_program_links("function value(base){const first=x=>x+base;const second=x=>x*base;return first(2)+second(3);}console.log(value(4));", "JavaScript").unwrap();
    for network in reload(&saved) {
        assert_eq!(
            observe(
                &emit_typed_program_links(&network, saved.root, "JavaScript")
                    .unwrap()
                    .code
            ),
            "18\n"
        );
        let application = constructs(&network, "apply")[0];
        let arguments = field(&network, application, "args");
        let invalid = edit_references(&network, arguments, &[]);
        for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
            assert!(portable_emission(&invalid, saved.root, target).is_err());
        }
        let lambdas = constructs(&network, "lambda");
        let first = network
            .link(field(&network, lambdas[0], "params"))
            .unwrap()
            .references()[0];
        let second_parameters = field(&network, lambdas[1], "params");
        let second = network.link(second_parameters).unwrap().references()[0];
        let mut changed = network.clone();
        let references = network
            .links()
            .filter_map(|link| match link.references() {
                [owner, label, target]
                    if *target == second
                        && network
                            .link(*label)
                            .and_then(|label| label.metadata().term())
                            == Some("binding reference") =>
                {
                    Some((link.id(), *owner, *label))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        for (reference, owner, label) in references {
            changed = edit_references(&changed, reference, &[owner, label, first]);
        }
        changed = edit_references(&changed, second_parameters, &[first]);
        assert!(
            decode_typed_program_links(&changed, saved.root)
                .unwrap_err()
                .message
                .contains("multiple lexical owners")
        );
    }
}

#[test]
fn native_callable_jsdoc_forms_recheck_and_emit_every_existing_standard_target() {
    for annotation in [
        "(bigint)=>bigint",
        "function(bigint):bigint",
        "((value: bigint) => bigint)",
    ] {
        let source = format!(
            "/** @param {{{annotation}}} callback @returns {{bigint}} */function use(callback){{return callback(2n);}}function increment(value){{return value+1n;}}console.log(use(increment));"
        );
        assert_eq!(observe(&source), "3n\n");
        let saved = create_typed_program_links(&source, "JavaScript").unwrap();
        for network in reload(&saved) {
            assert_eq!(
                observe(&portable_emission(&network, saved.root, "JavaScript").unwrap()),
                "3n\n"
            );
            for target in ["Rust", "Lean", "Rocq"] {
                assert!(
                    !portable_emission(&network, saved.root, target)
                        .unwrap()
                        .is_empty()
                );
            }
        }
    }
}

#[test]
fn native_callable_output_runs_at_invocation_in_javascript_and_rust_and_refuses_pure_targets() {
    let refusal = "effectful function value: function values that print or may abort require an output-threaded function type";
    for (case, (source, expected)) in [
        ("function make(){console.log('created');return ()=>{console.log('called');return 7;};}const next=make();console.log('ready');console.log(next());", "created\nready\ncalled\n7\n"),
        ("function noisy(value){console.log('called');return value+1;}function use(callback){console.log('before');return callback(2);}const selected=noisy;console.log('ready');console.log(use(selected));", "ready\nbefore\ncalled\n3\n"),
        ("console.log((()=>{console.log('called');return 7;})());", "called\n7\n"),
    ].iter().enumerate() {
        assert_eq!(observe(source), *expected);
        let saved = create_typed_program_links(source, "JavaScript").unwrap();
        for (format, network) in reload(&saved).iter().enumerate() {
            assert_eq!(observe(&portable_emission(network, saved.root, "JavaScript").unwrap()), *expected);
            observe_rust(&portable_emission(network, saved.root, "Rust").unwrap(), expected, None, &format!("output-{case}-{format}"));
            let program = decode_typed_program_links(network, saved.root).unwrap();
            for emit in [emit_lean, emit_rocq] {
                let error = emit(&program).unwrap_err();
                assert_eq!(error.construct.as_deref(), Some("effectful function value"));
                assert_eq!(error.reason, refusal);
                assert!(error.span.is_some());
            }
        }
    }
}

#[test]
fn native_callable_abort_waits_for_invocation_and_preserves_both_format_artifacts() {
    let source = "function make(){return ()=>{throw new Error('stop');};}const next=make();console.log('ready');next();";
    let original = Command::new("node")
        .args(["--input-type=module", "-e", source])
        .output()
        .unwrap();
    assert!(!original.status.success());
    assert_eq!(String::from_utf8_lossy(&original.stdout), "ready\n");
    assert!(String::from_utf8_lossy(&original.stderr).contains("stop"));
    let saved = create_typed_program_links(source, "JavaScript").unwrap();
    for (format, network) in reload(&saved).iter().enumerate() {
        let code = portable_emission(network, saved.root, "JavaScript").unwrap();
        let emitted = Command::new("node")
            .args(["--input-type=module", "-e", &code])
            .output()
            .unwrap();
        assert!(!emitted.status.success());
        assert_eq!(emitted.stdout, original.stdout);
        assert!(String::from_utf8_lossy(&emitted.stderr).contains("stop"));
        observe_rust(
            &portable_emission(network, saved.root, "Rust").unwrap(),
            "ready\n",
            Some("stop"),
            &format!("abort-{format}"),
        );
        let program = decode_typed_program_links(network, saved.root).unwrap();
        for emit in [emit_lean, emit_rocq] {
            assert_eq!(
                emit(&program).unwrap_err().reason,
                "effectful function value: function values that print or may abort require an output-threaded function type"
            );
        }
    }
}
