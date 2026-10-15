//! The JavaScript emitter: writes a checked program as a strict ES module
//! with its translation contract.
//!
//! Every portable integer is a `BigInt`, so naturals and integers stay unbounded
//! and machine integers are range-checked exactly as Rust checks them; a Number
//! stays a Number. Data values are plain objects whose `$` property names the
//! constructor. Modules become object literals referenced by qualified names.
//! Theorems cannot be proved in JavaScript: each becomes an executable
//! property, checked over a bounded domain by `--ml-check-theorems`, while the
//! proof obligation stays discharged by the source language's kernel.
//!
//! Mirrors `js/src/translation/emit-javascript.js`.

use std::collections::BTreeSet;

use super::Language;
use super::aborts::{cast_message, overflow_message, zero_divisor_message};
use super::diagnostics::{Result, TranslationError, type_error, unsupported};
use super::emit_common::{EmitOptions, EmitState, Emitted};
use super::ir::{
    Binder, ByZero, Case, Ctor, Decl, Effect, Expr, FnDecl, LitValue, Main, Node, Param, Pattern,
    Program, Semantics, rename_function, rename_main, tail_loop,
};
use super::lexer::json_string;
use super::surface::{BinaryOp, Flavor, Rounding, UnaryOp};
use super::types::{Type, fixed_bounds};

mod arrays;
mod helpers;
mod math;
mod names;
mod theorems;
use self::helpers::Helper;
use self::names::{KEYWORDS, field_key, ident};

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

fn ctor_object(ctor: &Ctor, args: &[String], language: Language, explicit_frozen: bool) -> String {
    let fields: Vec<String> = ctor
        .fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
            let arg = args.get(index).map_or("undefined", String::as_str);
            format!("[{}]: {arg}", field_key(&field.name))
        })
        .collect();
    let fields = if fields.is_empty() {
        String::new()
    } else {
        format!(", {}", fields.join(", "))
    };
    let object = format!("{{ $: {}{fields} }}", json_string(&ctor.name));
    if super::primitive_value_rules::require_data_object_freeze(language.as_str(), explicit_frozen)
    {
        format!("Object.freeze({object})")
    } else {
        format!("({object})")
    }
}

/// Emits a checked program as JavaScript.
///
/// # Errors
/// On constructs the target cannot express faithfully.
pub fn emit_javascript(program: &Program) -> Result<Emitted> {
    emit_javascript_with_options(program, JavaScriptEmissionOptions::default())
}

/// Explicit checked local-scope models for JavaScript emission.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct JavaScriptEmissionOptions<'a> {
    /// Canonical aliases for owned local binding records.
    pub scoped_binding_model: Option<&'a str>,
}

