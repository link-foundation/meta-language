//! Renders the Syntax links of a public `LinkNetwork` as a tree-sitter
//! S-expression (`tree-sitter parse` style: named nodes, field labels, ERROR and
//! MISSING), and reads tree-sitter test corpora (`test/corpus/*.txt`), so public
//! parses can be compared with trees printed by upstream grammars and the
//! native CLI. Mirrors `js/tests/support/cst-sexpression.js`.
#![allow(dead_code)]

use std::collections::HashMap;

use meta_language::{LinkId, LinkNetwork, LinkType};

/// One case of a tree-sitter corpus file.
#[derive(Clone, Debug)]
pub struct CorpusCase {
    pub name: String,
    pub attributes: Vec<String>,
    pub source: String,
    pub expected: String,
}

fn trim_line_end(text: &str) -> &str {
    let text = text.strip_suffix('\n').unwrap_or(text);
    text.strip_suffix('\r').unwrap_or(text)
}

/// Splits a tree-sitter corpus file into cases, as `parseCorpus` does.
pub fn parse_corpus(text: &str) -> Vec<CorpusCase> {
    // A header is a line of `=`s (with an optional suffix), the name and
    // attribute lines, and the same `=` line again.
    let lines: Vec<(usize, &str)> = text
        .split_inclusive('\n')
        .scan(0, |offset, line| {
            let start = *offset;
            *offset += line.len();
            Some((start, line))
        })
        .collect();
    let is_rule = |line: &str| trim_line_end(line).len() >= 3 && line.starts_with("===");
    let mut headers = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        let (start, line) = lines[index];
        if is_rule(line) {
            let rule = trim_line_end(line);
            if let Some(close) = (index + 2..lines.len())
                .find(|&candidate| trim_line_end(lines[candidate].1) == rule)
            {
                let body_start = lines[close].0 + lines[close].1.len();
                let header_lines: Vec<&str> = lines[index + 1..close]
                    .iter()
                    .map(|(_, line)| trim_line_end(line))
                    .collect();
                let suffix = rule.trim_start_matches('=').trim().to_string();
                headers.push((start, body_start, header_lines, suffix));
                index = close + 1;
                continue;
            }
        }
        index += 1;
    }
    let mut cases = Vec::new();
    for (position, (_start, body_start, header_lines, suffix)) in headers.iter().enumerate() {
        let body_end = headers.get(position + 1).map_or(text.len(), |next| next.0);
        let body = &text[*body_start..body_end];
        let mut divider = None;
        let mut offset = 0;
        for line in body.split_inclusive('\n') {
            let content = trim_line_end(line);
            let dashes = content.len() - content.trim_start_matches('-').len();
            if dashes >= 3 && &content[dashes..] == suffix {
                divider = Some((offset, offset + line.len()));
            }
            offset += line.len();
        }
        let source = trim_line_end(&body[..divider.map_or(body.len(), |(start, _)| start)]);
        let expected = divider.map_or("", |(_, end)| body[end..].trim());
        cases.push(CorpusCase {
            name: header_lines
                .first()
                .map_or("", |name| name.trim())
                .to_string(),
            attributes: header_lines[1.min(header_lines.len())..]
                .iter()
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty())
                .collect(),
            source: source.to_string(),
            expected: expected.to_string(),
        });
    }
    cases
}

struct Tree<'a> {
    network: &'a LinkNetwork,
    children: HashMap<LinkId, Vec<LinkId>>,
    fields: HashMap<(LinkId, LinkId), String>,
    language: &'a str,
}

impl Tree<'_> {
    fn children(&self, id: LinkId) -> &[LinkId] {
        self.children.get(&id).map_or(&[], Vec::as_slice)
    }

    fn render(&self, id: LinkId, field: Option<&str>) -> String {
        let metadata = self.network.link(id).expect("syntax link").metadata();
        let children = self.children(id);
        let prefix = field.map_or_else(String::new, |field| format!("{field}: "));
        let term = metadata.term().unwrap_or_default();
        // Every Rocq `ident` leaf carries a semantic identifier/primitive_type token child.
        if self.language == "Rocq" && term == "ident" {
            if let [child] = children {
                let child_term = self
                    .network
                    .link(*child)
                    .and_then(|link| link.metadata().term());
                if matches!(child_term, Some("identifier" | "primitive_type"))
                    && self.children(*child).is_empty()
                {
                    return format!("{prefix}(ident)");
                }
            }
        }
        let parts: Vec<String> = children
            .iter()
            .map(|child| self.render(*child, self.fields.get(&(id, *child)).map(String::as_str)))
            .filter(|part| !part.is_empty())
            .collect();
        let tail = if parts.is_empty() {
            String::new()
        } else {
            format!(" {}", parts.join(" "))
        };
        let flags = metadata.flags();
        if flags.is_missing() {
            let term = if metadata.is_named() {
                term.to_string()
            } else {
                serde_json::to_string(term).expect("term")
            };
            return format!("{prefix}(MISSING {term})");
        }
        if flags.is_error() {
            return format!("{prefix}(ERROR{tail})");
        }
        if !metadata.is_named() {
            return parts.join(" ");
        }
        format!("{prefix}({term}{tail})")
    }
}

/// The grammar tree of `network` as an S-expression, with the public layers
/// (Lean's `file` root and Rocq's semantic `ident` children) projected away.
pub fn render_network(network: &LinkNetwork, language: &str) -> String {
    let mut children: HashMap<LinkId, Vec<LinkId>> = HashMap::new();
    let mut fields = HashMap::new();
    let mut roots = Vec::new();
    for link in network.links() {
        let metadata = link.metadata();
        match metadata.link_type() {
            Some(LinkType::Syntax) => match link.references() {
                [parent]
                    if network
                        .link(*parent)
                        .and_then(|parent| parent.metadata().link_type())
                        == Some(LinkType::Syntax) =>
                {
                    children.entry(*parent).or_default().push(link.id());
                }
                _ => roots.push(link.id()),
            },
            Some(LinkType::Field) if metadata.term().is_none() => {
                if let [parent, label, child] = link.references() {
                    let label = network
                        .link(*label)
                        .and_then(|label| label.metadata().term())
                        .expect("field label term");
                    fields.insert((*parent, *child), label.to_string());
                }
            }
            _ => {}
        }
    }
    let tree = Tree {
        network,
        children,
        fields,
        language,
    };
    let grammar_roots: Vec<LinkId> = roots
        .iter()
        .flat_map(|root| {
            let term = network.link(*root).and_then(|link| link.metadata().term());
            if language == "Lean" && term == Some("file") {
                tree.children(*root).to_vec()
            } else {
                vec![*root]
            }
        })
        .collect();
    grammar_roots
        .iter()
        .map(|root| tree.render(*root, None))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Collapses whitespace so corpus trees and rendered trees compare as strings.
pub fn normalize(sexp: &str) -> String {
    sexp.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace(" )", ")")
        .replace("( ", "(")
}

/// Drops field labels, for corpus cases whose expected tree omits them.
pub fn strip_fields(sexp: &str) -> String {
    sexp.split(' ')
        .filter(|word| {
            !(word.len() > 1
                && word.ends_with(':')
                && word[..word.len() - 1].chars().all(|character| {
                    character.is_alphanumeric() || character == '_' || character == '-'
                }))
        })
        .collect::<Vec<_>>()
        .join(" ")
}
