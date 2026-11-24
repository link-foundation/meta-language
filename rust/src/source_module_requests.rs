//! Actual CST field coordinates into shared module-request definitions.
use crate::translation::source_module_request_rules as rules;
use crate::translation::{
    Language,
    lexer::{TokenKind, tokenize},
};
use crate::{LinkId, LinkMetadata, LinkNetwork, LinkType, ParseConfiguration};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceModuleRequest {
    pub kind: String,
    pub specifier: Option<String>,
    pub specifier_units: Option<Vec<u16>>,
    pub start: usize,
    pub end: usize,
    pub link_id: String,
    pub syntax_link_id: String,
}
pub struct SourceModuleRequests {
    pub network: LinkNetwork,
    pub root: LinkId,
    pub requests: Vec<SourceModuleRequest>,
    pub coordinate_unit: &'static str,
}
#[derive(Clone, Copy)]
pub struct SourceModuleRequestLimits {
    pub byte_limit: usize,
    pub work_limit: usize,
    pub tsx: bool,
}
impl Default for SourceModuleRequestLimits {
    fn default() -> Self {
        Self {
            byte_limit: 4 * 1024 * 1024,
            work_limit: 1000000,
            tsx: false,
        }
    }
}
fn fail(message: &str) -> String {
    format!("source module requests: {message}")
}
fn sequence(network: &mut LinkNetwork, links: &[LinkId], term: &str) -> LinkId {
    let id = network.insert_dynamic_link(
        links,
        LinkMetadata::new().with_link_type(LinkType::Semantic),
    );
    network.set_term(id, term);
    id
}
struct Node {
    id: LinkId,
    term: String,
    start: usize,
    end: usize,
    metadata: LinkMetadata,
}
pub fn analyze_source_module_requests(
    source: &str,
    language: &str,
) -> Result<SourceModuleRequests, String> {
    analyze_source_module_requests_with_limits(
        source,
        language,
        SourceModuleRequestLimits::default(),
    )
}
pub fn analyze_source_module_requests_with_limits(
    source: &str,
    language: &str,
    limits: SourceModuleRequestLimits,
) -> Result<SourceModuleRequests, String> {
    if !matches!(language, "JavaScript" | "TypeScript") {
        return Err(fail("expected scalar JavaScript or TypeScript source"));
    }
    if limits.byte_limit == 0
        || limits.byte_limit > 128 * 1024 * 1024
        || limits.work_limit == 0
        || limits.work_limit > 10000000
    {
        return Err(fail("invalid resource limit"));
    }
    if source.len() > limits.byte_limit {
        return Err(fail("source byte limit exceeded"));
    }
    if limits.tsx && language != "TypeScript" {
        return Err(fail("invalid syntax profile"));
    }
    let mut network = LinkNetwork::parse(
        source,
        &rules::read_source_module_request_grammar(language, limits.tsx),
        ParseConfiguration::default(),
    );
    if !network.verify_full_match(None).is_clean() {
        return Err(fail("source does not parse cleanly"));
    }
    let mut work = network.links().count();
    if work > limits.work_limit {
        return Err(fail("request work limit exceeded"));
    }
    let facts = network
        .links()
        .filter_map(|link| {
            let metadata = link.metadata();
            let span = metadata.span()?;
            (metadata.link_type() == Some(LinkType::Syntax)).then(|| Node {
                id: link.id(),
                term: metadata.term().unwrap_or("").to_owned(),
                start: span.byte_range().start(),
                end: span.byte_range().end(),
                metadata: metadata.clone(),
            })
        })
        .collect::<Vec<_>>();
    let mut nodes = facts
        .iter()
        .filter(|node| rules::accept_source_module_request_term(&node.term))
        .collect::<Vec<_>>();
    nodes.sort_by_key(|node| (node.start, node.end, node.id.as_u64()));
    let mut statements = nodes
        .iter()
        .copied()
        .filter(|node| node.term != "call_expression")
        .collect::<Vec<_>>();
    statements.sort_by_key(|node| (node.start, std::cmp::Reverse(node.end)));
    let mut type_spans = facts
        .iter()
        .filter(|node| rules::accept_source_module_request_type_scope(&node.term))
        .map(|node| (node.start, node.end))
        .collect::<Vec<_>>();
    type_spans.sort_by_key(|span| span.0);
    let mut maximum = 0;
    for span in &mut type_spans {
        maximum = maximum.max(span.1);
        span.1 = maximum;
    }
    let mut sources = BTreeMap::new();
    let mut functions = BTreeMap::new();
    let mut arguments = BTreeMap::new();
    for link in network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Field))
    {
        work += 1;
        if work > limits.work_limit {
            return Err(fail("request work limit exceeded"));
        }
        let refs = link.references();
        if refs.len() != 3 {
            continue;
        }
        let parent = network
            .link(refs[0])
            .ok_or_else(|| fail("invalid concrete field relationship"))?;
        let label = network
            .link(refs[1])
            .and_then(|link| link.metadata().term())
            .unwrap_or("");
        let child = network
            .link(refs[2])
            .ok_or_else(|| fail("invalid concrete field relationship"))?;
        if label == "function" {
            functions.insert(parent.id(), child.id());
        }
        if label == "arguments" {
            arguments.insert(parent.id(), child.id());
        }
        if label != "source"
            || !rules::accept_source_module_request_literal(child.metadata().term().unwrap_or(""))
        {
            continue;
        }
        let Some(span) = parent.metadata().span() else {
            continue;
        };
        let span = span.byte_range();
        let position = statements.partition_point(|node| node.start <= span.start());
        if let Some(owner) = position
            .checked_sub(1)
            .and_then(|position| statements.get(position))
            && owner.end >= span.end()
        {
            sources.insert(owner.id, child.id());
        }
    }
    let mut requests = Vec::new();
    let mut request_links = Vec::new();
    for node in nodes {
        work += 1;
        if work > limits.work_limit {
            return Err(fail("request work limit exceeded"));
        }
        let mut second = String::new();
        let mut third = String::new();
        let literal;
        if node.term == "call_expression" {
            if !functions
                .get(&node.id)
                .and_then(|id| network.link(*id))
                .is_some_and(|link| {
                    rules::accept_source_module_request_import_callee(
                        link.metadata().term().unwrap_or(""),
                    )
                })
            {
                continue;
            }
            let args = arguments
                .get(&node.id)
                .and_then(|id| network.link(*id))
                .ok_or_else(|| fail("missing concrete import arguments"))?;
            work += args.references().len();
            if work > limits.work_limit {
                return Err(fail("request work limit exceeded"));
            }
            literal = args
                .references()
                .iter()
                .filter_map(|id| network.link(*id))
                .find(|link| {
                    link.metadata().link_type() == Some(LinkType::Syntax)
                        && link.metadata().is_named()
                        && !link.metadata().flags().is_extra()
                })
                .filter(|link| {
                    rules::accept_source_module_request_literal(
                        link.metadata().term().unwrap_or(""),
                    )
                })
                .map(|link| link.id());
        } else {
            let Some(id) = sources.get(&node.id).copied() else {
                continue;
            };
            literal = Some(id);
            let start = network
                .link(id)
                .and_then(|link| link.metadata().span())
                .ok_or_else(|| fail("missing concrete specifier span"))?
                .byte_range()
                .start();
            let text = source
                .get(node.start..start)
                .ok_or_else(|| fail("invalid concrete specifier span"))?;
            work += text.len();
            if work > limits.work_limit {
                return Err(fail("request work limit exceeded"));
            }
            let prefix = tokenize(text, Language::JavaScript)
                .map_err(|error| error.to_string())?
                .tokens;
            second = prefix
                .get(1)
                .map(|token| token.value.clone())
                .unwrap_or_default();
            third = prefix
                .get(2)
                .map(|token| token.value.clone())
                .unwrap_or_default();
        }
        let units = if let Some(id) = literal {
            let span = network
                .link(id)
                .and_then(|link| link.metadata().span())
                .ok_or_else(|| fail("missing concrete specifier span"))?
                .byte_range();
            let text = source
                .get(span.start()..span.end())
                .ok_or_else(|| fail("invalid concrete specifier span"))?;
            work += text.len();
            if work > limits.work_limit {
                return Err(fail("request work limit exceeded"));
            }
            let tokens = tokenize(text, Language::JavaScript)
                .map_err(|error| error.to_string())?
                .tokens;
            if tokens.len() != 2 || tokens[0].kind != TokenKind::String {
                return Err(fail("invalid concrete specifier literal"));
            }
            Some(
                tokens[0]
                    .string_units
                    .clone()
                    .unwrap_or_else(|| tokens[0].value.encode_utf16().collect::<Vec<_>>()),
            )
        } else {
            None
        };
        work += 3 + units.as_ref().map_or(0, Vec::len);
        if work > limits.work_limit {
            return Err(fail("request work limit exceeded"));
        }
        let specifier = units
            .as_ref()
            .and_then(|units| String::from_utf16(units).ok());
        let position = type_spans.partition_point(|span| span.0 <= node.start);
        let context =
            language == "TypeScript" && position > 0 && type_spans[position - 1].1 >= node.end;
        let kind = rules::read_source_module_request_kind(rules::read_source_module_request_code(
            language, &node.term, &second, &third, context,
        ));
        let kind_link = network.insert_typed_point(
            &kind,
            LinkType::Concept,
            Some("A source module dependency request kind."),
        );
        let value = if let Some(units) = &units {
            let values = units
                .iter()
                .map(|unit| {
                    network.insert_typed_point(
                        &format!("UTF-16 code unit {unit}"),
                        LinkType::Semantic,
                        Some(&unit.to_string()),
                    )
                })
                .collect::<Vec<_>>();
            sequence(&mut network, &values, "UTF-16 code unit sequence")
        } else {
            network.insert_typed_point(
                "computed module specifier",
                LinkType::Concept,
                Some("A module specifier requiring a source interpretation."),
            )
        };
        let id = network.insert_link(
            [kind_link, node.id, value],
            node.metadata
                .clone()
                .with_link_type(LinkType::Semantic)
                .with_term("source module request"),
        );
        request_links.push(id);
        requests.push(SourceModuleRequest {
            kind,
            specifier,
            specifier_units: units,
            start: node.start,
            end: node.end,
            link_id: id.as_u64().to_string(),
            syntax_link_id: node.id.as_u64().to_string(),
        });
    }
    let root = sequence(&mut network, &request_links, "source module requests");
    Ok(SourceModuleRequests {
        network,
        root,
        requests,
        coordinate_unit: "UTF-8 bytes",
    })
}

