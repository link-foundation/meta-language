//! Paired rooted views over actual concept and formal-definition links.
//! Classification and type roles are declared assumptions, not entailment.
use crate::discovered_concept_links::read_discovered_concept_links;
use crate::structured_data_links::{decode_structured_data_links, encode_structured_data_links};
use crate::{FormalConceptArchive, Link, LinkId, LinkMetadata, LinkNetwork, LinkType};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

#[path = "view_rules.rs"]
mod rules;
use rules::{accept_concept_view_relation, accept_concept_view_shape};

const LANGUAGE: &str = "meta-language concept views";
const LIMIT: usize = 100_000;
const BYTE_LIMIT: usize = 128 * 1024 * 1024;

/// Paired view roots sharing the same concept identities and definition nodes.
#[derive(Clone, Debug)]
pub struct ConceptViews {
    /// Complete linked concepts, model alternatives, provenance and views.
    pub network: LinkNetwork,
    /// The versioned paired-view root.
    pub root: LinkId,
    /// Actual ontology entry-point link.
    pub ontology: LinkId,
    /// Actual taxonomy entry-point link.
    pub taxonomy: LinkId,
}

fn fail(message: &str) -> String {
    format!("concept views: {message}")
}
fn text(value: &Value) -> Result<&str, String> {
    value.as_str().ok_or_else(|| fail("expected text"))
}
fn items(value: &Value) -> Result<&[Value], String> {
    if value.is_null() {
        Ok(&[])
    } else {
        value
            .as_array()
            .map(Vec::as_slice)
            .ok_or_else(|| fail("expected ordered list"))
    }
}
fn meta(term: &str, definition: Option<&str>) -> LinkMetadata {
    let value = LinkMetadata::new()
        .with_link_type(LinkType::Semantic)
        .with_named(true)
        .with_language(LANGUAGE)
        .with_term(term);
    definition.map_or_else(
        || value.clone(),
        |definition| value.clone().with_definition(definition),
    )
}
fn validate(network: &LinkNetwork) -> Result<(), String> {
    if network.len() > LIMIT {
        return Err(fail("node resource limit exceeded"));
    }
    let mut references = 0_usize;
    let mut bytes = 0_usize;
    for link in network.links() {
        if link.id().as_u64() == 0 || link.id().as_u64() > 9_007_199_254_740_991 {
            return Err(fail("invalid identity"));
        }
        if link.references().iter().any(|id| {
            network.link(*id).is_none() || id.as_u64() == 0 || id.as_u64() > 9_007_199_254_740_991
        }) {
            return Err(fail("foreign or dangling reference"));
        }
        references = references.saturating_add(link.references().len());
        for value in [
            link.metadata().term(),
            link.metadata().language(),
            link.metadata().definition(),
        ]
        .into_iter()
        .flatten()
        {
            bytes = bytes.saturating_add(value.len());
        }
        if references > LIMIT || bytes > BYTE_LIMIT {
            return Err(fail("graph resource limit exceeded"));
        }
    }
    Ok(())
}

