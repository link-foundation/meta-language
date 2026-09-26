//! State shared by the emitters.
//!
//! The state holds target names for every declaration, the source-to-target
//! mapping, and the encodings and assumptions a translation relies on. Every
//! emitter records what it did here, so the translation contract lists exactly
//! the choices that were made for this program.
//!
//! Mirrors `js/src/translation/emit-common.js`.

use std::collections::{BTreeSet, HashMap, HashSet};

use serde::{Deserialize, Serialize};

use super::diagnostics::{unsupported, Result};
use super::ir::{Decl, Effect, Expr, Node, Pattern, Plan, Program, Prop};
use super::types::Type;
use super::{Language, Span};

/// A fixed assumption text, so that contracts compare across runtimes.
#[derive(Clone, Copy, Debug)]
pub struct AssumptionText {
    pub id: &'static str,
    pub statement: &'static str,
}

pub const NON_ABORTING: AssumptionText = AssumptionText {
    id: "non-aborting-executions",
    statement: "the translation agrees with the source on executions that do not abort; the source aborts on machine-integer overflow, checked conversion failure, division by zero or an explicit panic, and the target computes an unspecified value there instead",
};

/// Where a source declaration went in the target.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mapping {
    pub kind: String,
    pub source: String,
    pub target: String,
    pub source_span: Option<Span>,
}

/// An assumption the translation relies on, with the program-specific details.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assumption {
    pub id: String,
    pub statement: String,
    pub details: Vec<String>,
}

/// A representation choice the translation made.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Encoding {
    pub id: String,
    pub statement: String,
}

/// A theorem or assertion the target states, and how it is discharged.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TheoremRecord {
    pub source: String,
    pub target: String,
    pub kind: String,
    pub closed_goal: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discharge: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check: Option<String>,
}

/// An emitted target program together with its translation contract.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Emitted {
    pub language: Language,
    pub text: String,
    pub mappings: Vec<Mapping>,
    pub assumptions: Vec<Assumption>,
    pub encodings: Vec<Encoding>,
    pub theorems: Vec<TheoremRecord>,
    pub entry: Option<String>,
}

/// How constructors are named: inside a module next to their type (Rocq), or under the type.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CtorStyle {
    Module,
    #[default]
    Data,
}

/// Target naming rules for [`EmitState`].
#[derive(Clone, Copy, Default)]
#[allow(clippy::struct_excessive_bools)] // the flags mirror independent target namespaces
pub struct EmitOptions {
    pub ctor_style: CtorStyle,
    pub type_name: Option<fn(&str) -> String>,
    pub value_name: Option<fn(&str) -> String>,
    pub ctor_name: Option<fn(&str) -> String>,
    pub module_segment: Option<fn(&str) -> String>,
    /// Names the target itself derives from a declaration (Rocq's `f_equation`, `T_ind`).
    pub generated: Option<fn(&str) -> Vec<String>>,
    pub type_space: bool,
    pub modules_share_term_space: bool,
    pub modules_share_type_space: bool,
    /// Names root declarations may not take because the target's prelude has them.
    pub root_reserved: &'static [&'static str],
}

struct Target {
    local: String,
    path: Vec<String>,
}

pub struct EmitState<'p> {
    pub program: &'p Program,
    pub language: Language,
    pub ident: fn(&str) -> String,
    reserved: BTreeSet<String>,
    options: EmitOptions,
    names: HashMap<String, Target>,
    ctor_names: HashMap<String, String>,
    module_names: HashMap<String, String>,
    taken: HashMap<String, HashSet<String>>,
    claimed: BTreeSet<String>,
    pub mappings: Vec<Mapping>,
    assumptions: Vec<Assumption>,
    encodings: Vec<Encoding>,
    pub theorems: Vec<TheoremRecord>,
}

