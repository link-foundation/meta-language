//! The Rocq emitter: writes a checked program as Rocq with its translation contract.
//!
//! Naturals become binary `N` and integers `Z`, so programs run at their
//! source sizes (unary `nat` cannot hold 20!). Structural recursion over data
//! is a `Fixpoint`; recursion that decreases a natural is a `Function` with
//! the measure `N.to_nat`, whose obligations `lia` discharges and whose
//! equation lemma the reconstructed proofs rewrite with.
//!
//! Mirrors `js/src/translation/emit-rocq.js`.

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;

use super::diagnostics::{unsupported, ErrorKind, Result, TranslationError};
use super::emit_common::{order_declarations, CtorStyle, EmitOptions, EmitState, Emitted};
use super::ir::{
    rename_function, rename_main, rename_theorem, ByZero, Case, DataDecl, Decl, Effect, Expr,
    FnDecl, Hints, LitValue, Main, Node, Pattern, Plan, Program, Prop, Semantics, TheoremDecl,
};
use super::proof::prop_functions;
use super::surface::{BinaryOp, Flavor, Rounding, UnaryOp};
use super::types::Type;
use super::Language;

const KEYWORDS: &[&str] = &[
    "as",
    "at",
    "cofix",
    "else",
    "end",
    "exists",
    "exists2",
    "fix",
    "for",
    "forall",
    "fun",
    "if",
    "IF",
    "in",
    "let",
    "match",
    "mod",
    "Prop",
    "return",
    "Set",
    "SProp",
    "then",
    "Type",
    "using",
    "where",
    "with",
    "struct",
    "measure",
    "wf",
    "Definition",
    "Fixpoint",
    "Function",
    "Theorem",
    "Lemma",
    "Proof",
    "Qed",
    "Defined",
    "Module",
    "End",
    "Inductive",
    "Import",
    "Require",
    "From",
    "Open",
    "Scope",
    "Eval",
    "Compute",
    "Check",
    "Print",
    // Standard-library names the emitted code uses unqualified.
    "N",
    "Z",
    "String",
    "EmptyString",
    "negb",
    "andb",
    "orb",
    "true",
    "false",
    "bool",
    "string",
    "list",
    "nil",
    "cons",
    "unit",
    "tt",
    "nat",
    "O",
    "S",
    "Bool",
    "Ascii",
    "main",
    "lia",
    "nia",
    // Imported constructors: a pattern variable with one of these names would match the constructor instead.
    "left",
    "right",
    "inleft",
    "inright",
    "Some",
    "None",
    "pair",
    "inl",
    "inr",
    "exist",
    "existT",
    "I",
    "conj",
    "or_introl",
    "or_intror",
    "ex_intro",
    "eq_refl",
    "Eq",
    "Lt",
    "Gt",
    "CompEq",
    "CompLt",
    "CompGt",
    "xI",
    "xO",
    "xH",
    "N0",
    "Npos",
    "Z0",
    "Zpos",
    "Zneg",
    "ReflectT",
    "ReflectF",
    "identity_refl",
];

/// The `Require` lines every emitted file starts with.
pub const ROCQ_PRELUDE: &[&str] =
    &["From Stdlib Require Import NArith ZArith Lia Recdef String List Ascii."];

const DIGITS: &str = r"Fixpoint ml_digits (fuel : nat) (n : N) (acc : string) : string :=
  match fuel with
  | O => acc
  | S rest =>
      let acc' := String (ascii_of_N (48 + N.modulo n 10)) acc in
      if N.ltb n 10 then acc' else ml_digits rest (N.div n 10) acc'
  end.
Definition ml_N_to_string (n : N) : string := ml_digits (S (N.size_nat n)) n EmptyString.";

const Z_TO_STRING: &str = r#"Definition ml_Z_to_string (z : Z) : string :=
  if Z.ltb z 0 then String.append "-" (ml_N_to_string (Z.to_N (Z.opp z))) else ml_N_to_string (Z.to_N z)."#;

const BOOL_TO_STRING: &str =
    r#"Definition ml_bool_to_string (b : bool) : string := if b then "true" else "false"."#;

const EUCLID: &str = r"(* Euclidean division, the rounding of Lean's Int / and %: the remainder is never negative. *)
Definition ml_Z_ediv (a b : Z) : Z := Z.mul (Z.sgn b) (Z.div a (Z.abs b)).
Definition ml_Z_emod (a b : Z) : Z := Z.modulo a (Z.abs b).";

const TACTICS: &str = r"Ltac ml_N_norm := repeat first
  [ rewrite N.pred_succ
  | rewrite (proj2 (N.eqb_neq (N.succ _) 0) (N.neq_succ_0 _))
  | rewrite N.eqb_refl ].
