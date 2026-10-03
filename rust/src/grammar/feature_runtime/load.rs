//! Loading a grammar for the native executor, as
//! `js/src/grammar-runtime/load.js`: imports are resolved through the
//! injected resolver (no file system or network access), macros are
//! expanded, parameterized rules are instantiated, and `compile.rs` checks
//! every operation against its context and compiles the terminals.

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::sync::Arc;

use super::compile::Compiler;
use super::program::{
    Compiled, Expr, Matcher, Program, Rule, Scanner, Target, TokenRank, TokenRanks, Trivia,
};
use crate::grammar::interchange::render_native_expression;
use crate::grammar::{
    FeatureExpr, FeatureForm, FieldValue, Grammar, GrammarExpr, GrammarFormat, GrammarMacro,
    GrammarScanner, Operation, PrecedenceEntry, RuleAttributes, RuleKind,
};

/// A grammar that cannot be loaded or a parse that cannot run; `reason`
/// names the step: `import`, `macro`, `parameter`, `reference`,
/// `declaration`, `operation`, `pattern` or `embed`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarRuntimeError {
    /// The step that refused the grammar.
    pub reason: &'static str,
    /// What is wrong.
    pub message: String,
}

impl fmt::Display for GrammarRuntimeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} ({})", self.message, self.reason)
    }
}

impl std::error::Error for GrammarRuntimeError {}

pub(super) type LoadResult<T> = Result<T, GrammarRuntimeError>;

/// Fails a load with `reason` and `message`.
pub(super) fn refuse<T>(reason: &'static str, message: impl Into<String>) -> LoadResult<T> {
    Err(GrammarRuntimeError {
        reason,
        message: message.into(),
    })
}

/// Resolves the grammar an import or an embedded language names.
pub(super) type Resolver<'a> = Option<&'a dyn Fn(&str) -> Option<Grammar>>;

const INSTANCE_LIMIT: usize = 1000;

/// A map that keeps the order of first insertion and replaces in place.
struct Ordered<T> {
    items: Vec<(String, T)>,
    index: HashMap<String, usize>,
}

impl<T> Ordered<T> {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            index: HashMap::new(),
        }
    }

    fn set(&mut self, name: String, value: T) {
        if let Some(&position) = self.index.get(&name) {
            self.items[position].1 = value;
        } else {
            self.index.insert(name.clone(), self.items.len());
            self.items.push((name, value));
        }
    }

    fn get(&self, name: &str) -> Option<&T> {
        self.index
            .get(name)
            .map(|&position| &self.items[position].1)
    }

    fn contains(&self, name: &str) -> bool {
        self.index.contains_key(name)
    }

    const fn len(&self) -> usize {
        self.items.len()
    }
}

/// The form `head` when `expr` is one.
pub(super) fn form_of<'a>(expr: &'a GrammarExpr, head: &str) -> Option<&'a FeatureForm> {
    match expr {
        GrammarExpr::Feature(feature) => match &**feature {
            FeatureExpr::Form(form) if form.head == head => Some(form),
            _ => None,
        },
        _ => None,
    }
}

/// Rewrites the direct sub-expressions of `expr`.
fn map_children_mut(expr: &mut GrammarExpr, map: &mut impl FnMut(&mut GrammarExpr)) {
    match expr {
        GrammarExpr::Optional(inner)
        | GrammarExpr::ZeroOrMore(inner)
        | GrammarExpr::OneOrMore(inner)
        | GrammarExpr::And(inner)
        | GrammarExpr::Not(inner)
        | GrammarExpr::Repeat { expr: inner, .. }
        | GrammarExpr::Capture { expr: inner, .. } => map(inner),
        GrammarExpr::Choice { alternatives, .. } => alternatives.iter_mut().for_each(map),
        GrammarExpr::Sequence(items) => items.iter_mut().for_each(map),
        GrammarExpr::Feature(feature) => feature.map_expressions(map),
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(..)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar
        | GrammarExpr::NonTerminal(_) => {}
    }
}

