//! Vernacular sentences: inductives, binders, types, definitions, fixpoints and `main`.

use super::{
    Annotation, Definition, EVAL_STRATEGIES, Language, OUTSIDE_CORE_TYPES, Result, RocqParser,
    SBinder, SCtor, SData, SField, SFn, SItem, SMain, SModule, SParam, SProgram, THEOREM_KEYWORDS,
    Token, TokenKind, TranslationError, Type, UNIT, UNSUPPORTED_COMMANDS, builtin_type,
    check_portable, object_prototype_key, span, type_error, unsupported, walk_output,
};

impl RocqParser {
    pub(super) fn file(&mut self) -> Result<SProgram> {
        let mut main = None;
        let items = self.items(&mut main, &[])?;
        Ok(SProgram {
            language: Language::Rocq,
            items,
            main,
        })
    }

    pub(super) fn items(
        &mut self,
        main: &mut Option<SMain>,
        path: &[String],
    ) -> Result<Vec<SItem>> {
        let mut items = Vec::new();
        while !self.cursor.at_end() {
            let token = self.peek().clone();
            if self.is("End") {
                let Some(open) = path.last() else {
                    return Err(self.fail_here("unmatched End"));
                };
                self.cursor.advance();
                let name = self.identifier("End")?.value;
                if &name != open {
                    return Err(TranslationError::syntax(
                        format!("End {name} closes module {open}"),
                        Some(token.span()),
                    ));
                }
                self.end_sentence("End")?;
                return Ok(items);
            }
            if self.is("Module") {
                self.cursor.advance();
                if self.is("Type") || self.is("Import") || self.is("Export") {
                    return Err(unsupported(
                        &format!("Rocq Module {}", self.peek().value),
                        "module types and functors are outside the portable core",
                        Some(span(&token, self.peek())),
                    ));
                }
                let name = self.identifier("Module")?.value;
                if name.contains('.') {
                    return Err(unsupported(
                        "qualified module name",
                        "declare one module per level",
                        Some(span(&token, &token)),
                    ));
                }
                if !self.is(".") {
                    return Err(unsupported(
                        "module signature or functor",
                        "only plain modules are portable",
                        Some(span(&token, self.peek())),
                    ));
                }
                self.end_sentence("Module")?;
                let module_span = span(&token, self.peek());
                let mut inner = path.to_vec();
                inner.push(name.clone());
                let module_items = self.items(main, &inner)?;
                items.push(SItem::Module(SModule {
                    name,
                    items: module_items,
                    span: Some(module_span),
                }));
                continue;
            }
            if self.ignored_command()? {
                continue;
            }
            if self.is("Inductive") {
                items.push(SItem::Data(self.inductive()?));
                continue;
            }
            if self.is("Definition") || self.is("Fixpoint") || self.is("Function") {
                match self.definition(path)? {
                    Definition::Main(program, item_span) => {
                        if !path.is_empty() {
                            return Err(unsupported(
                                "main inside a module",
                                "main must be declared at the top level",
                                Some(item_span),
                            ));
                        }
                        if main.is_some() {
                            return Err(type_error("duplicate main", Some(item_span)));
                        }
                        *main = Some(program);
                    }
                    Definition::Fn(item) => items.push(SItem::Fn(*item)),
                }
                continue;
            }
            if token.kind == TokenKind::Identifier
                && THEOREM_KEYWORDS.contains(&token.value.as_str())
            {
                items.push(SItem::Theorem(self.theorem()?));
                continue;
            }
            if self.is("Eval") || self.is("Compute") {
                self.evaluation()?;
                continue;
            }
            if token.kind == TokenKind::Identifier
                && (UNSUPPORTED_COMMANDS.contains(&token.value.as_str())
                    || token.value.starts_with(|ch: char| ch.is_ascii_uppercase()))
            {
                return Err(unsupported(
                    &format!("Rocq {} command", token.value),
                    "outside the portable core",
                    Some(span(&token, &token)),
                ));
            }
            return Err(self.fail_here("expected a Rocq command"));
        }
        if let Some(open) = path.last() {
            return Err(TranslationError::syntax(
                format!("module {open} is not closed"),
                Some(self.peek().span()),
            ));
        }
        Ok(items)
    }

