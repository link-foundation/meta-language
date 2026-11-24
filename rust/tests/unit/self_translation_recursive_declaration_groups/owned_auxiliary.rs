//! Execution and semantic codec checks for recursive declarations with owned auxiliary code.

use super::super::issue_195_translation_pairs::{run, sha256};
use super::{
    archives, compile_and_observe, compile_complete_and_observe, javascript_bridge,
    javascript_fixture_program, observe_complete_javascript, observe_javascript, policy, root,
    versions,
};
use meta_language::{
    SelfTranslationOptions, create_typed_program_links, decode_typed_program_links,
    emit_typed_program_links, encode_typed_program_links, read_structured_data_lino,
    self_translate, self_translate_with, self_translation_signatures,
};
use serde_json::Value;
use std::{collections::BTreeMap, fs, process::Command};

fn owned_auxiliary_fixture() -> Value {
    let fixture = policy()["ownedAuxiliaryFixtures"].clone();
    assert!(
        fixture.is_object(),
        "the normal complete-source author records the new fixture profile"
    );
    for input in fixture["inputs"].as_array().unwrap() {
        let bytes = fs::read(root().join(input["path"].as_str().unwrap())).unwrap();
        assert_eq!(
            u64::try_from(bytes.len()).unwrap(),
            input["bytes"].as_u64().unwrap()
        );
        assert_eq!(sha256(&bytes), input["sourceSha256"].as_str().unwrap());
        assert_eq!(
            String::from_utf8(bytes).unwrap(),
            input["source"].as_str().unwrap()
        );
    }
    fixture
}

fn owned_callable_rust_main(fixture: &Value) -> String {
    let facts = &fixture["statementFacts"];
    let texts = |value: &Value| {
        format!(
            "vec![{}]",
            value
                .as_array()
                .unwrap()
                .iter()
                .map(|value| format!(
                    "crate::ml_string::Text::from_text({:?})",
                    value.as_str().unwrap()
                ))
                .collect::<Vec<_>>()
                .join(",")
        )
    };
    let numbers = |value: &Value| {
        format!(
            "vec![{}]",
            value
                .as_array()
                .unwrap()
                .iter()
                .map(|value| format!("{}f64", value.as_f64().unwrap()))
                .collect::<Vec<_>>()
                .join(",")
        )
    };
    let call = format!(
        "read_callable_statement_uses({},names.clone(),{},{},{},{},{}f64,{})",
        texts(&facts["forms"]),
        numbers(&facts["starts"]),
        numbers(&facts["counts"]),
        texts(&facts["labels"]),
        numbers(&facts["targets"]),
        facts["root"].as_u64().unwrap(),
        texts(&fixture["statementFixture"]["bound"])
    );
    format!(
        r#"fn main(){{
        let names={};let events={};
        assert_eq!(events.len()%2,0);let mut free=Vec::<String>::new();let mut assigned=Vec::<String>::new();
        for pair in events.chunks_exact(2){{
            assert!(pair[1]>=0.0&&pair[1].fract()==0.0&&(pair[1]as usize)<names.len());
            let name=names[pair[1]as usize].to_string();
            if pair[0]==0.0&&!free.contains(&name){{free.push(name.clone());}}
            if pair[0]==1.0&&!assigned.contains(&name){{assigned.push(name);}}
        }}
        println!("{{}}",free.join("|"));println!("{{}}",assigned.join("|"));
    }}"#,
        texts(&facts["texts"]),
        call
    )
}

