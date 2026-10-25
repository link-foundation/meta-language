//! Actual token/lexical transport for the generated required-field decisions.
use std::sync::OnceLock;

use super::{
    HashSet, JavaScriptParser, JsDoc, Result, SExpr, SNode, SParam, ScanEnd, Span, Stmt, Token,
    TokenKind, span, unsupported,
};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::javascript_record_parameter_rules::{self as rules, ml_string::Text};
use crate::translation::record_callable_parameter_rules::{
    self as callable_rules, ml_string::Text as CallableText,
};
use crate::translation::record_closure_parameter_rules::{
    self as closure_rules, ml_string::Text as ClosureText,
};

#[derive(Clone)]
pub(super) struct RecordParameters {
    pub(super) parameters: Vec<SParam>,
    source_names: Vec<String>,
    bindings: Vec<RecordBinding>,
    array_entries: Option<Vec<super::array_formal_parameters::ArrayFormalEntry>>,
}
impl RecordParameters {
    pub(super) fn from_array_formals(
        parameters: Vec<SParam>,
        source_names: Vec<String>,
        entries: Vec<super::array_formal_parameters::ArrayFormalEntry>,
    ) -> Self {
        Self {
            parameters,
            source_names,
            bindings: Vec::new(),
            array_entries: Some(entries),
        }
    }
}

#[derive(Clone)]
struct RecordBinding {
    parameter: String,
    property: String,
    name: String,
    parameter_span: Span,
    span: Span,
    binding_span: Span,
    property_span: Span,
    fallback: Option<SExpr>,
}

fn reserved_names() -> &'static [Text] {
    static NAMES: OnceLock<Vec<Text>> = OnceLock::new();
    NAMES.get_or_init(|| {
        let policy =
            crate::read_structured_data_lino(include_str!("../../data/array-bindings.lino"))
                .expect("the shared array binding policy is valid");
        policy["reservedBindings"]
            .as_array()
            .expect("ordered reserved bindings")
            .iter()
            .map(|name| Text::from_text(name.as_str().expect("a reserved binding is text")))
            .collect()
    })
}

pub(super) const fn token_kind(token: &Token) -> &'static str {
    match token.kind {
        TokenKind::Punct => "punct",
        TokenKind::Identifier => "identifier",
        TokenKind::Number => "number",
        TokenKind::String => "string",
        TokenKind::RegularExpression => "regex",
        TokenKind::Template => "template",
        TokenKind::Interpolation => "interpolation",
        TokenKind::Eof => "eof",
        _ => "other",
    }
}

const fn parameter(name: String, place: Span) -> SParam {
    SParam {
        name,
        optional: false,
        default_value: None,
        ty: None,
        span: Some(place),
        guard: None,
        rocq_type: None,
    }
}

struct Plan<'a> {
    values: &'a [f64],
    at: usize,
    span: Span,
}

impl Plan<'_> {
    fn position(&mut self) -> Result<usize> {
        let position = self
            .values
            .get(self.at)
            .copied()
            .and_then(number_index)
            .ok_or_else(|| {
                unsupported(
                    "record parameter source transport",
                    "a selected position must belong to the actual formal interval",
                    Some(self.span),
                )
            })?;
        self.at += 1;
        Ok(position)
    }

    fn token(&mut self, interval: &[Token]) -> Result<Token> {
        let position = self.position()?;
        interval.get(position).cloned().ok_or_else(|| {
            unsupported(
                "record parameter source transport",
                "a selected position must belong to the actual formal interval",
                Some(self.span),
            )
        })
    }
}

impl JavaScriptParser {
    pub(super) fn has_standard_builtin_owner(&self, token: &Token) -> bool {
        let name = &token.value;
        let shadowed = self
            .module_binding_scope
            .as_ref()
            .is_some_and(|scope| !scope.known)
            || self.scope.locals.contains(name)
            || self.scope.tdz.contains(name)
            || self.global_binding_names.contains(name)
            || self
                .externals
                .iter()
                .any(|external| external.name() == name.as_str())
            || self.literal_bindings.contains_key(name)
            || self.function_names.contains(name);
        rules::accept_java_script_standard_builtin_owner(&Text::from_text(name), shadowed)
    }

