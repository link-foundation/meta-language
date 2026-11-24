//! The Lean 4 emitter.
//!
//! Naturals are `Nat`, integers `Int`; machine integers are represented by
//! `Nat`/`Int` with explicit range checks that `panic!` where Rust would
//! panic. Recursion is structural and annotated as such, so the Lean kernel
//! checks termination. Theorems are reconstructed from portable proof plans
//! with Lean tactics and re-checked by the Lean kernel. A JavaScript Number
//! is a `Float`; the kernel cannot compute with `Float`, so an assertion about
//! Numbers is checked when `main` runs, as in the source.
//!
//! Mirrors `js/src/translation/emit-lean.js`.

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;

use super::Language;
use super::diagnostics::{Result, type_error, unsupported};
use super::emit_common::{
    CtorStyle, Dependence, EmitOptions, EmitState, Emitted, datatype_groups, dependencies,
    mutual_groups, order_declarations_with_dependencies, unthreaded,
};
use super::ir::{
    ByZero, Case, DataDecl, Decl, Expr, FnDecl, Hints, LitValue, Node, Pattern, Plan, Program,
    Prop, Semantics, TheoremDecl, rename_function, rename_theorem,
};
use super::lean_root_names::LEAN_ROOT_NAMES;
use super::output::thread_output_for_target;
use super::proof::prop_functions;
use super::surface::{BinaryOp, Flavor, Rounding, UnaryOp};
use super::typed_program_link_rules::accept_program_tuple_product_model;
use super::types::Type;

mod arrays;
mod callables;
mod casts;
mod helpers;
mod main;
mod math;
mod optional_values;
mod records;
mod types;
mod unit_strings;
use self::helpers::HELPERS;

const KEYWORDS: &[&str] = &[
    "MLStringNumber",
    "MLBigIntegerConversion",
    "MLUtf16Text",
    "MLUnicodeCase",
    "MLRecordPropertyOrder",
    "abbrev",
    "at",
    "attribute",
    "axiom",
    "by",
    "calc",
    "class",
    "def",
    "deriving",
    "do",
    "else",
    "end",
    "example",
    "export",
    "extends",
    "for",
    "from",
    "fun",
    "have",
    "if",
    "import",
    "in",
    "inductive",
    "instance",
    "let",
    "local",
    "match",
    "mut",
    "mutual",
    "namespace",
    "noncomputable",
    "open",
    "partial",
    "private",
    "protected",
    "return",
    "section",
    "set_option",
    "show",
    "structure",
    "suffices",
    "then",
    "theorem",
    "lemma",
    "unsafe",
    "universe",
    "variable",
    "where",
    "with",
    "termination_by",
    "decreasing_by",
    "macro",
    "syntax",
    "notation",
    "infix",
    "infixl",
    "infixr",
    "prefix",
    "postfix",
    "macro_rules",
    "elab",
    "opaque",
    "omit",
    "include",
    "nomatch",
    "nofun",
    "try",
    "catch",
    "finally",
    "unless",
    "break",
    "continue",
    "Type",
    "Prop",
    "Sort",
    "this",
    "using",
    "obtain",
    // Names the emitted code relies on, and names Lean derives inside a type's namespace.
    "Nat",
    "Int",
    "String",
    "Bool",
    "Unit",
    "IO",
    "true",
    "false",
    "main",
    "toString",
    "decide",
    "panic",
    "Float",
    "rec",
    "recOn",
    "casesOn",
    "noConfusion",
    "noConfusionType",
    "below",
    "brecOn",
    "binductionOn",
    "ibelow",
    "ctorIdx",
    "toCtorIdx",
    "sizeOf_spec",
    "injEq",
    "inj",
    "induct",
    "eq_def",
    "mk",
];

/// A legal Lean identifier for a source name.
fn ident(name: &str) -> String {
    let mut result: String = name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric()
                || character == '_'
                || character == '\''
                || ('\u{C0}'..='\u{FFFF}').contains(&character)
            {
                character
            } else {
                '_'
            }
        })
        .collect();
    if result.starts_with(|character: char| character.is_ascii_digit() || character == '\'') {
        result = format!("x{result}");
    }
    if KEYWORDS.contains(&result.as_str()) {
        result = format!("«{result}»");
    }
    result
}