/// A copy of `expr` with each direct sub-expression replaced by `map`'s result.
fn try_map_children(
    expr: &GrammarExpr,
    mut map: impl FnMut(&GrammarExpr) -> LoadResult<GrammarExpr>,
) -> LoadResult<GrammarExpr> {
    let mut copy = expr.clone();
    let mut failure = None;
    map_children_mut(&mut copy, &mut |child: &mut GrammarExpr| {
        if failure.is_none() {
            match map(child) {
                Ok(mapped) => *child = mapped,
                Err(error) => failure = Some(error),
            }
        }
    });
    failure.map_or(Ok(copy), Err)
}

/// A copy of `operations` with every expression they hold replaced by `map`'s result.
fn try_map_operations(
    operations: &[Operation],
    mut map: impl FnMut(&GrammarExpr) -> LoadResult<GrammarExpr>,
) -> LoadResult<Vec<Operation>> {
    let mut copy = operations.to_vec();
    let mut failure = None;
    for operation in &mut copy {
        operation.map_expressions(&mut |child: &mut GrammarExpr| {
            if failure.is_none() {
                match map(child) {
                    Ok(mapped) => *child = mapped,
                    Err(error) => failure = Some(error),
                }
            }
        });
    }
    failure.map_or(Ok(copy), Err)
}

/// The shared state of one load: the resolver and every language loaded so far.
struct Context<'a> {
    resolver: Resolver<'a>,
    programs: Vec<Program>,
    languages: HashMap<String, usize>,
}

impl Context<'_> {
    fn resolve(&self, name: &str, purpose: &'static str) -> LoadResult<Grammar> {
        let resolved = self.resolver.and_then(|resolve| resolve(name));
        resolved.map_or_else(
            || {
                let what = if purpose == "import" {
                    "imported grammar"
                } else {
                    "embedded language"
                };
                refuse(purpose, format!("{what} {name} cannot be resolved"))
            },
            Ok,
        )
    }
}

/// A grammar with its imports merged in.
struct Resolved {
    start: Option<String>,
    peg_source: bool,
    matching: Option<String>,
    rules: Ordered<(RuleKind, GrammarExpr, RuleAttributes)>,
    modes: Vec<String>,
    extras: Vec<GrammarExpr>,
    conflicts: Vec<Vec<String>>,
    precedences: Vec<Vec<PrecedenceEntry>>,
    macros: Ordered<GrammarMacro>,
    scanners: Vec<GrammarScanner>,
}

// Imports: every imported grammar's rules first (recursively, in import
// order), then the local rules, which override imported rules of the same
// name in place; declarations accumulate, and a local macro overrides an
// imported one of the same name.
fn resolve_imports(
    grammar: &Grammar,
    context: &Context<'_>,
    chain: &[String],
) -> LoadResult<Resolved> {
    let declarations = grammar.declarations();
    let mut resolved = Resolved {
        start: grammar
            .start()
            .map(str::to_owned)
            .or_else(|| grammar.rules().first().map(|rule| rule.name.clone())),
        peg_source: grammar.source_format() == Some(GrammarFormat::Peg),
        matching: declarations.matching.clone(),
        rules: Ordered::new(),
        modes: Vec::new(),
        extras: Vec::new(),
        conflicts: Vec::new(),
        precedences: Vec::new(),
        macros: Ordered::new(),
        scanners: Vec::new(),
    };
    for name in &declarations.imports {
        let mut next = chain.to_vec();
        next.push(name.clone());
        if chain.contains(name) {
            return refuse("import", format!("import cycle {}", next.join(" -> ")));
        }
        let imported = context.resolve(name, "import")?;
        let inner = resolve_imports(&imported, context, &next)?;
        for (rule_name, rule) in inner.rules.items {
            resolved.rules.set(rule_name, rule);
        }
        resolved.modes.extend(inner.modes);
        resolved.extras.extend(inner.extras);
        resolved.conflicts.extend(inner.conflicts);
        resolved.precedences.extend(inner.precedences);
        for (macro_name, item) in inner.macros.items {
            resolved.macros.set(macro_name, item);
        }
        resolved.scanners.extend(inner.scanners);
    }
    for rule in grammar.rules() {
        resolved.rules.set(
            rule.name.clone(),
            (rule.kind, rule.expr.clone(), rule.attributes.clone()),
        );
    }
    resolved.modes.extend(declarations.modes.iter().cloned());
    resolved.extras.extend(declarations.extras.iter().cloned());
    resolved
        .conflicts
        .extend(declarations.conflicts.iter().cloned());
    resolved
        .precedences
        .extend(declarations.precedences.iter().cloned());
    for item in &declarations.macros {
        resolved.macros.set(item.name.clone(), item.clone());
    }
    resolved
        .scanners
        .extend(declarations.scanners.iter().cloned());
    Ok(resolved)
}

