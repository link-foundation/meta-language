//! Canonical CST lines: every node of a concrete syntax tree in preorder, one
//! per line, as `<2 spaces per depth><field: ><•>LABEL SR:SC-ER:EC`, where
//! LABEL is the node kind (named), its JSON-quoted kind (anonymous), `ERROR`,
//! or `MISSING <kind>`, `•` marks nodes containing an error, and points are
//! zero-based rows and UTF-8 byte columns. The issue 195 conformance oracle
//! stores native `tree-sitter parse --cst` trees in this format; this module
//! renders public `LinkNetwork`s the same way and checks their trivia and
//! diagnostics. Mirrors `js/tests/support/cst-lines.js`.
#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::sync::OnceLock;

use meta_language::{LinkId, LinkNetwork, LinkType, SourceSpan};
use regex::Regex;

/// One node of canonical CST lines.
#[allow(clippy::struct_excessive_bools)] // The flags of one tree-sitter node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CstNode {
    pub depth: usize,
    pub field: Option<String>,
    pub has_error: bool,
    pub error: bool,
    pub missing: bool,
    pub named: bool,
    pub kind: String,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

fn line_expression() -> &'static Regex {
    static LINE: OnceLock<Regex> = OnceLock::new();
    LINE.get_or_init(|| {
        Regex::new(
            r#"^( *)(?:([a-z_][a-z0-9_]*): )?(•)?(ERROR|MISSING (?:"(?:[^"\\]|\\.)*"|\S+)|"(?:[^"\\]|\\.)*"|\S+) (\d+):(\d+)-(\d+):(\d+)$"#,
        )
        .expect("valid CST line expression")
    })
}

/// Parses canonical CST lines.
pub fn parse_cst_lines(text: &str) -> Vec<CstNode> {
    if text.is_empty() {
        return Vec::new();
    }
    text.split('\n')
        .map(|line| {
            let captures = line_expression()
                .captures(line)
                .unwrap_or_else(|| panic!("not a canonical CST line: {line:?}"));
            let number = |index: usize| captures[index].parse::<usize>().expect("CST point");
            let label = &captures[4];
            let missing = label.starts_with("MISSING ");
            let kind_label = label.strip_prefix("MISSING ").unwrap_or(label);
            let named = !kind_label.starts_with('"');
            CstNode {
                depth: captures[1].len() / 2,
                field: captures.get(2).map(|field| field.as_str().to_string()),
                has_error: captures.get(3).is_some(),
                error: label == "ERROR",
                missing,
                named,
                kind: if named {
                    kind_label.to_string()
                } else {
                    serde_json::from_str(kind_label).expect("JSON-quoted anonymous kind")
                },
                start: (number(5), number(6)),
                end: (number(7), number(8)),
            }
        })
        .collect()
}

/// Formats one node as a canonical CST line.
pub fn format_cst_line(node: &CstNode) -> String {
    let kind = if node.named {
        node.kind.clone()
    } else {
        serde_json::to_string(&node.kind).expect("kind")
    };
    let label = if node.error {
        "ERROR".to_string()
    } else if node.missing {
        format!("MISSING {kind}")
    } else {
        kind
    };
    let field = match &node.field {
        Some(field) if node.named || node.error || node.missing => format!("{field}: "),
        _ => String::new(),
    };
    let bullet = if node.has_error && !node.missing {
        "•"
    } else {
        ""
    };
    format!(
        "{}{field}{bullet}{label} {}:{}-{}:{}",
        "  ".repeat(node.depth),
        node.start.0,
        node.start.1,
        node.end.0,
        node.end.1
    )
}

/// The Syntax children and field labels of a network.
pub struct NetworkIndex<'a> {
    pub network: &'a LinkNetwork,
    children: HashMap<LinkId, Vec<LinkId>>,
    fields: HashMap<(LinkId, LinkId), String>,
    document_roots: Vec<LinkId>,
    region_roots: HashMap<LinkId, Vec<LinkId>>,
}

