//! CI-only original/translated execution, exact sources and both Links codecs.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::check::check_program;
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::rust::parse_rust;
use meta_language::{
    BinaryLinksOptions, LinkNetwork, ParseConfiguration, create_typed_program_links,
    decode_link_network_binary, decode_typed_program_links, emit_typed_program_links,
    encode_link_network_binary, encode_typed_program_links, read_structured_data_lino,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::fs;
use std::process::Command;
use std::sync::OnceLock;

#[path = "../../src/translation/ascii_text_predicate_rules.rs"]
mod generated_decisions;

fn fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../src/data/ascii-text-predicate-rules.lino"
    ))
    .unwrap()
}

fn assert_versions() {
    static CHECKED: OnceLock<()> = OnceLock::new();
    CHECKED.get_or_init(|| {
        let policy = fixture();
        assert_eq!(policy["assertionVersions"]["Rust"], ">=1.90.0 <1.100.0");
        assert_eq!(
            policy["assertionVersions"]["Node"],
            ">=22.15.0 <23.0.0 || >=24.0.0 <25.0.0"
        );
        let rust = run(Command::new("rustc").arg("--version"));
        assert!(rust.ok, "{}", rust.stderr);
        let version = rust.stdout.split_whitespace().nth(1).unwrap();
        let mut parts = version.split('.');
        let major = parts.next().unwrap().parse::<u32>().unwrap();
        let minor = parts.next().unwrap().parse::<u32>().unwrap();
        assert!(
            major == 1 && (90..100).contains(&minor),
            "uncovered Rust version {version}; supported >=1.90.0 <1.100.0"
        );
        let node = run(Command::new("node").arg("--version"));
        assert!(node.ok, "{}", node.stderr);
        let version = node.stdout.trim().trim_start_matches('v');
        let mut parts = version.split('.');
        let major = parts.next().unwrap().parse::<u32>().unwrap();
        let minor = parts.next().unwrap().parse::<u32>().unwrap();
        assert!(
            (major == 22 && minor >= 15) || major == 24,
            "uncovered Node version {version}; supported >=22.15.0 <23.0.0 || >=24.0.0 <25.0.0"
        );
    });
}