fn substitute(expr: &GrammarExpr, bindings: &HashMap<String, GrammarExpr>) -> GrammarExpr {
    if let Some(bound) = form_of(expr, "parameter")
        .and_then(|form| form.text("name"))
        .and_then(|name| bindings.get(name))
    {
        return bound.clone();
    }
    let mut copy = expr.clone();
    map_children_mut(&mut copy, &mut |child: &mut GrammarExpr| {
        *child = substitute(child, bindings);
    });
    copy
}

// Macros expand inline at load: `expand(NAME, ARG...)` becomes the macro body
// with each `parameter(P)` replaced by its argument.
fn expand_macros(
    expr: &GrammarExpr,
    macros: &Ordered<GrammarMacro>,
    active: &mut Vec<String>,
) -> LoadResult<GrammarExpr> {
    let Some(form) = form_of(expr, "expand") else {
        return try_map_children(expr, |child| expand_macros(child, macros, active));
    };
    let name = form.text("name").unwrap_or_default();
    let arguments = form.expressions("arguments");
    let Some(definition) = macros.get(name) else {
        return refuse("macro", format!("undefined macro {name}"));
    };
    if active.iter().any(|item| item == name) {
        return refuse(
            "macro",
            format!("recursive macro {} -> {name}", active.join(" -> ")),
        );
    }
    if definition.parameters.len() != arguments.len() {
        return refuse(
            "macro",
            format!(
                "macro {name} takes {} arguments, not {}",
                definition.parameters.len(),
                arguments.len()
            ),
        );
    }
    let mut bindings = HashMap::new();
    for (parameter, argument) in definition.parameters.iter().zip(arguments) {
        bindings.insert(parameter.clone(), expand_macros(argument, macros, active)?);
    }
    active.push(name.to_owned());
    let expanded = expand_macros(
        &substitute(&definition.expression, &bindings),
        macros,
        active,
    );
    active.pop();
    expanded
}

/// A rule after macro expansion.
struct Source {
    kind: RuleKind,
    expression: GrammarExpr,
    action: Option<Vec<Operation>>,
    attributes: RuleAttributes,
}

/// A rule after instantiation.
struct Instance {
    node_kind: String,
    source: String,
    expression: GrammarExpr,
    action: Option<Vec<Operation>>,
}

struct Pending {
    name: String,
    target: String,
    bindings: HashMap<String, GrammarExpr>,
}

// Parameterized rules are instantiated per distinct argument list; an
// instance is a rule named `NAME(ARGUMENT, ...)` whose nodes keep the kind NAME.
struct Instantiator<'a> {
    sources: &'a Ordered<Source>,
    external: &'a HashMap<String, usize>,
    rules: Ordered<Instance>,
    pending: VecDeque<Pending>,
    queued: HashSet<String>,
}