    fn check_record_parameter_directives(&self) -> Result<()> {
        let opening = self.cursor.peek();
        let close = self.matching(self.cursor.index);
        if self
            .cursor
            .tokens
            .get(close)
            .is_none_or(|token| token.kind != TokenKind::Punct || token.value != "}")
        {
            return Err(unsupported(
                "record parameter directive transport",
                "the original function body needs its complete actual closing brace",
                Some(opening.span()),
            ));
        }
        let tokens = &self.cursor.tokens[self.cursor.index + 1..=close];
        let kinds = tokens
            .iter()
            .map(|token| Text::from_text(token_kind(token)))
            .collect::<Vec<_>>();
        let values = tokens
            .iter()
            .map(|token| Text::from_text(&token.value))
            .collect::<Vec<_>>();
        let raws = tokens
            .iter()
            .map(|token| Text::from_text(&self.source.slice(token.start, token.end)))
            .collect::<Vec<_>>();
        let lines = tokens
            .iter()
            .enumerate()
            .map(|(index, token)| {
                let previous = index
                    .checked_sub(1)
                    .map_or(opening.end, |at| tokens[at].end);
                self.source.units()[previous..token.start]
                    .iter()
                    .any(|unit| super::accept_java_script_line_terminator(f64::from(*unit)))
            })
            .collect::<Vec<_>>();
        let strict = rules::find_java_script_record_parameter_strict_directive(
            &kinds, &values, &raws, &lines, 0.0,
        );
        if strict >= 0.0 {
            let token = number_index(strict)
                .and_then(|at| tokens.get(at))
                .ok_or_else(|| {
                    unsupported(
                        "record parameter directive transport",
                        "a selected directive must belong to the actual body interval",
                        Some(opening.span()),
                    )
                })?;
            return Err(unsupported(
                "record parameter strict directive",
                "a non-simple formal parameter list cannot have a Use Strict Directive in its function body",
                Some(token.span()),
            ));
        }
        if strict.to_bits() == (-2.0f64).to_bits() {
            return Err(unsupported(
                "record parameter directive prologue",
                "the selected body prologue needs a represented complete statement boundary",
                Some(opening.span()),
            ));
        }
        Ok(())
    }

    pub(super) fn read_named_record_parameters(
        &mut self,
        doc: Option<&JsDoc>,
        asynchronous: bool,
    ) -> Result<Option<RecordParameters>> {
        self.read_record_parameter_syntax(
            doc,
            asynchronous,
            "ordinary named function declaration",
            false,
        )
    }

    pub(super) fn read_record_closure_parameters(
        &mut self,
        method: bool,
        expression: bool,
    ) -> Result<Option<RecordParameters>> {
        let context =
            closure_rules::read_java_script_record_closure_context(method, expression).to_string();
        self.read_record_parameter_syntax(None, false, &context, false)
    }

    pub(super) fn read_record_constant_parameters(
        &mut self,
        doc: Option<&JsDoc>,
        asynchronous: bool,
        expression: bool,
    ) -> Result<Option<RecordParameters>> {
        let context =
            callable_rules::read_java_script_record_constant_context(expression).to_string();
        self.read_record_parameter_syntax(doc, asynchronous, &context, true)
    }
    pub(super) fn read_record_namespace_parameters(
        &mut self,
        doc: Option<&JsDoc>,
        asynchronous: bool,
    ) -> Result<Option<RecordParameters>> {
        self.read_record_parameter_syntax(
            doc,
            asynchronous,
            "ordinary namespace object method",
            true,
        )
    }

