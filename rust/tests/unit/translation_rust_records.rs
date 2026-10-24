//! Future complete paired graph/codec/original-Rust controls. CI compilation only.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::Language;
use meta_language::translation::lexer::{TokenCursor, TokenKind, tokenize};
use meta_language::translation::rust::parse_rust_with_source_signature_model;
use meta_language::translation::{check::check_program, emit_rust::emit_rust, rust::parse_rust};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, TypedProgramLinks, TypedProgramLinksEmission,
    TypedProgramLinksError, create_rust_record_program_links, decode_link_network_binary,
    emit_typed_program_links, encode_link_network_binary, encode_typed_program_links,
    read_structured_data_lino,
};
use serde_json::{Value, json};
use std::fs;
use std::process::Command;

fn emission(value: TypedProgramLinksEmission) -> Value {
    json!({"code":value.code,"language":value.language,"mappings":value.mappings,"assumptions":value.assumptions,"encodings":value.encodings,"theorems":value.theorems})
}

#[test]
fn nominal_records_preserve_complete_graphs_owners_order_and_original_values() {
    assert_nominal_translation_product(
        "../experiments/rust-records/oracle.mjs",
        "Rust nominal records",
        create_rust_record_program_links,
    );
}

#[test]
fn named_fields_preserve_declarations_calls_patterns_and_mixed_encoding() {
    assert_nominal_translation_product(
        "../experiments/rust-records/layout.mjs",
        "Rust named constructor fields",
        create_rust_record_program_links,
    );
}

#[test]
fn raw_identifiers_preserve_names_tokens_cursor_meaning_and_original_values() {
    assert_nominal_translation_product(
        "../experiments/rust-records/raw-identifiers.mjs",
        "Rust raw identifiers",
        create_rust_record_program_links,
    );
}

#[test]
fn nominal_bindings_preserve_complete_graphs_codecs_and_original_values() {
    assert_nominal_translation_product(
        "../experiments/rust-records/bindings.mjs",
        "Rust nominal bindings",
        create_rust_record_program_links,
    );
}

#[test]
fn tuple_patterns_preserve_complete_products_and_original_values() {
    assert_nominal_translation_product(
        "../experiments/rust-records/tuple-patterns.mjs",
        "Rust tuple patterns",
        create_rust_record_program_links,
    );
}

#[test]
fn preceding_tuple_struct_input_retains_complete_corrected_controls() {
    assert_nominal_translation_product(
        "../experiments/rust-records/product-correction-tuple-struct.mjs",
        "preceding tuple struct",
        create_rust_record_program_links,
    );
}

#[test]
fn preceding_tuple_match_input_retains_complete_corrected_controls() {
    assert_nominal_translation_product(
        "../experiments/rust-records/product-correction-tuple-match.mjs",
        "preceding tuple match",
        create_rust_record_program_links,
    );
}

#[test]
fn conditional_patterns_preserve_scopes_graphs_and_original_values() {
    assert_nominal_translation_product(
        "../experiments/rust-records/pattern-conditions.mjs",
        "Rust conditional patterns",
        create_rust_record_program_links,
    );
}

#[test]
fn let_else_preserves_fallback_scopes_graphs_and_original_values() {
    assert_nominal_translation_product(
        "../experiments/rust-records/let-else.mjs",
        "Rust let-else",
        create_rust_record_program_links,
    );
}

