//! Actual source scopes and pattern rows; no new checked term or runtime.
use super::{
    Options, Result, RustParser, SExpr, SNode, SPattern, SPatternNode, SRow, SourceOrigin, Token,
    span, unsupported,
};

impl RustParser {
    pub(super) fn conditional_pattern_value(
        &mut self,
        path: &[String],
        start: &Token,
    ) -> Result<SExpr> {
        self.cursor.expect("let", Some("conditional pattern"))?;
        let mut aliases = Vec::new();
        let mut alternatives = vec![self.pattern(path, &mut aliases)?];
        while self.cursor.eat("|").is_some() {
            alternatives.push(self.pattern(path, &mut aliases)?);
        }
        if alternatives.len() > 1 && !aliases.is_empty() {
            return Err(unsupported(
                "binding in an or-pattern",
                "bind the value in each alternative separately",
                span(start, self.cursor.peek()),
            ));
        }
        self.cursor.expect("=", Some("conditional pattern"))?;
        let scrutinee = self.comparison(
            path,
            Options {
                no_struct: true,
                statement: false,
            },
        )?;
        if self.cursor.is("&&") || self.cursor.is("||") {
            return Err(unsupported(
                "Rust conditional pattern chain",
                "chained conditions require their complete short-circuit scope interpretation",
                span(start, self.cursor.peek()),
            ));
        }
        let scope = self.origins.cases(&alternatives, &scrutinee, path)?;
        let previous = self.origins.enter(Some(scope));
        let result = (|| -> Result<(SExpr, SourceOrigin)> {
            for alias in &aliases {
                self.origins.bind(&alias.name, &alias.value, path, None)?;
            }
            let body = self.block(path)?;
            let origin = self.origins.origin(&body, path);
            Ok((body, origin))
        })();
        self.origins.scope = previous;
        let (mut body, origin) = result?;
        for alias in aliases.into_iter().rev() {
            let range = body.span;
            body = SExpr::new(
                SNode::Let {
                    name: alias.name,
                    ty: None,
                    value: Box::new(alias.value),
                    body: Box::new(body),
                },
                range,
            );
        }
        if self.cursor.eat("else").is_none() {
            return Err(unsupported(
                "if without else",
                "the value of an if must be defined on both branches",
                span(start, self.cursor.peek()),
            ));
        }
        let otherwise = if self.cursor.is("if") {
            self.if_expr(path)?
        } else {
            self.block(path)?
        };
        let result_origin = self
            .origins
            .checked_join(&[origin, self.origins.origin(&otherwise, path)])?;
        let mut rows = alternatives
            .into_iter()
            .map(|pattern| SRow {
                patterns: vec![pattern],
                body: body.clone(),
                span: None,
            })
            .collect::<Vec<_>>();
        rows.push(SRow {
            patterns: vec![SPattern {
                node: SPatternNode::Wild,
                span: None,
            }],
            body: otherwise,
            span: None,
        });
        let result = SExpr::new(
            SNode::Match {
                scrutinees: vec![scrutinee],
                rows,
            },
            span(start, self.cursor.peek()),
        );
        self.origins.mark(&result, result_origin);
        Ok(result)
    }
}
