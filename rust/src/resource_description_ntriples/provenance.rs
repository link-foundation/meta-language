//! Native link allocation and retained syntax; no lexical policy is handwritten.

use std::collections::{BTreeMap, BTreeSet};

use super::{
    Parsed, ResourceDescriptionNTriplesLinks, Result, error, graph_error, graph_schema, identity,
    parse, profile, units,
};
use crate::resource_description_ntriples_rules::read_resource_description_n_triples_token;
use crate::{
    LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType, ResourceDescriptionLinks,
    decode_resource_description_graph,
};
use serde_json::{Value, json};

fn refs(network: &LinkNetwork, id: LinkId) -> Vec<LinkId> {
    network
        .link(id)
        .expect("validated linked node exists")
        .references()
        .to_vec()
}
fn owned_terms(network: &LinkNetwork, root: LinkId) -> BTreeSet<LinkId> {
    refs(network, refs(network, root)[1])
        .into_iter()
        .flat_map(|id| refs(network, id))
        .collect()
}
fn blank(network: &LinkNetwork, id: LinkId) -> Value {
    let children = refs(network, id);
    json!({"kind":"blank", "sourceIdentifier":network.link(children[0]).unwrap().metadata().definition().unwrap(),
        "value":network.link(children[1]).unwrap().metadata().definition().unwrap()})
}
struct Writer<'a> {
    network: &'a mut LinkNetwork,
}
impl Writer<'_> {
    fn node(
        &mut self,
        kind: &str,
        children: &[LinkId],
        definition: Option<&str>,
    ) -> Result<LinkId> {
        if self.network.len() >= 100_000 {
            return Err(error(
                "resourceLimit",
                0,
                "the syntax structural node limit is exceeded",
            ));
        }
        let mut data = LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_named(true)
            .with_language(profile()["language"].as_str().unwrap())
            .with_term(profile()["terms"][kind].as_str().unwrap());
        if let Some(value) = definition {
            data = data.with_definition(value);
        }
        Ok(self.network.insert_dynamic_link(children, data))
    }
    fn text(&mut self, value: &str) -> Result<LinkId> {
        self.node("text", &[], Some(value))
    }
    fn position(&mut self, value: usize) -> Result<LinkId> {
        self.node("position", &[], Some(&value.to_string()))
    }
    fn sequence(&mut self, values: &[LinkId]) -> Result<LinkId> {
        self.node("sequence", values, None)
    }
    fn premise(&mut self) -> Result<LinkId> {
        let specification = self.text(profile()["specification"].as_str().unwrap())?;
        self.node("profile", &[specification], profile()["profile"].as_str())
    }
}
struct Reader<'a> {
    network: &'a LinkNetwork,
    visited: BTreeSet<LinkId>,
    fields: BTreeSet<LinkId>,
}
impl<'a> Reader<'a> {
    fn new(network: &'a LinkNetwork) -> Self {
        Self {
            network,
            visited: BTreeSet::new(),
            fields: network
                .links()
                .filter(|link| {
                    link.metadata().link_type() == Some(LinkType::Field)
                        && link.references().len() == 3
                })
                .map(|link| link.references()[0])
                .collect(),
        }
    }
    fn node(
        &mut self,
        id: LinkId,
        kind: &str,
        arity: Option<usize>,
    ) -> Result<(Vec<LinkId>, Option<String>)> {
        let link = self
            .network
            .link(id)
            .ok_or_else(|| error("malformed", 0, "a syntax reference is dangling"))?;
        self.visited.insert(id);
        if self.visited.len() > 100_000 {
            return Err(error(
                "resourceLimit",
                0,
                "the syntax structural node limit is exceeded",
            ));
        }
        let data = link.metadata();
        if data.link_type() != Some(LinkType::Semantic)
            || !data.is_named()
            || data.language() != profile()["language"].as_str()
            || data.term() != profile()["terms"][kind].as_str()
            || data.span().is_some()
            || data.flags() != LinkFlags::clean()
            || self.fields.contains(&id)
        {
            return Err(error(
                "malformed",
                0,
                "a syntax node has a foreign role or concealed fields",
            ));
        }
        if arity.is_some_and(|size| link.references().len() != size) {
            return Err(error("malformed", 0, "a syntax node has the wrong arity"));
        }
        if !["text", "position", "profile", "token"].contains(&kind) && data.definition().is_some()
        {
            return Err(error(
                "malformed",
                0,
                "a syntax node has unexpected scalar data",
            ));
        }
        Ok((
            link.references().to_vec(),
            data.definition().map(str::to_owned),
        ))
    }
    fn text(&mut self, id: LinkId) -> Result<String> {
        let value = self
            .node(id, "text", Some(0))?
            .1
            .ok_or_else(|| error("malformed", 0, "a syntax text value is invalid"))?;
        if value.encode_utf16().count() > 262_144 {
            return Err(error("malformed", 0, "a syntax text value is invalid"));
        }
        Ok(value)
    }
    fn premise(&mut self, id: LinkId) -> Result<()> {
        let (children, definition) = self.node(id, "profile", Some(1))?;
        if definition.as_deref() != profile()["profile"].as_str()
            || self.text(children[0])? != profile()["specification"]
        {
            return Err(error(
                "unsupported",
                0,
                "the syntax profile is not supported",
            ));
        }
        Ok(())
    }
}

