//! The Rocq emitter: writes a checked program as Rocq with its translation contract.
//!
//! Naturals become binary `N` and integers `Z`, so programs run at their
//! source sizes (unary `nat` cannot hold 20!). Structural recursion over data
//! is a `Fixpoint`; recursion that decreases a natural is a `Function` with
//! the measure `N.to_nat`, whose obligations `lia` discharges and whose
//! equation lemma the reconstructed proofs rewrite with. A JavaScript Number
//! is a primitive `float`, which the kernel computes with, so assertions about
//! Numbers are still theorems closed by computation.
//!
//! Mirrors `js/src/translation/emit-rocq.js`.

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::rc::Rc;

use super::Language;
use super::diagnostics::{ErrorKind, Result, TranslationError, unsupported};
use super::emit_common::{
    CtorStyle, EmitOptions, EmitState, Emitted, mutual_groups, order_declarations, unthreaded,
};
use super::ir::{
    ByZero, Case, DataDecl, Decl, Effect, Expr, FnDecl, Hints, LitValue, Main, Node, Param,
    Pattern, Plan, Program, Prop, Semantics, TheoremDecl, rename_function, rename_main,
    rename_theorem,
};
use super::output::thread_output;
use super::proof::prop_functions;
use super::surface::{BinaryOp, Flavor, Rounding, UnaryOp};
use super::types::Type;

mod arrays;
mod expressions;
mod floats;
mod math;
mod mutual;
mod proofs;

use self::arrays::{FLOAT_INDEX, LIST_AT};
pub use self::floats::FLOAT_PRELUDE;
use self::floats::{FLOAT_REM, FLOAT_SAME, JS_CONSOLE, JS_NUMBER};
use self::math::MATH;
use self::mutual::Mutual;

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
    "float",
    "PrimFloat",
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

const FIX: &str = "(* The least fixed point of a one-step unfolding F, unfolded lazily: level k
   nests 2^k calls of F before it would reach the fallback. *)
Fixpoint ml_fix {A B : Type} (k : nat) (F : (A -> B) -> A -> B) (fallback : A -> B) : A -> B :=
  match k with
  | O => F fallback
  | S k => fun a => ml_fix k F (ml_fix k F fallback) a
  end.";

const EMIT: &str = "(* The lines a step of main prints, before the lines the rest of main prints
   and the message of the abort that stops it, if one does. *)
Definition ml_emit (lines : list string) (rest : list string * option string) : list string * option string :=
  (app lines (fst rest), snd rest).";

/// A helper definition the emitted program may need, in the order they are written out.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Helper {
    Digits,
    ZToString,
    BoolToString,
    Euclid,
    JsNumber,
    JsConsole,
    FloatSame,
    FloatRem,
    Math,
    ListAt,
    FloatIndex,
    Fix,
    Tactics,
    Decide,
    Emit,
}

impl Helper {
    const ALL: [Self; 15] = [
        Self::Digits,
        Self::ZToString,
        Self::BoolToString,
        Self::Euclid,
        Self::JsNumber,
        Self::JsConsole,
        Self::FloatSame,
        Self::FloatRem,
        Self::Math,
        Self::ListAt,
        Self::FloatIndex,
        Self::Fix,
        Self::Tactics,
        Self::Decide,
        Self::Emit,
    ];

    const fn text(self) -> &'static str {
        match self {
            Self::Digits => DIGITS,
            Self::ZToString => Z_TO_STRING,
            Self::BoolToString => BOOL_TO_STRING,
            Self::Euclid => EUCLID,
            Self::JsNumber => JS_NUMBER,
            Self::JsConsole => JS_CONSOLE,
            Self::FloatSame => FLOAT_SAME,
            Self::FloatRem => FLOAT_REM,
            Self::Math => MATH,
            Self::ListAt => LIST_AT,
            Self::FloatIndex => FLOAT_INDEX,
            Self::Fix => FIX,
            Self::Tactics => TACTICS,
            Self::Decide => DECIDE,
            Self::Emit => EMIT,
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
pub fn emit_rocq(source: &Program) -> Result<Emitted> {
    let program = &*thread_output(source)?;
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
        uses_float: false,
    }
    .file()
}

/// The function being emitted, so its recursive calls use its local name.
struct Current {
    full_name: String,
    name: String,
    recursive: bool,
    /// Recursive with no termination argument: self-calls go through `ml_fix`.
    general: bool,
    /// Recursive on a natural through more than one threaded self-call: self-calls spend `ml_fuel`.
    fuel: bool,
    /// A member of a mutually recursive group: calls of the group go through `ml_rec`.
    mutual: Option<Rc<Mutual>>,
}

struct RocqEmitter<'p> {
    program: &'p Program,
    state: EmitState<'p>,
    helpers: HashSet<Helper>,
    /// Functions recursive on a natural: the decreasing parameter and the arity.
    nat_functions: HashMap<String, (usize, usize)>,
    current: Option<Current>,
    uses_float: bool,
}

