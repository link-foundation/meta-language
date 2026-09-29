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

use super::diagnostics::{type_error, unsupported, Result};
use super::emit_common::{
    mutual_groups, order_declarations, CtorStyle, Dependence, EmitOptions, EmitState, Emitted,
};
use super::ir::{
    rename_function, rename_theorem, ByZero, Case, DataDecl, Decl, Expr, FnDecl, Hints, LitValue,
    Node, Pattern, Plan, Program, Prop, Semantics, TheoremDecl,
};
use super::lean_root_names::LEAN_ROOT_NAMES;
use super::output::thread_output;
use super::proof::prop_functions;
use super::surface::{BinaryOp, Flavor, Rounding, UnaryOp};
use super::types::{fixed_bounds, Type};
use super::Language;

mod helpers;
mod main;
use self::helpers::HELPERS;

const KEYWORDS: &[&str] = &[
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
    let program = &*thread_output(source)?;
    let state = EmitState::new(
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
    LeanEmitter {
        program,
        state,
        helpers: HashSet::new(),
        successors: HashMap::new(),
        uses_number: Dependence::numbers(program),
        uses_partial: Dependence::partial(program),
    }
    .file()
}

struct LeanEmitter<'p> {
    program: &'p Program,
    state: EmitState<'p>,
    helpers: HashSet<&'static str>,
    // Inside `| p + 1 =>` of a match on `x`, `x` is written `p + 1`: Lean's
    // structural recursion sees through the pattern but not the variable.
    successors: HashMap<String, String>,
    uses_number: Dependence,
    uses_partial: Dependence,
}

