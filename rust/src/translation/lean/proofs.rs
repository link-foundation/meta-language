//! Theorems, tactic scripts, and propositions of the Lean frontend.

use super::{binder, column, connective, js_trimmed, prop_relation, span, LeanParser};
use crate::translation::diagnostics::{unsupported, Result};
use crate::translation::lexer::{Token, TokenKind};
use crate::translation::surface::{
    SComparison, SProof, SProp, SPropNode, SRule, SSplit, SSplitCase, SStep, STheorem,
};
use crate::translation::Language;

impl LeanParser<'_> {
    pub(super) fn theorem(&mut self) -> Result<STheorem> {
        let start = self.cursor.advance();
        let name = self.cursor.identifier(Some("theorem"))?.value;
        let binders = self
            .binders()?
            .into_iter()
            .map(|(name, ty)| binder(name, ty))
            .collect();
        self.cursor.expect(":", Some("theorem"))?;
        let prop = self.with_bound(0, Self::prop)?;
        self.cursor.expect(":=", Some("theorem"))?;
        let proof_start = self.peek();
        let steps = if self.cursor.eat("by").is_some() {
            self.tactics(&proof_start)?
        } else if self.cursor.is("rfl") {
            self.cursor.advance();
            vec![SStep::Compute { tactic: None }]
        } else {
            return Err(unsupported(
                "proof term",
                "only tactic proofs are reconstructed",
                Some(span(&proof_start, &proof_start)),
            ));
        };
        let next = self.cursor.peek();
        let end = if next.kind == TokenKind::Eof {
            self.source.len()
        } else {
            next.start
        };
        let units = self.source.units();
        let proof_source = js_trimmed(&units[proof_start.start.min(end)..end]);
        let proof = SProof {
            steps,
            source: Some(proof_source),
            source_language: Language::Lean,
        };
        self.end_decl(&start, "termination_by")?;
        Ok(crate::translation::surface::STheorem {
            name,
            binders,
            prop,
            proof,
            span: Some(self.span_to_next(&start)),
        })
    }

    // Tactic blocks keep their structure: intro, induction with per-case
    // steps, and closing steps. The emitters rebuild each case for the target
    // kernel, which then re-checks the proof.
    pub(super) fn tactics(&mut self, by_token: &Token) -> Result<Vec<SStep>> {
        let first = self.cursor.peek();
        let block_column = if first.first { column(first) } else { -1 };
        let mut steps = Vec::new();
        if block_column < 0 {
            self.with_bound(column(by_token), |parser| {
                parser.tactic_sequence(&mut steps)
            })?;
            return Ok(steps);
        }
        while !self.cursor.at_end()
            && self.cursor.peek().first
            && column(self.cursor.peek()) == block_column
        {
            self.with_bound(block_column, |parser| parser.tactic_sequence(&mut steps))?;
        }
        Ok(steps)
    }

    /// Tactics joined by `;` or `<;>`.
    pub(super) fn tactic_sequence(&mut self, steps: &mut Vec<SStep>) -> Result<()> {
        loop {
            steps.extend(self.tactic()?);
            if self.cursor.eat(";").is_some() || self.cursor.eat("<;>").is_some() {
                continue;
            }
            return Ok(());
        }
    }

    pub(super) fn names_until_blocked(&mut self) -> Vec<String> {
        let mut names = Vec::new();
        while self.cursor.is_kind(TokenKind::Identifier) && !self.blocked_next() {
            names.push(self.cursor.advance().value);
        }
        names
    }

    pub(super) fn tactic(&mut self) -> Result<Vec<SStep>> {
        let token = self.peek();
        let value = token.value.as_str();
        if token.kind != TokenKind::Identifier {
            return Err(self.fail("expected a tactic"));
        }
        self.cursor.advance();
        match value {
            "rfl" | "decide" | "trivial" => Ok(vec![SStep::Compute {
                tactic: Some(value.to_owned()),
            }]),
            "omega" => Ok(vec![SStep::Arith {
                tactic: Some(value.to_owned()),
            }]),
            "intro" | "intros" => Ok(vec![SStep::Intro {
                names: self.names_until_blocked(),
            }]),
            "unfold" => Ok(vec![SStep::Unfold {
                names: self.names_until_blocked(),
                tactic: None,
            }]),
            "rw" | "simp" | "simp_all" => {
                let only = value != "rw" && self.cursor.eat("only").is_some();
                let mut rules = Vec::new();
                if self.cursor.eat("[").is_some() {
                    while !self.cursor.is("]") {
                        let reverse =
                            self.cursor.eat("←").is_some() || self.cursor.eat("<-").is_some();
                        rules.push(SRule {
                            name: self.cursor.identifier(Some("rewrite rule"))?.value,
                            reverse,
                        });
                        if self.cursor.eat(",").is_none() {
                            break;
                        }
                    }
                    self.cursor.expect("]", Some(value))?;
                }
                if self.cursor.is("at") {
                    return Err(unsupported(
                        &format!("{value} at"),
                        "hypothesis rewriting is outside the portable proof model",
                        Some(self.span_to_next(&token)),
                    ));
                }
                let (only, tactic) = (Some(only), Some(value.to_owned()));
                Ok(vec![if value == "rw" {
                    SStep::Rewrite {
                        rules,
                        only,
                        tactic,
                    }
                } else {
                    SStep::Simp {
                        rules,
                        only,
                        tactic,
                    }
                }])
            }
            "induction" | "cases" => {
                let variable = self.cursor.identifier(Some(value))?.value;
                self.cursor.expect("with", Some(value))?;
                let mut cases = Vec::new();
                while self.cursor.is("|") && !self.outdented(column(&token)) {
                    let bar = self.cursor.advance();
                    let ctor = self.cursor.identifier(Some("case"))?.value;
                    let ctor = ctor.strip_prefix('.').unwrap_or(&ctor).to_owned();
                    let mut binds = Vec::new();
                    while self.cursor.is_kind(TokenKind::Identifier) && !self.cursor.is("=>") {
                        binds.push(Some(self.cursor.advance().value));
                    }
                    self.cursor.expect("=>", Some("case"))?;
                    let steps = self.case_tactics(&bar)?;
                    cases.push(SSplitCase {
                        ctor: Some(ctor),
                        index: None,
                        binds,
                        steps,
                    });
                }
                let split = SSplit {
                    variable,
                    cases,
                    positional: false,
                    blocks: None,
                };
                Ok(vec![if value == "induction" {
                    SStep::Induction(split)
                } else {
                    SStep::Cases(split)
                }])
            }
            // `exact`, `apply`, `constructor`, `exists`, `calc`, `have`, `show`,
            // `sorry`, `admit`, `native_decide`, `grind`, `aesop` and the rest.
            _ => Err(unsupported(
                &format!("Lean tactic {value}"),
                "outside the portable proof model",
                Some(span(&token, &token)),
            )),
        }
    }

    pub(super) fn case_tactics(&mut self, bar: &Token) -> Result<Vec<SStep>> {
        let mut steps = Vec::new();
        let first = self.cursor.peek();
        if !first.first {
            self.with_bound(column(bar), |parser| parser.tactic_sequence(&mut steps))?;
            return Ok(steps);
        }
        let case_column = column(first);
        while !self.cursor.at_end()
            && self.cursor.peek().first
            && column(self.cursor.peek()) == case_column
            && case_column > column(bar)
        {
            self.with_bound(case_column, |parser| parser.tactic_sequence(&mut steps))?;
        }
        Ok(steps)
    }

    pub(super) fn prop(&mut self) -> Result<SProp> {
        self.prop_implies()
    }

    pub(super) fn prop_implies(&mut self) -> Result<SProp> {
        let left = self.prop_or()?;
        if self.cursor.eat("→").is_some() || self.cursor.eat("->").is_some() {
            let right = self.prop_implies()?;
            return Ok(connective(SPropNode::Implies {
                left: Box::new(left),
                right: Box::new(right),
            }));
        }
        Ok(left)
    }

    pub(super) fn prop_or(&mut self) -> Result<SProp> {
        let mut left = self.prop_and()?;
        while self.cursor.eat("∨").is_some() {
            let right = self.prop_and()?;
            left = connective(SPropNode::Or {
                left: Box::new(left),
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    pub(super) fn prop_and(&mut self) -> Result<SProp> {
        let mut left = self.prop_unary()?;
        while self.cursor.eat("∧").is_some() {
            let right = self.prop_unary()?;
            left = connective(SPropNode::And {
                left: Box::new(left),
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    pub(super) fn prop_unary(&mut self) -> Result<SProp> {
        let token = self.peek();
        if self.cursor.eat("¬").is_some() {
            let arg = self.prop_unary()?;
            return Ok(connective(SPropNode::Not { arg: Box::new(arg) }));
        }
        if self.cursor.eat("∀").is_some() {
            let mut binders = Vec::new();
            while self.cursor.is("(") {
                binders.extend(self.binder_group()?);
            }
            if binders.is_empty() {
                let mut names = Vec::new();
                while self.cursor.is_kind(TokenKind::Identifier) {
                    names.push(self.cursor.advance().value);
                }
                self.cursor.expect(":", Some("∀"))?;
                let ty = self.ty()?;
                binders.extend(names.into_iter().map(|name| (name, ty.clone())));
            }
            self.cursor.expect(",", Some("∀"))?;
            let body = self.prop()?;
            return Ok(SProp {
                node: SPropNode::Forall {
                    binders: binders
                        .into_iter()
                        .map(|(name, ty)| binder(name, ty))
                        .collect(),
                    body: Box::new(body),
                },
                span: Some(self.span_to_next(&token)),
            });
        }
        if self.cursor.is("(") && self.prop_parenthesised() {
            self.cursor.advance();
            let inner = self.prop()?;
            self.cursor.expect(")", Some("proposition"))?;
            return Ok(inner);
        }
        let left = self.binary(55)?;
        let relation = self.cursor.peek();
        if relation.kind == TokenKind::Punct && !self.blocked(relation) {
            if let Some(build) = prop_relation(&relation.value) {
                self.cursor.advance();
                let right = self.binary(55)?;
                return Ok(SProp {
                    node: build(SComparison {
                        left,
                        right,
                        reference: false,
                    }),
                    span: Some(self.span_to_next(&token)),
                });
            }
        }
        Ok(SProp {
            node: SPropNode::Bool { expr: left },
            span: Some(self.span_to_next(&token)),
        })
    }

    /// A parenthesis opens a proposition when it contains a logical connective at depth one.
    pub(super) fn prop_parenthesised(&self) -> bool {
        let mut depth = 0i64;
        for offset in 0.. {
            let token = self.cursor.peek_at(offset);
            if token.kind == TokenKind::Eof {
                return false;
            }
            if token.value == "(" {
                depth += 1;
            }
            if token.value == ")" {
                depth -= 1;
                if depth == 0 {
                    return false;
                }
            }
            if depth == 1
                && ["∧", "∨", "→", "¬", "∀", "=", "≠", "↔"].contains(&token.value.as_str())
            {
                return true;
            }
        }
        false
    }
}
