//! Inspectable RDF 1.1 abstract graph terms, provenance and equality premises.
//! Shared automatically translated rules decide lexical admission and equality;
//! this adapter supplies native link identities and structured input boundaries.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use serde_json::{Value, json};

use crate::resource_description_rules::{
    accept_resource_description_identifier, accept_resource_description_language,
    accept_resource_description_unicode, same_resource_description_term,
};
use crate::{LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType, read_structured_data_lino};

/// An editable resource description graph and its explicit semantic root.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceDescriptionLinks {
    /// Terms, statements, ordered occurrences, archived graphs and witnesses.
    pub network: LinkNetwork,
    /// The versioned graph root.
    pub root: LinkId,
}

fn schema() -> &'static Value {
    static SCHEMA: OnceLock<Value> = OnceLock::new();
    SCHEMA.get_or_init(|| {
        read_structured_data_lino(include_str!("data/resource-description-schema.lino"))
            .expect("the packaged resource description schema is valid")
    })
}

fn fail(message: &str) -> String {
    format!("resource description graph: {message}")
}

fn units(text: &str) -> Vec<f64> {
    text.encode_utf16().map(f64::from).collect()
}

fn bounded_text(value: &Value) -> Result<&str, String> {
    let text = value
        .as_str()
        .ok_or_else(|| fail("a bounded Unicode scalar string is required"))?;
    let code_units = units(text);
    if code_units.len() > 65_536 || !accept_resource_description_unicode(&code_units) {
        return Err(fail("a bounded Unicode scalar string is required"));
    }
    Ok(text)
}

fn identifier(text: &str) -> Result<&str, String> {
    if !accept_resource_description_identifier(&units(text)) {
        return Err(fail(
            "an absolute identifier is malformed or uses an unsupported bracketed address",
        ));
    }
    Ok(text)
}

fn normalized_term(input: &Value, source: Option<&str>) -> Result<Value, String> {
    let record = input
        .as_object()
        .ok_or_else(|| fail("a term must be a data record"))?;
    let kind = input["kind"].as_str().unwrap_or("");
    let fields: &[&str] = match kind {
        "identifier" => &["kind", "value"],
        "blank" => &["kind", "value", "sourceIdentifier"],
        "literal" => &["kind", "value", "datatype", "language"],
        _ => return Err(fail("a term has an unknown field")),
    };
    if record.keys().any(|field| !fields.contains(&field.as_str())) {
        return Err(fail("a term has an unknown field"));
    }
    let lexical = bounded_text(&input["value"])?;
    if kind == "identifier" {
        return Ok(json!({"kind":kind, "value":identifier(lexical)?}));
    }
    if kind == "blank" {
        let default = source.map_or(Value::Null, |text| Value::String(text.to_owned()));
        let scope = bounded_text(
            input
                .get("sourceIdentifier")
                .filter(|value| !value.is_null())
                .unwrap_or(&default),
        )?;
        if lexical.is_empty() || scope.is_empty() {
            return Err(fail(
                "blank node identity needs a label and nonempty explicit source scope",
            ));
        }
        return Ok(json!({"kind":kind, "value":lexical, "sourceIdentifier":scope}));
    }
    let language = input
        .get("language")
        .map(bounded_text)
        .transpose()?
        .unwrap_or("")
        .to_lowercase();
    let datatype = input
        .get("datatype")
        .filter(|value| !value.is_null())
        .map(bounded_text)
        .transpose()?
        .unwrap_or_else(|| {
            schema()[if language.is_empty() {
                "stringDatatype"
            } else {
                "languageDatatype"
            }]
            .as_str()
            .unwrap()
        });
    identifier(datatype)?;
    if language.is_empty() {
        if datatype == schema()["languageDatatype"] {
            return Err(fail("a language string requires a nonempty language tag"));
        }
    } else if datatype != schema()["languageDatatype"]
        || !accept_resource_description_language(
            &units(&language),
            schema()["grandfatheredLanguages"]
                .as_array()
                .unwrap()
                .iter()
                .any(|value| value == &language),
        )
    {
        return Err(fail("a language tag or its datatype is invalid"));
    }
    Ok(json!({"kind":kind, "value":lexical, "datatype":datatype, "language":language}))
}