impl<'a> NetworkIndex<'a> {
    pub fn new(network: &'a LinkNetwork) -> Self {
        let mut index = Self {
            network,
            children: HashMap::new(),
            fields: HashMap::new(),
            document_roots: Vec::new(),
            region_roots: HashMap::new(),
        };
        let link_type = |id: LinkId| {
            network
                .link(id)
                .and_then(|link| link.metadata().link_type())
        };
        for link in network.links() {
            let metadata = link.metadata();
            match (metadata.link_type(), link.references()) {
                (Some(LinkType::Syntax), [parent]) => match link_type(*parent) {
                    Some(LinkType::Syntax) => {
                        index.children.entry(*parent).or_default().push(link.id());
                    }
                    Some(LinkType::Region) => {
                        index
                            .region_roots
                            .entry(*parent)
                            .or_default()
                            .push(link.id());
                    }
                    _ => index.document_roots.push(link.id()),
                },
                (Some(LinkType::Syntax), _) => index.document_roots.push(link.id()),
                (Some(LinkType::Field), [parent, label, child]) if metadata.term().is_none() => {
                    let label = network
                        .link(*label)
                        .and_then(|label| label.metadata().term())
                        .expect("field label term");
                    index.fields.insert((*parent, *child), label.to_string());
                }
                _ => {}
            }
        }
        // An incremental edit remaps link ids, so the tree order of siblings is
        // their source order: by start, then by end, as a zero-width sibling
        // (a MISSING node) precedes one that starts at the same byte and is
        // longer. The sort is stable, so zero-width siblings at one point keep
        // link order.
        let start = |id: &LinkId| {
            network
                .link(*id)
                .and_then(|link| link.metadata().span())
                .map_or((0, 0), |span| {
                    (span.byte_range().start(), span.byte_range().end())
                })
        };
        for children in index
            .children
            .values_mut()
            .chain(index.region_roots.values_mut())
        {
            children.sort_by_key(start);
        }
        index.document_roots.sort_by_key(start);
        index
    }

    pub fn children(&self, id: LinkId) -> &[LinkId] {
        self.children.get(&id).map_or(&[], Vec::as_slice)
    }

    fn term(&self, id: LinkId) -> Option<&str> {
        self.network
            .link(id)
            .and_then(|link| link.metadata().term())
    }

    // The public parser adds two documented layers on top of the upstream
    // grammar trees: Lean's `file` root wraps the grammar's `module` root, and
    // every Rocq `ident` leaf gets a semantic `identifier`/`primitive_type`
    // token child. The projection removes exactly those.
    fn project(&self, roots: &[LinkId], language: &str) -> Vec<LinkId> {
        roots
            .iter()
            .flat_map(|root| {
                if language == "Lean" && self.term(*root) == Some("file") {
                    self.children(*root).to_vec()
                } else {
                    vec![*root]
                }
            })
            .collect()
    }
}

/// Grammar roots to render, with the language and, for a region, its span.
pub struct GrammarRoots<'a> {
    pub index: &'a NetworkIndex<'a>,
    pub roots: Vec<LinkId>,
    pub language: String,
    pub span: Option<SourceSpan>,
}

/// The grammar roots of the document itself (not its embedded regions), after projection.
pub fn document_grammar_roots<'a>(index: &'a NetworkIndex<'a>, language: &str) -> GrammarRoots<'a> {
    GrammarRoots {
        index,
        roots: index.project(&index.document_roots, language),
        language: language.to_string(),
        span: None,
    }
}

/// The grammar roots of every region link, after projection.
pub fn region_grammar_roots<'a>(index: &'a NetworkIndex<'a>) -> Vec<GrammarRoots<'a>> {
    index
        .network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Region))
        .map(|region| {
            let language = region.metadata().language().unwrap_or_default().to_string();
            let roots = index
                .region_roots
                .get(&region.id())
                .map_or(&[][..], Vec::as_slice);
            GrammarRoots {
                index,
                roots: index.project(roots, &language),
                language,
                span: region.metadata().span(),
            }
        })
        .collect()
}

const SEMANTIC_ROCQ_LEAVES: [&str; 2] = ["identifier", "primitive_type"];

