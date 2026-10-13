//! Closed function completions, exact source profiles and independent oracles.
//! Native original/target compilation occurs only in CI.

use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::rust::{
    parse_rust, parse_rust_with_completion_model, parse_rust_with_source_models,
};
use meta_language::{
    LinkId, LinkNetwork, LinkType, create_typed_program_links_with_rust_completion_model,
    create_typed_program_links_with_rust_source_models, emit_typed_program_links,
};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn fixture() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-completions.lino"
    ))
    .expect("shared closed-completion fixtures are inspectable Links data")
}

fn field(network: &LinkNetwork, parent: LinkId, label: &str) -> Option<LinkId> {
    network.links().find_map(|link| match link.references() {
        [owner, name, child]
            if link.metadata().link_type() == Some(LinkType::Field)
                && *owner == parent
                && network.link(*name).and_then(|name| name.metadata().term()) == Some(label) =>
        {
            Some(*child)
        }
        _ => None,
    })
}

#[test]
fn closed_returns_retain_shared_surface_and_source_free_profiles_and_emitters() {
    let data = fixture();
    let model = data["completionModel"].as_str().unwrap();
    for case in data["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let source = case["source"].as_str().unwrap();
        let array_model = case["arrayResultModel"].as_str();
        let surface = parse_rust_with_source_models(source, array_model.unwrap_or(""), model)
            .unwrap_or_else(|error| panic!("{id}: {error}"));
        assert_eq!(
            serde_json::to_value(&surface).unwrap(),
            case["surface"],
            "{id}"
        );
        let program = check_program(&surface).unwrap();
        assert_eq!(
            emit_javascript(&program).unwrap().text,
            case["javascript"].as_str().unwrap(),
            "{id}"
        );
        assert_eq!(
            emit_rust(&program).unwrap().text,
            case["rust"].as_str().unwrap(),
            "{id}"
        );
        let user = serde_json::json!({"project":id});
        let archive = create_typed_program_links_with_rust_source_models(
            source,
            "Rust",
            array_model,
            Some(model),
            Some(&user),
        )
        .unwrap();
        let native = LinkNetwork::from_lino(&archive.network.to_lino()).unwrap();
        let provenance = field(&native, archive.root, "provenance").unwrap();
        let profile = field(&native, provenance, "source profile").unwrap();
        let model_id = field(&native, profile, "rust completion model").unwrap();
        assert_eq!(
            native.link(model_id).unwrap().metadata().definition(),
            Some(model)
        );
        let selected_array = field(&native, profile, "rust array result model");
        assert_eq!(
            selected_array.and_then(|id| native.link(id).unwrap().metadata().definition()),
            array_model
        );
        let user_id = field(&native, provenance, "user provenance").unwrap();
        let project = field(&native, user_id, "project").unwrap();
        assert_eq!(
            native.link(project).unwrap().metadata().definition(),
            Some(id)
        );
        assert!(native.links().all(|link| {
            !link
                .metadata()
                .definition()
                .is_some_and(|value| value.contains(source))
        }));
        let shared = LinkNetwork::from_lino(case["archive"].as_str().unwrap()).unwrap();
        for (network, root) in [
            (&native, archive.root),
            (&shared, LinkId::from_u64(case["root"].as_u64().unwrap())),
        ] {
            for (target, key) in [("Rust", "rust"), ("JavaScript", "javascript")] {
                assert_eq!(
                    emit_typed_program_links(network, root, target)
                        .unwrap()
                        .code,
                    case[key].as_str().unwrap(),
                    "{id}: {target}"
                );
            }
        }
    }
}

#[test]
fn default_refusals_and_selected_unsafe_completions_keep_exact_diagnostics() {
    let data = fixture();
    let model = data["completionModel"].as_str().unwrap();
    for (key, selected) in [("legacy", false), ("invalid", true)] {
        for case in data[key].as_array().unwrap() {
            let source = case["source"].as_str().unwrap();
            let surface = if selected {
                parse_rust_with_completion_model(source, model)
            } else {
                parse_rust(source)
            };
            let error = surface
                .and_then(|surface| check_program(&surface))
                .expect_err("the unchanged source obligation is required");
            assert_eq!(error.kind.as_str(), case["kind"].as_str().unwrap());
            assert_eq!(error.reason, case["reason"].as_str().unwrap());
            assert_eq!(serde_json::to_value(error.span).unwrap(), case["span"]);
        }
    }
    assert!(
        parse_rust_with_completion_model(data["legacy"][0]["source"].as_str().unwrap(), model)
            .is_ok()
    );
    assert!(parse_rust_with_completion_model("fn main(){}", "unknown").is_err());
    assert!(
        create_typed_program_links_with_rust_completion_model(
            "fn main(){}",
            "Rust",
            "unknown",
            None
        )
        .is_err()
    );
    assert!(
        create_typed_program_links_with_rust_completion_model(
            "function f(){return true;}",
            "JavaScript",
            model,
            None
        )
        .is_err()
    );
    assert!(
        create_typed_program_links_with_rust_source_models(
            "fn main(){}",
            "Rust",
            Some("unknown"),
            Some(model),
            None
        )
        .is_err()
    );
}