impl<'p> EmitState<'p> {
    #[must_use]
    pub fn new(
        program: &'p Program,
        language: Language,
        ident: fn(&str) -> String,
        reserved: &[&str],
        options: EmitOptions,
    ) -> Self {
        let mut state = Self {
            program,
            language,
            ident,
            reserved: reserved.iter().map(|name| (*name).to_owned()).collect(),
            options,
            names: HashMap::new(),
            ctor_names: HashMap::new(),
            module_names: HashMap::new(),
            taken: HashMap::new(),
            claimed: BTreeSet::new(),
            mappings: Vec::new(),
            assumptions: Vec::new(),
            encodings: Vec::new(),
            theorems: Vec::new(),
        };
        state.assign();
        state
    }

    /// Target names are fixed up front, so references never depend on emission order.
    fn assign(&mut self) {
        let module_name = self.options.module_segment.unwrap_or(self.ident);
        // JavaScript modules are values; Rust modules share the type namespace.
        let module_space = if self.options.modules_share_term_space {
            "term"
        } else if self.options.modules_share_type_space {
            "type"
        } else {
            "module"
        };
        let program = self.program;
        for entry in &program.declarations {
            let module_path = entry.module_path();
            let mut segments = Vec::new();
            for (index, segment) in module_path.iter().enumerate() {
                let key = module_path[..=index].join(".");
                if !self.module_names.contains_key(&key) {
                    let parent = module_path[..index].join(".");
                    let name =
                        self.claim(&format!("{module_space}:{parent}"), &module_name(segment));
                    self.module_names.insert(key.clone(), name);
                }
                segments.push(self.module_names[&key].clone());
            }
            let space = format!("term:{}", module_path.join("."));
            let base = match entry {
                Decl::Data(_) => self.options.type_name.unwrap_or(self.ident)(entry.name()),
                _ => self.options.value_name.unwrap_or(self.ident)(entry.name()),
            };
            let local_space = if matches!(entry, Decl::Data(_)) && self.options.type_space {
                format!("type:{}", module_path.join("."))
            } else {
                space.clone()
            };
            let local = self.claim(&local_space, &base);
            self.names.insert(
                entry.full_name().to_owned(),
                Target {
                    local,
                    path: segments,
                },
            );
            if let Decl::Data(data) = entry {
                for ctor in &data.ctors {
                    let ctor_base = self.options.ctor_name.unwrap_or(self.ident)(&ctor.name);
                    let ctor_space = if self.options.ctor_style == CtorStyle::Module {
                        space.clone()
                    } else {
                        format!("ctor:{}", data.full_name)
                    };
                    let name = self.claim(&ctor_space, &ctor_base);
                    self.ctor_names
                        .insert(format!("{}#{}", data.full_name, ctor.name), name);
                }
            }
        }
    }

    fn claim(&mut self, space: &str, name: &str) -> String {
        if !self.taken.contains_key(space) {
            // Root declarations may not redeclare the target's prelude (`root_reserved`).
            let mut used: HashSet<String> = self.reserved.iter().cloned().collect();
            if space.ends_with(':') {
                used.extend(
                    self.options
                        .root_reserved
                        .iter()
                        .map(|name| (*name).to_owned()),
                );
            }
            self.taken.insert(space.to_owned(), used);
        }
        let generated = self.options.generated;
        let used = self
            .taken
            .get_mut(space)
            .expect("namespace was just created");
        let is_generated = |candidate: &str, used: &HashSet<String>| {
            generated.is_some_and(|generated| {
                generated(candidate).iter().any(|name| used.contains(name))
            })
        };
        let mut candidate = name.to_owned();
        let mut index = 2;
        while used.contains(&candidate) || is_generated(&candidate, used) {
            candidate = format!("{name}_{index}");
            index += 1;
        }
        let mut names = vec![candidate.clone()];
        if let Some(generated) = generated {
            names.extend(generated(&candidate));
        }
        for name in names {
            used.insert(name.clone());
            self.claimed.insert(name);
        }
        candidate
    }

    /// Names a local binder may not take: a local named like a module, a
    /// top-level function or a constructor would shadow it in the target.
    #[must_use]
    pub fn local_reserved(&self) -> BTreeSet<String> {
        self.reserved.union(&self.claimed).cloned().collect()
    }