pub(super) fn correspondence(
    input: Option<&ResourceDescriptionLinks>,
) -> Result<BTreeMap<String, Value>> {
    let Some(input) = input else {
        return Ok(BTreeMap::new());
    };
    let mut read = Reader::new(&input.network);
    let children = read.node(input.root, "correspondenceRoot", Some(3))?.0;
    read.premise(children[2])?;
    decode_resource_description_graph(&input.network, children[0]).map_err(graph_error)?;
    let owned = owned_terms(&input.network, children[0]);
    let mut mapping = BTreeMap::new();
    let mut targets = BTreeSet::new();
    for id in read.node(children[1], "sequence", None)?.0 {
        let pair = read.node(id, "correspondence", Some(2))?.0;
        let label = read.text(pair[1])?;
        if !owned.contains(&pair[0])
            || input.network.link(pair[0]).unwrap().metadata().term()
                != graph_schema()["terms"]["blank"].as_str()
        {
            return Err(error(
                "malformed",
                0,
                "a blank correspondence is not owned by its graph",
            ));
        }
        let value = blank(&input.network, pair[0]);
        let lexical = units(&format!("_:{label}"));
        let token = read_resource_description_n_triples_token(&lexical, 0.0);
        if super::natural(token[0]) != 2
            || super::natural(token[1]) != lexical.len()
            || mapping.contains_key(&label)
            || !targets.insert(identity(&value))
        {
            return Err(error(
                "malformed",
                0,
                "blank correspondences must be valid and one to one",
            ));
        }
        mapping.insert(label, value);
    }
    Ok(mapping)
}
pub(super) fn attach(
    mut graph: ResourceDescriptionLinks,
    source: &str,
    parsed: &Parsed,
) -> Result<ResourceDescriptionNTriplesLinks> {
    let occurrences = refs(&graph.network, refs(&graph.network, graph.root)[2]);
    let mut output = Writer {
        network: &mut graph.network,
    };
    let mut tokens = Vec::new();
    for token in &parsed.tokens {
        let text = output.text(&token.raw)?;
        let start = output.position(token.start)?;
        let end = output.position(token.end)?;
        tokens.push(output.node("token", &[text, start, end], Some(&token.kind.to_string()))?);
    }
    let mut statements = Vec::new();
    for (index, statement) in parsed.statements.iter().enumerate() {
        let selected = statement
            .qualifiers
            .iter()
            .map(|index| tokens[*index])
            .collect::<Vec<_>>();
        let qualifiers = output.sequence(&selected)?;
        statements.push(output.node(
            "statement",
            &[
                occurrences[index],
                tokens[statement.tokens[0]],
                tokens[statement.tokens[1]],
                tokens[statement.tokens[2]],
                qualifiers,
                tokens[statement.period],
            ],
            None,
        )?);
    }
    let source = output.text(source)?;
    let tokens = output.sequence(&tokens)?;
    let statements = output.sequence(&statements)?;
    let premise = output.premise()?;
    let syntax_root = output.node(
        "syntaxRoot",
        &[graph.root, source, tokens, statements, premise],
        None,
    )?;
    Ok(ResourceDescriptionNTriplesLinks {
        network: graph.network,
        root: graph.root,
        syntax_root,
    })
}
fn read_tokens(read: &mut Reader<'_>, ids: &[LinkId]) -> Result<String> {
    let mut source = String::new();
    let mut position = 0;
    for id in ids {
        let (children, kind) = read.node(*id, "token", Some(3))?;
        let start = read.node(children[1], "position", Some(0))?.1;
        let end = read.node(children[2], "position", Some(0))?.1;
        let raw = read.text(children[0])?;
        let input = units(&raw);
        if start.as_deref() != Some(position.to_string().as_str())
            || end.as_deref() != Some((position + input.len()).to_string().as_str())
        {
            return Err(error(
                "malformed",
                position,
                "syntax token positions are not contiguous",
            ));
        }
        let token = read_resource_description_n_triples_token(&input, 0.0);
        if super::natural(token[0]) == 0
            || kind.as_deref() != Some(super::natural(token[0]).to_string().as_str())
            || super::natural(token[1]) != input.len()
        {
            return Err(error(
                "malformed",
                position,
                "a syntax token does not match its lexical role",
            ));
        }
        position += input.len();
        source.push_str(&raw);
        if position > 262_144 {
            return Err(error(
                "resourceLimit",
                position,
                "the syntax source limit is exceeded",
            ));
        }
    }
    Ok(source)
}
fn verify_statements(
    read: &mut Reader<'_>,
    ids: &[LinkId],
    tokens: &[LinkId],
    occurrences: &[LinkId],
    parsed: &Parsed,
) -> Result<()> {
    if ids.len() != parsed.statements.len() || occurrences.len() != ids.len() {
        return Err(error(
            "malformed",
            0,
            "syntax statements and occurrences differ",
        ));
    }
    for (index, id) in ids.iter().enumerate() {
        let children = read.node(*id, "statement", Some(6))?.0;
        let statement = &parsed.statements[index];
        let expected = [
            occurrences[index],
            tokens[statement.tokens[0]],
            tokens[statement.tokens[1]],
            tokens[statement.tokens[2]],
            children[4],
            tokens[statement.period],
        ];
        if children != expected {
            return Err(error(
                "malformed",
                0,
                "a syntax statement references a foreign occurrence or token",
            ));
        }
        let qualifiers = read.node(children[4], "sequence", None)?.0;
        let expected = statement
            .qualifiers
            .iter()
            .map(|index| tokens[*index])
            .collect::<Vec<_>>();
        if qualifiers != expected {
            return Err(error(
                "malformed",
                0,
                "a syntax qualifier references a foreign token",
            ));
        }
    }
    Ok(())
}