#[derive(Default)]
struct Builder {
    network: LinkNetwork,
    archives: Vec<LinkId>,
    identities: BTreeMap<String, LinkId>,
    order: Vec<String>,
}
impl Builder {
    fn node(&mut self, term: &str, references: &[LinkId], definition: Option<&str>) -> LinkId {
        let id = self.network.insert_link([], meta(term, definition));
        self.network.set_references(id, references);
        id
    }
    fn sequence(&mut self, ids: &[LinkId]) -> LinkId {
        self.node("concept view sequence", ids, None)
    }
    fn copy(&mut self, source: &LinkNetwork) -> Result<(BTreeMap<LinkId, LinkId>, LinkId), String> {
        validate(source)?;
        if self.network.len().saturating_add(source.len()) > LIMIT {
            return Err(fail("node resource limit exceeded"));
        }
        let mut mapping = BTreeMap::new();
        for link in source.links() {
            if link.metadata().language() == Some(LANGUAGE) {
                return Err(fail("foreign view archive"));
            }
            mapping.insert(
                link.id(),
                self.network.insert_link([], link.metadata().clone()),
            );
        }
        for link in source.links() {
            let refs = link
                .references()
                .iter()
                .map(|id| mapping[id])
                .collect::<Vec<_>>();
            self.network.set_references(mapping[&link.id()], &refs);
        }
        let mut registered = Vec::new();
        for (term, id) in &source.terms {
            if source.link(*id).and_then(|link| link.metadata().term()) != Some(term.as_ref()) {
                return Err(fail("inconsistent archive registration"));
            }
            registered.push(*id);
        }
        registered.sort_unstable();
        let inventory = self.sequence(&mapping.values().copied().collect::<Vec<_>>());
        let registration =
            self.sequence(&registered.iter().map(|id| mapping[id]).collect::<Vec<_>>());
        let group = self.node("concept view archive", &[inventory, registration], None);
        self.archives.push(group);
        Ok((mapping, group))
    }
    fn archive(&mut self, value: &Value) -> Result<LinkId, String> {
        let formal = FormalConceptArchive::from_json(value)?;
        let (mapping, group) = self.copy(&formal.network)?;
        let root = mapping
            .get(&formal.root)
            .copied()
            .ok_or_else(|| fail("foreign interpretation root"))?;
        Ok(self.node("concept view definition", &[root, group], None))
    }
    fn identity(&self, name: &str) -> Result<LinkId, String> {
        self.identities
            .get(name)
            .copied()
            .ok_or_else(|| fail("unknown concept identity"))
    }
    fn add(&mut self, name: &str, id: LinkId) -> Result<(), String> {
        if name.is_empty() || self.identities.contains_key(name) {
            return Err(fail("ambiguous or colliding declared identity"));
        }
        self.identities.insert(name.to_owned(), id);
        self.order.push(name.to_owned());
        Ok(())
    }
    fn view(&mut self, role: &str, options: &Value) -> Result<LinkId, String> {
        let members = if options["members"].is_null() {
            self.order
                .iter()
                .map(|name| self.identities[name])
                .collect::<Vec<_>>()
        } else {
            items(&options["members"])?
                .iter()
                .map(|value| self.identity(text(value)?))
                .collect::<Result<Vec<_>, _>>()?
        };
        let known = members.iter().copied().collect::<BTreeSet<_>>();
        if known.len() != members.len() {
            return Err(fail("duplicate view membership"));
        }
        let mut roots = Vec::new();
        for value in items(&options["roots"])? {
            let target = self.identity(text(&value["identity"])?)?;
            if !known.contains(&target) {
                return Err(fail("root is outside view membership"));
            }
            let types = if value.get("typeModel").is_none() {
                Vec::new()
            } else {
                let model = self.archive(&value["typeModel"])?;
                vec![self.node("concept view root type declaration", &[target, model], None)]
            };
            let declarations = self.sequence(&types);
            roots.push(self.node("concept view root", &[target, declarations], None));
        }
        let mut relations = Vec::new();
        for value in items(&options["relations"])? {
            let kind = text(&value["kind"])?;
            if !accept_concept_view_relation(role.to_owned(), kind.to_owned()) {
                return Err(fail("relation does not belong to the selected view"));
            }
            let source = self.identity(text(&value["source"])?)?;
            let target = self.identity(text(&value["target"])?)?;
            if !known.contains(&source) || !known.contains(&target) {
                return Err(fail("relation is outside view membership"));
            }
            let model = self.archive(&value["model"])?;
            relations.push(self.node(
                "concept view relation",
                &[source, target, model],
                Some(kind),
            ));
        }
        let members = self.sequence(&members);
        let roots = self.sequence(&roots);
        let relations = self.sequence(&relations);
        Ok(self.node(role, &[members, roots, relations], None))
    }
}

