//! Modules, data, functions, theorem runners and propositions.

use std::collections::BTreeSet;

use super::{
    ARRAY_PRELUDE, Binder, Decl, Emitted, Expr, External, MATH_PRELUDE, ModuleTree, NUMBER_PRELUDE,
    Node, PRELUDE, Prelude, Prop, Result, RustEmitter, TheoremCheck, Type, block,
    comparison_operator, format_escape, indent, own, rename_function, rename_theorem, rust_string,
    snake, tail_loop,
};

impl<'p> RustEmitter<'p> {
    pub(super) fn file(mut self) -> Result<Emitted> {
        let mut tree = ModuleTree::default();
        let program = self.program;
        for entry in &program.declarations {
            let mut node = &mut tree;
            for segment in entry.module_path() {
                let position = if let Some(position) =
                    node.modules.iter().position(|(name, _)| name == segment)
                {
                    position
                } else {
                    node.modules.push((segment.clone(), ModuleTree::default()));
                    node.modules.len() - 1
                };
                node = &mut node.modules[position].1;
            }
            node.items.push(entry);
        }
        let mut body: Vec<String> = program
            .imports
            .iter()
            .map(|import| self.use_declaration(import))
            .collect();
        body.extend(self.module_body(&tree, &[])?);
        let main = match &program.main {
            Some(main) => Some(self.main(main)?),
            None => None,
        };
        let runner = self.theorem_runner();
        let entry = self.entry(main.is_some());
        let mut lines = vec![
            format!(
                "// Translated from {} by meta-language: portable core, Rust target.",
                program.source_language.as_str()
            ),
            "#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]"
                .to_owned(),
            String::new(),
        ];
        let preludes: Vec<String> = self
            .preludes
            .iter()
            .map(|prelude| {
                match prelude {
                    Prelude::Big => PRELUDE,
                    Prelude::Number => NUMBER_PRELUDE,
                    Prelude::Math => MATH_PRELUDE,
                    Prelude::Array => ARRAY_PRELUDE,
                }
                .to_owned()
            })
            .collect();
        for prelude in &preludes {
            lines.push(prelude.clone());
            lines.push(String::new());
        }
        let has_main = main.is_some();
        let definitions = body.clone();
        for block in body.into_iter().chain(main).chain(runner).chain([entry]) {
            lines.push(block);
            lines.push(String::new());
        }
        let mut emitted = self
            .state
            .finish(lines.join("\n"), has_main.then(|| "main".to_owned()));
        emitted.preludes = preludes;
        emitted.definitions = definitions;
        Ok(emitted)
    }

    pub(super) fn module_body(
        &mut self,
        node: &ModuleTree<'p>,
        path: &[String],
    ) -> Result<Vec<String>> {
        let mut blocks = Vec::new();
        for entry in &node.items {
            blocks.push(self.declaration(entry)?);
        }
        for (segment, child) in &node.modules {
            let mut inner_path = path.to_vec();
            inner_path.push(segment.clone());
            let inner = self.module_body(child, &inner_path)?;
            let name = self.state.module_name(&inner_path).unwrap_or_default();
            blocks.push(format!(
                "pub mod {name} {{\n{}\n}}",
                indent(&inner.join("\n\n"), 1)
            ));
        }
        Ok(blocks)
    }

    pub(super) fn declaration(&mut self, entry: &'p Decl) -> Result<String> {
        match entry {
            Decl::Data(_) => Ok(self.data(entry)),
            Decl::Fn(_) => self.function(entry),
            Decl::Theorem(_) => self.theorem(entry),
        }
    }

    pub(super) fn ty(&mut self, ty: &Type) -> String {
        match ty {
            Type::Nat | Type::Int => {
                self.preludes.insert(Prelude::Big);
                self.state.encode(
                    "unbounded-integers",
                    "naturals and integers are ml::Big, an arbitrary-precision integer defined in the translated program; naturals stay non-negative because natural subtraction truncates and conversions to naturals are checked",
                );
                "crate::ml::Big".to_owned()
            }
            Type::Fixed { .. } => ty.key(),
            Type::Float => {
                self.floats();
                "f64".to_owned()
            }
            Type::Bool => "bool".to_owned(),
            Type::String => "String".to_owned(),
            Type::Unit => "()".to_owned(),
            Type::Data { name } => format!("crate::{}", self.state.reference(name, "::")),
            Type::Array { element } => {
                self.state.encode(
                    "arrays",
                    "a JavaScript array, which the portable core never mutates, is a Rust Vec; a read outside it panics",
                );
                format!("Vec<{}>", self.ty(element))
            }
            other => unreachable!("no Rust type for {}", other.kind()),
        }
    }

