//! Lossless host graph projection for the shared lexical-plan policies.

use std::collections::{BTreeSet, HashMap, HashSet};

use super::lexical_kernel as kernel;
use super::load::{LoadResult, refuse};
use super::operations::Statement;
use super::program::Name;
use crate::grammar::{
    FeatureExpr, FeatureForm, Grammar, GrammarExpr, GrammarLexer, GrammarRule, Operation, RuleKind,
};

#[derive(Clone, Debug)]
pub(super) struct LexicalObligation {
    pub(super) name: String,
    pub(super) category: String,
    pub(super) source: String,
}

#[derive(Debug)]
pub(super) struct LexicalRule {
    pub(super) name: Name,
    pub(super) token_type: Name,
    pub(super) mode: Name,
    pub(super) channel: Name,
    pub(super) disposition: Name,
    pub(super) action: Vec<Statement>,
    pub(super) matcher: usize,
    pub(super) obligations: Vec<LexicalObligation>,
}

#[derive(Debug)]
pub(super) struct LexicalPlan {
    pub(super) name: Name,
    pub(super) rules: Vec<LexicalRule>,
    pub(super) bindings: Vec<(Name, Name)>,
    pub(super) literals: HashMap<Vec<u8>, Name>,
    pub(super) owners: HashMap<Name, Name>,
    pub(super) original_owners: HashSet<Name>,
    pub(super) obligations: Vec<LexicalObligation>,
    pub(super) program: usize,
}

pub(super) struct LexicalSource {
    pub(super) plan: LexicalPlan,
    pub(super) grammar: Grammar,
    pub(super) rules: Vec<FeatureForm>,
    pub(super) modes: HashSet<String>,
}

fn form(expr: &GrammarExpr) -> Option<&FeatureForm> {
    match expr {
        GrammarExpr::Feature(feature) => match &**feature {
            FeatureExpr::Form(form) => Some(form),
            _ => None,
        },
        _ => None,
    }
}

pub(super) fn kind(expr: &GrammarExpr) -> &str {
    match expr {
        GrammarExpr::Empty => "empty",
        GrammarExpr::AnyChar => "any",
        GrammarExpr::Terminal(_) => "literal",
        GrammarExpr::TerminalInsensitive(_) => "literalInsensitive",
        GrammarExpr::CharRange(..) => "charRange",
        GrammarExpr::CharClass { .. } => "charClass",
        GrammarExpr::NonTerminal(_) => "ref",
        GrammarExpr::Sequence(_) => "seq",
        GrammarExpr::Choice { .. } => "choice",
        GrammarExpr::Optional(_) => "optional",
        GrammarExpr::ZeroOrMore(_) => "repeat0",
        GrammarExpr::OneOrMore(_) => "repeat1",
        GrammarExpr::And(_) => "and",
        GrammarExpr::Not(_) => "not",
        GrammarExpr::Repeat { .. } => "repeat",
        GrammarExpr::Capture { .. } => "capture",
        GrammarExpr::Feature(feature) => feature.head(),
    }
}

fn inner(expr: &GrammarExpr) -> Option<&GrammarExpr> {
    match expr {
        GrammarExpr::Optional(item)
        | GrammarExpr::ZeroOrMore(item)
        | GrammarExpr::OneOrMore(item)
        | GrammarExpr::And(item)
        | GrammarExpr::Not(item)
        | GrammarExpr::Repeat { expr: item, .. }
        | GrammarExpr::Capture { expr: item, .. } => Some(item),
        _ => form(expr).and_then(|value| value.expression("item")),
    }
}

fn children(expr: &GrammarExpr) -> &[GrammarExpr] {
    match expr {
        GrammarExpr::Sequence(items)
        | GrammarExpr::Choice {
            alternatives: items,
            ..
        } => items,
        _ => form(expr).map_or(&[], |value| value.expressions("items")),
    }
}

