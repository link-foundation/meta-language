//! Semantic invariant mining over inferred grammars.
//!
//! This module is a clean-room, deterministic implementation of the D8
//! inference layer. It keeps the public model grammar-aware and query-shaped,
//! while the current backend mines observations from the positive corpus until a
//! public derivation-tree parser is available from the CFG runtime.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

mod observation;

use observation::{Observation, ObservedValue};

use crate::grammar::Grammar;
use crate::link_network::LinkType;
use crate::query::LinkQuery;
use crate::semantics::{ProbabilisticTruthValue, Probability, TruthValue};

const DEF_SLOT: &str = "def";
const USE_SLOT: &str = "use";
const LEFT_SLOT: &str = "left";
const RIGHT_SLOT: &str = "right";
const FIELD_SLOT: &str = "field";
const BODY_SLOT: &str = "body";
const TARGET_SLOT: &str = "target";

const DEF_BEFORE_USE_SLOTS: &[&str] = &[DEF_SLOT, USE_SLOT];
const EQUAL_COUNT_SLOTS: &[&str] = &[LEFT_SLOT, RIGHT_SLOT];
const LENGTH_FIELD_SLOTS: &[&str] = &[FIELD_SLOT, BODY_SLOT];
const SINGLE_TARGET_SLOTS: &[&str] = &[TARGET_SLOT];

/// A grammar-aware atomic predicate over derivation-tree observations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstraintAtom {
    /// Every use must match a previously seen definition.
    DefBeforeUse {
        /// Non-terminal that contributes definitions.
        def: NonTerminalRef,
        /// Non-terminal that contributes uses.
        use_: NonTerminalRef,
    },
    /// The number of left and right matches must be equal.
    EqualCount {
        /// Left-hand non-terminal.
        left: NonTerminalRef,
        /// Right-hand non-terminal.
        right: NonTerminalRef,
    },
    /// A numeric field must equal the measured length of a body.
    LengthField {
        /// Numeric length field.
        field: NonTerminalRef,
        /// Body whose length is measured.
        body: NonTerminalRef,
        /// Unit used for measuring the body.
        unit: LengthUnit,
    },
    /// Values matched at the target must be pairwise distinct.
    Unique {
        /// Non-terminal whose values must be distinct.
        target: NonTerminalRef,
    },
    /// Values matched at the target must be non-decreasing in document order.
    Ordered {
        /// Non-terminal whose values must be ordered.
        target: NonTerminalRef,
    },
}

/// A reference to a non-terminal plus the structural query that locates it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NonTerminalRef {
    /// Rule name in the inferred grammar.
    pub rule: String,
    /// Query used by derivation-tree backends to extract instances.
    pub query: LinkQuery,
}

impl NonTerminalRef {
    /// Builds a reference whose query is keyed by the rule term.
    #[must_use]
    pub fn new(rule: impl Into<String>) -> Self {
        let rule = rule.into();
        Self {
            query: LinkQuery::by_type(LinkType::Grammar).with_term(rule.clone()),
            rule,
        }
    }

    /// Builds a reference with an explicit structural query.
    #[must_use]
    pub fn with_query(rule: impl Into<String>, query: LinkQuery) -> Self {
        Self {
            rule: rule.into(),
            query,
        }
    }
}

/// Unit used by [`ConstraintAtom::LengthField`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LengthUnit {
    /// Count comma-separated elements, falling back to characters for scalar
    /// bodies.
    Elements,
    /// Count UTF-8 bytes.
    Bytes,
    /// Count Unicode scalar values.
    Chars,
}

/// One conjunctive clause in a disjunctive-normal-form constraint.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ConstraintClause {
    /// Atoms that must all hold.
    pub atoms: Vec<ConstraintAtom>,
}

impl ConstraintClause {
    /// Builds a clause from atoms.
    #[must_use]
    pub const fn new(atoms: Vec<ConstraintAtom>) -> Self {
        Self { atoms }
    }

    /// Evaluates the clause over one input.
    #[must_use]
    pub fn evaluate(&self, grammar: &Grammar, input: &str) -> TruthValue {
        let observation = Observation::from_grammar(grammar, input);
        evaluate_clause_observation(&observation, self)
    }
}

/// A ranked disjunctive-normal-form semantic constraint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemanticConstraint {
    /// Disjunction of clauses. An empty list is treated as trivially true.
    pub clauses: Vec<ConstraintClause>,
    /// Deterministic ranking score; larger values are more specific.
    pub specificity: u32,
    /// Fraction of the positive corpus satisfied by the DNF.
    pub recall: Probability,
}

