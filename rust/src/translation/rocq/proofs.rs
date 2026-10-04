//! Theorems, proof blocks, bullets and tactics.

use super::{
    Bullet, Done, Language, ProofTypes, Result, RocqParser, SProof, SRule, SSplit, SSplitCase,
    SStep, STheorem, Token, TokenKind, collect_forall_types, describe, js_trim, span, unsupported,
};

impl RocqParser {
    pub(super) fn theorem(&mut self) -> Result<STheorem> {
        let start = self.cursor.advance();
        let keyword = start.value.clone();
        let name = self.identifier(&keyword)?.value;
        let binders = self.binders()?;
        self.expect(":", &keyword)?;
        let prop = self.prop()?;
        self.end_sentence(&keyword)?;
        let proof_start = self.peek().clone();
        if self.cursor.eat("Proof").is_none() {
            return Err(unsupported(
                "proof term",
                "only tactic proofs are reconstructed",
                Some(span(&proof_start, &proof_start)),
            ));
        }
        self.end_sentence("Proof")?;
        let body_start = self.peek().start;
        let mut types = ProofTypes::new();
        for binder in &binders {
            if let Some(rocq_type) = &binder.rocq_type {
                types.insert(binder.name.clone(), rocq_type.clone());
            }
        }
        collect_forall_types(&prop, &mut types);
        let steps = self.proof_block(&types, &|parser: &Self| {
            parser.is("Qed") || parser.is("Defined") || parser.is("Admitted") || parser.is("Abort")
        })?;
        let end = self.peek().clone();
        if self.is("Admitted") || self.is("Abort") {
            return Err(unsupported(
                &format!("Rocq {}", end.value),
                "incomplete proofs cannot be translated",
                Some(span(&end, &end)),
            ));
        }
        self.cursor.advance();
        self.end_sentence(&end.value)?;
        let proof = SProof {
            steps,
            source: Some(js_trim(&self.source, body_start, end.start)),
            source_language: Language::Rocq,
        };
        Ok(STheorem {
            name,
            binders,
            prop,
            proof,
            span: Some(span(&start, self.peek())),
        })
    }

    /// A tactic script: sentences, where a sentence that splits a goal is
    /// followed either by `;` tactics for every subgoal or by one bullet or
    /// brace block per subgoal.
    pub(super) fn proof_block(&mut self, types: &ProofTypes, done: Done<'_>) -> Result<Vec<SStep>> {
        let mut steps = Vec::new();
        while !self.cursor.at_end() && !done(self) {
            let mut sentence = self.tactic_chain(types)?;
            self.end_sentence("tactic")?;
            let split = sentence
                .iter()
                .position(|step| matches!(step, SStep::Induction(_) | SStep::Cases(_)));
            let Some(split) = split else {
                steps.extend(sentence);
                continue;
            };
            if split == sentence.len() - 1 && !done(self) {
                // One block per subgoal, in constructor order.
                let blocks = self.subgoal_blocks(types, done)?;
                if let SStep::Induction(step) | SStep::Cases(step) = &mut sentence[split] {
                    let count = blocks.len();
                    let mut blocks = blocks.into_iter();
                    for case in &mut step.cases {
                        case.steps = blocks.next().unwrap_or_default();
                    }
                    let first = step.cases.len();
                    for (offset, block) in blocks.enumerate() {
                        step.cases.push(SSplitCase {
                            ctor: None,
                            index: Some(first + offset),
                            binds: Vec::new(),
                            steps: block,
                        });
                    }
                    step.blocks = Some(count);
                }
            }
            steps.extend(sentence);
            if !done(self) {
                return Err(unsupported(
                    "tactics after a case split",
                    "every subgoal must be closed inside its bullet or brace block",
                    Some(span(self.peek(), self.peek())),
                ));
            }
        }
        Ok(steps)
    }