impl LeanEmitter<'_> {
    fn file(mut self) -> Result<Emitted> {
        let mut blocks = Vec::new();
        let mut open: Vec<String> = Vec::new();
        let groups = mutual_groups(self.program);
        for entry in order_declarations(self.program, false)? {
            self.move_to(&mut open, entry.module_path(), &mut blocks);
            let block = match (entry, groups.get(entry.full_name())) {
                (_, Some(group)) if std::ptr::eq(group[0], entry) => self.mutual(group)?,
                (_, Some(_)) => continue,
                (Decl::Data(data), None) => self.data(entry, data)?,
                (Decl::Fn(function), None) => self.function(entry, function)?,
                (Decl::Theorem(theorem), None) => self.theorem(entry, theorem)?,
            };
            blocks.push(block);
        }
        self.move_to(&mut open, &[], &mut blocks);
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
                lines.push(text.to_owned());
                lines.push(String::new());
            }
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
    fn move_to(&self, open: &mut Vec<String>, path: &[String], blocks: &mut Vec<String>) {
        let common = open
            .iter()
            .zip(path)
            .take_while(|(left, right)| left == right)
            .count();
        while open.len() > common {
            blocks.push(format!("end {}", self.module_name(open)));
            open.pop();
        }
        while open.len() < path.len() {
            open.push(path[open.len()].clone());
            blocks.push(format!("namespace {}", self.module_name(open)));
        }
    }

    fn module_name(&self, path: &[String]) -> String {
        self.state.module_name(path).unwrap_or_default().to_owned()
    }

    fn ty(&mut self, ty: &Type) -> Result<String> {
        Ok(match ty {
            Type::Nat => "Nat".to_owned(),
            Type::Int => "Int".to_owned(),
            Type::Fixed { signed, .. } => {
                let key = ty.key();
                let represented = if *signed { "Int" } else { "Nat" };
                self.state.encode(
                    &format!("machine-integer:{key}"),
                    &format!("{key} values are {represented} values; every operation checks the {key} range and panics outside it, where Rust panics"),
                );
                represented.to_owned()
            }
            Type::Float => {
                self.state.encode(
                    "floats",
                    "a JavaScript Number is a Lean Float, the same IEEE-754 binary64 with the same arithmetic; % is ml_float_rem, the exact truncated remainder, and ml_js_number prints a value as JavaScript does",
                );
                "Float".to_owned()
            }
            Type::Bool => "Bool".to_owned(),
            Type::String => "String".to_owned(),
            Type::Unit => "Unit".to_owned(),
            Type::Output => "List String".to_owned(),
            Type::Data { name } => self.state.reference(name, "."),
            other => {
                return Err(type_error(
                    format!("no Lean type for {}", other.kind()),
                    None,
                ))
            }
        })
    }

    fn data(&mut self, entry: &Decl, data: &DataDecl) -> Result<String> {
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
        let deriving = if data.output {
            "Inhabited"
        } else {
            "Repr, DecidableEq, Inhabited"
        };
        lines.push(format!("  deriving {deriving}"));
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

    fn expr(&mut self, e: &Expr, depth: usize) -> Result<String> {
        Ok(match &e.node {
            Node::Lit { value } => {
                if e.ty.is_float() {
                    self.ty(&e.ty)?;
                }
                literal(&e.ty, value)?
            }
            Node::Unit => "()".to_owned(),
            Node::OutNil => "([] : List String)".to_owned(),
            Node::OutCons { head, tail } => {
                format!(
                    "({} :: {})",
                    self.expr(head, depth)?,
                    self.expr(tail, depth)?
                )
            }
            Node::Var { name } => self.successors.get(name).unwrap_or(name).clone(),
            Node::Call { func, args } => {
                let head = self.state.reference(func, ".");
                self.application(head, args, depth)?
            }
            Node::Ctor { data, ctor, args } => {
                let head = self.state.ctor_ref(data, ctor, ".");
                self.application(head, args, depth)?
            }
            Node::Unary { op, arg, .. } => {
                let arg = self.expr(arg, depth)?;
                if *op == UnaryOp::Not {
                    format!("(!{arg})")
                } else {
                    self.checked(format!("(-{arg})"), &e.ty, "negation")
                }
            }
            Node::Binary { .. } => self.binary(e, depth)?,
            Node::If {
                cond,
                then,
                otherwise,
            } => {
                let cond = self.expr(cond, depth)?;
                let then = self.expr(then, depth)?;
                format!(
                    "(if {cond} then {then} else {})",
                    self.expr(otherwise, depth)?
                )
            }
            Node::Let { name, value, body } => {
                let value = self.expr(value, depth)?;
                format!(
                    "(let {name} := {value};\n{}{})",
                    pad(depth + 1),
                    self.expr(body, depth + 1)?
                )
            }
            Node::Match { scrutinee, cases } => self.match_expr(scrutinee, cases, depth)?,
            Node::ToString { arg, console } => {
                if arg.ty == Type::String {
                    self.expr(arg, depth)?
                } else {
                    self.text_of(arg, depth, *console)?
                }
            }
            Node::Cast {
                arg,
                from,
                to,
                flavor,
            } => self.cast(arg, from, to, *flavor, depth)?,
            Node::Abort { message } => {
                self.state.abort_to_total(message);
                format!("(panic! {} : {})", json_string(message), self.ty(&e.ty)?)
            }
            // Output threading leaves no `print` node.
            Node::Print { .. } => {
                return Err(type_error(
                    "print with its output not threaded".to_owned(),
                    e.span,
                ))
            }
        })
    }

    /// A call or constructor application: the bare head without arguments.
    fn application(&mut self, head: String, args: &[Expr], depth: usize) -> Result<String> {
        if args.is_empty() {
            return Ok(head);
        }
        let mut parts = vec![head];
        for arg in args {
            parts.push(self.expr(arg, depth)?);
        }
        Ok(format!("({})", parts.join(" ")))
    }

    fn text_of(&mut self, arg: &Expr, depth: usize, console: bool) -> Result<String> {
        if matches!(arg.ty, Type::Data { .. } | Type::Unit) {
            return Err(unsupported(
                "output of structured values",
                &format!("a {} value has no portable textual form", arg.ty.kind()),
                arg.span,
            ));
        }
        if arg.ty.is_float() {
            self.helpers.insert("jsNumber");
            if console {
                self.helpers.insert("jsConsole");
            }
            let function = if console {
                "ml_js_console"
            } else {
                "ml_js_number"
            };
            return Ok(format!("({function} {})", self.expr(arg, depth)?));
        }
        Ok(format!("(toString {})", self.expr(arg, depth)?))
    }

    /// Range-checks a machine-integer result computed in Int.
    fn checked(&mut self, text: String, ty: &Type, what: &str) -> String {
        let Type::Fixed { bits, signed } = *ty else {
            return text;
        };
        let key = ty.key();
        self.state.abort_to_total(&format!("{key} {what} overflow"));
        let (min, max) = fixed_bounds(bits, signed);
        if signed {
            self.helpers.insert("fixed");
            return format!("(ml_fixed {text} ({min}) {max} \"{key} {what}\")");
        }
        self.helpers.insert("fixedNat");
        format!("(ml_fixed_nat {text} {max} \"{key} {what}\")")
    }

    fn binary(&mut self, e: &Expr, depth: usize) -> Result<String> {
        let Node::Binary {
            op, left, right, ..
        } = &e.node
        else {
            unreachable!("binary expressions only");
        };
        let left_text = self.expr(left, depth)?;
        let right_text = self.expr(right, depth)?;
        let comparison = match op {
            BinaryOp::And => return Ok(format!("({left_text} && {right_text})")),
            BinaryOp::Or => return Ok(format!("({left_text} || {right_text})")),
            BinaryOp::Concat => return Ok(format!("({left_text} ++ {right_text})")),
            BinaryOp::Eq => return Ok(format!("({left_text} == {right_text})")),
            BinaryOp::Ne => return Ok(format!("({left_text} != {right_text})")),
            BinaryOp::Lt => "<",
            BinaryOp::Le => "≤",
            BinaryOp::Gt => ">",
            BinaryOp::Ge => "≥",
            _ => return self.arithmetic(e, left_text, right_text),
        };
        Ok(format!("(decide ({left_text} {comparison} {right_text}))"))
    }

    fn arithmetic(&mut self, e: &Expr, left_text: String, right_text: String) -> Result<String> {
        let Node::Binary {
            op,
            left,
            right,
            domain,
            rounding,
            by_zero,
            semantics,
        } = &e.node
        else {
            unreachable!("binary expressions only");
        };
        if *semantics == Some(Semantics::Ieee) {
            let operator = match op {
                BinaryOp::Add => "+",
                BinaryOp::Sub => "-",
                BinaryOp::Mul => "*",
                BinaryOp::Div => "/",
                _ => {
                    self.helpers.insert("floatRem");
                    return Ok(format!("(ml_float_rem {left_text} {right_text})"));
                }
            };
            return Ok(format!("({left_text} {operator} {right_text})"));
        }
        let fixed = matches!(e.ty, Type::Fixed { .. });
        // Machine-integer operations run in Int and are range-checked afterwards.
        let wide = |text: String, operand: &Expr| {
            if matches!(operand.ty, Type::Fixed { signed: false, .. }) {
                format!("(Int.ofNat {text})")
            } else {
                text
            }
        };
        let (a, b) = if fixed {
            (wide(left_text, left), wide(right_text, right))
        } else {
            (left_text, right_text)
        };
        match op {
            BinaryOp::Add => Ok(self.checked(format!("({a} + {b})"), &e.ty, "addition")),
            BinaryOp::Mul => Ok(self.checked(format!("({a} * {b})"), &e.ty, "multiplication")),
            BinaryOp::Sub => Ok(self.checked(format!("({a} - {b})"), &e.ty, "subtraction")),
            BinaryOp::Div | BinaryOp::Rem => {
                let division = *op == BinaryOp::Div;
                let natural = !fixed && matches!(domain, Some(Type::Nat));
                let mut divisor = b;
                if *by_zero == Some(ByZero::Abort) {
                    self.state.abort_to_total("division by zero");
                    self.helpers
                        .insert(if natural { "divideNat" } else { "divide" });
                    let guard = if natural {
                        "ml_nonzero_nat"
                    } else {
                        "ml_nonzero"
                    };
                    divisor = format!("({guard} {divisor})");
                }
                let text = if natural {
                    format!("({a} {} {divisor})", if division { "/" } else { "%" })
                } else if *rounding == Some(Rounding::Trunc) {
                    format!(
                        "(Int.{} {a} {divisor})",
                        if division { "tdiv" } else { "tmod" }
                    )
                } else if *rounding == Some(Rounding::Floor) {
                    format!(
                        "(Int.{} {a} {divisor})",
                        if division { "fdiv" } else { "fmod" }
                    )
                } else {
                    format!("({a} {} {divisor})", if division { "/" } else { "%" })
                };
                let what = if division { "division" } else { "remainder" };
                Ok(self.checked(text, &e.ty, what))
            }
            other => Err(type_error(
                format!("no Lean operator {}", operator_name(*other)),
                None,
            )),
        }
    }

    fn match_expr(&mut self, scrutinee: &Expr, cases: &[Case], depth: usize) -> Result<String> {
        let subject = self.expr(scrutinee, depth)?;
        let mut arms = Vec::new();
        for kase in cases {
            let text = match &kase.pattern {
                Pattern::NatZero => "0".to_owned(),
                Pattern::NatSucc { name } => format!("{name} + 1"),
                Pattern::Wild => "_".to_owned(),
                Pattern::Bind { name } => name.clone(),
                Pattern::Ctor { data, ctor, binds } => {
                    let mut parts = vec![self.state.ctor_ref(data, ctor, ".")];
                    parts.extend(
                        binds
                            .iter()
                            .map(|bind| bind.clone().unwrap_or_else(|| "_".to_owned())),
                    );
                    parts.join(" ")
                }
            };
            let saved = self.successors.clone();
            if let (Pattern::NatSucc { name }, Some(variable)) =
                (&kase.pattern, scrutinee.var_name())
            {
                self.successors
                    .insert(variable.to_owned(), format!("({name} + 1)"));
            }
            let body = self.expr(&kase.body, depth + 2);
            self.successors = saved;
            arms.push(format!("{}| {text} => {}", pad(depth + 1), body?));
        }
        Ok(format!("(match {subject} with\n{})", arms.join("\n")))
    }

    fn cast(
        &mut self,
        arg: &Expr,
        from: &Type,
        to: &Type,
        flavor: Flavor,
        depth: usize,
    ) -> Result<String> {
        let arg = self.expr(arg, depth)?;
        let natural_from = from.is_natural();
        let natural_to = to.is_natural();
        if natural_from == natural_to {
            return Ok(arg);
        }
        if natural_from {
            return Ok(format!("(Int.ofNat {arg})"));
        }
        if flavor == Flavor::Clamp {
            return Ok(format!("(Int.toNat {arg})"));
        }
        self.state.checked_to_total("conversion to a natural");
        self.helpers.insert("toNatChecked");
        Ok(format!("(ml_to_nat_checked {arg})"))
    }
}

