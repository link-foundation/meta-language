//! Native host transport for the shared grammar-selection decisions. Grammar
//! matching itself stays in the existing executor; the generated JavaScript
//! policies choose admitted mappings, witness counts and language ambiguity.

use std::rc::Rc;

use serde_json::{Value, json};

use super::executor::{Executor, Run};
use super::program::{Expr, Name, SelectionDescriptor};
use super::results::{Outcome, Res, Tree, TreeType, no_children, with_leaf};
use super::tree::{SyntaxTree, public_tree};
use crate::grammar::interchange::serialize_grammar;
use crate::grammar::{Grammar, default_settling};
use crate::translation::grammar_selection_rules::{
    accept_grammar_selection_count, accept_grammar_selection_mapping,
    read_grammar_source_default_matching, report_grammar_selection_ambiguity,
};

/// One independently accepted concrete interpretation of a selected region.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarSelectionAlternative {
    /// The explicit source-qualified grammar name.
    pub language: String,
    /// Its complete concrete tree on the original absolute byte range.
    pub root: Box<SyntaxTree>,
    /// Source, edition, rule aliases and checked global origin membership.
    pub provenance: Value,
}

/// Inspectable selection metadata; parallel witnesses do not own source twice.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarSelection {
    /// Explicit any/all policy, or the qualified root invocation.
    pub mode: String,
    /// The declared source mapping; currently only identity is executable.
    pub mapping: String,
    /// First original source byte.
    pub start: usize,
    /// Original source byte after the region.
    pub end: usize,
    /// Every candidate's accepted, rejected or unknown outcome.
    pub candidates: Vec<Value>,
    /// The primary grammar's source-qualified origin membership.
    pub provenance: Value,
    /// Every other accepted concrete interpretation.
    pub alternatives: Vec<GrammarSelectionAlternative>,
    /// Whether the explicit any policy admitted more than one grammar.
    pub ambiguous: bool,
}

#[derive(Clone, Debug)]
pub(super) struct AcceptedSelection {
    pub(super) language: Name,
    pub(super) root: Rc<Tree>,
    pub(super) program: usize,
    pub(super) provenance: Value,
}

#[derive(Clone, Debug)]
pub(super) struct SelectionMetadata {
    pub(super) mode: Name,
    pub(super) mapping: Name,
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) candidates: Vec<Value>,
    pub(super) provenance: Value,
    pub(super) alternatives: Vec<AcceptedSelection>,
    pub(super) ambiguous: bool,
}

/// Snapshot the actual grammar input, including metadata the ordinary JSON
/// interchange intentionally omits. This is data transport, not a proof.
pub(super) fn selection_source_definition(grammar: &Grammar) -> Value {
    let mut value: Value = serde_json::from_str(&serialize_grammar(grammar))
        .expect("the existing grammar serializer produces valid structured data");
    if let Some(rules) = value["rules"].as_array_mut() {
        for (definition, rule) in rules.iter_mut().zip(grammar.rules()) {
            if let Some(concept) = &rule.concept {
                definition["concept"] = json!(concept);
            }
            if !rule.source_names.is_empty() {
                definition["sourceNames"] = json!(
                    rule.source_names
                        .iter()
                        .map(|alias| json!({"source":alias.source,"name":alias.name}))
                        .collect::<Vec<_>>()
                );
            }
        }
    }
    value
}

pub(super) fn public_selection(selection: &SelectionMetadata, bytes: &[u8]) -> GrammarSelection {
    GrammarSelection {
        mode: selection.mode.to_string(),
        mapping: selection.mapping.to_string(),
        start: selection.start,
        end: selection.end,
        candidates: selection.candidates.clone(),
        provenance: selection.provenance.clone(),
        alternatives: selection
            .alternatives
            .iter()
            .map(|alternative| GrammarSelectionAlternative {
                language: alternative.language.to_string(),
                root: Box::new(public_tree(&alternative.root, bytes)),
                provenance: alternative.provenance.clone(),
            })
            .collect(),
        ambiguous: selection.ambiguous,
    }
}

