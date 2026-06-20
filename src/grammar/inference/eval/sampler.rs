use std::collections::{BTreeSet, HashMap, HashSet};

use crate::grammar::{CharClassItem, Grammar, GrammarExpr};

use super::{EvalError, SampleConfig};

const PRINTABLE_ASCII: &[u8] =
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 _-.,:;()[]{}";

pub(super) fn sample(grammar: &Grammar, config: &SampleConfig) -> Result<Vec<String>, EvalError> {
    let start = start_rule_name(grammar)?;
    let plan = TerminationPlan::new(grammar, &start)?;
    let start_rule = grammar.rule(&start).ok_or_else(|| EvalError::UnknownRule {
        rule: start.clone(),
    })?;
    let mut rng = SplitMix64::new(config.seed);
    let mut seen = BTreeSet::new();
    let mut samples = Vec::new();

    for _ in 0..config.count {
        let text = expand_expr(
            grammar,
            start_rule.expr(),
            config,
            &mut rng,
            config.max_depth,
            &plan,
        )?;
        if seen.insert(text.clone()) {
            samples.push(text);
        }
    }

    Ok(samples)
}

#[derive(Clone, Debug)]
struct TerminationPlan {
    min_lengths: HashMap<String, usize>,
}

impl TerminationPlan {
    fn new(grammar: &Grammar, start: &str) -> Result<Self, EvalError> {
        let reachable = reachable_rules(grammar, start)?;
        let raw_lengths = compute_min_lengths(grammar);
        let mut min_lengths = HashMap::new();

        for rule in &reachable {
            let Some(Some(length)) = raw_lengths.get(rule) else {
                return Err(EvalError::NonTerminating { rule: rule.clone() });
            };
            min_lengths.insert(rule.clone(), *length);
        }

        Ok(Self { min_lengths })
    }

    fn expr_min_length(&self, expr: &GrammarExpr) -> Option<usize> {
        expr_min_length(expr, &self.min_lengths)
    }
}

fn expand_expr(
    grammar: &Grammar,
    expr: &GrammarExpr,
    config: &SampleConfig,
    rng: &mut SplitMix64,
    fuel: usize,
    plan: &TerminationPlan,
) -> Result<String, EvalError> {
    match expr {
        GrammarExpr::Empty | GrammarExpr::And(_) | GrammarExpr::Not(_) => Ok(String::new()),
        GrammarExpr::Terminal(value) | GrammarExpr::TerminalInsensitive(value) => Ok(value.clone()),
        GrammarExpr::CharRange(start, end) => pick_char_range(*start, *end, rng).map(String::from),
        GrammarExpr::CharClass { negated, items } => {
            pick_char_class(*negated, items, rng).map(String::from)
        }
        GrammarExpr::AnyChar => Ok(String::from(char::from(
            PRINTABLE_ASCII[rng.next_usize(PRINTABLE_ASCII.len())],
        ))),
        GrammarExpr::NonTerminal(name) => {
            if fuel == 0 {
                return shortest_rule(grammar, name, plan);
            }
            let rule = grammar
                .rule(name)
                .ok_or_else(|| EvalError::UnknownRule { rule: name.clone() })?;
            expand_expr(
                grammar,
                rule.expr(),
                config,
                rng,
                fuel.saturating_sub(1),
                plan,
            )
        }
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => {
            let alternative = choose_alternative(alternatives, *ordered, fuel, rng, plan)?;
            expand_expr(grammar, alternative, config, rng, fuel, plan)
        }
        GrammarExpr::Sequence(items) => {
            let mut output = String::new();
            for item in items {
                output.push_str(&expand_expr(grammar, item, config, rng, fuel, plan)?);
            }
            Ok(output)
        }
        GrammarExpr::Optional(inner) => {
            if fuel == 0 || !rng.next_bool() {
                Ok(String::new())
            } else {
                expand_expr(grammar, inner, config, rng, fuel, plan)
            }
        }
        GrammarExpr::ZeroOrMore(inner) => {
            let count = if fuel == 0 {
                0
            } else {
                rng.next_usize(config.repeat_cap.saturating_add(1))
            };
            repeat_expand(grammar, inner, count, config, rng, fuel, plan)
        }
        GrammarExpr::OneOrMore(inner) => {
            let upper = config.repeat_cap.max(1);
            let count = if fuel == 0 {
                1
            } else {
                1 + rng.next_usize(upper)
            };
            repeat_expand(grammar, inner, count, config, rng, fuel, plan)
        }
        GrammarExpr::Repeat { expr, min, max } => {
            let count = repeat_count(*min, *max, config, fuel, rng)?;
            repeat_expand(grammar, expr, count, config, rng, fuel, plan)
        }
        GrammarExpr::Capture { expr, .. } => expand_expr(grammar, expr, config, rng, fuel, plan),
    }
}