#[test]
fn actual_owned_loop_bodies_keep_every_helper_and_execute_through_both_semantic_codecs() {
    versions();
    let fixture = owned_auxiliary_fixture();
    let source = fixture["sources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|source| source.as_str().unwrap())
        .collect::<Vec<_>>()
        .join("\n\n");
    let expected = fixture["expected"].as_str().unwrap().replace('|', "\n") + "\n";
    observe_javascript(&source, &fixture["calls"], &expected, false);
    let actual = self_translate(&source, "JavaScript", "Rust").unwrap();
    let javascript = javascript_bridge(&source, "JavaScript", false);
    assert_eq!(
        actual.code,
        javascript["translation"]["code"].as_str().unwrap()
    );
    assert_eq!(
        actual
            .items
            .iter()
            .filter(|item| matches!(
                item.term.as_str(),
                "function_declaration" | "export_statement"
            ))
            .map(|item| item.status)
            .collect::<Vec<_>>(),
        vec!["translated", "translated"]
    );
    assert_eq!(
        self_translate(&actual.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        source
    );
    compile_and_observe(
        &actual.code,
        &fixture["rustCalls"],
        &expected,
        "owned-loop-source",
    );
    let complete = javascript_fixture_program(&source, &fixture["calls"]);
    let saved = create_typed_program_links(&complete, "JavaScript").unwrap();
    for network in archives(&saved.network) {
        let program = decode_typed_program_links(&network, saved.root).unwrap();
        assert!(program.declarations.iter().any(|declaration|matches!(declaration,meta_language::translation::ir::Decl::Data(data)if data.generated)));
        let native = emit_typed_program_links(&network, saved.root, "Rust")
            .unwrap()
            .code;
        compile_complete_and_observe(&native, &expected, "owned-loop-typed-codec");
        let javascript = emit_typed_program_links(&network, saved.root, "JavaScript")
            .unwrap()
            .code;
        observe_complete_javascript(&javascript, &expected, "owned-loop-js-codec");
    }
}

#[test]
fn actual_callable_module_imports_and_both_semantic_codecs_execute_the_original_statement_fixture()
{
    versions();
    let fixture = owned_auxiliary_fixture();
    let inputs = fixture["inputs"].as_array().unwrap();
    let facts = inputs[0]["source"].as_str().unwrap();
    let types = inputs[1]["source"].as_str().unwrap();
    let callable = inputs[2]["source"].as_str().unwrap();
    let base = SelfTranslationOptions {
        utf16_strings: true,
        ..Default::default()
    };
    let fact_signatures = self_translation_signatures(facts, "JavaScript", &base).unwrap();
    let fact_translation = self_translate_with(facts, "JavaScript", "Rust", &base).unwrap();
    let type_options = SelfTranslationOptions {
        imports: BTreeMap::from([("./fact-column-rules.js".to_owned(), fact_signatures.clone())]),
        ..base.clone()
    };
    let type_signatures = self_translation_signatures(types, "JavaScript", &type_options).unwrap();
    let type_translation = self_translate_with(types, "JavaScript", "Rust", &type_options).unwrap();
    let options = SelfTranslationOptions {
        imports: BTreeMap::from([
            ("./fact-column-rules.js".to_owned(), fact_signatures),
            ("./type-syntax-rules.js".to_owned(), type_signatures),
        ]),
        ..base
    };
    let actual = self_translate_with(callable, "JavaScript", "Rust", &options).unwrap();
    let script = r"
import {readOwnedRecursiveSourceFixtures} from './js/scripts/declaration-owned-auxiliary-fixtures.mjs';
import {selfTranslate,selfTranslationSignatures} from './js/src/self-translation.js';
import {structuredDataToLino} from './js/src/structured-data-links.js';
const f=readOwnedRecursiveSourceFixtures(),[facts,types,source]=f.inputs.map(row=>row.source);
const fs=selfTranslationSignatures(facts,'JavaScript',{utf16Strings:true});
const ts=selfTranslationSignatures(types,'JavaScript',{utf16Strings:true,imports:{'./fact-column-rules.js':fs}});
process.stdout.write(structuredDataToLino(selfTranslate(source,'JavaScript','Rust',{utf16Strings:true,imports:{'./fact-column-rules.js':fs,'./type-syntax-rules.js':ts}})));
";
    let output =
        run(Command::new("node")
            .current_dir(root())
            .args(["--input-type=module", "-e", script]));
    assert!(output.ok, "{}", output.stderr);
    let javascript = read_structured_data_lino(&output.stdout).unwrap();
    assert_eq!(actual.code, javascript["code"].as_str().unwrap());
    for name in ["readCallableStatementUses", "callableStatementUses"] {
        let owner = actual
            .items
            .iter()
            .find(|item| callable[item.start..item.end].contains(&format!("function {name}(")))
            .unwrap();
        assert_eq!(owner.status, "translated", "{name}: {:?}", owner.reason);
    }
    assert_eq!(
        self_translate(&actual.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        callable
    );
    let expected = fixture["statementFixture"]["expectedFree"]
        .as_array()
        .unwrap()
        .iter()
        .map(|name| name.as_str().unwrap())
        .collect::<Vec<_>>()
        .join("|")
        + "\n"
        + &fixture["statementFixture"]["expectedAssigned"]
            .as_array()
            .unwrap()
            .iter()
            .map(|name| name.as_str().unwrap())
            .collect::<Vec<_>>()
            .join("|")
        + "\n";
    let main = owned_callable_rust_main(&fixture);
    let combined = format!(
        "{}\nmod fact_column_rules{{\n{}\n}}\nmod type_syntax_rules{{\n{}\n}}\n{main}",
        actual.code, fact_translation.code, type_translation.code
    );
    compile_complete_and_observe(&combined, &expected, "actual-owned-callable-modules");
    // Native codec coverage uses the complete genuine existing callable
    // source/import closure. This is CI-only; no local native execution.
    let packet =
        create_typed_program_links(fixture["callableClosure"].as_str().unwrap(), "JavaScript")
            .unwrap();
    let mut checked = decode_typed_program_links(&packet.network, packet.root).unwrap();
    checked.utf16_strings = Some(true);
    let saved = encode_typed_program_links(&checked).unwrap();
    for network in archives(&saved.network) {
        let restored = decode_typed_program_links(&network, saved.root).unwrap();
        assert_eq!(restored, checked);
        assert!(
            restored
                .main
                .as_ref()
                .is_none_or(|main| main.effects.is_empty()),
            "library extraction cannot drop source effects"
        );
        let native = meta_language::translation::emit_rust::emit_rust_with_array_consumption(
            &restored,
            meta_language::translation::emit_rust::RustEmissionOptions::default(),
        )
        .unwrap();
        assert_eq!(
            native.text,
            emit_typed_program_links(&network, saved.root, "Rust")
                .unwrap()
                .code
        );
        // Use the normal library parts, retaining EVERY complete emitted
        // source declaration. Its empty driver is replaced by actual calls.
        let header = native
            .text
            .lines()
            .find(|line| line.starts_with("#![allow("))
            .unwrap();
        let library = std::iter::once(header)
            .chain(native.preludes.iter().map(String::as_str))
            .chain(native.definitions.iter().map(String::as_str))
            .collect::<Vec<_>>()
            .join("\n\n");
        compile_complete_and_observe(
            &format!("{library}\n{main}"),
            &expected,
            "actual-owned-callable-codec",
        );
    }
}
