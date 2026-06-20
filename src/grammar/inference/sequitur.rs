//! Sequitur structural compression for a single symbol sequence.
//!
//! The implementation follows the two invariants from Nevill-Manning and
//! Witten's Sequitur algorithm: every live digram is unique, and every generated
//! rule is referenced more than once. It uses an index-linked arena rather than
//! reference-counted cells so substitutions and inlining can update local list
//! boundaries directly. The digram table is a [`BTreeMap`] to keep emitted rule
//! names and conflict handling deterministic.

use std::collections::{BTreeMap, VecDeque};

use crate::grammar::{Grammar, GrammarExpr, GrammarFormat, GrammarRule};

/// Terminal symbol consumed by [`run_sequitur`].
pub type Symbol = String;

type NodeId = usize;
type RuleId = usize;

const START_RULE: RuleId = 0;

/// Runs Sequitur over `sequence` and emits the compressed hierarchy as grammar IR.
///
/// The result contains a `start` rule plus zero or more generated `R1`, `R2`, ...
/// rules. Terminals are emitted as [`GrammarExpr::Terminal`] values, generated
/// rule references as [`GrammarExpr::NonTerminal`], and the grammar source format
/// is set to [`GrammarFormat::Inferred`]. The grammar accepts exactly the
/// concatenation of the input symbols; later inference stages are responsible for
/// generalising beyond that single sequence.
#[must_use]
pub fn run_sequitur(sequence: &[Symbol]) -> Grammar {
    let mut builder = Sequitur::new();
    for symbol in sequence {
        builder.append(symbol.clone());
    }
    builder.finish()
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SymRef {
    Terminal(Symbol),
    Rule(RuleId),
}

#[derive(Clone, Debug)]
struct SymbolNode {
    value: SymRef,
    prev: Option<NodeId>,
    next: Option<NodeId>,
    rule: RuleId,
    alive: bool,
}

#[derive(Clone, Debug)]
struct RuleState {
    head: Option<NodeId>,
    tail: Option<NodeId>,
    active: bool,
    ref_count: usize,
}

impl RuleState {
    const fn active() -> Self {
        Self {
            head: None,
            tail: None,
            active: true,
            ref_count: 0,
        }
    }
}

#[derive(Debug)]
struct Sequitur {
    nodes: Vec<SymbolNode>,
    rules: Vec<RuleState>,
    digrams: BTreeMap<(SymRef, SymRef), NodeId>,
    digram_queue: VecDeque<NodeId>,
    utility_queue: VecDeque<RuleId>,
}

impl Sequitur {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            rules: vec![RuleState::active()],
            digrams: BTreeMap::new(),
            digram_queue: VecDeque::new(),
            utility_queue: VecDeque::new(),
        }
    }

    fn append(&mut self, terminal: Symbol) {
        let node = self.append_node(START_RULE, SymRef::Terminal(terminal));
        if let Some(prev) = self.nodes[node].prev {
            self.queue_digram(prev);
        }
        self.settle();
    }

    fn finish(mut self) -> Grammar {
        self.settle();

        let names = self.final_rule_names();
        let mut grammar = Grammar::new().with_source_format(GrammarFormat::Inferred);
        grammar.add_rule(GrammarRule::new(
            "start",
            GrammarExpr::Sequence(self.rule_exprs(START_RULE, &names)),
        ));

        for rule_id in 1..self.rules.len() {
            if self.rules[rule_id].active {
                let name = names
                    .get(&rule_id)
                    .expect("active generated rules must be named")
                    .clone();
                grammar.add_rule(GrammarRule::new(
                    name,
                    GrammarExpr::Sequence(self.rule_exprs(rule_id, &names)),
                ));
            }
        }

        grammar.set_start("start");
        grammar
    }

    fn settle(&mut self) {
        while !self.digram_queue.is_empty() || !self.utility_queue.is_empty() {
            if let Some(start) = self.digram_queue.pop_front() {
                self.enforce_digram_at(start);
            } else if let Some(rule_id) = self.utility_queue.pop_front() {
                self.enforce_rule_utility(rule_id);
            }
        }
    }

    fn append_node(&mut self, rule: RuleId, value: SymRef) -> NodeId {
        self.increment_ref_in_value(&value);

        let node = self.nodes.len();
        let prev = self.rules[rule].tail;
        self.nodes.push(SymbolNode {
            value,
            prev,
            next: None,
            rule,
            alive: true,
        });

        if let Some(prev) = prev {
            self.nodes[prev].next = Some(node);
        } else {
            self.rules[rule].head = Some(node);
        }
        self.rules[rule].tail = Some(node);

        node
    }

    fn insert_node_after(&mut self, previous: NodeId, value: SymRef) -> NodeId {
        self.increment_ref_in_value(&value);

        let node = self.nodes.len();
        let rule = self.nodes[previous].rule;
        let next = self.nodes[previous].next;
        self.nodes.push(SymbolNode {
            value,
            prev: Some(previous),
            next,
            rule,
            alive: true,
        });

        self.nodes[previous].next = Some(node);
        if let Some(next) = next {
            self.nodes[next].prev = Some(node);
        } else {
            self.rules[rule].tail = Some(node);
        }

        node
    }

    fn enforce_digram_at(&mut self, start: NodeId) {
        let Some(key) = self.digram_key(start) else {
            return;
        };

        let Some(existing) = self.digrams.get(&key).copied() else {
            self.digrams.insert(key, start);
            return;
        };

        if existing == start {
            return;
        }

        if self.digram_key(existing).as_ref() != Some(&key) {
            self.digrams.remove(&key);
            self.queue_digram(start);
            return;
        }

        if self.occurrences_overlap(existing, start) {
            return;
        }

        if let Some(rule_id) = self.rule_matching_body(existing) {
            self.replace_digram_with_rule(start, rule_id);
        } else {
            self.create_rule_for_duplicate(key, existing, start);
        }
    }

    fn create_rule_for_duplicate(
        &mut self,
        key: (SymRef, SymRef),
        existing: NodeId,
        current: NodeId,
    ) {
        if self.digram_key(existing).as_ref() != Some(&key)
            || self.digram_key(current).as_ref() != Some(&key)
        {
            self.queue_digram(current);
            return;
        }

        let rule_id = self.rules.len();
        self.rules.push(RuleState::active());
        let first = self.append_node(rule_id, key.0.clone());
        self.append_node(rule_id, key.1.clone());

        self.replace_digram_with_rule(existing, rule_id);
        if self.digram_key(current).as_ref() == Some(&key) {
            self.replace_digram_with_rule(current, rule_id);
        }

        if self.rules[rule_id].active {
            self.digrams.insert(key, first);
            if self.rules[rule_id].ref_count <= 1 {
                self.queue_utility(rule_id);
            }
        }
    }

    fn replace_digram_with_rule(&mut self, start: NodeId, rule_id: RuleId) {
        let Some(second) = self.nodes.get(start).and_then(|node| node.next) else {
            return;
        };
        if !self.nodes[start].alive
            || !self.nodes[second].alive
            || self.nodes[start].rule != self.nodes[second].rule
        {
            return;
        }

        let containing_rule = self.nodes[start].rule;
        let prev = self.nodes[start].prev;
        let next = self.nodes[second].next;

        if let Some(prev) = prev {
            self.unregister_digram_at(prev);
        }
        self.unregister_digram_at(start);
        self.unregister_digram_at(second);

        let first_value = self.nodes[start].value.clone();
        let second_value = self.nodes[second].value.clone();
        self.decrement_ref_in_value(&first_value);
        self.decrement_ref_in_value(&second_value);

        self.nodes[start].value = SymRef::Rule(rule_id);
        self.increment_ref(rule_id);
        self.nodes[start].next = next;

        if let Some(next) = next {
            self.nodes[next].prev = Some(start);
        } else {
            self.rules[containing_rule].tail = Some(start);
        }

        self.nodes[second].alive = false;
        self.nodes[second].prev = None;
        self.nodes[second].next = None;

        if self.rules[containing_rule].head == Some(second) {
            self.rules[containing_rule].head = Some(start);
        }
        if self.rules[containing_rule].tail == Some(second) {
            self.rules[containing_rule].tail = Some(start);
        }

        if let Some(prev) = prev {
            self.queue_digram(prev);
        }
        self.queue_digram(start);
    }

    fn enforce_rule_utility(&mut self, rule_id: RuleId) {
        if rule_id == START_RULE
            || !self
                .rules
                .get(rule_id)
                .is_some_and(|rule| rule.active && rule.ref_count <= 1)
        {
            return;
        }

        if self.rules[rule_id].ref_count == 0 {
            self.delete_rule(rule_id);
            return;
        }

        if let Some(reference) = self.find_reference(rule_id) {
            self.inline_rule_at(reference, rule_id);
        } else {
            self.rules[rule_id].ref_count = 0;
            self.delete_rule(rule_id);
        }
    }

    fn inline_rule_at(&mut self, reference: NodeId, rule_id: RuleId) {
        let body = self.rule_values(rule_id);
        if body.is_empty() {
            self.remove_node(reference);
            self.delete_rule(rule_id);
            return;
        }

        let prev = self.nodes[reference].prev;
        let old_next = self.nodes[reference].next;

        if let Some(prev) = prev {
            self.unregister_digram_at(prev);
        }
        self.unregister_digram_at(reference);

        let old_value = self.nodes[reference].value.clone();
        self.decrement_ref_in_value(&old_value);

        self.nodes[reference].value = body[0].clone();
        self.increment_ref_in_value(&body[0]);

        let mut last = reference;
        for value in body.into_iter().skip(1) {
            last = self.insert_node_after(last, value);
        }

        if let Some(prev) = prev {
            self.queue_digram(prev);
        }
        self.queue_inserted_digrams(reference, old_next);
        self.delete_rule(rule_id);
    }

    fn remove_node(&mut self, node: NodeId) {
        if !self.nodes[node].alive {
            return;
        }

        let rule = self.nodes[node].rule;
        let prev = self.nodes[node].prev;
        let next = self.nodes[node].next;

        if let Some(prev) = prev {
            self.unregister_digram_at(prev);
        }
        self.unregister_digram_at(node);

        let value = self.nodes[node].value.clone();
        self.decrement_ref_in_value(&value);

        if let Some(prev) = prev {
            self.nodes[prev].next = next;
        } else {
            self.rules[rule].head = next;
        }
        if let Some(next) = next {
            self.nodes[next].prev = prev;
        } else {
            self.rules[rule].tail = prev;
        }

        self.nodes[node].alive = false;
        self.nodes[node].prev = None;
        self.nodes[node].next = None;

        if let Some(prev) = prev {
            self.queue_digram(prev);
        }
    }

    fn delete_rule(&mut self, rule_id: RuleId) {
        if rule_id == START_RULE || !self.rules[rule_id].active {
            return;
        }

        let mut cursor = self.rules[rule_id].head;
        while let Some(node) = cursor {
            cursor = self.nodes[node].next;
            self.unregister_digram_at(node);
        }

        cursor = self.rules[rule_id].head;
        while let Some(node) = cursor {
            cursor = self.nodes[node].next;
            let value = self.nodes[node].value.clone();
            self.decrement_ref_in_value(&value);
            self.nodes[node].alive = false;
            self.nodes[node].prev = None;
            self.nodes[node].next = None;
        }

        self.rules[rule_id].head = None;
        self.rules[rule_id].tail = None;
        self.rules[rule_id].active = false;
        self.rules[rule_id].ref_count = 0;
    }

    fn find_reference(&self, rule_id: RuleId) -> Option<NodeId> {
        self.nodes.iter().enumerate().find_map(|(node_id, node)| {
            (node.alive && node.value == SymRef::Rule(rule_id)).then_some(node_id)
        })
    }

    fn queue_inserted_digrams(&mut self, first: NodeId, old_next: Option<NodeId>) {
        let mut cursor = Some(first);
        while let Some(node) = cursor {
            self.queue_digram(node);
            if self.nodes[node].next == old_next {
                break;
            }
            cursor = self.nodes[node].next;
        }
    }

    fn queue_digram(&mut self, start: NodeId) {
        if self.nodes.get(start).is_some_and(|node| node.alive) {
            self.digram_queue.push_back(start);
        }
    }

    fn queue_utility(&mut self, rule_id: RuleId) {
        if rule_id != START_RULE {
            self.utility_queue.push_back(rule_id);
        }
    }

    fn unregister_digram_at(&mut self, start: NodeId) {
        let Some(key) = self.digram_key(start) else {
            return;
        };
        if self.digrams.get(&key).copied() == Some(start) {
            self.digrams.remove(&key);
        }
    }

    fn digram_key(&self, start: NodeId) -> Option<(SymRef, SymRef)> {
        let node = self.nodes.get(start)?;
        if !node.alive {
            return None;
        }
        let next = node.next?;
        let next_node = self.nodes.get(next)?;
        if !next_node.alive || next_node.rule != node.rule {
            return None;
        }

        Some((node.value.clone(), next_node.value.clone()))
    }

    fn occurrences_overlap(&self, left: NodeId, right: NodeId) -> bool {
        left == right
            || self.nodes[left].next == Some(right)
            || self.nodes[right].next == Some(left)
    }

    fn rule_matching_body(&self, start: NodeId) -> Option<RuleId> {
        let node = self.nodes.get(start)?;
        let rule_id = node.rule;
        if rule_id == START_RULE {
            return None;
        }

        let next = node.next?;
        let rule = self.rules.get(rule_id)?;
        (rule.active && rule.head == Some(start) && rule.tail == Some(next)).then_some(rule_id)
    }

    fn increment_ref_in_value(&mut self, value: &SymRef) {
        if let SymRef::Rule(rule_id) = value {
            self.increment_ref(*rule_id);
        }
    }

    fn increment_ref(&mut self, rule_id: RuleId) {
        if let Some(rule) = self.rules.get_mut(rule_id) {
            rule.ref_count = rule.ref_count.saturating_add(1);
        }
    }

    fn decrement_ref_in_value(&mut self, value: &SymRef) {
        if let SymRef::Rule(rule_id) = value {
            self.decrement_ref(*rule_id);
        }
    }

    fn decrement_ref(&mut self, rule_id: RuleId) {
        if rule_id == START_RULE {
            return;
        }

        let should_queue = if let Some(rule) = self.rules.get_mut(rule_id) {
            rule.ref_count = rule.ref_count.saturating_sub(1);
            rule.active && rule.ref_count <= 1
        } else {
            false
        };

        if should_queue {
            self.queue_utility(rule_id);
        }
    }

    fn final_rule_names(&self) -> BTreeMap<RuleId, String> {
        let mut names = BTreeMap::from([(START_RULE, "start".to_string())]);
        let mut next = 1usize;

        for rule_id in 1..self.rules.len() {
            if self.rules[rule_id].active {
                names.insert(rule_id, format!("R{next}"));
                next += 1;
            }
        }

        names
    }

    fn rule_exprs(&self, rule_id: RuleId, names: &BTreeMap<RuleId, String>) -> Vec<GrammarExpr> {
        self.rule_values(rule_id)
            .into_iter()
            .map(|value| Self::symref_to_expr(value, names))
            .collect()
    }

    fn rule_values(&self, rule_id: RuleId) -> Vec<SymRef> {
        let mut values = Vec::new();
        let mut cursor = self.rules[rule_id].head;

        while let Some(node) = cursor {
            if self.nodes[node].alive {
                values.push(self.nodes[node].value.clone());
            }
            cursor = self.nodes[node].next;
        }

        values
    }

    fn symref_to_expr(value: SymRef, names: &BTreeMap<RuleId, String>) -> GrammarExpr {
        match value {
            SymRef::Terminal(value) => GrammarExpr::Terminal(value),
            SymRef::Rule(rule_id) => GrammarExpr::NonTerminal(
                names
                    .get(&rule_id)
                    .expect("referenced generated rules must be named")
                    .clone(),
            ),
        }
    }
}
