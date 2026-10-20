//! Whole real Rust declarations, concrete preservation and translation execution.
//! Rust compilation and subprocess execution occur only in native CI.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::check::check_program;
use meta_language::translation::diagnostics::ErrorKind;
use meta_language::translation::rust::parse_rust;
use meta_language::{
    BinaryLinksOptions, LinkNetwork, ParseConfiguration, create_typed_program_links,
    decode_link_network_binary, decode_typed_program_links, emit_typed_program_links,
    encode_link_network_binary, read_structured_data_lino,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::fs;
use std::process::Command;

const JAVASCRIPT_WORDS: &str = "arguments await break case catch class const continue debugger default delete do else enum eval export extends false finally for function if implements import in instanceof interface let new null package private protected public return static super switch this throw true try typeof var void while with yield";
const RUST_WORDS: &str = "as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn";

fn fixture() -> Value {
    read_structured_data_lino(include_str!("../../src/data/rust-literal-match-rules.lino")).unwrap()
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
fn digest(source: &str) -> String {
    Sha256::digest(source.as_bytes())
        .iter()
        .fold(String::new(), |mut text, byte| {
            write!(text, "{byte:02x}").expect("writing a hexadecimal digest to String succeeds");
            text
        })
}
fn observe(source: &str, expected: &str, native: bool) {
    let directory = TemporaryDirectory::new("rust-literal-matches");
    let file = directory
        .0
        .join(if native { "program.rs" } else { "program.cjs" });
    fs::write(&file, source).unwrap();
    let result = if native {
        let binary = directory
            .0
            .join(format!("program{}", std::env::consts::EXE_SUFFIX));
        let mut command = Command::new("rustc");
        command.args(["--edition", "2024"]);
        #[cfg(windows)]
        if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
            command.arg("-C").arg(format!("linker={linker}"));
        }
        let compiled = run(command.arg(&file).arg("-o").arg(&binary));
        assert!(compiled.ok, "{}\n{}", compiled.stdout, compiled.stderr);
        run(&mut Command::new(binary))
    } else {
        run(Command::new("node").arg(&file))
    };
    assert!(result.ok, "{}\n{}", result.stdout, result.stderr);
    assert_eq!(result.stdout, expected);
}
fn actual_context(path: &str) -> &'static str {
    match path {
        "rust/src/grammar/emit/javascript.rs" => {
            include_str!("../../src/grammar/emit/javascript.rs")
        }
        "rust/src/grammar/emit/tree_sitter.rs" => {
            include_str!("../../src/grammar/emit/tree_sitter.rs")
        }
        "rust/src/grammar/emit/rust.rs" => include_str!("../../src/grammar/emit/rust.rs"),
        _ => {
            panic!("the complete finite own-source fixture retains its three original module paths")
        }
    }
}

