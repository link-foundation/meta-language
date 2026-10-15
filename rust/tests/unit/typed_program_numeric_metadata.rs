//! Raw integral metadata aliases use the actual Links decoder. Native runs in CI.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::{
    LinkId, LinkNetwork, decode_typed_program_links, emit_typed_program_links,
    encode_typed_program_links,
};
use serde_json::{Value, json};
use std::fs;
use std::process::Command;

fn archive(entry: &Value, edit: Option<&Value>) -> (LinkNetwork, LinkId) {
    let mut source = entry["archive"].as_str().unwrap().to_owned();
    if let Some(edit) = edit {
        let network = LinkNetwork::from_lino(&source).unwrap();
        let scalar = LinkId::from_u64(edit["scalar"].as_u64().unwrap());
        let before = network
            .link(scalar)
            .unwrap()
            .metadata()
            .definition()
            .unwrap();
        let prefix = format!("({}:", scalar.as_u64());
        let old = format!("(def: {before})");
        let new = format!("(def: {})", edit["spelling"].as_str().unwrap());
        let mut replaced = false;
        source = source
            .lines()
            .map(|line| {
                if line.starts_with(&prefix) {
                    assert!(line.contains(&old));
                    replaced = true;
                    line.replacen(&old, &new, 1)
                } else {
                    line.to_owned()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert!(replaced);
    }
    (
        LinkNetwork::from_lino(&source).unwrap(),
        LinkId::from_u64(entry["root"].as_u64().unwrap()),
    )
}

fn javascript_archive(network: &LinkNetwork, root: LinkId, observation: &Value) {
    let directory = TemporaryDirectory::new("integral-metadata-public-archive");
    let archive = directory.0.join("program.lino");
    fs::write(&archive, network.to_lino()).unwrap();
    let index = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../js/src/index.js");
    let source = format!(
        "import {{pathToFileURL}} from 'node:url';import {{readFileSync}} from 'node:fs';import vm from 'node:vm';const {{LinkNetwork,decodeTypedProgramLinks,emitTypedProgramLinks}}=await import(pathToFileURL({}));const network=LinkNetwork.fromLino(readFileSync({},'utf8'));decodeTypedProgramLinks(network,{});emitTypedProgramLinks(network,{},'Rust');const code=emitTypedProgramLinks(network,{},'JavaScript').code;const context={{console:{{log(){{}}}}}};vm.createContext(context);vm.runInContext(code,context);const observation={};if(observation.name){{for(const[text,count,flag,expected]of observation.observations){{if(context[observation.name](text,BigInt(count),flag)!==expected)throw new Error('wrong owned text value');}}}}else if(context.keep(7n)!==7n)throw new Error('wrong fixed value');console.log('metadata-values');",
        json!(index.to_string_lossy()),
        json!(archive.to_string_lossy()),
        root.as_u64(),
        root.as_u64(),
        root.as_u64(),
        observation,
    );
    let path = directory.0.join("observe.mjs");
    fs::write(&path, source).unwrap();
    let result = run(Command::new("node").arg(path));
    assert!(result.ok, "{}", result.stderr);
    assert_eq!(result.stdout, "metadata-values\n");
}

#[test]
fn actual_integral_fixed_width_aliases_retain_checked_programs_and_native_javascript_archives() {
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../experiments/program-numeric-metadata-fixture.lino"
    ))
    .unwrap();
    let mut aliases = 0;
    for entry in fixture["cases"].as_array().unwrap() {
        let (network, root) = archive(entry, None);
        let expected = decode_typed_program_links(&network, root).unwrap();
        for edit in entry["aliases"].as_array().unwrap() {
            let (network, root) = archive(entry, Some(edit));
            let actual = decode_typed_program_links(&network, root).unwrap();
            assert_eq!(actual, expected, "{}", edit["id"]);
            let rebuilt = encode_typed_program_links(&actual).unwrap();
            assert_eq!(
                decode_typed_program_links(&rebuilt.network, rebuilt.root).unwrap(),
                expected
            );
            for target in ["JavaScript", "Rust"] {
                assert_eq!(
                    emit_typed_program_links(&network, root, target)
                        .unwrap()
                        .code,
                    emit_typed_program_links(&rebuilt.network, rebuilt.root, target)
                        .unwrap()
                        .code,
                    "{}:{target}",
                    edit["id"]
                );
            }
            if edit["spelling"] == "32.0" {
                javascript_archive(&rebuilt.network, rebuilt.root, entry);
            }
            aliases += 1;
        }
        for edit in entry["invalid"].as_array().unwrap() {
            let (network, root) = archive(entry, Some(edit));
            assert!(
                decode_typed_program_links(&network, root).is_err(),
                "{}",
                edit["id"]
            );
        }
    }
    assert_eq!(aliases, 24);
}
