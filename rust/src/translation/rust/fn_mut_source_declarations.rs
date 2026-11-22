//! Retain original lexer coordinates before the ordinary body reader specializes them.

use super::{Result, Span, Token, unsupported};
use crate::translation::{
    lexer::Source,
    rust_fnmut_source_token_rules::{
        read_rust_fn_mut_source_token_error, read_rust_fn_mut_source_token_plan,
    },
    source_diagnostic_attributes::token_kind,
};
use crate::translation::{
    source_fn_mut_specializations::{GenericBinder, OriginalCallback, SourceDeclaration},
    source_loop_folds::SourceFoldSpan,
};
use std::{ops::Range, sync::Arc};

#[derive(Clone, Debug)]
pub(super) struct CallbackDeclaration {
    pub(super) parameter: usize,
    pub(super) binder: usize,
    pub(super) exclusive: bool,
    pub(super) mutable: bool,
    pub(super) callback_type: Range<usize>,
    pub(super) parameters: Vec<Range<usize>>,
    pub(super) result: Range<usize>,
}

#[derive(Clone, Debug)]
pub(super) struct FnMutSourceDeclaration {
    pub(super) owner: Vec<String>,
    pub(super) name: String,
    pub(super) declaration: Range<usize>,
    pub(super) body: Range<usize>,
    pub(super) result: Option<Range<usize>>,
    pub(super) formal_count: usize,
    pub(super) generics: Vec<usize>,
    pub(super) callback: CallbackDeclaration,
}

pub(super) struct FnMutSourceDeclarations {
    source: Source,
    pub(super) source_version: String,
    tokens: Arc<Vec<Token>>,
    kinds: Vec<String>,
    values: Vec<String>,
}

fn refuse(reason: &str, span: Option<Span>) -> super::TranslationError {
    unsupported("Rust source FnMut specialization", reason, span)
}

fn position(value: f64) -> Option<usize> {
    crate::translation::rust_fnmut_source_token_rules::ml_array::number_index(value)
}

impl FnMutSourceDeclarations {
    pub(super) fn new(
        source: &str,
        tokens: Arc<Vec<Token>>,
        source_version: String,
    ) -> Result<Self> {
        const SOURCE_LIMIT: usize = 16_777_216;
        if source.encode_utf16().take(SOURCE_LIMIT + 1).count() > SOURCE_LIMIT {
            return Err(refuse(
                "the bounded actual original source and retained lexer inventory are required",
                None,
            ));
        }
        Ok(Self {
            source: Source::new(source),
            source_version,
            kinds: tokens
                .iter()
                .map(|token| token_kind(token).to_owned())
                .collect(),
            values: tokens.iter().map(|token| token.value.clone()).collect(),
            tokens,
        })
    }

    pub(super) fn read_tokens(&self, range: &Range<usize>) -> Vec<Token> {
        let mut tokens = self.tokens[range.clone()].to_vec();
        let mut end = self
            .tokens
            .last()
            .expect("the actual source lexer retains EOF")
            .clone();
        let offset = tokens
            .last()
            .expect("an admitted source range is nonempty")
            .end;
        end.start = offset;
        end.end = offset;
        tokens.push(end);
        tokens
    }

    pub(super) fn position(&self, range: &Range<usize>) -> SourceFoldSpan {
        SourceFoldSpan {
            start: self.tokens[range.start].start,
            end: self.tokens[range.end - 1].end,
        }
    }

    pub(super) fn checked_owner(&self, declaration: &FnMutSourceDeclaration) -> SourceDeclaration {
        let callback = &declaration.callback;
        let binder = &self.tokens[callback.binder];
        let path = declaration
            .owner
            .iter()
            .cloned()
            .chain(std::iter::once(declaration.name.clone()))
            .collect();
        SourceDeclaration {
            path,
            range: self.position(&declaration.declaration),
            body_range: self.position(&declaration.body),
            generics: declaration
                .generics
                .iter()
                .map(|index| {
                    let binder = &self.tokens[*index];
                    GenericBinder {
                        name: binder.value.clone(),
                        start: binder.start,
                        end: binder.end,
                    }
                })
                .collect(),
            formal_count: declaration.formal_count,
            callback: OriginalCallback {
                index: callback.parameter,
                name: binder.value.clone(),
                mode: if callback.exclusive {
                    "exclusive"
                } else {
                    "value"
                }
                .to_owned(),
                mutable: callback.mutable,
                binder: SourceFoldSpan {
                    start: binder.start,
                    end: binder.end,
                },
                type_range: self.position(&callback.callback_type),
                parameter_ranges: callback
                    .parameters
                    .iter()
                    .map(|range| self.position(range))
                    .collect(),
                result_range: self.position(&callback.result),
            },
        }
    }

