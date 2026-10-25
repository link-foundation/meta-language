//! Actual token and scope transport for original formal initializer intervals.
use super::{JavaScriptParser, Result, SExpr, Token, unsupported};
use crate::translation::array_formal_parameter_rules::{self as rules, ml_string::Text};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::javascript_record_parameter_rules::{
    self as context_rules, ml_string::Text as ContextText,
};
pub(super) struct FormalInitializer<'a> {
    pub(super) interval_positions: (usize, usize, usize),
    pub(super) binding_positions: &'a [f64],
    pub(super) binding: f64,
    pub(super) bound_names: &'a [String],
    pub(super) context: &'a str,
    pub(super) profile: &'a str,
}
impl JavaScriptParser {
    pub(super) fn read_formal_initializer(
        &mut self,
        tokens: &[Token],
        initializer: FormalInitializer<'_>,
    ) -> Result<SExpr> {
        let FormalInitializer {
            interval_positions: (first, last, offset),
            binding_positions,
            binding,
            bound_names,
            context,
            profile,
        } = initializer;
        let context = crate::translation::recursive_binding_rules::read_java_script_recursive_binding_initializer_context(&crate::translation::recursive_binding_rules::ml_string::Text::from_text(context)).to_string();
        let text = ContextText::from_text(&context);
        let boundary = u8::try_from(
            number_index(context_rules::read_java_script_pattern_initializer_scope_boundary(&text))
                .expect("actual source initializer boundary"),
        )
        .expect("boundary is zero, one or two");
        let subject =
            context_rules::read_java_script_pattern_initializer_subject(&text).to_string();
        let initialization = rules::partition_java_script_formal_initializer_binding_positions(
            binding_positions,
            binding,
        );
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
                &format!("{profile} transport"),
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
                &format!("{profile} reading {subject} {}", token.value),
                &context_rules::read_java_script_pattern_initializer_reference_reason(&text)
                    .to_string(),
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
                &format!("{profile} transport"),
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
                &format!("{profile} assigning {subject} {name}"),
                &context_rules::read_java_script_pattern_initializer_assignment_reason(&text)
                    .to_string(),
                tokens.get(first).map(Token::span),
            ));
        }
        Ok(value)
    }
}