fn identity_parts(term: &Value) -> Vec<String> {
    let fields: &[&str] = match term["kind"].as_str().unwrap() {
        "identifier" => &["value"],
        "blank" => &["sourceIdentifier", "value"],
        _ => &["value", "datatype", "language"],
    };
    fields
        .iter()
        .map(|field| term[field].as_str().unwrap().to_owned())
        .collect()
}

fn framed(text: &str) -> String {
    format!("{}:{text}", text.encode_utf16().count())
}

fn identity(term: &Value) -> String {
    std::iter::once(term["kind"].as_str().unwrap().to_owned())
        .chain(identity_parts(term))
        .map(|part| framed(&part))
        .collect()
}

fn statement_identity(triple: &[Value]) -> String {
    triple.iter().map(|term| framed(&identity(term))).collect()
}

fn same(left: &Value, right: &Value) -> bool {
    same_resource_description_term(
        left["kind"].as_str().unwrap(),
        &identity_parts(left),
        right["kind"].as_str().unwrap(),
        &identity_parts(right),
    )
}

fn check_positions(triple: &[Value]) -> Result<(), String> {
    for (index, role) in ["subject", "predicate", "object"].iter().enumerate() {
        if !schema()["positions"][role]
            .as_array()
            .unwrap()
            .contains(&triple[index]["kind"])
        {
            return Err(fail("a term cannot occupy its statement position"));
        }
    }
    Ok(())
}

struct Writer {
    network: LinkNetwork,
    terms: BTreeMap<String, LinkId>,
    statements: BTreeMap<Vec<u16>, LinkId>,
    occurrences: Vec<LinkId>,
    sources: Vec<LinkId>,
    witnesses: Vec<LinkId>,
    model: LinkId,
}

impl Writer {
    fn new() -> Result<Self, String> {
        let mut output = Self {
            network: LinkNetwork::new(),
            terms: BTreeMap::new(),
            statements: BTreeMap::new(),
            occurrences: Vec::new(),
            sources: Vec::new(),
            witnesses: Vec::new(),
            model: LinkId::from_u64(0),
        };
        let premise = output.text(schema()["specification"].as_str().unwrap())?;
        output.model = output.node("model", &[premise], schema()["model"].as_str())?;
        Ok(output)
    }

    fn append(&mut self, references: &[LinkId], data: LinkMetadata) -> Result<LinkId, String> {
        if self.network.len() >= 100_000 {
            return Err(fail("the output structural node limit is exceeded"));
        }
        Ok(self.network.insert_dynamic_link(references, data))
    }

    fn node(
        &mut self,
        kind: &str,
        references: &[LinkId],
        definition: Option<&str>,
    ) -> Result<LinkId, String> {
        let mut data = LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_named(true)
            .with_language(schema()["language"].as_str().unwrap())
            .with_term(schema()["terms"][kind].as_str().unwrap());
        if let Some(text) = definition {
            data = data.with_definition(text);
        }
        self.append(references, data)
    }

    fn text(&mut self, text: &str) -> Result<LinkId, String> {
        self.node("text", &[], Some(text))
    }

    fn term(&mut self, term: &Value) -> Result<LinkId, String> {
        let signature = identity(term);
        if let Some(id) = self.terms.get(&signature) {
            return Ok(*id);
        }
        let kind = term["kind"].as_str().unwrap();
        let references = match kind {
            "identifier" => vec![self.text(term["value"].as_str().unwrap())?],
            "blank" => vec![
                self.text(term["sourceIdentifier"].as_str().unwrap())?,
                self.text(term["value"].as_str().unwrap())?,
            ],
            _ => vec![
                self.text(term["value"].as_str().unwrap())?,
                self.term(&json!({"kind":"identifier","value":term["datatype"]}))?,
                self.text(term["language"].as_str().unwrap())?,
            ],
        };
        let id = self.node(kind, &references, None)?;
        self.terms.insert(signature, id);
        Ok(id)
    }