fn repeat_expand(
    grammar: &Grammar,
    inner: &GrammarExpr,
    count: usize,
    config: &SampleConfig,
    rng: &mut SplitMix64,
    fuel: usize,
    plan: &TerminationPlan,
) -> Result<String, EvalError> {
    if count > 0 && plan.expr_min_length(inner).is_none() {
        return Err(EvalError::NonTerminating {
            rule: "<repeat>".to_string(),
        });
    }

    let mut output = String::new();
    for _ in 0..count {
        output.push_str(&expand_expr(grammar, inner, config, rng, fuel, plan)?);
    }
    Ok(output)
}

fn repeat_count(
    min: usize,
    max: Option<usize>,
    config: &SampleConfig,
    fuel: usize,
    rng: &mut SplitMix64,
) -> Result<usize, EvalError> {
    let explicit_or_cap = max.unwrap_or_else(|| config.repeat_cap.max(min));
    if explicit_or_cap < min {
        return Err(EvalError::InvalidRepeat {
            min,
            max: explicit_or_cap,
        });
    }
    let upper = explicit_or_cap.min(config.repeat_cap.max(min));
    if fuel == 0 || upper == min {
        Ok(min)
    } else {
        Ok(min + rng.next_usize(upper - min + 1))
    }
}

fn choose_alternative<'a>(
    alternatives: &'a [GrammarExpr],
    ordered: bool,
    fuel: usize,
    rng: &mut SplitMix64,
    plan: &TerminationPlan,
) -> Result<&'a GrammarExpr, EvalError> {
    let viable = alternatives
        .iter()
        .enumerate()
        .filter_map(|(index, expr)| {
            plan.expr_min_length(expr)
                .map(|length| (index, expr, length))
        })
        .collect::<Vec<_>>();

    if viable.is_empty() {
        return Err(EvalError::NonTerminating {
            rule: "<choice>".to_string(),
        });
    }

    if fuel == 0 {
        return viable
            .iter()
            .min_by_key(|(index, _, length)| (*length, *index))
            .map(|(_, expr, _)| *expr)
            .ok_or_else(|| EvalError::NonTerminating {
                rule: "<choice>".to_string(),
            });
    }

    if ordered {
        let total_weight = viable.len().saturating_mul(viable.len().saturating_add(1)) / 2;
        let mut pick = rng.next_usize(total_weight);
        for (rank, (_, expr, _)) in viable.iter().enumerate() {
            let weight = viable.len() - rank;
            if pick < weight {
                return Ok(*expr);
            }
            pick -= weight;
        }
        Ok(viable[0].1)
    } else {
        Ok(viable[rng.next_usize(viable.len())].1)
    }
}

fn shortest_rule(
    grammar: &Grammar,
    name: &str,
    plan: &TerminationPlan,
) -> Result<String, EvalError> {
    shortest_rule_inner(grammar, name, plan, &mut HashSet::new()).ok_or_else(|| {
        EvalError::NonTerminating {
            rule: name.to_string(),
        }
    })
}

