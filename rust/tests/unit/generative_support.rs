//! Reproducible generation for the issue 195 generative suite: the seeded
//! PRNG, the source edits that keep UTF-8 boundaries, the metamorphic
//! relations and the oracle-free properties every parse must keep (round
//! trip, Unicode spans, diagnostics). Mirrors `js/tests/support/generative.js`
//! draw for draw, so a seed names the same inputs in both suites.

use meta_language::{Link, LinkNetwork, LinkType};

use super::cst_lines::{parse_cst_lines, row_offsets, CstNode};

/// FNV-1a (32-bit) of a UTF-8 string: the numeric seed of a textual seed.
pub fn seed_number(text: &str) -> u32 {
    text.bytes().fold(0x811c_9dc5_u32, |hash, byte| {
        (hash ^ u32::from(byte)).wrapping_mul(0x0100_0193)
    })
}

/// mulberry32: a small 32-bit PRNG, identical in both suites.
pub struct Random {
    state: u32,
}

impl Random {
    pub fn new(seed: &str) -> Self {
        Self::from_number(seed_number(seed))
    }

    pub const fn from_number(state: u32) -> Self {
        Self { state }
    }

    pub const fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_add(0x6d2b_79f5);
        let mut value = self.state;
        value = (value ^ (value >> 15)).wrapping_mul(value | 1);
        value ^= value.wrapping_add((value ^ (value >> 7)).wrapping_mul(value | 0x3d));
        value ^ (value >> 14)
    }

    pub fn int(&mut self, bound: usize) -> usize {
        if bound == 0 {
            return 0;
        }
        let bound = u32::try_from(bound).expect("bound fits in u32");
        usize::try_from(self.next_u32() % bound).expect("u32 fits in usize")
    }

    pub fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.int(items.len())]
    }
}

/// Inserted text that stresses spans: astral, combining, joined, invisible and line-ending characters.
pub const UNICODE_POOL: [&str; 15] = [
    "𝒳",
    "👩\u{200D}👩\u{200D}👧",
    "e\u{0301}",
    "日本",
    "🦀",
    "λ",
    "\u{00A0}",
    "\u{200B}",
    "\u{2060}",
    "\u{FEFF}",
    "\r\n",
    "\r",
    "\t",
    "\n",
    " ",
];

/// Tokens of each language that open, close or split constructs.
pub fn token_pool(language: &str) -> &'static [&'static str] {
    match language {
        "JavaScript" => &[
            "(", ")", "{", "}", "[", "]", ";", ",", "=>", "\"", "`", "${", "/*", "//", "function",
            "let", "=", ".", "class", "<div>",
        ],
        "Lean" => &[
            "(", ")", "⟨", "⟩", ":=", ":", "def", "theorem", "by", "fun", "=>", "|", "--", "/-",
            "-/", "do", "←", "\"", ".", "where",
        ],
        "Rocq" => &[
            "(",
            ")",
            ".",
            ":=",
            ":",
            "Definition",
            "Lemma",
            "Proof.",
            "Qed.",
            "(*",
            "*)",
            "fun",
            "=>",
            "|",
            "match",
            "end",
            "\"",
            "%",
            ";",
            "forall",
        ],
        "Rust" => &[
            "(", ")", "{", "}", "[", "]", ";", ",", "::", "->", "=>", "\"", "'a", "fn", "let",
            "#[", "/*", "//", "r#\"", "<", ">",
        ],
        other => panic!("no token pool for {other}"),
    }
}

/// A source edit in UTF-8 byte offsets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edit {
    pub start: usize,
    pub end: usize,
    pub replacement: String,
}

/// One random edit of `source` on code point boundaries.
pub fn random_edit(random: &mut Random, source: &str, language: &str) -> Edit {
    let points: Vec<char> = source.chars().collect();
    let pool: &[&str] = if random.int(3) == 0 {
        &UNICODE_POOL
    } else {
        token_pool(language)
    };
    let at = random.int(points.len() + 1);
    let length = (points.len() - at).min(1 + random.int(8));
    let (from, to, replacement) = match random.int(5) {
        0 => (at, at + length, String::new()),
        1 => (at, at, points[at..at + length].iter().collect()),
        2 => (at, at + length, (*random.pick(pool)).to_string()),
        3 => (at, points.len(), String::new()),
        _ => (at, at, (*random.pick(pool)).to_string()),
    };
    let byte_offset = |index: usize| points[..index].iter().map(|point| point.len_utf8()).sum();
    Edit {
        start: byte_offset(from),
        end: byte_offset(to),
        replacement,
    }
}