    /// The target name of a declaration within its module.
    ///
    /// # Panics
    /// When `full_name` is not a declaration of the program.
    #[must_use]
    pub fn local_name(&self, full_name: &str) -> &str {
        &self.names[full_name].local
    }

    /// The target module path of a declaration.
    ///
    /// # Panics
    /// When `full_name` is not a declaration of the program.
    #[must_use]
    pub fn module_path(&self, full_name: &str) -> &[String] {
        &self.names[full_name].path
    }

    /// The target name of a source module.
    #[must_use]
    pub fn module_name(&self, source_path: &[String]) -> Option<&str> {
        self.module_names
            .get(&source_path.join("."))
            .map(String::as_str)
    }

    /// Fully qualified reference from the top level.
    #[must_use]
    pub fn reference(&self, full_name: &str, separator: &str) -> String {
        let target = &self.names[full_name];
        let mut parts = target.path.clone();
        parts.push(target.local.clone());
        parts.join(separator)
    }

    /// The target name of a constructor within its namespace.
    ///
    /// # Panics
    /// When the constructor is not declared.
    #[must_use]
    pub fn ctor_local(&self, data_name: &str, ctor: &str) -> &str {
        &self.ctor_names[&format!("{data_name}#{ctor}")]
    }

    /// Fully qualified constructor reference from the top level.
    #[must_use]
    pub fn ctor_ref(&self, data_name: &str, ctor: &str, separator: &str) -> String {
        let local = self.ctor_local(data_name, ctor);
        if self.options.ctor_style == CtorStyle::Module {
            let mut parts = self.module_path(data_name).to_vec();
            parts.push(local.to_owned());
            return parts.join(separator);
        }
        format!("{}{separator}{local}", self.reference(data_name, separator))
    }

    /// Records where a declaration (and a data type's constructors) went.
    pub fn map(&mut self, entry: &Decl, target: &str) {
        let source_span = entry.span();
        let mut path = self.module_path(entry.full_name()).to_vec();
        path.push(target.to_owned());
        self.mappings.push(Mapping {
            kind: match entry {
                Decl::Fn(_) => "function",
                other => other.kind(),
            }
            .to_owned(),
            source: entry.full_name().to_owned(),
            target: path.join("."),
            source_span,
        });
        if let Decl::Data(data) = entry {
            for ctor in &data.ctors {
                self.mappings.push(Mapping {
                    kind: "constructor".to_owned(),
                    source: format!("{}.{}", data.full_name, ctor.name),
                    target: self.ctor_ref(&data.full_name, &ctor.name, "."),
                    source_span,
                });
            }
        }
    }

    pub fn assume(&mut self, assumption: AssumptionText, detail: Option<&str>) {
        let index = if let Some(index) = self
            .assumptions
            .iter()
            .position(|record| record.id == assumption.id)
        {
            index
        } else {
            self.assumptions.push(Assumption {
                id: assumption.id.to_owned(),
                statement: assumption.statement.to_owned(),
                details: Vec::new(),
            });
            self.assumptions.len() - 1
        };
        let record = &mut self.assumptions[index];
        if let Some(detail) = detail {
            if !record.details.iter().any(|known| known == detail) {
                record.details.push(detail.to_owned());
            }
        }
    }

    pub fn fixed_to_unbounded(&mut self, ty: &Type) {
        let key = ty.key();
        let unbounded = if matches!(ty, Type::Fixed { signed: true, .. }) {
            "integers"
        } else {
            "naturals"
        };
        self.encode(
            &format!("machine-integer:{key}"),
            &format!("{key} values are represented by the target's unbounded {unbounded}; they agree while no operation overflows"),
        );
        self.assume(
            NON_ABORTING,
            Some(&format!("{key} arithmetic stays in range")),
        );
    }

    pub fn checked_to_total(&mut self, operation: &str) {
        self.assume(
            NON_ABORTING,
            Some(&format!("checked {operation} does not fail")),
        );
    }