/// Exact UTF-8 or binary command output.
pub struct SourceModuleRequestCommandOutput {
    pub exit_code: i32,
    pub stdout: Vec<u8>,
    pub stderr: String,
}
/// Export the complete concrete source and request links through shared CLI arguments.
pub fn run_source_module_request_command<F>(
    args: &[String],
    read_file: F,
) -> SourceModuleRequestCommandOutput
where
    F: Fn(&str) -> Result<String, String>,
{
    const USAGE: &str = "usage: meta-language module-requests <file> [--language <JavaScript|TypeScript>] [--tsx] [--binary]\nOutputs complete CST and module-request links; TypeScript extensions select their grammar automatically.\n";
    let plan = rules::plan_source_module_request_command(args);
    if plan == [-1.0] {
        return SourceModuleRequestCommandOutput {
            exit_code: 0,
            stdout: USAGE.as_bytes().to_vec(),
            stderr: String::new(),
        };
    }
    if plan.len() != 4 {
        return SourceModuleRequestCommandOutput {
            exit_code: 2,
            stdout: Vec::new(),
            stderr: format!("error: invalid module request arguments\n{USAGE}"),
        };
    }
    let result = (|| -> Result<Vec<u8>, String> {
        let index = plan[0]
            .to_string()
            .parse::<usize>()
            .map_err(|_| fail("invalid shared command plan"))?;
        let path = args
            .get(index)
            .ok_or_else(|| fail("invalid shared command plan"))?;
        let source = read_file(path)?;
        let packet = analyze_source_module_requests_with_limits(
            &source,
            if plan[1] == 2.0 {
                "TypeScript"
            } else {
                "JavaScript"
            },
            SourceModuleRequestLimits {
                tsx: plan[2] == 1.0,
                ..SourceModuleRequestLimits::default()
            },
        )?;
        if plan[3] == 1.0 {
            crate::encode_link_network_binary(&packet.network, crate::BinaryLinksOptions::default())
                .map_err(|error| error.to_string())
        } else {
            Ok(packet.network.to_lino().into_bytes())
        }
    })();
    match result {
        Ok(stdout) => SourceModuleRequestCommandOutput {
            exit_code: 0,
            stdout,
            stderr: String::new(),
        },
        Err(error) => SourceModuleRequestCommandOutput {
            exit_code: 1,
            stdout: Vec::new(),
            stderr: format!("error: {error}\n"),
        },
    }
}