#[test]
fn three_complete_own_keyword_declarations_keep_original_source_and_independent_execution_through_both_archives()
 {
    let fixture = fixture();
    let owners = fixture["originalDeclarations"].as_array().unwrap();
    assert_eq!(owners.len(), 3);
    for owner in owners {
        let context = actual_context(owner["path"].as_str().unwrap());
        assert_eq!(context, owner["context"].as_str().unwrap());
        assert_eq!(
            context.len(),
            usize::try_from(owner["bytes"].as_u64().unwrap()).unwrap()
        );
        assert_eq!(digest(context), owner["sourceSha256"].as_str().unwrap());
        let declaration = owner["declaration"].as_str().unwrap();
        assert!(context.contains(declaration));
        assert_eq!(
            digest(declaration),
            owner["declarationSha256"].as_str().unwrap()
        );
        let name = owner["name"].as_str().unwrap();
        assert!(declaration.starts_with(&format!("fn {name}(")));
        let words = if name == "is_rust_keyword" {
            RUST_WORDS
        } else {
            JAVASCRIPT_WORDS
        };
        let mut values = words.split(' ').map(str::to_owned).collect::<Vec<_>>();
        values.extend(["", "π", "😀", "While", "keyword"].map(str::to_owned));
        values.extend(words.split(' ').map(|word| format!("{word}_")));
        let mut source = format!("{declaration}\nfn main(){{");
        let mut expected = String::new();
        for value in &values {
            write!(source, "println!(\"{{}}\",{name}({value:?}));").unwrap();
            writeln!(
                expected,
                "{}",
                words.split(' ').any(|word| word == value.as_str())
            )
            .unwrap();
        }
        source.push('}');
        observe(&source, &expected, true);
        let packet = create_typed_program_links(&source, "Rust").unwrap();
        let original_rust = emit_typed_program_links(&packet.network, packet.root, "Rust").unwrap();
        let original_javascript =
            emit_typed_program_links(&packet.network, packet.root, "JavaScript").unwrap();
        for network in archives(&packet.network) {
            assert!(network.links().all(|link| {
                !link
                    .metadata()
                    .definition()
                    .is_some_and(|text| text.contains(declaration))
            }));
            let program = decode_typed_program_links(&network, packet.root).unwrap();
            assert_eq!(program.declaration(name).unwrap().kind(), "fn");
            assert_eq!(
                emit_typed_program_links(&network, packet.root, "Rust")
                    .unwrap()
                    .code,
                original_rust.code
            );
            assert_eq!(
                emit_typed_program_links(&network, packet.root, "JavaScript")
                    .unwrap()
                    .code,
                original_javascript.code
            );
        }
        observe(&original_rust.code, &expected, true);
        observe(&original_javascript.code, &expected, false);
        let reverse = create_typed_program_links(&original_javascript.code, "JavaScript").unwrap();
        observe(
            &emit_typed_program_links(&reverse.network, reverse.root, "Rust")
                .unwrap()
                .code,
            &expected,
            true,
        );
    }
}

#[test]
fn ordinary_full_cst_keeps_every_selected_declaration_byte_through_text_and_binary_links() {
    let fixture = fixture();
    for owner in fixture["originalDeclarations"].as_array().unwrap() {
        let source = format!(
            "// actual declaration: {}\n{}\n",
            owner["path"].as_str().unwrap(),
            owner["declaration"].as_str().unwrap()
        );
        let concrete = LinkNetwork::parse(&source, "Rust", ParseConfiguration::default());
        assert!(
            concrete
                .links()
                .any(|link| link.metadata().term() == Some("function_item"))
        );
        assert!(
            concrete
                .links()
                .any(|link| link.metadata().term() == Some("macro_invocation"))
        );
        assert!(
            concrete
                .links()
                .all(|link| !link.metadata().flags().has_error()
                    && !link.metadata().flags().is_missing())
        );
        assert_eq!(concrete.reconstruct_text().as_bytes(), source.as_bytes());
        for restored in archives(&concrete) {
            assert_eq!(restored.reconstruct_text().as_bytes(), source.as_bytes());
        }
    }
}