impl SemanticConstraint {
    /// Builds a constraint value.
    #[must_use]
    pub const fn new(
        clauses: Vec<ConstraintClause>,
        specificity: u32,
        recall: Probability,
    ) -> Self {
        Self {
            clauses,
            specificity,
            recall,
        }
    }

    /// Builds the trivially true constraint.
    #[must_use]
    pub const fn trivially_true() -> Self {
        Self {
            clauses: Vec::new(),
            specificity: 0,
            recall: Probability::ONE,
        }
    }

    /// Evaluates the DNF over one input.
    #[must_use]
    pub fn evaluate(&self, grammar: &Grammar, input: &str) -> TruthValue {
        evaluate_constraint(grammar, input, self)
    }

    /// Evaluates the DNF as a probabilistic truth value.
    #[must_use]
    pub fn evaluate_probabilistic(
        &self,
        grammar: &Grammar,
        input: &str,
    ) -> ProbabilisticTruthValue {
        evaluate_probabilistic(grammar, input, self)
    }
}

/// Function type used by a constraint-pattern template.
pub type ConstraintInstantiator = fn(&BTreeMap<&'static str, String>) -> Vec<ConstraintAtom>;

/// A reusable invariant template with named non-terminal slots.
#[derive(Clone, Copy)]
pub struct ConstraintPattern {
    /// Stable template name.
    pub name: &'static str,
    /// Slot names the instantiator binds to grammar rules.
    pub slots: &'static [&'static str],
    /// Builds concrete atoms from a slot binding.
    pub instantiate: ConstraintInstantiator,
}

impl fmt::Debug for ConstraintPattern {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ConstraintPattern")
            .field("name", &self.name)
            .field("slots", &self.slots)
            .finish_non_exhaustive()
    }
}

/// Configuration for semantic invariant mining.
#[derive(Clone, Debug)]
pub struct SemanticInferenceConfig {
    /// Pattern catalog to instantiate.
    pub catalog: Vec<ConstraintPattern>,
    /// Maximum grammar path depth considered by augmentation.
    pub k_path_depth: usize,
    /// Maximum number of augmented mutants to consider.
    pub max_augmented: usize,
    /// Minimum positive-corpus recall for a clause.
    pub min_recall: Probability,
}

impl Default for SemanticInferenceConfig {
    fn default() -> Self {
        Self {
            catalog: default_pattern_catalog(),
            k_path_depth: 3,
            max_augmented: 128,
            min_recall: Probability::ONE,
        }
    }
}

/// Returns the built-in semantic-invariant pattern catalog.
#[must_use]
pub fn default_pattern_catalog() -> Vec<ConstraintPattern> {
    vec![
        ConstraintPattern {
            name: "def-before-use",
            slots: DEF_BEFORE_USE_SLOTS,
            instantiate: instantiate_def_before_use,
        },
        ConstraintPattern {
            name: "equal-count",
            slots: EQUAL_COUNT_SLOTS,
            instantiate: instantiate_equal_count,
        },
        ConstraintPattern {
            name: "length-field",
            slots: LENGTH_FIELD_SLOTS,
            instantiate: instantiate_length_field,
        },
        ConstraintPattern {
            name: "unique",
            slots: SINGLE_TARGET_SLOTS,
            instantiate: instantiate_unique,
        },
        ConstraintPattern {
            name: "ordered",
            slots: SINGLE_TARGET_SLOTS,
            instantiate: instantiate_ordered,
        },
    ]
}

/// Mines a ranked DNF of semantic constraints from a grammar and positive corpus.
#[must_use]
pub fn mine_semantic_constraints(
    grammar: &Grammar,
    positive_examples: &[String],
    config: &SemanticInferenceConfig,
) -> SemanticConstraint {
    if grammar.rules().is_empty() || positive_examples.is_empty() || config.catalog.is_empty() {
        return SemanticConstraint::trivially_true();
    }

    let observations = positive_examples
        .iter()
        .map(|example| Observation::from_grammar(grammar, example))
        .collect::<Vec<_>>();
    let candidates = instantiate_candidates(grammar, &config.catalog);
    let mut surviving = candidates
        .into_iter()
        .filter(|atom| holds_on_positive_corpus(&observations, atom))
        .filter(|atom| discriminates_augmented_variant(&observations, atom, config))
        .collect::<Vec<_>>();

    surviving.sort_by_key(atom_sort_key);
    surviving.dedup();
    build_semantic_constraint(surviving, &observations, config.min_recall)
}