/// Renders grammar roots as canonical CST lines plus the rendered link ids.
pub fn render_cst_lines(roots: &GrammarRoots<'_>) -> (String, HashSet<LinkId>) {
    fn visit(
        roots: &GrammarRoots<'_>,
        id: LinkId,
        depth: usize,
        field: Option<&String>,
        lines: &mut Vec<String>,
        rendered: &mut HashSet<LinkId>,
    ) {
        let index = roots.index;
        let metadata = index.network.link(id).expect("syntax link").metadata();
        let mut children = index.children(id);
        if roots.language == "Rocq"
            && metadata.term() == Some("ident")
            && let [child] = children
            && index
                .term(*child)
                .is_some_and(|term| SEMANTIC_ROCQ_LEAVES.contains(&term))
            && index.children(*child).is_empty()
        {
            children = &[];
        }
        rendered.insert(id);
        let flags = metadata.flags();
        let span = metadata.span().expect("syntax span");
        lines.push(format_cst_line(&CstNode {
            depth,
            field: field.cloned(),
            has_error: flags.has_error(),
            error: flags.is_error(),
            missing: flags.is_missing(),
            named: metadata.is_named(),
            kind: metadata.term().unwrap_or_default().to_string(),
            start: (span.start_point().row(), span.start_point().column()),
            end: (span.end_point().row(), span.end_point().column()),
        }));
        for child in children {
            let field = index.fields.get(&(id, *child));
            visit(roots, *child, depth + 1, field, lines, rendered);
        }
    }
    let mut lines = Vec::new();
    let mut rendered = HashSet::new();
    for root in &roots.roots {
        visit(roots, *root, 0, None, &mut lines, &mut rendered);
    }
    (lines.join("\n"), rendered)
}

/// Byte offset of every row start in `source`.
pub fn row_offsets(source: &str) -> Vec<usize> {
    std::iter::once(0)
        .chain(
            source
                .bytes()
                .enumerate()
                .filter(|(_, byte)| *byte == b'\n')
                .map(|(index, _)| index + 1),
        )
        .collect()
}

const fn is_gap_whitespace(character: char) -> bool {
    character.is_whitespace() || matches!(character, '\u{200B}' | '\u{2060}' | '\u{FEFF}')
}

/// Trivia problems: every byte outside the oracle's leaf tokens must be
/// covered by a trivia link, and every trivia link must cover only such bytes
/// or exactly one oracle node (a comment). The one exception is text a hidden
/// grammar rule matched (Lean's `#eval`), which the CLI prints no leaf for: a
/// non-extra source token outside every leaf, trimmed of whitespace.
pub fn trivia_problems(
    network: &LinkNetwork,
    source: &str,
    oracle: &str,
    window: Option<(usize, usize)>,
) -> Vec<String> {
    let offsets = row_offsets(source);
    let to_byte = |(row, column): (usize, usize)| offsets[row] + column;
    let nodes = parse_cst_lines(oracle);
    let (from, to) = window.unwrap_or((0, source.len()));
    let mut in_leaf = vec![false; to - from];
    let mut node_ranges = HashSet::new();
    for (position, node) in nodes.iter().enumerate() {
        let (start, end) = (to_byte(node.start), to_byte(node.end));
        node_ranges.insert((start, end));
        let leaf = nodes
            .get(position + 1)
            .is_none_or(|next| next.depth != node.depth + 1);
        if leaf {
            in_leaf[start - from..end - from].fill(true);
        }
    }
    let mut covered = vec![false; to - from];
    let mut problems = Vec::new();
    for link in network.links() {
        let metadata = link.metadata();
        let hidden_text =
            metadata.link_type() == Some(LinkType::Token) && !metadata.flags().is_extra();
        let token_trivia = metadata.link_type() == Some(LinkType::Trivia)
            && metadata.term() == Some("token trivia");
        if !hidden_text && !token_trivia {
            continue;
        }
        let Some(span) = metadata.span() else {
            continue;
        };
        let (start, end) = (span.byte_range().start(), span.byte_range().end());
        if start < from || end > to {
            continue;
        }
        let gap = in_leaf[start - from..end - from].iter().all(|flag| !flag);
        if hidden_text {
            if !gap || start == end {
                continue;
            }
            let text = &source[start..end];
            if text.starts_with(is_gap_whitespace) || text.ends_with(is_gap_whitespace) {
                problems.push(format!(
                    "hidden-rule text {start}..{end} keeps surrounding whitespace"
                ));
            }
        } else if !gap && !node_ranges.contains(&(start, end)) {
            problems.push(format!("trivia {start}..{end} overlaps a token"));
        }
        covered[start - from..end - from].fill(true);
    }
    if let Some(byte) = (0..to - from).find(|&byte| !in_leaf[byte] && !covered[byte]) {
        problems.push(format!(
            "byte {} is outside every token and every trivia link",
            byte + from
        ));
    }
    problems
}