/// Read original syntax provenance. Canonical output uses the semantic RDF root.
///
/// # Errors
/// Rejects malformed lexical roles, positions and occurrence ownership.
pub fn read_resource_description_ntriples_source(
    network: &LinkNetwork,
    syntax_root: LinkId,
) -> Result<String> {
    let mut read = Reader::new(network);
    let children = read.node(syntax_root, "syntaxRoot", Some(5))?.0;
    read.premise(children[4])?;
    decode_resource_description_graph(network, children[0]).map_err(graph_error)?;
    let scope = read.text(children[1])?;
    let tokens = read.node(children[2], "sequence", None)?.0;
    let source = read_tokens(&mut read, &tokens)?;
    let parsed = parse(&source, &scope, &BTreeMap::new())?;
    let statements = read.node(children[3], "sequence", None)?.0;
    let occurrences = refs(network, refs(network, children[0])[2]);
    verify_statements(&mut read, &statements, &tokens, &occurrences, &parsed)?;
    Ok(source)
}
fn copy(
    source: &LinkNetwork,
    id: LinkId,
    target: &mut LinkNetwork,
    mapping: &mut BTreeMap<LinkId, LinkId>,
    active: &mut BTreeSet<LinkId>,
) -> Result<LinkId> {
    if let Some(mapped) = mapping.get(&id) {
        return Ok(*mapped);
    }
    if target.len() >= 100_000 || active.len() > 64 {
        return Err(error(
            "resourceLimit",
            0,
            "the correspondence graph limit is exceeded",
        ));
    }
    if !active.insert(id) {
        return Err(error(
            "malformed",
            0,
            "a correspondence graph has a structural cycle",
        ));
    }
    let link = source.link(id).expect("validated graph link exists");
    let children = link
        .references()
        .iter()
        .map(|id| copy(source, *id, target, mapping, active))
        .collect::<Result<Vec<_>>>()?;
    if target.len() >= 100_000 {
        return Err(error(
            "resourceLimit",
            0,
            "the correspondence graph limit is exceeded",
        ));
    }
    let mapped = target.insert_dynamic_link(&children, link.metadata().clone());
    mapping.insert(id, mapped);
    active.remove(&id);
    Ok(mapped)
}
pub(super) fn create_correspondence(
    network: &LinkNetwork,
    root: LinkId,
    labels: &BTreeMap<String, String>,
) -> Result<ResourceDescriptionLinks> {
    let mut target = LinkNetwork::new();
    let mut mapping = BTreeMap::new();
    let copied_root = copy(
        network,
        root,
        &mut target,
        &mut mapping,
        &mut BTreeSet::new(),
    )?;
    let owned = owned_terms(network, root)
        .into_iter()
        .filter(|id| {
            network.link(*id).unwrap().metadata().term()
                == graph_schema()["terms"]["blank"].as_str()
        })
        .map(|id| (identity(&blank(network, id)), id))
        .collect::<BTreeMap<_, _>>();
    let mut output = Writer {
        network: &mut target,
    };
    let mut pairs = Vec::new();
    let mut ordered = labels.iter().collect::<Vec<_>>();
    ordered.sort_by(|(left, _), (right, _)| left.encode_utf16().cmp(right.encode_utf16()));
    for (identity, label) in ordered {
        let text = output.text(label)?;
        pairs.push(output.node("correspondence", &[mapping[&owned[identity]], text], None)?);
    }
    let pairs = output.sequence(&pairs)?;
    let premise = output.premise()?;
    let root = output.node("correspondenceRoot", &[copied_root, pairs, premise], None)?;
    Ok(ResourceDescriptionLinks {
        network: target,
        root,
    })
}