    fn read_record_parameter_syntax(
        &mut self,
        doc: Option<&JsDoc>,
        asynchronous: bool,
        context: &str,
        callable: bool,
    ) -> Result<Option<RecordParameters>> {
        let start = self.cursor.index;
        let Some(opening) = start
            .checked_sub(1)
            .and_then(|at| self.cursor.tokens.get(at))
            .cloned()
        else {
            return Ok(None);
        };
        if opening.kind != TokenKind::Punct || opening.value != "(" {
            return Ok(None);
        }
        let close = self.matching(start - 1);
        if self
            .cursor
            .tokens
            .get(close)
            .is_none_or(|token| token.kind != TokenKind::Punct || token.value != ")")
        {
            return Ok(None);
        }
        // Only the actual formal interval supplies facts, never a whole-module
        // projection per declaration. Native token ownership is parser-private.
        let interval = self.cursor.tokens[start..=close].to_vec();
        let kinds = interval
            .iter()
            .map(|token| Text::from_text(token_kind(token)))
            .collect::<Vec<_>>();
        let values = interval
            .iter()
            .map(|token| Text::from_text(&token.value))
            .collect::<Vec<_>>();
        let selected = rules::find_java_script_record_parameter_pattern(&kinds, &values, 0.0);
        if selected < 0.0 {
            return Ok(None);
        }
        let selected = number_index(selected)
            .and_then(|at| interval.get(at))
            .unwrap_or(&opening);
        let documentation_parameters = doc.map_or(0, |doc| doc.params.len());
        let documentation_parameters = serde_json::Number::from(documentation_parameters)
            .as_f64()
            .expect("a source parameter count is a Number");
        let admitted = if callable {
            callable_rules::accept_java_script_record_callable_parameter_profile(
                &CallableText::from_text(context),
                asynchronous,
                documentation_parameters,
            )
        } else if context == "ordinary named function declaration" {
            rules::accept_java_script_record_parameter_profile(
                &Text::from_text(context),
                asynchronous,
                documentation_parameters,
                false,
                false,
            )
        } else {
            closure_rules::accept_java_script_record_closure_parameter_profile(
                &ClosureText::from_text(context),
                asynchronous,
                documentation_parameters,
            )
        };
        if !admitted && callable {
            return Err(unsupported(
                "record callable source profile",
                "ordinary non-simple callables require synchronous bodies without parameter documentation tags",
                Some(selected.span()),
            ));
        }
        if !admitted {
            return Err(unsupported(
                "record parameter source profile",
                "the initial named declaration profile requires synchronous functions without parameter documentation tags",
                Some(selected.span()),
            ));
        }
        let values = rules::plan_java_script_source_record_parameters_with_defaults(
            &kinds,
            &values,
            0.0,
            reserved_names(),
        );
        if values.first().is_some_and(|value| *value < 0.0) {
            let token = values
                .get(1)
                .copied()
                .and_then(number_index)
                .and_then(|at| interval.get(at))
                .unwrap_or_else(|| self.cursor.peek());
            // Reason eight carries the actual rejected BoundName position.
            if values[0].to_bits() == (-8f64).to_bits() {
                crate::translation::lexer::require_java_script_identifier_role(token, true)?;
            }
            return Err(unsupported(
                "record parameter binding",
                &format!(
                    "a nonempty flat required object pattern needs distinct lexical names; defaults, arrays, nested/rest/computed patterns are separate obligations (reason {})",
                    -values[0]
                ),
                Some(token.span()),
            ));
        }
        let mut plan = Plan {
            values: &values,
            at: 0,
            span: opening.span(),
        };
        let end = plan.position()?;
        let count = plan.position()?;
        if end != interval.len() {
            return Err(unsupported(
                "record parameter source transport",
                "the plan needs the complete actual formal interval and its parser-owned token/name inventory",
                Some(opening.span()),
            ));
        }
        // Context is installed after new(), so reserve it at the first selected
        // source pattern. Preserve the growing set and first-free ordinal.
        if self.record_parameter_names.is_none() {
            let names = self
                .cursor
                .tokens
                .iter()
                .filter(|token| token.kind == TokenKind::Identifier)
                .map(|token| token.value.clone())
                .chain(self.externals.iter().map(|item| item.name().to_owned()))
                .chain(self.literal_bindings.keys().cloned())
                .chain(self.global_binding_names.iter().cloned())
                .collect();
            self.record_parameter_names = Some(names);
        }
        let binding_positions =
            rules::read_java_script_record_formal_default_binding_positions(&values);
        let bound_names = binding_positions
            .iter()
            .map(|position| {
                interval[number_index(*position).expect("actual admitted BoundName position")]
                    .value
                    .clone()
            })
            .collect::<Vec<_>>();
        let mut syntax = RecordParameters {
            parameters: Vec::new(),
            source_names: Vec::new(),
            bindings: Vec::new(),
            array_entries: None,
        };
        while plan.at < values.len() {
            let form = plan.position()?;
            let slot = plan.position()?;
            if slot != syntax.parameters.len() {
                return Err(unsupported(
                    "record parameter source transport",
                    "logical arguments retain their actual ordered slots",
                    Some(opening.span()),
                ));
            }
            if form == 0 {
                let token = plan.token(&interval)?;
                crate::translation::lexer::require_java_script_identifier_role(&token, true)?;
                syntax
                    .parameters
                    .push(parameter(token.value.clone(), token.span()));
                syntax.source_names.push(token.value);
                continue;
            }
            if form != 1 {
                return Err(unsupported(
                    "record parameter source transport",
                    "a parameter needs a represented ordinary or flat record form",
                    Some(opening.span()),
                ));
            }
            let open = plan.token(&interval)?;
            let close = plan.token(&interval)?;
            let pairs = plan.position()?;
            let name = loop {
                if self.record_parameter_ordinal >= 9_007_199_254_740_991 {
                    return Err(unsupported(
                        "record parameter source transport",
                        "the parser-owned logical argument name ordinal exceeds its safe integer range",
                        Some(open.span()),
                    ));
                }
                let name = format!("record_parameter_value{}", self.record_parameter_ordinal);
                self.record_parameter_ordinal += 1;
                if self
                    .record_parameter_names
                    .as_mut()
                    .expect("the actual name set was initialized")
                    .insert(name.clone())
                {
                    break name;
                }
            };
            let parameter_span = Span::new(open.start, close.end);
            syntax
                .parameters
                .push(parameter(name.clone(), parameter_span));
            for _ in 0..pairs {
                let property = plan.token(&interval)?;
                let binding = values[plan.at];
                let local = plan.token(&interval)?;
                let first = values[plan.at];
                let last = values[plan.at + 1];
                plan.at += 2;
                crate::translation::lexer::require_java_script_identifier_role(&local, true)?;
                let fallback = if let Some(first) = number_index(first) {
                    let last = number_index(last).expect("actual source initializer end");
                    let boundary=crate::translation::array_formal_parameter_rules::read_java_script_array_formal_default_scope_boundary(&crate::translation::array_formal_parameter_rules::ml_string::Text::from_text(context));
                    Some(
                        self.read_formal_initializer(
                            &interval,
                            super::formal_initializers::FormalInitializer {
                                interval_positions: (first, last, start),
                                binding_positions: &binding_positions,
                                binding,
                                bound_names: &bound_names,
                                boundary: u8::try_from(
                                    number_index(boundary)
                                        .expect("actual source callable boundary"),
                                )
                                .expect("boundary is one or two"),
                                profile: "record formal default",
                            },
                        )?,
                    )
                } else {
                    None
                };
                syntax.bindings.push(RecordBinding {
                    parameter: name.clone(),
                    property: property.value.clone(),
                    name: local.value.clone(),
                    parameter_span,
                    span: Span::new(
                        property.start,
                        fallback
                            .as_ref()
                            .and_then(|value| value.span)
                            .map_or(local.end, |place| place.end),
                    ),
                    binding_span: local.span(),
                    property_span: property.span(),
                    fallback,
                });
                syntax.source_names.push(local.value);
            }
        }
        if syntax.parameters.len() != count {
            return Err(unsupported(
                "record parameter source transport",
                "every actual argument slot must be represented",
                Some(opening.span()),
            ));
        }
        self.cursor.index = start + end;
        Ok(Some(syntax))
    }