fn digest(source: &str) -> String {
    Sha256::digest(source.as_bytes())
        .iter()
        .fold(String::new(), |mut output, byte| {
            write!(output, "{byte:02x}").expect("a hexadecimal byte writes to String");
            output
        })
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

fn observe(source: &str, expected: &str, native: bool) {
    assert_versions();
    let directory = TemporaryDirectory::new("ascii-text-predicates");
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

fn inputs() -> Vec<String> {
    let mut values = [
        "", "_", "a", "Z", "a0_", "0a", "-a", "a-b", "a\n", "a\r\n", "π", "aπ", "é", "K", "İ",
        "a\u{301}", "😀", "a😀", "\0", "\u{7f}",
    ]
    .map(str::to_owned)
    .to_vec();
    for scalar in 0..256 {
        let character = char::from_u32(scalar).unwrap();
        values.push(character.to_string());
        values.push(format!("A{character}"));
    }
    values
}

fn expected(value: &str, terminal: bool) -> bool {
    if terminal {
        return value
            .chars()
            .next()
            .is_some_and(|character| (65..=90).contains(&u32::from(character)));
    }
    !value.is_empty()
        && value.chars().enumerate().all(|(index, character)| {
            let scalar = u32::from(character);
            (65..=90).contains(&scalar)
                || (97..=122).contains(&scalar)
                || scalar == 95
                || (index > 0 && (48..=57).contains(&scalar))
        })
}

#[test]
fn actual_generated_ascii_methods_match_complete_independent_byte_ranges_and_unicode_boundaries() {
    assert_versions();
    use generated_decisions::{accept_ascii_character_method, ml_string::Text};
    let methods = [
        "is_ascii_alphabetic",
        "is_ascii_alphanumeric",
        "is_ascii_uppercase",
        "is_ascii_lowercase",
        "is_ascii_digit",
    ];
    for scalar in 0..256 {
        let upper = (65..=90).contains(&scalar);
        let lower = (97..=122).contains(&scalar);
        let digit = (48..=57).contains(&scalar);
        let values = [upper || lower, upper || lower || digit, upper, lower, digit];
        for (method, expected) in methods.iter().zip(values) {
            assert_eq!(
                accept_ascii_character_method(&Text::from_text(method), f64::from(scalar)),
                expected
            );
        }
    }
    for scalar in [
        -1.0,
        0.5,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        256.0,
        960.0,
        383.0,
        8490.0,
        55296.0,
        56320.0,
        128512.0,
        1114111.0,
    ] {
        for method in methods {
            assert!(!accept_ascii_character_method(
                &Text::from_text(method),
                scalar
            ));
        }
    }
    for method in [
        "is_alphabetic",
        "is_ascii_whitespace",
        "",
        "is_ascii_digit()",
    ] {
        assert!(!accept_ascii_character_method(
            &Text::from_text(method),
            65.0
        ));
    }
}

#[test]
fn both_actual_whole_helpers_preserve_sources_cst_native_execution_and_bidirectional_semantic_archives()
 {
    assert_versions();
    let fixture = fixture();
    let owners = fixture["originalDeclarations"].as_array().unwrap();
    assert_eq!(owners.len(), 2);
    for owner in owners {
        let path = owner["path"].as_str().unwrap();
        let context = match path {
            "rust/src/grammar/emit/tree_sitter_json.rs" => {
                include_str!("../../src/grammar/emit/tree_sitter_json.rs")
            }
            "rust/src/grammar/emit/lark.rs" => include_str!("../../src/grammar/emit/lark.rs"),
            _ => panic!("exactly the two original source module owners are retained"),
        };
        assert_eq!(context, owner["context"].as_str().unwrap());
        assert_eq!(
            context.len(),
            usize::try_from(owner["bytes"].as_u64().unwrap()).unwrap()
        );
        assert_eq!(digest(context), owner["sourceSha256"].as_str().unwrap());
        let declaration = owner["declaration"].as_str().unwrap();
        let name = owner["name"].as_str().unwrap();
        assert!(declaration.starts_with(&format!("fn {name}(")));
        assert!(context.contains(declaration));
        assert_eq!(
            digest(declaration),
            owner["declarationSha256"].as_str().unwrap()
        );
        let concrete_source = format!("// actual owner π: {path}\n{declaration}\n");
        let concrete = LinkNetwork::parse(&concrete_source, "Rust", ParseConfiguration::default());
        assert!(
            concrete
                .links()
                .any(|link| link.metadata().term() == Some("function_item"))
        );
        assert!(
            concrete
                .links()
                .any(|link| link.metadata().term() == Some("closure_expression"))
        );
        assert!(
            concrete
                .links()
                .all(|link| !link.metadata().flags().has_error()
                    && !link.metadata().flags().is_missing())
        );
        assert_eq!(
            concrete.reconstruct_text().as_bytes(),
            concrete_source.as_bytes()
        );
        for restored in archives(&concrete) {
            assert_eq!(
                restored.reconstruct_text().as_bytes(),
                concrete_source.as_bytes()
            );
        }
        let mut source = format!("{declaration}\nfn main(){{");
        let mut output = String::new();
        for value in inputs() {
            write!(source, "println!(\"{{}}\",{name}({value:?}));").unwrap();
            writeln!(output, "{}", expected(&value, name == "is_terminal_name")).unwrap();
        }
        source.push('}');
        observe(&source, &output, true);
        let packet = create_typed_program_links(&source, "Rust").unwrap();
        let original_rust = emit_typed_program_links(&packet.network, packet.root, "Rust")
            .unwrap()
            .code;
        let original_javascript =
            emit_typed_program_links(&packet.network, packet.root, "JavaScript")
                .unwrap()
                .code;
        for network in archives(&packet.network) {
            let program = decode_typed_program_links(&network, packet.root).unwrap();
            assert_eq!(program.declaration(name).unwrap().kind(), "fn");
            assert!(network.links().all(|link| {
                !link
                    .metadata()
                    .definition()
                    .is_some_and(|text| text.contains(declaration))
            }));
            assert_eq!(
                emit_typed_program_links(&network, packet.root, "Rust")
                    .unwrap()
                    .code,
                original_rust
            );
            assert_eq!(
                emit_typed_program_links(&network, packet.root, "JavaScript")
                    .unwrap()
                    .code,
                original_javascript
            );
        }
        observe(&original_rust, &output, true);
        observe(&original_javascript, &output, false);
        let reverse = create_typed_program_links(&original_javascript, "JavaScript").unwrap();
        for network in archives(&reverse.network) {
            observe(
                &emit_typed_program_links(&network, reverse.root, "Rust")
                    .unwrap()
                    .code,
                &output,
                true,
            );
            observe(
                &emit_typed_program_links(&network, reverse.root, "JavaScript")
                    .unwrap()
                    .code,
                &output,
                false,
            );
        }
    }
}

#[test]
fn pure_queries_keep_one_receiver_evaluation_hygiene_vacuity_and_actual_unicode_behavior() {
    assert_versions();
    let mut source = "fn provider(value:&str)->&str{value}
fn first(value:&str)->bool{provider(value).starts_with(|c|c.is_ascii_digit())}
fn all(value:&str)->bool{provider(value).chars().all(|c|c.is_ascii_digit())}
fn selected(ascii_predicate_points0:&str,ascii_predicate_point0:bool)->bool{ascii_predicate_points0.chars().all(|c|c.is_ascii_digit()) && ascii_predicate_point0}
fn main(){".to_owned();
    for value in ["", "1", "12", "1x", "x1", "π", "1π", "😀", "1😀"] {
        write!(
            source,
            "println!(\"{{}}\",first({value:?}));println!(\"{{}}\",all({value:?}));"
        )
        .unwrap();
    }
    source.push_str(
        "println!(\"{}\",selected(\"00\",true));println!(\"{}\",selected(\"00\",false));}",
    );
    let expected = "false\ntrue\ntrue\ntrue\ntrue\ntrue\ntrue\nfalse\nfalse\nfalse\nfalse\nfalse\ntrue\nfalse\nfalse\nfalse\ntrue\nfalse\ntrue\nfalse\n";
    observe(&source, expected, true);
    let packet = create_typed_program_links(&source, "Rust").unwrap();
    for network in archives(&packet.network) {
        let javascript = emit_typed_program_links(&network, packet.root, "JavaScript")
            .unwrap()
            .code;
        let instrumented = javascript.replace(
            "function provider(value) {",
            "function provider(value) { counter.calls += 1;",
        );
        assert_ne!(javascript, instrumented);
        observe(
            &format!(
                "const counter={{calls:0}};\n{instrumented}\nif(counter.calls!==18)throw new Error('one receiver evaluation per call');"
            ),
            expected,
            false,
        );
        observe(
            &emit_typed_program_links(&network, packet.root, "Rust")
                .unwrap()
                .code,
            expected,
            true,
        );
    }
}

#[test]
fn actual_wrong_owners_aliases_annotations_effects_and_existing_const_boundaries_refuse() {
    assert_versions();
    for callback in [
        "other.is_ascii_digit()",
        "side_effect()",
        "{c.is_ascii_digit()}",
        "c.is_ascii_digit(1)",
        "c.is_alphabetic()",
        "c=='π'",
        "c==b'_'",
        "c.is_ascii_digit()&&true",
        "c.is_ascii_digit()||",
        "c.is_ascii_digit(),false",
    ] {
        let source = format!(
            "fn selected(value:&str)->bool{{value.starts_with(|c|{callback})}}fn main(){{}}"
        );
        assert!(
            parse_rust(&source)
                .and_then(|surface| check_program(&surface))
                .is_err(),
            "{source}"
        );
    }
    for source in [
        "fn selected(value:String)->bool{value.starts_with(|c|c.is_ascii_digit())}fn main(){}",
        "fn selected(value:&&str)->bool{value.starts_with(|c|c.is_ascii_digit())}fn main(){}",
        "fn selected(value:bool)->bool{value.starts_with(|c|c.is_ascii_digit())}fn main(){}",
        "enum str{Value}fn selected(value:&str)->bool{value.starts_with(|c|c.is_ascii_digit())}fn main(){}",
        "use crate::foreign::str;fn selected(value:&str)->bool{value.starts_with(|c|c.is_ascii_digit())}fn main(){}",
        "fn selected(value:&str)->bool{value.starts_with(|c|c.is_ascii_digit())}use crate::foreign::other as str;fn main(){}",
        "mod nested{use crate::foreign::str;fn selected(value:&str)->bool{value.starts_with(|c|c.is_ascii_digit())}}fn main(){}",
        "enum char{Value}fn selected(value:&str)->bool{value.starts_with(|c:char|c.is_ascii_digit())}fn main(){}",
        "use crate::foreign::char;fn selected(value:&str)->bool{value.starts_with(|c:char|c.is_ascii_digit())}fn main(){}",
        "fn selected(value:&str)->bool{value.starts_with(|c:char|c.is_ascii_digit())}use crate::foreign::other as char;fn main(){}",
        "mod nested{use crate::foreign::other as char;fn selected(value:&str)->bool{value.chars().all(|c:char|c.is_ascii_digit())}}fn main(){}",
        "fn selected(value:&str)->bool{value.starts_with(|mut c|c.is_ascii_digit())}fn main(){}",
        "fn selected(value:&str)->bool{value.chars().all(|c,d|c.is_ascii_digit())}fn main(){}",
        "fn selected(value:&str)->bool{value.chars(1).all(|c|c.is_ascii_digit())}fn main(){}",
        "fn selected(value:&str)->bool{value.bytes().all(|c|c.is_ascii_digit())}fn main(){}",
        "const fn selected(value:&str)->bool{value.starts_with(|c|c.is_ascii_digit())}fn main(){}",
    ] {
        assert!(
            create_typed_program_links(source, "Rust").is_err(),
            "{source}"
        );
    }
    for prefix in [
        "enum char{Value}",
        "fn char(value:bool)->bool{value}",
        "fn str(value:bool)->bool{value}",
    ] {
        let source = format!(
            "{prefix}fn selected(value:&str)->bool{{value.starts_with(|c|c.is_ascii_digit())}}fn main(){{println!(\"{{}}\",selected(\"0\"));println!(\"{{}}\",selected(\"π\"));}}"
        );
        observe(&source, "true\nfalse\n", true);
        let packet = create_typed_program_links(&source, "Rust").unwrap();
        observe(
            &emit_typed_program_links(&packet.network, packet.root, "JavaScript")
                .unwrap()
                .code,
            "true\nfalse\n",
            false,
        );
    }
}

#[test]
fn actual_javascript_string_spreads_retain_code_points_and_lone_surrogates_with_explicit_utf16_native_representation()
 {
    assert_versions();
    let source = "/** @param {string} value @returns {string[]} */function points(value){return [...value];}";
    let javascript_values = r#"["","A","Aπ","😀","A😀Z","\ud800","\udc00","\ud800x\udc00"]"#;
    let output = "[[],[\"A\"],[\"A\",\"π\"],[\"😀\"],[\"A\",\"😀\",\"Z\"],[\"\\ud800\"],[\"\\udc00\"],[\"\\ud800\",\"x\",\"\\udc00\"]]\n";
    let observations =
        format!("\nconsole.log(JSON.stringify({javascript_values}.map(value=>points(value))));");
    observe(&format!("{source}{observations}"), output, false);
    // Preserve the original default archive and every JavaScript observation.
    let ordinary = create_typed_program_links(source, "JavaScript").unwrap();
    for network in archives(&ordinary.network) {
        let code = emit_typed_program_links(&network, ordinary.root, "JavaScript")
            .unwrap()
            .code;
        observe(&format!("{code}{observations}"), output, false);
    }
    // The existing explicit UTF-16 representation is a checked program facet,
    // matching JavaScript createTypedProgramLinks(...,{utf16Strings:true}). A
    // Rust UTF-8 String cannot represent the independent lone-surrogate inputs.
    let mut program = check_program(&parse_javascript(source).unwrap()).unwrap();
    program.utf16_strings = Some(true);
    let packet = encode_typed_program_links(&program).unwrap();
    let cases = [
        (vec![], vec![]),
        (vec![65], vec![vec![65]]),
        (vec![65, 960], vec![vec![65], vec![960]]),
        (vec![55357, 56832], vec![vec![55357, 56832]]),
        (
            vec![65, 55357, 56832, 90],
            vec![vec![65], vec![55357, 56832], vec![90]],
        ),
        (vec![55296], vec![vec![55296]]),
        (vec![56320], vec![vec![56320]]),
        (
            vec![55296, 120, 56320],
            vec![vec![55296], vec![120], vec![56320]],
        ),
    ];
    let mut harness = "\nfn ml_ascii_point_observations(){".to_owned();
    for (units, expected) in cases {
        write!(harness, "let actual=points(crate::ml_string::Text::from_units(vec!{units:?})).into_iter().map(|point|point.0).collect::<Vec<_>>();let expected:Vec<Vec<u16>>=vec!{expected:?};assert_eq!(actual,expected);").unwrap();
    }
    harness.push_str("println!(\"true\");}\n");
    for network in archives(&packet.network) {
        let restored = decode_typed_program_links(&network, packet.root).unwrap();
        assert_eq!(restored.utf16_strings, Some(true));
        let javascript = emit_typed_program_links(&network, packet.root, "JavaScript")
            .unwrap()
            .code;
        observe(&format!("{javascript}{observations}"), output, false);
        let rust = emit_typed_program_links(&network, packet.root, "Rust")
            .unwrap()
            .code;
        assert_eq!(rust.matches("fn main() {").count(), 1);
        let instrumented =
            rust.replace("fn main() {", "fn main() { ml_ascii_point_observations();");
        assert_ne!(instrumented, rust);
        observe(&format!("{instrumented}{harness}"), "true\n", true);
    }
    let wrong = "/** @param {number} value */function points(value){return [...value];}";
    assert!(create_typed_program_links(wrong, "JavaScript").is_err());
}
