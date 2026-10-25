//! Actual formal intervals, private binder names and original source expressions.
//! The generated source plan owns syntax and BoundName admission.
use super::record_parameters::RecordParameters;
use super::{
    JavaScriptParser, JsDoc, Result, SExpr, SNode, SParam, Span, Stmt, Token, TokenKind,
    unsupported,
};
use crate::translation::array_formal_parameter_rules::{self as rules, ml_string::Text};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::surface::SArrayItem;
use std::collections::{BTreeMap, HashSet};

#[derive(Clone)]
pub(super) struct ArrayFormalEntry {
    temporary: bool,
    name: String,
    value: SExpr,
    span: Span,
}
pub(super) fn binding_statements(
    entries: &[ArrayFormalEntry],
    written: &HashSet<String>,
) -> Vec<Stmt> {
    entries.iter().map(|entry|{
  let mutable=crate::translation::javascript_record_parameter_rules::read_java_script_record_parameter_binding_statement(written.contains(&entry.name)).to_string()=="let";
  if !entry.temporary&&mutable {Stmt::Let{name:entry.name.clone(),value:entry.value.clone(),span:entry.span}}
  else {Stmt::Const{name:entry.name.clone(),value:entry.value.clone(),span:entry.span}}
 }).collect()
}
fn read_name(name: &str, span: Span) -> SExpr {
    SExpr::new(
        SNode::Name {
            path: vec![name.to_owned()],
        },
        Some(span),
    )
}

