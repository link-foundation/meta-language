//! Native host transport for the shared grammar-selection decisions. Grammar
//! matching itself stays in the existing executor; the generated JavaScript
//! policies choose admitted mappings, witness counts and language ambiguity.

use std::cell::RefCell;
use std::rc::Rc;

use serde_json::{Value, json};

use super::executor::{Executor, Run};
use super::program::{Expr, Name, SelectionDescriptor};
use super::results::{
    Expectations, KeywordLexing, Outcome, Res, Shared, TokenOrder, Tree, TreeType, no_children,
    with_leaf,
};
use super::selection_source_mapping::mapped_cover;
use super::tree::{SyntaxTree, public_tree};
use crate::grammar::interchange::serialize_grammar;
use crate::grammar::{Grammar, default_settling};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::grammar_selection_field_range_rules::{
    accept_grammar_selection_field_range_candidate, read_grammar_selection_field_range_context,
};
use crate::translation::grammar_selection_included_range_rules::{
    accept_grammar_selection_included_range_agreement,
    accept_grammar_selection_included_range_candidate,
    read_grammar_selection_included_range_context,
};
use crate::translation::grammar_selection_mapping_rules::read_grammar_selection_mapping_context;
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
    /// The explicit source-mapping profile selected by the grammar.
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
    /// Actual source mapping and ordered interval references, only on mapped selections.
    pub source_mapping: Option<Value>,
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
    pub(super) source_mapping: Option<Value>,
    pub(super) source_bytes: Option<Rc<[u8]>>,
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
    let bytes = selection.source_bytes.as_deref().unwrap_or(bytes);
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
        source_mapping: selection.source_mapping.clone(),
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

pub(super) fn mapping_rejection(
    diagnostics: &[Value],
    bytes: &[u8],
) -> Option<super::ParseRejection> {
    let primary = diagnostics
        .iter()
        .max_by_key(|diagnostic| diagnostic["start"].as_u64().unwrap_or_default())?;
    let reason = match primary["reason"].as_str()? {
        "mappedSyntax" => "syntax",
        "invalidEmbeddingMapping" => "invalidEmbeddingMapping",
        _ => "unsupportedEmbeddingMapping",
    };
    let offset = primary["start"]
        .as_u64()
        .and_then(|value| usize::try_from(value).ok())?;
    let mut rejection = super::ParseRejection::positioned(reason, bytes, offset);
    rejection.diagnostics = Some(diagnostics.to_vec());
    rejection.offset_semantics = Some("covering-source-spelling".to_owned());
    Some(rejection)
}