    pub(super) fn subgoal_blocks(
        &mut self,
        types: &ProofTypes,
        done: Done<'_>,
    ) -> Result<Vec<Vec<SStep>>> {
        let mut blocks = Vec::new();
        if let Some(bullet) = self.bullet() {
            while self.bullet().is_some_and(|next| next.text == bullet.text) {
                self.consume_bullet();
                let nested = |parser: &Self| {
                    done(parser)
                        || parser.is("}")
                        || parser.bullet().is_some_and(|next| {
                            next.text == bullet.text || next.depth < bullet.depth
                        })
                };
                blocks.push(self.proof_block(types, &nested)?);
            }
            return Ok(blocks);
        }
        if self.is("{") {
            while self.cursor.eat("{").is_some() {
                blocks.push(self.proof_block(types, &|parser: &Self| parser.is("}"))?);
                self.expect("}", "subgoal block")?;
            }
            return Ok(blocks);
        }
        Err(unsupported(
            "unstructured subgoals",
            "close each subgoal of a case split inside a bullet or brace block, or with ;",
            Some(span(self.peek(), self.peek())),
        ))
    }

    /// A bullet is a run of adjacent `-`, `+` or `*` at the start of a sentence.
    pub(super) fn bullet(&self) -> Option<Bullet> {
        let first = self.peek();
        if first.kind != TokenKind::Punct || !["-", "+", "*"].contains(&first.value.as_str()) {
            return None;
        }
        let mut text = first.value.clone();
        let mut offset = 1;
        loop {
            let token = self.cursor.peek_at(offset);
            if token.kind != TokenKind::Punct
                || token.value != first.value
                || token.start != self.cursor.peek_at(offset - 1).end
            {
                break;
            }
            text.push_str(&first.value);
            offset += 1;
        }
        let depth = text.encode_utf16().count();
        Some(Bullet {
            text,
            length: offset,
            depth,
        })
    }

    pub(super) fn consume_bullet(&mut self) {
        let length = self.bullet().map_or(0, |bullet| bullet.length);
        for _ in 0..length {
            self.cursor.advance();
        }
    }

    pub(super) fn tactic_chain(&mut self, types: &ProofTypes) -> Result<Vec<SStep>> {
        let mut steps = self.tactic(types)?;
        while self.cursor.eat(";").is_some() {
            steps.extend(self.tactic(types)?);
        }
        Ok(steps)
    }

    pub(super) fn no_target(&self, token: &Token, value: &str) -> Result<()> {
        if self.is("in") || self.is("at") {
            return Err(unsupported(
                &format!("{value} {}", self.peek().value),
                "hypothesis rewriting is outside the portable proof model",
                Some(span(token, self.peek())),
            ));
        }
        Ok(())
    }