fn literal(ty: &Type, value: &LitValue) -> Result<String> {
    Ok(match ty {
        Type::Float => lean_float(&value.text()),
        Type::Nat => format!("({} : Nat)", value.text()),
        Type::Int => format!("({} : Int)", value.text()),
        Type::Fixed { signed, .. } => {
            format!(
                "({} : {})",
                value.text(),
                if *signed { "Int" } else { "Nat" }
            )
        }
        Type::Bool => value.text(),
        Type::String => json_string(&value.text()),
        other => {
            return Err(type_error(
                format!("no Lean literal for {}", other.kind()),
                None,
            ))
        }
    })
}

/// A Number's canonical JavaScript text as a Float term; Lean reads decimal
/// literals correctly rounded.
fn lean_float(value: &str) -> String {
    let text = match value {
        "NaN" => "0.0 / 0.0".to_owned(),
        "Infinity" => "1.0 / 0.0".to_owned(),
        "-Infinity" => "-1.0 / 0.0".to_owned(),
        _ => value.replacen("e+", "e", 1),
    };
    format!("({text} : Float)")
}

/// A string literal as JavaScript's `JSON.stringify` writes it.
fn json_string(text: &str) -> String {
    serde_json::to_string(text).unwrap_or_default()
}

/// The operator's name in the checked program's JSON form.
fn operator_name(op: BinaryOp) -> String {
    serde_json::to_value(op)
        .ok()
        .and_then(|value| value.as_str().map(str::to_owned))
        .unwrap_or_default()
}

fn collect_unfold(plan: &Plan) -> Vec<&String> {
    match plan {
        Plan::Close { hints } => hints.unfold.iter().collect(),
        Plan::Induction(split) | Plan::Cases(split) => split
            .cases
            .iter()
            .flat_map(|kase| collect_unfold(&kase.plan))
            .collect(),
    }
}

fn pad(depth: usize) -> String {
    "  ".repeat(depth)
}
