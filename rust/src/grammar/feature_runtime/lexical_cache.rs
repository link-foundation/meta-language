//! Fixed lexical matching cache, with classification generated from JavaScript.
use super::executor::{Executor, FRAME_LIMIT, Run};
use super::operations::{Abort, State};
use super::program::Expr;
use super::results::Res;
use crate::translation::frontend_rules::{
    accept_lexical_cache_context, read_lexical_cache_expression_form,
};

impl Executor<'_> {
    pub(super) fn accept_fixed_lexical_expression(&mut self, expression: &Expr) -> bool {
        let mut pending = vec![(expression, false)];
        while let Some((item, finished)) = pending.pop() {
            let identity = std::ptr::from_ref(item) as usize;
            if self.lexical_cache_forms.contains_key(&identity) {
                continue;
            }
            // Loaded Rust expressions combine source forms with identical shapes.
            // Only the translated classifier decides which shapes are cacheable.
            let kind = match item {
                Expr::Empty => "empty",
                Expr::LexicalScope { .. } => "lexicalScope",
                Expr::LexicalToken { .. } => "lexicalToken",
                Expr::Terminal { .. } => "literal",
                Expr::Seq(_) => "seq",
                Expr::Choice { .. } => "choice",
                Expr::Longest(_) => "longest",
                Expr::Repeat { .. } => "repeat",
                Expr::And(_) => "and",
                Expr::Not(_) => "not",
                Expr::Capture { .. } => "capture",
                Expr::Alias { .. } => "alias",
                Expr::Precedence { .. } => "precedence",
                Expr::DynamicPrecedence { .. } => "dynamicPrecedence",
                Expr::LexicalPrecedence { .. } => "lexicalPrecedence",
                Expr::Token(_) => "token",
                Expr::ImmediateToken(_) => "immediateToken",
                Expr::Ref(_) => "ref",
                Expr::Predicate { .. } => "predicate",
                Expr::Recover { .. } => "recover",
                Expr::Missing { .. } => "missing",
                Expr::Embed { .. } => "embed",
            };
            let form = read_lexical_cache_expression_form(kind);
            if form <= 1.0 {
                self.lexical_cache_forms.insert(identity, form > 0.0);
                continue;
            }
            let children: Vec<&Expr> = match item {
                Expr::Seq(items) | Expr::Choice { items, .. } | Expr::Longest(items) => {
                    items.iter().collect()
                }
                Expr::Repeat { item, .. }
                | Expr::Capture { item, .. }
                | Expr::Alias { item, .. }
                | Expr::Precedence { item, .. }
                | Expr::DynamicPrecedence { item, .. }
                | Expr::LexicalPrecedence { item, .. }
                | Expr::And(item)
                | Expr::Not(item)
                | Expr::Token(item)
                | Expr::ImmediateToken(item) => vec![item],
                _ => Vec::new(),
            };
            if finished {
                let allowed = children.iter().all(|child| {
                    self.lexical_cache_forms
                        .get(&(std::ptr::from_ref(*child) as usize))
                        .copied()
                        .unwrap_or(false)
                });
                self.lexical_cache_forms.insert(identity, allowed);
            } else {
                pending.push((item, true));
                pending.extend(children.into_iter().map(|child| (child, false)));
            }
        }
        self.lexical_cache_forms
            .get(&(std::ptr::from_ref(expression) as usize))
            .copied()
            .unwrap_or(false)
    }

    /// Every result of `expr` at `position` in `state`.
    pub(super) fn evaluate(
        &mut self,
        expr: &Expr,
        position: usize,
        state: &State,
        in_token: bool,
    ) -> Run<Vec<Res>> {
        // Quiet token context has no trivia, syntax expectations or diagnostic
        // effects. References, scanners and operations are never cacheable.
        let key = (accept_lexical_cache_context(
            in_token,
            self.suppressed > 0,
            self.repair_points.is_some(),
        ) && self.accept_fixed_lexical_expression(expr))
        .then(|| (std::ptr::from_ref(expr) as usize, position, state.clone()));
        if let Some(cached) = key
            .as_ref()
            .and_then(|key| self.lexical_cache.entries.borrow().get(key).cloned())
        {
            return Ok(cached);
        }
        self.step()?;
        let frames = self.shared.frames.get() + 1;
        if frames > FRAME_LIMIT {
            return Err(Abort::NestingTooDeep);
        }
        self.shared.frames.set(frames);
        let results = self.evaluate_case(expr, position, state, in_token);
        self.shared.frames.set(frames - 1);
        let results = results?;
        let cache_has_capacity = self.lexical_cache.entries.borrow().len() < self.shared.memo_limit;
        if let Some(key) = key
            && cache_has_capacity
            && self.retain_lexical_cache(1 + results.len())
        {
            self.lexical_cache
                .entries
                .borrow_mut()
                .insert(key, results.clone());
        }
        Ok(results)
    }
}