impl Instantiator<'_> {
    fn instantiate(
        &mut self,
        expr: &GrammarExpr,
        bindings: Option<&HashMap<String, GrammarExpr>>,
        owner: &str,
    ) -> LoadResult<GrammarExpr> {
        if let Some(form) = form_of(expr, "parameter") {
            let name = form.text("name").unwrap_or_default();
            return bindings.and_then(|bound| bound.get(name)).map_or_else(
                || {
                    refuse(
                        "parameter",
                        format!("parameter {name} is not a parameter of {owner}"),
                    )
                },
                |bound| Ok(bound.clone()),
            );
        }
        let (name, arguments): (&str, &[GrammarExpr]) = match expr {
            GrammarExpr::NonTerminal(name) => (name, &[]),
            GrammarExpr::Feature(feature) => match &**feature {
                FeatureExpr::Call { name, arguments } => (name, arguments),
                _ => {
                    return try_map_children(expr, |child| {
                        self.instantiate(child, bindings, owner)
                    });
                }
            },
            _ => return try_map_children(expr, |child| self.instantiate(child, bindings, owner)),
        };
        if self.external.contains_key(name) {
            if !arguments.is_empty() {
                return refuse(
                    "parameter",
                    format!("external token {name} takes no arguments"),
                );
            }
            return Ok(GrammarExpr::NonTerminal(name.to_owned()));
        }
        let Some(target) = self.sources.get(name) else {
            return refuse("reference", format!("undefined rule {name} in {owner}"));
        };
        let parameters = &target.attributes.parameters;
        if arguments.len() != parameters.len() {
            return refuse(
                "parameter",
                format!(
                    "rule {name} takes {} arguments, not {}",
                    parameters.len(),
                    arguments.len()
                ),
            );
        }
        if parameters.is_empty() {
            return Ok(GrammarExpr::NonTerminal(name.to_owned()));
        }
        let concrete = arguments
            .iter()
            .map(|argument| self.instantiate(argument, bindings, owner))
            .collect::<LoadResult<Vec<_>>>()?;
        let rendered = concrete
            .iter()
            .map(render_native_expression)
            .collect::<Vec<_>>();
        let instance = format!("{name}({})", rendered.join(", "));
        if !self.rules.contains(&instance) && !self.queued.contains(&instance) {
            if self.rules.len() + self.pending.len() >= INSTANCE_LIMIT + self.sources.len() {
                return refuse(
                    "parameter",
                    format!("parameterized rule {name} needs more than {INSTANCE_LIMIT} instances"),
                );
            }
            self.queued.insert(instance.clone());
            self.pending.push_back(Pending {
                name: instance.clone(),
                target: name.to_owned(),
                bindings: parameters.iter().cloned().zip(concrete).collect(),
            });
        }
        Ok(GrammarExpr::NonTerminal(instance))
    }

    fn instantiate_operations(
        &mut self,
        operations: &[Operation],
        bindings: Option<&HashMap<String, GrammarExpr>>,
        owner: &str,
    ) -> LoadResult<Vec<Operation>> {
        try_map_operations(operations, |expr| self.instantiate(expr, bindings, owner))
    }

    fn drain(&mut self) -> LoadResult<()> {
        while let Some(Pending {
            name,
            target,
            bindings,
        }) = self.pending.pop_front()
        {
            let source = self
                .sources
                .get(&target)
                .expect("a pending instance names a rule");
            let owner = format!("rule {target}");
            let expression = self.instantiate(&source.expression, Some(&bindings), &owner)?;
            let action = match &source.action {
                Some(action) => {
                    Some(self.instantiate_operations(action, Some(&bindings), &owner)?)
                }
                None => None,
            };
            self.rules.set(
                name,
                Instance {
                    node_kind: target.clone(),
                    source: target,
                    expression,
                    action,
                },
            );
        }
        Ok(())
    }
}

