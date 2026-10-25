//! Original lexical declarations through the complete shared recursive plan.
use super::recursive_object_bindings::{binding_positions, index};
use super::{JavaScriptParser, Result, SExpr, SNode, Span, Stmt, Token, unsupported};
use crate::translation::javascript_record_parameter_rules::{self as rules, ml_string::Text};
pub(super) struct ObjectBindingDeclaration {
    pub(super) statements: Vec<Stmt>,
    pub(super) bindings: Vec<Token>,
    pub(super) span: Span,
}
impl JavaScriptParser {
    pub(super) fn object_binding_names(&self, start: usize) -> Vec<Token> {
        let syntax = self.recursive_object_plan(start);
        binding_positions(&syntax.plan)
            .iter()
            .map(|at| syntax.tokens[index(*at)].clone())
            .collect()
    }
    pub(super) fn object_binding_declaration(
        &mut self,
        start_token: &Token,
        mode: &str,
    ) -> Result<ObjectBindingDeclaration> {
        let start = self.cursor.index;
        let syntax = self.recursive_object_plan(start);
        self.require_recursive_object_plan(&syntax, start_token)?;
        let statement =
            rules::read_java_script_object_binding_statement(&Text::from_text(mode)).to_string();
        if statement.is_empty() {
            return Err(unsupported(
                "object binding transport",
                "the declaration needs its complete actual object pattern and lexical mode",
                Some(self.cursor.peek().span()),
            ));
        }
        let positions = binding_positions(&syntax.plan);
        let bindings = positions
            .iter()
            .map(|at| syntax.tokens[index(*at)].clone())
            .collect::<Vec<_>>();
        for token in &bindings {
            crate::translation::lexer::require_java_script_identifier_role(token, true)?;
            self.lexical.declaration(&token.value, token.span(), 1);
            self.scope.tdz.insert(token.value.clone());
        }
        self.cursor.index = start + index(syntax.plan[0]);
        self.cursor.expect("=", Some("destructuring"))?;
        let value = self.expr()?;
        let whole = self.to_here(start_token);
        self.generated_count += 1;
        let name = format!(
            "{}temporary_binding{}",
            self.generated_prefix, self.generated_count
        );
        let object = SExpr::new(
            SNode::Name {
                path: vec![name.clone()],
            },
            Some(whole),
        );
        let entries = self.recursive_object_entries(
            &syntax,
            object,
            &positions,
            "ordinary object binding declaration",
            "object binding initializer",
            false,
        )?;
        let mut statements = vec![Stmt::Const {
            name,
            value,
            span: whole,
        }];
        statements.extend(entries.into_iter().map(|entry| {
            if !entry.temporary && statement == "let" {
                Stmt::Let {
                    name: entry.name,
                    value: entry.value,
                    span: whole,
                }
            } else {
                Stmt::Const {
                    name: entry.name,
                    value: entry.value,
                    span: whole,
                }
            }
        }));
        for token in &bindings {
            self.declare_local(&token.value, mode == "let");
            self.lexical
                .binding(&token.value, Some(self.cursor.peek().start));
        }
        Ok(ObjectBindingDeclaration {
            statements,
            bindings,
            span: whole,
        })
    }
}