/// Lean derives the equation lemma `f.eq_1` for every definition `f`.
fn generated(name: &str) -> Vec<String> {
    vec![format!("{name}.eq_1")]
}

/// Emits a checked program as Lean.
///
/// # Errors
/// On constructs the target cannot express faithfully.
pub fn emit_lean(source: &Program) -> Result<Emitted> {
    super::class_owners::validate_program(source)?;
    let prepared = super::ordered_record_preparation::prepare(source)?;
    let source = super::output::local_cells::prepare(prepared.as_ref());
    let source = super::output::reference_collections::prepare(source.as_ref());
    let program = &*thread_output_for_target(source.as_ref(), "Lean")?;
    let utf16_strings = super::formal_unit_strings::requires(program);
    let mut state = EmitState::new(
        program,
        Language::Lean,
        ident,
        KEYWORDS,
        EmitOptions {
            ctor_style: CtorStyle::Data,
            generated: Some(generated),
            root_reserved: LEAN_ROOT_NAMES,
            ..EmitOptions::default()
        },
    );
    let plan = super::formal_datatype_modules::plan_formal_datatype_modules(program)?;
    let aliases =
        super::formal_datatype_aliases::capture_formal_datatype_aliases(&plan.aliases, &state)?;
    let alias_dependencies =
        super::formal_datatype_modules::formal_datatype_module_dependencies(&plan)?;
    state.relocate_datatypes(&plan.program, &plan.aliases)?;
    let program = &plan.program;
    LeanEmitter {
        program,
        state,
        aliases: super::formal_datatype_aliases::FormalDatatypeAliases::new(aliases, "Lean"),
        alias_dependencies,
        helpers: if utf16_strings {
            HashSet::from(["utf16Text"])
        } else {
            HashSet::new()
        },
        utf16_strings,
        successors: HashMap::new(),
        uses_number: Dependence::numbers(program),
        uses_partial: Dependence::partial(program),
    }
    .file()
}

struct LeanEmitter<'p> {
    utf16_strings: bool,
    program: &'p Program,
    state: EmitState<'p>,
    aliases: super::formal_datatype_aliases::FormalDatatypeAliases,
    alias_dependencies: HashMap<String, HashSet<String>>,
    helpers: HashSet<&'static str>,
    // Inside `| p + 1 =>` of a match on `x`, `x` is written `p + 1`: Lean's
    // structural recursion sees through the pattern but not the variable.
    successors: HashMap<String, String>,
    uses_number: Dependence,
    uses_partial: Dependence,
}