    pub(super) fn read_named_record_parameter_body(
        &mut self,
        syntax: &RecordParameters,
    ) -> Result<Vec<Stmt>> {
        self.read_record_parameter_body_with(syntax, Self::block_statements)
    }

    pub(super) fn read_record_parameter_body_with<F>(
        &mut self,
        syntax: &RecordParameters,
        read_body: F,
    ) -> Result<Vec<Stmt>>
    where
        F: FnOnce(&mut Self) -> Result<Vec<Stmt>>,
    {
        let names = syntax.source_names.iter().cloned().collect::<HashSet<_>>();
        if self.cursor.is_kind(TokenKind::Punct) && self.cursor.is("{") {
            self.check_record_parameter_directives()?;
            let declarations = self.block_declarations(self.cursor.index + 1, ScanEnd::Brace);
            for name in &syntax.source_names {
                if declarations.contains(name) {
                    return Err(unsupported(
                        "record parameter body",
                        &format!(
                            "a top-level lexical declaration cannot redeclare parameter {name}"
                        ),
                        Some(self.peek().span()),
                    ));
                }
            }
        }
        for name in &syntax.source_names {
            self.scope.locals.insert(name.clone());
            self.scope.mutable.insert(name.clone());
            self.lexical.parameter(name);
            self.lexical
                .tag_binding(name, super::tag_context::TagReceiverFact::Declared(None));
        }
        let before = self.lexical.read_count();
        let statements = read_body(self)?;
        let written = self
            .lexical
            .assigned_names_since(before)
            .filter(|name| names.contains(*name))
            .map(str::to_owned)
            .collect::<HashSet<_>>();
        let mut prefix = if let Some(entries) = &syntax.array_entries {
            super::array_formal_parameters::binding_statements(entries, &written)
        } else {
            syntax
                .bindings
                .iter()
                .map(|binding| {
                    let value = SExpr::new(
                        SNode::Field {
                            object: Box::new(SExpr::new(
                                SNode::Name {
                                    path: vec![binding.parameter.clone()],
                                },
                                Some(binding.parameter_span),
                            )),
                            field: binding.property.clone(),
                        },
                        Some(binding.span),
                    );
                    let value = if let Some(fallback) = &binding.fallback {
                        SExpr::new(
                            SNode::Math {
                                op: rules::read_java_script_record_formal_default_operation()
                                    .to_string(),
                                name: String::new(),
                                args: vec![
                                    crate::translation::surface::SArrayItem::new(
                                        SExpr::new(
                                            SNode::Name {
                                                path: vec![binding.parameter.clone()],
                                            },
                                            Some(binding.parameter_span),
                                        ),
                                        false,
                                    ),
                                    crate::translation::surface::SArrayItem::new(
                                        SExpr::new(
                                            SNode::Str {
                                                value: binding.property.clone(),
                                            },
                                            Some(binding.property_span),
                                        ),
                                        false,
                                    ),
                                    crate::translation::surface::SArrayItem::new(
                                        fallback.clone(),
                                        false,
                                    ),
                                ],
                            },
                            Some(binding.span),
                        )
                    } else {
                        value
                    };
                    if rules::read_java_script_record_parameter_binding_statement(
                        written.contains(&binding.name),
                    )
                    .to_string()
                        == "let"
                    {
                        Stmt::Let {
                            name: binding.name.clone(),
                            value,
                            span: binding.binding_span,
                        }
                    } else {
                        Stmt::Const {
                            name: binding.name.clone(),
                            value,
                            span: binding.binding_span,
                        }
                    }
                })
                .collect::<Vec<_>>()
        };
        prefix.extend(statements);
        Ok(prefix)
    }
}