Ltac ml_obligation := intros; repeat match goal with H : N.eqb _ _ = false |- _ => apply N.eqb_neq in H end; lia.
Ltac ml_close := first [reflexivity | lia | nia | congruence].";

// A closed assertion computes to connectives over literal (in)equalities:
// prove the goal, or refute a hypothesis, one connective at a time.
const DECIDE: &str = r#"Ltac ml_prove := first
  [ reflexivity | discriminate | exact I | lia
  | split; ml_prove
  | left; ml_prove
  | right; ml_prove
  | let H := fresh "H" in intro H; first [ml_prove | ml_refute H] ]
with ml_refute H := first
  [ discriminate H | exact H | lia
  | let A := fresh "H" in let B := fresh "H" in destruct H as [A B]; first [ml_refute A | ml_refute B]
  | let A := fresh "H" in let B := fresh "H" in destruct H as [A | B]; [ml_refute A | ml_refute B]
  | apply H; ml_prove ].
Ltac ml_decide := vm_compute; ml_prove."#;

/// A helper definition the emitted program may need, in the order they are written out.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Helper {
    Digits,
    ZToString,
    BoolToString,
    Euclid,
    Tactics,
    Decide,
}

impl Helper {
    const ALL: [Self; 6] = [
        Self::Digits,
        Self::ZToString,
        Self::BoolToString,
        Self::Euclid,
        Self::Tactics,
        Self::Decide,
    ];

    const fn text(self) -> &'static str {
        match self {
            Self::Digits => DIGITS,
            Self::ZToString => Z_TO_STRING,
            Self::BoolToString => BOOL_TO_STRING,
            Self::Euclid => EUCLID,
            Self::Tactics => TACTICS,
            Self::Decide => DECIDE,
        }
    }
}

fn ident(name: &str) -> String {
    let mut result: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '\'' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if result.starts_with(|c: char| c.is_ascii_digit() || c == '\'') {
        result = format!("x{result}");
    }
    if KEYWORDS.contains(&result.as_str()) {
        result.push('_');
    }
    result
}

/// Rocq derives these from inductives and Functions in the same namespace.
fn generated(name: &str) -> Vec<String> {
    [
        "_ind",
        "_rect",
        "_rec",
        "_sind",
        "_equation",
        "_tcc",
        "_terminate",
        "_F",
        "_graph",
        "_complete",
        "_correct",
        "_ind_",
        "_rect_",
    ]
    .iter()
    .map(|suffix| format!("{name}{suffix}"))
    .chain(std::iter::once(format!("R_{name}")))
    .collect()
}

/// An error the JavaScript emitter raises with a plain `Error`: the checker never produces its input.
fn internal(message: String) -> TranslationError {
    TranslationError::new(ErrorKind::Type, message, None)
}

/// Emits a checked program as Rocq.
///
/// # Errors
/// On constructs the target cannot express faithfully.
pub fn emit_rocq(program: &Program) -> Result<Emitted> {
    let state = EmitState::new(
        program,
        Language::Rocq,
        ident,
        KEYWORDS,
        EmitOptions {
            ctor_style: CtorStyle::Module,
            generated: Some(generated),
            ..EmitOptions::default()
        },
    );
    RocqEmitter {
        program,
        state,
        helpers: HashSet::from([Helper::Tactics]),
        nat_functions: HashMap::new(),
        current: None,
    }
    .file()
}

/// The function being emitted, so its recursive calls use its local name.
struct Current {
    full_name: String,
    name: String,
    recursive: bool,
}

struct RocqEmitter<'p> {
    program: &'p Program,
    state: EmitState<'p>,
    helpers: HashSet<Helper>,
    /// Functions recursive on a natural: the decreasing parameter and the arity.
    nat_functions: HashMap<String, (usize, usize)>,
    current: Option<Current>,
}