fn shortest_rule_inner(
    grammar: &Grammar,
    name: &str,
    plan: &TerminationPlan,
    visiting: &mut HashSet<String>,
) -> Option<String> {
    if !visiting.insert(name.to_string()) {
        return None;
    }
    let rule = grammar.rule(name)?;
    let output = shortest_expr(grammar, rule.expr(), plan, visiting);
    visiting.remove(name);
    output
}

fn shortest_expr(
    grammar: &Grammar,
    expr: &GrammarExpr,
    plan: &TerminationPlan,
    visiting: &mut HashSet<String>,
) -> Option<String> {
    match expr {
        GrammarExpr::Empty
        | GrammarExpr::Optional(_)
        | GrammarExpr::ZeroOrMore(_)
        | GrammarExpr::And(_)
        | GrammarExpr::Not(_) => Some(String::new()),
        GrammarExpr::Terminal(value) | GrammarExpr::TerminalInsensitive(value) => {
            Some(value.clone())
        }
        GrammarExpr::CharRange(start, end) => first_char_in_range(*start, *end).map(String::from),
        GrammarExpr::CharClass { negated, items } => {
            first_char_in_class(*negated, items).map(String::from)
        }
        GrammarExpr::AnyChar => Some(String::from(char::from(PRINTABLE_ASCII[0]))),
        GrammarExpr::NonTerminal(name) => shortest_rule_inner(grammar, name, plan, visiting),
        GrammarExpr::Choice { alternatives, .. } => alternatives
            .iter()
            .enumerate()
            .filter_map(|(index, alternative)| {
                plan.expr_min_length(alternative).map(|length| {
                    (
                        index,
                        length,
                        shortest_expr(grammar, alternative, plan, visiting),
                    )
                })
            })
            .filter_map(|(index, length, output)| output.map(|output| (index, length, output)))
            .min_by_key(|(index, length, _)| (*length, *index))
            .map(|(_, _, output)| output),
        GrammarExpr::Sequence(items) => {
            let mut output = String::new();
            for item in items {
                output.push_str(&shortest_expr(grammar, item, plan, visiting)?);
            }
            Some(output)
        }
        GrammarExpr::OneOrMore(inner) => shortest_expr(grammar, inner, plan, visiting),
        GrammarExpr::Repeat { expr, min, max } => {
            if max.is_some_and(|max| max < *min) {
                return None;
            }
            let mut output = String::new();
            for _ in 0..*min {
                output.push_str(&shortest_expr(grammar, expr, plan, visiting)?);
            }
            Some(output)
        }
        GrammarExpr::Capture { expr, .. } => shortest_expr(grammar, expr, plan, visiting),
    }
}

fn start_rule_name(grammar: &Grammar) -> Result<String, EvalError> {
    if grammar.rules().is_empty() {
        return Err(EvalError::EmptyGrammar);
    }
    grammar.start().map_or_else(
        || Ok(grammar.rules()[0].name().to_string()),
        |start| {
            if grammar.rule(start).is_some() {
                Ok(start.to_string())
            } else {
                Err(EvalError::UnknownRule {
                    rule: start.to_string(),
                })
            }
        },
    )
}

fn compute_min_lengths(grammar: &Grammar) -> HashMap<String, Option<usize>> {
    let mut lengths = grammar
        .rules()
        .iter()
        .map(|rule| (rule.name().to_string(), None))
        .collect::<HashMap<_, _>>();
    let mut changed = true;

    while changed {
        changed = false;
        for rule in grammar.rules() {
            let Some(next) = expr_min_length_with_options(rule.expr(), &lengths) else {
                continue;
            };
            let current = lengths.get(rule.name()).copied().flatten();
            if current.map_or(true, |current| next < current) {
                lengths.insert(rule.name().to_string(), Some(next));
                changed = true;
            }
        }
    }

    lengths
}