/// Create paired linked views. Portable formal archives are transient input;
/// every archive node is copied into the result, including unknown models.
///
/// # Errors
/// Rejects colliding identities, dangling references and undeclared view roles.
pub fn create_concept_views(
    base: Option<(&LinkNetwork, LinkId)>,
    options: &Value,
) -> Result<ConceptViews, String> {
    let mut builder = Builder::default();
    let mut bases = Vec::new();
    if let Some((network, root)) = base {
        let checked = read_discovered_concept_links(network, root)?;
        let (mapping, group) = builder.copy(network)?;
        let root_node = network
            .link(root)
            .ok_or_else(|| fail("foreign base root"))?;
        let list = network
            .link(root_node.references()[2])
            .ok_or_else(|| fail("missing base identities"))?;
        for id in list.references() {
            let name = network
                .link(*id)
                .and_then(|link| link.metadata().definition())
                .ok_or_else(|| fail("ambiguous base identity"))?;
            if !items(&checked.space["concepts"])?
                .iter()
                .any(|concept| concept["id"] == name)
            {
                return Err(fail("ambiguous base identity"));
            }
            builder.add(name, mapping[id])?;
        }
        bases.push(builder.node("concept view base", &[mapping[&root], group], None));
    }
    let mut orphans = Vec::new();
    for orphan in items(&options["orphans"])? {
        let identity = text(&orphan["identity"])?;
        if identity.is_empty() || builder.identities.contains_key(identity) {
            return Err(fail("ambiguous or colliding declared identity"));
        }
        let definition = builder.archive(&orphan["definition"])?;
        let provenance = orphan
            .get("provenance")
            .cloned()
            .unwrap_or_else(|| json!({}));
        let data = encode_structured_data_links(&provenance)?;
        let (mapping, group) = builder.copy(&data.network)?;
        let provenance = builder.node(
            "concept view provenance",
            &[mapping[&data.root], group],
            None,
        );
        let concept = builder.node(
            "concept view concept",
            &[definition, provenance],
            Some(identity),
        );
        builder.add(identity, concept)?;
        orphans.push(concept);
    }
    let ontology = builder.view("ontology view", &options["ontology"])?;
    let taxonomy = builder.view("taxonomy view", &options["taxonomy"])?;
    let bases = builder.sequence(&bases);
    let orphans = builder.sequence(&orphans);
    let archives = builder.sequence(&builder.archives.clone());
    let root = builder.node(
        "concept view graph",
        &[bases, orphans, archives, ontology, taxonomy],
        Some("1"),
    );
    validate(&builder.network)?;
    read_concept_ontology(&builder.network, root, None)?;
    Ok(ConceptViews {
        network: builder.network,
        root,
        ontology,
        taxonomy,
    })
}