    pub fn abort_to_total(&mut self, message: &str) {
        self.assume(NON_ABORTING, Some(&format!("no abort: {message}")));
    }

    pub fn encode(&mut self, id: &str, statement: &str) {
        if !self.encodings.iter().any(|encoding| encoding.id == id) {
            self.encodings.push(Encoding {
                id: id.to_owned(),
                statement: statement.to_owned(),
            });
        }
    }

    /// Records a theorem; `bounded` theorems are checked by the source
    /// kernel and tested on bounded inputs in the target.
    pub fn theorem(&mut self, full_name: &str, name: &str, closed_goal: bool, bounded: bool) {
        let mut path = self.module_path(full_name).to_vec();
        path.push(name.to_owned());
        self.theorems.push(TheoremRecord {
            source: full_name.to_owned(),
            target: path.join("."),
            kind: "theorem".to_owned(),
            closed_goal,
            discharge: bounded.then(|| "source-kernel".to_owned()),
            check: bounded.then(|| "bounded".to_owned()),
        });
    }

    pub fn assertion_theorem(&mut self, name: &str, effect: &Effect) {
        let span = match effect {
            Effect::Print { span, .. } | Effect::Let { span, .. } | Effect::Assert { span, .. } => {
                *span
            }
        };
        self.theorems.push(TheoremRecord {
            source: format!(
                "main:assert@{}",
                span.map_or_else(|| "?".to_owned(), |span| span.start.to_string())
            ),
            target: name.to_owned(),
            kind: "assertion".to_owned(),
            closed_goal: true,
            discharge: None,
            check: None,
        });
    }

    #[must_use]
    pub fn assumption_list(&self) -> Vec<Assumption> {
        self.assumptions.clone()
    }

    #[must_use]
    pub fn encoding_list(&self) -> Vec<Encoding> {
        self.encodings.clone()
    }

    /// The emitted result with this state's contract.
    #[must_use]
    pub fn finish(&self, text: String, entry: Option<String>) -> Emitted {
        Emitted {
            language: self.language,
            text,
            mappings: self.mappings.clone(),
            assumptions: self.assumption_list(),
            encodings: self.encoding_list(),
            theorems: self.theorems.clone(),
            entry,
        }
    }
}

fn type_dependencies(ty: &Type, found: &mut HashSet<String>) {
    if let Type::Data { name } = ty {
        found.insert(name.clone());
    }
}

fn expr_dependencies(expr: &Expr, found: &mut HashSet<String>) {
    type_dependencies(&expr.ty, found);
    match &expr.node {
        Node::Call { func, .. } => {
            found.insert(func.clone());
        }
        Node::Ctor { data, .. } => {
            found.insert(data.clone());
        }
        Node::Binary {
            domain: Some(domain),
            ..
        } => type_dependencies(domain, found),
        Node::Cast { from, to, .. } => {
            type_dependencies(from, found);
            type_dependencies(to, found);
        }
        Node::Match { cases, .. } => {
            for kase in cases {
                if let Pattern::Ctor { data, .. } = &kase.pattern {
                    found.insert(data.clone());
                }
            }
        }
        _ => {}
    }
    for child in expr.children() {
        expr_dependencies(child, found);
    }
}

fn prop_dependencies(prop: &Prop, found: &mut HashSet<String>) {
    match prop {
        Prop::Forall { binders, body } => {
            for binder in binders {
                type_dependencies(&binder.ty, found);
            }
            prop_dependencies(body, found);
        }
        Prop::And { left, right } | Prop::Or { left, right } | Prop::Implies { left, right } => {
            prop_dependencies(left, found);
            prop_dependencies(right, found);
        }
        Prop::Not { arg } => prop_dependencies(arg, found),
        Prop::Bool { expr } => expr_dependencies(expr, found),
        other => {
            if let Some((_, comparison)) = other.comparison() {
                expr_dependencies(&comparison.left, found);
                expr_dependencies(&comparison.right, found);
                type_dependencies(&comparison.domain, found);
            }
        }
    }
}