    fn statement(&mut self, triple: &[Value]) -> Result<LinkId, String> {
        let signature: Vec<u16> = statement_identity(triple).encode_utf16().collect();
        if let Some(id) = self.statements.get(&signature) {
            return Ok(*id);
        }
        let references = triple
            .iter()
            .map(|term| self.term(term))
            .collect::<Result<Vec<_>, _>>()?;
        let id = self.node("statement", &references, None)?;
        self.statements.insert(signature, id);
        Ok(id)
    }

    fn occurrence(
        &mut self,
        triple: &[Value],
        source: &str,
        position: u64,
        language: &str,
    ) -> Result<(), String> {
        let statement = self.statement(triple)?;
        let scope = self.text(source)?;
        let position = self.node("position", &[], Some(&position.to_string()))?;
        let language = self.text(language)?;
        let id = self.node("occurrence", &[statement, scope, position, language], None)?;
        self.occurrences.push(id);
        Ok(())
    }

    fn finish(mut self) -> Result<ResourceDescriptionLinks, String> {
        let ids = self.statements.values().copied().collect::<Vec<_>>();
        let statements = self.node("sequence", &ids, None)?;
        let occurrences = self.node("sequence", &self.occurrences.clone(), None)?;
        let sources = self.node("sequence", &self.sources.clone(), None)?;
        let witnesses = self.node("sequence", &self.witnesses.clone(), None)?;
        let root = self.node(
            "root",
            &[self.model, statements, occurrences, sources, witnesses],
            Some("1"),
        )?;
        Ok(ResourceDescriptionLinks {
            network: self.network,
            root,
        })
    }
}

/// Import bounded structured triples, retaining exact lexical terms and scoped blank nodes.
///
/// # Errors
/// Rejects malformed terms, unsupported lexical forms and invalid statement roles.
pub fn import_resource_description_graph(
    triples: &Value,
    source_identifier: &str,
) -> Result<ResourceDescriptionLinks, String> {
    bounded_text(&Value::String(source_identifier.to_owned()))?;
    let statements = triples
        .as_array()
        .ok_or_else(|| fail("a bounded graph is required"))?;
    if source_identifier.is_empty() || statements.len() > 10_000 {
        return Err(fail(
            "a bounded graph and a nonempty source identifier are required",
        ));
    }
    let mut output = Writer::new()?;
    for (position, input) in statements.iter().enumerate() {
        let fields = input
            .as_array()
            .filter(|values| values.len() == 3)
            .ok_or_else(|| fail("a statement needs exactly three terms"))?;
        let triple = fields
            .iter()
            .map(|term| normalized_term(term, Some(source_identifier)))
            .collect::<Result<Vec<_>, _>>()?;
        check_positions(&triple)?;
        let language = if input[2]["kind"] == "literal" {
            input[2]
                .get("language")
                .map(bounded_text)
                .transpose()?
                .unwrap_or("")
        } else {
            ""
        };
        output.occurrence(
            &triple,
            source_identifier,
            u64::try_from(position).unwrap(),
            language,
        )?;
    }
    let result = output.finish()?;
    decode_resource_description_graph(&result.network, result.root)?;
    Ok(result)
}

#[derive(Clone)]
struct Graph {
    triples: Vec<Vec<Value>>,
    occurrences: Vec<Value>,
    statement_ids: Vec<LinkId>,
}

struct Reader<'a> {
    network: &'a LinkNetwork,
    node_limit: usize,
    visited: BTreeSet<LinkId>,
    active: BTreeSet<LinkId>,
    fields: BTreeSet<LinkId>,
    terms: BTreeMap<LinkId, Value>,
    roots: BTreeMap<LinkId, Graph>,
}

