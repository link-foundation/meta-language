//! Named field patterns are ordinary ordered constructor patterns.
use super::super::{
    Alias, Result, RustParser, SPattern, SPatternNode, SourceDeclaration, Span, Token, TokenKind,
    span, type_error, unsupported,
};
use crate::translation::rust_source_render_interfaces::plan_rust_record_pattern_fields;

impl RustParser {
    pub(in super::super) fn expand_constructor_rest(
        &self,
        resolved: &[String],
        args: Vec<SPattern>,
        rest: Option<(usize, Token)>,
        path: &[String],
    ) -> Result<Vec<SPattern>> {
        let Some((position, token)) = rest else {
            return Ok(args);
        };
        let fields = self.origins.lookup(resolved, path).and_then(|item| {
            if let SourceDeclaration::Constructor { fields, .. } = item.declaration {
                Some(fields.len())
            } else {
                None
            }
        });
        let Some(fields) = fields else {
            if self.origins.collecting {
                return Ok(args);
            }
            return Err(unsupported(
                "Rust constructor rest pattern",
                "the constructor needs its actual source field count",
                Some(token.span()),
            ));
        };
        let count = |value| {
            f64::from(
                u32::try_from(value).expect("source constructor width fits the shared array limit"),
            )
        };
        let plan = crate::translation::rust_source_render_interfaces::plan_tuple_pattern_fields(
            count(fields),
            count(args.len()),
            count(position),
        );
        if plan.first() != Some(&0.0) {
            return Err(type_error(
                "a Rust constructor rest pattern has too many fields",
                Some(token.span()),
            ));
        }
        plan.into_iter()
            .skip(1)
            .map(|index| {
                if index < 0.0 {
                    Ok(SPattern {
                        node: SPatternNode::Wild,
                        span: Some(token.span()),
                    })
                } else {
                    let index = index.to_string().parse::<usize>().map_err(|_| {
                        type_error(
                            "constructor pattern plan has an invalid source index",
                            Some(token.span()),
                        )
                    })?;
                    Ok(args[index].clone())
                }
            })
            .collect()
    }
    pub(in super::super) fn record_pattern(
        &mut self,
        segments: &[String],
        path: &[String],
        aliases: &mut Vec<Alias>,
        start: &Token,
    ) -> Result<SPattern> {
        let record = self.record_brace_owner(segments, path);
        let mut names = Vec::new();
        let mut patterns = Vec::new();
        let mut rest = None;
        self.cursor.expect("{", Some("record pattern"))?;
        while !self.cursor.is("}") {
            if self.cursor.is("..") {
                rest = Some(self.cursor.advance());
                break;
            }
            self.attributes(None)?;
            let next = self.cursor.peek();
            let numeric = next.kind == TokenKind::Number
                && (next.raw == "0"
                    || !next.raw.starts_with('0')
                        && next.raw.bytes().all(|byte| byte.is_ascii_digit()));
            let field = if numeric {
                self.cursor.advance()
            } else {
                self.cursor.identifier(Some("record pattern field"))?
            };
            let pattern = if self.cursor.eat(":").is_some() {
                self.record_pattern_depth += 1;
                let result = self.pattern(path, aliases);
                self.record_pattern_depth -= 1;
                result?
            } else {
                if numeric {
                    self.cursor
                        .expect(":", Some("tuple record pattern field"))?;
                }
                SPattern {
                    node: SPatternNode::Bind {
                        name: field.value.clone(),
                    },
                    span: span(&field, &field),
                }
            };
            names.push(if numeric { field.raw } else { field.value });
            patterns.push(pattern);
            if self.cursor.eat(",").is_none() {
                break;
            }
        }
        self.cursor.expect("}", Some("record pattern"))?;
        let range = span(start, self.cursor.peek());
        let Some(record) = record else {
            if !self.origins.collecting {
                return Err(unsupported(
                    "Rust record pattern",
                    "the complete source must own its nominal constructor",
                    range,
                ));
            }
            let mut guessed = self.resolve(segments, path);
            guessed.push(segments.last().cloned().unwrap_or_default());
            return Ok(SPattern {
                node: SPatternNode::Ctor {
                    path: guessed,
                    args: patterns,
                },
                span: range,
            });
        };
        let plan = plan_rust_record_pattern_fields(&record.names, &names, rest.is_some());
        if plan.first().is_some_and(|code| *code < 0.0) {
            return Err(type_error(
                "a Rust record pattern has duplicate, unknown or missing fields",
                range,
            ));
        }
        let args =
            plan.iter()
                .skip(1)
                .map(|position| {
                    if *position < 0.0 {
                        let rest = rest.as_ref().ok_or_else(|| {
                            type_error("invalid Rust record rest position", range)
                        })?;
                        Ok(SPattern {
                            node: SPatternNode::Wild,
                            span: Some(Span::new(rest.start, rest.end)),
                        })
                    } else {
                        let index = position.to_string().parse::<usize>().map_err(|_| {
                            type_error("invalid Rust record source position", range)
                        })?;
                        patterns
                            .get(index)
                            .cloned()
                            .ok_or_else(|| type_error("invalid Rust record source position", range))
                    }
                })
                .collect::<Result<Vec<_>>>()?;
        let mut constructor = vec!["crate".to_owned()];
        constructor.extend(record.constructor.split('.').map(str::to_owned));
        Ok(SPattern {
            node: SPatternNode::Ctor {
                path: constructor,
                args,
            },
            span: range,
        })
    }
}