/// Declarations every declaration refers to (types, functions, constructors, lemmas).
#[must_use]
pub fn dependencies(entry: &Decl) -> HashSet<String> {
    fn lemmas(plan: &Plan, found: &mut HashSet<String>) {
        match plan {
            Plan::Close { hints } => {
                found.extend(hints.lemmas.iter().cloned());
                found.extend(hints.unfold.iter().cloned());
            }
            Plan::Induction(split) | Plan::Cases(split) => {
                for kase in &split.cases {
                    lemmas(&kase.plan, found);
                }
            }
        }
    }
    let mut found = HashSet::new();
    match entry {
        Decl::Data(data) => {
            for ctor in &data.ctors {
                for field in &ctor.fields {
                    type_dependencies(&field.ty, &mut found);
                }
            }
        }
        Decl::Fn(function) => {
            for param in &function.params {
                type_dependencies(&param.ty, &mut found);
            }
            type_dependencies(&function.ret, &mut found);
            expr_dependencies(&function.body, &mut found);
        }
        Decl::Theorem(theorem) => {
            for binder in &theorem.binders {
                type_dependencies(&binder.ty, &mut found);
            }
            prop_dependencies(&theorem.prop, &mut found);
            lemmas(&theorem.proof.plan, &mut found);
        }
    }
    found.remove(entry.full_name());
    found
}

/// Declarations in an order where each follows everything it uses.
///
/// Source order is kept where possible and, among ready declarations, the
/// ones in the module being emitted come first so modules stay contiguous.
///
/// # Errors
/// On cyclic declarations, and on modules that would have to be reopened
/// when `require_contiguous_modules` is set.
pub fn order_declarations(
    program: &Program,
    require_contiguous_modules: bool,
) -> Result<Vec<&Decl>> {
    let entries = &program.declarations;
    let position: HashMap<&str, usize> = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| (entry.full_name(), index))
        .collect();
    let pending: Vec<HashSet<String>> = entries.iter().map(dependencies).collect();
    let mut done: HashSet<&str> = HashSet::new();
    let mut order: Vec<&Decl> = Vec::new();
    let mut current: &[String] = &[];
    while order.len() < entries.len() {
        let mut ready: Vec<usize> = (0..entries.len())
            .filter(|&index| {
                !done.contains(entries[index].full_name())
                    && pending[index].iter().all(|name| {
                        done.contains(name.as_str()) || !position.contains_key(name.as_str())
                    })
            })
            .collect();
        if ready.is_empty() {
            let stuck = entries
                .iter()
                .find(|entry| !done.contains(entry.full_name()))
                .expect("some declaration is pending");
            return Err(unsupported(
                "cyclic declarations",
                &format!(
                    "{} depends on declarations that depend on it, which needs mutual definitions",
                    stuck.full_name()
                ),
                stuck.span(),
            ));
        }
        let shared = |index: usize| {
            let path = entries[index].module_path();
            current
                .iter()
                .zip(path)
                .take_while(|(left, right)| left == right)
                .count()
        };
        ready.sort_by(|&left, &right| shared(right).cmp(&shared(left)).then(left.cmp(&right)));
        let next = &entries[ready[0]];
        order.push(next);
        done.insert(next.full_name());
        current = next.module_path();
    }
    if require_contiguous_modules {
        let mut closed: HashSet<String> = HashSet::new();
        let mut open: &[String] = &[];
        for entry in &order {
            let path = entry.module_path();
            for depth in 0..open.len() {
                if path.get(depth) != Some(&open[depth]) {
                    for index in depth..open.len() {
                        closed.insert(open[..=index].join("."));
                    }
                    break;
                }
            }
            for depth in 1..=path.len() {
                let key = path[..depth].join(".");
                if closed.contains(&key) {
                    return Err(unsupported(
                        "interleaved modules",
                        &format!("module {key} would have to be reopened after its declarations depend on later ones; the target cannot reopen a module"),
                        entry.span(),
                    ));
                }
            }
            open = path;
        }
    }
    Ok(order)
}