/// Applies a byte-offset edit to a string.
pub fn apply_text_edit(source: &str, edit: &Edit) -> String {
    format!(
        "{}{}{}",
        &source[..edit.start],
        edit.replacement,
        &source[edit.end..]
    )
}

/// The metamorphic relations, by name.
pub const RELATIONS: [&str; 2] = ["prepend-blank-lines", "crlf"];

/// The variant of `source` a relation compares with it.
pub fn relation_transform(relation: &str, source: &str) -> String {
    match relation {
        "prepend-blank-lines" => format!("\n\n{source}"),
        "crlf" => source.replace("\r\n", "\n").replace('\n', "\r\n"),
        other => panic!("unknown relation {other}"),
    }
}

/// Whether a relation must hold: two leading blank lines can change error
/// recovery, and the lexer skips U+FEFF only at byte 0.
pub fn relation_applies(relation: &str, source: &str, clean: bool) -> bool {
    relation != "prepend-blank-lines" || (clean && !source.starts_with('\u{FEFF}'))
}

/// Whether the variant tree is what the relation predicts from the base tree:
/// every node two rows down, or (CRLF) the same nodes with the same starts.
pub fn relation_holds(relation: &str, base: &str, variant: &str) -> bool {
    let base = parse_cst_lines(base);
    let variant = parse_cst_lines(variant);
    match relation {
        "prepend-blank-lines" => {
            let predicted: Vec<CstNode> = base
                .into_iter()
                .map(|node| CstNode {
                    start: (node.start.0 + 2, node.start.1),
                    end: (node.end.0 + 2, node.end.1),
                    ..node
                })
                .collect();
            predicted == variant
        }
        "crlf" => {
            let without_end = |nodes: Vec<CstNode>| -> Vec<CstNode> {
                nodes
                    .into_iter()
                    .map(|node| CstNode {
                        end: (0, 0),
                        ..node
                    })
                    .collect()
            };
            without_end(base) == without_end(variant)
        }
        other => panic!("unknown relation {other}"),
    }
}

/// Oracle-free problems of a parsed network: exact reconstruction; every span
/// inside the source, on code point boundaries, with points that name its byte
/// offsets; every Syntax child inside its parent; and a verification report
/// that is clean exactly when no link carries an error or missing flag.
pub fn property_problems(network: &LinkNetwork, source: &str) -> Vec<String> {
    let mut problems = Vec::new();
    if network.reconstruct_text() != source {
        problems.push("reconstruction differs from the source".to_string());
    }
    let rows = row_offsets(source);
    let point_of = |offset: usize| {
        let row = rows.partition_point(|start| *start <= offset) - 1;
        (row, offset - rows[row])
    };
    let mut flagged = false;
    for link in network.links() {
        let metadata = link.metadata();
        let flags = metadata.flags();
        flagged |= flags.is_error() || flags.is_missing();
        let Some(span) = metadata.span() else {
            continue;
        };
        let (start, end) = (span.byte_range().start(), span.byte_range().end());
        let link_type = metadata.link_type();
        let term = metadata.term().unwrap_or_default();
        let place = format!("{link_type:?} {term} {start}..{end}");
        if !(start <= end && end <= source.len()) {
            problems.push(format!("{place} is outside the source"));
            continue;
        }
        if !source.is_char_boundary(start) || !source.is_char_boundary(end) {
            problems.push(format!("{place} splits a UTF-8 code point"));
        }
        let points = (
            (span.start_point().row(), span.start_point().column()),
            (span.end_point().row(), span.end_point().column()),
        );
        if points != (point_of(start), point_of(end)) {
            problems.push(format!("{place} has points {points:?}"));
        }
        if link_type == Some(LinkType::Syntax) {
            for parent in link.references() {
                let Some(parent) = network.link(*parent).map(Link::metadata) else {
                    continue;
                };
                if parent.link_type() != Some(LinkType::Syntax) {
                    continue;
                }
                if let Some(outer) = parent.span() {
                    if start < outer.byte_range().start() || end > outer.byte_range().end() {
                        problems.push(format!(
                            "{place} lies outside its parent {}",
                            parent.term().unwrap_or_default()
                        ));
                    }
                }
            }
        }
        if problems.len() > 8 {
            break;
        }
    }
    if network.verify_full_match(None).is_clean() == flagged {
        problems.push(format!(
            "verify_full_match(None).is_clean() is {} while {} link is flagged",
            !flagged,
            if flagged { "a" } else { "no" }
        ));
    }
    problems
}

