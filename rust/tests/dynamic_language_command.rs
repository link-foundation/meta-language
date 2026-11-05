//! Deferred shared controls; do not run until the complete code draft is ready.
use meta_language::{
    DynamicMetaLanguage, DynamicMetaLanguageOptions, LinkId, LinkNetwork, MetaLanguageArchiveEntry,
    MetaLanguageArchiveLimits, decode_structured_data_links, parse_grammar_links,
    read_meta_language_archive_binary, read_meta_language_archive_text, read_structured_data_lino,
    render_grammar_links, run_dynamic_language_command,
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

fn concrete(entry: &MetaLanguageArchiveEntry) -> Value {
    let mut network = LinkNetwork::new();
    let mut pending = vec![entry.root];
    let mut seen = BTreeSet::new();
    let mut index = 0;
    while index < pending.len() {
        let id = pending[index];
        index += 1;
        if !seen.insert(id) {
            continue;
        }
        let link = entry.network.link(id).unwrap();
        pending.extend_from_slice(link.references());
    }
    let coordinates = seen
        .iter()
        .enumerate()
        .map(|(index, id)| (*id, LinkId::from_u64(u64::try_from(index + 1).unwrap())))
        .collect::<BTreeMap<_, _>>();
    for id in &seen {
        let link = entry.network.link(*id).unwrap();
        network.insert_dynamic_link(
            &link
                .references()
                .iter()
                .map(|id| coordinates[id])
                .collect::<Vec<_>>(),
            link.metadata().clone(),
        );
    }
    decode_structured_data_links(&network, coordinates[&entry.root]).unwrap()
}
fn bytes(value: &Value) -> Vec<u8> {
    match value["type"].as_str().unwrap() {
        "missing" => Vec::new(),
        "token" | "error" => {
            if let Some(text) = value["text"].as_str() {
                return text.as_bytes().to_vec();
            }
            let hex = value["hex"].as_str().unwrap();
            (0..hex.len())
                .step_by(2)
                .map(|index| u8::from_str_radix(&hex[index..index + 2], 16).unwrap())
                .collect()
        }
        _ => value.get("children").and_then(Value::as_array).map_or_else(
            || bytes(&value["root"]),
            |children| children.iter().flat_map(bytes).collect(),
        ),
    }
}
#[test]
fn shared_dynamic_command_sequence_retains_complete_archive_contexts() {
    let fixture = read_structured_data_lino(include_str!(
        "../../parity/fixtures/dynamic-language-command.lino"
    ))
    .unwrap();
    let mut files = fixture["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|file| {
            (
                file["path"].as_str().unwrap().to_owned(),
                file["text"].as_str().unwrap().as_bytes().to_vec(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for step in fixture["steps"].as_array().unwrap() {
        let args = step["args"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        let output = run_dynamic_language_command(
            &args,
            |path| {
                files
                    .get(path)
                    .ok_or_else(|| format!("unexpected source read {path}"))
                    .and_then(|value| {
                        String::from_utf8(value.clone()).map_err(|error| error.to_string())
                    })
            },
            |path| {
                files
                    .get(path)
                    .cloned()
                    .ok_or_else(|| format!("unexpected source read {path}"))
            },
        );
        assert_eq!(
            i64::from(output.exit_code),
            step["exit"].as_i64().unwrap(),
            "{}: {}",
            step["name"],
            output.stderr
        );
        if let Some(error) = step["error"].as_str() {
            assert!(
                output.stderr.contains(error),
                "{}: {}",
                step["name"],
                output.stderr
            );
        }
        if step["rejection"] == true {
            assert!(read_structured_data_lino(&output.stderr).unwrap()["rejection"].is_object());
        }
        if let Some(save) = step["save"].as_str() {
            files.insert(save.to_owned(), output.stdout.clone());
        }
        if let Some(expected) = step["inventory"].as_array() {
            let rows =
                read_structured_data_lino(std::str::from_utf8(&output.stdout).unwrap()).unwrap();
            assert_eq!(
                rows.as_array()
                    .unwrap()
                    .iter()
                    .map(|row| row["identifier"].clone())
                    .collect::<Vec<_>>(),
                *expected
            );
        }
        if let Some(path) = step["grammar"].as_str() {
            let actual = parse_grammar_links(std::str::from_utf8(&output.stdout).unwrap()).unwrap();
            let expected = parse_grammar_links(std::str::from_utf8(&files[path]).unwrap()).unwrap();
            assert_eq!(
                render_grammar_links(&actual),
                render_grammar_links(&expected)
            );
        }
        if step["archive"] == true {
            let limits = MetaLanguageArchiveLimits::default();
            let archive = if step["binary"] == true {
                read_meta_language_archive_binary(&output.stdout, limits)
                    .unwrap()
                    .archive
            } else {
                read_meta_language_archive_text(
                    std::str::from_utf8(&output.stdout).unwrap(),
                    limits,
                )
                .unwrap()
                .archive
            };
            let session =
                DynamicMetaLanguage::new(&archive, &DynamicMetaLanguageOptions::default()).unwrap();
            if let Some(name) = step["concrete"].as_str() {
                let entry = session
                    .entries()
                    .iter()
                    .find(|entry| entry.identifier == name)
                    .unwrap();
                assert_eq!(entry.kind, "concrete syntax");
                let value = concrete(entry);
                assert_eq!(value["projection"], "concrete syntax");
                assert_eq!(
                    bytes(&value["outcome"]["tree"]),
                    step["text"].as_str().unwrap().as_bytes()
                );
            }
            if let Some(name) = step["abstract"].as_str() {
                assert_eq!(
                    session
                        .entries()
                        .iter()
                        .find(|entry| entry.identifier == name)
                        .unwrap()
                        .kind,
                    "abstract syntax"
                );
            }
        }
    }
    let args = fixture["steps"][0]["args"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect::<Vec<_>>();
    let prepared = run_dynamic_language_command(
        &args,
        |path| String::from_utf8(files[path].clone()).map_err(|error| error.to_string()),
        |path| Ok(files[path].clone()),
    );
    assert_eq!(prepared.exit_code, 0, "{}", prepared.stderr);
    files.insert("prepared.lino".to_owned(), prepared.stdout);
    files.remove("first.lino");
    files.remove("configuration.lino");
    let args = fixture["steps"][3]["args"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect::<Vec<_>>();
    let parsed = run_dynamic_language_command(
        &args,
        |path| {
            files
                .get(path)
                .ok_or_else(|| format!("unexpected source read {path}"))
                .and_then(|value| {
                    String::from_utf8(value.clone()).map_err(|error| error.to_string())
                })
        },
        |path| {
            files
                .get(path)
                .cloned()
                .ok_or_else(|| format!("unexpected source read {path}"))
        },
    );
    assert_eq!(parsed.exit_code, 0, "{}", parsed.stderr);
}