    pub(super) fn field_type(&mut self, ty: &Type) -> String {
        if matches!(ty, Type::Data { .. }) {
            format!("Box<{}>", self.ty(ty))
        } else {
            self.ty(ty)
        }
    }

    pub(super) fn data(&mut self, entry: &'p Decl) -> String {
        let Decl::Data(data) = entry else {
            unreachable!("a data declaration")
        };
        let name = self.state.local_name(&data.full_name).to_owned();
        self.state.map(entry, &name);
        self.state.encode(
            "data",
            "a data type is an enum with one tuple variant per constructor; data-typed fields are boxed, and values are compared structurally",
        );
        // f64 is not `Eq`, so a data type that holds a Number, directly or through another data type, derives `PartialEq` alone.
        let derives = if self.holds_float(&data.full_name, &mut BTreeSet::new()) {
            "Clone, Debug, PartialEq"
        } else {
            "Clone, Debug, PartialEq, Eq"
        };
        let mut lines = vec![
            format!("#[derive({derives})]"),
            format!("pub enum {name} {{"),
        ];
        for ctor in &data.ctors {
            let local = self
                .state
                .ctor_local(&data.full_name, &ctor.name)
                .to_owned();
            if ctor.fields.is_empty() {
                lines.push(format!("    {local},"));
                continue;
            }
            let fields: Vec<String> = ctor
                .fields
                .iter()
                .map(|field| self.field_type(&field.ty))
                .collect();
            lines.push(format!("    {local}({}),", fields.join(", ")));
        }
        lines.push("}".to_owned());
        lines.join("\n")
    }

    pub(super) fn function(&mut self, entry: &'p Decl) -> Result<String> {
        let Decl::Fn(function) = entry else {
            unreachable!("a function declaration")
        };
        let (params, body) = rename_function(function, &snake, &self.state.local_reserved());
        let name = self.state.local_name(&function.full_name).to_owned();
        self.state.map(entry, &name);
        if tail_loop(function) {
            // A lifted loop runs as a loop: each iteration assigns the parameters their next values.
            let names = params.iter().map(|param| param.name.clone()).collect();
            self.loop_params = Some((function.full_name.clone(), names));
            let text = self.expr(&body);
            self.loop_params = None;
            let text = text?;
            let binders: Vec<String> = params
                .iter()
                .map(|param| format!("mut {}: {}", param.name, self.ty(&param.ty)))
                .collect();
            let ret = self.ty(&function.ret);
            let body = format!("loop {{\n{}\n}}", indent(&format!("return {text};"), 1));
            return Ok(format!(
                "pub fn {name}({}) -> {ret} {{\n{}\n}}",
                binders.join(", "),
                indent(&body, 1)
            ));
        }
        let binders: Vec<String> = params
            .iter()
            .map(|param| format!("{}: {}", param.name, self.ty(&param.ty)))
            .collect();
        let ret = self.ty(&function.ret);
        let body = self.expr(&body)?;
        Ok(format!(
            "pub fn {name}({}) -> {ret} {{\n{}\n}}",
            binders.join(", "),
            indent(&body, 1)
        ))
    }

    fn holds_float(&self, name: &str, seen: &mut BTreeSet<String>) -> bool {
        if !seen.insert(name.to_owned()) {
            return false;
        }
        self.program.data(name).ctors.iter().any(|ctor| {
            ctor.fields
                .iter()
                .any(|field| self.type_holds_float(&field.ty, seen))
        })
    }

    fn type_holds_float(&self, ty: &Type, seen: &mut BTreeSet<String>) -> bool {
        match ty {
            Type::Float => true,
            Type::Array { element } => self.type_holds_float(element, seen),
            Type::Data { name } => self.holds_float(name, seen),
            _ => false,
        }
    }