fn expr_min_length(expr: &GrammarExpr, lengths: &HashMap<String, usize>) -> Option<usize> {
    match expr {
        GrammarExpr::NonTerminal(name) => lengths.get(name).copied(),
        GrammarExpr::Choice { alternatives, .. } => alternatives
            .iter()
            .filter_map(|alternative| expr_min_length(alternative, lengths))
            .min(),
        GrammarExpr::Sequence(items) => {
            let mut total = 0usize;
            for item in items {
                total = total.saturating_add(expr_min_length(item, lengths)?);
            }
            Some(total)
        }
        GrammarExpr::OneOrMore(expr) | GrammarExpr::Capture { expr, .. } => {
            expr_min_length(expr, lengths)
        }
        GrammarExpr::Repeat { expr, min, max } => {
            if max.is_some_and(|max| max < *min) {
                None
            } else {
                expr_min_length(expr, lengths).map(|length| length.saturating_mul(*min))
            }
        }
        GrammarExpr::Empty
        | GrammarExpr::Optional(_)
        | GrammarExpr::ZeroOrMore(_)
        | GrammarExpr::And(_)
        | GrammarExpr::Not(_) => Some(0),
        GrammarExpr::Terminal(value) | GrammarExpr::TerminalInsensitive(value) => {
            Some(value.chars().count())
        }
        GrammarExpr::CharRange(start, end) => (start <= end).then_some(1),
        GrammarExpr::CharClass { negated, items } => {
            first_char_in_class(*negated, items).map(|_| 1)
        }
        GrammarExpr::AnyChar => Some(1),
    }
}

fn expr_min_length_with_options(
    expr: &GrammarExpr,
    lengths: &HashMap<String, Option<usize>>,
) -> Option<usize> {
    match expr {
        GrammarExpr::NonTerminal(name) => lengths.get(name).copied().flatten(),
        GrammarExpr::Choice { alternatives, .. } => alternatives
            .iter()
            .filter_map(|alternative| expr_min_length_with_options(alternative, lengths))
            .min(),
        GrammarExpr::Sequence(items) => {
            let mut total = 0usize;
            for item in items {
                total = total.saturating_add(expr_min_length_with_options(item, lengths)?);
            }
            Some(total)
        }
        GrammarExpr::OneOrMore(expr) | GrammarExpr::Capture { expr, .. } => {
            expr_min_length_with_options(expr, lengths)
        }
        GrammarExpr::Repeat { expr, min, max } => {
            if max.is_some_and(|max| max < *min) {
                None
            } else {
                expr_min_length_with_options(expr, lengths)
                    .map(|length| length.saturating_mul(*min))
            }
        }
        GrammarExpr::Empty
        | GrammarExpr::Optional(_)
        | GrammarExpr::ZeroOrMore(_)
        | GrammarExpr::And(_)
        | GrammarExpr::Not(_) => Some(0),
        GrammarExpr::Terminal(value) | GrammarExpr::TerminalInsensitive(value) => {
            Some(value.chars().count())
        }
        GrammarExpr::CharRange(start, end) => (start <= end).then_some(1),
        GrammarExpr::CharClass { negated, items } => {
            first_char_in_class(*negated, items).map(|_| 1)
        }
        GrammarExpr::AnyChar => Some(1),
    }
}

fn reachable_rules(grammar: &Grammar, start: &str) -> Result<BTreeSet<String>, EvalError> {
    let mut reachable = BTreeSet::new();
    let mut stack = vec![start.to_string()];

    while let Some(name) = stack.pop() {
        if !reachable.insert(name.clone()) {
            continue;
        }
        let rule = grammar
            .rule(&name)
            .ok_or_else(|| EvalError::UnknownRule { rule: name.clone() })?;
        let mut references = BTreeSet::new();
        collect_nonterminals(rule.expr(), &mut references);
        for reference in references {
            if grammar.rule(&reference).is_none() {
                return Err(EvalError::UnknownRule { rule: reference });
            }
            stack.push(reference);
        }
    }

    Ok(reachable)
}

