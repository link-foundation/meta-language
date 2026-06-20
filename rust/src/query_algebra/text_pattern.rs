use std::collections::BTreeSet;

use crate::{LinkId, LinkNetwork, LinkType};

use super::{
    normalize_capture_name, structural_children, LinkRuleCaptures, LinkRuleMatch,
    LinkRuleParseError,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct TextPattern {
    parts: Vec<TextPatternPart>,
}

impl TextPattern {
    pub(super) fn parse(source: String) -> Result<Self, LinkRuleParseError> {
        let mut parts = Vec::new();
        let mut rest = source.as_str();
        while let Some(start) = rest.find("{{") {
            if start > 0 {
                parts.push(TextPatternPart::Literal(rest[..start].to_string()));
            }
            let after_open = &rest[start + 2..];
            let Some(end) = after_open.find("}}") else {
                return Err(LinkRuleParseError::new("unterminated text placeholder"));
            };
            let name = after_open[..end].trim();
            if name.is_empty() {
                return Err(LinkRuleParseError::new("text placeholder is empty"));
            }
            parts.push(TextPatternPart::Placeholder(normalize_capture_name(name)));
            rest = &after_open[end + 2..];
        }
        if !rest.is_empty() {
            parts.push(TextPatternPart::Literal(rest.to_string()));
        }
        if parts.is_empty() {
            parts.push(TextPatternPart::Literal(source));
        }
        Ok(Self { parts })
    }

    pub(super) fn matches(&self, network: &LinkNetwork) -> Vec<LinkRuleMatch> {
        network
            .links()
            .filter(|link| link.metadata().link_type() == Some(LinkType::Document))
            .filter_map(|document| {
                let tokens = source_tokens(network, document.id());
                let text = tokens
                    .iter()
                    .filter_map(|(_, _, term)| term.as_deref())
                    .collect::<String>();
                let captures = self.match_text(&text, &tokens)?;
                Some(LinkRuleMatch {
                    link_id: document.id(),
                    captures,
                })
            })
            .collect()
    }

    fn match_text(
        &self,
        text: &str,
        tokens: &[(LinkId, std::ops::Range<usize>, Option<String>)],
    ) -> Option<LinkRuleCaptures> {
        let mut captures = LinkRuleCaptures::default();
        let mut position = 0;
        for (index, part) in self.parts.iter().enumerate() {
            match part {
                TextPatternPart::Literal(literal) => {
                    let remaining = text.get(position..)?;
                    if !remaining.starts_with(literal) {
                        return None;
                    }
                    position += literal.len();
                }
                TextPatternPart::Placeholder(name) => {
                    let capture_start = position;
                    let capture_end = if let Some(literal) = next_literal(&self.parts[index + 1..])
                    {
                        text[position..]
                            .find(literal)
                            .map(|offset| position + offset)?
                    } else {
                        text.len()
                    };
                    let captured_text_value = text.get(capture_start..capture_end)?.to_string();
                    let link_ids = tokens
                        .iter()
                        .filter(|(_, range, _)| {
                            range.start >= capture_start && range.end <= capture_end
                        })
                        .map(|(link_id, _, _)| *link_id)
                        .collect::<Vec<_>>();
                    captures = captures.with_text(name, captured_text_value, link_ids);
                    position = capture_end;
                }
            }
        }
        (position == text.len()).then_some(captures)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TextPatternPart {
    Literal(String),
    Placeholder(String),
}

fn next_literal(parts: &[TextPatternPart]) -> Option<&str> {
    parts.iter().find_map(|part| match part {
        TextPatternPart::Literal(literal) if !literal.is_empty() => Some(literal.as_str()),
        _ => None,
    })
}

fn source_tokens(
    network: &LinkNetwork,
    root: LinkId,
) -> Vec<(LinkId, std::ops::Range<usize>, Option<String>)> {
    let mut tokens = Vec::new();
    collect_tokens(network, root, &mut BTreeSet::new(), &mut tokens);
    tokens.sort_by_key(|(link_id, range, _)| (range.start, link_id.as_u64()));
    tokens
}

fn collect_tokens(
    network: &LinkNetwork,
    link_id: LinkId,
    visited: &mut BTreeSet<LinkId>,
    tokens: &mut Vec<(LinkId, std::ops::Range<usize>, Option<String>)>,
) {
    if !visited.insert(link_id) {
        return;
    }
    let Some(link) = network.link(link_id) else {
        return;
    };
    if link.metadata().link_type() == Some(LinkType::Token) && !link.metadata().flags().is_missing()
    {
        if let Some(span) = link.metadata().span() {
            tokens.push((
                link_id,
                span.byte_range().start()..span.byte_range().end(),
                link.metadata().term().map(str::to_string),
            ));
        }
        return;
    }
    for child in structural_children(network, link_id) {
        collect_tokens(network, child, visited, tokens);
    }
}
