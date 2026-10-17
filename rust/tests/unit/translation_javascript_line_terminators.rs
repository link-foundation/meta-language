//! Actual JavaScript line boundaries and source-free native replay.

use std::process::Command;

use meta_language::translation::diagnostics::ErrorKind;
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::lexer::tokenize;
use meta_language::translation::{Language, Span};
use meta_language::{LinkNetwork, create_typed_program_links, emit_typed_program_links};

const TERMINATORS: [&str; 5] = ["\n", "\r", "\u{2028}", "\u{2029}", "\r\n"];

fn observe(source: &str) -> String {
    let result = Command::new("node")
        .args(["-e", source])
        .output()
        .expect("run the actual JavaScript source");
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    String::from_utf8(result.stdout).expect("the observation is UTF-8")
}

#[test]
fn every_line_comment_ending_preserves_actual_active_output_after_links_reload() {
    for separator in TERMINATORS {
        let source = format!("// hidden{separator}console.log('ACTIVE');\nconsole.log('END');");
        assert_eq!(observe(&source), "ACTIVE\nEND\n");
        let tokens = tokenize(&source, Language::JavaScript).unwrap();
        assert_eq!(tokens.comments.len(), 1);
        assert_eq!(tokens.comments[0].text, "// hidden");
        assert_eq!(tokens.comments[0].end, 9);
        assert_eq!(tokens.tokens[0].value, "console");
        assert_eq!(tokens.tokens[0].start, 9 + separator.encode_utf16().count());
        let packet = create_typed_program_links(&source, "JavaScript").unwrap();
        let reloaded = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
        let emitted = emit_typed_program_links(&reloaded, packet.root, "JavaScript").unwrap();
        assert_eq!(observe(&emitted.code), observe(&source));
    }
}

#[test]
fn return_and_jump_restrictions_retain_all_line_terminators_including_block_comments() {
    for separator in TERMINATORS {
        for gap in [
            separator.to_owned(),
            format!("/*{separator}*/"),
            format!("// comment{separator}"),
        ] {
            let source = format!(
                "/** @returns {{number}} */ function f(){{return{gap}1;}} console.log(f());"
            );
            assert_eq!(observe(&source), "undefined\n");
            let error =
                parse_javascript(&source).expect_err("the value is unreachable after return");
            assert_eq!(error.kind, ErrorKind::Unsupported);
            assert!(error.reason.starts_with("unreachable statement:"));
            let empty = format!("function f(){{return{gap};}} f();");
            assert!(parse_javascript(&empty).is_ok());
        }
        for jump in ["break", "continue"] {
            let source = format!(
                "function f(){{let n=0;while(n<1){{n++;{jump}/*{separator}*/missing();}}return n;}} console.log(f());"
            );
            assert_eq!(observe(&source), "1\n");
            let error =
                parse_javascript(&source).expect_err("a jump makes the following call unreachable");
            assert_eq!(error.kind, ErrorKind::Unsupported);
            assert!(error.reason.starts_with("unreachable statement:"));
        }
    }
}

#[test]
fn throw_line_breaks_fail_with_exact_diagnostics_and_nonterminating_spaces_stay_in_comments() {
    for separator in TERMINATORS {
        for gap in [
            separator.to_owned(),
            format!("/*{separator}*/"),
            format!("// comment{separator}"),
        ] {
            let source = format!("function f(){{throw{gap}new Error('no');}}");
            let error =
                parse_javascript(&source).expect_err("throw cannot precede a line terminator");
            assert_eq!(error.kind, ErrorKind::Syntax);
            assert_eq!(error.reason, "line terminator after throw");
            assert_eq!(error.span, Some(Span::new(13, 18)));
            let native = Command::new("node")
                .args([
                    "-e",
                    "new (require('node:vm').Script)(process.argv[1]);",
                    &source,
                ])
                .output()
                .unwrap();
            assert!(!native.status.success());
            assert!(String::from_utf8_lossy(&native.stderr).contains("SyntaxError"));
        }
    }
    for whitespace in [
        "\u{b}", "\u{c}", "\u{85}", "\u{a0}", "\u{2000}", "\u{feff}", "😀",
    ] {
        let source = format!("// hidden{whitespace}console.log('HIDDEN');\nconsole.log('END');");
        assert_eq!(observe(&source), "END\n");
        let tokens = tokenize(&source, Language::JavaScript).unwrap();
        assert_eq!(
            tokens
                .tokens
                .iter()
                .filter(|token| token.value == "console")
                .count(),
            1
        );
    }
}