pub(super) fn lower_expression(
    expr: &GrammarExpr,
    name: &str,
    obligations: &mut Vec<LexicalObligation>,
) -> LoadResult<GrammarExpr> {
    struct Projection<'a> {
        nodes: Vec<&'a GrammarExpr>,
        inner: Vec<f64>,
        children: Vec<Vec<usize>>,
        sources: Vec<f64>,
        targets: Vec<f64>,
    }
    fn capture<'a>(expr: &'a GrammarExpr, projection: &mut Projection<'a>) -> usize {
        let index = projection.nodes.len();
        projection.nodes.push(expr);
        projection.inner.push(-1.0);
        projection.children.push(Vec::new());
        if let Some(item) = inner(expr) {
            projection.inner[index] = kernel::number(capture(item, projection));
        }
        for item in children(expr) {
            let child = capture(item, projection);
            projection.children[index].push(child);
            projection.sources.push(kernel::number(index));
            projection.targets.push(kernel::number(child));
        }
        index
    }
    fn materialize(
        index: usize,
        projection: &Projection<'_>,
        replacements: &HashMap<usize, usize>,
    ) -> GrammarExpr {
        if let Some(&delimiter) = replacements.get(&index) {
            return GrammarExpr::zero_or_more(GrammarExpr::Sequence(vec![
                GrammarExpr::not(projection.nodes[delimiter].clone()),
                GrammarExpr::AnyChar,
            ]));
        }
        let mut value = projection.nodes[index].clone();
        let mut cursor = 0;
        let mut positions = Vec::new();
        if projection.inner[index] >= 0.0 {
            positions.push(kernel::position(projection.inner[index]));
        }
        positions.extend(&projection.children[index]);
        super::load::map_children_mut(&mut value, &mut |child| {
            if let Some(&position) = positions.get(cursor) {
                *child = materialize(position, projection, replacements);
                cursor += 1;
            }
        });
        value
    }
    let mut projection = Projection {
        nodes: Vec::new(),
        inner: Vec::new(),
        children: Vec::new(),
        sources: Vec::new(),
        targets: Vec::new(),
    };
    capture(expr, &mut projection);
    let kinds = projection
        .nodes
        .iter()
        .map(|node| kind(node).to_owned())
        .collect::<Vec<_>>();
    let labels = projection
        .nodes
        .iter()
        .map(|node| match node {
            GrammarExpr::Capture { label, .. } => label.clone().unwrap_or_default(),
            _ => String::new(),
        })
        .collect::<Vec<_>>();
    let values = projection
        .nodes
        .iter()
        .map(|node| match node {
            GrammarExpr::Terminal(value) => value.clone(),
            _ => String::new(),
        })
        .collect::<Vec<_>>();
    let normalization = kernel::normalization(
        &kinds,
        &labels,
        &values,
        &projection.inner,
        &projection.sources,
        &projection.targets,
    );
    if normalization.first() == Some(&-1.0) {
        return refuse("declaration", "invalid lexical expression graph projection");
    }
    let mut replacements = HashMap::new();
    for pair in normalization.as_chunks::<2>().0 {
        if pair[1] < 0.0 {
            obligations.push(LexicalObligation {
                name: name.to_owned(),
                category: "non-greedy".to_owned(),
                source: "unsupported non-greedy lexical decision".to_owned(),
            });
        } else {
            replacements.insert(kernel::position(pair[0]), kernel::position(pair[1]));
        }
    }
    Ok(materialize(0, &projection, &replacements))
}

