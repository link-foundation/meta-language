//! Deferred original multi-file compiler oracle, actual APIs and both codecs.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::{check::check_program, emit_rust::emit_rust};
use meta_language::{
    BinaryLinksOptions, LinkId, LinkNetwork, compose_rust_source_project,
    create_rust_source_project_program_links, decode_link_network_binary, emit_typed_program_links,
    encode_link_network_binary, parse_rust_source_project,
    read_rust_source_project_concrete_graphs, read_structured_data_lino,
};
use serde_json::{Value, json};
use std::{fs, process::Command};
fn emission(value: meta_language::TypedProgramLinksEmission) -> Value {
    json!({"code":value.code,"language":value.language,"mappings":value.mappings,"assumptions":value.assumptions,"encodings":value.encodings,"theorems":value.theorems})
}
#[test]
fn complete_source_projects_keep_original_files_executable_meanings_and_graph_islands() {
    assert!(
        cfg!(target_pointer_width = "64"),
        "uncovered source usize width"
    );
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let parts = version
        .stdout
        .split_whitespace()
        .nth(1)
        .unwrap()
        .split('.')
        .collect::<Vec<_>>();
    let major = parts[0].parse::<u32>().unwrap();
    let minor = parts[1].parse::<u32>().unwrap();
    assert!(
        major == 1 && (90..100).contains(&minor),
        "uncovered Rust {}",
        version.stdout
    );
    let produced = run(Command::new("node").arg("../experiments/rust-records/source-project.mjs"));
    assert!(produced.ok, "{}", produced.stderr);
    let product = read_structured_data_lino(&produced.stdout).unwrap();
    for (ordinal, row) in product["cases"].as_array().unwrap().iter().enumerate() {
        let inputs = &row["inputs"];
        let entry = row["entry"].as_str().unwrap();
        let options = &row["options"];
        let composition = compose_rust_source_project(inputs, entry, options).unwrap();
        assert_eq!(composition, row["composition"], "{}", row["name"]);
        let packet = create_rust_source_project_program_links(inputs, entry, options).unwrap();
        let node_graph = LinkNetwork::from_lino(row["links"].as_str().unwrap()).unwrap();
        let node_root = LinkId::from_u64(row["root"].as_u64().unwrap());
        for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
            let expected = &row["emissions"][target];
            assert_eq!(
                emission(emit_typed_program_links(&packet.network, packet.root, target).unwrap()),
                *expected
            );
            assert_eq!(
                emission(emit_typed_program_links(&node_graph, node_root, target).unwrap()),
                *expected
            );
        }
        let text = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
        let binary = decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap();
        for graph in [&packet.network, &text, &binary] {
            let originals =
                read_rust_source_project_concrete_graphs(graph, packet.root, &json!({})).unwrap();
            let observed = originals
                .iter()
                .map(|file| {
                    assert!(file.network.verify_full_match(None).is_clean());
                    assert_eq!(file.network.reconstruct_text(), file.source);
                    json!({"path":file.path,"source":file.source})
                })
                .collect::<Vec<_>>();
            let mut expected = inputs.as_array().unwrap().clone();
            expected.sort_by(|a, b| {
                a["path"]
                    .as_str()
                    .unwrap()
                    .encode_utf16()
                    .cmp(b["path"].as_str().unwrap().encode_utf16())
            });
            assert_eq!(observed, expected);
            for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
                assert_eq!(
                    emission(emit_typed_program_links(graph, packet.root, target).unwrap()),
                    row["emissions"][target]
                );
            }
        }
        let directory = TemporaryDirectory::new(&format!("source-project-{ordinal}"));
        for file in inputs.as_array().unwrap() {
            let path = directory.0.join(file["path"].as_str().unwrap());
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            let mut source = file["source"].as_str().unwrap().to_owned();
            if options["library"] == true && file["path"] == row["entry"] {
                source.push_str("\nfn main(){println!(\"{}\",result());}\n");
            }
            fs::write(path, source).unwrap();
        }
        let original = directory.0.join("original-source");
        let compiled = run(Command::new("rustc")
            .arg("--edition=2024")
            .arg(directory.0.join(entry))
            .arg("-o")
            .arg(&original));
        assert!(compiled.ok, "{}: {}", row["name"], compiled.stderr);
        let observed = run(&mut Command::new(&original));
        assert!(observed.ok, "{}", observed.stderr);
        assert_eq!(observed.stdout.trim_end(), row["value"].as_str().unwrap());
        let (_, surface, normalization) =
            parse_rust_source_project(inputs, entry, options).unwrap();
        let emitted = emit_rust(&check_program(&surface).unwrap()).unwrap();
        let target_source = if options["library"] == true {
            assert_eq!(normalization, "empty frontend entry for a source library");
            let target = emitted
                .mappings
                .iter()
                .find(|mapping| mapping.kind == "function" && mapping.source == "result")
                .unwrap()
                .target
                .replace('.', "::");
            let header = emitted
                .text
                .lines()
                .find(|line| line.starts_with("#![allow("))
                .unwrap()
                .to_owned();
            std::iter::once(header)
                .chain(emitted.preludes)
                .chain(emitted.definitions)
                .chain(std::iter::once(format!(
                    "fn main(){{println!(\"{{}}\",{target}());}}"
                )))
                .collect::<Vec<_>>()
                .join("\n\n")
        } else {
            emitted.text
        };
        let source = directory.0.join("translated.rs");
        let executable = directory.0.join("translated-source");
        fs::write(&source, target_source).unwrap();
        let compiled = run(Command::new("rustc")
            .arg("--edition=2024")
            .arg(&source)
            .arg("-o")
            .arg(&executable));
        assert!(compiled.ok, "{}: {}", row["name"], compiled.stderr);
        let observed = run(&mut Command::new(&executable));
        assert!(observed.ok, "{}", observed.stderr);
        assert_eq!(observed.stdout.trim_end(), row["value"].as_str().unwrap());
        let js = &row["emissions"]["JavaScript"];
        let mut code = js["code"].as_str().unwrap().to_owned();
        if options["library"] == true {
            let target = js["mappings"]
                .as_array()
                .unwrap()
                .iter()
                .find(|mapping| mapping["kind"] == "function" && mapping["source"] == "result")
                .unwrap()["target"]
                .as_str()
                .unwrap();
            code.push_str(&format!("\nconsole.log({target}());"));
        }
        let source = directory.0.join("translated.mjs");
        fs::write(&source, code).unwrap();
        let observed = run(Command::new("node").arg(source));
        assert!(observed.ok, "{}", observed.stderr);
        assert_eq!(observed.stdout.trim_end(), row["value"].as_str().unwrap());
    }
    for row in product["refusals"].as_array().unwrap() {
        let error = compose_rust_source_project(
            &row["inputs"],
            row["entry"].as_str().unwrap(),
            &row["options"],
        )
        .unwrap_err();
        assert!(error.contains(row["message"].as_str().unwrap()), "{error}");
        assert!(
            row["diagnostic"]["message"]
                .as_str()
                .unwrap()
                .contains(row["message"].as_str().unwrap())
        );
    }
    for row in product["planning"].as_array().unwrap() {
        let actual =
            compose_rust_source_project(&row["inputs"], row["entry"].as_str().unwrap(), &json!({}))
                .unwrap();
        assert_eq!(
            actual["modules"].as_array().unwrap().len(),
            row["modules"].as_u64().unwrap() as usize
        );
        assert_eq!(actual["source"], row["inputs"][0]["source"]);
    }
    let row = &product["cases"][0];
    let input = meta_language::structured_data_to_lino(
        &json!({"inputs":row["inputs"],"options":row["options"]}),
    )
    .unwrap();
    let base = vec![
        "inventory.lino".to_owned(),
        "--entry".to_owned(),
        row["entry"].as_str().unwrap().to_owned(),
    ];
    for binary in [false, true] {
        let mut args = base.clone();
        if binary {
            args.push("--binary".to_owned());
        }
        let output = meta_language::run_rust_source_project_command(&args, |path| {
            assert_eq!(path, "inventory.lino");
            Ok(input.clone())
        });
        assert_eq!(output.exit_code, 0, "{}", output.stderr);
        let graph = if binary {
            decode_link_network_binary(&output.stdout, BinaryLinksOptions::default()).unwrap()
        } else {
            LinkNetwork::from_lino(std::str::from_utf8(&output.stdout).unwrap()).unwrap()
        };
        let root = graph
            .links()
            .find(|link| {
                link.metadata().term()
                    == meta_language::typed_program_links_schema()["root"].as_str()
            })
            .unwrap()
            .id();
        assert_eq!(
            read_rust_source_project_concrete_graphs(&graph, root, &json!({}))
                .unwrap()
                .len(),
            row["inputs"].as_array().unwrap().len()
        );
    }
    for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
        let mut args = base.clone();
        args.extend(["--to".to_owned(), target.to_owned()]);
        let output = meta_language::run_rust_source_project_command(&args, |_| Ok(input.clone()));
        assert_eq!(output.exit_code, 0, "{}", output.stderr);
        assert_eq!(
            std::str::from_utf8(&output.stdout).unwrap(),
            row["emissions"][target]["code"].as_str().unwrap()
        );
    }
    for arguments in [
        vec!["inventory.lino"],
        vec!["--entry"],
        vec![
            "inventory.lino",
            "--entry",
            "src/main.rs",
            "--binary",
            "--to",
            "Rust",
        ],
    ] {
        let args = arguments
            .iter()
            .map(|value| (*value).to_owned())
            .collect::<Vec<_>>();
        let output = meta_language::run_rust_source_project_command(&args, |_| {
            panic!("argument failures cannot read files")
        });
        assert_eq!(output.exit_code, 2);
    }
    let output = meta_language::run_rust_source_project_command(&["help".to_owned()], |_| {
        panic!("help cannot read a file")
    });
    assert_eq!(output.exit_code, 0);
    assert!(
        std::str::from_utf8(&output.stdout)
            .unwrap()
            .contains("Default output retains")
    );
}