/// Evaluates one atom over one input.
#[must_use]
pub fn evaluate_atom(grammar: &Grammar, input: &str, atom: &ConstraintAtom) -> TruthValue {
    let observation = Observation::from_grammar(grammar, input);
    evaluate_atom_observation(&observation, atom)
}

/// Evaluates one clause over one input.
#[must_use]
pub fn evaluate_clause(grammar: &Grammar, input: &str, clause: &ConstraintClause) -> TruthValue {
    let observation = Observation::from_grammar(grammar, input);
    evaluate_clause_observation(&observation, clause)
}

/// Evaluates a semantic DNF over one input.
#[must_use]
pub fn evaluate_constraint(
    grammar: &Grammar,
    input: &str,
    constraint: &SemanticConstraint,
) -> TruthValue {
    if constraint.clauses.is_empty() {
        return TruthValue::True;
    }

    let observation = Observation::from_grammar(grammar, input);
    constraint
        .clauses
        .iter()
        .map(|clause| evaluate_clause_observation(&observation, clause))
        .fold(TruthValue::False, TruthValue::or)
}

/// Evaluates a semantic DNF as a probabilistic truth value.
#[must_use]
pub fn evaluate_probabilistic(
    grammar: &Grammar,
    input: &str,
    constraint: &SemanticConstraint,
) -> ProbabilisticTruthValue {
    ProbabilisticTruthValue::new(probability_for_truth(evaluate_constraint(
        grammar, input, constraint,
    )))
}

fn instantiate_def_before_use(bindings: &BTreeMap<&'static str, String>) -> Vec<ConstraintAtom> {
    let Some(def) = bindings.get(DEF_SLOT) else {
        return Vec::new();
    };
    let Some(use_) = bindings.get(USE_SLOT) else {
        return Vec::new();
    };
    vec![ConstraintAtom::DefBeforeUse {
        def: NonTerminalRef::new(def),
        use_: NonTerminalRef::new(use_),
    }]
}

fn instantiate_equal_count(bindings: &BTreeMap<&'static str, String>) -> Vec<ConstraintAtom> {
    let Some(left) = bindings.get(LEFT_SLOT) else {
        return Vec::new();
    };
    let Some(right) = bindings.get(RIGHT_SLOT) else {
        return Vec::new();
    };
    vec![ConstraintAtom::EqualCount {
        left: NonTerminalRef::new(left),
        right: NonTerminalRef::new(right),
    }]
}

fn instantiate_length_field(bindings: &BTreeMap<&'static str, String>) -> Vec<ConstraintAtom> {
    let Some(field) = bindings.get(FIELD_SLOT) else {
        return Vec::new();
    };
    let Some(body) = bindings.get(BODY_SLOT) else {
        return Vec::new();
    };
    vec![ConstraintAtom::LengthField {
        field: NonTerminalRef::new(field),
        body: NonTerminalRef::new(body),
        unit: LengthUnit::Bytes,
    }]
}

fn instantiate_unique(bindings: &BTreeMap<&'static str, String>) -> Vec<ConstraintAtom> {
    let Some(target) = bindings.get(TARGET_SLOT) else {
        return Vec::new();
    };
    vec![ConstraintAtom::Unique {
        target: NonTerminalRef::new(target),
    }]
}

fn instantiate_ordered(bindings: &BTreeMap<&'static str, String>) -> Vec<ConstraintAtom> {
    let Some(target) = bindings.get(TARGET_SLOT) else {
        return Vec::new();
    };
    vec![ConstraintAtom::Ordered {
        target: NonTerminalRef::new(target),
    }]
}

fn instantiate_candidates(grammar: &Grammar, catalog: &[ConstraintPattern]) -> Vec<ConstraintAtom> {
    let rule_names = grammar
        .rule_names()
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let mut atoms = Vec::new();

    for pattern in catalog {
        enumerate_bindings(pattern, &rule_names, 0, &mut BTreeMap::new(), &mut atoms);
    }

    atoms.sort_by_key(atom_sort_key);
    atoms.dedup();
    atoms
}

