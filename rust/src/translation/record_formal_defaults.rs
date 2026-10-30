//! Actual source field/default transport; the ordinary default checker owns IR.
use super::javascript_record_parameter_rules as rules;
use super::surface::{SArrayItem, SExpr, SNode};
use super::{Result, Span};
pub(super) fn operands<'a>(
    args: &'a [SArrayItem],
    span: Option<Span>,
    language: &str,
) -> Result<(SExpr, &'a SExpr)> {
    let count = serde_json::Number::from(args.len())
        .as_f64()
        .expect("actual operand count has a Number representation");
    if !rules::accept_java_script_record_formal_default_source(
        &rules::ml_string::Text::from_text(language),
        count,
        args.iter().any(|argument| argument.spread),
        &rules::ml_string::Text::from_text(
            args.get(1)
                .map_or("", |argument| argument.value.node.kind()),
        ),
    ) {
        return Err(super::diagnostics::type_error(
            "a record formal default needs its exact receiver, literal property and original initializer",
            span,
        ));
    }
    let SNode::Str { value: field } = &args[1].value.node else {
        unreachable!("source profile admits an actual literal property")
    };
    Ok((
        SExpr::new(
            SNode::Field {
                object: Box::new(args[0].value.clone()),
                field: field.clone(),
            },
            span,
        ),
        &args[2].value,
    ))
}