pub(super) fn selection_rejection(
    diagnostics: &[Value],
    bytes: &[u8],
) -> Option<super::ParseRejection> {
    let first = diagnostics.first()?;
    let reason = match first["reason"].as_str()? {
        "nonConsumingEmbeddingCycle" => "nonConsumingEmbeddingCycle",
        "unsupportedEmbeddingMapping" => "unsupportedEmbeddingMapping",
        "unknownGrammar" => "unknownGrammar",
        "unprovedGrammarOrigin" => "unprovedGrammarOrigin",
        _ => "grammarSelection",
    };
    let offset = first["start"]
        .as_u64()
        .and_then(|value| usize::try_from(value).ok())
        .unwrap_or_default();
    let mut rejection = super::ParseRejection::positioned(reason, bytes, offset);
    rejection.diagnostics = Some(diagnostics.to_vec());
    Some(rejection)
}

impl Executor<'_> {
    pub(super) fn report_selection_diagnostic(&self, diagnostic: Value) -> Run<()> {
        let duplicate = self
            .shared
            .selection_diagnostics
            .borrow()
            .iter()
            .any(|item| {
                ["reason", "language", "start", "end"]
                    .iter()
                    .all(|key| item[key] == diagnostic[key])
            });
        if !duplicate {
            self.retain(1)?;
            self.shared
                .selection_diagnostics
                .borrow_mut()
                .push(diagnostic);
        }
        Ok(())
    }

    /// The same completed program and bounds cannot be entered while active.
    pub(super) fn run_embedded_grammar(
        &self,
        program: usize,
        start: usize,
        end: usize,
        language: &Name,
    ) -> Run<Option<Rc<Outcome>>> {
        let key = (program, start, end);
        if !self.shared.active_embedded.borrow_mut().insert(key) {
            self.report_selection_diagnostic(json!({"reason":"nonConsumingEmbeddingCycle","language":&**language,"start":start,"end":end}))?;
            return Ok(None);
        }
        let nested_program = &self.compiled.programs[program];
        let start_rule = nested_program
            .start
            .as_deref()
            .and_then(|name| nested_program.rule_index.get(name))
            .copied()
            .unwrap_or_default();
        let mut nested = Self::new(
            self.compiled,
            program,
            self.bytes,
            start,
            end,
            self.shared,
            self.max_depth.saturating_sub(self.depth),
        );
        let outcome = nested.run(start_rule);
        self.shared.active_embedded.borrow_mut().remove(&key);
        outcome.map(|outcome| Some(Rc::new(outcome)))
    }

    fn charge_selection_value(&self, value: &Value) -> Run<()> {
        self.step()?;
        match value {
            Value::Array(items) => {
                for item in items {
                    self.charge_selection_value(item)?;
                }
            }
            Value::Object(fields) => {
                for item in fields.values() {
                    self.charge_selection_value(item)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn read_selection_provenance(
        &self,
        program: usize,
        language: &str,
        start: usize,
        end: usize,
    ) -> Run<Value> {
        let program = &self.compiled.programs[program];
        self.retain(program.rules.len())?;
        let space = self.shared.global_concept_space.as_deref();
        let source = space
            .and_then(|space| space["sources"].as_array())
            .and_then(|sources| sources.iter().find(|source| source["id"] == language));
        let declarations = &program.source_definition["declarations"];
        let matching = declarations["matching"].as_str().unwrap_or(if program.peg {
            "peg"
        } else {
            "generalized"
        });
        let steps = declarations
            .get("settling")
            .filter(|value| !value.is_null())
            .cloned()
            .unwrap_or_else(|| json!(default_settling(matching)));
        let mut actual_context = json!({"matching":matching,"settling":steps,"imports":[],"modes":[],"extras":[],"conflicts":[],"precedences":[],"macros":[],"scanners":[]});
        if let Some(fields) = declarations.as_object() {
            for (key, value) in fields {
                actual_context[key] = value.clone();
            }
        }
        if actual_context["matching"].is_null() {
            actual_context["matching"] = json!(matching);
        }
        if actual_context["settling"].is_null() {
            actual_context["settling"] = steps;
        }
        let mut source_context = source.map_or(Value::Null, |source| source["context"].clone());
        if !source_context.is_null() {
            if source_context["matching"].is_null() {
                source_context["matching"] = json!(read_grammar_source_default_matching(
                    source
                        .and_then(|source| source["sourceFormat"].as_str())
                        .unwrap_or_default()
                ));
            }
            if source_context["settling"].is_null() {
                source_context["settling"] = json!(default_settling(
                    source_context["matching"].as_str().unwrap_or("generalized")
                ));
            }
        }
        let mut rules = Vec::new();
        for rule in &program.rules {
            let name = &*rule.name;
            let origin = format!("{language}:{name}");
            let original = program.source_definition["rules"]
                .as_array()
                .and_then(|rules| rules.iter().find(|rule| rule["name"] == name));
            let concept = space
                .and_then(|space| space["concepts"].as_array())
                .and_then(|concepts| {
                    concepts.iter().find(|concept| {
                        space.is_some_and(|space| concept["id"] == space["mappings"][&origin])
                    })
                });
            let stored = concept
                .and_then(|concept| concept["definitions"].as_array())
                .and_then(|definitions| {
                    definitions
                        .iter()
                        .find(|definition| definition["origin"] == origin)
                });
            let stored_origin = concept
                .and_then(|concept| concept["origins"].as_array())
                .and_then(|origins| origins.iter().find(|item| item["alias"] == origin));
            let mut definition = original.map_or(
                Value::Null,
                |original| json!({"kind":original["kind"],"expression":original["expression"]}),
            );
            if let Some(original) = original {
                for key in ["parameters", "channel", "modes", "action"] {
                    if let Some(value) = original.get(key) {
                        definition[key] = value.clone();
                    }
                }
            }
            let declared_concept = original.map_or(Value::Null, |rule| rule["concept"].clone());
            let mut matches = stored.is_some()
                && stored_origin.is_some_and(|stored| stored["concept"] == declared_concept);
            if matches {
                let stored_definition = &stored.expect("checked definition")["definition"];
                self.charge_selection_value(stored_definition)?;
                self.charge_selection_value(&definition)?;
                matches = stored_definition == &definition;
                if matches {
                    self.charge_selection_value(&source_context)?;
                    self.charge_selection_value(&actual_context)?;
                    matches = source_context == actual_context;
                }
            }
            if space.is_some() && !matches {
                self.report_selection_diagnostic(json!({"reason":"unprovedGrammarOrigin","language":language,"start":start,"end":end,"rule":name}))?;
            }
            rules.push(json!({"rule":name,"origin":origin,"concept":if matches {concept.map_or(Value::Null, |concept| concept["id"].clone())} else {Value::Null},
                "declaredConcept":original.map_or(Value::Null, |rule| rule["concept"].clone()),
                "sourceNames":original.and_then(|rule| rule.get("sourceNames")).cloned().unwrap_or_else(|| json!([]))}));
        }
        Ok(
            json!({"source":language,"language":source.map_or_else(|| json!(language), |source| source["language"].clone()),"edition":source.map_or_else(|| json!(""), |source| source["edition"].clone()),"rules":rules}),
        )
    }

    pub(super) fn evaluate_grammar_selection(
        &mut self,
        descriptor: &SelectionDescriptor,
        item: &Expr,
        position: usize,
        state: &super::operations::State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        let skipped = self.terminal_start(position, state, in_token)?;
        let start = skipped.end;
        let regions = self.evaluate(item, start, &skipped.state, true)?;
        let mut results = Vec::new();
        for region in regions {
            let mut accepted = Vec::new();
            let mut candidates = Vec::new();
            for (language, mapping) in &descriptor.candidates {
                self.step()?;
                self.retain(1)?;
                if !accept_grammar_selection_mapping(mapping) {
                    self.report_selection_diagnostic(json!({"reason":"unsupportedEmbeddingMapping","language":&**language,"start":start,"end":region.end,"mapping":&**mapping}))?;
                    candidates.push(json!({"language":&**language,"relation":"unknown","reason":"unsupportedEmbeddingMapping","mapping":&**mapping}));
                    continue;
                }
                let Some(&program) = self.compiled.languages.get(&**language) else {
                    self.report_selection_diagnostic(json!({"reason":"unknownGrammar","language":&**language,"start":start,"end":region.end}))?;
                    candidates.push(json!({"language":&**language,"relation":"unknown","reason":"unknownGrammar"}));
                    continue;
                };
                let key = (
                    Name::from(format!("selection|{language}")),
                    start,
                    region.end,
                );
                let outcome = if let Some(outcome) = self.embed_memo.get(&key) {
                    Some(outcome.clone())
                } else {
                    let outcome =
                        self.run_embedded_grammar(program, start, region.end, language)?;
                    if let Some(outcome) = &outcome {
                        self.embed_memo.insert(key, outcome.clone());
                    }
                    outcome
                };
                let Some(outcome) = outcome else {
                    candidates.push(json!({"language":&**language,"relation":"unknown","reason":"nonConsumingEmbeddingCycle"}));
                    continue;
                };
                match &*outcome {
                    Outcome::Failed {
                        farthest, expected, ..
                    } => {
                        candidates.push(json!({"language":&**language,"relation":"rejected","offset":farthest,"expected":expected}));
                        for expectation in expected {
                            self.fail(*farthest, &Name::from(expectation.as_str()));
                        }
                    }
                    Outcome::Parsed(root) => {
                        let provenance =
                            self.read_selection_provenance(program, language, start, region.end)?;
                        candidates.push(json!({"language":&**language,"relation":"accepted"}));
                        accepted.push(AcceptedSelection {
                            language: language.clone(),
                            root: root.clone(),
                            program,
                            provenance,
                        });
                    }
                }
            }
            let accepted_count = f64::from(u32::try_from(accepted.len()).unwrap_or(u32::MAX));
            let total = f64::from(u32::try_from(descriptor.candidates.len()).unwrap_or(u32::MAX));
            if !accept_grammar_selection_count(&descriptor.mode, accepted_count, total) {
                continue;
            }
            let primary = accepted.remove(0);
            let ambiguous = report_grammar_selection_ambiguity(&descriptor.mode, accepted_count);
            let mut child = Tree::new(TreeType::Embed, None, start, region.end);
            child.language = Some(primary.language);
            child.root = Some(primary.root);
            child.program = primary.program;
            child.selection = Some(Rc::new(SelectionMetadata {
                mode: descriptor.mode.clone(),
                mapping: Name::from("identity"),
                start,
                end: region.end,
                candidates,
                provenance: primary.provenance,
                alternatives: accepted,
                ambiguous,
            }));
            let children = if in_token {
                no_children()
            } else {
                with_leaf(&skipped.leaves, child)
            };
            let mut result = Res::new(region.end, region.state, children, region.dynamic);
            result.ambiguous = ambiguous;
            result.cost = region.cost;
            results.push(result);
        }
        Ok(results)
    }

    pub(super) fn qualify_grammar_root(&self, root: Rc<Tree>, language: &str) -> Run<Rc<Tree>> {
        let provenance =
            self.read_selection_provenance(self.program_index, language, self.begin, self.end)?;
        let mut child = Tree::new(TreeType::Embed, None, self.begin, self.end);
        child.language = Some(Name::from(language));
        child.root = Some(root);
        child.program = self.program_index;
        child.selection = Some(Rc::new(SelectionMetadata {
            mode: Name::from("grammar-root"),
            mapping: Name::from("identity"),
            start: self.begin,
            end: self.end,
            candidates: vec![json!({"language":language,"relation":"accepted"})],
            provenance,
            alternatives: Vec::new(),
            ambiguous: false,
        }));
        Ok(Rc::new(child))
    }
}