    #[allow(clippy::too_many_lines)] // one arm per tactic, as in the JavaScript runtime
    pub(super) fn tactic(&mut self, types: &ProofTypes) -> Result<Vec<SStep>> {
        let token = self.peek().clone();
        if token.kind != TokenKind::Identifier {
            return Err(self.fail_here("expected a tactic"));
        }
        self.cursor.advance();
        let value = token.value.clone();
        match value.as_str() {
            "reflexivity" | "vm_compute" | "native_compute" | "compute" | "cbv" | "lazy"
            | "trivial" | "easy" | "auto" | "congruence" | "discriminate" => {
                self.no_target(&token, &value)?;
                Ok(vec![SStep::Compute {
                    tactic: Some(value),
                }])
            }
            "lia" | "nia" | "omega" | "ring" => Ok(vec![SStep::Arith {
                tactic: Some(value),
            }]),
            "intro" | "intros" => {
                let mut names = Vec::new();
                while self.cursor.is_kind(TokenKind::Identifier) {
                    names.push(self.cursor.advance().value);
                }
                Ok(vec![SStep::Intro { names }])
            }
            "simpl" | "cbn" => {
                let mut names = Vec::new();
                if self.cursor.eat("[").is_some() {
                    while !self.is("]") {
                        names.push(self.identifier(&value)?.value);
                    }
                    self.expect("]", &value)?;
                } else {
                    while self.cursor.is_kind(TokenKind::Identifier)
                        && !self.is("in")
                        && !self.is("at")
                    {
                        names.push(self.cursor.advance().value);
                    }
                }
                self.no_target(&token, &value)?;
                if names.is_empty() {
                    return Ok(vec![SStep::Simp {
                        rules: Vec::new(),
                        only: Some(false),
                        tactic: Some(value),
                    }]);
                }
                Ok(vec![SStep::Unfold {
                    names,
                    tactic: Some(value),
                }])
            }
            "unfold" => {
                let mut names = vec![self.identifier("unfold")?.value];
                while self.cursor.eat(",").is_some() {
                    names.push(self.identifier("unfold")?.value);
                }
                self.no_target(&token, &value)?;
                Ok(vec![SStep::Unfold {
                    names,
                    tactic: None,
                }])
            }
            "rewrite" => {
                let mut rules = Vec::new();
                loop {
                    // The Rocq lexer reads `<-` as `<` then `-`, so this never matches.
                    let reverse = self.cursor.eat("<-").is_some();
                    self.cursor.eat("?");
                    self.cursor.eat("!");
                    rules.push(SRule {
                        name: self.identifier("rewrite rule")?.value,
                        reverse,
                    });
                    if self.cursor.eat(",").is_none() {
                        break;
                    }
                }
                self.no_target(&token, &value)?;
                Ok(vec![SStep::Rewrite {
                    rules,
                    only: None,
                    tactic: Some(value),
                }])
            }
            "now" => {
                let mut inner = self.tactic(types)?;
                inner.push(SStep::Compute {
                    tactic: Some("easy".to_owned()),
                });
                Ok(inner)
            }
            "induction" | "destruct" => {
                let variable = self.identifier(&value)?.value;
                let mut names = None;
                let mut using = None;
                loop {
                    if self.cursor.eat("as").is_some() {
                        names = Some(self.intro_pattern()?);
                        continue;
                    }
                    if self.cursor.eat("using").is_some() {
                        using = Some(self.identifier("using")?.value);
                        continue;
                    }
                    break;
                }
                let rocq_type = types.get(&variable).map(String::as_str);
                let peano = using.as_deref() == Some("N.peano_ind");
                let here = span(&token, self.peek());
                if rocq_type == Some("N") && value == "induction" && !peano {
                    return Err(unsupported(
                        "binary induction on N",
                        "N is split as zero and successor only with `using N.peano_ind`",
                        Some(here),
                    ));
                }
                if rocq_type == Some("N") && value == "destruct" {
                    return Err(unsupported(
                        "binary case analysis on N",
                        "destructing N yields N0 and Npos, which have no portable counterpart",
                        Some(here),
                    ));
                }
                if rocq_type == Some("Z") {
                    return Err(unsupported(
                        &format!("{value} on Z"),
                        "integers are not split in the portable proof model",
                        Some(here),
                    ));
                }
                if let Some(using) = using.as_deref().filter(|_| !peano) {
                    // An empty `using` name is falsy in JavaScript, but the
                    // lexer never produces an empty identifier.
                    return Err(unsupported(
                        &format!("induction using {using}"),
                        "custom induction principles are outside the portable proof model",
                        Some(here),
                    ));
                }
                let cases = names
                    .unwrap_or_default()
                    .into_iter()
                    .enumerate()
                    .map(|(index, binds)| SSplitCase {
                        ctor: None,
                        index: Some(index),
                        binds: binds.into_iter().map(Some).collect(),
                        steps: Vec::new(),
                    })
                    .collect();
                let split = SSplit {
                    variable,
                    cases,
                    positional: true,
                    blocks: None,
                };
                Ok(vec![if value == "induction" {
                    SStep::Induction(split)
                } else {
                    SStep::Cases(split)
                }])
            }
            _ => Err(unsupported(
                &format!("Rocq tactic {value}"),
                "outside the portable proof model",
                Some(span(&token, &token)),
            )),
        }
    }

    /// `[ | k ih ]`: one list of names per constructor, in declaration order.
    pub(super) fn intro_pattern(&mut self) -> Result<Vec<Vec<String>>> {
        self.expect("[", "intro pattern")?;
        let mut alternatives = vec![Vec::new()];
        while !self.is("]") {
            if self.cursor.eat("|").is_some() {
                alternatives.push(Vec::new());
                continue;
            }
            let last = alternatives.len() - 1;
            if self.cursor.eat("_").is_some() {
                alternatives[last].push("_".to_owned());
                continue;
            }
            let token = self.peek();
            if token.kind != TokenKind::Identifier {
                return Err(unsupported(
                    "intro pattern",
                    &format!("{} is outside the portable proof model", describe(token)),
                    Some(span(token, token)),
                ));
            }
            alternatives[last].push(self.cursor.advance().value);
        }
        self.expect("]", "intro pattern")?;
        Ok(alternatives)
    }
}
