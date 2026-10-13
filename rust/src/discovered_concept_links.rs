//! Source-free integration of explicit grammar discovery outputs.
//! Native graph storage and serialization wrap the existing shared semantic
//! verifier and the automatically translated discovery role admission policy.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use serde_json::{Map, Value, json};

use crate::discovered_concept_rules::accept_discovered_concept_shape;
use crate::formal_concept_definition::{FormalConceptArchive, read_formal_concept_definition};
use crate::global_concept_space::{
    GlobalConceptOptions, GlobalConceptSource, create_global_concept_space,
};
use crate::grammar::{GrammarKind, GrammarSourceName, serialize_grammar};
use crate::structured_data_links::{decode_structured_data_links, encode_structured_data_links};
use crate::{Link, LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType};

const LANGUAGE: &str = "meta-language discovered concepts";
const NODE_LIMIT: usize = 100_000;
const BYTE_LIMIT: usize = 128 * 1024 * 1024;

/// An inspectable discovery graph and the checked common concept space.
#[derive(Clone, Debug)]
pub struct DiscoveredConceptLinks {
    /// Actual candidate, identity and formal definition links.
    pub network: LinkNetwork,
    /// The versioned integration root.
    pub root: LinkId,
    /// The current checked global concept space.
    pub space: Value,
}

/// Reloaded executable candidate grammars and preserved formal alternatives.
#[derive(Clone, Debug)]
pub struct DiscoveredConceptGraph {
    /// Complete source-qualified candidate grammars.
    pub sources: Vec<GlobalConceptSource>,
    /// Revalidated global identities, proofs and unresolved obligations.
    pub space: Value,
    /// Preserved graph archives, including unsupported interpretations.
    pub formal_definitions: BTreeMap<String, FormalConceptArchive>,
}

fn fail(message: &str) -> String {
    format!("discovered concepts: {message}")
}

fn array(value: &Value) -> Result<&[Value], String> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| fail("expected candidate array"))
}

fn text(value: &Value) -> Result<&str, String> {
    value
        .as_str()
        .ok_or_else(|| fail("expected candidate text"))
}

fn metadata(term: &str, definition: Option<&str>) -> LinkMetadata {
    let result = LinkMetadata::new()
        .with_link_type(LinkType::Semantic)
        .with_named(true)
        .with_language(LANGUAGE)
        .with_term(term);
    if let Some(definition) = definition {
        result.with_definition(definition)
    } else {
        result
    }
}

fn node(
    network: &mut LinkNetwork,
    references: &[LinkId],
    term: &str,
    definition: Option<&str>,
) -> LinkId {
    let id = network.insert_link([], metadata(term, definition));
    network.set_references(id, references);
    id
}

fn sequence(network: &mut LinkNetwork, references: &[LinkId]) -> LinkId {
    node(network, references, "discovered concept sequence", None)
}

fn bare_space(space: &Value) -> Result<Value, String> {
    let mut result = space
        .as_object()
        .ok_or_else(|| fail("invalid global space"))?
        .clone();
    result.remove("formalDefinitions");
    Ok(Value::Object(result))
}

fn validate_network(network: &LinkNetwork) -> Result<(), String> {
    if network.len() > NODE_LIMIT {
        return Err(fail("node resource limit exceeded"));
    }
    let mut references: usize = 0;
    let mut bytes: usize = 0;
    for link in network.links() {
        if std::iter::once(link.id())
            .chain(link.references().iter().copied())
            .any(|id| id.as_u64() == 0 || id.as_u64() > 9_007_199_254_740_991)
        {
            return Err(fail("invalid graph identity"));
        }
        references = references.saturating_add(link.references().len());
        let data = link.metadata();
        for value in [data.term(), data.language(), data.definition()]
            .into_iter()
            .flatten()
        {
            bytes = bytes.saturating_add(value.len());
        }
        if references > NODE_LIMIT || bytes > BYTE_LIMIT {
            return Err(fail("graph resource limit exceeded"));
        }
    }
    Ok(())
}