    pub(super) fn theorem(&mut self, entry: &'p Decl) -> Result<String> {
        let Decl::Theorem(theorem) = entry else {
            unreachable!("a theorem declaration")
        };
        let (binders, prop, _) = rename_theorem(theorem, &snake, &self.state.local_reserved());
        let name = self.state.local_name(&theorem.full_name).to_owned();
        self.state.map(entry, &name);
        self.state
            .theorem(&theorem.full_name, &name, binders.is_empty(), true);
        self.theorem_checks.push(TheoremCheck {
            reference: format!("crate::{}", self.state.reference(&theorem.full_name, "::")),
            binders: binders.clone(),
            source: theorem.full_name.clone(),
        });
        let params: Vec<String> = binders
            .iter()
            .map(|binder| format!("{}: {}", binder.name, self.ty(&binder.ty)))
            .collect();
        let prop = self.prop(&prop)?;
        Ok(format!(
            "/// Executable form of theorem {}.\npub fn {name}({}) -> bool {{\n{}\n}}",
            theorem.full_name,
            params.join(", "),
            indent(&prop, 1)
        ))
    }

    /// The property checked on every input of the binders' bounded domains.
    pub(super) fn all_of(&mut self, binders: &[Binder], body: String) -> String {
        binders.iter().rev().fold(body, |inner, binder| {
            format!(
                "{}.into_iter().all(|{}| {inner})",
                self.domain(&binder.ty, 3),
                binder.name
            )
        })
    }

    pub(super) fn theorem_runner(&mut self) -> Option<String> {
        if self.theorem_checks.is_empty() {
            return None;
        }
        self.state.encode(
            "theorem-properties",
            "each theorem is an executable property; --ml-check-theorems evaluates it on every input of a bounded domain, and its proof remains checked by the source kernel",
        );
        let checks: Vec<String> = self
            .theorem_checks
            .clone()
            .iter()
            .map(|check| {
                let args: Vec<String> = check
                    .binders
                    .iter()
                    .map(|binder| own(&binder.name, &binder.ty))
                    .collect();
                let call = format!("{}({})", check.reference, args.join(", "));
                let holds = self.all_of(&check.binders, call);
                let source = format_escape(&check.source);
                [
                    format!("    if !({holds}) {{"),
                    format!("        panic!(\"theorem {source} fails on a bounded input\");"),
                    "    }".to_owned(),
                    format!("    println!(\"theorem {source}: holds on the bounded domain\");"),
                ]
                .join("\n")
            })
            .collect();
        Some(format!(
            "fn ml_check_theorems() {{\n{}\n}}",
            checks.join("\n")
        ))
    }

    pub(super) fn entry(&self, has_main: bool) -> String {
        let run = match (self.theorem_checks.is_empty(), has_main) {
            (false, true) => "if check { ml_check_theorems() } else { ml_main() }",
            (false, false) => "if check { ml_check_theorems() }",
            (true, true) => "ml_main()",
            (true, false) => "",
        };
        [
            "// Deep recursion in the source is not bounded by a small native stack.",
            "fn main() {",
            "    let check = std::env::args().any(|argument| argument == \"--ml-check-theorems\");",
            "    let worker = std::thread::Builder::new()",
            "        .stack_size(1 << 28)",
            &format!("        .spawn(move || {{ {run} }})"),
            "        .expect(\"spawn the program thread\");",
            "    if worker.join().is_err() {",
            "        std::process::exit(101);",
            "    }",
            "}",
        ]
        .join("\n")
    }