impl RocqEmitter<'_> {
    fn file(mut self) -> Result<Emitted> {
        let program = self.program;
        let mut blocks: Vec<String> = Vec::new();
        let order = order_declarations(program, true)?;
        let mut open_modules: Vec<String> = Vec::new();
        for entry in order {
            self.move_to(&mut open_modules, entry.module_path(), &mut blocks);
            blocks.push(match entry {
                Decl::Data(data) => self.data(entry, data)?,
                Decl::Fn(function) => self.function(entry, function)?,
                Decl::Theorem(theorem) => self.theorem(entry, theorem)?,
            });
        }
        self.move_to(&mut open_modules, &[], &mut blocks);
        let main_text = match &program.main {
            Some(main) => Some(self.main(main)?),
            None => None,
        };
        let helper_text = Helper::ALL.into_iter().filter(|helper| {
            self.helpers.contains(helper)
                || (*helper == Helper::Digits && self.helpers.contains(&Helper::ZToString))
        });
        let mut lines: Vec<String> = vec![format!(
            "(* Translated from {} by meta-language: portable core, Rocq target. *)",
            program.source_language.as_str()
        )];
        lines.extend(ROCQ_PRELUDE.iter().map(|line| (*line).to_owned()));
        lines.push(String::new());
        for helper in helper_text {
            lines.push(helper.text().to_owned());
            lines.push(String::new());
        }
        for block in blocks {
            lines.push(block);
            lines.push(String::new());
        }
        let entry = main_text.as_ref().map(|_| "main".to_owned());
        if let Some(main_text) = main_text {
            lines.push(main_text);
            lines.push(String::new());
            lines.push("Eval vm_compute in main.".to_owned());
            lines.push(String::new());
        }
        Ok(self.state.finish(lines.join("\n"), entry))
    }

    /// Closes and opens `Module` blocks until `path` is the open module.
    fn move_to(&self, open_modules: &mut Vec<String>, path: &[String], blocks: &mut Vec<String>) {
        let common = open_modules
            .iter()
            .zip(path)
            .take_while(|(open, wanted)| open == wanted)
            .count();
        while open_modules.len() > common {
            blocks.push(format!("End {}.", self.module_name(open_modules)));
            open_modules.pop();
        }
        while open_modules.len() < path.len() {
            open_modules.push(path[open_modules.len()].clone());
            blocks.push(format!("Module {}.", self.module_name(open_modules)));
        }
    }

    fn module_name(&self, path: &[String]) -> &str {
        self.state.module_name(path).unwrap_or("undefined")
    }

    fn ty(&mut self, ty: &Type) -> Result<String> {
        Ok(match ty {
            Type::Nat => "N".to_owned(),
            Type::Int => "Z".to_owned(),
            Type::Fixed { signed, .. } => {
                self.state.fixed_to_unbounded(ty);
                if *signed { "Z" } else { "N" }.to_owned()
            }
            Type::Bool => "bool".to_owned(),
            Type::String => "string".to_owned(),
            Type::Unit => "unit".to_owned(),
            Type::Data { name } => self.state.reference(name, "."),
            other => return Err(internal(format!("no Rocq type for {}", other.kind()))),
        })
    }

    fn data(&mut self, entry: &Decl, data: &DataDecl) -> Result<String> {
        let name = self.state.local_name(&data.full_name).to_owned();
        let mut lines = vec![format!("Inductive {name} : Type :=")];
        for ctor in &data.ctors {
            let mut fields = String::new();
            for field in &ctor.fields {
                fields.push_str(&self.ty(&field.ty)?);
                fields.push_str(" -> ");
            }
            let local = self.state.ctor_local(&data.full_name, &ctor.name);
            lines.push(format!("| {local} : {fields}{name}"));
        }
        self.state.map(entry, &name);
        Ok(lines.join("\n") + ".")
    }

    fn function(&mut self, entry: &Decl, function: &FnDecl) -> Result<String> {
        let (params, body) = rename_function(function, &ident, &self.state.local_reserved());
        let name = self.state.local_name(&function.full_name).to_owned();
        self.state.map(entry, &name);
        self.current = Some(Current {
            full_name: function.full_name.clone(),
            name: name.clone(),
            recursive: function.recursive,
        });
        let mut binders = String::new();
        for param in &params {
            let ty = self.ty(&param.ty)?;
            let _ = write!(binders, " ({} : {ty})", param.name);
        }
        let result = self.ty(&function.ret)?;
        let text = self.expr(&body)?;
        self.current = None;
        if function.mutual {
            return Err(unsupported(
                "mutual recursion",
                &format!(
                    "{} is mutually recursive; the Rocq target emits only single recursive definitions",
                    function.full_name
                ),
                function.span,
            ));
        }
        if !function.recursive {
            return Ok(format!(
                "Definition {name}{binders} : {result} :=\n  {text}."
            ));
        }
        let Some(index) = function.decreasing else {
            return Err(unsupported(
                "general recursion",
                &format!(
                    "{} is not structurally recursive and Rocq requires a termination argument",
                    function.full_name
                ),
                function.span,
            ));
        };
        let decreasing = &params[index];
        if matches!(decreasing.ty, Type::Data { .. }) {
            return Ok(format!(
                "Fixpoint {name}{binders} {{struct {}}} : {result} :=\n  {text}.",
                decreasing.name
            ));
        }
        self.nat_functions
            .insert(function.full_name.clone(), (index, params.len()));
        self.state.encode(
            "nat-recursion",
            "recursion that decreases a natural is a Rocq Function with measure N.to_nat; lia discharges the decrease obligations",
        );
        Ok(format!(
            "Function {name}{binders} {{measure N.to_nat {}}} : {result} :=\n  {text}.\nProof. all: ml_obligation. Defined.",
            decreasing.name
        ))
    }

    fn theorem(&mut self, entry: &Decl, theorem: &TheoremDecl) -> Result<String> {
        let (binders, prop, plan) = rename_theorem(theorem, &ident, &self.state.local_reserved());
        let name = self.state.local_name(&theorem.full_name).to_owned();
        self.state.map(entry, &name);
        let mut statement = format!("Theorem {name}");
        for binder in &binders {
            let ty = self.ty(&binder.ty)?;
            let _ = write!(statement, " ({} : {ty})", binder.name);
        }
        let prop_text = self.prop(&prop)?;
        let _ = write!(statement, " : {prop_text}.");
        let mut functions = prop_functions(&prop, Vec::new());
        for name in collect_hint_functions(&plan) {
            if !functions.contains(name) {
                functions.push(name.clone());
            }
        }
        let script = self.plan(&plan, &functions, binders.is_empty(), 1);
        self.state
            .theorem(&theorem.full_name, &name, binders.is_empty(), false);
        Ok(format!("{statement}\nProof.\n{script}\nQed."))
    }

    fn plan(&self, plan: &Plan, functions: &[String], closed: bool, depth: usize) -> String {
        let indent = "  ".repeat(depth);
        let (split, induction) = match plan {
            Plan::Close { hints } => {
                return format!("{indent}{}.", self.closer(hints, functions, closed));
            }
            Plan::Induction(split) => (split, true),
            Plan::Cases(split) => (split, false),
        };
        let pattern = if split.ty == Type::Nat {
            let succ = split
                .cases
                .iter()
                .find(|item| item.ctor == "succ")
                .expect("a natural split has a successor case");
            let ih = if induction {
                succ.ihs.first().map_or("undefined", String::as_str)
            } else {
                "_"
            };
            let field = succ.fields.first().map_or("undefined", String::as_str);
            format!(
                "induction {} as [|{field} {ih}] using N.peano_ind.",
                split.variable
            )
        } else {
            let groups: Vec<String> = split
                .cases
                .iter()
                .map(|item| {
                    let mut names: Vec<&str> = Vec::new();
                    for (index, field) in item.fields.iter().enumerate() {
                        names.push(field);
                        if induction && item.recursive.get(index).copied().unwrap_or(false) {
                            let earlier = item.recursive[..index]
                                .iter()
                                .filter(|recursive| **recursive)
                                .count();
                            names.push(item.ihs.get(earlier).map_or("undefined", String::as_str));
                        }
                    }
                    names.join(" ")
                })
                .collect();
            format!(
                "{} {} as [{}].",
                if induction { "induction" } else { "destruct" },
                split.variable,
                groups.join(" | ")
            )
        };
        let mut lines = vec![format!("{indent}{pattern}")];
        for item in &split.cases {
            lines.push(format!("{indent}{{"));
            lines.push(self.plan(&item.plan, functions, false, depth + 1));
            lines.push(format!("{indent}}}"));
        }
        lines.join("\n")
    }

    fn closer(&self, hints: &Hints, functions: &[String], closed: bool) -> String {
        let mut steps: Vec<String> = Vec::new();
        let mut unfold_all: Vec<&String> = Vec::new();
        for name in functions.iter().chain(&hints.unfold) {
            if !unfold_all.contains(&name) {
                unfold_all.push(name);
            }
        }
        let plain: Vec<String> = unfold_all
            .iter()
            .filter(|name| !self.nat_functions.contains_key(name.as_str()))
            .map(|name| self.state.reference(name, "."))
            .collect();
        if !plain.is_empty() {
            steps.push(format!("cbn [{}]", plain.join(" ")));
        }
        for name in &unfold_all {
            let Some(&(index, arity)) = self.nat_functions.get(name.as_str()) else {
                continue;
            };
            let reference = self.state.reference(name, ".");
            let args = |value: &str| {
                (0..arity)
                    .map(|position| {
                        if position == index {
                            value.to_owned()
                        } else {
                            format!("?a{position}")
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            };
            let uses = |value: &str| {
                (0..arity)
                    .map(|position| {
                        if position == index {
                            value.to_owned()
                        } else {
                            format!("a{position}")
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            };
            steps.push(format!(
                "repeat match goal with |- context [{reference} {}] => rewrite ({reference}_equation {}) end",
                args("(N.succ ?k)"),
                uses("(N.succ k)")
            ));
            steps.push(format!(
                "repeat match goal with |- context [{reference} {}] => rewrite ({reference}_equation {}) end",
                args("0%N"),
                uses("0%N")
            ));
        }
        steps.push("ml_N_norm".to_owned());
        steps.push("cbv beta iota zeta".to_owned());
        let mut alternatives: Vec<String> = Vec::new();
        if closed {
            alternatives.push("(vm_compute; reflexivity)".to_owned());
        }
        alternatives.push("ml_close".to_owned());
        let rewrites: Vec<String> = hints
            .hyps
            .iter()
            .cloned()
            .chain(
                hints
                    .lemmas
                    .iter()
                    .map(|lemma| self.state.reference(lemma, ".")),
            )
            .collect();
        if !rewrites.is_empty() {
            let rules = rewrites
                .iter()
                .map(|rule| format!("?{rule}"))
                .collect::<Vec<_>>()
                .join(", ");
            alternatives.push(format!("(rewrite {rules}; ml_close)"));
            alternatives.push(format!("(rewrite <- {rules}; ml_close)"));
        }
        format!("{}; first [{}]", steps.join("; "), alternatives.join(" | "))
    }

    fn prop(&mut self, prop: &Prop) -> Result<String> {
        Ok(match prop {
            Prop::Forall { binders, body } => {
                let mut parts = Vec::new();
                for binder in binders {
                    let ty = self.ty(&binder.ty)?;
                    parts.push(format!("({} : {ty})", binder.name));
                }
                format!("(forall {}, {})", parts.join(" "), self.prop(body)?)
            }
            Prop::And { left, right } => {
                let left = self.prop(left)?;
                format!("({left} /\\ {})", self.prop(right)?)
            }
            Prop::Or { left, right } => {
                let left = self.prop(left)?;
                format!("({left} \\/ {})", self.prop(right)?)
            }
            Prop::Implies { left, right } => {
                let left = self.prop(left)?;
                format!("({left} -> {})", self.prop(right)?)
            }
            Prop::Not { arg } => format!("(~ {})", self.prop(arg)?),
            Prop::Bool { expr } => format!("({} = true)", self.expr(expr)?),
            Prop::Eq(comparison) => {
                let left = self.expr(&comparison.left)?;
                format!("({left} = {})", self.expr(&comparison.right)?)
            }
            Prop::Ne(comparison) => {
                let left = self.expr(&comparison.left)?;
                format!("({left} <> {})", self.expr(&comparison.right)?)
            }
            Prop::Lt(comparison)
            | Prop::Le(comparison)
            | Prop::Gt(comparison)
            | Prop::Ge(comparison) => {
                let module = self.numeric_module(&comparison.domain)?;
                let left = self.expr(&comparison.left)?;
                let right = self.expr(&comparison.right)?;
                match prop {
                    Prop::Lt(_) => format!("({module}.lt {left} {right})"),
                    Prop::Le(_) => format!("({module}.le {left} {right})"),
                    Prop::Gt(_) => format!("({module}.lt {right} {left})"),
                    _ => format!("({module}.le {right} {left})"),
                }
            }
        })
    }

    fn numeric_module(&mut self, ty: &Type) -> Result<&'static str> {
        match ty {
            Type::Nat => Ok("N"),
            Type::Int => Ok("Z"),
            Type::Fixed { signed, .. } => {
                self.state.fixed_to_unbounded(ty);
                Ok(if *signed { "Z" } else { "N" })
            }
            other => Err(internal(format!("not numeric: {}", other.kind()))),
        }
    }

    fn expr(&mut self, e: &Expr) -> Result<String> {
        match &e.node {
            Node::Lit { value } => self.literal(&e.ty, value),
            Node::Unit => Ok("tt".to_owned()),
            Node::Var { name } => Ok(name.clone()),
            Node::Call { func, args } => {
                let head = match &self.current {
                    Some(current) if current.full_name == *func && current.recursive => {
                        current.name.clone()
                    }
                    _ => self.state.reference(func, "."),
                };
                self.application(head, args)
            }
            Node::Ctor { data, ctor, args } => {
                let head = self.state.ctor_ref(data, ctor, ".");
                self.application(head, args)
            }
            Node::Unary { op, arg, semantics } => {
                if *op == UnaryOp::Not {
                    return Ok(format!("(negb {})", self.expr(arg)?));
                }
                if *semantics == Some(Semantics::Checked) {
                    self.state.checked_to_total("negation");
                }
                Ok(format!("(Z.opp {})", self.expr(arg)?))
            }
            Node::Binary { .. } => self.binary(e),
            Node::If {
                cond,
                then,
                otherwise,
            } => {
                let cond = self.expr(cond)?;
                let then = self.expr(then)?;
                Ok(format!(
                    "(if {cond} then {then} else {})",
                    self.expr(otherwise)?
                ))
            }
            Node::Let { name, value, body } => {
                let value = self.expr(value)?;
                Ok(format!("(let {name} := {value} in {})", self.expr(body)?))
            }
            Node::Match { scrutinee, cases } => self.match_expr(scrutinee, cases),
            Node::ToString { arg } => self.text_of(arg),
            Node::Cast {
                arg,
                from,
                to,
                flavor,
            } => self.cast(arg, from, to, *flavor),
            Node::Abort { message } => {
                self.state.abort_to_total(message);
                Ok(format!(
                    "(* unreachable under the non-aborting assumption: {} *) {}",
                    message.replace("*)", "* )"),
                    self.inhabitant(&e.ty)?
                ))
            }
        }
    }

    fn application(&mut self, head: String, args: &[Expr]) -> Result<String> {
        if args.is_empty() {
            return Ok(head);
        }
        let mut parts = vec![head];
        for arg in args {
            parts.push(self.expr(arg)?);
        }
        Ok(format!("({})", parts.join(" ")))
    }

    fn literal(&mut self, ty: &Type, value: &LitValue) -> Result<String> {
        let text = value.text();
        let integer = |text: String| {
            if text.starts_with('-') {
                format!("({text})%Z")
            } else {
                format!("{text}%Z")
            }
        };
        Ok(match ty {
            Type::Nat => format!("{text}%N"),
            Type::Int => integer(text),
            Type::Fixed { signed, .. } => {
                self.state.fixed_to_unbounded(ty);
                if *signed {
                    integer(text)
                } else {
                    format!("{text}%N")
                }
            }
            Type::Bool => text,
            Type::String => format!("\"{}\"%string", text.replace('"', "\"\"")),
            other => return Err(internal(format!("no Rocq literal for {}", other.kind()))),
        })
    }

    fn inhabitant(&self, ty: &Type) -> Result<String> {
        Ok(match ty {
            Type::Nat | Type::Fixed { signed: false, .. } => "0%N".to_owned(),
            Type::Int | Type::Fixed { signed: true, .. } => "0%Z".to_owned(),
            Type::Bool => "false".to_owned(),
            Type::String => "EmptyString".to_owned(),
            Type::Unit => "tt".to_owned(),
            Type::Data { name } => {
                let entry = self.program.data(name);
                let Some(ctor) = entry
                    .ctors
                    .iter()
                    .find(|candidate| {
                        candidate
                            .fields
                            .iter()
                            .all(|field| !matches!(field.ty, Type::Data { .. }))
                    })
                    .or_else(|| entry.ctors.first())
                else {
                    // JavaScript reads the fields of the missing constructor.
                    return Err(internal(
                        "Cannot read properties of undefined (reading 'fields')".to_owned(),
                    ));
                };
                let mut parts = vec![self.state.ctor_ref(name, &ctor.name, ".")];
                for field in &ctor.fields {
                    parts.push(self.inhabitant(&field.ty)?);
                }
                if parts.len() == 1 {
                    parts.swap_remove(0)
                } else {
                    format!("({})", parts.join(" "))
                }
            }
            other => return Err(internal(format!("no Rocq inhabitant for {}", other.kind()))),
        })
    }

    fn binary(&mut self, e: &Expr) -> Result<String> {
        let Node::Binary {
            op,
            left,
            right,
            domain,
            semantics,
            rounding,
            by_zero,
        } = &e.node
        else {
            unreachable!("binary is only called on binary expressions");
        };
        let left = self.expr(left)?;
        let right = self.expr(right)?;
        let domain = domain.as_ref();
        match op {
            BinaryOp::And => Ok(format!("(andb {left} {right})")),
            BinaryOp::Or => Ok(format!("(orb {left} {right})")),
            BinaryOp::Concat => Ok(format!("(String.append {left} {right})")),
            BinaryOp::Eq
            | BinaryOp::Ne
            | BinaryOp::Lt
            | BinaryOp::Le
            | BinaryOp::Gt
            | BinaryOp::Ge => {
                let domain =
                    domain.ok_or_else(|| internal("comparison without a domain".to_owned()))?;
                let module = match domain {
                    Type::Bool => "Bool",
                    Type::String => "String",
                    other => self.numeric_module(other)?,
                };
                Ok(match op {
                    BinaryOp::Eq => format!("({module}.eqb {left} {right})"),
                    BinaryOp::Ne => format!("(negb ({module}.eqb {left} {right}))"),
                    BinaryOp::Lt => format!("({module}.ltb {left} {right})"),
                    BinaryOp::Le => format!("({module}.leb {left} {right})"),
                    BinaryOp::Gt => format!("({module}.ltb {right} {left})"),
                    _ => format!("({module}.leb {right} {left})"),
                })
            }
            _ => {
                let domain =
                    domain.ok_or_else(|| internal("arithmetic without a domain".to_owned()))?;
                let module = self.numeric_module(domain)?;
                if *semantics == Some(Semantics::Checked) {
                    self.state.checked_to_total(op_name(*op));
                }
                if *by_zero == Some(ByZero::Abort) {
                    self.state.abort_to_total("division by zero");
                }
                match op {
                    BinaryOp::Add => Ok(format!("({module}.add {left} {right})")),
                    BinaryOp::Mul => Ok(format!("({module}.mul {left} {right})")),
                    // N.sub truncates at zero, which is exactly natural subtraction; a
                    // checked unsigned subtraction agrees with it on non-aborting runs.
                    BinaryOp::Sub => Ok(format!("({module}.sub {left} {right})")),
                    BinaryOp::Div | BinaryOp::Rem => {
                        let division = *op == BinaryOp::Div;
                        if module == "N" {
                            let name = if division { "div" } else { "modulo" };
                            return Ok(format!("(N.{name} {left} {right})"));
                        }
                        match rounding {
                            Some(Rounding::Trunc) => {
                                let name = if division { "quot" } else { "rem" };
                                Ok(format!("(Z.{name} {left} {right})"))
                            }
                            Some(Rounding::Floor) => {
                                let name = if division { "div" } else { "modulo" };
                                Ok(format!("(Z.{name} {left} {right})"))
                            }
                            _ => {
                                self.helpers.insert(Helper::Euclid);
                                let name = if division { "ediv" } else { "emod" };
                                Ok(format!("(ml_Z_{name} {left} {right})"))
                            }
                        }
                    }
                    other => Err(internal(format!("no Rocq arithmetic {}", op_name(*other)))),
                }
            }
        }
    }

    fn match_expr(&mut self, scrutinee: &Expr, cases: &[Case]) -> Result<String> {
        if let Some(subject) = scrutinee.var_name() {
            return self.match_on(subject, &scrutinee.ty, cases);
        }
        let name = "ml_scrutinee";
        let value = self.expr(scrutinee)?;
        Ok(format!(
            "(let {name} := {value} in {})",
            self.match_on(name, &scrutinee.ty, cases)?
        ))
    }

    fn match_on(&mut self, subject: &str, ty: &Type, cases: &[Case]) -> Result<String> {
        let fallback = || {
            cases
                .iter()
                .find(|kase| matches!(kase.pattern, Pattern::Wild | Pattern::Bind { .. }))
        };
        let Type::Data { name: data_name } = ty else {
            // Natural-number matches test for zero; the successor case binds the predecessor.
            let zero = cases
                .iter()
                .find(|kase| matches!(kase.pattern, Pattern::NatZero));
            let succ = cases
                .iter()
                .find(|kase| matches!(kase.pattern, Pattern::NatSucc { .. }));
            let zero_text = match zero {
                Some(zero) => self.expr(&zero.body)?,
                None => self.bind_fallback(subject, fallback())?,
            };
            let succ_text = match succ {
                Some(Case {
                    pattern: Pattern::NatSucc { name },
                    body,
                }) => format!("(let {name} := N.pred {subject} in {})", self.expr(body)?),
                _ => self.bind_fallback(subject, fallback())?,
            };
            return Ok(format!(
                "(if N.eqb {subject} 0%N then {zero_text} else {succ_text})"
            ));
        };
        let program = self.program;
        let entry = program.data(data_name);
        let mut arms = Vec::new();
        for ctor in &entry.ctors {
            let kase = cases
                .iter()
                .find(|item| {
                    matches!(&item.pattern, Pattern::Ctor { ctor: name, .. } if *name == ctor.name)
                })
                .or_else(fallback)
                .expect("the checker makes every match exhaustive");
            let binds: Vec<&str> = match &kase.pattern {
                Pattern::Ctor { binds, .. } => binds
                    .iter()
                    .map(|bind| bind.as_deref().unwrap_or("_"))
                    .collect(),
                _ => ctor.fields.iter().map(|_| "_").collect(),
            };
            let mut body = self.expr(&kase.body)?;
            if let Pattern::Bind { name } = &kase.pattern {
                body = format!("(let {name} := {subject} in {body})");
            }
            let mut head = vec![self.state.ctor_ref(&entry.full_name, &ctor.name, ".")];
            head.extend(binds.into_iter().map(str::to_owned));
            arms.push(format!("| {} => {body}", head.join(" ")));
        }
        Ok(format!("(match {subject} with {} end)", arms.join(" ")))
    }

    fn bind_fallback(&mut self, subject: &str, kase: Option<&Case>) -> Result<String> {
        let kase = kase.expect("the checker makes every match exhaustive");
        let body = self.expr(&kase.body)?;
        Ok(match &kase.pattern {
            Pattern::Bind { name } => format!("(let {name} := {subject} in {body})"),
            _ => body,
        })
    }

    fn text_of(&mut self, arg: &Expr) -> Result<String> {
        let text = self.expr(arg)?;
        match &arg.ty {
            Type::String => Ok(text),
            Type::Bool => {
                self.helpers.insert(Helper::BoolToString);
                Ok(format!("(ml_bool_to_string {text})"))
            }
            Type::Nat => {
                self.helpers.insert(Helper::Digits);
                Ok(format!("(ml_N_to_string {text})"))
            }
            Type::Int => {
                self.helpers.insert(Helper::ZToString);
                Ok(format!("(ml_Z_to_string {text})"))
            }
            Type::Fixed { signed, .. } => {
                self.state.fixed_to_unbounded(&arg.ty);
                if *signed {
                    self.helpers.insert(Helper::ZToString);
                    Ok(format!("(ml_Z_to_string {text})"))
                } else {
                    self.helpers.insert(Helper::Digits);
                    Ok(format!("(ml_N_to_string {text})"))
                }
            }
            other => Err(unsupported(
                "output of structured values",
                &format!("a {} value has no portable textual form", other.kind()),
                arg.span,
            )),
        }
    }

    fn cast(&mut self, arg: &Expr, from: &Type, to: &Type, flavor: Flavor) -> Result<String> {
        let arg = self.expr(arg)?;
        let module = |ty: &Type| match ty {
            Type::Fixed { signed, .. } => {
                if *signed {
                    "Z"
                } else {
                    "N"
                }
            }
            Type::Nat => "N",
            _ => "Z",
        };
        let (from, to) = (module(from), module(to));
        if flavor == Flavor::Checked {
            self.state.checked_to_total("conversion to a natural");
        }
        if from == to {
            return Ok(arg);
        }
        if from == "N" {
            return Ok(format!("(Z.of_N {arg})"));
        }
        Ok(format!("(Z.to_N {arg})"))
    }

    fn main(&mut self, main: &Main) -> Result<String> {
        let effects = rename_main(main, &ident, &self.state.local_reserved());
        let mut assertion = 0;
        let mut theorems: Vec<String> = Vec::new();
        let mut body = String::new();
        for (index, effect) in effects.iter().enumerate() {
            match effect {
                Effect::Print { expr, .. } => {
                    body.push_str(&self.expr(expr)?);
                    body.push_str(" ::\n  ");
                }
                Effect::Let { name, value, .. } => {
                    let value = self.expr(value)?;
                    let _ = write!(body, "let {name} := {value} in\n  ");
                }
                Effect::Assert { prop, .. } => {
                    assertion += 1;
                    let mut lets = String::new();
                    for earlier in &effects[..index] {
                        if let Effect::Let { name, value, .. } = earlier {
                            let value = self.expr(value)?;
                            let _ = write!(lets, "let {name} := {value} in ");
                        }
                    }
                    let name = format!("ml_assertion_{assertion}");
                    let prop = self.prop(prop)?;
                    theorems.push(format!(
                        "Theorem {name} : {lets}{prop}.\nProof. ml_decide. Qed."
                    ));
                    self.helpers.insert(Helper::Decide);
                    self.state.assertion_theorem(&name, effect);
                }
            }
        }
        body.push_str("nil");
        self.state.encode(
            "program-output",
            "main is the list of lines the source program prints, in order; evaluating it with vm_compute runs the program",
        );
        theorems.push(format!("Definition main : list string :=\n  {body}."));
        Ok(theorems.join("\n\n"))
    }
}

/// The operator's name as the checked program spells it.
const fn op_name(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "add",
        BinaryOp::Sub => "sub",
        BinaryOp::Mul => "mul",
        BinaryOp::Div => "div",
        BinaryOp::Rem => "rem",
        BinaryOp::Eq => "eq",
        BinaryOp::Ne => "ne",
        BinaryOp::Lt => "lt",
        BinaryOp::Le => "le",
        BinaryOp::Gt => "gt",
        BinaryOp::Ge => "ge",
        BinaryOp::And => "and",
        BinaryOp::Or => "or",
        BinaryOp::Concat => "concat",
        BinaryOp::Plus => "plus",
    }
}

fn collect_hint_functions(plan: &Plan) -> Vec<&String> {
    match plan {
        Plan::Close { hints } => hints.unfold.iter().collect(),
        Plan::Induction(split) | Plan::Cases(split) => split
            .cases
            .iter()
            .flat_map(|kase| collect_hint_functions(&kase.plan))
            .collect(),
    }
}