impl<'a> Reader<'a> {
    fn new(network: &'a LinkNetwork, node_limit: usize) -> Self {
        Self {
            network,
            node_limit,
            visited: BTreeSet::new(),
            active: BTreeSet::new(),
            terms: BTreeMap::new(),
            roots: BTreeMap::new(),
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
    ) -> Result<(Vec<LinkId>, Option<String>), String> {
        let link = self
            .network
            .link(id)
            .ok_or_else(|| fail("a reference is dangling"))?;
        self.visited.insert(id);
        if self.visited.len() > self.node_limit {
            return Err(fail("the structural node limit is exceeded"));
        }
        let data = link.metadata();
        if data.link_type() != Some(LinkType::Semantic)
            || !data.is_named()
            || data.language() != schema()["language"].as_str()
            || data.term() != schema()["terms"][kind].as_str()
        {
            return Err(fail("a semantic node has a foreign role or model"));
        }
        if data.span().is_some() || data.flags() != LinkFlags::clean() {
            return Err(fail(
                "a semantic node carries unsupported concrete metadata",
            ));
        }
        if self.fields.contains(&id) {
            return Err(fail(
                "positional semantic nodes cannot conceal named fields",
            ));
        }
        if arity.is_some_and(|arity| link.references().len() != arity) {
            return Err(fail("a semantic node has the wrong arity"));
        }
        if !["text", "position", "model", "root"].contains(&kind) && data.definition().is_some() {
            return Err(fail("a semantic node has unexpected scalar data"));
        }
        Ok((
            link.references().to_vec(),
            data.definition().map(str::to_owned),
        ))
    }

    fn text(&mut self, id: LinkId) -> Result<String, String> {
        let (_, definition) = self.node(id, "text", Some(0))?;
        let value = Value::String(definition.ok_or_else(|| fail("a linked text value is absent"))?);
        Ok(bounded_text(&value)?.to_owned())
    }

    fn sequence(&mut self, id: LinkId) -> Result<Vec<LinkId>, String> {
        Ok(self.node(id, "sequence", None)?.0)
    }

    fn model(&mut self, id: LinkId) -> Result<(), String> {
        let (references, definition) = self.node(id, "model", Some(1))?;
        if definition.as_deref() != schema()["model"].as_str()
            || self.text(references[0])? != schema()["specification"]
        {
            return Err(fail(
                "the selected term equality premises are not supported",
            ));
        }
        Ok(())
    }

    fn term(&mut self, id: LinkId) -> Result<Value, String> {
        if let Some(term) = self.terms.get(&id) {
            return Ok(term.clone());
        }
        if !self.active.insert(id) {
            return Err(fail("a term has a structural cycle"));
        }
        let label = self
            .network
            .link(id)
            .and_then(|link| link.metadata().term());
        let kind = ["identifier", "blank", "literal"]
            .into_iter()
            .find(|kind| label == schema()["terms"][kind].as_str())
            .ok_or_else(|| fail("a statement reference is not a resource term"))?;
        let fields = self
            .node(
                id,
                kind,
                Some(match kind {
                    "identifier" => 1,
                    "blank" => 2,
                    _ => 3,
                }),
            )?
            .0;
        let input = match kind {
            "identifier" => json!({"kind":kind,"value":self.text(fields[0])?}),
            "blank" => {
                json!({"kind":kind,"sourceIdentifier":self.text(fields[0])?,"value":self.text(fields[1])?})
            }
            _ => {
                self.node(fields[1], "identifier", Some(1))?;
                let datatype = self.term(fields[1])?;
                json!({"kind":kind,"value":self.text(fields[0])?,"datatype":datatype["value"],"language":self.text(fields[2])?})
            }
        };
        let normalized = normalized_term(&input, None)?;
        if kind == "literal" && normalized["language"] != input["language"] {
            return Err(fail("a semantic language tag must be canonical lowercase"));
        }
        self.terms.insert(id, normalized.clone());
        self.active.remove(&id);
        Ok(normalized)
    }

    fn statement(&mut self, id: LinkId) -> Result<Vec<Value>, String> {
        let triple = self
            .node(id, "statement", Some(3))?
            .0
            .into_iter()
            .map(|id| self.term(id))
            .collect::<Result<Vec<_>, _>>()?;
        check_positions(&triple)?;
        Ok(triple)
    }

    fn graph(&mut self, root: LinkId, depth: usize) -> Result<Graph, String> {
        if depth > 64 {
            return Err(fail("the source graph nesting limit is exceeded"));
        }
        if let Some(graph) = self.roots.get(&root) {
            return Ok(graph.clone());
        }
        if !self.active.insert(root) {
            return Err(fail("source graph provenance is cyclic"));
        }
        let (fields, revision) = self.node(root, "root", Some(5))?;
        if revision.as_deref() != Some("1") {
            return Err(fail("the graph schema revision is unsupported"));
        }
        self.model(fields[0])?;
        let statement_ids = self.sequence(fields[1])?;
        let mut signatures = BTreeSet::new();
        let mut triples = Vec::new();
        for id in &statement_ids {
            let triple = self.statement(*id)?;
            if !signatures.insert(statement_identity(&triple)) {
                return Err(fail("the graph statement set contains a duplicate"));
            }
            triples.push(triple);
        }
        let mut occurrences = Vec::new();
        for id in self.sequence(fields[2])? {
            let references = self.node(id, "occurrence", Some(4))?.0;
            let index = statement_ids
                .iter()
                .position(|id| *id == references[0])
                .ok_or_else(|| fail("an occurrence refers outside its graph statement set"))?;
            let source = self.text(references[1])?;
            let (_, position_text) = self.node(references[2], "position", Some(0))?;
            let position_text =
                position_text.ok_or_else(|| fail("an occurrence source position is invalid"))?;
            let position = position_text
                .parse::<u64>()
                .map_err(|_| fail("an occurrence source position is invalid"))?;
            if position > 9_007_199_254_740_991 || position.to_string() != position_text {
                return Err(fail("an occurrence source position is invalid"));
            }
            let language = self.text(references[3])?;
            let semantic_language = if triples[index][2]["kind"] == "literal" {
                triples[index][2]["language"].as_str().unwrap()
            } else {
                ""
            };
            if source.is_empty() || language.to_lowercase() != semantic_language {
                return Err(fail(
                    "an occurrence source or language provenance is inconsistent",
                ));
            }
            occurrences.push(json!({"statementIndex":index,"sourceIdentifier":source,"position":position,"sourceLanguage":language}));
        }
        let sources = self.sequence(fields[3])?;
        for id in &sources {
            self.graph(*id, depth + 1)?;
        }
        let mut owned = BTreeSet::new();
        let mut pending = sources;
        while let Some(id) = pending.pop() {
            if owned.contains(&id) {
                continue;
            }
            if owned.len() >= self.node_limit {
                return Err(fail("the provenance traversal limit is exceeded"));
            }
            owned.insert(id);
            let link = self
                .network
                .link(id)
                .ok_or_else(|| fail("a source provenance reference is dangling"))?;
            pending.extend_from_slice(link.references());
        }
        let target_terms = statement_ids
            .iter()
            .flat_map(|id| self.network.link(*id).unwrap().references().iter().copied())
            .collect::<BTreeSet<_>>();
        for id in self.sequence(fields[4])? {
            let references = self.node(id, "witness", Some(3))?.0;
            self.model(references[2])?;
            if !owned.contains(&references[0])
                || !target_terms.contains(&references[1])
                || !same(&self.term(references[0])?, &self.term(references[1])?)
            {
                return Err(fail(
                    "a term equality witness has false or unowned premises",
                ));
            }
        }
        self.active.remove(&root);
        let result = Graph {
            triples,
            occurrences,
            statement_ids,
        };
        self.roots.insert(root, result.clone());
        Ok(result)
    }
}

/// Decode graph semantics without the original structured terms or source text.
///
/// # Errors
/// Rejects malformed links, counterfeit witnesses and unsupported models.
pub fn decode_resource_description_graph(
    network: &LinkNetwork,
    root: LinkId,
) -> Result<Value, String> {
    decode_resource_description_graph_with_limit(network, root, 100_000)
}

/// Decode with a caller-selected positive structural node limit.
///
/// # Errors
/// Rejects nonpositive limits and graphs exceeding their declared bounds.
pub fn decode_resource_description_graph_with_limit(
    network: &LinkNetwork,
    root: LinkId,
    node_limit: usize,
) -> Result<Value, String> {
    if node_limit == 0
        || !u64::try_from(node_limit).is_ok_and(|limit| limit <= 9_007_199_254_740_991)
    {
        return Err(fail("a positive structural limit is required"));
    }
    let graph = Reader::new(network, node_limit).graph(root, 0)?;
    Ok(json!({"model":schema()["model"],"triples":graph.triples,"occurrences":graph.occurrences}))
}

/// Merge exact term-equal statements while archiving source graphs and witnesses.
///
/// # Errors
/// Rejects malformed inputs, unowned references and excessive source graphs.
pub fn merge_resource_description_graphs(
    inputs: &[ResourceDescriptionLinks],
) -> Result<ResourceDescriptionLinks, String> {
    if inputs.len() > 100 {
        return Err(fail("a bounded source graph list is required"));
    }
    let mut output = Writer::new()?;
    for input in inputs {
        let mut reader = Reader::new(&input.network, 100_000);
        let graph = reader.graph(input.root, 0)?;
        let mut copied = BTreeMap::new();
        let mut pending = vec![input.root];
        while let Some(id) = pending.pop() {
            if copied.contains_key(&id) {
                continue;
            }
            if copied.len() >= 100_000 {
                return Err(fail("the source copy limit is exceeded"));
            }
            let link = input
                .network
                .link(id)
                .ok_or_else(|| fail("a source reference is dangling"))?;
            copied.insert(id, output.append(&[], link.metadata().clone())?);
            pending.extend_from_slice(link.references());
        }
        for (before, after) in &copied {
            let references = input
                .network
                .link(*before)
                .unwrap()
                .references()
                .iter()
                .map(|id| copied[id])
                .collect::<Vec<_>>();
            output.network.set_references(*after, &references);
        }
        output.sources.push(copied[&input.root]);
        for occurrence in &graph.occurrences {
            let index = usize::try_from(occurrence["statementIndex"].as_u64().unwrap()).unwrap();
            output.occurrence(
                &graph.triples[index],
                occurrence["sourceIdentifier"].as_str().unwrap(),
                occurrence["position"].as_u64().unwrap(),
                occurrence["sourceLanguage"].as_str().unwrap(),
            )?;
        }
        for triple in &graph.triples {
            output.statement(triple)?;
        }
        let source_terms = graph
            .statement_ids
            .iter()
            .flat_map(|id| {
                input
                    .network
                    .link(*id)
                    .unwrap()
                    .references()
                    .iter()
                    .copied()
            })
            .collect::<BTreeSet<_>>();
        for before in source_terms {
            let target = output.term(&reader.term(before)?)?;
            let witness = output.node("witness", &[copied[&before], target, output.model], None)?;
            output.witnesses.push(witness);
        }
    }
    let result = output.finish()?;
    decode_resource_description_graph(&result.network, result.root)?;
    Ok(result)
}

/// Validate every archived term-equality witness under its explicit primitive model.
///
/// # Errors
/// Rejects false equality, foreign targets, missing premises and malformed graphs.
pub fn verify_resource_description_graph_witnesses(
    network: &LinkNetwork,
    root: LinkId,
) -> Result<bool, String> {
    decode_resource_description_graph(network, root)?;
    Ok(true)
}
