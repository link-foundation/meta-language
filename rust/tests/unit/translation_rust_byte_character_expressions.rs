//! Byte expression and unsigned-negation parity; native execution belongs to CI.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::check::check_program;
use meta_language::translation::diagnostics::ErrorKind;
use meta_language::translation::ir::{Decl, LitValue, Node};
use meta_language::translation::rust::parse_rust;
use meta_language::translation::types::Type;
use meta_language::{
    BinaryLinksOptions, LinkNetwork, create_typed_program_links, decode_link_network_binary,
    decode_typed_program_links, emit_typed_program_links, encode_link_network_binary,
};
use std::fs;
use std::process::Command;

const BYTE_PROGRAM: &str = r#"fn zero()->u8{b'\0'}
fn ascii()->u8{b'0'}
fn high()->u8{b'\xFF'}
fn digit(value:u8)->u8{value-b'0'}
fn nine(value:u8)->bool{value==b'9'}
fn widened()->i16{-(b'0' as i16)}
fn main(){println!("{}",zero());println!("{}",ascii());println!("{}",high());println!("{}",digit(b'9'));println!("{}",nine(b'9'));println!("{}",widened());}"#;
const BYTE_OUTPUT: &str = "0\n48\n255\n9\ntrue\n-48\n";

fn execute_byte_source(source: &str, native: bool) {
    let directory = TemporaryDirectory::new("byte-character-expressions");
    let file = directory
        .0
        .join(if native { "program.rs" } else { "program.cjs" });
    fs::write(&file, source).unwrap();
    let observed = if native {
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
    assert!(observed.ok, "{}\n{}", observed.stdout, observed.stderr);
    assert_eq!(observed.stdout, BYTE_OUTPUT);
}

#[test]
fn complete_byte_program_and_both_saved_formats_preserve_fixed_values_and_execution() {
    let packet = create_typed_program_links(BYTE_PROGRAM, "Rust").unwrap();
    let native = emit_typed_program_links(&packet.network, packet.root, "Rust").unwrap();
    let javascript = emit_typed_program_links(&packet.network, packet.root, "JavaScript").unwrap();
    let archives = [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ];
    for network in &archives {
        let program = decode_typed_program_links(network, packet.root).unwrap();
        for (name, value) in [("zero", "0"), ("ascii", "48"), ("high", "255")] {
            let Some(Decl::Fn(function)) = program.declaration(name) else {
                panic!("an actual byte function exists")
            };
            assert_eq!(
                function.ret,
                Type::Fixed {
                    bits: 8,
                    signed: false
                }
            );
            assert_eq!(function.body.ty, function.ret);
            let Node::Lit {
                value: LitValue::Text(actual),
            } = &function.body.node
            else {
                panic!("the byte has an actual integer value")
            };
            assert_eq!(actual, value);
        }
        assert_eq!(
            emit_typed_program_links(network, packet.root, "Rust")
                .unwrap()
                .code,
            native.code
        );
        assert_eq!(
            emit_typed_program_links(network, packet.root, "JavaScript")
                .unwrap()
                .code,
            javascript.code
        );
    }
    // Execute complete, unchanged source artifacts once; both archives above
    // must emit exactly these tested bytes.
    execute_byte_source(BYTE_PROGRAM, true);
    execute_byte_source(&native.code, true);
    execute_byte_source(&javascript.code, false);
}

#[test]
fn byte_literals_keep_their_explicit_type_and_original_source_intervals() {
    for literal in ["b'}'", r"b'\xFF'"] {
        let original = format!("fn value()->u32{{{literal}}}");
        let source = format!("{original}fn main(){{}}");
        let surface = parse_rust(&source).unwrap();
        let failure = check_program(&surface).unwrap_err();
        assert_eq!(failure.kind, ErrorKind::Type);
        assert_eq!(failure.reason, "expected u32 but found u8");
        let tokens = meta_language::translation::lexer::tokenize(
            &source,
            meta_language::translation::Language::Rust,
        )
        .unwrap();
        let token = tokens
            .tokens
            .iter()
            .find(|token| token.raw == literal)
            .unwrap();
        let start = source[..source.find(literal).unwrap()]
            .encode_utf16()
            .count();
        assert_eq!(token.start, start);
        assert_eq!(token.end, start + literal.encode_utf16().count());
        assert_eq!(token.raw, literal);
        assert!(source.starts_with(&original));
    }
}

#[test]
fn unsigned_zero_and_cast_precedence_refuse_without_rejecting_signed_minima() {
    for (result, expression) in [
        ("u8", r"-b'\0'"),
        ("u8", r"-(b'\0')"),
        ("u8", "-0u8"),
        ("u8", "-0"),
        ("u8", "-(0)"),
        ("i16", "-b'0' as i16"),
        ("u8", "let value:u8=0u8;-value"),
    ] {
        let source = format!("fn value()->{result}{{{expression}}}fn main(){{}}");
        let failure = check_program(&parse_rust(&source).unwrap()).unwrap_err();
        assert_eq!(failure.kind, ErrorKind::Type, "{source}");
        assert_eq!(failure.reason, "negation of u8", "{source}");
    }
    let source = "fn minimum()->i8{-128i8}fn zero()->i32{-0}fn float_zero()->f64{-0.0}fn main(){}";
    let program = check_program(&parse_rust(source).unwrap()).unwrap();
    for (name, expected) in [("minimum", "-128"), ("zero", "0")] {
        let Some(Decl::Fn(function)) = program.declaration(name) else {
            panic!("a signed function exists")
        };
        let Node::Lit {
            value: LitValue::Text(actual),
        } = &function.body.node
        else {
            panic!("a signed integer exists")
        };
        assert_eq!(actual, expected);
    }
    let Some(Decl::Fn(function)) = program.declaration("float_zero") else {
        panic!("a floating function exists")
    };
    assert_eq!(
        function.body.node,
        Node::Lit {
            value: LitValue::Text("-0".to_owned())
        }
    );
    let failure =
        check_program(&parse_rust("fn value()->u8{!b'a'}fn main(){}").unwrap()).unwrap_err();
    assert_eq!(failure.kind, ErrorKind::Type);
}