fn collect_nonterminals(expr: &GrammarExpr, names: &mut BTreeSet<String>) {
    match expr {
        GrammarExpr::NonTerminal(name) => {
            names.insert(name.clone());
        }
        GrammarExpr::Choice { alternatives, .. } => {
            for alternative in alternatives {
                collect_nonterminals(alternative, names);
            }
        }
        GrammarExpr::Sequence(items) => {
            for item in items {
                collect_nonterminals(item, names);
            }
        }
        GrammarExpr::Optional(expr)
        | GrammarExpr::ZeroOrMore(expr)
        | GrammarExpr::OneOrMore(expr)
        | GrammarExpr::And(expr)
        | GrammarExpr::Not(expr)
        | GrammarExpr::Capture { expr, .. }
        | GrammarExpr::Repeat { expr, .. } => collect_nonterminals(expr, names),
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar => {}
    }
}

#[derive(Clone, Copy, Debug)]
struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut value = self.state;
        value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        value ^ (value >> 31)
    }

    fn next_usize(&mut self, bound: usize) -> usize {
        if bound <= 1 {
            return 0;
        }
        let bound = u64::try_from(bound).unwrap_or(u64::MAX);
        let value = self.next_u64() % bound;
        usize::try_from(value).unwrap_or(0)
    }

    fn next_bool(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }
}

fn pick_char_range(start: char, end: char, rng: &mut SplitMix64) -> Result<char, EvalError> {
    if start > end {
        return Err(EvalError::InvalidCharRange { start, end });
    }
    let start = u32::from(start);
    let end = u32::from(end);
    let span = u64::from(end - start) + 1;
    let offset = u32::try_from(rng.next_u64() % span).unwrap_or(0);
    let mut candidate = start + offset;

    loop {
        if let Some(value) = char::from_u32(candidate) {
            return Ok(value);
        }
        candidate = if candidate == end {
            start
        } else {
            candidate.saturating_add(1)
        };
    }
}

fn pick_char_class(
    negated: bool,
    items: &[CharClassItem],
    rng: &mut SplitMix64,
) -> Result<char, EvalError> {
    if negated {
        PRINTABLE_ASCII
            .iter()
            .map(|value| char::from(*value))
            .find(|value| !class_accepts(*value, false, items))
            .ok_or(EvalError::EmptyCharClass)
    } else if items.is_empty() {
        Err(EvalError::EmptyCharClass)
    } else {
        match items[rng.next_usize(items.len())] {
            CharClassItem::Char(value) => Ok(value),
            CharClassItem::Range(start, end) => pick_char_range(start, end, rng),
        }
    }
}

fn first_char_in_range(start: char, end: char) -> Option<char> {
    if start > end {
        return None;
    }
    let mut candidate = u32::from(start);
    let end = u32::from(end);
    while candidate <= end {
        if let Some(value) = char::from_u32(candidate) {
            return Some(value);
        }
        candidate = candidate.saturating_add(1);
    }
    None
}

fn first_char_in_class(negated: bool, items: &[CharClassItem]) -> Option<char> {
    if negated {
        return PRINTABLE_ASCII
            .iter()
            .map(|value| char::from(*value))
            .find(|value| !class_accepts(*value, false, items));
    }

    items.iter().find_map(|item| match item {
        CharClassItem::Char(value) => Some(*value),
        CharClassItem::Range(start, end) => first_char_in_range(*start, *end),
    })
}

fn class_accepts(value: char, negated: bool, items: &[CharClassItem]) -> bool {
    let contains = items.iter().any(|item| match item {
        CharClassItem::Char(item) => *item == value,
        CharClassItem::Range(start, end) => *start <= value && value <= *end,
    });
    contains != negated
}