#[test]
fn unicode_and_all_delimiters_keep_one_receiver_evaluation_and_source_free_literal_edits() {
    let source = r##"fn provider(value:&str)->&str{value}
fn selected(value:&str)->bool{matches!(provider(value), | "π" | r#"😀"# | "|" | ")" | ",",)}
fn bracket(value:&str)->bool{matches![value,"π"|"😀"]}
fn brace(value:&str)->bool{matches!{value,"π"|"😀"}}
fn main(){println!("{}",selected("π"));println!("{}",selected("😀"));println!("{}",selected("|"));println!("{}",selected(")"));println!("{}",selected(","));println!("{}",selected("other"));println!("{}",bracket("π"));println!("{}",brace("😀"));}"##;
    let expected = "true\ntrue\ntrue\ntrue\ntrue\nfalse\ntrue\ntrue\n";
    let packet = create_typed_program_links(source, "Rust").unwrap();
    let javascript = emit_typed_program_links(&packet.network, packet.root, "JavaScript")
        .unwrap()
        .code;
    let instrumented = javascript.replace(
        "function provider(value) {",
        "function provider(value) { counter.calls += 1;",
    );
    assert_ne!(instrumented, javascript);
    observe(
        &format!(
            "const counter={{calls:0}};\n{instrumented}\nif(counter.calls!==6)throw new Error('one receiver evaluation per call');"
        ),
        expected,
        false,
    );
    observe(source, expected, true);
    let literal = packet
        .network
        .links()
        .find(|link| link.metadata().definition() == Some("π"))
        .unwrap()
        .id();
    let prefix = format!("({literal}: ");
    let changed = packet
        .network
        .to_lino()
        .lines()
        .map(|line| {
            if line.starts_with(&prefix) {
                line.replace("(def: %CF%80)", "(def: %CE%B4)")
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert_ne!(changed, packet.network.to_lino());
    let changed = LinkNetwork::from_lino(&changed).unwrap();
    for network in archives(&changed) {
        let code = emit_typed_program_links(&network, packet.root, "JavaScript")
            .unwrap()
            .code;
        observe(
            &format!("{code}\nconsole.log(selected(\"π\"));console.log(selected(\"δ\"));"),
            &format!("{expected}false\ntrue\n"),
            false,
        );
    }
}

#[test]
fn unsupported_patterns_and_source_owners_preserve_explicit_refusals_and_const_boundary() {
    for pattern in [
        "",
        "_",
        "binding",
        "\"a\" if true",
        "\"a\" | _",
        "\"a\"..=\"z\"",
        "(\"a\")",
        "Some(\"a\")",
        "\"a\",\"b\"",
        "\"a\" || \"b\"",
        "\"a\"|",
    ] {
        let source =
            format!("fn selected(value:&str)->bool{{matches!(value,{pattern})}}fn main(){{}}");
        let error = parse_rust(&source)
            .and_then(|surface| check_program(&surface))
            .unwrap_err();
        assert!(
            error.kind == ErrorKind::Unsupported || error.kind == ErrorKind::Syntax,
            "{source}: {error}"
        );
    }
    for source in [
        "fn selected(value:String)->bool{matches!(value,\"a\")}fn main(){}",
        "fn selected(value:bool)->bool{matches!(value,\"a\")}fn main(){}",
        "fn selected(value:&&str)->bool{matches!(value,\"a\")}fn main(){}",
        "enum str{Value} fn selected(value:&str)->bool{matches!(value,\"a\")}fn main(){}",
        "enum str{Value} mod nested{fn str()->bool{true}fn selected()->bool{matches!(\"a\",\"a\")}}fn main(){}",
        "fn selected(value:&str)->bool{other!(value,\"a\")}fn main(){}",
        "const fn selected(value:&str)->bool{matches!(value,\"a\")}fn main(){}",
    ] {
        assert!(
            create_typed_program_links(source, "Rust").is_err(),
            "{source}"
        );
    }
}

#[test]
fn imported_matches_macro_names_refuse_in_actual_module_scope_and_function_names_remain_separate() {
    for source in [
        "use crate::foreign::matches;fn selected(value:&str)->bool{matches!(value,\"a\")}fn main(){}",
        "fn selected(value:&str)->bool{matches!(value,\"a\")}use crate::foreign::matches;fn main(){}",
        "use crate::foreign::other as matches;fn selected(value:&str)->bool{matches!(value,\"a\")}fn main(){}",
        "mod nested{use crate::foreign::matches;fn selected(value:&str)->bool{matches!(value,\"a\")}}fn main(){}",
        "mod nested{fn selected(value:&str)->bool{matches!(value,\"a\")}use crate::foreign::other as matches;}fn main(){}",
    ] {
        let error = parse_rust(source)
            .and_then(|surface| check_program(&surface))
            .unwrap_err();
        assert_eq!(error.kind, ErrorKind::Unsupported, "{source}: {error}");
        assert!(
            error.to_string().contains("Rust literal matches!"),
            "{source}: {error}"
        );
    }
    let source = "fn matches(_value:&str)->bool{false}fn selected(value:&str)->bool{matches!(value,\"a\")}fn main(){println!(\"{}\",selected(\"a\"));println!(\"{}\",selected(\"b\"));}";
    let expected = "true\nfalse\n";
    observe(source, expected, true);
    let packet = create_typed_program_links(source, "Rust").unwrap();
    for network in archives(&packet.network) {
        for target in ["Rust", "JavaScript"] {
            let code = emit_typed_program_links(&network, packet.root, target)
                .unwrap()
                .code;
            observe(&code, expected, target == "Rust");
        }
    }
}
