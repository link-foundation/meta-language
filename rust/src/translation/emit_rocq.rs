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

use super::diagnostics::{unsupported, ErrorKind, Result, TranslationError};
use super::emit_common::{order_declarations, CtorStyle, EmitOptions, EmitState, Emitted};
use super::ir::{
    rename_function, rename_main, rename_theorem, ByZero, Case, DataDecl, Decl, Effect, Expr,
    FnDecl, Hints, LitValue, Main, Node, Param, Pattern, Plan, Program, Prop, Semantics,
    TheoremDecl,
};
use super::proof::prop_functions;
use super::surface::{BinaryOp, Flavor, Rounding, UnaryOp};
use super::types::Type;
use super::Language;

mod expressions;
mod floats;
mod proofs;

pub use self::floats::FLOAT_PRELUDE;
use self::floats::{FLOAT_REM, FLOAT_SAME, JS_CONSOLE, JS_NUMBER};

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
    Fix,
    Tactics,
    Decide,
}

impl Helper {
    const ALL: [Self; 11] = [
        Self::Digits,
        Self::ZToString,
        Self::BoolToString,
        Self::Euclid,
        Self::JsNumber,
        Self::JsConsole,
        Self::FloatSame,
        Self::FloatRem,
        Self::Fix,
        Self::Tactics,
        Self::Decide,
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
            Self::Fix => FIX,
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
                self.state.fixed_to_unbounded(ty);
                if *signed { "Z" } else { "N" }.to_owned()
            }
            Type::Float => {
                self.floats();
                "float".to_owned()
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
        self.current = Some(Current {
            full_name: function.full_name.clone(),
            name: name.clone(),
            recursive: function.recursive,
            general: function.recursive && function.decreasing.is_none() && !function.mutual,
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
            return self.general_recursion(&name, &params, &binders, &result, &text, &function.ret);
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
