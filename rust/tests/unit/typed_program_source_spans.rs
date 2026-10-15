//! Source-span number aliases retain values through the actual Links decoder.
use meta_language::{
    LinkId, LinkNetwork, decode_typed_program_links, emit_typed_program_links,
    encode_typed_program_links,
};
use serde_json::Value;

fn archive(entry: &Value, edit: Option<&Value>) -> (LinkNetwork, LinkId) {
    let mut source = entry["archive"].as_str().unwrap().to_owned();
    if let Some(edit) = edit {
        let original = LinkNetwork::from_lino(&source).unwrap();
        let scalar = LinkId::from_u64(edit["scalar"].as_u64().unwrap());
        let previous = original
            .link(scalar)
            .unwrap()
            .metadata()
            .definition()
            .unwrap();
        let prefix = format!("({}:", scalar.as_u64());
        let old = format!("(def: {previous})");
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

#[test]
fn decimal_and_exponent_source_offsets_recheck_without_changing_program_meaning() {
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/source-span-numbers.lino"
    ))
    .unwrap();
    let mut aliases = 0;
    for entry in fixture["cases"].as_array().unwrap() {
        let (network, root) = archive(entry, None);
        let expected = decode_typed_program_links(&network, root).unwrap();
        for edit in entry["aliases"].as_array().unwrap() {
            let (network, root) = archive(entry, Some(edit));
            let program = decode_typed_program_links(&network, root).unwrap();
            assert_eq!(program, expected, "{}", edit["id"]);
            let rebuilt = encode_typed_program_links(&program).unwrap();
            let loaded = LinkNetwork::from_lino(&rebuilt.network.to_lino()).unwrap();
            assert_eq!(
                decode_typed_program_links(&loaded, rebuilt.root).unwrap(),
                expected,
                "{}",
                edit["id"]
            );
            for target in ["JavaScript", "Rust"] {
                assert_eq!(
                    emit_typed_program_links(&network, root, target)
                        .unwrap()
                        .code,
                    emit_typed_program_links(&loaded, rebuilt.root, target)
                        .unwrap()
                        .code,
                    "{}:{target}",
                    edit["id"]
                );
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
            for target in ["JavaScript", "Rust"] {
                assert!(
                    emit_typed_program_links(&network, root, target).is_err(),
                    "{}:{target}",
                    edit["id"]
                );
            }
        }
    }
    assert_eq!(aliases, 74);
}