/// Loads `grammar` and every language it embeds.
pub(super) fn load(grammar: &Grammar, resolver: Resolver<'_>) -> LoadResult<Compiled> {
    let mut context = Context {
        resolver,
        programs: Vec::new(),
        languages: HashMap::new(),
    };
    let main = load_in_context(grammar, &mut context)?;
    Ok(Compiled {
        programs: context.programs,
        languages: context.languages,
        main,
    })
}

fn load_in_context(grammar: &Grammar, context: &mut Context<'_>) -> LoadResult<usize> {
    let resolved = resolve_imports(grammar, context, &[])?;
    let macros = &resolved.macros;
    let expand = |expr: &GrammarExpr| expand_macros(expr, macros, &mut Vec::new());

    let mut external = HashMap::new();
    for (index, scanner) in resolved.scanners.iter().enumerate() {
        for token in &scanner.tokens {
            if external.contains_key(token) {
                return refuse(
                    "declaration",
                    format!("external token {token} is declared twice"),
                );
            }
            if resolved.rules.contains(token) {
                return refuse(
                    "declaration",
                    format!("external token {token} is also a rule"),
                );
            }
            external.insert(token.clone(), index);
        }
    }
    let mut modes: HashSet<String> = HashSet::from(["default".to_owned()]);
    modes.extend(resolved.modes.iter().cloned());

    let mut sources = Ordered::new();
    for (name, (kind, expression, attributes)) in &resolved.rules.items {
        let action = match &attributes.action {
            Some(action) => Some(try_map_operations(action, expand)?),
            None => None,
        };
        sources.set(
            name.clone(),
            Source {
                kind: *kind,
                expression: expand(expression)?,
                action,
                attributes: attributes.clone(),
            },
        );
    }

    let mut instantiator = Instantiator {
        sources: &sources,
        external: &external,
        rules: Ordered::new(),
        pending: VecDeque::new(),
        queued: HashSet::new(),
    };
    for (name, source) in &sources.items {
        if !source.attributes.parameters.is_empty() {
            continue;
        }
        let owner = format!("rule {name}");
        let expression = instantiator.instantiate(&source.expression, None, &owner)?;
        let action = match &source.action {
            Some(action) => Some(instantiator.instantiate_operations(action, None, &owner)?),
            None => None,
        };
        instantiator.rules.set(
            name.clone(),
            Instance {
                node_kind: name.clone(),
                source: name.clone(),
                expression,
                action,
            },
        );
    }
    instantiator.drain()?;

    let start = resolved.start.clone();
    let Some(start_source) = start.as_deref().and_then(|name| sources.get(name)) else {
        return refuse(
            "reference",
            format!(
                "undefined start rule {}",
                start.as_deref().unwrap_or("null")
            ),
        );
    };
    if !start_source.attributes.parameters.is_empty() {
        return refuse(
            "parameter",
            format!(
                "start rule {} is parameterized",
                start.as_deref().unwrap_or_default()
            ),
        );
    }

    let mut extras = Vec::new();
    for extra in &resolved.extras {
        extras.push(instantiator.instantiate(&expand(extra)?, None, "an extra")?);
    }
    let mut scanner_operations = Vec::new();
    for scanner in &resolved.scanners {
        let expanded = try_map_operations(&scanner.operations, expand)?;
        let owner = format!("scanner {}", scanner.name);
        scanner_operations.push(instantiator.instantiate_operations(&expanded, None, &owner)?);
    }
    instantiator.drain()?;

    let mut conflicts = HashSet::new();
    for group in &resolved.conflicts {
        for name in group {
            if !sources.contains(name) {
                return refuse(
                    "declaration",
                    format!("conflict names undefined rule {name}"),
                );
            }
            conflicts.insert(name.clone());
        }
    }
    // The orders of named precedences (see `compare_precedence`).
    for order in &resolved.precedences {
        for entry in order {
            if let PrecedenceEntry::Rule(name) = entry
                && !sources.contains(name)
            {
                return refuse(
                    "declaration",
                    format!("precedences names undefined rule {name}"),
                );
            }
        }
    }

    let peg = resolved
        .matching
        .as_deref()
        .map_or(resolved.peg_source, |matching| matching == "peg");
    let longest = resolved.matching.as_deref() == Some("longest");
    let instances = instantiator.rules;
    let rule_index: HashMap<String, usize> = instances
        .items
        .iter()
        .enumerate()
        .map(|(index, (name, _))| (name.clone(), index))
        .collect();
    let node_kinds = instances
        .items
        .iter()
        .map(|(name, instance)| (name.clone(), Arc::from(instance.node_kind.as_str())))
        .collect();
    let mut compiler = Compiler::new(&rule_index, &external, &modes, node_kinds);
    // The items a scanner's `expected` asks about, each with an id, which
    // the parse requests where its rules try them (see `Expectations` in
    // executor.rs); a scanner that asks answers per context offset.
    let mut consults = Vec::new();
    for (scanner, operations) in resolved.scanners.iter().zip(&scanner_operations) {
        let mut items = Vec::new();
        if !scanner.tokens.is_empty() {
            expected_items(operations, &mut items);
        }
        consults.push(!items.is_empty());
        for item in items {
            if let Some(key) = compiler.expectation(item) {
                let id = compiler.expectations.len();
                compiler.expectations.entry(key).or_insert(id);
            }
        }
    }
    let expected_references = compiler
        .expectations
        .iter()
        .filter_map(|(key, &id)| Some((key.strip_prefix("ref ")?.to_owned(), id)))
        .collect();

    // Trivia: the `extra` expressions first, in declaration order, then every
    // rule on a channel other than `default`, each limited to its modes.
    let mut trivia = Vec::new();
    for extra in &extras {
        let expression = compiler.expression(extra, "an extra")?;
        let kind = match extra {
            GrammarExpr::NonTerminal(name) => Some(Arc::from(name.as_str())),
            _ => None,
        };
        trivia.push(Trivia {
            expression,
            kind,
            modes: None,
        });
    }
    let mut rules = Vec::new();
    for (index, (name, instance)) in instances.items.iter().enumerate() {
        let source = sources
            .get(&instance.source)
            .expect("an instance has a source");
        let owner = format!("rule {}", instance.node_kind);
        compiler.requesting = true;
        let expression = compiler.expression(&instance.expression, &owner);
        compiler.requesting = false;
        let expression = expression?;
        let action = match &instance.action {
            Some(action) => {
                if source.kind == RuleKind::Silent && action.iter().any(builds_node) {
                    return refuse(
                        "operation",
                        format!("silent {owner} has no node for its action"),
                    );
                }
                Some(compiler.statements(action, "action", &owner, None)?)
            }
            None => None,
        };
        let rule_modes = match &source.attributes.modes {
            Some(rule_modes) => {
                for mode in rule_modes {
                    compiler.check_mode(mode, &owner)?;
                }
                Some(
                    rule_modes
                        .iter()
                        .map(|mode| Arc::from(mode.as_str()))
                        .collect(),
                )
            }
            None => None,
        };
        let lexical_priority = form_of(&instance.expression, "lexicalPrecedence")
            .and_then(|form| form.integer("level"))
            .unwrap_or(0);
        let node_kind: Arc<str> = Arc::from(instance.node_kind.as_str());
        if source
            .attributes
            .channel
            .as_deref()
            .is_some_and(|channel| channel != "default")
        {
            trivia.push(Trivia {
                expression: super::program::Expr::Ref(Target::Rule(index)),
                kind: Some(node_kind.clone()),
                modes: rule_modes.clone(),
            });
        }
        rules.push(Rule {
            name: Arc::from(name.as_str()),
            node_kind,
            kind: source.kind,
            expression,
            action,
            modes: rule_modes,
            lexical_priority,
        });
    }
    let mut scanners = Vec::new();
    for ((scanner, operations), consults) in resolved
        .scanners
        .iter()
        .zip(&scanner_operations)
        .zip(consults)
    {
        let owner = format!("scanner {}", scanner.name);
        let operations = if scanner.tokens.is_empty() {
            Vec::new()
        } else {
            compiler.statements(
                operations,
                "scanner",
                &owner,
                Some(scanner.tokens.as_slice()),
            )?
        };
        scanners.push(Scanner {
            operations,
            consults,
        });
    }
    let embedded = compiler.into_embedded();

    let index = context.programs.len();
    let token_ranks = longest.then(|| token_ranks(&rules));
    context.programs.push(Program {
        peg,
        token_ranks,
        start,
        rules,
        rule_index,
        external,
        scanners,
        conflicts,
        precedence_orders: resolved.precedences.clone(),
        trivia,
        expected_references,
    });
    for language in embedded {
        if context.languages.contains_key(&language) {
            continue;
        }
        context.languages.insert(language.clone(), usize::MAX);
        let grammar = context.resolve(&language, "embed")?;
        let loaded = load_in_context(&grammar, context)?;
        context.languages.insert(language, loaded);
    }
    Ok(index)
}

