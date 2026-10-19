//! Public production paths for the frozen Rust lexical corpus. Native execution is CI-only.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::diagnostics::ErrorKind;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::ir::{Decl, LitValue, Node, Program};
use meta_language::translation::lexer::{TokenKind, tokenize};
use meta_language::translation::{Language, Span};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, TypedProgramLinks, create_typed_program_links,
    decode_link_network_binary, decode_typed_program_links, emit_typed_program_links,
    encode_link_network_binary, encode_typed_program_links, read_structured_data_lino,
};
use serde_json::Value;
use std::fmt::Write;
use std::fs;
use std::process::Command;

const WHITESPACE: [u32; 11] = [9, 10, 11, 12, 13, 32, 133, 8206, 8207, 8232, 8233];

fn cooked_fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-cooked-string-escapes.lino"
    ))
    .unwrap()
}

fn unicode_fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-unicode-escapes.lino"
    ))
    .unwrap()
}

fn source_for(body: &str) -> String {
    format!(
        "// 😀 α\nfn choose()->String{{String::from(\"{body}\")}}fn main(){{println!(\"{{}}\",choose());}}"
    )
}

fn original_offset(source: &str, needle: &str) -> usize {
    source[..source.find(needle).expect("original spelling")]
        .encode_utf16()
        .count()
}

fn archives(packet: &TypedProgramLinks) -> [LinkNetwork; 2] {
    [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ]
}

fn string_value<'a>(program: &'a Program, name: &str) -> &'a str {
    let Some(Decl::Fn(function)) = program.declaration(name) else {
        panic!("the actual function exists")
    };
    let Node::Lit {
        value: LitValue::Text(value),
    } = &function.body.node
    else {
        panic!("the actual function holds a string literal")
    };
    value
}