fn enumerate_bindings(
    pattern: &ConstraintPattern,
    rule_names: &[String],
    slot_index: usize,
    bindings: &mut BTreeMap<&'static str, String>,
    atoms: &mut Vec<ConstraintAtom>,
) {
    if slot_index == pattern.slots.len() {
        if binding_is_compatible(pattern.name, bindings) {
            atoms.extend((pattern.instantiate)(bindings));
        }
        return;
    }

    let slot = pattern.slots[slot_index];
    for rule_name in rule_names {
        bindings.insert(slot, rule_name.clone());
        enumerate_bindings(pattern, rule_names, slot_index + 1, bindings, atoms);
    }
    bindings.remove(slot);
}

fn binding_is_compatible(pattern_name: &str, bindings: &BTreeMap<&'static str, String>) -> bool {
    match pattern_name {
        "def-before-use" => {
            let Some(def) = bindings.get(DEF_SLOT) else {
                return false;
            };
            let Some(use_) = bindings.get(USE_SLOT) else {
                return false;
            };
            def != use_
                && has_any(def, &["def", "decl", "bind", "let", "var"])
                && has_any(use_, &["use", "ref", "call"])
        }
        "equal-count" => {
            let Some(left) = bindings.get(LEFT_SLOT) else {
                return false;
            };
            let Some(right) = bindings.get(RIGHT_SLOT) else {
                return false;
            };
            left != right
                && (has_pair(left, right, "open", "close")
                    || has_pair(left, right, "left", "right")
                    || has_pair(left, right, "start", "end")
                    || has_pair(left, right, "begin", "end"))
        }
        "length-field" => {
            let Some(field) = bindings.get(FIELD_SLOT) else {
                return false;
            };
            let Some(body) = bindings.get(BODY_SLOT) else {
                return false;
            };
            field != body
                && has_any(field, &["len", "length", "size", "count", "field"])
                && has_any(body, &["body", "payload", "data", "content"])
        }
        "unique" => bindings
            .get(TARGET_SLOT)
            .is_some_and(|target| has_any(target, &["id", "name", "symbol", "item", "key"])),
        "ordered" => bindings
            .get(TARGET_SLOT)
            .is_some_and(|target| has_any(target, &["number", "index", "order", "rank", "seq"])),
        _ => true,
    }
}

fn has_pair(left: &str, right: &str, left_marker: &str, right_marker: &str) -> bool {
    has_any(left, &[left_marker]) && has_any(right, &[right_marker])
}

fn has_any(value: &str, needles: &[&str]) -> bool {
    let lower = value.to_ascii_lowercase();
    needles.iter().any(|needle| lower.contains(needle))
}

fn holds_on_positive_corpus(observations: &[Observation], atom: &ConstraintAtom) -> bool {
    let mut saw_true = false;
    for observation in observations {
        match evaluate_atom_observation(observation, atom) {
            TruthValue::True => saw_true = true,
            TruthValue::Unknown => {}
            TruthValue::False | TruthValue::Both => return false,
        }
    }
    saw_true
}

fn discriminates_augmented_variant(
    observations: &[Observation],
    atom: &ConstraintAtom,
    config: &SemanticInferenceConfig,
) -> bool {
    if config.k_path_depth == 0 || config.max_augmented == 0 {
        return false;
    }

    observations
        .iter()
        .any(|observation| atom_has_discriminating_mutation(observation, atom))
}

fn atom_has_discriminating_mutation(observation: &Observation, atom: &ConstraintAtom) -> bool {
    match atom {
        ConstraintAtom::DefBeforeUse { def, use_ } => {
            !observation.values(def).is_empty() && !observation.values(use_).is_empty()
        }
        ConstraintAtom::EqualCount { left, right } => {
            let left_count = observation.values(left).len();
            let right_count = observation.values(right).len();
            left_count > 0 && left_count == right_count
        }
        ConstraintAtom::LengthField { field, body, .. } => {
            !observation.values(field).is_empty() && !observation.values(body).is_empty()
        }
        ConstraintAtom::Unique { target } => observation.values(target).len() > 1,
        ConstraintAtom::Ordered { target } => {
            let values = observation.values(target);
            values.len() > 1
                && values.windows(2).any(|pair| {
                    comparable_value(&pair[0].value) != comparable_value(&pair[1].value)
                })
        }
    }
}