/// The items of the `expected` conditions in a scanner's operations.
fn expected_items<'a>(operations: &'a [Operation], items: &mut Vec<&'a GrammarExpr>) {
    for operation in operations {
        if operation.head == "expected"
            && let Some(item) = operation.expression("item")
        {
            items.push(item);
        }
        for value in &operation.fields {
            match value {
                FieldValue::Operation(operation) => {
                    expected_items(std::slice::from_ref(operation), items);
                }
                FieldValue::Operations(block) | FieldValue::Block(Some(block)) => {
                    expected_items(block, items);
                }
                _ => {}
            }
        }
    }
}

fn builds_node(operation: &Operation) -> bool {
    if operation.head == "setAttribute" || operation.head == "buildNode" {
        return true;
    }
    ["consequent", "alternative", "body"].iter().any(|key| {
        operation
            .operations(key)
            .is_some_and(|block| block.iter().any(builds_node))
    })
}

/// The rank of each token of a `(matching longest)` grammar, by which two
/// tokens over the same text conflict as a lexer orders them: the higher
/// lexical precedence, then (at one length) a literal over a pattern, an
/// immediate token by one more, then the earlier token. Tokens are numbered
/// in rule order as they first appear: a token rule at its own definition, a
/// literal, an inline token and an alias of either where they are used; a
/// token rule's kind ranks as the rule even where an alias took it first. It
/// mirrors tokenRanks in js/src/grammar-runtime/load.js.
fn token_ranks(rules: &[Rule]) -> TokenRanks {
    let mut ranks = Ranking::default();
    let mut aliased = Vec::new();
    // A token rule ranks its own kind, over an alias of another token to it
    // that came first (Rust's `(alias identifier (literal default))`).
    let mut defined = HashSet::new();
    for rule in rules {
        if rule.kind != RuleKind::Token {
            ranks.walk(&rule.expression, &mut aliased);
        } else if defined.insert(rule.node_kind.clone()) {
            ranks.ranks.kinds.remove(rule.node_kind.as_ref());
            ranks.assign_kind(&rule.node_kind, rank_of(&rule.expression, 0));
        }
    }
    for (name, index) in aliased {
        if rules[index].kind == RuleKind::Token
            && !ranks.ranks.kinds.contains_key(name.as_ref())
            && let Some(rank) = ranks.ranks.kinds.get(&rules[index].node_kind).copied()
        {
            ranks.ranks.kinds.insert(name, rank);
        }
    }
    ranks.ranks
}