/// Cases per language and family, as `COUNTS` of the JavaScript support.
pub const PROPERTY_CASES: usize = 16;
pub const FUZZ_CASES: usize = 32;
pub const METAMORPHIC_CASES: usize = 8;
pub const EDIT_CASES: usize = 8;
pub const EDIT_STEPS: usize = 4;
const MAX_SEED_BYTES: usize = 600;

/// One seed input: the source of a conformance case.
pub struct SeedSource {
    pub source: String,
}

fn conformance_text(path: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../parity/fixtures/issue-195-conformance")
        .join(path);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("{} is readable: {error}", path.display()))
}

/// The seed sources of one language: its small conformance inputs, in the
/// order of the JavaScript support.
pub fn seed_sources(language: &str) -> Vec<SeedSource> {
    let manifest: serde_json::Value =
        serde_json::from_str(&conformance_text("manifest.json")).expect("conformance manifest");
    let details = &manifest["languages"][language];
    let oracle: serde_json::Value = serde_json::from_str(&conformance_text(
        details["oracle"].as_str().expect("oracle"),
    ))
    .expect("conformance oracle");
    let directory = details["corpus"]["directory"].as_str().expect("directory");
    let mut sources = Vec::new();
    for file in details["corpus"]["files"]
        .as_object()
        .expect("files")
        .keys()
    {
        let text = conformance_text(&format!("{directory}/{file}"));
        sources.extend(super::cst_sexpression::parse_corpus(&text).into_iter().map(
            |corpus_case| SeedSource {
                source: corpus_case.source,
            },
        ));
    }
    for entry in oracle["cases"].as_array().expect("cases") {
        if let Some(source) = entry["source"].as_str() {
            sources.push(SeedSource {
                source: source.to_string(),
            });
        }
    }
    // JavaScript `trim()` also removes U+FEFF.
    sources.retain(|seed| {
        !seed
            .source
            .trim_matches(|character: char| character.is_whitespace() || character == '\u{FEFF}')
            .is_empty()
            && seed.source.len() <= MAX_SEED_BYTES
    });
    sources
}

/// One generated input before the oracle parses it.
pub struct Input {
    pub id: String,
    pub source: String,
    pub variant: Option<String>,
    pub steps: Vec<Edit>,
}

/// The generated inputs of one language, draw for draw as `generateInputs`.
pub fn generate_inputs(language: &str, seeds: &[SeedSource], seed: &str) -> Vec<Input> {
    const SEPARATORS: [&str; 7] = [
        "\n",
        "\n\n",
        "\r\n",
        " ",
        "\u{00A0}\n",
        "\n\u{200B}",
        "\t\n",
    ];
    let mut random = Random::new(&format!("{seed}:{language}"));
    let mut inputs = Vec::new();
    let input = |id: String, source: String| Input {
        id,
        source,
        variant: None,
        steps: Vec::new(),
    };
    for index in 0..PROPERTY_CASES {
        let count = 2 + random.int(2);
        let parts: Vec<&str> = (0..count)
            .map(|_| random.pick(seeds).source.as_str())
            .collect();
        let mut source = parts[0].to_string();
        for part in &parts[1..] {
            source.push_str(random.pick::<&str>(&SEPARATORS));
            source.push_str(part);
        }
        inputs.push(input(format!("property/{index}"), source));
    }
    for index in 0..FUZZ_CASES {
        let mut source = random.pick(seeds).source.clone();
        for _ in 0..=random.int(4) {
            let edit = random_edit(&mut random, &source, language);
            source = apply_text_edit(&source, &edit);
        }
        inputs.push(input(format!("fuzz/{index}"), source));
    }
    for index in 0..METAMORPHIC_CASES {
        let base = &random.pick(seeds).source;
        for relation in RELATIONS {
            inputs.push(Input {
                variant: Some(relation_transform(relation, base)),
                ..input(format!("metamorphic/{index}/{relation}"), base.clone())
            });
        }
    }
    for index in 0..EDIT_CASES {
        let base = random.pick(seeds).source.clone();
        let mut source = base.clone();
        let mut steps = Vec::new();
        for _ in 0..EDIT_STEPS {
            let edit = random_edit(&mut random, &source, language);
            source = apply_text_edit(&source, &edit);
            steps.push(edit);
        }
        inputs.push(Input {
            steps,
            ..input(format!("edit/{index}"), base)
        });
    }
    inputs
}
