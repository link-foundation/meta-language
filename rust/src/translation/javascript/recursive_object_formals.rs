//! Actual synchronous formal slots through the shared recursive source plan.
use super::record_parameters::RecordParameters;
use super::recursive_object_bindings::{RecursiveObjectSyntax, columns, index, reserved_names};
use super::{JavaScriptParser, Result, SExpr, SNode, Span, Token, unsupported};
use crate::translation::recursive_object_binding_rules as rules;
impl JavaScriptParser {
    pub(super) fn read_recursive_object_formals(
        &mut self,
        tokens: &[Token],
        start: usize,
        context: &str,
    ) -> Result<Option<RecordParameters>> {
        let (kinds, values) = columns(tokens);
        let plan = rules::plan_java_script_recursive_object_formals(
            &kinds,
            &values,
            0.0,
            reserved_names(),
            rules::read_java_script_object_binding_depth_limit(),
        );
        if crate::translation::frontend_rules::ml_array::number_index(-plan[0]) == Some(9) {
            return Err(unsupported(
                "object binding depth",
                "the actual recursive pattern exceeds its shared source depth budget",
                Some(tokens[index(plan[1])].span()),
            ));
        }
        if !rules::contains_java_script_recursive_object_formals(&plan) {
            return Ok(None);
        }
        if index(plan[0]) != tokens.len() {
            return Err(unsupported(
                "record parameter source transport",
                "the plan needs the complete actual formal interval and its parser-owned token/name inventory",
                Some(tokens[0].span()),
            ));
        }
        let positions = rules::read_java_script_recursive_object_formal_binding_positions(&plan);
        let names = positions
            .iter()
            .map(|at| tokens[index(*at)].value.clone())
            .collect();
        let mut parameters = Vec::new();
        let mut entries = Vec::new();
        let mut cursor = 2;
        while cursor < plan.len() {
            let form = index(plan[cursor]);
            let slot = index(plan[cursor + 1]);
            cursor += 2;
            if slot != parameters.len() {
                return Err(unsupported(
                    "record parameter source transport",
                    "logical arguments retain their actual ordered slots",
                    Some(tokens[0].span()),
                ));
            }
            if form == 0 {
                let token = &tokens[index(plan[cursor])];
                cursor += 1;
                crate::translation::lexer::require_java_script_identifier_role(token, true)?;
                parameters.push(super::record_parameters::parameter(
                    token.value.clone(),
                    token.span(),
                ));
                continue;
            }
            let length = index(plan[cursor]);
            cursor += 1;
            let tree = plan[cursor..cursor + length].to_vec();
            cursor += length;
            let span = Span::new(tokens[index(tree[8])].start, tokens[index(tree[9])].end);
            let name = self.recursive_object_parameter_name("record_parameter_value", span)?;
            parameters.push(super::record_parameters::parameter(name.clone(), span));
            let root = SExpr::new(SNode::Name { path: vec![name] }, Some(span));
            entries.extend(self.recursive_object_entries(
                &RecursiveObjectSyntax {
                    tokens: tokens.to_vec(),
                    plan: tree,
                    start,
                },
                root,
                &positions,
                context,
                "record formal default",
                true,
            )?);
        }
        self.cursor.index = start + index(plan[0]);
        Ok(Some(RecordParameters::from_recursive_object_formals(
            parameters, names, entries,
        )))
    }
}