    /// Library loading and notation scopes do not change the program's meaning
    /// in the portable core: operators are resolved by their operand types.
    pub(super) fn ignored_command(&mut self) -> Result<bool> {
        let command = self.peek().value.clone();
        if self.is("From") {
            self.cursor.advance();
            self.identifier("From")?;
            self.expect("Require", "From")?;
            return self.skip_sentence(&command);
        }
        if self.is("Require")
            || (self.is("Import") && self.cursor.is_at("ListNotations", 1))
            || (self.is("Open") && self.cursor.is_at("Scope", 1))
            || (self.is("Local") && self.cursor.is_at("Open", 1) && self.cursor.is_at("Scope", 2))
        {
            return self.skip_sentence(&command);
        }
        Ok(false)
    }

    pub(super) fn skip_sentence(&mut self, command: &str) -> Result<bool> {
        while !self.cursor.at_end() && !self.is(".") {
            self.cursor.advance();
        }
        self.end_sentence(command)?;
        Ok(true)
    }

    /// `Eval <strategy> in main.` or `Compute main.` displays the program output.
    pub(super) fn evaluation(&mut self) -> Result<()> {
        let token = self.cursor.advance();
        if token.value == "Eval" {
            let strategy = self.identifier("Eval")?.value;
            if !EVAL_STRATEGIES.contains(&strategy.as_str()) {
                return Err(unsupported(
                    &format!("Eval {strategy}"),
                    "unknown reduction strategy",
                    Some(span(&token, self.peek())),
                ));
            }
            self.expect("in", "Eval")?;
        }
        if !self.is("main") {
            return Err(unsupported(
                "evaluation command",
                "only the evaluation of main is part of the program output",
                Some(span(&token, self.peek())),
            ));
        }
        self.cursor.advance();
        self.end_sentence(&token.value)
    }

    pub(super) fn inductive(&mut self) -> Result<SData> {
        let start = self.cursor.advance();
        let name = self.identifier("Inductive")?.value;
        if name.contains('.') {
            return Err(unsupported(
                "qualified inductive name",
                "declare inductives inside their module",
                Some(span(&start, &start)),
            ));
        }
        if self.is("(") || self.is("{") {
            return Err(unsupported(
                "parameterised inductive",
                "type parameters are outside the portable core",
                Some(span(&start, self.peek())),
            ));
        }
        if self.cursor.eat(":").is_some() {
            let sort = self.identifier("Inductive sort")?;
            if !["Type", "Set"].contains(&sort.value.as_str()) {
                return Err(unsupported(
                    &format!("inductive in {}", sort.value),
                    "only data types are portable",
                    Some(span(&start, &sort)),
                ));
            }
        }
        self.expect(":=", "Inductive")?;
        let mut ctors = Vec::new();
        self.cursor.eat("|");
        loop {
            let ctor_start = self.peek().clone();
            let ctor_name = self.identifier("constructor")?.value;
            let mut fields = Vec::new();
            while self.is("(") {
                fields.extend(self.binder_group()?.into_iter().map(|binder| SField {
                    name: Some(binder.name),
                    ty: binder.ty.unwrap_or(UNIT),
                    rocq_type: binder.rocq_type,
                    span: binder.span,
                }));
            }
            if self.cursor.eat(":").is_some() {
                let mut types = self.arrow_type()?;
                let result = types.pop();
                let constructs =
                    matches!(&result, Some(Type::Named { path, .. }) if path.join(".") == name);
                if !constructs {
                    return Err(unsupported(
                        "indexed constructor",
                        &format!("{ctor_name} must construct {name}"),
                        Some(span(&ctor_start, self.peek())),
                    ));
                }
                fields.extend(types.into_iter().map(|ty| SField {
                    name: None,
                    ty,
                    rocq_type: None,
                    span: None,
                }));
            }
            ctors.push(SCtor {
                name: ctor_name,
                fields,
            });
            if self.cursor.eat("|").is_none() {
                break;
            }
        }
        if self.is("with") {
            return Err(unsupported(
                "mutual inductive",
                "mutually inductive types are outside the portable core",
                Some(span(&start, self.peek())),
            ));
        }
        self.end_sentence("Inductive")?;
        Ok(SData {
            name,
            ctors,
            span: Some(span(&start, self.peek())),
            generated: false,
        })
    }

