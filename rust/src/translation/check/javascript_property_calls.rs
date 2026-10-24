//! Actual type/child transport into existing checked strings and callable fields.
use super::{Checker, Env, Expr, Language, Result, SExpr, SNode, Span, unsupported};
use crate::translation::javascript_property_call_rules as rules;

impl Checker {
    pub(super) fn source_property_call(
        &mut self,
        source: &SExpr,
        operation: &str,
        args: &[Option<SExpr>],
        env: &Env,
        path: &[String],
        span: Option<Span>,
    ) -> Result<Option<Expr>> {
        if self.language == Language::JavaScript && operation == "concat" {
            return self.source_concatenation(source, args, env, path, span);
        }
        if self.language != Language::JavaScript
            || rules::read_java_script_source_property_operation_kind(operation).is_empty()
        {
            return Ok(None);
        }
        let receiver = self.expr(source, env, path, None, false)?;
        let object = Self::reference_value(receiver);
        let form = rules::read_java_script_resolved_property_call_form(object.ty.kind(), operation);
        let node = match form.as_str() {
            "record" | "" => return Ok(None),
            "range" => {
                return self
                    .javascript_string_range(operation, object, args, span, env, path)
                    .map(Some);
            }
            "test" | "index" => {
                return self
                    .javascript_string_search(operation, object, args, span, env, path)
                    .map(Some);
            }
            "map" => {
                return self
                    .javascript_string_map_call(operation, object, args, span, env, path)
                    .map(Some);
            }
            "text" => {
                if !args.is_empty() {
                    return Err(unsupported(
                        "toString with a radix",
                        "only decimal text is portable",
                        span,
                    ));
                }
                SNode::ToString {
                    arg: Box::new(source.clone()),
                }
            }
            _ => unreachable!("the shared source property form is exhaustive"),
        };
        self.expr(&SExpr::new(node, span), env, path, None, false)
            .map(Some)
    }
}