fn build_semantic_constraint(
    atoms: Vec<ConstraintAtom>,
    observations: &[Observation],
    min_recall: Probability,
) -> SemanticConstraint {
    if atoms.is_empty() {
        return SemanticConstraint::trivially_true();
    }

    let mut groups = BTreeMap::<Vec<u8>, Vec<ConstraintAtom>>::new();
    for atom in atoms {
        let signature = observations
            .iter()
            .map(|observation| truth_signature(evaluate_atom_observation(observation, &atom)))
            .collect::<Vec<_>>();
        groups.entry(signature).or_default().push(atom);
    }

    let mut clauses = groups
        .into_values()
        .map(|mut clause_atoms| {
            clause_atoms.sort_by_key(atom_sort_key);
            ConstraintClause::new(clause_atoms)
        })
        .filter(|clause| clause_recall(clause, observations) >= min_recall)
        .collect::<Vec<_>>();

    clauses.sort_by(|left, right| {
        clause_specificity(right, observations)
            .cmp(&clause_specificity(left, observations))
            .then_with(|| clause_sort_key(left).cmp(&clause_sort_key(right)))
    });

    let specificity = clauses
        .iter()
        .map(|clause| clause_specificity(clause, observations))
        .fold(0_u32, u32::saturating_add);
    let recall = constraint_recall(&clauses, observations);

    SemanticConstraint::new(clauses, specificity, recall)
}

const fn truth_signature(value: TruthValue) -> u8 {
    match value {
        TruthValue::Both => 0,
        TruthValue::False => 1,
        TruthValue::Unknown => 2,
        TruthValue::True => 3,
    }
}

fn clause_recall(clause: &ConstraintClause, observations: &[Observation]) -> Probability {
    let satisfied = observations
        .iter()
        .filter(|observation| truth_is_satisfied(evaluate_clause_observation(observation, clause)))
        .count();
    probability_from_counts(satisfied, observations.len())
}

fn constraint_recall(clauses: &[ConstraintClause], observations: &[Observation]) -> Probability {
    if observations.is_empty() {
        return Probability::ONE;
    }
    if clauses.is_empty() {
        return Probability::ONE;
    }

    let satisfied = observations
        .iter()
        .filter(|observation| {
            let truth = clauses
                .iter()
                .map(|clause| evaluate_clause_observation(observation, clause))
                .fold(TruthValue::False, TruthValue::or);
            truth_is_satisfied(truth)
        })
        .count();
    probability_from_counts(satisfied, observations.len())
}

fn probability_from_counts(numerator: usize, denominator: usize) -> Probability {
    if denominator == 0 {
        return Probability::ONE;
    }
    let numerator = u64::try_from(numerator).unwrap_or(u64::MAX);
    let denominator = u64::try_from(denominator).unwrap_or(u64::MAX);
    Probability::from_ratio(numerator, denominator).unwrap_or(Probability::ZERO)
}

const fn truth_is_satisfied(value: TruthValue) -> bool {
    matches!(value, TruthValue::True | TruthValue::Unknown)
}

fn evaluate_clause_observation(observation: &Observation, clause: &ConstraintClause) -> TruthValue {
    clause
        .atoms
        .iter()
        .map(|atom| evaluate_atom_observation(observation, atom))
        .fold(TruthValue::True, TruthValue::and)
}

fn evaluate_atom_observation(observation: &Observation, atom: &ConstraintAtom) -> TruthValue {
    match atom {
        ConstraintAtom::DefBeforeUse { def, use_ } => {
            evaluate_def_before_use(observation.values(def), observation.values(use_))
        }
        ConstraintAtom::EqualCount { left, right } => {
            evaluate_equal_count(observation.values(left), observation.values(right))
        }
        ConstraintAtom::LengthField { field, body, unit } => {
            evaluate_length_field(observation.values(field), observation.values(body), *unit)
        }
        ConstraintAtom::Unique { target } => evaluate_unique(observation.values(target)),
        ConstraintAtom::Ordered { target } => evaluate_ordered(observation.values(target)),
    }
}

fn evaluate_def_before_use(defs: &[ObservedValue], uses: &[ObservedValue]) -> TruthValue {
    if defs.is_empty() || uses.is_empty() {
        return TruthValue::Unknown;
    }

    let mut defs = defs.to_vec();
    defs.sort_by_key(|value| value.position);
    let mut uses = uses.to_vec();
    uses.sort_by_key(|value| value.position);

    let mut def_index = 0;
    let mut seen = BTreeSet::new();
    for use_value in uses {
        while def_index < defs.len() && defs[def_index].position < use_value.position {
            seen.insert(defs[def_index].value.as_str());
            def_index += 1;
        }
        if !seen.contains(use_value.value.as_str()) {
            return TruthValue::False;
        }
    }

    TruthValue::True
}