struct Reader<'a> {
    network: &'a LinkNetwork,
    owned: BTreeSet<LinkId>,
    groups: BTreeMap<LinkId, LinkNetwork>,
}
impl Reader<'_> {
    fn inspect(&mut self, id: LinkId, term: &str) -> Result<Link, String> {
        let link = self
            .network
            .link(id)
            .ok_or_else(|| fail("invalid view role"))?;
        let value = link.metadata();
        if value.term() != Some(term)
            || value.language() != Some(LANGUAGE)
            || value.link_type() != Some(LinkType::Semantic)
            || !value.is_named()
            || value.span().is_some()
            || value.flags() != crate::LinkFlags::clean()
            || !accept_concept_view_shape(
                term.to_owned(),
                link.references().len().to_string(),
                value.definition().is_some(),
            )
        {
            return Err(fail("invalid view role"));
        }
        self.owned.insert(id);
        Ok(link.clone())
    }
    fn list(&mut self, id: LinkId) -> Result<Vec<LinkId>, String> {
        Ok(self
            .inspect(id, "concept view sequence")?
            .references()
            .to_vec())
    }
    fn group(&mut self, id: LinkId) -> Result<(), String> {
        if self.groups.contains_key(&id) {
            return Err(fail("duplicate archive"));
        }
        let link = self.inspect(id, "concept view archive")?;
        let inventory = self.list(link.references()[0])?;
        let registration = self.list(link.references()[1])?;
        let members = inventory.iter().copied().collect::<BTreeSet<_>>();
        if members.len() != inventory.len()
            || registration.iter().copied().collect::<BTreeSet<_>>().len() != registration.len()
        {
            return Err(fail("duplicate archive identity"));
        }
        let mut result = LinkNetwork::new();
        for id in inventory {
            let link = self
                .network
                .link(id)
                .ok_or_else(|| fail("foreign archive identity"))?;
            if self.owned.contains(&id)
                || link.metadata().language() == Some(LANGUAGE)
                || link
                    .references()
                    .iter()
                    .any(|target| !members.contains(target))
            {
                return Err(fail("foreign archive identity"));
            }
            self.owned.insert(id);
            result.links.insert(id, Arc::new(link.clone()));
            result.next_id = result.next_id.max(id.as_u64().saturating_add(1));
        }
        for id in registration {
            let term = result
                .link(id)
                .and_then(|link| link.metadata().term())
                .ok_or_else(|| fail("foreign or ambiguous archive registration"))?;
            if result.terms.contains_key(term) {
                return Err(fail("foreign or ambiguous archive registration"));
            }
            let term = Arc::from(term);
            result.terms.insert(term, id);
        }
        self.groups.insert(id, result);
        Ok(())
    }
    fn archive(&mut self, id: LinkId, role: &str) -> Result<(LinkId, LinkId), String> {
        let link = self.inspect(id, role)?;
        let root = link.references()[0];
        let group = link.references()[1];
        if self
            .groups
            .get(&group)
            .and_then(|network| network.link(root))
            .is_none()
        {
            return Err(fail("foreign interpretation root"));
        }
        Ok((root, group))
    }
    fn view(
        &mut self,
        id: LinkId,
        role: &str,
        identities: &BTreeMap<String, Value>,
    ) -> Result<Value, String> {
        let link = self.inspect(id, role)?;
        let members = self.list(link.references()[0])?;
        let member_set = members.iter().copied().collect::<BTreeSet<_>>();
        let known = identities
            .values()
            .map(|value| LinkId::from_u64(value["link"].as_u64().unwrap()))
            .collect::<BTreeSet<_>>();
        if members.len() != member_set.len() || !member_set.is_subset(&known) {
            return Err(fail("foreign or duplicate membership"));
        }
        let mut roots = Vec::new();
        let mut root_ids = BTreeSet::new();
        for id in self.list(link.references()[1])? {
            if !root_ids.insert(id) {
                return Err(fail("duplicate root entry"));
            }
            let root = self.inspect(id, "concept view root")?;
            let concept = root.references()[0];
            if !member_set.contains(&concept) {
                return Err(fail("foreign root membership"));
            }
            let types = self.list(root.references()[1])?;
            if types.len() > 1 {
                return Err(fail("ambiguous root type declaration"));
            }
            let type_model = if let Some(id) = types.first() {
                let declaration = self.inspect(*id, "concept view root type declaration")?;
                if declaration.references()[0] != concept {
                    return Err(fail("foreign root type declaration"));
                }
                let (root, _) =
                    self.archive(declaration.references()[1], "concept view definition")?;
                Some(root.as_u64())
            } else {
                None
            };
            roots.push(
                json!({"link":id.as_u64(),"concept":concept.as_u64(),"typeModel":type_model}),
            );
        }
        let mut relations = Vec::new();
        let mut relation_ids = BTreeSet::new();
        for id in self.list(link.references()[2])? {
            if !relation_ids.insert(id) {
                return Err(fail("duplicate relation entry"));
            }
            let relation = self.inspect(id, "concept view relation")?;
            let source = relation.references()[0];
            let target = relation.references()[1];
            let kind = relation
                .metadata()
                .definition()
                .ok_or_else(|| fail("foreign relation or classification role"))?;
            if !accept_concept_view_relation(role.to_owned(), kind.to_owned())
                || !member_set.contains(&source)
                || !member_set.contains(&target)
            {
                return Err(fail("foreign relation or classification role"));
            }
            let (model, _) = self.archive(relation.references()[2], "concept view definition")?;
            relations.push(json!({"link":id.as_u64(),"source":source.as_u64(),"target":target.as_u64(),"kind":kind,"model":model.as_u64()}));
        }
        let concepts = members
            .iter()
            .map(|id| {
                identities
                    .values()
                    .find(|value| value["link"] == id.as_u64())
                    .unwrap()
                    .clone()
            })
            .collect::<Vec<_>>();
        Ok(json!({"view":id.as_u64(),"concepts":concepts,"roots":roots,"relations":relations}))
    }
}

fn add(
    identities: &mut BTreeMap<String, Value>,
    name: &str,
    id: LinkId,
    definitions: &[LinkId],
    provenance: Value,
    provenance_roots: &[LinkId],
) -> Result<(), String> {
    if name.is_empty() || identities.contains_key(name) {
        return Err(fail("ambiguous or colliding declared identity"));
    }
    identities.insert(name.to_owned(), json!({"identity":name,"link":id.as_u64(),"definitionRoots":definitions.iter().map(|id| id.as_u64()).collect::<Vec<_>>(),"provenanceRoots":provenance_roots.iter().map(|id| id.as_u64()).collect::<Vec<_>>(),"provenance":provenance}));
    Ok(())
}