/// The ranks assigned so far and the next order.
#[derive(Default)]
struct Ranking {
    ranks: TokenRanks,
    order: usize,
}

impl Ranking {
    fn assign_kind(&mut self, kind: &Arc<str>, (priority, specificity): (i64, u8)) {
        if !self.ranks.kinds.contains_key(kind) {
            let order = self.next();
            self.ranks.kinds.insert(
                kind.clone(),
                TokenRank {
                    priority,
                    specificity,
                    order,
                },
            );
        }
    }

    fn assign_literal(&mut self, literal: &[u8], (priority, specificity): (i64, u8)) {
        if !self.ranks.literals.contains_key(literal) {
            let order = self.next();
            self.ranks.literals.insert(
                literal.to_vec(),
                TokenRank {
                    priority,
                    specificity,
                    order,
                },
            );
        }
    }

    const fn next(&mut self) -> usize {
        self.order += 1;
        self.order - 1
    }

    fn walk(&mut self, expr: &Expr, aliased: &mut Vec<(Arc<str>, usize)>) {
        match expr {
            Expr::Alias { name, item } if matches!(**item, Expr::Ref(_)) => {
                if let Expr::Ref(Target::Rule(index)) = **item {
                    aliased.push((name.clone(), index));
                }
            }
            Expr::Alias { name, item } if lexical(item) => self.assign_kind(name, rank_of(item, 0)),
            _ if lexical(expr) => {
                if let Some(literal) = bare(expr) {
                    self.assign_literal(literal, rank_of(expr, 0));
                }
            }
            Expr::Seq(items) | Expr::Choice { items, .. } | Expr::Longest(items) => {
                for item in items {
                    self.walk(item, aliased);
                }
            }
            Expr::Recover { item, synchronize } => {
                self.walk(item, aliased);
                self.walk(synchronize, aliased);
            }
            Expr::Repeat { item, .. }
            | Expr::And(item)
            | Expr::Not(item)
            | Expr::Capture { item, .. }
            | Expr::Alias { item, .. }
            | Expr::Precedence { item, .. }
            | Expr::DynamicPrecedence { item, .. }
            | Expr::LexicalPrecedence { item, .. }
            | Expr::Token(item)
            | Expr::ImmediateToken(item)
            | Expr::Predicate { item, .. }
            | Expr::Missing { item, .. }
            | Expr::Embed { item, .. } => self.walk(item, aliased),
            Expr::Empty | Expr::Terminal { .. } | Expr::Ref(_) => {}
        }
    }
}