fn compile_rust(code: &str, directory: &Path, name: &str) -> PathBuf {
    let source = directory.join(format!("{name}.rs"));
    let binary = directory.join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
    fs::write(&source, code).unwrap();
    let mut compiler = Command::new("rustc");
    compiler.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        compiler.arg("-C").arg(format!("linker={linker}"));
    }
    let result = run(compiler.arg(&source).arg("-o").arg(&binary));
    assert!(result.ok, "{name}: {}", result.stderr);
    binary
}

fn rust_output(code: &str, directory: &Path, name: &str) -> Vec<String> {
    let result = run(&mut Command::new(compile_rust(code, directory, name)));
    assert!(result.ok, "{name}: {}", result.stderr);
    result.stdout.lines().map(str::to_owned).collect()
}
fn javascript_output(code: &str) -> Vec<String> {
    let result = run(Command::new("node").args(["--input-type=module", "--eval", code]));
    assert!(result.ok, "{}", result.stderr);
    result.stdout.lines().map(str::to_owned).collect()
}

#[test]
fn original_compilers_and_both_complete_emitted_directions_match_independent_results() {
    let data = fixture();
    let directory = TemporaryDirectory::new("rust-closed-completions");
    for (index, case) in data["cases"].as_array().unwrap().iter().enumerate() {
        let expected = case["output"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        for key in ["source", "rust", "reverseRust"] {
            assert_eq!(
                rust_output(
                    case[key].as_str().unwrap(),
                    &directory.0,
                    &format!("{key}_{index}")
                ),
                expected
            );
        }
        assert_eq!(
            javascript_output(case["originalJavaScript"].as_str().unwrap()),
            expected
        );
        let network = LinkNetwork::from_lino(case["archive"].as_str().unwrap()).unwrap();
        let root = LinkId::from_u64(case["root"].as_u64().unwrap());
        assert_eq!(
            javascript_output(
                &emit_typed_program_links(&network, root, "JavaScript")
                    .unwrap()
                    .code
            ),
            expected
        );
    }
}

#[test]
fn linked_return_edit_and_abort_continuation_change_only_the_selected_branch() {
    let data = fixture();
    let directory = TemporaryDirectory::new("rust-closed-completion-edits");
    let case = &data["cases"][0];
    let network = LinkNetwork::from_lino(case["archive"].as_str().unwrap()).unwrap();
    let literal = network
        .links()
        .find(|link| link.metadata().definition() == Some("early"))
        .unwrap()
        .id();
    let text = network
        .to_lino()
        .lines()
        .map(|line| {
            if line.starts_with(&format!("({literal}: ")) {
                line.replace("(def: early)", "(def: changed)")
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let changed = LinkNetwork::from_lino(&text).unwrap();
    let root = LinkId::from_u64(case["root"].as_u64().unwrap());
    assert_eq!(
        javascript_output(
            &emit_typed_program_links(&changed, root, "JavaScript")
                .unwrap()
                .code
        ),
        ["changed", "late"]
    );
    assert_eq!(
        rust_output(
            &emit_typed_program_links(&changed, root, "Rust")
                .unwrap()
                .code,
            &directory.0,
            "changed"
        ),
        ["changed", "late"]
    );
    let abort = data["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["id"] == "abort-continuation")
        .unwrap();
    let main = "fn main(){println!(\"{}\",choose(false));}";
    let original = abort["source"]
        .as_str()
        .unwrap()
        .replace("choose(true)", "choose(false)");
    let emitted = format!("{}\n{main}", abort["rustDefinitions"].as_str().unwrap());
    for (name, source) in [("original_abort", original), ("emitted_abort", emitted)] {
        let result = run(&mut Command::new(compile_rust(&source, &directory.0, name)));
        assert!(!result.ok);
        assert!(result.stderr.contains("selected failure"));
        assert!(result.stdout.is_empty());
    }
    let observed = format!(
        "{}\nconsole.log(choose(false));",
        abort["javascript"].as_str().unwrap()
    );
    let result = run(Command::new("node").args(["--input-type=module", "--eval", &observed]));
    assert!(!result.ok);
    assert!(result.stderr.contains("selected failure"));
    assert_eq!(result.stdout.trim(), "safe");
}

#[test]
fn canonical_literal_panic_replays_exact_data_and_preserves_default_refusals() {
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-completion-panic.lino"
    ))
    .unwrap();
    let model = data["completionModel"].as_str().unwrap();
    let directory = TemporaryDirectory::new("rust-completion-literal-panic");
    for (index, case) in data["cases"].as_array().unwrap().iter().enumerate() {
        let source = case["source"].as_str().unwrap();
        let message = case["message"].as_str().unwrap();
        let surface = parse_rust_with_completion_model(source, model).unwrap();
        assert_eq!(serde_json::to_value(&surface).unwrap(), case["surface"]);
        let program = check_program(&surface).unwrap();
        let native =
            create_typed_program_links_with_rust_completion_model(source, "Rust", model, None)
                .unwrap();
        let shared = LinkNetwork::from_lino(case["archive"].as_str().unwrap()).unwrap();
        let reloaded = LinkNetwork::from_lino(&native.network.to_lino()).unwrap();
        for (network, root) in [
            (&shared, LinkId::from_u64(case["root"].as_u64().unwrap())),
            (&reloaded, native.root),
        ] {
            for (target, key) in [("Rust", "rust"), ("JavaScript", "javascript")] {
                assert_eq!(
                    emit_typed_program_links(network, root, target)
                        .unwrap()
                        .code,
                    case[key].as_str().unwrap()
                );
            }
        }
        for key in ["source", "rust", "reverseRust"] {
            assert_eq!(
                rust_output(
                    case[key].as_str().unwrap(),
                    &directory.0,
                    &format!("{key}_{index}")
                ),
                ["true"]
            );
        }
        assert_eq!(
            javascript_output(case["originalJavaScript"].as_str().unwrap()),
            ["true"]
        );
        for code in [
            case["originalJavaScript"].as_str().unwrap(),
            case["javascript"].as_str().unwrap(),
        ] {
            let observed = format!(
                "{code}\ntry {{ fail(false); }} catch (error) {{ console.log(error.message); }}"
            );
            let result =
                run(Command::new("node").args(["--input-type=module", "--eval", &observed]));
            assert!(result.ok, "{}", result.stderr);
            assert_eq!(result.stdout, format!("true\n{message}\n"));
        }
        let original = source.replace("fail(true)", "fail(false)");
        let definitions = emit_rust(&program).unwrap().definitions.join("\n\n");
        let reimport =
            parse_rust_with_completion_model(&format!("{definitions}\nfn main(){{}}"), model)
                .unwrap();
        assert!(check_program(&reimport).is_ok());
        let emitted = format!("{definitions}\nfn main(){{fail(false);}}");
        for (name, code) in [("original", original), ("emitted", emitted)] {
            let result = run(&mut Command::new(compile_rust(
                &code,
                &directory.0,
                &format!("failed_{name}_{index}"),
            )));
            assert!(!result.ok);
            assert!(result.stdout.is_empty());
            assert!(result.stderr.contains(message));
        }
    }
    for (case, selected) in data["invalid"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| (case, true))
        .chain([
            (&data["legacy"], false),
            (&data["legacy"], true),
            (&data["defaultCanonical"], false),
        ])
    {
        let source = case["source"].as_str().unwrap();
        let surface = if selected {
            parse_rust_with_completion_model(source, model)
        } else {
            parse_rust(source)
        };
        let error = surface
            .and_then(|surface| check_program(&surface))
            .expect_err("only exact selected literal panic data is supported");
        assert_eq!(error.kind.as_str(), case["kind"].as_str().unwrap());
        assert_eq!(error.reason, case["reason"].as_str().unwrap());
        assert_eq!(serde_json::to_value(error.span).unwrap(), case["span"]);
    }
}

#[test]
fn terminal_divergent_branches_do_not_hide_actual_function_return_paths() {
    let model = "closed guarded function returns 1";
    let hidden_return = "fn f(flag:bool,choice:bool)->bool{if flag{if choice{return true;}panic!(\"stop\");}else{false}}fn main(){}";
    let error = parse_rust_with_completion_model(hidden_return, model)
        .and_then(|surface| check_program(&surface))
        .expect_err("a return path is not an aborted-only branch");
    assert!(error.reason.starts_with("guarded return completion:"));
    let source = "fn choose(flag:bool,choice:bool)->String{if flag{if choice{panic!(\"first\");}else{panic!(\"second\");}}String::from(\"safe\")}fn main(){println!(\"{}\",choose(false,false));}";
    let program = check_program(&parse_rust_with_completion_model(source, model).unwrap()).unwrap();
    let directory = TemporaryDirectory::new("rust-aborted-completions");
    assert_eq!(rust_output(source, &directory.0, "original"), ["safe"]);
    assert_eq!(
        rust_output(&emit_rust(&program).unwrap().text, &directory.0, "emitted"),
        ["safe"]
    );
    let javascript = format!(
        "{}\nfor (const choice of [true,false]) {{try {{choose(true,choice);}} catch(error) {{console.log(error.message);}}}}",
        emit_javascript(&program).unwrap().text
    );
    assert_eq!(javascript_output(&javascript), ["safe", "first", "second"]);
}