/// Diagnostic problems: the verification report must list exactly the
/// oracle's error, missing and error-containing nodes among the rendered
/// links, and nothing else; with `check_clean` (for a whole document),
/// `is_clean()` must also agree with the oracle.
pub fn diagnostic_problems(
    network: &LinkNetwork,
    rendered: &HashSet<LinkId>,
    oracle: &str,
    check_clean: bool,
) -> Vec<String> {
    let describe = |kind: &str, start: (usize, usize), end: (usize, usize)| {
        format!("{kind} {}:{}-{}:{}", start.0, start.1, end.0, end.1)
    };
    let mut expected: Vec<String> = parse_cst_lines(oracle)
        .iter()
        .filter(|node| node.has_error || node.missing || node.error)
        .map(|node| {
            let kind = if node.error {
                "error"
            } else if node.missing {
                "missing"
            } else {
                "has-error"
            };
            describe(kind, node.start, node.end)
        })
        .collect();
    expected.sort();
    let report = network.verify_full_match(None);
    let mut actual: Vec<String> = report
        .issues()
        .iter()
        .filter(|issue| rendered.contains(&issue.link_id()))
        .map(|issue| {
            let metadata = network
                .link(issue.link_id())
                .expect("issue link")
                .metadata();
            let flags = metadata.flags();
            let kind = if flags.is_error() {
                "error"
            } else if flags.is_missing() {
                "missing"
            } else {
                "has-error"
            };
            let span = metadata.span().expect("issue span");
            describe(
                kind,
                (span.start_point().row(), span.start_point().column()),
                (span.end_point().row(), span.end_point().column()),
            )
        })
        .collect();
    actual.sort();
    let mut problems = Vec::new();
    if actual != expected {
        problems.push(format!(
            "diagnostics {actual:?} differ from the oracle {expected:?}"
        ));
    }
    if check_clean && report.is_clean() != expected.is_empty() {
        problems.push(format!("is_clean() is {}", report.is_clean()));
    }
    problems
}

/// The first differing line of two CST texts, for failure messages.
pub fn first_difference(actual: &str, expected: &str) -> String {
    let left: Vec<&str> = actual.split('\n').collect();
    let right: Vec<&str> = expected.split('\n').collect();
    let at = (0..left.len().max(right.len()))
        .find(|&index| left.get(index) != right.get(index))
        .unwrap_or(0);
    format!(
        "line {}\n  actual   {}\n  expected {}",
        at + 1,
        left.get(at).unwrap_or(&""),
        right.get(at).unwrap_or(&"")
    )
}

/// Problems of the public tree of a whole `source` document against its
/// oracle CST lines: structure, kinds, fields, spans and flags, exact
/// reconstruction, trivia and diagnostics. Returns the rendered tree too.
pub fn document_oracle_problems(
    network: &LinkNetwork,
    language: &str,
    source: &str,
    oracle: &str,
) -> (Vec<String>, String) {
    let index = NetworkIndex::new(network);
    let (tree, rendered) = render_cst_lines(&document_grammar_roots(&index, language));
    let mut problems = Vec::new();
    if tree != oracle {
        problems.push(format!(
            "CST differs at {}",
            first_difference(&tree, oracle)
        ));
    }
    if network.reconstruct_text() != source {
        problems.push("reconstruction differs from the source".to_string());
    }
    problems.extend(trivia_problems(network, source, oracle, None));
    problems.extend(diagnostic_problems(network, &rendered, oracle, true));
    (problems, tree)
}

/// The S-expression of canonical CST lines: named nodes with fields, ERROR and MISSING.
pub fn cst_lines_to_sexp(text: &str) -> String {
    let mut output = String::new();
    let mut stack: Vec<usize> = Vec::new();
    let close = |depth: usize, stack: &mut Vec<usize>, output: &mut String| {
        while stack.last().is_some_and(|open| *open >= depth) {
            stack.pop();
            output.push(')');
        }
    };
    for node in parse_cst_lines(text) {
        close(node.depth, &mut stack, &mut output);
        let field = node
            .field
            .as_ref()
            .map_or_else(String::new, |field| format!("{field}: "));
        if node.missing {
            let kind = if node.named {
                node.kind.clone()
            } else {
                serde_json::to_string(&node.kind).expect("kind")
            };
            let _ = write!(output, " {field}(MISSING {kind}");
        } else if node.error {
            let _ = write!(output, " {field}(ERROR");
        } else if node.named {
            let _ = write!(output, " {field}({}", node.kind);
        } else {
            continue;
        }
        stack.push(node.depth);
    }
    close(0, &mut stack, &mut output);
    output.trim().to_string()
}
