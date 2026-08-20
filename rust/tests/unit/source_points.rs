//! Point conventions at the parser boundary.
//!
//! Parsers report a `(row, column)` point for every span they emit. Resolving
//! those points through a shared line index instead of rescanning the source
//! (issue #193) must not move a single point, so these tests pin both
//! conventions the parsers use: columns counted in bytes over program source,
//! and columns counted in characters over prose and region boundaries.

use meta_language::{LinkNetwork, LinkType, ParseConfiguration, Point};

/// Row and column with the column counted in bytes, scanned directly from the
/// text so the expectation stays independent of the indexed lookup under test.
fn byte_point(text: &str, byte: usize) -> Point {
    let mut row = 0;
    let mut line_start = 0;
    for (index, value) in text.bytes().enumerate().take(byte) {
        if value == b'\n' {
            row += 1;
            line_start = index + 1;
        }
    }
    Point::new(row, byte - line_start)
}

/// Sources whose points are unambiguous: with only ASCII bytes a byte column
/// and a character column are the same number, so every parser in the pipeline
/// must land on exactly this value.
const ASCII_SOURCES: &[(&str, &str)] = &[
    ("Rust", "fn main() {\n    let value = 1;\n}\n"),
    ("Rust", "fn main() {\r\n    let value = 1;\r\n}"),
    ("Rust", "fn a() {}\n\n\n\nfn b() {}\n"),
    ("Python", "def item(value):\n    return value\n"),
    ("JavaScript", "const unit = 1;\nconst other = unit + 2;\n"),
    ("JSON", "{\n  \"key\": [1, 2, 3],\n  \"other\": null\n}\n"),
    ("CSV", "name,value\n\"quoted, cell\",2\n"),
    ("Lino", "(item: source target)\n(other: a b)\n"),
    (
        "Markdown",
        "# Title\n\nText here.\n\n```rust\nfn item() {}\n```\n",
    ),
    (
        "HTML",
        "<p style=\"color: red\">Text</p>\n<style>\n.a { color: blue }\n</style>\n",
    ),
    (
        "English",
        "Hawaii is a state.\nA second sentence follows.\n",
    ),
];

#[test]
fn ascii_source_points_match_a_direct_scan_of_the_text() {
    for (language, source) in ASCII_SOURCES {
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
        for link in network.links() {
            let Some(span) = link.metadata().span() else {
                continue;
            };
            let range = span.byte_range();
            assert_eq!(
                span.start_point(),
                byte_point(source, range.start()),
                "{language} start point of {:?} at byte {}",
                link.metadata().term(),
                range.start(),
            );
            assert_eq!(
                span.end_point(),
                byte_point(source, range.end()),
                "{language} end point of {:?} at byte {}",
                link.metadata().term(),
                range.end(),
            );
        }
    }
}

/// Span of the token whose text is `term`, as `(start, end)` points.
fn token_points(network: &LinkNetwork, term: &str) -> (Point, Point) {
    let link = network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Token))
        .find(|link| link.metadata().term() == Some(term))
        .unwrap_or_else(|| panic!("token {term:?} is present"));
    let span = link.metadata().span().expect("token carries a span");
    (span.start_point(), span.end_point())
}

/// Span of the region link for `language`, as `(start, end)` points.
fn region_points(network: &LinkNetwork, language: &str) -> (Point, Point) {
    let link = network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Region))
        .find(|link| link.metadata().language() == Some(language))
        .unwrap_or_else(|| panic!("{language} region is present"));
    let span = link.metadata().span().expect("region carries a span");
    (span.start_point(), span.end_point())
}

/// Program source keeps byte columns, so a two-byte character advances the
/// column by two.
#[test]
fn program_source_points_count_columns_in_bytes() {
    let source = "fn main() {\n    let s = \"héllo\";\n}\n";
    let network = LinkNetwork::parse(source, "Rust", ParseConfiguration::default());

    // `;` sits at byte 32, on a line that starts at byte 12 and holds one
    // two-byte character before it, so its byte column is 20 where a character
    // column would be 19.
    assert_eq!(source.as_bytes()[32], b';');
    assert_eq!(
        token_points(&network, ";"),
        (Point::new(1, 20), Point::new(1, 21))
    );
    assert_eq!(
        token_points(&network, "}"),
        (Point::new(2, 0), Point::new(2, 1))
    );
}

/// Prose keeps character columns, so a two-byte character advances the column
/// by one.
#[test]
fn natural_language_points_count_columns_in_characters() {
    let source = "El niño lee.\nLa señora escribe.\n";
    let network = LinkNetwork::parse(source, "Spanish", ParseConfiguration::default());

    assert_eq!(
        token_points(&network, "lee"),
        (Point::new(0, 8), Point::new(0, 11))
    );
    assert_eq!(
        token_points(&network, "escribe"),
        (Point::new(1, 10), Point::new(1, 17))
    );
}

/// A region boundary keeps character columns, while the parse of the embedded
/// source continues to report byte columns relative to that boundary.
#[test]
fn embedded_region_points_keep_the_boundary_and_inner_conventions() {
    let source = "# Título\n\nTexto aquí.\n\n```rust\nfn ítem() {}\n```\n";
    let network = LinkNetwork::parse(source, "Markdown", ParseConfiguration::default());

    assert_eq!(
        region_points(&network, "rust"),
        (Point::new(5, 0), Point::new(6, 0))
    );
    // `ítem` starts one column into the embedded line and spans five bytes.
    assert_eq!(
        token_points(&network, "ítem"),
        (Point::new(5, 3), Point::new(5, 8))
    );
}
