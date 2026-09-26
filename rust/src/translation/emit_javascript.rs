//! The JavaScript emitter: writes a checked program as a strict ES module
//! with its translation contract.
//!
//! Every portable number is a `BigInt`, so naturals and integers stay unbounded
//! and machine integers are range-checked exactly as Rust checks them. Data
//! values are plain objects whose `$` property names the constructor. Modules
//! become object literals referenced by qualified names. Theorems cannot be
//! proved in JavaScript: each becomes an executable property, checked over a
//! bounded domain by `--ml-check-theorems`, while the proof obligation stays
//! discharged by the source language's kernel.
//!
//! Mirrors `js/src/translation/emit-javascript.js`.

use std::collections::BTreeSet;

use super::diagnostics::{type_error, unsupported, Result, TranslationError};
use super::emit_common::{EmitOptions, EmitState, Emitted};
use super::ir::{
    rename_function, rename_main, rename_theorem, Binder, ByZero, Case, Ctor, Decl, Effect, Expr,
    FnDecl, LitValue, Main, Node, Param, Pattern, Program, Prop, Semantics, TheoremDecl,
};
use super::lexer::json_string;
use super::surface::{BinaryOp, Flavor, Rounding, UnaryOp};
use super::types::{fixed_bounds, Type};
use super::Language;

const KEYWORDS: &[&str] = &[
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "enum",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "function",
    "if",
    "import",
    "in",
    "instanceof",
    "new",
    "null",
    "return",
    "super",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "typeof",
    "var",
    "void",
    "while",
    "with",
    "yield",
    "await",
    "let",
    "static",
    "implements",
    "interface",
    "package",
    "private",
    "protected",
    "public",
    "arguments",
    "eval",
    "undefined",
    "NaN",
    "Infinity",
    "globalThis",
    "console",
    "process",
    "BigInt",
    "String",
    "Object",
    "Error",
    "RangeError",
    "Math",
    "Number",
    "Array",
    "JSON",
    "Symbol",
    "main",
    "__proto__",
    "constructor",
    "prototype",
    "of",
    "async",
    "get",
    "set",
];

/// Runtime helpers, declared in the order they are written to the file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Helper {
    NatSub,
    Fixed,
    Divide,
    ToNatChecked,
    Abort,
    Assert,
    Equal,
    Forall,
    Domains,
}

impl Helper {
    const fn text(self) -> &'static str {
        match self {
            Self::NatSub => {
                "function ml_natSub(a, b) {
  return a > b ? a - b : 0n;
}"
            }
            Self::Fixed => {
                "function ml_fixed(value, min, max, what) {
  if (value < min || value > max) throw new RangeError(`${what} overflowed`);
  return value;
}"
            }
            Self::Divide => {
                "// Integer division with the source's rounding; by zero it either aborts or is total (x / 0 = 0, x % 0 = x).
function ml_divide(a, b, rounding, byZero, remainder) {
  if (b === 0n) {
    if (byZero === 'abort') throw new RangeError('division by zero');
    return remainder ? a : 0n;
  }
  let q = a / b;
  const r = a - q * b;
  if (r !== 0n) {
    if (rounding === 'floor' && (r < 0n) !== (b < 0n)) q -= 1n;
    if (rounding === 'euclid' && r < 0n) q = b > 0n ? q - 1n : q + 1n;
  }
  return remainder ? a - q * b : q;
}"
            }
            Self::ToNatChecked => {
                "function ml_toNatChecked(value) {
  if (value < 0n) throw new RangeError(`${value} is not a natural number`);
  return value;
}"
            }
            Self::Abort => {
                "function ml_abort(message) {
  throw new Error(message);
}"
            }
            Self::Assert => {
                "function ml_assert(holds, statement) {
  if (!holds) throw new Error(`assertion failed: ${statement}`);
}"
            }
            Self::Equal => {
                "function ml_equal(left, right) {
  if (typeof left !== 'object' || left === null) return left === right;
  const keys = Object.keys(left);
  return keys.length === Object.keys(right).length && keys.every((key) => ml_equal(left[key], right[key]));
}"
            }
            Self::Forall => {
                "function ml_forall(values, property) {
  return values.every(property);
}"
            }
            Self::Domains => {
                "// Bounded domains for executable theorem checks.
const ml_nat = [0n, 1n, 2n, 3n, 4n, 5n, 6n];
const ml_int = [-4n, -3n, -2n, -1n, 0n, 1n, 2n, 3n, 4n];
const ml_small_nat = [0n, 1n, 2n];
const ml_small_int = [-1n, 0n, 1n];
function ml_product(lists) {
  return lists.reduce((rows, list) => rows.flatMap((row) => list.map((value) => [...row, value])), [[]]);
}"
            }
        }
    }
}