impl RocqEmitter<'_> {
    fn file(mut self) -> Result<Emitted> {
        let program = self.program;
        let mut blocks: Vec<String> = Vec::new();
        let order = order_declarations(program, true)?;
        let mut open_modules: Vec<String> = Vec::new();
        let groups = mutual_groups(program);
        for entry in order {
            self.move_to(&mut open_modules, entry.module_path(), &mut blocks);
            blocks.push(match (entry, groups.get(entry.full_name())) {
                (_, Some(group)) if std::ptr::eq(group[0], entry) => self.mutual(group)?,
                (_, Some(_)) => continue,
                (Decl::Data(data), None) => self.data(entry, data)?,
                (Decl::Fn(function), None) => self.function(entry, function)?,
                (Decl::Theorem(theorem), None) => self.theorem(entry, theorem)?,
            });
        }
        self.move_to(&mut open_modules, &[], &mut blocks);
        let main_text = match &program.main {
            Some(main) => Some(self.main(main)?),
            None => None,
        };
        let helper_text = Helper::ALL.into_iter().filter(|helper| {
            self.helpers.contains(helper)
                || (*helper == Helper::Digits
                    && (self.helpers.contains(&Helper::ZToString)
                        || self.helpers.contains(&Helper::JsNumber)))
        });
        let mut lines: Vec<String> = vec![format!(
            "(* Translated from {} by meta-language: portable core, Rocq target. *)",
            program.source_language.as_str()
        )];
        let prelude = if self.uses_float {
            FLOAT_PRELUDE
        } else {
            ROCQ_PRELUDE
        };
        lines.extend(prelude.iter().map(|line| (*line).to_owned()));
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
                self.state
                    .machine_integer(ty, if *signed { "Z" } else { "N" });
                if *signed { "Z" } else { "N" }.to_owned()
            }
            Type::Float => {
                self.floats();
                "float".to_owned()
            }
            Type::Bool => "bool".to_owned(),
            Type::String => "string".to_owned(),
            Type::Unit => "unit".to_owned(),
            Type::Output => "list string".to_owned(),
            Type::Data { name } => self.state.reference(name, "."),
            Type::Array { element } => {
                self.state.encode(
                    "arrays",
                    "a JavaScript array, which the portable core never mutates, is a Rocq list; a read walks it to the index",
                );
                format!("(list {})", self.ty(element)?)
            }
            other => return Err(internal(format!("no Rocq type for {}", other.kind()))),
        })
    }

    fn data(&mut self, entry: &Decl, data: &DataDecl) -> Result<String> {
        fn lists(ty: &Type) -> bool {
            ty.element()
                .is_some_and(|element| matches!(element, Type::Data { .. }) || lists(element))
        }
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
        // A type nested through list has no registered All scheme, which Rocq would warn of on every definition.
        let nested = data
            .ctors
            .iter()
            .any(|ctor| ctor.fields.iter().any(|field| lists(&field.ty)));
        if nested {
            lines.insert(0, "#[warnings=\"-register-all\"]".to_owned());
        }
        Ok(lines.join("\n") + ".")
    }

    /// Recursion that decreases a natural by one each call, in a function whose
    /// output or abort is threaded, is a structural fixpoint over a fuel one more
    /// than the natural: every call spends one fuel and decreases the natural, so
    /// the fuel never runs out before the natural reaches zero.
    #[allow(clippy::too_many_arguments)]
    fn fuel_recursion(
        &mut self,
        name: &str,
        params: &[Param],
        binders: &str,
        result: &str,
        text: &str,
        decreasing: &str,
        ret: &Type,
    ) -> Result<String> {
        self.state.encode(
            "nat-fuel-recursion",
            "a threaded function that decreases a natural through more than one call is a fixpoint over a fuel of one more than the natural, which every call spends as it decreases the natural, so the fuel outlasts every run",
        );
        let args: Vec<&str> = params.iter().map(|param| param.name.as_str()).collect();
        Ok(format!(
            "Definition {name}{binders} : {result} :=\n  (fix ml_go (ml_fuel : nat){binders} {{struct ml_fuel}} : {result} :=\n    match ml_fuel with\n    | O => {}\n    | S ml_fuel => {text}\n    end) (S (N.to_nat {decreasing})) {}.",
            self.inhabitant(ret)?,
            args.join(" ")
        ))
    }

    /// Recursion with no termination argument Rocq could check is `ml_fix`
    /// applied to the function's one-step unfolding: the parameters travel as
    /// one tuple, a self-call is a call of `ml_rec`, and a run nested deeper
    /// than 2^64 calls, which no machine reaches, would fall back to an inhabitant.
    fn general_recursion(
        &mut self,
        name: &str,
        params: &[Param],
        binders: &str,
        result: &str,
        text: &str,
        ret: &Type,
    ) -> Result<String> {
        self.helpers.insert(Helper::Fix);
        self.state.encode(
            "general-recursion",
            "recursion without a termination argument Rocq could check is ml_fix over the function's one-step unfolding, which unfolds lazily to 2^64 nested calls: every run that terminates computes the same value, and Rocq accepts it as a plain Definition",
        );
        let mut domain = Vec::new();
        for param in params {
            domain.push(self.ty(&param.ty)?);
        }
        let domain = if domain.is_empty() {
            "unit".to_owned()
        } else {
            domain.join(" * ")
        };
        let names: Vec<&str> = params.iter().map(|param| param.name.as_str()).collect();
        let tuple = format!("({})", names.join(", "));
        let (input, unpack, args) = match names.as_slice() {
            [] => ("_", String::new(), "tt"),
            [only] => (*only, String::new(), *only),
            _ => (
                "ml_args",
                format!("let '{tuple} := ml_args in "),
                tuple.as_str(),
            ),
        };
        Ok(format!(
            "Definition {name}{binders} : {result} :=\n  ml_fix 64 (fun (ml_rec : {domain} -> {result}) ({input} : {domain}) =>\n    {unpack}{text})\n    (fun _ => {}) {args}.",
            self.inhabitant(ret)?
        ))
    }

    fn function(&mut self, entry: &Decl, function: &FnDecl) -> Result<String> {
        let (params, body) = rename_function(function, &ident, &self.state.local_reserved());
        let name = self.state.local_name(&function.full_name).to_owned();
        self.state.map(entry, &name);
        let general = function.recursive && function.decreasing.is_none() && !function.mutual;
        // A threaded function calling itself more than once matches on one call's pair around the next, whose equation Function cannot generate.
        let fuel = !general
            && function.recursive
            && !function.mutual
            && !function
                .decreasing
                .and_then(|index| params.get(index))
                .is_some_and(|param| matches!(param.ty, Type::Data { .. }))
            && matches!(&function.ret, Type::Data { name: ret } if matches!(self.program.declaration(ret), Some(Decl::Data(data)) if data.output))
            && self_calls(&body, &function.full_name) > 1;
        self.current = Some(Current {
            full_name: function.full_name.clone(),
            name: name.clone(),
            recursive: function.recursive,
            general,
            fuel,
            mutual: None,
        });
        let mut binders = String::new();
        for param in &params {
            let ty = self.ty(&param.ty)?;
            let _ = write!(binders, " ({} : {ty})", param.name);
        }
        let result = self.ty(&function.ret)?;
        let text = self.expr(&body)?;
        self.current = None;
        if !function.recursive {
            return Ok(format!(
                "Definition {name}{binders} : {result} :=\n  {text}."
            ));
        }
        let Some(index) = function.decreasing else {
            return self.general_recursion(&name, &params, &binders, &result, &text, &function.ret);
        };
        let decreasing = &params[index];
        if matches!(decreasing.ty, Type::Data { .. }) {
            return Ok(format!(
                "Fixpoint {name}{binders} {{struct {}}} : {result} :=\n  {text}.",
                decreasing.name
            ));
        }
        if fuel {
            return self.fuel_recursion(
                &name,
                &params,
                &binders,
                &result,
                &text,
                &decreasing.name,
                &function.ret,
            );
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

/// How many times a function body calls the function itself.
fn self_calls(node: &Expr, name: &str) -> usize {
    let own = usize::from(matches!(&node.node, Node::Call { func, .. } if func == name));
    own + node
        .children()
        .into_iter()
        .map(|child| self_calls(child, name))
        .sum::<usize>()
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