    pub(super) fn binder_group(&mut self) -> Result<Vec<SBinder>> {
        let open = self.expect("(", "binder")?;
        let mut names = Vec::new();
        while self.cursor.is_kind(TokenKind::Identifier) {
            names.push(self.cursor.advance().value);
        }
        if names.is_empty() {
            return Err(self.fail_here("expected binder names"));
        }
        self.expect(":", "binder")?;
        let rocq_type = self.peek().value.clone();
        let ty = self.parse_type()?;
        self.expect(")", "binder")?;
        let group_span = span(&open, self.peek());
        Ok(names
            .into_iter()
            .map(|name| SBinder {
                name,
                ty: Some(ty.clone()),
                rocq_type: Some(rocq_type.clone()),
                span: Some(group_span),
            })
            .collect())
    }

    pub(super) fn binders(&mut self) -> Result<Vec<SBinder>> {
        let mut result = Vec::new();
        while self.is("(") {
            result.extend(self.binder_group()?);
        }
        if self.is("{") && !self.cursor.is_at("struct", 1) && !self.cursor.is_at("measure", 1) {
            return Err(unsupported(
                "implicit binder",
                "implicit binders are outside the portable core",
                Some(span(self.peek(), self.peek())),
            ));
        }
        if self.is("`") {
            return Err(unsupported(
                "generalised binder",
                "outside the portable core",
                Some(span(self.peek(), self.peek())),
            ));
        }
        Ok(result)
    }

    pub(super) fn arrow_type(&mut self) -> Result<Vec<Type>> {
        let mut types = vec![self.parse_type()?];
        while self.cursor.eat("->").is_some() {
            types.push(self.parse_type()?);
        }
        Ok(types)
    }

    pub(super) fn product_type(&self, token: &Token) -> TranslationError {
        unsupported(
            "product type",
            "tuples are outside the portable core",
            Some(span(token, self.peek())),
        )
    }

    pub(super) fn parse_type(&mut self) -> Result<Type> {
        let token = self.peek().clone();
        if self.cursor.eat("(").is_some() {
            let mut types = self.arrow_type()?;
            if self.is("*") {
                return Err(self.product_type(&token));
            }
            self.expect(")", "type")?;
            if types.len() != 1 {
                return Err(unsupported(
                    "function type",
                    "higher-order values are outside the portable core",
                    Some(span(&token, &token)),
                ));
            }
            return Ok(types.remove(0));
        }
        let name = self.identifier("type")?.value;
        if let Some(builtin) = builtin_type(&name) {
            if self.is("*") {
                return Err(self.product_type(&token));
            }
            return Ok(builtin);
        }
        if OUTSIDE_CORE_TYPES.contains(&name.as_str()) {
            return Err(unsupported(
                &format!("Rocq type {name}"),
                "outside the portable core",
                Some(span(&token, &token)),
            ));
        }
        let next = self.peek();
        if next.kind == TokenKind::Identifier
            && !["with", "end", "in", "then", "else"].contains(&next.value.as_str())
        {
            return Err(unsupported(
                "type application",
                &format!("{name} {} is outside the portable core", next.value),
                Some(span(&token, next)),
            ));
        }
        if self.is("*") {
            return Err(self.product_type(&token));
        }
        Ok(Type::Named {
            path: name.split('.').map(str::to_owned).collect(),
            span: Some(span(&token, &token)),
        })
    }

    pub(super) fn definition(&mut self, path: &[String]) -> Result<Definition> {
        let start = self.cursor.advance();
        let keyword = start.value.clone();
        let name_token = self.identifier(&keyword)?;
        let name = name_token.value.clone();
        if name.contains('.') {
            return Err(unsupported(
                "qualified definition name",
                "declare definitions inside their module",
                Some(span(&name_token, &name_token)),
            ));
        }
        let params = self.binders()?;
        let annotation = self.recursion_annotation(&keyword, &params, &start)?;
        if !self.is(":") {
            return Err(unsupported(
                "definition without a result type",
                &format!("{name} needs an explicit result type"),
                Some(span(&start, self.peek())),
            ));
        }
        self.cursor.advance();
        if name == "main"
            && path.is_empty()
            && keyword == "Definition"
            && self.is("list")
            && self.cursor.is_at("string", 1)
        {
            return self.main(&start);
        }
        let ret = self.parse_type()?;
        self.expect(":=", &keyword)?;
        let body = self.expr()?;
        check_portable(&body)?;
        if self.is("with") {
            return Err(unsupported(
                "mutual fixpoint",
                "mutually recursive definitions are outside the portable core",
                Some(span(&start, self.peek())),
            ));
        }
        self.end_sentence(&keyword)?;
        if keyword == "Function" {
            self.skip_obligations(&start, annotation)?;
        }
        Ok(Definition::Fn(Box::new(SFn {
            name,
            params: params
                .into_iter()
                .map(|binder| SParam {
                    name: binder.name,
                    default_value: None,
                    ty: binder.ty,
                    span: binder.span,
                    guard: None,
                    rocq_type: binder.rocq_type,
                })
                .collect(),
            ret: Some(ret),
            body,
            span: Some(span(&start, self.peek())),
            generated: false,
        })))
    }