fn copy_network(
    target: &mut LinkNetwork,
    source: &LinkNetwork,
) -> Result<BTreeMap<LinkId, LinkId>, String> {
    validate_network(source)?;
    if target.len().saturating_add(source.len()) > NODE_LIMIT {
        return Err(fail("node resource limit exceeded"));
    }
    let mut ids = BTreeMap::new();
    for link in source.links() {
        ids.insert(link.id(), target.insert_link([], link.metadata().clone()));
    }
    for link in source.links() {
        let references = link
            .references()
            .iter()
            .map(|id| {
                ids.get(id)
                    .copied()
                    .ok_or_else(|| fail("dangling formal definition reference"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        target.set_references(ids[&link.id()], &references);
    }
    Ok(ids)
}

fn island(
    network: &LinkNetwork,
    ids: &[LinkId],
    root: LinkId,
    registered: &[LinkId],
) -> Result<LinkNetwork, String> {
    let members: BTreeSet<_> = ids.iter().copied().collect();
    if members.len() != ids.len() || !members.contains(&root) {
        return Err(fail("invalid definition node inventory"));
    }
    let mut result = LinkNetwork::new();
    for id in ids {
        let link = network
            .link(*id)
            .ok_or_else(|| fail("foreign or dangling definition reference"))?;
        if link
            .references()
            .iter()
            .any(|reference| !members.contains(reference))
        {
            return Err(fail("foreign or dangling definition reference"));
        }
        result.links.insert(*id, Arc::new(link.clone()));
        result.next_id = result.next_id.max(id.as_u64().saturating_add(1));
    }
    let registrations: BTreeSet<_> = registered.iter().copied().collect();
    if registrations.len() != registered.len() {
        return Err(fail("duplicate registered definition point"));
    }
    for id in registered {
        let term = result
            .link(*id)
            .and_then(|link| link.metadata().term())
            .ok_or_else(|| fail("foreign or ambiguous registered definition point"))?;
        if result.terms.contains_key(term) {
            return Err(fail("foreign or ambiguous registered definition point"));
        }
        let term = Arc::from(term);
        result.terms.insert(term, *id);
    }
    Ok(result)
}

fn data_island(network: &LinkNetwork, root: LinkId) -> Result<LinkNetwork, String> {
    let mut seen = BTreeSet::new();
    let mut pending = vec![root];
    let mut at = 0;
    while at < pending.len() {
        let id = pending[at];
        at += 1;
        if !seen.insert(id) {
            continue;
        }
        let link = network.link(id).ok_or_else(|| fail("invalid data graph"))?;
        if seen.len() > NODE_LIMIT {
            return Err(fail("invalid data graph"));
        }
        pending.extend_from_slice(link.references());
    }
    island(network, &seen.into_iter().collect::<Vec<_>>(), root, &[])
}

fn encode_data(network: &mut LinkNetwork, value: &Value) -> Result<LinkId, String> {
    let data = encode_structured_data_links(value)?;
    Ok(copy_network(network, &data.network)?[&data.root])
}

fn dependencies(expression: &Value, source: &str, space: &Value) -> Result<Vec<String>, String> {
    let mut result = Vec::new();
    let mut pending = vec![expression];
    let mut at = 0;
    while at < pending.len() {
        let value = pending[at];
        at += 1;
        if at > NODE_LIMIT {
            return Err(fail("production traversal resource limit exceeded"));
        }
        if value["kind"] == "ref"
            && let Some(name) = value["name"].as_str()
            && let Some(target) = space["mappings"].get(format!("{source}:{name}"))
        {
            result.push(text(target)?.to_owned());
        }
        match value {
            Value::Array(values) => pending.extend(
                values
                    .iter()
                    .filter(|value| value.is_object() || value.is_array()),
            ),
            Value::Object(values) => pending.extend(
                values
                    .values()
                    .filter(|value| value.is_object() || value.is_array()),
            ),
            _ => {}
        }
    }
    Ok(result)
}

fn prepared_definitions(definitions: &Map<String, Value>) -> Vec<(String, Value)> {
    definitions
        .iter()
        .filter_map(|(alias, definition)| {
            read_formal_concept_definition(definition)
                .ok()
                .map(|definition| (alias.clone(), definition))
        })
        .collect()
}

fn definition_aliases(
    origin: &Value,
    space: &Value,
    definitions: &Map<String, Value>,
    prepared: &[(String, Value)],
) -> Result<Vec<String>, String> {
    let alias = text(&origin["alias"])?;
    let mut result = Vec::new();
    for name in [Some(alias), origin["concept"].as_str()]
        .into_iter()
        .flatten()
    {
        if definitions.contains_key(name) && !result.iter().any(|entry| entry == name) {
            result.push(name.to_owned());
        }
    }
    if let Some(accepted) = space["formalDefinitions"].get(alias) {
        let premise = read_formal_concept_definition(accepted)?;
        for (name, definition) in prepared {
            if premise == *definition && !result.contains(name) {
                result.push(name.clone());
            }
        }
    }
    Ok(result)
}

fn source_data(source: &GlobalConceptSource) -> Result<Value, String> {
    let mut grammar: Value = serde_json::from_str(&serialize_grammar(&source.grammar))
        .map_err(|error| error.to_string())?;
    let rules = grammar["rules"]
        .as_array_mut()
        .ok_or_else(|| fail("invalid candidate grammar"))?;
    for rule in rules {
        let original = source
            .grammar
            .rules()
            .iter()
            .find(|original| rule["name"] == original.name)
            .ok_or_else(|| fail("missing candidate production"))?;
        if let Some(concept) = &original.concept {
            rule["concept"] = json!(concept);
        }
        if !original.source_names.is_empty() {
            rule["sourceNames"] = Value::Array(
                original
                    .source_names
                    .iter()
                    .map(|alias| json!({"source": alias.source, "name": alias.name}))
                    .collect(),
            );
        }
    }
    grammar["kinds"] = Value::Array(source.grammar.kinds().iter().map(|kind| json!({
        "name": kind.name,
        "sourceNames": kind.source_names.iter().map(|alias| json!({"source": alias.source, "name": alias.name})).collect::<Vec<_>>()
    })).collect());
    Ok(
        json!({"id": source.id, "language": source.language, "edition": source.edition, "grammar": grammar}),
    )
}

fn source_from_data(value: &Value) -> Result<GlobalConceptSource, String> {
    let mut source = GlobalConceptSource::from_json(value)?;
    let kinds = value["grammar"]
        .get("kinds")
        .map(array)
        .transpose()?
        .unwrap_or_default()
        .iter()
        .map(|kind| {
            Ok(GrammarKind {
                name: text(&kind["name"])?.to_owned(),
                source_names: array(&kind["sourceNames"])?
                    .iter()
                    .map(|alias| {
                        Ok(GrammarSourceName {
                            source: text(&alias["source"])?.to_owned(),
                            name: text(&alias["name"])?.to_owned(),
                        })
                    })
                    .collect::<Result<Vec<_>, String>>()?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    source.grammar.set_kinds(kinds);
    Ok(source)
}

/// Copy explicit discovery outputs and retain every provided formal graph.
/// Sharing is decided by the existing checked global concept verifier.
///
/// # Errors
/// Returns an error for invalid candidate grammars, malformed formal archives,
/// unsupported graph records or exceeded resource bounds.
pub fn create_discovered_concept_links(
    sources: &[GlobalConceptSource],
    options: &GlobalConceptOptions,
) -> Result<DiscoveredConceptLinks, String> {
    let mut records = options.records.clone().unwrap_or_else(|| json!([]));
    let mut definitions = options
        .formal_definitions
        .as_ref()
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    for record in records
        .as_array_mut()
        .ok_or_else(|| fail("expected semantic records"))?
    {
        let id = text(&record["id"])?.to_owned();
        if let Some(definition) = record
            .as_object_mut()
            .and_then(|record| record.remove("formalDefinition"))
        {
            definitions.entry(id).or_insert(definition);
        }
    }
    let settings = GlobalConceptOptions {
        records: Some(records.clone()),
        formal_definitions: Some(Value::Object(definitions.clone())),
        previous: options.previous.clone(),
        equivalences: options.equivalences.clone(),
    };
    let space = create_global_concept_space(sources, &settings)?;
    let previous = options
        .previous
        .as_ref()
        .map(|previous| -> Result<Value, String> {
            Ok(Value::Array(
                array(&previous["concepts"])?
                    .iter()
                    .map(|concept| json!({"id": concept["id"], "origins": concept["origins"]}))
                    .collect(),
            ))
        })
        .transpose()?
        .unwrap_or(Value::Null);
    let value = json!({"schemaVersion": 1, "sources": sources.iter().map(source_data).collect::<Result<Vec<_>, _>>()?, "records": records, "equivalences": options.equivalences, "previous": previous, "space": bare_space(&space)?});
    let mut network = LinkNetwork::new();
    let data_root = encode_data(&mut network, &value)?;
    let mut definition_ids = BTreeMap::new();
    // Preserve the caller's declaration order, including unsupported meanings.
    for (alias, definition) in &definitions {
        let original = LinkNetwork::from_lino(text(&definition["links"])?)
            .map_err(|error| error.to_string())?;
        if original.terms.len() > original.len()
            || original.terms.iter().any(|(term, id)| {
                original.link(*id).and_then(|link| link.metadata().term()) != Some(term.as_ref())
            })
        {
            return Err(fail("inconsistent formal term registration"));
        }
        let ids = copy_network(&mut network, &original)?;
        let root = LinkId::from_u64(
            definition["root"]
                .as_u64()
                .ok_or_else(|| fail("missing formal definition root"))?,
        );
        let root = *ids
            .get(&root)
            .ok_or_else(|| fail("missing formal definition root"))?;
        let inventory = sequence(&mut network, &ids.values().copied().collect::<Vec<_>>());
        let registered = original
            .terms
            .values()
            .copied()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|id| ids[&id])
            .collect::<Vec<_>>();
        let registration = sequence(&mut network, &registered);
        definition_ids.insert(
            alias.clone(),
            node(
                &mut network,
                &[root, inventory, registration],
                "discovered formal definition",
                Some(alias),
            ),
        );
    }
    let mut identities = BTreeMap::new();
    for concept in array(&space["concepts"])? {
        let id = text(&concept["id"])?;
        identities.insert(
            id.to_owned(),
            node(&mut network, &[], "discovered concept identity", Some(id)),
        );
    }
    let mut candidates = Vec::new();
    let prepared = prepared_definitions(&definitions);
    for concept in array(&space["concepts"])? {
        let identity = identities[text(&concept["id"])?];
        let mut members = Vec::new();
        for origin in array(&concept["origins"])? {
            let alias = text(&origin["alias"])?;
            let origin_root = encode_data(&mut network, origin)?;
            let formal = definition_aliases(origin, &space, &definitions, &prepared)?
                .iter()
                .map(|name| definition_ids[name])
                .collect::<Vec<_>>();
            let definition = array(&concept["definitions"])?
                .iter()
                .find(|definition| definition["origin"] == alias)
                .ok_or_else(|| fail("missing candidate definition"))?;
            let references = dependencies(
                &definition["definition"]["expression"],
                text(&origin["source"])?,
                &space,
            )?
            .iter()
            .map(|id| identities[id])
            .collect::<Vec<_>>();
            let formal = sequence(&mut network, &formal);
            let references = sequence(&mut network, &references);
            let candidate = node(
                &mut network,
                &[origin_root, identity, formal, references],
                "discovered concept candidate",
                Some(alias),
            );
            members.push(candidate);
            candidates.push(candidate);
        }
        network.set_references(identity, &members);
    }
    let candidate_root = sequence(&mut network, &candidates);
    // Match the global verifier's concept order, rather than sorting digest IDs.
    let identity_ids = array(&space["concepts"])?
        .iter()
        .map(|concept| text(&concept["id"]).map(|id| identities[id]))
        .collect::<Result<Vec<_>, _>>()?;
    let identity_root = sequence(&mut network, &identity_ids);
    let formal_ids = definitions
        .keys()
        .map(|alias| definition_ids[alias])
        .collect::<Vec<_>>();
    let formal_root = sequence(&mut network, &formal_ids);
    let root = node(
        &mut network,
        &[data_root, candidate_root, identity_root, formal_root],
        "discovered concept graph",
        Some("1"),
    );
    validate_network(&network)?;
    Ok(DiscoveredConceptLinks {
        network,
        root,
        space,
    })
}

struct Reader<'a> {
    network: &'a LinkNetwork,
    owned: BTreeSet<LinkId>,
}

impl<'a> Reader<'a> {
    fn inspect(&mut self, id: LinkId, term: &str) -> Result<&'a Link, String> {
        let link = self
            .network
            .link(id)
            .ok_or_else(|| fail("invalid discovery graph role"))?;
        let data = link.metadata();
        if data.term() != Some(term)
            || data.link_type() != Some(LinkType::Semantic)
            || data.language() != Some(LANGUAGE)
            || !data.is_named()
            || data.span().is_some()
            || data.flags() != LinkFlags::clean()
            || !accept_discovered_concept_shape(
                term,
                &link.references().len().to_string(),
                data.definition().is_some(),
            )
        {
            return Err(fail("invalid discovery graph role"));
        }
        self.owned.insert(id);
        Ok(link)
    }

    fn list(&mut self, id: LinkId) -> Result<Vec<LinkId>, String> {
        Ok(self
            .inspect(id, "discovered concept sequence")?
            .references()
            .to_vec())
    }

    fn decode(&mut self, root: LinkId) -> Result<Value, String> {
        let graph = data_island(self.network, root)?;
        self.owned.extend(graph.links().map(Link::id));
        decode_structured_data_links(&graph, root)
    }
}

/// Reload complete candidate grammars, then revalidate every identity and proof.
///
/// # Errors
/// Returns an error for malformed graph records, foreign references, changed
/// formal premises, inconsistent term registrations or exceeded resource bounds.
pub fn read_discovered_concept_links(
    network: &LinkNetwork,
    root: LinkId,
) -> Result<DiscoveredConceptGraph, String> {
    validate_network(network)?;
    let mut reader = Reader {
        network,
        owned: BTreeSet::new(),
    };
    let graph = reader.inspect(root, "discovered concept graph")?;
    if graph.metadata().definition() != Some("1") {
        return Err(fail("unknown schema version"));
    }
    let [data_root, candidate_root, identity_root, formal_root] = graph.references() else {
        return Err(fail("invalid discovery root"));
    };
    let value = reader.decode(*data_root)?;
    let names = value
        .as_object()
        .ok_or_else(|| fail("invalid discovery data envelope"))?
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if value["schemaVersion"] != 1
        || names
            != [
                "equivalences",
                "previous",
                "records",
                "schemaVersion",
                "sources",
                "space",
            ]
            .into_iter()
            .collect()
    {
        return Err(fail("invalid discovery data envelope"));
    }
    let mut definitions = Map::new();
    let mut formal_archives = BTreeMap::new();
    let mut definition_ids = BTreeMap::new();
    for id in reader.list(*formal_root)? {
        let record = reader.inspect(id, "discovered formal definition")?;
        let alias = record.metadata().definition().unwrap_or("");
        if alias.is_empty() || definitions.contains_key(alias) {
            return Err(fail("duplicate formal interpretation"));
        }
        let [definition, inventory, registration] = record.references() else {
            return Err(fail("invalid formal interpretation"));
        };
        let members = reader.list(*inventory)?;
        let registered = reader.list(*registration)?;
        let formal = island(network, &members, *definition, &registered)?;
        for link in formal.links() {
            if reader.owned.contains(&link.id()) || link.metadata().language() == Some(LANGUAGE) {
                return Err(fail("formal interpretations share foreign graph nodes"));
            }
            reader.owned.insert(link.id());
        }
        definitions.insert(
            alias.to_owned(),
            json!({"links": formal.to_lino(), "root": definition.as_u64()}),
        );
        formal_archives.insert(
            alias.to_owned(),
            FormalConceptArchive {
                network: formal,
                root: *definition,
            },
        );
        definition_ids.insert(id, alias.to_owned());
    }
    let sources = array(&value["sources"])?
        .iter()
        .map(source_from_data)
        .collect::<Result<Vec<_>, _>>()?;
    let settings = GlobalConceptOptions {
        records: Some(value["records"].clone()),
        formal_definitions: Some(Value::Object(definitions.clone())),
        previous: (!value["previous"].is_null()).then(|| json!({"concepts": value["previous"]})),
        equivalences: array(&value["equivalences"])?
            .iter()
            .map(|pair| Ok([text(&pair[0])?.to_owned(), text(&pair[1])?.to_owned()]))
            .collect::<Result<Vec<_>, String>>()?,
    };
    let space = create_global_concept_space(&sources, &settings)?;
    if bare_space(&space)? != value["space"] {
        return Err(fail("stale or counterfeit global concept certificate"));
    }
    let mut identities = BTreeMap::new();
    let mut identities_by_id = BTreeMap::new();
    for id in reader.list(*identity_root)? {
        let identity = reader.inspect(id, "discovered concept identity")?;
        let name = identity.metadata().definition().unwrap_or("");
        if identities.contains_key(name)
            || !array(&space["concepts"])?
                .iter()
                .any(|concept| concept["id"] == name)
        {
            return Err(fail("unknown or duplicate global identity"));
        }
        identities.insert(name.to_owned(), identity);
        identities_by_id.insert(id, name.to_owned());
    }
    if identities.len() != array(&space["concepts"])?.len() {
        return Err(fail("missing global identity"));
    }
    let mut candidates = BTreeMap::new();
    let prepared = prepared_definitions(&definitions);
    for id in reader.list(*candidate_root)? {
        let candidate = reader.inspect(id, "discovered concept candidate")?;
        let alias = candidate.metadata().definition().unwrap_or("");
        let concept_id = space["mappings"]
            .get(alias)
            .and_then(Value::as_str)
            .ok_or_else(|| fail("unknown or duplicate candidate origin"))?;
        if candidates.contains_key(alias) {
            return Err(fail("unknown or duplicate candidate origin"));
        }
        let [origin_root, identity, formal, references] = candidate.references() else {
            return Err(fail("invalid candidate record"));
        };
        let concept = array(&space["concepts"])?
            .iter()
            .find(|concept| concept["id"] == concept_id)
            .ok_or_else(|| fail("unknown candidate identity"))?;
        let origin = array(&concept["origins"])?
            .iter()
            .find(|origin| origin["alias"] == alias)
            .ok_or_else(|| fail("unknown candidate origin"))?;
        if reader.decode(*origin_root)? != *origin
            || identities_by_id.get(identity).map(String::as_str) != Some(concept_id)
        {
            return Err(fail("foreign candidate provenance or identity"));
        }
        let expected = definition_aliases(origin, &space, &definitions, &prepared)?;
        let actual = reader
            .list(*formal)?
            .iter()
            .map(|id| definition_ids.get(id).cloned())
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| fail("foreign formal interpretation"))?;
        if actual != expected {
            return Err(fail("foreign formal interpretation"));
        }
        let definition = array(&concept["definitions"])?
            .iter()
            .find(|definition| definition["origin"] == alias)
            .ok_or_else(|| fail("missing candidate definition"))?;
        let actual = reader
            .list(*references)?
            .iter()
            .map(|id| identities_by_id.get(id).cloned())
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| fail("foreign recursive concept dependency"))?;
        if actual
            != dependencies(
                &definition["definition"]["expression"],
                text(&origin["source"])?,
                &space,
            )?
        {
            return Err(fail("foreign recursive concept dependency"));
        }
        candidates.insert(alias.to_owned(), id);
    }
    if candidates.len()
        != space["mappings"]
            .as_object()
            .ok_or_else(|| fail("missing discovered candidate"))?
            .len()
    {
        return Err(fail("missing discovered candidate"));
    }
    for concept in array(&space["concepts"])? {
        let members = array(&concept["origins"])?
            .iter()
            .map(|origin| text(&origin["alias"]).map(|alias| candidates[alias]))
            .collect::<Result<Vec<_>, _>>()?;
        if identities[text(&concept["id"])?].references() != members {
            return Err(fail("incomplete global identity membership"));
        }
    }
    if reader.owned.len() != network.len() {
        return Err(fail("unowned discovery graph nodes"));
    }
    Ok(DiscoveredConceptGraph {
        sources,
        space,
        formal_definitions: formal_archives,
    })
}