pub(super) fn assert_nominal_translation_product(
    producer_path: &str,
    product_label: &str,
    create_definition: fn() -> Result<TypedProgramLinks, TypedProgramLinksError>,
) {
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let version = version.stdout.split_whitespace().nth(1).unwrap();
    let parts = version
        .split('.')
        .map(|part| part.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {version}; >=1.90 <1.100"
    );
    let producer = run(Command::new("node")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--max-old-space-size=352",
            "--max-semi-space-size=1",
            producer_path,
        ]));
    assert!(producer.ok, "{}", producer.stderr);
    let expected = read_structured_data_lino(&producer.stdout).unwrap();
    let source_language = match expected.get("sourceLanguage").and_then(Value::as_str) {
        None | Some("Rust") => Language::Rust,
        Some("JavaScript") => Language::JavaScript,
        Some(other) => panic!("unknown source control language: {other}"),
    };
    if let Some(rows) = expected["lexicalCases"].as_array() {
        for row in rows {
            let tokens = tokenize(row["source"].as_str().unwrap(), source_language)
                .unwrap()
                .tokens;
            let declared = row["tokens"].as_array().unwrap();
            assert_eq!(tokens.len(), declared.len() + 1);
            let mut cursor = TokenCursor::new(tokens, source_language);
            for expected in declared {
                let token = cursor.peek();
                let kind = match token.kind {
                    TokenKind::Identifier => "identifier",
                    TokenKind::Macro => "macro",
                    TokenKind::String => "string",
                    TokenKind::Punct => "punct",
                    TokenKind::Character => "char",
                    _ => panic!("unexpected declared token {token:?}"),
                };
                assert_eq!(
                    json!({"raw":token.raw,"kind":kind,"value":token.value,"start":token.start,"end":token.end}),
                    json!({"raw":expected["raw"],"kind":expected["kind"],"value":expected["value"],"start":expected["start"],"end":expected["end"]})
                );
                assert_eq!(
                    cursor.is(expected["value"].as_str().unwrap()),
                    expected["matchesValue"].as_bool().unwrap()
                );
                cursor.advance();
            }
            assert_eq!(cursor.peek().kind, TokenKind::Eof);
        }
    }
    if let Some(rows) = expected["unitLexicalCases"].as_array() {
        for row in rows {
            let tokens = tokenize(row["source"].as_str().unwrap(), source_language)
                .unwrap()
                .tokens;
            assert_eq!(tokens.len(), 2);
            let token = &tokens[0];
            let (value, units) = if row["kind"] == "template" {
                assert_eq!(token.kind, TokenKind::Template);
                assert_eq!(token.parts.len(), 1);
                let part = &token.parts[0];
                (
                    &part.text,
                    part.string_units
                        .clone()
                        .unwrap_or_else(|| part.text.encode_utf16().collect()),
                )
            } else {
                assert_eq!(token.kind, TokenKind::String);
                (
                    &token.value,
                    token
                        .string_units
                        .clone()
                        .unwrap_or_else(|| token.value.encode_utf16().collect()),
                )
            };
            assert_eq!(value, row["value"].as_str().unwrap());
            assert_eq!(serde_json::to_value(units).unwrap(), row["units"]);
        }
    }
    if let Some(rows) = expected["lexicalErrors"].as_array() {
        for row in rows {
            let error = tokenize(row["source"].as_str().unwrap(), source_language).unwrap_err();
            assert_eq!(error.kind.as_str(), row["kind"].as_str().unwrap());
            assert_eq!(error.reason, row["reason"].as_str().unwrap());
            assert_eq!(serde_json::to_value(error.span).unwrap(), row["span"]);
        }
    }
    let surface = if source_language == Language::Rust {
        parse_rust(expected["source"].as_str().unwrap())
    } else {
        meta_language::translation::javascript::parse_javascript(
            expected["source"].as_str().unwrap(),
        )
    }
    .unwrap();
    let program = check_program(&surface).unwrap();
    if let Some(rows) = expected["syntaxCases"].as_array() {
        for row in rows {
            let error = meta_language::translation::javascript::parse_javascript(
                row["source"].as_str().unwrap(),
            )
            .unwrap_err();
            assert_eq!(error.kind.as_str(), row["kind"].as_str().unwrap());
            assert_eq!(row["originalSyntax"]["source"], row["source"]);
            assert_ne!(row["originalSyntax"]["exitCode"], 0);
            assert!(row["originalSyntax"]["signal"].is_null());
            assert!(
                row["originalSyntax"]["stderr"]
                    .as_str()
                    .unwrap()
                    .contains("SyntaxError")
            );
        }
    }
    let packet = encode_typed_program_links(&program).unwrap();
    assert_eq!(packet.root.as_u64(), expected["root"].as_u64().unwrap());
    assert_eq!(
        packet.network.to_lino(),
        expected["links"].as_str().unwrap()
    );
    let binary =
        encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap();
    for network in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
    ] {
        for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
            assert_eq!(
                emission(emit_typed_program_links(&network, packet.root, target).unwrap()),
                expected["emissions"][target]
            );
        }
    }
    let definition = create_definition().unwrap();
    assert_eq!(
        definition.root.as_u64(),
        expected["definitionRoot"].as_u64().unwrap()
    );
    assert_eq!(
        definition.network.to_lino(),
        expected["definitionLinks"].as_str().unwrap()
    );
    let emitted = emit_rust(&program).unwrap();
    let header = emitted
        .text
        .lines()
        .find(|line| line.starts_with("#![allow("))
        .unwrap()
        .to_owned();
    let library = std::iter::once(header)
        .chain(emitted.preludes)
        .chain(emitted.definitions)
        .collect::<Vec<_>>()
        .join("\n\n");
    assert_eq!(library, expected["nativeLibrary"].as_str().unwrap());
    if let Some(layout) = expected.get("layout") {
        for piece in layout["pieces"].as_array().unwrap() {
            assert!(emitted.text.contains(piece.as_str().unwrap()), "{piece}");
        }
        assert!(emitted.text.lines().any(|line| {
            line.contains(layout["patternPrefix"].as_str().unwrap()) && line.contains(" } => ")
        }));
        assert_eq!(
            emitted
                .encodings
                .iter()
                .find(|encoding| encoding.id == "data")
                .unwrap()
                .statement,
            layout["encoding"].as_str().unwrap()
        );
    }
    let mut main = r#"