/// Emit using a selected scoped binding interpretation.
///
/// # Errors
/// Refuses unknown models and unsupported or invalid ownership scopes.
pub fn emit_javascript_with_options(
    program: &Program,
    options: JavaScriptEmissionOptions<'_>,
) -> Result<Emitted> {
    let runtime = super::runtime_shell::validate_program(program)?;
    if runtime.is_some() && options.scoped_binding_model.is_some() {
        return Err(crate::translation::diagnostics::unsupported(
            "closed Rust runtime shell",
            "scoped target bindings require an independent runtime composition proof",
            None,
        ));
    }
    if let Some(model) = options.scoped_binding_model {
        super::scoped_binding_emission::validate_program(program, model)?;
    }
    super::owned_local_state::validate_owned_local_state(program)?;
    super::source_const_functions::validate_const_functions(program)?;
    let mut reserved = KEYWORDS.to_vec();
    if program.declarations.iter().any(|declaration| matches!(declaration, Decl::Fn(function) if function.source_local_state.is_some())) {
        reserved.push("ml_ownedTextInput");
    }
    let state = EmitState::new(
        program,
        Language::JavaScript,
        ident,
        &reserved,
        EmitOptions {
            modules_share_term_space: true,
            type_space: true,
            ..EmitOptions::default()
        },
    );
    let mut emitted = JavaScriptEmitter {
        program,
        state,
        helpers: BTreeSet::new(),
        temporaries: 0,
        theorem_checks: Vec::new(),
        uses_float: false,
        loop_params: None,
        scoped_bindings: options.scoped_binding_model.is_some(),
    }
    .file()?;
    if let Some(runtime) = runtime {
        super::runtime_shell::record_assumptions(&mut emitted, &runtime)?;
    }
    Ok(emitted)
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
    uses_float: bool,
    /// The lifted loop being emitted as a loop, and its parameters, which
    /// each call to it assigns before the next iteration.
    loop_params: Option<(String, Vec<String>)>,
    /// Selection is validated before any existing lexical allocator runs.
    scoped_bindings: bool,
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
        let preludes: Vec<String> = self
            .helpers
            .iter()
            .map(|helper| helper.text().to_owned())
            .collect();
        for prelude in &preludes {
            lines.push(prelude.clone());
            lines.push(String::new());
        }
        let definitions = blocks.clone();
        for block in blocks.into_iter().chain(entry) {
            lines.push(block);
            lines.push(String::new());
        }
        self.state.encode("numbers", "every natural, integer and machine integer is a BigInt; machine-integer results are range-checked and throw RangeError where Rust would panic");
        if self.uses_float {
            self.state.encode("floats", "a Number is a JavaScript Number, an IEEE-754 double, with its own arithmetic, comparisons and String conversion");
        }
        let mut emitted = self
            .state
            .finish(lines.join("\n"), main.map(|_| "main".to_owned()));
        emitted.preludes = preludes;
        emitted.definitions = definitions;
        Ok(emitted)
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
                self.state.encode("data", if self.program.source_language == Language::JavaScript {
                    "a JavaScript data value is an object whose $ property names its constructor and whose other properties retain their own field names; explicit source freezing is preserved"
                } else {
                    "a data value is a frozen object whose $ property names its constructor and whose other properties are its fields"
                });
                Ok(None)
            }
            Decl::Fn(function) => self.function(entry, function).map(Some),
            Decl::Theorem(theorem) => self.theorem(entry, theorem).map(Some),
        }
    }

    fn function(&mut self, entry: &Decl, function: &FnDecl) -> Result<String> {
        let preserve = if self.scoped_bindings {
            super::scoped_binding_emission::function_role(self.program, function)?
        } else {
            false
        };
        let local = |name: &str| {
            if self.scoped_bindings {
                ident(&super::scoped_binding_emission_rules::render_java_script_scoped_binding_identifier(name, preserve))
            } else {
                ident(name)
            }
        };
        // The revalidated closed loop has no global declaration reads: its
        // only call is its own tail continuation, lowered to writes/continue.
        let reserved = if self.scoped_bindings
            && !super::scoped_binding_emission_rules::reserve_java_script_scoped_binding_declaration_names(preserve)
        {
            KEYWORDS.iter().map(|name| (*name).to_owned()).collect()
        } else {
            self.state.local_reserved()
        };
        let (params, body) = rename_function(function, &local, &reserved);
        if params.iter().any(|param| param.ty.is_float()) || function.ret.is_float() {
            self.uses_float = true;
        }
        let name = self.state.local_name(&function.full_name).to_owned();
        self.state.map(entry, &name);
        let mut lines = Vec::new();
        for param in &params {
            if function.source_local_state.is_some() {
                lines.push(self.owned_local_parameter_guard(param));
            }
            lines.extend(self.parameter_guard(param));
        }
        if function.source_const_function.is_some() {
            for param in &params {
                let kind = if param.ty.kind() == "bool" {
                    "boolean"
                } else {
                    "number"
                };
                let name = &param.name;
                lines.push(format!(
                    "if (typeof {name} !== {kind:?}) throw new RangeError({:?});",
                    format!("const predicate argument {name} requires {kind}")
                ));
            }
            self.state.encode("rust-const-predicate-inputs", "the selected Rust const predicate model admits only primitive Boolean and binary64 input values; const evaluation requests remain outside this source interpretation");
        }
        if function.source_local_state.is_some() {
            self.state.encode("owned-text-input", "the selected Rust local value model admits primitive Unicode scalar text, Boolean and signed i32 inputs; JavaScript rejects other input values before evaluating the body");
            let assumption = &super::owned_local_state::policy()["targetInputAssumption"];
            self.state.assume(
                super::emit_common::AssumptionText {
                    id: assumption["id"].as_str().unwrap(),
                    statement: assumption["statement"].as_str().unwrap(),
                },
                None,
            );
        }
        if tail_loop(function) {
            // A lifted loop runs as a loop: each iteration assigns the parameters their next values.
            let names = params.iter().map(|param| param.name.clone()).collect();
            self.loop_params = Some((function.full_name.clone(), names));
            let statements = self.statements(&body);
            self.loop_params = None;
            lines.push("for (;;) {".to_owned());
            lines.push(indent(&statements?.join("\n"), 1));
            lines.push("}".to_owned());
        } else {
            lines.extend(self.statements(&body)?);
        }
        let names: Vec<&str> = params.iter().map(|param| param.name.as_str()).collect();
        Ok(format!(
            "{name}({}) {{\n{}\n}}",
            names.join(", "),
            indent(&lines.join("\n"), 1)
        ))
    }

    fn owned_local_parameter_guard(&mut self, param: &Param) -> String {
        if param.ty == super::types::STRING {
            self.helpers.insert(Helper::OwnedTextInput);
            return format!(
                "if (!ml_ownedTextInput({})) throw new RangeError({});",
                param.name,
                json_string(&format!(
                    "String argument {} requires Unicode scalar text",
                    param.name
                ))
            );
        }
        let kind = if param.ty == super::types::BOOL {
            "boolean"
        } else {
            "bigint"
        };
        format!(
            "if (typeof {} !== {}) throw new RangeError({});",
            param.name,
            json_string(kind),
            json_string(&format!(
                "{} argument {} requires {kind}",
                param.ty.key(),
                param.name
            ))
        )
    }

    /// Machine-integer parameters are range-checked, as the Rust type guarantees.
    fn parameter_guard(&mut self, param: &Param) -> Option<String> {
        let Type::Fixed { bits, signed } = param.ty else {
            return None;
        };
        self.helpers.insert(Helper::Fixed);
        let (min, max) = fixed_bounds(bits, signed);
        Some(format!(
            "ml_fixed({name}, {min}n, {max}n, {message});",
            name = param.name,
            message = json_string(&format!(
                "{} argument {} out of range",
                param.ty.key(),
                param.name
            ))
        ))
    }

    /// A function body as statements that return its value.
    fn statements(&mut self, e: &Expr) -> Result<Vec<String>> {
        match &e.node {
            Node::Let { name, value, body } => {
                let mut lines = vec![format!("const {name} = {};", self.expr(value)?)];
                lines.extend(self.statements(body)?);
                Ok(lines)
            }
            Node::Print { text, body } => {
                let mut lines = vec![format!("console.log({});", self.expr(text)?)];
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
            Node::Call { func, args }
                if self
                    .loop_params
                    .as_ref()
                    .is_some_and(|(name, _)| name == func) =>
            {
                let mut texts = Vec::with_capacity(args.len());
                for arg in args {
                    texts.push(self.expr(arg)?);
                }
                let names = self
                    .loop_params
                    .as_ref()
                    .map(|(_, names)| names.as_slice())
                    .unwrap_or_default();
                let mut lines = match names {
                    [] => Vec::new(),
                    [name] => vec![format!("{name} = {};", texts[0])],
                    _ => vec![format!("[{}] = [{}];", names.join(", "), texts.join(", "))],
                };
                lines.push("continue;".to_owned());
                Ok(lines)
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
                            "const {bind} = {subject}[{}];",
                            field_key(&ctor.fields[index].name)
                        )
                    })
                })
                .collect();
            lines.push(format!("  case {}: {{", json_string(&ctor.name)));
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
        if e.ty.is_float() {
            self.uses_float = true;
        }
        match &e.node {
            Node::Lit { value } => literal(&e.ty, value),
            Node::None { absence } => Ok(if absence == "null" {
                "null"
            } else {
                "undefined"
            }
            .to_owned()),
            Node::Unit => Ok(super::product_emission_rules::render_java_script_unit(
                self.program.source_language.as_str(),
            )),
            Node::Tuple {
                items,
                frozen,
                product_model,
            } => {
                if product_model.as_ref().is_some_and(|model| {
                    !super::typed_program_link_rules::accept_program_tuple_product_model(
                        model, *frozen,
                    )
                }) {
                    return Err(unsupported(
                        "immutable tuple product model",
                        "the declared immutable value product requires its retained frozen facet",
                        e.span,
                    ));
                }
                let texts = self.exprs(items)?;
                Ok(
                    crate::translation::product_emission_rules::render_tuple_value(
                        "JavaScript",
                        &texts,
                        *frozen,
                    ),
                )
            }
            Node::TupleIndex { object, index } => {
                let text = self.expr(object)?;
                Ok(
                    crate::translation::product_emission_rules::render_tuple_projection(
                        "JavaScript",
                        &text,
                        &index.to_string(),
                    ),
                )
            }
            Node::Var { name } => Ok(name.clone()),
            Node::Print { text, body } => Ok(format!(
                "(console.log({}), {})",
                self.expr(text)?,
                self.expr(body)?
            )),
            Node::OutNil | Node::OutCons { .. } => {
                unreachable!("only the Lean and Rocq emitters thread output")
            }
            Node::Call { func, args } => {
                let target = self.state.reference(func, ".");
                let args = self.exprs(args)?;
                Ok(format!("{target}({})", args.join(", ")))
            }
            Node::Ctor {
                data,
                ctor,
                args,
                frozen,
            } => {
                let entry = self.program.data(data);
                let ctor = entry
                    .ctors
                    .iter()
                    .find(|candidate| candidate.name == *ctor)
                    .unwrap_or_else(|| panic!("{data} has no constructor {ctor}"));
                let args = self.exprs(args)?;
                Ok(ctor_object(
                    ctor,
                    &args,
                    self.program.source_language,
                    *frozen,
                ))
            }
            Node::Unary { op, arg, .. } => {
                let arg = self.expr(arg)?;
                Ok(match op {
                    UnaryOp::Not => format!("!{arg}"),
                    UnaryOp::Neg => self.checked(negate(&arg), &e.ty, "neg"),
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
            Node::StringMap { op, string } => {
                let value = self.expr(string)?;
                let object = if matches!(string.node, Node::Var { .. } | Node::Lit { .. }) {
                    value
                } else {
                    format!("({value})")
                };
                Ok(
                    crate::translation::frontend_rules::render_string_map_expression(
                        "JavaScript",
                        op,
                        &object,
                    ),
                )
            }
            Node::StringTest { op, string, search } => {
                let value = self.expr(string)?;
                let object = if matches!(string.node, Node::Var { .. } | Node::Lit { .. }) {
                    value
                } else {
                    format!("({value})")
                };
                let search = self.expr(search)?;
                Ok(
                    crate::translation::frontend_rules::render_string_test_expression(
                        "JavaScript",
                        op,
                        &object,
                        &search,
                    ),
                )
            }
            Node::ToString { arg, console } => {
                if *console && arg.ty.is_float() {
                    self.helpers.insert(Helper::ShowNumber);
                    Ok(format!("ml_showNumber({})", self.expr(arg)?))
                } else if matches!(arg.ty, Type::String) {
                    self.expr(arg)
                } else {
                    self.text_of(arg)
                }
            }
            Node::Cast {
                arg,
                flavor,
                message,
                ..
            } => self.cast(arg, *flavor, message.as_deref()),
            Node::Abort { message } => {
                self.helpers.insert(Helper::Abort);
                Ok(format!("ml_abort({})", json_string(message)))
            }
            Node::Array { .. } | Node::Append { .. } | Node::Index { .. } | Node::Length { .. } => {
                self.array_expr(e)
            }
            Node::Math { .. } => self.math(e),
        }
    }

    pub(super) fn exprs(&mut self, list: &[Expr]) -> Result<Vec<String>> {
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

    /// A machine-integer result out of range panics as Rust does.
    fn checked(&mut self, text: String, ty: &Type, op: &str) -> String {
        let Type::Fixed { bits, signed } = *ty else {
            return text;
        };
        self.helpers.insert(Helper::Fixed);
        let (min, max) = fixed_bounds(bits, signed);
        format!(
            "ml_fixed({text}, {min}n, {max}n, {})",
            json_string(overflow_message(op))
        )
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
            BinaryOp::Add => self.checked(comparison("+"), &e.ty, "add"),
            BinaryOp::Mul => self.checked(comparison("*"), &e.ty, "mul"),
            BinaryOp::Sub => {
                if *semantics == Some(Semantics::Truncated) {
                    self.helpers.insert(Helper::NatSub);
                    format!("ml_natSub({left}, {right})")
                } else {
                    self.checked(comparison("-"), &e.ty, "sub")
                }
            }
            BinaryOp::Div | BinaryOp::Rem if *semantics == Some(Semantics::Ieee) => {
                comparison(if *op == BinaryOp::Div { "/" } else { "%" })
            }
            BinaryOp::Div | BinaryOp::Rem => {
                self.helpers.insert(Helper::Divide);
                let rounding = match rounding {
                    Some(Rounding::Trunc) => "trunc",
                    Some(Rounding::Euclid) => "euclid",
                    Some(Rounding::Floor) => "floor",
                    None => "undefined",
                };
                let zero = if *by_zero == Some(ByZero::Abort) {
                    json_string(zero_divisor_message(op.name(), &e.ty))
                } else {
                    "null".to_owned()
                };
                let remainder = *op == BinaryOp::Rem;
                let Type::Fixed { bits, signed } = e.ty else {
                    return Ok(format!(
                        "ml_divide({left}, {right}, '{rounding}', {zero}, {remainder})"
                    ));
                };
                let (min, max) = fixed_bounds(bits, signed);
                format!(
                    "ml_divide({left}, {right}, '{rounding}', {zero}, {remainder}, [{min}n, {max}n, {}])",
                    json_string(overflow_message(op.name()))
                )
            }
            BinaryOp::Plus => return Err(malformed("no JavaScript operator plus".to_owned())),
        })
    }

    fn cast(&mut self, arg: &Expr, flavor: Flavor, message: Option<&str>) -> Result<String> {
        let arg = self.expr(arg)?;
        Ok(match flavor {
            Flavor::Exact => arg,
            Flavor::Clamp => format!("((value) => (value < 0n ? 0n : value))({arg})"),
            Flavor::Checked => {
                self.helpers.insert(Helper::ToNatChecked);
                format!(
                    "ml_toNatChecked({arg}, {})",
                    json_string(cast_message(message))
                )
            }
        })
    }

    fn main(&mut self, main: &Main) -> Result<String> {
        if self.scoped_bindings {
            super::scoped_binding_emission::main_role(self.program, main)?;
        }
        let local = |name: &str| {
            if self.scoped_bindings {
                ident(&super::scoped_binding_emission_rules::render_java_script_scoped_binding_identifier(name, false))
            } else {
                ident(name)
            }
        };
        let effects = rename_main(main, &local, &self.state.local_reserved());
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
                Effect::Output { .. } | Effect::Unwrap { .. } => {
                    unreachable!("only the Lean and Rocq emitters thread output")
                }
            }
        }
        if main.sequential_async {
            self.state.encode(
                "sequential-async",
                "an async function is the function its body computes and await is its call: every call of one is awaited where it is made, so nothing runs concurrently and the output is the same, in the same order",
            );
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

/// `-x`, parenthesised so that `-(-x)` does not read as a decrement.
fn negate(text: &str) -> String {
    if text.starts_with('-') {
        format!("-({text})")
    } else {
        format!("-{text}")
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
        Type::Float => Ok(if text.starts_with('-') {
            format!("({text})")
        } else {
            text
        }),
        Type::Bool => Ok(text),
        Type::String => Ok(json_string(&text)),
        other => Err(malformed(format!(
            "no JavaScript literal for {}",
            other.kind()
        ))),
    }
}
