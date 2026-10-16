//! Capture existing parser tokens as source-local declaration facts.

use super::lexical::Section;
use super::{GrammarImportError, Lexer, Parser, TokenKind, parse_options};

impl Parser {
    pub(super) fn parse_header(&mut self) -> Result<bool, GrammarImportError> {
        let kind = if self.check_keyword("grammar") {
            self.advance();
            "combined"
        } else if (self.check_keyword("lexer") || self.check_keyword("parser"))
            && self.check_next_keyword("grammar")
        {
            let kind = if self.check_keyword("lexer") {
                "lexer"
            } else {
                "parser"
            };
            self.advance();
            self.advance();
            kind
        } else {
            return Ok(false);
        };
        let name = self.expect_ident("grammar name")?;
        self.expect_semicolon()?;
        self.pending_comments.clear();
        self.case_insensitive = false;
        self.mode = None;
        self.section = Some(self.sections.len());
        self.sections.push(Section {
            name,
            kind: kind.to_owned(),
            token_vocabulary: String::new(),
            channels: Vec::new(),
            types: Vec::new(),
            modes: vec!["default".to_owned()],
            imports: Vec::new(),
            unsupported_options: Vec::new(),
        });
        Ok(true)
    }

    pub(super) fn parse_skipped_directive(&mut self) -> Result<bool, GrammarImportError> {
        if self.check_any_keyword(&["options", "tokens", "channels"]) {
            let keyword = self.advance().text();
            if let Some(TokenKind::Action(options)) = self.peek_kind() {
                let options = options.clone();
                if keyword == "options" {
                    let parsed = parse_options(&options);
                    self.case_insensitive = parsed
                        .iter()
                        .rev()
                        .find(|(key, _)| key == "caseInsensitive")
                        .is_some_and(|(_, value)| value == "true");
                    if let Some(section) = self.section {
                        self.sections[section].token_vocabulary = parsed
                            .iter()
                            .rev()
                            .find(|(key, _)| key == "tokenVocab")
                            .map_or_else(String::new, |(_, value)| value.clone());
                        if parsed
                            .iter()
                            .any(|(key, _)| key != "caseInsensitive" && key != "tokenVocab")
                        {
                            self.sections[section]
                                .unsupported_options
                                .push(options.clone());
                        }
                    }
                } else if let Some(section) = self.section {
                    let names = Lexer::new(&options)
                        .tokenize()?
                        .into_iter()
                        .filter_map(|token| match token.kind {
                            TokenKind::Ident(name) => Some(name),
                            _ => None,
                        });
                    if keyword == "channels" {
                        self.sections[section].channels.extend(names);
                    } else {
                        self.sections[section].types.extend(names);
                    }
                }
                self.advance();
                self.try_consume_semicolon();
                self.pending_comments.clear();
                return Ok(true);
            }
            self.skip_until_semicolon()?;
            self.pending_comments.clear();
            return Ok(true);
        }
        if self.check_keyword("mode") {
            self.advance();
            let mode = self.expect_ident("lexer mode")?;
            if let Some(section) = self.section
                && !self.sections[section].modes.contains(&mode)
            {
                self.sections[section].modes.push(mode.clone());
            }
            self.mode = Some(mode);
            if !self.try_consume_semicolon() {
                if self.is_end() {
                    return Err(self.expected("';' after directive"));
                }
                self.expect_semicolon()?;
            }
            self.pending_comments.clear();
            return Ok(true);
        }
        if self.check_keyword("import") {
            self.advance();
            let mut imported = Vec::new();
            while !self.is_end() && !matches!(self.peek_kind(), Some(TokenKind::Semicolon)) {
                imported.push(self.advance().text());
            }
            if let Some(section) = self.section {
                self.sections[section].imports.push(imported.join(" "));
            }
            self.skip_until_semicolon()?;
            self.pending_comments.clear();
            return Ok(true);
        }
        Ok(false)
    }
}