impl JavaScriptParser {
    fn array_formal_name(&mut self, prefix: &str, place: Span) -> Result<String> {
        if self.record_parameter_names.is_none() {
            self.record_parameter_names = Some(
                self.cursor
                    .tokens
                    .iter()
                    .filter(|token| token.kind == TokenKind::Identifier)
                    .map(|token| token.value.clone())
                    .chain(self.externals.iter().map(|item| item.name().to_owned()))
                    .chain(self.literal_bindings.keys().cloned())
                    .chain(self.global_binding_names.iter().cloned())
                    .collect(),
            );
        }
        loop {
            if self.record_parameter_ordinal >= 9_007_199_254_740_991 {
                return Err(unsupported(
                    "array formal parameter transport",
                    "the logical binder supply exceeds its exact ordinal range",
                    Some(place),
                ));
            }
            let name = format!("{prefix}{}", self.record_parameter_ordinal);
            self.record_parameter_ordinal += 1;
            if self
                .record_parameter_names
                .as_mut()
                .expect("the actual binder inventory is installed")
                .insert(name.clone())
            {
                return Ok(name);
            }
        }
    }
    fn read_array_formal_default(
        &mut self,
        tokens: &[Token],
        interval_positions: (usize, usize, usize),
        plan: &[f64],
        binding: f64,
        bound_names: &[String],
        boundary: u8,
    ) -> Result<SExpr> {
        let (first, last, offset) = interval_positions;
        let initialization =
            rules::partition_java_script_array_formal_initializer_bindings(plan, binding);
        let count = number_index(initialization[0])
            .expect("the source plan owns an actual available name count");
        let names = |positions: &[f64]| {
            positions
                .iter()
                .map(|position| {
                    tokens[number_index(*position)
                        .expect("the source plan owns an actual binding position")]
                    .value
                    .clone()
                })
                .collect::<Vec<_>>()
        };
        let available = names(&initialization[1..1 + count]);
        let uninitialized = names(&initialization[1 + count..]);
        let interval = tokens.get(first..last).ok_or_else(|| {
            unsupported(
                "array formal default transport",
                "the actual expression must consume its complete planned initializer interval",
                tokens.get(first).map(Token::span),
            )
        })?;
        let kinds = interval
            .iter()
            .map(|token| super::record_parameters::token_kind(token).to_owned())
            .collect::<Vec<_>>();
        let values = interval
            .iter()
            .map(|token| token.value.clone())
            .collect::<Vec<_>>();
        let reference = crate::translation::frontend_rules::find_default_parameter_reference(
            kinds,
            values,
            uninitialized.clone(),
        );
        if let Some(token) = number_index(reference).and_then(|index| interval.get(index)) {
            return Err(unsupported(
                &format!("array formal default reading parameter {}", token.value),
                "the current default profile requires a closed parameter initialization environment",
                Some(token.span()),
            ));
        }
        let saved = self.cursor.index;
        let outer = self.scope.clone();
        let before = self.lexical.read_count();
        self.cursor.index = offset + first;
        self.scope.locals.extend(bound_names.iter().cloned());
        self.scope.mutable.extend(available.iter().cloned());
        for name in &available {
            self.scope.tdz.remove(name);
        }
        self.scope.tdz.extend(uninitialized);
        let parsed = self.with_lexical_scope(bound_names, boundary, Self::expr);
        let consumed = self.cursor.index;
        self.cursor.index = saved;
        self.scope = outer;
        let value = parsed?;
        if consumed != offset + last {
            return Err(unsupported(
                "array formal default transport",
                "the actual expression must consume its complete planned initializer interval",
                tokens.get(first).map(Token::span),
            ));
        }
        let assignments = self
            .lexical
            .assigned_names_since(before)
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let assigned = rules::find_java_script_array_formal_initializer_assignment(
            &assignments
                .iter()
                .map(|name| Text::from_text(name))
                .collect::<Vec<_>>(),
            &bound_names
                .iter()
                .map(|name| Text::from_text(name))
                .collect::<Vec<_>>(),
        );
        if let Some(name) = number_index(assigned).and_then(|position| assignments.get(position)) {
            return Err(unsupported(
                &format!("array formal default assigning parameter {name}"),
                "writes during parameter initialization need their shared environment interpretation",
                tokens.get(first).map(Token::span),
            ));
        }
        Ok(value)
    }
    pub(super) fn read_array_formal_parameters(
        &mut self,
        context: &str,
        doc: Option<&JsDoc>,
        asynchronous: bool,
    ) -> Result<Option<RecordParameters>> {
        let start = self.cursor.index;
        let Some(opening) = start
            .checked_sub(1)
            .and_then(|index| self.cursor.tokens.get(index))
            .cloned()
        else {
            return Ok(None);
        };
        if opening.kind != TokenKind::Punct || opening.value != "(" {
            return Ok(None);
        }
        let close = self.matching(start - 1);
        let Some(closing) = self.cursor.tokens.get(close) else {
            return Ok(None);
        };
        if closing.kind != TokenKind::Punct || closing.value != ")" {
            return Ok(None);
        }
        let tokens = self.cursor.tokens[start..=close].to_vec();
        let kinds = tokens
            .iter()
            .map(|token| Text::from_text(super::record_parameters::token_kind(token)))
            .collect::<Vec<_>>();
        let values = tokens
            .iter()
            .map(|token| Text::from_text(&token.value))
            .collect::<Vec<_>>();
        let selected = rules::find_java_script_array_formal_pattern(&kinds, &values, 0.0);
        if selected < 0.0 {
            return Ok(None);
        }
        let selected = number_index(selected)
            .and_then(|index| tokens.get(index))
            .unwrap_or(&opening);
        let documentation = serde_json::Number::from(doc.map_or(0, |doc| doc.params.len()))
            .as_f64()
            .expect("source parameter counts have a Number representation");
        if !rules::accept_java_script_array_formal_context(
            &Text::from_text(context),
            asynchronous,
            documentation,
        ) {
            return Err(unsupported(
                "array formal parameter source profile",
                "ordinary array patterns require synchronous callables without parameter documentation tags",
                Some(selected.span()),
            ));
        }
        let policy =
            crate::read_structured_data_lino(include_str!("../../data/array-bindings.lino"))
                .expect("the actual array binding policy is structured Links");
        let reserved = policy["reservedBindings"]
            .as_array()
            .expect("the policy owns an ordered reserved name column")
            .iter()
            .map(|value| Text::from_text(value.as_str().expect("a reserved name is text")))
            .collect::<Vec<_>>();
        let plan = rules::plan_java_script_array_formal_parameters_with_defaults(
            &kinds, &values, 0.0, &reserved,
        );
        if plan[0] < 0.0 {
            let selected = number_index(plan[1])
                .and_then(|index| tokens.get(index))
                .unwrap_or(&opening);
            return Err(unsupported(
                "array formal parameter syntax",
                &format!(
                    "flat names, elisions and final named rest use distinct actual BoundNames; defaults and nested/mixed object patterns need separate interpretations (reason {})",
                    -plan[0]
                ),
                Some(selected.span()),
            ));
        }
        let position = |value| {
            number_index(value).ok_or_else(|| {
                unsupported(
                    "array formal parameter transport",
                    "every selected token belongs to its actual formal interval",
                    Some(opening.span()),
                )
            })
        };
        let actual = |value| -> Result<&Token> {
            tokens.get(position(value)?).ok_or_else(|| {
                unsupported(
                    "array formal parameter transport",
                    "every selected token belongs to its actual formal interval",
                    Some(opening.span()),
                )
            })
        };
        let bound_names = rules::read_java_script_array_formal_default_binding_positions(&plan)
            .iter()
            .map(|value| actual(*value).map(|token| token.value.clone()))
            .collect::<Result<Vec<_>>>()?;
        let mut parameters = Vec::new();
        let mut source_names = Vec::new();
        let mut entries = Vec::new();
        let mut cursor = 2;
        while cursor < plan.len() {
            let form = position(plan[cursor])?;
            let slot = position(plan[cursor + 1])?;
            cursor += 2;
            if slot != parameters.len() {
                return Err(unsupported(
                    "array formal parameter transport",
                    "logical argument slots retain their actual order",
                    Some(opening.span()),
                ));
            }
            if form == 0 {
                let token = actual(plan[cursor])?;
                cursor += 1;
                crate::translation::lexer::require_java_script_identifier_role(token, true)?;
                parameters.push(parameter(&token.value, token.span()));
                source_names.push(token.value.clone());
                continue;
            }
            if form != 2 {
                return Err(unsupported(
                    "array formal parameter transport",
                    "unknown actual formal syntax plan",
                    Some(opening.span()),
                ));
            }
            let open = actual(plan[cursor])?;
            let end = actual(plan[cursor + 1])?;
            let count = position(plan[cursor + 2])?;
            cursor += 3;
            let place = Span::new(open.start, end.end);
            let name = self.array_formal_name("array_parameter_value", place)?;
            let snapshot = self.array_formal_name("array_parameter_values", place)?;
            parameters.push(parameter(&name, place));
            entries.push(ArrayFormalEntry {
                temporary: true,
                name: snapshot.clone(),
                value: SExpr::new(
                    SNode::Math {
                        op: rules::read_java_script_array_formal_iterable_operation().to_string(),
                        name: String::new(),
                        args: vec![SArrayItem::new(read_name(&name, place), false)],
                    },
                    Some(place),
                ),
                span: place,
            });
            for _ in 0..count {
                let rest = position(plan[cursor])? == 1;
                let offset = position(plan[cursor + 1])?;
                let binding = plan[cursor + 2];
                let token = actual(binding)?;
                let default_start = plan[cursor + 3];
                let default_end = plan[cursor + 4];
                cursor += 5;
                crate::translation::lexer::require_java_script_identifier_role(token, true)?;
                source_names.push(token.value.clone());
                let offset = SExpr::new(
                    SNode::Num {
                        value: offset.to_string(),
                        ty: None,
                        negative: false,
                        unit: false,
                    },
                    Some(token.span()),
                );
                let value = if rest {
                    SExpr::new(
                        SNode::ArrayBindingRest {
                            object: Box::new(read_name(&snapshot, place)),
                            start: serde_json::to_value(&offset)
                                .expect("an actual offset expression has source data"),
                            extra: BTreeMap::new(),
                        },
                        Some(place),
                    )
                } else {
                    SExpr::new(
                        SNode::Index {
                            object: Box::new(read_name(&snapshot, place)),
                            index: Box::new(offset),
                            optional: None,
                            proven_in_bounds: None,
                        },
                        Some(place),
                    )
                };
                let value = if default_start >= 0.0 {
                    let first = position(default_start)?;
                    let last = position(default_end)?;
                    let fallback = self.read_array_formal_default(
                        &tokens,
                        (first, last, start),
                        &plan,
                        binding,
                        &bound_names,
                        u8::try_from(position(
                            rules::read_java_script_array_formal_default_scope_boundary(
                                &Text::from_text(context),
                            ),
                        )?)
                        .expect("actual source boundary is one or two"),
                    )?;
                    SExpr::new(
                        SNode::ArrayBindingDefault {
                            value: Box::new(value),
                            fallback: Box::new(fallback),
                            extra: BTreeMap::new(),
                        },
                        Some(place),
                    )
                } else {
                    value
                };
                entries.push(ArrayFormalEntry {
                    temporary: false,
                    name: token.value.clone(),
                    value,
                    span: token.span(),
                });
            }
        }
        if parameters.len() != position(plan[1])? {
            return Err(unsupported(
                "array formal parameter transport",
                "every actual parameter slot must be retained",
                Some(opening.span()),
            ));
        }
        self.cursor.index = start + position(plan[0])?;
        Ok(Some(RecordParameters::from_array_formals(
            parameters,
            source_names,
            entries,
        )))
    }
}
fn parameter(name: &str, place: Span) -> SParam {
    SParam {
        name: name.to_owned(),
        ty: None,
        optional: false,
        default_value: None,
        guard: None,
        rocq_type: None,
        span: Some(place),
    }
}