    /// `{struct x}` names the structural argument and `{measure N.to_nat x}`
    /// justifies a `Function`; the portable core finds its own decreasing
    /// argument and every target re-establishes termination.
    pub(super) fn recursion_annotation(
        &mut self,
        keyword: &str,
        params: &[SBinder],
        start: &Token,
    ) -> Result<Option<Annotation>> {
        if !self.is("{") {
            if keyword == "Function" {
                return Err(unsupported(
                    "Function without a measure",
                    "Function needs {measure …} or {struct …}",
                    Some(span(start, self.peek())),
                ));
            }
            return Ok(None);
        }
        let open = self.cursor.advance();
        let is_param = |name: &str| params.iter().any(|param| param.name == name);
        let kind = self.identifier("recursion annotation")?.value;
        if kind == "struct" {
            if keyword == "Definition" {
                return Err(Self::fail("struct annotation on a Definition", &open));
            }
            let arg = self.identifier("struct")?.value;
            if !is_param(&arg) {
                return Err(type_error(
                    format!("struct argument {arg} is not a parameter"),
                    Some(open.span()),
                ));
            }
            self.expect("}", "struct")?;
            return Ok(Some(Annotation::Struct));
        }
        if kind == "measure" && keyword == "Function" {
            let measure_fn = self.identifier("measure")?.value;
            // `measureFn in {}` also holds for the names on `Object.prototype`.
            if !["N.to_nat", "Z.to_nat"].contains(&measure_fn.as_str())
                && !object_prototype_key(&measure_fn)
            {
                if !is_param(&measure_fn) {
                    return Err(unsupported(
                        &format!("measure {measure_fn}"),
                        "only N.to_nat x, Z.to_nat x or a nat argument are portable measures",
                        Some(span(&open, self.peek())),
                    ));
                }
                self.expect("}", "measure")?;
                return Ok(Some(Annotation::Measure));
            }
            let arg = self.identifier("measure")?.value;
            if !is_param(&arg) {
                return Err(type_error(
                    format!("measure argument {arg} is not a parameter"),
                    Some(open.span()),
                ));
            }
            self.expect("}", "measure")?;
            return Ok(Some(Annotation::Measure));
        }
        Err(unsupported(
            &format!("{{{kind} …}}"),
            "only struct and measure annotations are portable",
            Some(span(&open, self.peek())),
        ))
    }

    /// The termination obligations of a `Function` are the target's to discharge again.
    pub(super) fn skip_obligations(
        &mut self,
        start: &Token,
        annotation: Option<Annotation>,
    ) -> Result<()> {
        if annotation != Some(Annotation::Measure) {
            return Ok(());
        }
        if !self.is("Proof") {
            return Err(unsupported(
                "Function without its obligation proof",
                "the measure obligations must be proved",
                Some(span(start, self.peek())),
            ));
        }
        self.cursor.advance();
        self.end_sentence("Proof")?;
        while !self.cursor.at_end()
            && !((self.is("Defined") || self.is("Qed")) && self.cursor.is_at(".", 1))
        {
            if self.is("Admitted") || self.is("admit") {
                return Err(unsupported(
                    "admitted obligation",
                    "incomplete proofs cannot be translated",
                    Some(span(self.peek(), self.peek())),
                ));
            }
            self.cursor.advance();
        }
        if self.cursor.at_end() {
            return Err(self.fail_here("expected Defined"));
        }
        self.cursor.advance();
        self.end_sentence("Defined")
    }

    pub(super) fn main(&mut self, start: &Token) -> Result<Definition> {
        self.expect("list", "main")?;
        self.expect("string", "main")?;
        self.expect(":=", "main")?;
        let body = self.expr()?;
        self.end_sentence("main")?;
        let mut effects = Vec::new();
        walk_output(body, &mut effects)?;
        let main_span = span(start, self.peek());
        Ok(Definition::Main(
            SMain {
                effects,
                span: Some(main_span),
                sequential_async: false,
            },
            main_span,
        ))
    }
}