fn read_views(network: &LinkNetwork, root: LinkId) -> Result<(Value, Value), String> {
    validate(network)?;
    let mut reader = Reader {
        network,
        owned: BTreeSet::new(),
        groups: BTreeMap::new(),
    };
    let graph = reader.inspect(root, "concept view graph")?;
    if graph.metadata().definition() != Some("1") {
        return Err(fail("unsupported schema version"));
    }
    let refs = graph.references();
    for id in reader.list(refs[2])? {
        reader.group(id)?;
    }
    let mut identities = BTreeMap::new();
    let bases = reader.list(refs[0])?;
    if bases.len() > 1 {
        return Err(fail("ambiguous grammar base"));
    }
    for id in bases {
        let (root, group) = reader.archive(id, "concept view base")?;
        let source = &reader.groups[&group];
        let checked = read_discovered_concept_links(source, root)?;
        let list = source
            .link(source.link(root).unwrap().references()[2])
            .unwrap();
        for target in list.references() {
            let identity = source.link(*target).unwrap();
            let name = identity.metadata().definition().unwrap();
            if !items(&checked.space["concepts"])?
                .iter()
                .any(|concept| concept["id"] == name)
            {
                return Err(fail("foreign global identity"));
            }
            let mut definitions = Vec::new();
            let mut seen = BTreeSet::new();
            for candidate in identity.references() {
                let formal_list = source
                    .link(source.link(*candidate).unwrap().references()[2])
                    .unwrap();
                for formal in formal_list.references() {
                    let definition = source.link(*formal).unwrap().references()[0];
                    if seen.insert(definition) {
                        definitions.push(definition);
                    }
                }
            }
            let provenance_roots = identity
                .references()
                .iter()
                .map(|candidate| source.link(*candidate).unwrap().references()[0])
                .collect::<Vec<_>>();
            let provenance = items(&checked.space["concepts"])?
                .iter()
                .find(|concept| concept["id"] == name)
                .unwrap()["origins"]
                .clone();
            add(
                &mut identities,
                name,
                *target,
                &definitions,
                provenance,
                &provenance_roots,
            )?;
        }
    }
    for id in reader.list(refs[1])? {
        let concept = reader.inspect(id, "concept view concept")?;
        let (definition, _) = reader.archive(concept.references()[0], "concept view definition")?;
        let (root, group) = reader.archive(concept.references()[1], "concept view provenance")?;
        let origin = decode_structured_data_links(&reader.groups[&group], root)?;
        add(
            &mut identities,
            concept.metadata().definition().unwrap(),
            id,
            &[definition],
            origin,
            &[root],
        )?;
    }
    let ontology = reader.view(refs[3], "ontology view", &identities)?;
    let taxonomy = reader.view(refs[4], "taxonomy view", &identities)?;
    if reader.owned.len() != network.len() {
        return Err(fail("unowned graph nodes"));
    }
    Ok((ontology, taxonomy))
}

fn select(mut view: Value, selected_root: Option<LinkId>) -> Result<Value, String> {
    let selected = items(&view["roots"])?
        .iter()
        .filter(|root| selected_root.is_none_or(|id| root["link"] == id.as_u64()))
        .cloned()
        .collect::<Vec<_>>();
    if selected_root.is_some() && selected.is_empty() {
        return Err(fail("unknown selected root"));
    }
    let mut pending = selected
        .iter()
        .map(|root| root["concept"].as_u64().unwrap())
        .collect::<Vec<_>>();
    let mut reachable = Vec::new();
    let mut visited = BTreeSet::new();
    let mut at = 0;
    while at < pending.len() {
        let id = pending[at];
        at += 1;
        if !visited.insert(id) {
            continue;
        }
        reachable.push(id);
        for edge in items(&view["relations"])? {
            if edge["source"] == id {
                pending.push(edge["target"].as_u64().unwrap());
            }
        }
    }
    view["selectedRoots"] = json!(
        selected
            .iter()
            .map(|root| root["link"].as_u64().unwrap())
            .collect::<Vec<_>>()
    );
    view["reachable"] = json!(reachable);
    Ok(view)
}

/// Read ontology concepts and declared general relations at explicit root links.
/// # Errors
/// Refuses malformed archives, foreign memberships and unknown selected roots.
pub fn read_concept_ontology(
    network: &LinkNetwork,
    root: LinkId,
    selected_root: Option<LinkId>,
) -> Result<Value, String> {
    select(read_views(network, root)?.0, selected_root)
}

/// Read only explicit classification and subsumption declarations.
/// # Errors
/// Refuses general relations presented as classification and malformed graphs.
pub fn read_concept_taxonomy(
    network: &LinkNetwork,
    root: LinkId,
    selected_root: Option<LinkId>,
) -> Result<Value, String> {
    select(read_views(network, root)?.1, selected_root)
}