impl LeanEmitter<'_> {
    fn file(mut self) -> Result<Emitted> {
        if self.utf16_strings {
            self.state.encode("utf16-formal-strings","source strings are ordered UTF-16 code-unit columns represented by natural numbers 0..65535; generated complete shared operations retain isolated units, and scalar display alone replaces unpaired surrogates");
        }
        let mut blocks = Vec::new();
        blocks.extend(self.aliases.open(&[], &self.state)?);
        let mut open: Vec<String> = Vec::new();
        let groups = mutual_groups(self.program);
        let data_groups = datatype_groups(self.program)?;
        for entry in order_declarations_with_dependencies(
            self.program,
            self.aliases.has_aliases(),
            &self.alias_dependencies,
        )? {
            self.move_to(&mut open, entry.module_path(), &mut blocks)?;
            let block = match (entry, groups.get(entry.full_name())) {
                (_, Some(group)) if std::ptr::eq(group[0], entry) => self.mutual(group)?,
                (_, Some(_)) => continue,
                (Decl::Data(data), None) => {
                    if let Some(group) = data_groups.get(entry.full_name()) {
                        if std::ptr::eq(group[0], entry) {
                            self.mutual_data(group)?
                        } else {
                            continue;
                        }
                    } else {
                        self.data(entry, data)?
                    }
                }
                (Decl::Fn(function), None) => self.function(entry, function)?,
                (Decl::Theorem(theorem), None) => self.theorem(entry, theorem)?,
            };
            if let Some(group) = data_groups.get(entry.full_name()) {
                self.aliases.complete(group);
            } else {
                self.aliases.complete(&[entry]);
            }
            blocks.push(block);
        }
        self.move_to(&mut open, &[], &mut blocks)?;
        blocks.extend(self.aliases.close(&[], &self.state)?);
        self.aliases.finish()?;
        let main = match &self.program.main {
            Some(main) => Some(self.main(main)?),
            None => None,
        };
        let mut lines = vec![
            format!(
                "-- Translated from {} by meta-language: portable core, Lean target.",
                self.program.source_language.as_str()
            ),
            "-- Source binders are kept even where unused or named like a constructor, and proof hints are shared by every closing tactic.".to_owned(),
            "set_option linter.unusedVariables false".to_owned(),
            "set_option linter.unusedSimpArgs false".to_owned(),
            "set_option linter.constructorNameAsVariable false".to_owned(),
            String::new(),
        ];
        for (name, text) in HELPERS {
            if self.helpers.contains(name) {
                lines.push(if name == "stringIncludes" {
                    crate::translation::frontend_rules::read_string_test_support("Lean", "includes")
                } else {
                    text.to_owned()
                });
                lines.push(String::new());
            }
        }
        if self.helpers.contains("stringNumber") {
            lines.push(include_str!("runtime/string-number.lean").to_owned());
            lines.push(String::new());
        }
        if self.helpers.contains("bigIntegerConversion") {
            lines.push(include_str!("runtime/big-integer-conversion.lean").to_owned());
            lines.push(String::new());
        }
        if self.helpers.contains("utf16Text") {
            lines.push(include_str!("runtime/utf16-text.lean").to_owned());
            lines.push(String::new());
        }
        if self.helpers.contains("unicodeCase") {
            lines.push(include_str!("runtime/unicode-case.lean").to_owned());
            lines.push(String::new());
        }
        if self.helpers.contains("recordPropertyOrder") {
            lines.push(include_str!("runtime/record-property-order.lean").to_owned());
            lines.push(String::new());
        }
        for block in blocks {
            lines.push(block);
            lines.push(String::new());
        }
        let entry = main.map(|main| {
            lines.push(main);
            lines.push(String::new());
            "main".to_owned()
        });
        Ok(self.state.finish(lines.join("\n"), entry))
    }

    /// Closes and opens namespaces so that `path` is the open one.
    fn move_to(
        &mut self,
        open: &mut Vec<String>,
        path: &[String],
        blocks: &mut Vec<String>,
    ) -> Result<()> {
        let common = open
            .iter()
            .zip(path)
            .take_while(|(left, right)| left == right)
            .count();
        while open.len() > common {
            blocks.extend(self.aliases.close(open, &self.state)?);
            blocks.push(format!("end {}", self.module_name(open)));
            open.pop();
        }
        while open.len() < path.len() {
            open.push(path[open.len()].clone());
            blocks.push(format!("namespace {}", self.module_name(open)));
            blocks.extend(self.aliases.open(open, &self.state)?);
        }
        Ok(())
    }

    fn module_name(&self, path: &[String]) -> String {
        self.state.module_name(path).unwrap_or_default().to_owned()
    }

    fn data(&mut self, entry: &Decl, data: &DataDecl) -> Result<String> {
        self.data_definition(entry, data, false)
    }
    fn data_definition(&mut self, entry: &Decl, data: &DataDecl, mutual: bool) -> Result<String> {
        let facts = crate::translation::callable_data::read_callable_data_facts(
            &Type::Data {
                name: data.full_name.clone(),
            },
            &|name| match self.program.declaration(name) {
                Some(Decl::Data(data)) => Some(data),
                _ => None,
            },
        )?;
        let profile = if facts.callable && !mutual {
            Some(
                crate::translation::callable_data::read_callable_data_recursive_profile(
                    &Type::Data {
                        name: data.full_name.clone(),
                    },
                    &|name| match self.program.declaration(name) {
                        Some(Decl::Data(data)) => Some(data),
                        _ => None,
                    },
                )?,
            )
        } else {
            None
        };
        let self_profile =
            if !mutual && !facts.callable && dependencies(entry).contains(&data.full_name) {
                crate::translation::callable_data::read_formal_datatype_constructive_profiles(
                    &Type::Data {
                        name: data.full_name.clone(),
                    },
                    &|name| match self.program.declaration(name) {
                        Some(Decl::Data(data)) => Some(data),
                        _ => None,
                    },
                )?
                .remove(&data.full_name)
            } else {
                None
            };
        let name = self.state.local_name(&data.full_name).to_owned();
        self.state.map(entry, &name);
        let mut lines = vec![format!("inductive {name} where")];
        for ctor in &data.ctors {
            let mut fields = String::new();
            for field in &ctor.fields {
                let ty = self.ty(&field.ty)?;
                let _ = write!(fields, " ({} : {ty})", ident(&field.name));
            }
            let local = self.state.ctor_local(&data.full_name, &ctor.name);
            lines.push(format!("  | {local}{fields} : {name}"));
        }
        // A pair of printed lines and a value is only built and taken apart; its value may be a Float, which has no DecidableEq.
        // Lean derives no DecidableEq for a Float, nor through an Array of data; the portable core never compares such values.
        let undecidable = self.undecidable(&data.full_name, &mut HashSet::new());
        let deriving = if mutual || self_profile.is_some() {
            crate::translation::callable_data_derivation_rules::read_formal_datatype_lean_block_derivation(facts.callable,data.output,undecidable)
        } else {
            crate::translation::callable_data_derivation_rules::read_callable_data_derivation(
                "Lean",
                facts.callable,
                false,
                data.output,
                undecidable,
            )
        };
        if let Some(profile) = profile.filter(|profile| profile.recursive) {
            let witness = &data.ctors[profile.witness];
            let constructor = self.state.ctor_ref(&data.full_name, &witness.name, ".");
            lines.push(crate::translation::callable_data_derivation_rules::render_callable_data_lean_default(&name,&constructor,witness.fields.len() as f64));
        } else if !deriving.is_empty() {
            lines.push(format!("  deriving {deriving}"));
        }
        if let Some(profile) = self_profile {
            let witness = &data.ctors[profile.witness];
            let constructor = self.state.ctor_ref(&data.full_name, &witness.name, ".");
            lines.push(crate::translation::callable_data_derivation_rules::render_callable_data_lean_default(&name,&constructor,witness.fields.len() as f64));
        }
        Ok(lines.join("\n"))
    }
    fn mutual_data(&mut self, group: &[&Decl]) -> Result<String> {
        for entry in group {
            let ty = Type::Data {
                name: entry.full_name().to_owned(),
            };
            let lookup = |name: &str| match self.program.declaration(name) {
                Some(Decl::Data(data)) => Some(data),
                _ => None,
            };
            if crate::translation::callable_data::read_callable_data_facts(&ty, &lookup)?.callable {
                crate::translation::callable_data::read_callable_data_recursive_profile(
                    &ty, &lookup,
                )?;
            }
        }
        let profiles =
            crate::translation::callable_data::read_formal_datatype_constructive_profiles(
                &Type::Data {
                    name: group[0].full_name().to_owned(),
                },
                &|name| match self.program.declaration(name) {
                    Some(Decl::Data(data)) => Some(data),
                    _ => None,
                },
            )?;
        let mut lines = vec!["mutual".to_owned()];
        for entry in group {
            let Decl::Data(data) = entry else {
                return Err(unsupported(
                    "formal mutual datatype",
                    "the complete component needs actual data declarations",
                    None,
                ));
            };
            lines.push(self.data_definition(entry, data, true)?);
        }
        lines.push("end".to_owned());
        let mut defaults = group.to_vec();
        defaults.sort_by_key(|entry| profiles[entry.full_name()].rank);
        for entry in defaults {
            let Decl::Data(data) = entry else {
                unreachable!("validated actual data component")
            };
            let witness = &data.ctors[profiles[entry.full_name()].witness];
            lines.push(crate::translation::callable_data_derivation_rules::render_callable_data_lean_default(self.state.local_name(&data.full_name),&self.state.ctor_ref(&data.full_name,&witness.name,"."),witness.fields.len() as f64));
        }
        Ok(lines.join("\n"))
    }

    fn definition(&mut self, entry: &Decl, function: &FnDecl) -> Result<String> {
        let (params, body) = rename_function(function, &ident, &self.state.local_reserved());
        let name = self.state.local_name(&function.full_name).to_owned();
        self.state.map(entry, &name);
        let mut binders = String::new();
        for param in &params {
            let ty = self.ty(&param.ty)?;
            let _ = write!(binders, " ({} : {ty})", param.name);
        }
        let ret = self.ty(&function.ret)?;
        let body = self.expr(&body, 1)?;
        Ok(format!("def {name}{binders} : {ret} :=\n  {body}"))
    }

    fn function(&mut self, entry: &Decl, function: &FnDecl) -> Result<String> {
        let text = self.definition(entry, function)?;
        if !function.recursive {
            return Ok(text);
        }
        match function.decreasing {
            None => {
                self.state.encode(
                    "general-recursion",
                    "recursion without a structurally decreasing argument is a Lean partial def: it runs as the source does but its equations are opaque to proofs",
                );
                Ok(format!("partial {text}"))
            }
            // Lean tries structural recursion on each argument before
            // well-founded recursion, so the structurally decreasing argument
            // needs no annotation.
            Some(_) => Ok(text),
        }
    }

    /// Mutually recursive functions are one `mutual` block of partial defs.
    fn mutual(&mut self, group: &[&Decl]) -> Result<String> {
        self.state.encode(
            "mutual-recursion",
            "mutually recursive functions are a Lean mutual block of partial defs: they run as the source does but their equations are opaque to proofs",
        );
        let mut blocks = vec!["mutual".to_owned()];
        for entry in group {
            if let Decl::Fn(function) = entry {
                blocks.push(format!("partial {}", self.definition(entry, function)?));
            }
        }
        blocks.push("end".to_owned());
        Ok(blocks.join("\n\n"))
    }

    fn theorem(&mut self, entry: &Decl, theorem: &TheoremDecl) -> Result<String> {
        let (binders, prop, plan) = rename_theorem(theorem, &ident, &self.state.local_reserved());
        let name = self.state.local_name(&theorem.full_name).to_owned();
        self.state.map(entry, &name);
        let mut functions = prop_functions(&prop, Vec::new());
        for unfold in collect_unfold(&plan) {
            if !functions.contains(unfold) {
                functions.push(unfold.clone());
            }
        }
        let mut statement = format!("theorem {name}");
        for binder in &binders {
            let ty = self.ty(&binder.ty)?;
            let _ = write!(statement, " ({} : {ty})", binder.name);
        }
        let _ = write!(statement, " : {} := by", self.prop(&prop)?);
        self.state
            .theorem(&theorem.full_name, &name, binders.is_empty(), false);
        Ok(format!("{statement}\n{}", self.plan(&plan, &functions, 1)))
    }

    fn plan(&self, plan: &Plan, functions: &[String], depth: usize) -> String {
        let pad = pad(depth);
        let (tactic, split) = match plan {
            Plan::Close { hints } => {
                return format!("{pad}{}", self.closer(hints, functions, depth));
            }
            Plan::Induction(split) => ("induction", split),
            Plan::Cases(split) => ("cases", split),
        };
        // `case tag names =>` names each constructor's fields, then its induction hypotheses.
        let mut lines = vec![format!("{pad}{tactic} {}", split.variable)];
        for kase in &split.cases {
            let ctor = match &split.ty {
                Type::Nat => kase.ctor.as_str(),
                other => self
                    .state
                    .ctor_local(other.data_name().unwrap_or_default(), &kase.ctor),
            };
            let ihs: &[String] = if tactic == "induction" {
                &kase.ihs
            } else {
                &[]
            };
            let names: String =
                kase.fields
                    .iter()
                    .chain(ihs)
                    .fold(String::new(), |mut names, item| {
                        let _ = write!(names, " {item}");
                        names
                    });
            lines.push(format!("{pad}case {ctor}{names} =>"));
            lines.push(self.plan(&kase.plan, functions, depth + 1));
        }
        lines.join("\n")
    }

    fn closer(&self, hints: &Hints, functions: &[String], depth: usize) -> String {
        let mut unfold: Vec<&String> = Vec::new();
        for function in functions.iter().chain(&hints.unfold) {
            if !unfold.contains(&function) {
                unfold.push(function);
            }
        }
        let mut rules: Vec<String> = unfold
            .into_iter()
            .map(|function| self.state.reference(function, "."))
            .collect();
        rules.extend(
            hints
                .lemmas
                .iter()
                .map(|lemma| self.state.reference(lemma, ".")),
        );
        rules.extend(hints.hyps.iter().cloned());
        let ring = ["Nat.mul_add", "Nat.add_mul", "Int.mul_add", "Int.add_mul"];
        let list = rules.join(", ");
        let mut alternatives = vec!["rfl".to_owned(), "decide".to_owned()];
        if rules.is_empty() {
            alternatives.push("omega".to_owned());
            alternatives.push(format!("simp_all [{}] <;> omega", ring.join(", ")));
        } else {
            let with_ring: Vec<&str> = rules.iter().map(String::as_str).chain(ring).collect();
            alternatives.extend([
                format!("simp only [{list}] <;> done"),
                format!("simp_all only [{list}] <;> omega"),
                format!("simp [{list}] <;> done"),
                format!("simp_all [{list}] <;> omega"),
                format!("simp_all [{}] <;> omega", with_ring.join(", ")),
            ]);
        }
        // Each `try` closes the goal or leaves it untouched, and once one
        // closes it the rest find no goal, so the lines act as ordered
        // alternatives.
        let alternatives: Vec<String> = alternatives
            .iter()
            .map(|alternative| format!("try {alternative}"))
            .collect();
        alternatives.join(&format!("\n{}", pad(depth)))
    }

    fn prop(&mut self, prop: &Prop) -> Result<String> {
        Ok(match prop {
            Prop::Forall { binders, body } => {
                let mut parts = Vec::new();
                for binder in binders {
                    parts.push(format!("({} : {})", binder.name, self.ty(&binder.ty)?));
                }
                format!("(∀ {}, {})", parts.join(" "), self.prop(body)?)
            }
            Prop::And { left, right } => {
                let left = self.prop(left)?;
                format!("({left} ∧ {})", self.prop(right)?)
            }
            Prop::Or { left, right } => {
                let left = self.prop(left)?;
                format!("({left} ∨ {})", self.prop(right)?)
            }
            Prop::Implies { left, right } => {
                let left = self.prop(left)?;
                format!("({left} → {})", self.prop(right)?)
            }
            Prop::Not { arg } => format!("(¬ {})", self.prop(arg)?),
            Prop::Bool { expr } => format!("({} = true)", self.expr(expr, 0)?),
            other => {
                let (op, comparison) = other.comparison().expect("comparison");
                let operator = match op {
                    BinaryOp::Eq => "=",
                    BinaryOp::Ne => "≠",
                    BinaryOp::Lt => "<",
                    BinaryOp::Le => "≤",
                    BinaryOp::Gt => ">",
                    _ => "≥",
                };
                let left = self.expr(&comparison.left, 0)?;
                format!("({left} {operator} {})", self.expr(&comparison.right, 0)?)
            }
        })
    }
}

include!("emit_lean_expressions.rs");