fn ident(name: &str) -> String {
    let mut result: String = name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' {
                character
            } else {
                '_'
            }
        })
        .collect();
    if result.starts_with(|character: char| character.is_ascii_digit()) {
        result.insert(0, 'x');
    }
    if KEYWORDS.contains(&result.as_str()) {
        result.push('_');
    }
    result
}

fn field_key(name: &str) -> String {
    ident(name)
}

fn indent(text: &str, depth: usize) -> String {
    let pad = "  ".repeat(depth);
    text.split('\n')
        .map(|line| {
            if line.is_empty() {
                String::new()
            } else {
                format!("{pad}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Top-level members are method-shaped (`name(…) {`); at the root they become function declarations.
fn declare_function(text: &str) -> String {
    let name = text
        .find(|character: char| {
            !(character.is_ascii_alphanumeric() || character == '_' || character == '$')
        })
        .unwrap_or(text.len());
    if name > 0 && text[name..].starts_with('(') {
        format!("function {text}")
    } else {
        text.to_owned()
    }
}

/// The error JavaScript throws on a node the checker never produces.
fn malformed(message: String) -> TranslationError {
    type_error(message, None)
}

fn ctor_object(ctor: &Ctor, args: &[String]) -> String {
    let fields: Vec<String> = ctor
        .fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
            let arg = args.get(index).map_or("undefined", String::as_str);
            format!("{}: {arg}", field_key(&field.name))
        })
        .collect();
    let fields = if fields.is_empty() {
        String::new()
    } else {
        format!(", {}", fields.join(", "))
    };
    format!(
        "Object.freeze({{ $: '{}'{fields} }})",
        ctor.name.replace('\'', "\\'")
    )
}

/// Emits a checked program as JavaScript.
///
/// # Errors
/// On constructs the target cannot express faithfully.
pub fn emit_javascript(program: &Program) -> Result<Emitted> {
    let state = EmitState::new(
        program,
        Language::JavaScript,
        ident,
        KEYWORDS,
        EmitOptions {
            modules_share_term_space: true,
            type_space: true,
            ..EmitOptions::default()
        },
    );
    JavaScriptEmitter {
        program,
        state,
        helpers: BTreeSet::new(),
        temporaries: 0,
        theorem_checks: Vec::new(),
    }
    .file()
}

/// Declarations grouped by module, in source order.
#[derive(Default)]
struct ModuleTree<'p> {
    items: Vec<&'p Decl>,
    modules: Vec<(String, Self)>,
}

#[derive(Clone)]
struct TheoremCheck {
    reference: String,
    binders: Vec<Binder>,
    source: String,
}

struct JavaScriptEmitter<'p> {
    program: &'p Program,
    state: EmitState<'p>,
    helpers: BTreeSet<Helper>,
    temporaries: usize,
    theorem_checks: Vec<TheoremCheck>,
}

impl<'p> JavaScriptEmitter<'p> {
    fn file(mut self) -> Result<Emitted> {
        let program = self.program;
        let mut tree = ModuleTree::default();
        for entry in &program.declarations {
            let mut node = &mut tree;
            for segment in entry.module_path() {
                let index = if let Some(index) =
                    node.modules.iter().position(|(name, _)| name == segment)
                {
                    index
                } else {
                    node.modules.push((segment.clone(), ModuleTree::default()));
                    node.modules.len() - 1
                };
                node = &mut node.modules[index].1;
            }
            node.items.push(entry);
        }
        let mut blocks = Vec::new();
        for entry in &tree.items {
            if let Some(text) = self.declaration(entry)? {
                blocks.push(declare_function(&text));
            }
        }
        for (segment, node) in &tree.modules {
            let path = [segment.clone()];
            let name = self.module_name(&path);
            let object = self.module_object(node, &path)?;
            blocks.push(format!("const {name} = {object};"));
        }
        let main = match &program.main {
            Some(main) => Some(self.main(main)?),
            None => None,
        };
        let theorems = !self.theorem_checks.is_empty();
        if theorems {
            self.helpers.insert(Helper::Domains);
            self.helpers.insert(Helper::Forall);
        }
        let mut entry = Vec::new();
        if theorems {
            entry.push(self.theorem_runner()?);
        }
        entry.extend(main.clone());
        if theorems {
            entry.push(
                "if (process.argv.includes('--ml-check-theorems')) ml_checkTheorems();\nelse main();"
                    .to_owned(),
            );
        } else if main.is_some() {
            entry.push("main();".to_owned());
        }
        let mut lines = vec![
            format!(
                "// Translated from {} by meta-language: portable core, JavaScript target.",
                program.source_language.as_str()
            ),
            "'use strict';".to_owned(),
            String::new(),
        ];
        for helper in &self.helpers {
            lines.push(helper.text().to_owned());
            lines.push(String::new());
        }
        for block in blocks.into_iter().chain(entry) {
            lines.push(block);
            lines.push(String::new());
        }
        self.state.encode("numbers", "every natural, integer and machine integer is a BigInt; machine-integer results are range-checked and throw RangeError where Rust would panic");
        Ok(self
            .state
            .finish(lines.join("\n"), main.map(|_| "main".to_owned())))
    }

    fn module_name(&self, path: &[String]) -> String {
        self.state
            .module_name(path)
            .unwrap_or("undefined")
            .to_owned()
    }

    fn module_object(&mut self, node: &ModuleTree<'p>, path: &[String]) -> Result<String> {
        let mut members = Vec::new();
        for entry in &node.items {
            if let Some(text) = self.declaration(entry)? {
                members.push(indent(&format!("{text},"), 1));
            }
        }
        for (segment, child) in &node.modules {
            let mut child_path = path.to_vec();
            child_path.push(segment.clone());
            let name = self.module_name(&child_path);
            let object = self.module_object(child, &child_path)?;
            members.push(indent(&format!("{name}: {object},"), 1));
        }
        Ok(format!("{{\n{}\n}}", members.join("\n")))
    }

    fn declaration(&mut self, entry: &Decl) -> Result<Option<String>> {
        match entry {
            Decl::Data(data) => {
                let name = self.state.local_name(&data.full_name).to_owned();
                self.state.map(entry, &name);
                self.state.encode("data", "a data value is a frozen object whose $ property names its constructor and whose other properties are its fields");
                Ok(None)
            }
            Decl::Fn(function) => self.function(entry, function).map(Some),
            Decl::Theorem(theorem) => self.theorem(entry, theorem).map(Some),
        }
    }

    fn function(&mut self, entry: &Decl, function: &FnDecl) -> Result<String> {
        let (params, body) = rename_function(function, &ident, &self.state.local_reserved());
        let name = self.state.local_name(&function.full_name).to_owned();
        self.state.map(entry, &name);
        let mut lines: Vec<String> = params
            .iter()
            .filter_map(|param| self.parameter_guard(param))
            .collect();
        lines.extend(self.statements(&body)?);
        let names: Vec<&str> = params.iter().map(|param| param.name.as_str()).collect();
        Ok(format!(
            "{name}({}) {{\n{}\n}}",
            names.join(", "),
            indent(&lines.join("\n"), 1)
        ))
    }

    /// Machine-integer parameters are range-checked, as the Rust type guarantees.
    fn parameter_guard(&mut self, param: &Param) -> Option<String> {
        let Type::Fixed { bits, signed } = param.ty else {
            return None;
        };
        self.helpers.insert(Helper::Fixed);
        let (min, max) = fixed_bounds(bits, signed);
        Some(format!(
            "ml_fixed({name}, {min}n, {max}n, '{key} argument {name}');",
            name = param.name,
            key = param.ty.key()
        ))
    }

    fn theorem(&mut self, entry: &Decl, theorem: &TheoremDecl) -> Result<String> {
        let (binders, prop, _) = rename_theorem(theorem, &ident, &self.state.local_reserved());
        let name = self.state.local_name(&theorem.full_name).to_owned();
        self.state.map(entry, &name);
        self.state
            .theorem(&theorem.full_name, &name, binders.is_empty(), true);
        self.theorem_checks.push(TheoremCheck {
            reference: self.state.reference(&theorem.full_name, "."),
            binders: binders.clone(),
            source: theorem.full_name.clone(),
        });
        let names: Vec<&str> = binders.iter().map(|binder| binder.name.as_str()).collect();
        Ok(format!(
            "{name}({}) {{\n  return {};\n}}",
            names.join(", "),
            self.prop(&prop)?
        ))
    }

    fn theorem_runner(&mut self) -> Result<String> {
        self.state.encode("theorem-properties", "each theorem is an executable property; --ml-check-theorems evaluates it on every input of a bounded domain, and its proof remains checked by the source kernel");
        let mut checks = Vec::new();
        for check in self.theorem_checks.clone() {
            let mut domains = Vec::new();
            for binder in &check.binders {
                domains.push(self.domain(&binder.ty, 3)?);
            }
            let TheoremCheck {
                reference, source, ..
            } = check;
            let call = if domains.is_empty() {
                format!("{reference}()")
            } else {
                format!(
                    "ml_product([{}]).every((args) => {reference}(...args))",
                    domains.join(", ")
                )
            };
            checks.push(format!("  if (!({call})) throw new Error('theorem {source} fails on a bounded input');\n  console.log('theorem {source}: holds on the bounded domain');"));
        }
        Ok(format!(
            "function ml_checkTheorems() {{\n{}\n}}",
            checks.join("\n")
        ))
    }

    /// Values of a type up to a constructor depth, as JavaScript source.
    fn domain(&mut self, ty: &Type, depth: i64) -> Result<String> {
        self.helpers.insert(Helper::Domains);
        let values = match ty {
            Type::Nat => if depth >= 3 { "ml_nat" } else { "ml_small_nat" }.to_owned(),
            Type::Int => if depth >= 3 { "ml_int" } else { "ml_small_int" }.to_owned(),
            Type::Fixed { signed, .. } => if *signed {
                "ml_small_int"
            } else {
                "ml_small_nat"
            }
            .to_owned(),
            Type::Bool => "[false, true]".to_owned(),
            Type::String => "['', 'a', 'ab']".to_owned(),
            Type::Unit => "[null]".to_owned(),
            Type::Data { name } => self.data_domain(name, depth)?,
            other => {
                return Err(malformed(format!(
                    "no JavaScript domain for {}",
                    other.kind()
                )))
            }
        };
        Ok(values)
    }

    fn data_domain(&mut self, name: &str, depth: i64) -> Result<String> {
        let entry = self.program.data(name);
        let mut values = Vec::new();
        for ctor in &entry.ctors {
            let recursive = ctor
                .fields
                .iter()
                .any(|field| matches!(field.ty, Type::Data { .. }));
            if recursive && depth <= 1 {
                continue;
            }
            let mut fields = Vec::new();
            for field in &ctor.fields {
                let field_depth = if matches!(field.ty, Type::Data { .. }) {
                    depth - 1
                } else {
                    1
                };
                fields.push(self.domain(&field.ty, field_depth)?);
            }
            if fields.is_empty() {
                values.push(format!("[{}]", ctor_object(ctor, &[])));
            } else {
                let args: Vec<String> = (0..ctor.fields.len())
                    .map(|index| format!("args[{index}]"))
                    .collect();
                values.push(format!(
                    "ml_product([{}]).map((args) => {})",
                    fields.join(", "),
                    ctor_object(ctor, &args)
                ));
            }
        }
        if values.is_empty() {
            return Ok("[]".to_owned());
        }
        let spread: Vec<String> = values.iter().map(|value| format!("...{value}")).collect();
        Ok(format!("[{}]", spread.join(", ")))
    }

    fn prop(&mut self, prop: &Prop) -> Result<String> {
        Ok(match prop {
            Prop::Forall { binders, body } => {
                let mut inner = self.prop(body)?;
                for binder in binders.iter().rev() {
                    let domain = self.domain(&binder.ty, 3)?;
                    inner = format!("ml_forall({domain}, ({}) => {inner})", binder.name);
                }
                inner
            }
            Prop::And { left, right } => {
                format!("({} && {})", self.prop(left)?, self.prop(right)?)
            }
            Prop::Or { left, right } => {
                format!("({} || {})", self.prop(left)?, self.prop(right)?)
            }
            Prop::Implies { left, right } => {
                format!("(!{} || {})", self.prop(left)?, self.prop(right)?)
            }
            Prop::Not { arg } => format!("!{}", self.prop(arg)?),
            Prop::Bool { expr } => self.expr(expr)?,
            Prop::Eq(comparison) | Prop::Ne(comparison) => {
                let left = self.expr(&comparison.left)?;
                let right = self.expr(&comparison.right)?;
                let structured = matches!(comparison.left.ty, Type::Data { .. } | Type::Unit);
                let equal = if structured {
                    self.helpers.insert(Helper::Equal);
                    format!("ml_equal({left}, {right})")
                } else {
                    format!("({left} === {right})")
                };
                if matches!(prop, Prop::Eq(_)) {
                    equal
                } else {
                    format!("!{equal}")
                }
            }
            Prop::Lt(comparison)
            | Prop::Le(comparison)
            | Prop::Gt(comparison)
            | Prop::Ge(comparison) => {
                let operator = match prop {
                    Prop::Lt(_) => "<",
                    Prop::Le(_) => "<=",
                    Prop::Gt(_) => ">",
                    _ => ">=",
                };
                format!(
                    "({} {operator} {})",
                    self.expr(&comparison.left)?,
                    self.expr(&comparison.right)?
                )
            }
        })
    }

    /// A function body as statements that return its value.
    fn statements(&mut self, e: &Expr) -> Result<Vec<String>> {
        match &e.node {
            Node::Let { name, value, body } => {
                let mut lines = vec![format!("const {name} = {};", self.expr(value)?)];
                lines.extend(self.statements(body)?);
                Ok(lines)
            }
            Node::If {
                cond,
                then,
                otherwise,
            } => {
                let cond = self.expr(cond)?;
                let then = self.statements(then)?.join("\n");
                let otherwise = self.statements(otherwise)?.join("\n");
                Ok(vec![
                    format!("if ({cond}) {{"),
                    indent(&then, 1),
                    "} else {".to_owned(),
                    indent(&otherwise, 1),
                    "}".to_owned(),
                ])
            }
            Node::Match { scrutinee, cases } => self.match_statements(scrutinee, cases),
            Node::Abort { message } => {
                self.helpers.insert(Helper::Abort);
                Ok(vec![format!("return ml_abort({});", json_string(message))])
            }
            _ => Ok(vec![format!("return {};", self.expr(e)?)]),
        }
    }

    /// The body of the first wildcard or binding case, with the binding declared.
    fn fallback_body(&mut self, fallback: Option<&Case>, subject: &str) -> Result<Vec<String>> {
        // A checked match is exhaustive, so a missing case always has a fallback.
        let kase = fallback.ok_or_else(|| {
            malformed("Cannot read properties of undefined (reading 'pattern')".to_owned())
        })?;
        let mut lines = Vec::new();
        if let Pattern::Bind { name } = &kase.pattern {
            lines.push(format!("const {name} = {subject};"));
        }
        lines.extend(self.statements(&kase.body)?);
        Ok(lines)
    }

    fn match_statements(&mut self, scrutinee: &Expr, cases: &[Case]) -> Result<Vec<String>> {
        let mut lines = Vec::new();
        let subject = if let Some(name) = scrutinee.var_name() {
            name.to_owned()
        } else {
            self.temporaries += 1;
            let subject = format!("ml_subject{}", self.temporaries);
            lines.push(format!("const {subject} = {};", self.expr(scrutinee)?));
            subject
        };
        let fallback = cases
            .iter()
            .find(|kase| matches!(kase.pattern, Pattern::Wild | Pattern::Bind { .. }));
        let Type::Data { name: data } = &scrutinee.ty else {
            let zero = cases
                .iter()
                .find(|kase| matches!(kase.pattern, Pattern::NatZero));
            let succ = cases
                .iter()
                .find(|kase| matches!(kase.pattern, Pattern::NatSucc { .. }));
            let zero_lines = match zero {
                Some(kase) => self.statements(&kase.body)?,
                None => self.fallback_body(fallback, &subject)?,
            };
            let succ_lines = match succ {
                Some(Case {
                    pattern: Pattern::NatSucc { name },
                    body,
                }) => {
                    let mut lines = vec![format!("const {name} = {subject} - 1n;")];
                    lines.extend(self.statements(body)?);
                    lines
                }
                _ => self.fallback_body(fallback, &subject)?,
            };
            lines.extend([
                format!("if ({subject} === 0n) {{"),
                indent(&zero_lines.join("\n"), 1),
                "} else {".to_owned(),
                indent(&succ_lines.join("\n"), 1),
                "}".to_owned(),
            ]);
            return Ok(lines);
        };
        let entry = self.program.data(data);
        lines.push(format!("switch ({subject}.$) {{"));
        for kase in cases {
            let Pattern::Ctor { ctor, binds, .. } = &kase.pattern else {
                continue;
            };
            let ctor = entry
                .ctors
                .iter()
                .find(|candidate| candidate.name == *ctor)
                .unwrap_or_else(|| panic!("{data} has no constructor {ctor}"));
            let mut body: Vec<String> = binds
                .iter()
                .enumerate()
                .filter_map(|(index, bind)| {
                    bind.as_ref().map(|bind| {
                        format!(
                            "const {bind} = {subject}.{};",
                            field_key(&ctor.fields[index].name)
                        )
                    })
                })
                .collect();
            lines.push(format!("  case '{}': {{", ctor.name.replace('\'', "\\'")));
            body.extend(self.statements(&kase.body)?);
            lines.push(indent(&body.join("\n"), 2));
            lines.push("  }".to_owned());
        }
        lines.push("  default: {".to_owned());
        if fallback.is_some() {
            let body = self.fallback_body(fallback, &subject)?;
            lines.push(indent(&body.join("\n"), 2));
        } else {
            lines.push(format!(
                "    throw new TypeError(`unexpected constructor ${{{subject}.$}}`);"
            ));
        }
        lines.push("  }".to_owned());
        lines.push("}".to_owned());
        Ok(lines)
    }

    fn expr(&mut self, e: &Expr) -> Result<String> {
        match &e.node {
            Node::Lit { value } => literal(&e.ty, value),
            Node::Unit => Ok("null".to_owned()),
            Node::Var { name } => Ok(name.clone()),
            Node::Call { func, args } => {
                let target = self.state.reference(func, ".");
                let args = self.exprs(args)?;
                Ok(format!("{target}({})", args.join(", ")))
            }
            Node::Ctor { data, ctor, args } => {
                let entry = self.program.data(data);
                let ctor = entry
                    .ctors
                    .iter()
                    .find(|candidate| candidate.name == *ctor)
                    .unwrap_or_else(|| panic!("{data} has no constructor {ctor}"));
                let args = self.exprs(args)?;
                Ok(ctor_object(ctor, &args))
            }
            Node::Unary { op, arg, .. } => {
                let arg = self.expr(arg)?;
                Ok(match op {
                    UnaryOp::Not => format!("!{arg}"),
                    UnaryOp::Neg => self.checked(format!("-{arg}"), &e.ty, "negation"),
                })
            }
            Node::Binary { .. } => self.binary(e),
            Node::If {
                cond,
                then,
                otherwise,
            } => Ok(format!(
                "({} ? {} : {})",
                self.expr(cond)?,
                self.expr(then)?,
                self.expr(otherwise)?
            )),
            Node::Let { .. } | Node::Match { .. } => Ok(format!(
                "(() => {{\n{}\n}})()",
                indent(&self.statements(e)?.join("\n"), 1)
            )),
            Node::ToString { arg } => {
                if matches!(arg.ty, Type::String) {
                    self.expr(arg)
                } else {
                    self.text_of(arg)
                }
            }
            Node::Cast { arg, flavor, .. } => self.cast(arg, *flavor),
            Node::Abort { message } => {
                self.helpers.insert(Helper::Abort);
                Ok(format!("ml_abort({})", json_string(message)))
            }
        }
    }

    fn exprs(&mut self, list: &[Expr]) -> Result<Vec<String>> {
        list.iter().map(|arg| self.expr(arg)).collect()
    }

    fn text_of(&mut self, arg: &Expr) -> Result<String> {
        if matches!(arg.ty, Type::Data { .. } | Type::Unit) {
            return Err(unsupported(
                "output of structured values",
                &format!("a {} value has no portable textual form", arg.ty.kind()),
                arg.span,
            ));
        }
        Ok(format!("String({})", self.expr(arg)?))
    }

    fn checked(&mut self, text: String, ty: &Type, what: &str) -> String {
        let Type::Fixed { bits, signed } = *ty else {
            return text;
        };
        self.helpers.insert(Helper::Fixed);
        let (min, max) = fixed_bounds(bits, signed);
        format!("ml_fixed({text}, {min}n, {max}n, '{} {what}')", ty.key())
    }

    fn binary(&mut self, e: &Expr) -> Result<String> {
        let Node::Binary {
            op,
            left,
            right,
            semantics,
            rounding,
            by_zero,
            ..
        } = &e.node
        else {
            unreachable!("binary called on a {} node", e.kind());
        };
        let left = self.expr(left)?;
        let right = self.expr(right)?;
        let comparison = |operator: &str| format!("({left} {operator} {right})");
        Ok(match op {
            BinaryOp::And => comparison("&&"),
            BinaryOp::Or => comparison("||"),
            BinaryOp::Concat => comparison("+"),
            BinaryOp::Eq => comparison("==="),
            BinaryOp::Ne => comparison("!=="),
            BinaryOp::Lt => comparison("<"),
            BinaryOp::Le => comparison("<="),
            BinaryOp::Gt => comparison(">"),
            BinaryOp::Ge => comparison(">="),
            BinaryOp::Add => self.checked(comparison("+"), &e.ty, "addition"),
            BinaryOp::Mul => self.checked(comparison("*"), &e.ty, "multiplication"),
            BinaryOp::Sub => {
                if *semantics == Some(Semantics::Truncated) {
                    self.helpers.insert(Helper::NatSub);
                    format!("ml_natSub({left}, {right})")
                } else {
                    self.checked(comparison("-"), &e.ty, "subtraction")
                }
            }
            BinaryOp::Div | BinaryOp::Rem => {
                self.helpers.insert(Helper::Divide);
                let rounding = match rounding {
                    Some(Rounding::Trunc) => "trunc",
                    Some(Rounding::Euclid) => "euclid",
                    Some(Rounding::Floor) => "floor",
                    None => "undefined",
                };
                let by_zero = match by_zero {
                    Some(ByZero::Abort) => "abort",
                    Some(ByZero::Total) => "total",
                    None => "undefined",
                };
                let remainder = *op == BinaryOp::Rem;
                let call =
                    format!("ml_divide({left}, {right}, '{rounding}', '{by_zero}', {remainder})");
                let what = if remainder { "remainder" } else { "division" };
                self.checked(call, &e.ty, what)
            }
            BinaryOp::Plus => return Err(malformed("no JavaScript operator plus".to_owned())),
        })
    }

    fn cast(&mut self, arg: &Expr, flavor: Flavor) -> Result<String> {
        let arg = self.expr(arg)?;
        Ok(match flavor {
            Flavor::Exact => arg,
            Flavor::Clamp => format!("((value) => (value < 0n ? 0n : value))({arg})"),
            Flavor::Checked => {
                self.helpers.insert(Helper::ToNatChecked);
                format!("ml_toNatChecked({arg})")
            }
        })
    }

    fn main(&mut self, main: &Main) -> Result<String> {
        let effects = rename_main(main, &ident, &self.state.local_reserved());
        let mut lines = Vec::new();
        let mut assertion = 0;
        for effect in &effects {
            match effect {
                Effect::Print { expr, .. } => {
                    lines.push(format!("console.log({});", self.expr(expr)?));
                }
                Effect::Let { name, value, .. } => {
                    lines.push(format!("const {name} = {};", self.expr(value)?));
                }
                Effect::Assert { prop, .. } => {
                    assertion += 1;
                    self.helpers.insert(Helper::Assert);
                    let label = format!("assertion {assertion}");
                    lines.push(format!(
                        "ml_assert({}, {});",
                        self.prop(prop)?,
                        json_string(&label)
                    ));
                    self.state.assertion_theorem(&label, effect);
                }
            }
        }
        self.state.encode(
            "program-output",
            "main prints the lines the source program prints, in order, with console.log",
        );
        Ok(format!(
            "function main() {{\n{}\n}}",
            indent(&lines.join("\n"), 1)
        ))
    }
}

fn literal(ty: &Type, value: &LitValue) -> Result<String> {
    let text = value.text();
    match ty {
        Type::Nat | Type::Int | Type::Fixed { .. } => Ok(if text.starts_with('-') {
            format!("({text}n)")
        } else {
            format!("{text}n")
        }),
        Type::Bool => Ok(text),
        Type::String => Ok(json_string(&text)),
        other => Err(malformed(format!(
            "no JavaScript literal for {}",
            other.kind()
        ))),
    }
}