/// A literal closed by lookaheads, such as a keyword, ranks as the literal.
fn closed(expr: &Expr) -> &Expr {
    if let Expr::Seq(items) = expr {
        let mut kept = items
            .iter()
            .filter(|item| !matches!(item, Expr::Not(_) | Expr::And(_)));
        if let (Some(only), None) = (kept.next(), kept.next()) {
            return closed(only);
        }
    }
    expr
}

/// The lexical precedence and the specificity of a token expression.
fn rank_of(expr: &Expr, priority: i64) -> (i64, u8) {
    match closed(expr) {
        Expr::LexicalPrecedence { level, item } => rank_of(item, *level),
        Expr::Token(item) => rank_of(item, priority),
        Expr::ImmediateToken(item) => {
            let (priority, specificity) = rank_of(item, priority);
            (priority, specificity + 1)
        }
        Expr::Terminal {
            matcher: Matcher::Literal(_),
            ..
        } => (priority, 2),
        _ => (priority, 0),
    }
}

fn lexical(expr: &Expr) -> bool {
    match expr {
        Expr::Terminal {
            matcher: Matcher::Literal(_),
            ..
        }
        | Expr::Token(_)
        | Expr::ImmediateToken(_) => true,
        Expr::LexicalPrecedence { item, .. } => lexical(item),
        _ => false,
    }
}

/// The literal a token expression is, under its token wrappers.
fn bare(expr: &Expr) -> Option<&[u8]> {
    match closed(expr) {
        Expr::Terminal {
            matcher: Matcher::Literal(literal),
            ..
        } => Some(literal),
        Expr::LexicalPrecedence { item, .. } | Expr::Token(item) | Expr::ImmediateToken(item) => {
            bare(item)
        }
        _ => None,
    }
}