#[test]
fn all_ascii_values_keep_original_raw_spelling_unicode_offsets_and_exact_consumption() {
    let fixture = cooked_fixture();
    assert_eq!(fixture["ascii"].as_array().unwrap().len(), 128);
    for value in fixture["ascii"].as_array().unwrap() {
        let code = u32::try_from(value.as_u64().unwrap()).unwrap();
        let expected = char::from_u32(code).unwrap();
        for escape in [format!("\\x{code:02x}"), format!("\\x{code:02X}")] {
            let literal = format!("\"α😀{escape}β\"");
            let source = format!("// 😀\n{literal}");
            let tokens = tokenize(&source, Language::Rust).unwrap();
            let token = &tokens.tokens[0];
            assert_eq!(token.kind, TokenKind::String);
            assert_eq!(token.value, format!("α😀{expected}β"), "{escape}");
            assert_eq!(token.raw, literal);
            assert_eq!(token.start, original_offset(&source, &literal));
            assert_eq!(token.end, source.encode_utf16().count());
            assert_eq!(
                &source.as_bytes()[source.find(&literal).unwrap()..],
                literal.as_bytes()
            );
        }
    }
    for (spelling, values) in [
        ("suffixEscapes", "suffixValues"),
        ("simpleEscapes", "simpleValues"),
    ] {
        for (escape, expected) in fixture[spelling]
            .as_array()
            .unwrap()
            .iter()
            .zip(fixture[values].as_array().unwrap())
        {
            let literal = format!("\"{}\"", escape.as_str().unwrap());
            assert_eq!(
                tokenize(&literal, Language::Rust).unwrap().tokens[0].value,
                expected.as_str().unwrap()
            );
        }
    }
    assert_eq!(
        tokenize(r#""\x0a  b""#, Language::Rust).unwrap().tokens[0].value,
        "\n  b"
    );
}

#[test]
fn public_import_refuses_high_ascii_and_malformed_escapes_at_the_original_backslash() {
    let fixture = cooked_fixture();
    assert_eq!(fixture["nonAscii"].as_array().unwrap().len(), 128);
    assert_eq!(fixture["invalidEscapes"].as_array().unwrap().len(), 15);
    let mut invalid = fixture["nonAscii"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| format!("\\x{:02x}", value.as_u64().unwrap()))
        .collect::<Vec<_>>();
    invalid.extend(
        fixture["invalidEscapes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned()),
    );
    for escape in invalid {
        let source = source_for(&escape);
        let start = original_offset(&source, "\\");
        let failure = tokenize(&source, Language::Rust).unwrap_err();
        assert_eq!(failure.kind, ErrorKind::Syntax, "{escape}");
        assert_eq!(failure.span, Some(Span::new(start, start + 2)), "{escape}");
        assert!(
            failure.reason.starts_with("unsupported string escape")
                || failure.reason == "malformed Rust ASCII escape"
                || failure.reason == "malformed unicode escape",
            "{escape}: {failure}"
        );
        let public = create_typed_program_links(&source, "Rust").unwrap_err();
        assert!(public.message.contains(&failure.to_string()), "{public}");
    }
    for source in ["\"\\", "\"\\x", "\"\\x0"] {
        assert_eq!(
            tokenize(source, Language::Rust).unwrap_err().kind,
            ErrorKind::Syntax
        );
    }
}

#[test]
fn isolated_source_carriage_returns_refuse_but_crlf_and_escaped_values_remain_distinct() {
    let fixture = cooked_fixture();
    assert_eq!(
        fixture["isolatedCarriageReturns"].as_array().unwrap().len(),
        4
    );
    for value in fixture["isolatedCarriageReturns"].as_array().unwrap() {
        let body = value.as_str().unwrap();
        let source = source_for(body);
        let start = original_offset(&source, "\r");
        let failure = tokenize(&source, Language::Rust).unwrap_err();
        assert_eq!(failure.kind, ErrorKind::Syntax);
        assert_eq!(failure.reason, "isolated carriage return in a Rust string");
        assert_eq!(failure.span, Some(Span::new(start, start + 1)));
        assert!(create_typed_program_links(&source, "Rust").is_err());
        for language in [Language::Lean, Language::Rocq] {
            let literal = format!("\"{body}\"");
            assert_eq!(tokenize(&literal, language).unwrap().tokens[0].value, body);
        }
    }
    for (body, expected) in [
        ("a\r\nb", "a\nb"),
        ("a\\r\\x0db", "a\r\rb"),
        ("a\\\r\n  b", "ab"),
    ] {
        let packet = create_typed_program_links(&source_for(body), "Rust").unwrap();
        for network in archives(&packet) {
            let program = decode_typed_program_links(&network, packet.root).unwrap();
            assert_eq!(string_value(&program, "choose"), expected);
        }
    }
}

#[test]
fn every_linked_unicode_scalar_and_refusal_uses_the_original_full_source_span() {
    let fixture = unicode_fixture();
    assert_eq!(fixture["scalarCases"].as_array().unwrap().len(), 17);
    assert_eq!(fixture["refusalCases"].as_array().unwrap().len(), 16);
    for case in fixture["scalarCases"].as_array().unwrap() {
        let escape = case["escape"].as_str().unwrap();
        let source = source_for(escape);
        let literal = format!("\"{escape}\"");
        let tokens = tokenize(&source, Language::Rust).unwrap();
        let token = tokens
            .tokens
            .iter()
            .find(|token| token.raw == literal)
            .unwrap();
        assert_eq!(token.value, case["value"].as_str().unwrap());
        assert_eq!(token.start, original_offset(&source, &literal));
        assert_eq!(token.end, token.start + literal.encode_utf16().count());
        let packet = create_typed_program_links(&source, "Rust").unwrap();
        for network in archives(&packet) {
            let program = decode_typed_program_links(&network, packet.root).unwrap();
            assert_eq!(
                string_value(&program, "choose"),
                case["value"].as_str().unwrap()
            );
        }
        for end in 1..escape.len() {
            let truncated = source_for(&escape[..end]);
            assert!(
                tokenize(&truncated, Language::Rust).is_err(),
                "{escape} at {end}"
            );
        }
    }
    for case in fixture["refusalCases"].as_array().unwrap() {
        let escape = case["escape"].as_str().unwrap();
        let source = source_for(escape);
        if let Some(expected) = case.get("sourceValue") {
            let packet = create_typed_program_links(&source, "Rust").unwrap();
            let program = decode_typed_program_links(&packet.network, packet.root).unwrap();
            assert_eq!(string_value(&program, "choose"), expected.as_str().unwrap());
            continue;
        }
        let unsupported = case["result"]["$"] == "unsupported";
        let failure = tokenize(&source, Language::Rust).unwrap_err();
        assert_eq!(
            failure.kind,
            if unsupported {
                ErrorKind::Unsupported
            } else {
                ErrorKind::Syntax
            }
        );
        let start = original_offset(&source, escape);
        assert_eq!(
            failure.span,
            Some(Span::new(
                start,
                start + if unsupported { escape.len() } else { 2 }
            ))
        );
        assert!(create_typed_program_links(&source, "Rust").is_err());
    }
}

#[test]
fn arbitrarily_long_unicode_separators_preserve_original_end_and_do_not_reprocess_values() {
    let separators = "_".repeat(16_384);
    let escape = format!(
        "\\u{{0{separators}0{separators}0{separators}0{separators}4{separators}1{separators}}}"
    );
    let source = source_for(&escape);
    let literal = format!("\"{escape}\"");
    let tokens = tokenize(&source, Language::Rust).unwrap();
    let token = tokens
        .tokens
        .iter()
        .find(|token| token.raw == literal)
        .unwrap();
    assert_eq!(token.value, "A");
    assert_eq!(token.start, original_offset(&source, &literal));
    assert_eq!(token.end, token.start + literal.encode_utf16().count());
    assert_eq!(
        &source.as_bytes()[source.find(&literal).unwrap()..][..literal.len()],
        literal.as_bytes()
    );
    for literal in [r#""\\u{41}""#, r#""\u{5_c}u{41}""#] {
        assert_eq!(
            tokenize(literal, Language::Rust).unwrap().tokens[0].value,
            r"\u{41}"
        );
    }
    assert!(tokenize(&format!("\"\\u{{000000{separators}0}}\""), Language::Rust).is_err());
}

#[test]
fn all_eleven_rust_whitespace_units_and_only_the_first_bom_keep_public_source_offsets() {
    for unit in WHITESPACE {
        let separator = char::from_u32(unit).unwrap();
        let source = format!(
            "\u{feff}// 😀\npub{separator}fn identity(value:i32)->i32{{value}}\nfn main(){{}}"
        );
        let tokens = tokenize(&source, Language::Rust).unwrap();
        assert_eq!(tokens.tokens[0].value, "pub");
        assert_eq!(tokens.tokens[0].start, original_offset(&source, "pub"));
        let first_fn = tokens
            .tokens
            .iter()
            .find(|token| token.value == "fn")
            .unwrap();
        assert_eq!(first_fn.start, original_offset(&source, "fn"));
        assert_eq!(
            tokens.tokens.last().unwrap().end,
            source.encode_utf16().count()
        );
        let packet = create_typed_program_links(&source, "Rust").unwrap();
        for network in archives(&packet) {
            let program = decode_typed_program_links(&network, packet.root).unwrap();
            let Some(Decl::Fn(function)) = program.declaration("identity") else {
                panic!("identity function")
            };
            assert_eq!(function.span.unwrap().start, original_offset(&source, "fn"));
            assert!(
                emit_typed_program_links(&network, packet.root, "JavaScript")
                    .unwrap()
                    .code
                    .contains("function identity(")
            );
        }
    }
    for unit in [
        0, 8, 14, 31, 0xa0, 0x1680, 0x2000, 0x2001, 0x2002, 0x2003, 0x2004, 0x2005, 0x2006, 0x2007,
        0x2008, 0x2009, 0x200a, 0x202f, 0x205f, 0x3000, 0xfeff,
    ] {
        let separator = char::from_u32(unit).unwrap();
        let source = format!("pub{separator}fn identity(value:i32)->i32{{value}}fn main(){{}}");
        let tokens = tokenize(&source, Language::Rust).unwrap();
        let spelling = separator.to_string();
        let token = tokens
            .tokens
            .iter()
            .find(|token| token.raw == spelling)
            .unwrap();
        assert_eq!(token.kind, TokenKind::Punct);
        assert_eq!(token.start, 3);
        assert!(create_typed_program_links(&source, "Rust").is_err());
    }
    for prefix in ["\u{feff}\u{feff}", " \u{feff}", "// initial line\n\u{feff}"] {
        assert!(create_typed_program_links(&format!("{prefix}fn main(){{}}"), "Rust").is_err());
    }
    assert_eq!(
        tokenize("\"\u{feff}\u{85}\u{200e}\u{a0}\"", Language::Rust)
            .unwrap()
            .tokens[0]
            .value,
        "\u{feff}\u{85}\u{200e}\u{a0}"
    );
    let comment = "// \u{feff}\u{85}\u{200e}\u{a0}";
    assert_eq!(
        tokenize(&format!("{comment}\nfn main(){{}}"), Language::Rust)
            .unwrap()
            .comments[0]
            .text,
        comment
    );
}

#[test]
fn whitespace_composition_preserves_exact_integer_suffixes_and_all_old_boundary_refusals() {
    for suffix in [
        "u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "u128", "i128", "usize", "isize",
    ] {
        for unit in WHITESPACE {
            let separator = char::from_u32(unit).unwrap();
            let source = format!("12_345_{suffix}{separator}");
            let tokens = tokenize(&source, Language::Rust).unwrap();
            assert_eq!(tokens.tokens[0].value, "12345");
            assert_eq!(tokens.tokens[0].suffix, suffix);
            assert_eq!(tokens.tokens[0].raw, format!("12_345_{suffix}"));
        }
    }
    for literal in [
        "1_",
        "1__",
        "1_u",
        "1_u7",
        "1_u64extra",
        "1_u64_",
        "1_u64_foo",
        "1_U64",
        "1_unknown",
        "1_u64é",
        "1_u64λ",
        "1_u64\u{301}",
        "1_u64𝔘",
        "1_u64\u{a0}",
    ] {
        let error = tokenize(literal, Language::Rust).unwrap_err();
        assert_eq!(error.kind, ErrorKind::Syntax);
        assert!(
            error.reason.starts_with("malformed number"),
            "{literal}: {error}"
        );
    }
    for literal in [
        "1_",
        "1__0",
        "1_u64",
        "9_007_199_254_740_992_u64",
        "1_0n_",
        "0xff_u8",
        "08",
        "1.5n",
        "3in",
        "0x_1",
        "1e_2",
    ] {
        assert_eq!(
            tokenize(literal, Language::JavaScript).unwrap_err().kind,
            ErrorKind::Syntax
        );
    }
    let source = "fn f(n: u64) -> u64 {\n  1_\n}\nfn main() { println!(\"x\"); }\n";
    let error = tokenize(source, Language::Rust).unwrap_err();
    let start = original_offset(source, "1_");
    assert_eq!(error.reason, "malformed number 1_");
    assert_eq!(error.span, Some(Span::new(start, start + 2)));
}

#[test]
fn other_languages_conserve_published_native_baselines_without_a_lean_parity_claim() {
    // The existing JS Lean lexer accepts b/f/v; the published native lexer
    // refuses them. Preserve each baseline in this Rust-specific change.
    // These controls are not official Lean or cross-runtime support evidence.
    for escape in [r"\b", r"\f", r"\v"] {
        let source = format!("\"{escape}\"");
        assert!(tokenize(&source, Language::Lean).is_err());
        assert_eq!(
            tokenize(&source, Language::Rocq).unwrap().tokens[0].value,
            escape
        );
    }
    for (escape, lean) in [(r"\n", "\n"), (r"\r", "\r"), (r"\t", "\t"), (r"\{", r"\{")] {
        let source = format!("\"{escape}\"");
        assert_eq!(
            tokenize(&source, Language::Lean).unwrap().tokens[0].value,
            lean
        );
        assert_eq!(
            tokenize(&source, Language::Rocq).unwrap().tokens[0].value,
            escape
        );
    }
    assert_eq!(
        tokenize(r#""\b\f\v\{""#, Language::JavaScript)
            .unwrap()
            .tokens[0]
            .value,
        "\u{8}\u{c}\u{b}{"
    );
    assert_eq!(
        tokenize(r#""\xE9""#, Language::JavaScript).unwrap().tokens[0].value,
        "é"
    );
    assert!(tokenize(r#""\00""#, Language::JavaScript).is_err());
    for language in [Language::JavaScript, Language::Lean] {
        assert!(tokenize(r#""\u{4_1_}""#, language).is_err());
    }
    assert_eq!(
        tokenize(r#""\u{4_1_}""#, Language::Rocq).unwrap().tokens[0].value,
        r"\u{4_1_}"
    );
    for language in [Language::JavaScript, Language::Lean, Language::Rocq] {
        for unit in [
            9, 10, 11, 12, 13, 32, 0xa0, 0x1680, 0x2000, 0x2028, 0x2029, 0x202f, 0x205f, 0x3000,
            0xfeff,
        ] {
            let source = format!("{}name", char::from_u32(unit).unwrap());
            let tokens = tokenize(&source, language).unwrap();
            assert_eq!(tokens.tokens[0].value, "name");
            assert_eq!(tokens.tokens[0].start, 1);
        }
    }
}

fn complete_program() -> (String, String, String) {
    let ascii = cooked_fixture();
    let mut escaped = String::new();
    let mut expected_ascii = String::new();
    for value in ascii["ascii"].as_array().unwrap() {
        let code = u32::try_from(value.as_u64().unwrap()).unwrap();
        write!(&mut escaped, "\\x{code:02x}").unwrap();
        expected_ascii.push(char::from_u32(code).unwrap());
    }
    let unicode = unicode_fixture();
    let mut scalar_escapes = String::new();
    let mut expected_unicode = String::new();
    for case in unicode["scalarCases"].as_array().unwrap() {
        scalar_escapes.push_str(case["escape"].as_str().unwrap());
        expected_unicode.push_str(case["value"].as_str().unwrap());
    }
    let separators = "_".repeat(16_384);
    let long = format!(
        "\\u{{0{separators}0{separators}0{separators}0{separators}4{separators}1{separators}}}"
    );
    let whitespace: String = WHITESPACE
        .into_iter()
        .map(|unit| char::from_u32(unit).unwrap())
        .collect();
    let source = format!(
        "\u{feff}// 😀 full original input\npub{whitespace}fn identity(value:i32)->i32{{value}}\nfn choose()->String{{String::from(\"{escaped}\")}}\nfn unicode()->String{{String::from(\"{scalar_escapes}\")}}\nfn long_value()->String{{String::from(\"{long}\")}}\nfn exact()->u64{{9_007_199_254_740_992_u64\u{200e}}}\nfn main(){{println!(\"{{}}\",choose());println!(\"{{}}\",unicode());println!(\"{{}}\",long_value());println!(\"{{}}\",exact());println!(\"{{}}\",identity(7i32));}}"
    );
    (source, expected_ascii, expected_unicode)
}

fn observe_javascript(code: &str, ascii: &str, unicode: &str) {
    let directory = TemporaryDirectory::new("rust-lexical-javascript");
    let file = directory.0.join("observations.cjs");
    let script = format!(
        "const assert=require('node:assert/strict');\n{code}\nassert.deepEqual([...Buffer.from(choose(),'utf8')],{});\nassert.deepEqual([...Buffer.from(unicode(),'utf8')],{});\nassert.equal(long_value(),'A');assert.equal(identity(7n),7n);assert.equal(exact(),9007199254740992n);",
        serde_json::to_string(ascii.as_bytes()).unwrap(),
        serde_json::to_string(unicode.as_bytes()).unwrap()
    );
    fs::write(&file, script).unwrap();
    let observed = run(Command::new("node").arg(&file));
    assert!(
        observed.ok,
        "JavaScript lexical observations failed\n{}\n{}",
        observed.stdout, observed.stderr
    );
}

fn compile_and_observe(code: &str, ascii: &str, unicode: &str) {
    let directory = TemporaryDirectory::new("rust-lexical-native");
    let source = directory.0.join("witness.rs");
    let binary = directory
        .0
        .join(format!("witness{}", std::env::consts::EXE_SUFFIX));
    // Compile the complete original or actual emitted artifact byte-for-byte;
    // execute its own main, without extracting or replacing any definition.
    fs::write(&source, code).unwrap();
    let mut build_command = Command::new("rustc");
    build_command.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        build_command.arg("-C").arg(format!("linker={linker}"));
    }
    let compiled = run(build_command.arg(&source).arg("-o").arg(&binary));
    assert!(
        compiled.ok,
        "Rust lexical compilation failed\n{}\n{}",
        compiled.stdout, compiled.stderr
    );
    let observed = run(&mut Command::new(&binary));
    assert!(
        observed.ok,
        "Rust lexical observations failed\n{}\n{}",
        observed.stdout, observed.stderr
    );
    assert_eq!(
        observed.stdout,
        format!("{ascii}\n{unicode}\nA\n9007199254740992\n7\n")
    );
}

#[test]
fn full_original_and_emitted_programs_execute_after_text_binary_reload_and_source_free_edit() {
    let (source, expected_ascii, expected_unicode) = complete_program();
    let packet = create_typed_program_links(&source, "Rust").unwrap();
    let original = decode_typed_program_links(&packet.network, packet.root).unwrap();
    assert_eq!(string_value(&original, "choose"), expected_ascii);
    assert_eq!(string_value(&original, "unicode"), expected_unicode);
    compile_and_observe(&source, &expected_ascii, &expected_unicode);
    for network in archives(&packet) {
        let mut program = decode_typed_program_links(&network, packet.root).unwrap();
        assert_eq!(program, original);
        let javascript = emit_typed_program_links(&network, packet.root, "JavaScript").unwrap();
        observe_javascript(&javascript.code, &expected_ascii, &expected_unicode);
        let emitted = emit_typed_program_links(&network, packet.root, "Rust").unwrap();
        compile_and_observe(&emitted.code, &expected_ascii, &expected_unicode);
        // Every actual emitted definition reimports as a complete source program.
        // The native thread/process wrapper remains a separate frontend boundary.
        let definitions = emit_rust(&program).unwrap().definitions.join("\n\n");
        let replay =
            create_typed_program_links(&format!("{definitions}\nfn main(){{}}"), "Rust").unwrap();
        let reread = decode_typed_program_links(&replay.network, replay.root).unwrap();
        assert_eq!(
            emit_rust(&reread).unwrap().definitions.join("\n\n"),
            definitions
        );
        assert_eq!(string_value(&reread, "choose"), expected_ascii);
        let Some(Decl::Fn(function)) = program
            .declarations
            .iter_mut()
            .find(|item| item.full_name() == "choose")
        else {
            panic!("choose function")
        };
        let edited = "edited\0\u{8}\u{c}\u{b}{α😀";
        function.body.node = Node::Lit {
            value: LitValue::Text(edited.to_owned()),
        };
        let saved = encode_typed_program_links(&program).unwrap();
        for changed in archives(&saved) {
            assert_eq!(
                string_value(
                    &decode_typed_program_links(&changed, saved.root).unwrap(),
                    "choose"
                ),
                edited
            );
            observe_javascript(
                &emit_typed_program_links(&changed, saved.root, "JavaScript")
                    .unwrap()
                    .code,
                edited,
                &expected_unicode,
            );
        }
        compile_and_observe(
            &emit_typed_program_links(&saved.network, saved.root, "Rust")
                .unwrap()
                .code,
            edited,
            &expected_unicode,
        );
    }
}