impl Executor<'_> {
    pub(super) fn report_selection_diagnostic(&self, diagnostic: Value) -> Run<()> {
        let diagnostic = self
            .shared
            .selection_sources
            .borrow()
            .qualify(self.bytes, diagnostic);
        let duplicate = self
            .shared
            .selection_diagnostics
            .borrow()
            .iter()
            .any(|item| {
                ["reason", "language", "start", "end", "source"]
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

    /// The same actual source/program/bounds cannot be entered while active.
    pub(super) fn run_embedded_grammar(
        &self,
        program: usize,
        start: usize,
        end: usize,
        language: &Name,
    ) -> Run<Option<Rc<Outcome>>> {
        self.run_embedded_source(program, (start, end), language, self.bytes)
    }

    fn run_embedded_source(
        &self,
        program: usize,
        range: (usize, usize),
        language: &Name,
        bytes: &[u8],
    ) -> Run<Option<Rc<Outcome>>> {
        let (start, end) = range;
        let key = (program, bytes.as_ptr().addr(), start, end);
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
        let result = (|| {
            if std::ptr::eq(bytes, self.bytes) {
                let mut nested = Executor::new(
                    self.compiled,
                    program,
                    bytes,
                    start,
                    end,
                    self.shared,
                    self.max_depth.saturating_sub(self.depth),
                );
                return nested.run(start_rule);
            }
            // Only scanner offset context is local to this new source plane.
            // Step/frame/memory counters and active source guards stay shared.
            let local = Shared {
                runtime_error: self.shared.runtime_error.clone(),
                global_concept_space: self.shared.global_concept_space.clone(),
                selection_diagnostics: self.shared.selection_diagnostics.clone(),
                active_embedded: self.shared.active_embedded.clone(),
                selection_sources: self.shared.selection_sources.clone(),
                selection_mapping_failures: self.shared.selection_mapping_failures.clone(),
                steps: self.shared.steps.clone(),
                limit: self.shared.limit,
                frames: self.shared.frames.clone(),
                memo_limit: self.shared.memo_limit,
                cells: self.shared.cells.clone(),
                lexical_caches: self.shared.lexical_caches.clone(),
                memory_limit: self.shared.memory_limit,
                expectations: RefCell::new(Expectations::default()),
            };
            let tokens = nested_program
                .token_ranks
                .as_ref()
                .filter(|_| {
                    nested_program
                        .settling
                        .has(super::program::SettlingStep::Tokens)
                })
                .map(|ranks| TokenOrder {
                    ranks,
                    bytes,
                    orders: &nested_program.precedence_orders,
                    grammar: &nested_program.grammar,
                    trivia: &nested_program.trivia,
                });
            let keywords = tokens.map(|_| RefCell::new(KeywordLexing::default()));
            loop {
                local.expectations.borrow_mut().restart();
                let mut nested = Executor::new(
                    self.compiled,
                    program,
                    bytes,
                    start,
                    end,
                    &local,
                    self.max_depth.saturating_sub(self.depth),
                );
                nested.keywords = keywords.as_ref();
                let outcome = nested.run(start_rule)?;
                if local.expectations.borrow().stale {
                    continue;
                }
                if let (Outcome::Parsed(root), Some(keywords), Some(tokens)) =
                    (&outcome, &keywords, tokens)
                    && keywords
                        .borrow_mut()
                        .conflicts(root, tokens, &nested_program.rules)
                {
                    continue;
                }
                return Ok(outcome);
            }
        })();
        self.shared.active_embedded.borrow_mut().remove(&key);
        result.map(|outcome| Some(Rc::new(outcome)))
    }

    fn remember_mapping_failure(&self, diagnostic: Value) -> Run<()> {
        let diagnostic = self
            .shared
            .selection_sources
            .borrow_mut()
            .remember(self.bytes, diagnostic, self)?;
        self.shared
            .selection_mapping_failures
            .borrow_mut()
            .push(diagnostic);
        Ok(())
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
        let mappings = descriptor
            .candidates
            .iter()
            .map(|candidate| candidate.mapping.to_string())
            .collect::<Vec<_>>();
        let field_context = number_index(read_grammar_selection_field_range_context(
            &mappings, in_token,
        ));
        let skipped = self.terminal_start(position, state, in_token || field_context == Some(1))?;
        let start = skipped.end;
        let range_context = if field_context == Some(0) {
            number_index(read_grammar_selection_included_range_context(
                &mappings, in_token,
            ))
        } else {
            Some(0)
        };
        let context = if field_context != Some(0) {
            field_context
        } else if range_context == Some(0) {
            number_index(read_grammar_selection_mapping_context(&mappings, in_token))
        } else {
            range_context
        };
        if context == Some(2) {
            self.report_selection_diagnostic(json!({"reason":"unsupportedEmbeddingMapping","start":start,"end":start,
                "mapping":if field_context == Some(2) {"field-ranges"} else if range_context == Some(2) {"included-ranges"} else {"json-string"},"detail":"mapped selection requires syntactic context and one common source mapping"}))?;
            return Ok(Vec::new());
        }
        let fielded = field_context == Some(1);
        let mut selector: Option<&Value> = None;
        if fielded {
            for candidate in &descriptor.candidates {
                if !accept_grammar_selection_field_range_candidate(
                    &candidate.kind,
                    &candidate.mapping,
                ) {
                    self.report_selection_diagnostic(json!({"reason":"unsupportedEmbeddingMapping","start":start,"end":start,
                        "mapping":"field-ranges","detail":"field ranges require an explicit grammar-field-candidate"}))?;
                    return Ok(Vec::new());
                }
                self.step()?;
                self.retain(5)?;
                // Feature integer transport has one zero representation.
                // These are the original validated primitive selector fields.
                let parameters = candidate
                    .selector
                    .as_ref()
                    .expect("a validated field candidate retains its selector");
                if selector.is_some_and(|first| {
                    ["kind", "label", "mode", "first", "count"]
                        .iter()
                        .any(|key| first[*key] != parameters[*key])
                }) {
                    self.report_selection_diagnostic(json!({"reason":"unsupportedEmbeddingMapping","start":start,"end":start,
                        "mapping":"field-ranges","detail":"field range candidates require one common selector"}))?;
                    return Ok(Vec::new());
                }
                if selector.is_none() {
                    selector = Some(parameters);
                }
            }
        }
        let has_included_ranges = range_context == Some(1);
        let mut ranges: Option<&[[f64; 2]]> = None;
        if has_included_ranges {
            if descriptor.candidates.iter().any(|candidate| {
                !accept_grammar_selection_included_range_candidate(
                    &candidate.kind,
                    &candidate.mapping,
                )
            }) {
                self.report_selection_diagnostic(json!({"reason":"unsupportedEmbeddingMapping","start":start,"end":start,
                    "mapping":"included-ranges","detail":"included ranges require an explicit grammar-range-candidate"}))?;
                return Ok(Vec::new());
            }
            for candidate in &descriptor.candidates {
                self.retain(candidate.ranges.len().saturating_mul(2).saturating_add(1))?;
                let mut parameters = Vec::new();
                for pair in &candidate.ranges {
                    self.step()?;
                    parameters.extend_from_slice(pair);
                }
                if let Some(first) = ranges {
                    let first = first.iter().flatten().copied().collect::<Vec<_>>();
                    if !accept_grammar_selection_included_range_agreement(&first, &parameters) {
                        self.report_selection_diagnostic(json!({"reason":"unsupportedEmbeddingMapping","start":start,"end":start,
                            "mapping":"included-ranges","detail":"included range candidates require one common ordered range plan"}))?;
                        return Ok(Vec::new());
                    }
                } else {
                    ranges = Some(&candidate.ranges);
                }
            }
        }
        let mapped = context == Some(1);
        let mapping_profile = if fielded {
            "field-ranges"
        } else if has_included_ranges {
            "included-ranges"
        } else {
            "json-string"
        };
        let regions = self.evaluate(item, start, &skipped.state, !mapped)?;
        let mut results = Vec::new();
        for region in regions {
            let derived = if mapped {
                let result = if let Some(selector) = selector {
                    self.shared.selection_sources.borrow_mut().map_field(
                        self.bytes,
                        (start, region.end),
                        selector,
                        &region.children,
                        self,
                        self.max_depth.saturating_sub(self.depth),
                    )?
                } else if let Some(ranges) = ranges {
                    self.shared
                        .selection_sources
                        .borrow_mut()
                        .map_included(self.bytes, start, region.end, ranges, self)?
                } else {
                    self.shared
                        .selection_sources
                        .borrow_mut()
                        .map(self.bytes, start, region.end, self)?
                };
                match result {
                    Ok(derived) => Some(derived),
                    Err(failed) => {
                        self.remember_mapping_failure(
                            json!({"reason":failed.reason,"start":failed.offset,"end":failed.offset,
                            "mapping":mapping_profile}),
                        )?;
                        self.fail(
                            failed.offset,
                            &Name::from(if fielded {
                                "a scalar-aligned field-selected source range view"
                            } else if has_included_ranges {
                                "an ordered scalar-aligned source range view"
                            } else {
                                "a Unicode scalar JSON string source"
                            }),
                        );
                        continue;
                    }
                }
            } else {
                None
            };
            let mut accepted = Vec::new();
            let mut candidates = Vec::new();
            for candidate in &descriptor.candidates {
                let language = &candidate.name;
                let mapping = &candidate.mapping;
                self.step()?;
                self.retain(1)?;
                if !fielded && !has_included_ranges && !accept_grammar_selection_mapping(mapping) {
                    self.report_selection_diagnostic(json!({"language":&**language,"start":start,"end":region.end,"reason":"unsupportedEmbeddingMapping","mapping":&**mapping}))?;
                    candidates.push(json!({"language":&**language,"relation":"unknown","reason":"unsupportedEmbeddingMapping","mapping":&**mapping}));
                    continue;
                }
                let Some(&program) = self.compiled.languages.get(&**language) else {
                    self.report_selection_diagnostic(json!({"language":&**language,"start":start,"end":region.end,"reason":"unknownGrammar"}))?;
                    candidates.push(json!({"language":&**language,"relation":"unknown","reason":"unknownGrammar"}));
                    continue;
                };
                let mut parameters = vec![
                    json!("selection"),
                    json!(&**mapping),
                    json!(&**language),
                    json!(start),
                    json!(region.end),
                ];
                if let Some(selector) = selector {
                    parameters.push(selector.clone());
                } else if has_included_ranges {
                    parameters.push(json!(ranges));
                }
                if let Some(derived) = &derived {
                    parameters.push(derived.mapping["output"].clone());
                }
                let key = (
                    Name::from(Value::Array(parameters).to_string()),
                    start,
                    region.end,
                );
                let outcome = if let Some(outcome) = self.embed_memo.get(&key) {
                    Some(outcome.clone())
                } else {
                    let outcome = if let Some(derived) = &derived {
                        self.run_embedded_source(
                            program,
                            (0, derived.bytes.len()),
                            language,
                            &derived.bytes,
                        )?
                    } else {
                        self.run_embedded_grammar(program, start, region.end, language)?
                    };
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
                        let offset = if let Some(derived) = &derived {
                            let Some(cover) = mapped_cover(&derived.mapping, *farthest, *farthest)
                            else {
                                self.report_selection_diagnostic(json!({"reason":"unsupportedEmbeddingMapping","start":start,"end":region.end,
                                    "mapping":mapping_profile}))?;
                                continue;
                            };
                            candidates.push(json!({"language":&**language,"relation":"rejected","offset":cover[0],"expected":expected,
                                "source":derived.mapping["output"],"sourceOffset":farthest,"physicalRange":[cover[1],cover[2]],"offsetSemantics":"covering-source-spelling"}));
                            self.remember_mapping_failure(json!({"reason":"mappedSyntax","language":&**language,"start":cover[1],"end":cover[2],
                                "source":derived.mapping["output"],"sourceStart":farthest,"sourceEnd":farthest,"expected":expected,"offsetSemantics":"covering-source-spelling",
                                "sourceRoute":[{"output":derived.mapping["output"],"outputStart":farthest,"outputEnd":farthest,"input":derived.mapping["input"],"inputStart":cover[1],"inputEnd":cover[2],"displayAnchor":cover[0]}]}))?;
                            cover[0]
                        } else {
                            candidates.push(json!({"language":&**language,"relation":"rejected","offset":farthest,"expected":expected}));
                            *farthest
                        };
                        for expectation in expected {
                            self.fail(offset, &Name::from(expectation.as_str()));
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
                mapping: if mapped {
                    descriptor.candidates[0].mapping.clone()
                } else {
                    Name::from("identity")
                },
                start,
                end: region.end,
                candidates,
                provenance: primary.provenance,
                alternatives: accepted,
                ambiguous,
                source_mapping: derived.as_ref().map(|source| source.mapping.clone()),
                source_bytes: derived.as_ref().map(|source| source.bytes.clone()),
            }));
            if mapped {
                child.source_children = Some(region.children.clone());
            }
            let children = if in_token {
                no_children()
            } else {
                with_leaf(&skipped.leaves, child)
            };
            let mut result = Res::new(region.end, region.state, children, region.dynamic);
            result.ambiguous = ambiguous || mapped && region.ambiguous;
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
            source_mapping: None,
            source_bytes: None,
        }));
        Ok(Rc::new(child))
    }
}