pub(super) fn read_source(declaration: &GrammarLexer) -> LoadResult<LexicalSource> {
    let GrammarExpr::Sequence(entries) = &declaration.expression else {
        return refuse(
            "declaration",
            format!(
                "lexer {} needs an ordered declaration list",
                declaration.name
            ),
        );
    };
    let forms = entries
        .iter()
        .map(|entry| form(entry).cloned())
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| super::load::GrammarRuntimeError {
            reason: "declaration",
            message: "a lexical declaration entry needs a shared feature form".to_owned(),
        })?;
    let mut columns: [Vec<String>; 7] = std::array::from_fn(|_| Vec::new());
    let (mut action_owners, mut actions, mut action_modes) = (Vec::new(), Vec::new(), Vec::new());
    for (index, entry) in forms.iter().enumerate() {
        columns[0].push(entry.head.clone());
        columns[1].push(entry.text("name").unwrap_or_default().to_owned());
        columns[2].push(entry.expression("item").map_or("", kind).to_owned());
        for (column, field) in [
            (3, "value"),
            (4, "mode"),
            (5, "channel"),
            (6, "disposition"),
        ] {
            columns[column].push(entry.text(field).unwrap_or_default().to_owned());
        }
        for action in entry.operations("action").unwrap_or_default() {
            action_owners.push(kernel::number(index));
            actions.push(action.head.clone());
            action_modes.push(action.text("mode").unwrap_or_default().to_owned());
        }
    }
    let issue = kernel::declaration_issue(&columns, &action_owners, &actions, &action_modes);
    if issue.first() != Some(&0.0) {
        return refuse(
            "declaration",
            format!(
                "invalid lexical declaration in lexer {} (issue {}, entry {}, action {})",
                declaration.name, issue[0], issue[1], issue[2]
            ),
        );
    }
    let mut plan = LexicalPlan {
        name: Name::from(declaration.name.as_str()),
        rules: Vec::new(),
        bindings: Vec::new(),
        literals: HashMap::new(),
        owners: HashMap::new(),
        original_owners: HashSet::new(),
        obligations: Vec::new(),
        program: 0,
    };
    let mut grammar = Grammar::new();
    let mut rules = Vec::new();
    let mut modes = HashSet::from(["default".to_owned()]);
    for entry in forms {
        let name = entry.text("name").unwrap_or_default();
        match kernel::entry_form(&entry.head) {
            1 | 2 => {
                let Some(GrammarExpr::NonTerminal(target)) = entry.expression("item") else {
                    return refuse(
                        "declaration",
                        "a lexical binding needs a public rule reference",
                    );
                };
                if kernel::entry_form(&entry.head) == 1 {
                    plan.owners
                        .insert(Name::from(target.as_str()), Name::from(name));
                    plan.original_owners.insert(Name::from(name));
                } else {
                    plan.bindings
                        .push((Name::from(name), Name::from(target.as_str())));
                }
            }
            3 => {
                plan.literals.insert(
                    entry.text("value").unwrap_or_default().as_bytes().to_vec(),
                    Name::from(name),
                );
            }
            4 => {
                modes.insert(name.to_owned());
            }
            6 => plan.obligations.push(LexicalObligation {
                name: name.to_owned(),
                category: entry.text("category").unwrap_or_default().to_owned(),
                source: entry.text("source").unwrap_or_default().to_owned(),
            }),
            7 | 8 => {
                let expression = lower_expression(
                    entry.expression("item").expect("validated lexical item"),
                    name,
                    &mut plan.obligations,
                )?;
                if grammar.rules().is_empty() {
                    grammar.set_start(name);
                }
                grammar.add_rule(GrammarRule::new(name, expression).with_kind(RuleKind::Silent));
                if kernel::entry_form(&entry.head) == 7 {
                    rules.push(entry);
                }
            }
            _ => {}
        }
    }
    if let Some(unresolved) = plan
        .obligations
        .iter()
        .find(|obligation| obligation.name.is_empty())
    {
        return refuse(
            "operation",
            format!(
                "lexer {} has an unresolved {}: {}",
                plan.name, unresolved.category, unresolved.source
            ),
        );
    }
    Ok(LexicalSource {
        plan,
        grammar,
        rules,
        modes,
    })
}

pub(super) fn selected_obligations(
    plan: &LexicalPlan,
    grammar: &Grammar,
    name: &str,
) -> LoadResult<Vec<LexicalObligation>> {
    let names = grammar
        .rules()
        .iter()
        .map(|rule| rule.name.as_str())
        .collect::<Vec<_>>();
    let positions = names
        .iter()
        .enumerate()
        .map(|(index, &name)| (name, index))
        .collect::<HashMap<_, _>>();
    let (mut sources, mut targets) = (Vec::new(), Vec::new());
    for (index, rule) in grammar.rules().iter().enumerate() {
        let mut references = BTreeSet::new();
        rule.expr.collect_nonterminals(&mut references);
        for reference in references {
            if let Some(&target) = positions.get(reference.as_str()) {
                sources.push(kernel::number(index));
                targets.push(kernel::number(target));
            }
        }
    }
    let owners = plan
        .obligations
        .iter()
        .map(|item| {
            positions
                .get(item.name.as_str())
                .map_or(-1.0, |&index| kernel::number(index))
        })
        .collect::<Vec<_>>();
    let categories = plan
        .obligations
        .iter()
        .map(|item| item.category.clone())
        .collect::<Vec<_>>();
    let selected = kernel::obligations(
        names.len(),
        positions[name],
        &sources,
        &targets,
        &owners,
        &categories,
    );
    if selected.first() == Some(&-1.0) {
        return refuse(
            "declaration",
            "invalid source-local lexical obligation graph projection",
        );
    }
    Ok(selected
        .iter()
        .map(|&position| plan.obligations[kernel::position(position)].clone())
        .collect())
}

pub(super) fn action(entry: &FeatureForm) -> &[Operation] {
    entry.operations("action").unwrap_or_default()
}