    pub(super) fn capture(
        &self,
        owner: &[String],
        start: &Token,
    ) -> Result<Option<FnMutSourceDeclaration>> {
        let unavailable = || {
            refuse(
                &read_rust_fn_mut_source_token_error(1.0),
                Some(start.span()),
            )
        };
        let index = self
            .tokens
            .binary_search_by_key(&start.start, |token| token.start)
            .map_err(|_| unavailable())?;
        if self.tokens[index] != *start || owner.iter().any(String::is_empty) {
            return Err(unavailable());
        }
        let number = u32::try_from(index)
            .map(f64::from)
            .map_err(|_| unavailable())?;
        let plan = read_rust_fn_mut_source_token_plan(&self.kinds, &self.values, number);
        if plan.is_empty() {
            return Ok(None);
        }
        if let [-1.0, reason, selected] = plan.as_slice() {
            return Err(refuse(
                &read_rust_fn_mut_source_token_error(*reason),
                position(*selected)
                    .and_then(|selected| self.tokens.get(selected))
                    .map(Token::span)
                    .or(Some(start.span())),
            ));
        }
        let invalid = || {
            refuse(
                "the declaration plan must retain every actual original source position",
                Some(start.span()),
            )
        };
        let integer = |value: f64| position(value).ok_or_else(invalid);
        let range = |from, to| {
            let from = integer(from)?;
            let to = integer(to)?;
            if from >= to || from < index || to > self.tokens.len() {
                return Err(invalid());
            }
            Ok(from..to)
        };
        let [
            body_start,
            body_end,
            result_start,
            result_end,
            formals,
            callback,
            generic_count,
            remaining @ ..,
        ] = plan.as_slice()
        else {
            return Err(invalid());
        };
        let (generics, remaining) = remaining
            .split_at_checked(integer(*generic_count)?)
            .ok_or_else(invalid)?;
        let [
            binder,
            exclusive,
            mutable,
            callback_start,
            callback_end,
            output_start,
            output_end,
            parameters @ ..,
        ] = remaining
        else {
            return Err(invalid());
        };
        let body = range(*body_start, *body_end + 1.0)?;
        let declaration = index..body.end;
        let mut previous_end = start.start;
        for token in &self.tokens[declaration.clone()] {
            if token.start < previous_end
                || token.end < token.start
                || token.end > self.source.len()
                || self.source.slice(token.start, token.end) != token.raw
            {
                return Err(refuse(
                    "each generic body token must retain its real original source spelling and range",
                    Some(token.span()),
                ));
            }
            previous_end = token.end;
        }
        if parameters.len() % 2 != 0 {
            return Err(invalid());
        }
        Ok(Some(FnMutSourceDeclaration {
            owner: owner.to_vec(),
            name: self.tokens[index + 1].value.clone(),
            declaration,
            body,
            result: if *result_start < 0.0 {
                None
            } else {
                Some(range(*result_start, *result_end)?)
            },
            formal_count: integer(*formals)?,
            generics: generics
                .iter()
                .copied()
                .map(integer)
                .collect::<Result<_>>()?,
            callback: CallbackDeclaration {
                parameter: integer(*callback)?,
                binder: integer(*binder)?,
                exclusive: *exclusive == 1.0,
                mutable: *mutable == 1.0,
                callback_type: range(*callback_start, *callback_end)?,
                parameters: parameters
                    .chunks_exact(2)
                    .map(|pair| range(pair[0], pair[1]))
                    .collect::<Result<_>>()?,
                result: range(*output_start, *output_end)?,
            },
        }))
    }
}

include!("../../../tests/internal/translation/rust/fn_mut_source_declarations.rs");