    /// Values of a type up to a constructor depth, as a Rust `Vec`.
    pub(super) fn domain(&mut self, ty: &Type, depth: i32) -> String {
        match ty {
            Type::Nat => {
                self.preludes.insert(Prelude::Big);
                if depth >= 3 {
                    "crate::ml::range(0, 6)"
                } else {
                    "crate::ml::range(0, 2)"
                }
                .to_owned()
            }
            Type::Int => {
                self.preludes.insert(Prelude::Big);
                if depth >= 3 {
                    "crate::ml::range(-4, 4)"
                } else {
                    "crate::ml::range(-1, 1)"
                }
                .to_owned()
            }
            Type::Fixed { signed, .. } => {
                let key = ty.key();
                let values = if *signed { [-1, 0, 1] } else { [0, 1, 2] };
                let values: Vec<String> =
                    values.iter().map(|value| format!("{value}{key}")).collect();
                format!("vec![{}]", values.join(", "))
            }
            Type::Bool => "vec![false, true]".to_owned(),
            Type::String => {
                "vec![String::new(), String::from(\"a\"), String::from(\"ab\")]".to_owned()
            }
            Type::Unit => "vec![()]".to_owned(),
            Type::Data { name } => {
                let program = self.program;
                let entry = program.data(name);
                let mut lines = vec![format!(
                    "let mut values: Vec<{}> = Vec::new();",
                    self.ty(ty)
                )];
                for ctor in &entry.ctors {
                    let recursive = ctor
                        .fields
                        .iter()
                        .any(|field| matches!(field.ty, Type::Data { .. }));
                    if recursive && depth <= 1 {
                        continue;
                    }
                    let names: Vec<String> = (0..ctor.fields.len())
                        .map(|index| format!("f{index}"))
                        .collect();
                    let args: Vec<String> = ctor
                        .fields
                        .iter()
                        .zip(&names)
                        .map(|(field, name)| {
                            if matches!(field.ty, Type::Data { .. }) {
                                format!("Box::new({name}.clone())")
                            } else {
                                format!("{name}.clone()")
                            }
                        })
                        .collect();
                    let head = format!(
                        "crate::{}",
                        self.state.ctor_ref(&entry.full_name, &ctor.name, "::")
                    );
                    let mut statement = if args.is_empty() {
                        format!("values.push({head});")
                    } else {
                        format!("values.push({head}({}));", args.join(", "))
                    };
                    for (field, name) in ctor.fields.iter().zip(&names).rev() {
                        let inner_depth = if matches!(field.ty, Type::Data { .. }) {
                            depth - 1
                        } else {
                            1
                        };
                        statement = format!(
                            "for {name} in {} {{\n{}\n}}",
                            self.domain(&field.ty, inner_depth),
                            indent(&statement, 1)
                        );
                    }
                    lines.push(statement);
                }
                lines.push("values".to_owned());
                block(&lines.join("\n"))
            }
            other => unreachable!("no Rust domain for {}", other.kind()),
        }
    }

    pub(super) fn prop(&mut self, prop: &Prop) -> Result<String> {
        Ok(match prop {
            Prop::Forall { binders, body } => {
                let body = self.prop(body)?;
                self.all_of(binders, body)
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
            other => {
                let (op, comparison) = other.comparison().expect("a comparison");
                if comparison.same_value {
                    self.preludes.insert(Prelude::Number);
                    let same = format!(
                        "crate::ml_number::same_value({}, {})",
                        self.expr(&comparison.left)?,
                        self.expr(&comparison.right)?
                    );
                    return Ok(if matches!(other, Prop::Eq(_)) {
                        same
                    } else {
                        format!("!{same}")
                    });
                }
                // Comparisons take their operands by reference, so they read them in place.
                format!(
                    "({} {} {})",
                    self.compared(&comparison.left)?,
                    comparison_operator(op),
                    self.compared(&comparison.right)?
                )
            }
        })
    }

    /// An expression whose value is only read: variables are borrowed, not cloned.
    pub(super) fn borrow(&mut self, expr: &Expr) -> Result<String> {
        if expr.ty.is_float() {
            return self.expr(expr);
        }
        match expr.var_name() {
            Some(name) => Ok(format!("&{name}")),
            None => Ok(format!("&{}", self.expr(expr)?)),
        }
    }

    /// A comparison operand: read in place, a string literal as a `&str`.
    pub(super) fn compared(&mut self, expr: &Expr) -> Result<String> {
        if let (Node::Lit { value }, Type::String) = (&expr.node, &expr.ty) {
            return Ok(rust_string(&value.text()));
        }
        if let (Node::Call { func, .. }, Type::String) = (&expr.node, &expr.ty)
            && matches!(
                self.program.external(func),
                Some(External::Constant { literal: true, .. })
            )
        {
            return Ok(func.clone());
        }
        self.receiver(expr)
    }

    /// A method receiver: `&self` methods borrow a variable in place.
    pub(super) fn receiver(&mut self, expr: &Expr) -> Result<String> {
        match expr.var_name() {
            Some(name) => Ok(name.to_owned()),
            None => Ok(format!("({})", self.expr(expr)?)),
        }
    }
}