fn ml_record_result(result:std::thread::Result<String>)->(&'static str,String,Option<String>,Option<Vec<u16>>,Option<bool>){
 match result{Ok(value)=>("value",value,None,None,None),Err(error)=>{
  if let Some((class_name,message_units,present))=error.downcast_ref::<(String,Vec<u16>,bool)>(){return ("abort",String::from_utf16_lossy(message_units),Some(class_name.clone()),Some(message_units.clone()),Some(*present));}
  if let Some((class_name,message_units))=error.downcast_ref::<(String,Vec<u16>)>(){return ("abort",String::from_utf16_lossy(message_units),Some(class_name.clone()),Some(message_units.clone()),None);}
  let message=error.downcast_ref::<&str>().map(|value|(*value).to_owned()).or_else(||error.downcast_ref::<String>().cloned()).expect("the original panic retains its actual text");
  let units=message.encode_utf16().collect();
  ("abort",message,Some("Error".to_owned()),Some(units),Some(true))
 }}
}
fn main(){std::panic::set_hook(Box::new(|_|{}));let mut failures=Vec::new();
"#.to_owned();
    for row in expected["cases"].as_array().unwrap() {
        let module = row["module"].as_str().unwrap();
        if row.get("previousExpectation").is_some() {
            assert!(
                expected["source"]
                    .as_str()
                    .unwrap()
                    .starts_with(row["source"].as_str().unwrap())
            );
            let correction = row.get("correctionKind").and_then(Value::as_str);
            let preceding = match correction {
                Some("tuple-struct") => {
                    json!({"kind":"unsupported","messagePattern":"Rust struct item"})
                }
                Some("tuple-match") => {
                    json!({"kind":"unsupported","messagePattern":"tuple match pattern"})
                }
                _ => {
                    let preceding_reason = match correction {
                        None => {
                            "Rust record update: update syntax needs complete move and field provenance"
                        }
                        Some("named lifetime receiver") => {
                            "Rust mutable or explicit lifetime receiver: receiver mutation and named regions need an explicit ownership interpretation"
                        }
                        Some("explicit receiver type") => {
                            "Rust explicit receiver type: explicit receiver types need their complete arbitrary-self interpretation"
                        }
                        Some("boxed receiver needs dereference model") => {
                            "Rust inherent receiver: the method needs its actual owned or single immutable borrowed receiver"
                        }
                        Some(other) => panic!("unregistered expectation correction: {other}"),
                    };
                    json!({"kind":"unsupported","reason":preceding_reason})
                }
            };
            assert_eq!(row["previousExpectation"], preceding);
        }
        let native = row["nativeTarget"].as_str().unwrap().replace('.', "::");
        let (kind, value) = if let Some(message) = row["abort"].as_str() {
            ("abort", message)
        } else {
            ("value", row["value"].as_str().unwrap())
        };
        let mut expected_observation = json!({"kind":kind,"value":value});
        if let Some(class) = row.get("errorClass") {
            expected_observation["errorClass"] = class.clone();
            expected_observation["messageUnits"] = row["messageUnits"].clone();
            if let Some(present) = row.get("messagePresent") {
                expected_observation["messagePresent"] = present.clone();
            }
        }
        assert_eq!(row["observed"], expected_observation, "{}", row["name"]);
        let mut calls = Vec::new();
        if source_language == Language::Rust {
            calls.push((
                "original",
                if module.is_empty() {
                    "original::result()".to_owned()
                } else {
                    format!("original::{module}::result()")
                },
            ));
        } else {
            assert_eq!(
                row["originalObserved"], expected_observation,
                "{}",
                row["name"]
            );
        }
        calls.push(("translated", format!("{native}()")));
        for (owner, call) in calls {
            let class_check = if let Some(class) = row.get("errorClass").and_then(Value::as_str) {
                let units = row["messageUnits"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|unit| u16::try_from(unit.as_u64().unwrap()).unwrap())
                    .collect::<Vec<_>>();
                let presence_check = row
                    .get("messagePresent")
                    .and_then(Value::as_bool)
                    .map_or_else(String::new, |present| {
                        format!("||observed.4!=Some({present})")
                    });
                format!(
                    "if observed.2.as_deref()!=Some({class:?})||observed.3.as_deref()!=Some(&{units:?}[..]){presence_check}{{failures.push(format!(\"{owner} {module} class/units: {{:?}}\",observed));}}"
                )
            } else {
                String::new()
            };
            main.push_str(&format!("{{let observed=ml_record_result(std::panic::catch_unwind(||({call}).to_string()));if (observed.0,observed.1.clone())!=({kind:?},{value:?}.to_owned()){{failures.push(format!(\"{owner} {module}: {{:?}}\",observed));}}{class_check}}}\n"));
        }
    }
    main.push_str("if !failures.is_empty(){eprintln!(\"{}\",failures.join(\"\n\"));}assert!(failures.is_empty());");
    main.push_str(&format!("println!({product_label:?});}}\n"));
    let original = if source_language == Language::Rust {
        format!(
            "\nmod original{{\n{}\n}}\n",
            expected["source"].as_str().unwrap()
        )
    } else {
        String::new()
    };
    let directory = TemporaryDirectory::new(product_label);
    let source = directory.0.join("product.rs");
    let binary = directory.0.join(if cfg!(windows) {
        "product.exe"
    } else {
        "product"
    });
    fs::write(&source, library + &original + &main).unwrap();
    let mut compiler = Command::new("rustc");
    compiler.args(["--edition", "2024", "-C", "overflow-checks=on"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        compiler.arg("-C").arg(format!("linker={linker}"));
    }
    let built = run(compiler.arg(&source).arg("-o").arg(&binary));
    assert!(built.ok, "{}", built.stderr);
    let observed = run(&mut Command::new(&binary));
    assert!(observed.ok, "{}", observed.stderr);
    assert_eq!(observed.stdout, format!("{product_label}\n"));
    for row in expected["refusals"].as_array().unwrap() {
        if expected["sourceSyntaxOracle"] == Value::Bool(true) {
            assert_eq!(source_language, Language::JavaScript);
            assert_eq!(row["kind"], "syntax");
            let original = &row["originalSyntax"];
            assert_eq!(original["source"], row["source"]);
            assert_ne!(original["exitCode"].as_i64().unwrap(), 0);
            assert!(original["signal"].is_null());
            assert!(original["stderr"].as_str().unwrap().contains("SyntaxError"));
        }
        let surface = if source_language == Language::JavaScript {
            meta_language::translation::javascript::parse_javascript(
                row["source"].as_str().unwrap(),
            )
        } else if let Some(model) = row["sourceSignatureModel"].as_str() {
            parse_rust_with_source_signature_model(row["source"].as_str().unwrap(), model)
        } else {
            parse_rust(row["source"].as_str().unwrap())
        };
        let result = surface.and_then(|surface| check_program(&surface));
        let error = result.expect_err(row["name"].as_str().unwrap());
        assert_eq!(error.kind.as_str(), row["kind"].as_str().unwrap());
        assert_eq!(error.reason, row["reason"].as_str().unwrap());
    }
}
