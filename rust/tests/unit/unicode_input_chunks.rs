use meta_language::{LinkNetwork, LinkType, ParseConfiguration};

type SourceBuilder = fn(usize) -> String;

// Mirrors js/tests/unicode-input-chunks.test.js: the JavaScript runtime reads input in
// 4096-byte chunks through a 5119-unit buffer, so both runtimes must keep astral characters
// on every alignment around both boundaries.
const ASTRAL_CASES: [(&str, SourceBuilder, &str); 4] = [
    (
        "JavaScript",
        |padding| format!("const {}𝓝 = 1;\n", "a".repeat(padding)),
        "𝓝",
    ),
    (
        "Rust",
        |padding| format!("//{}\nconst C: char = '𐲝';\n", " ".repeat(padding)),
        "'𐲝'",
    ),
    (
        "Lean",
        |padding| format!("def s : String := \"{}😀\"\n", "a".repeat(padding)),
        "😀",
    ),
    (
        "Rocq",
        |padding| format!("Definition s := \"{}😀\".\n", "a".repeat(padding)),
        "😀",
    ),
];

#[test]
fn astral_characters_straddling_a_parser_input_chunk_boundary_stay_whole() {
    for (language, build, expected) in ASTRAL_CASES {
        for padding in (4070..=4100).chain(5095..=5125) {
            let source = build(padding);
            let network = LinkNetwork::parse(&source, language, ParseConfiguration::default());
            let label = format!("{language} with {padding} padding code units");
            assert_eq!(network.reconstruct_text(), source, "{label}");
            assert!(network.verify_full_match(None).is_clean(), "{label}");
            let texts = network
                .links()
                .filter(|link| link.metadata().link_type() == Some(LinkType::Syntax))
                .filter_map(|link| link.metadata().span())
                .map(|span| &source[span.byte_range().start()..span.byte_range().end()])
                .collect::<Vec<_>>();
            assert!(texts.iter().any(|text| text.contains(expected)), "{label}");
        }
    }
}