const fn evaluate_equal_count(left: &[ObservedValue], right: &[ObservedValue]) -> TruthValue {
    if left.is_empty() || right.is_empty() {
        return TruthValue::Unknown;
    }
    if left.len() == right.len() {
        TruthValue::True
    } else {
        TruthValue::False
    }
}

fn evaluate_length_field(
    fields: &[ObservedValue],
    bodies: &[ObservedValue],
    unit: LengthUnit,
) -> TruthValue {
    if fields.is_empty() || bodies.is_empty() {
        return TruthValue::Unknown;
    }

    for field in fields {
        let Ok(expected) = field.value.parse::<usize>() else {
            return TruthValue::Unknown;
        };
        let Some(body) = nearest_following_body(field.position, bodies).or_else(|| bodies.first())
        else {
            return TruthValue::Unknown;
        };
        if measure_body(&body.value, unit) != expected {
            return TruthValue::False;
        }
    }

    TruthValue::True
}

fn nearest_following_body(position: usize, bodies: &[ObservedValue]) -> Option<&ObservedValue> {
    bodies
        .iter()
        .filter(|body| body.position > position)
        .min_by_key(|body| body.position)
}

fn measure_body(body: &str, unit: LengthUnit) -> usize {
    match unit {
        LengthUnit::Elements => {
            let elements = body
                .split(',')
                .filter(|element| !element.trim().is_empty())
                .count();
            if elements > 1 {
                elements
            } else {
                body.chars().count()
            }
        }
        LengthUnit::Bytes => body.len(),
        LengthUnit::Chars => body.chars().count(),
    }
}

fn evaluate_unique(values: &[ObservedValue]) -> TruthValue {
    if values.is_empty() {
        return TruthValue::Unknown;
    }
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(value.value.as_str()) {
            return TruthValue::False;
        }
    }
    TruthValue::True
}

fn evaluate_ordered(values: &[ObservedValue]) -> TruthValue {
    if values.is_empty() {
        return TruthValue::Unknown;
    }

    let mut ordered = values.to_vec();
    ordered.sort_by_key(|value| value.position);
    if ordered
        .windows(2)
        .all(|pair| comparable_value(&pair[0].value) <= comparable_value(&pair[1].value))
    {
        TruthValue::True
    } else {
        TruthValue::False
    }
}

fn comparable_value(value: &str) -> ComparableValue<'_> {
    value
        .parse::<i64>()
        .map_or(ComparableValue::Text(value), ComparableValue::Number)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ComparableValue<'a> {
    Number(i64),
    Text(&'a str),
}

fn probability_for_truth(value: TruthValue) -> Probability {
    match value {
        TruthValue::True => Probability::ONE,
        TruthValue::False => Probability::ZERO,
        TruthValue::Unknown | TruthValue::Both => {
            Probability::from_basis_points(5_000).expect("5,000 basis points is in range")
        }
    }
}

fn atom_sort_key(atom: &ConstraintAtom) -> String {
    format!("{atom:?}")
}

fn clause_sort_key(clause: &ConstraintClause) -> String {
    clause
        .atoms
        .iter()
        .map(atom_sort_key)
        .collect::<Vec<_>>()
        .join("\n")
}

fn clause_specificity(clause: &ConstraintClause, observations: &[Observation]) -> u32 {
    clause
        .atoms
        .iter()
        .map(|atom| atom_specificity(atom, observations))
        .fold(0_u32, u32::saturating_add)
}

fn atom_specificity(atom: &ConstraintAtom, observations: &[Observation]) -> u32 {
    let rules = atom_rules(atom);
    let touched = observations
        .iter()
        .map(|observation| {
            rules
                .iter()
                .map(|rule| {
                    observation
                        .values_by_rule
                        .get(rule.as_str())
                        .map_or(0, Vec::len)
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    let touched = u32::try_from(touched).unwrap_or(u32::MAX);
    touched.saturating_add(1)
}

fn atom_rules(atom: &ConstraintAtom) -> Vec<&String> {
    match atom {
        ConstraintAtom::DefBeforeUse { def, use_ } => vec![&def.rule, &use_.rule],
        ConstraintAtom::EqualCount { left, right } => vec![&left.rule, &right.rule],
        ConstraintAtom::LengthField { field, body, .. } => vec![&field.rule, &body.rule],
        ConstraintAtom::Unique { target } | ConstraintAtom::Ordered { target } => {
            vec![&target.rule]
        }
    }
}
