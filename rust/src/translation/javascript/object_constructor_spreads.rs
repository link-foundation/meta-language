//! Checked finite expansion of record literals copied into a tagged value.
use super::{Result, SExpr, SNode, Span, node, type_error, unsupported};
use crate::translation::surface::SRecordEntry;

#[derive(Clone)]
enum Step {
    Field(String, Box<SExpr>),
    Choice {
        condition: Box<SExpr>,
        yes: Vec<Self>,
        no: Vec<Self>,
    },
}

fn refusal(reason: &str, place: Option<Span>) -> crate::translation::diagnostics::TranslationError {
    unsupported("tagged object spread", reason, place)
}

fn check_count(count: u32, place: Option<Span>) -> Result<u32> {
    if f64::from(count) > super::surface_rules::read_java_script_constructor_tag_alternative_limit()
    {
        return Err(refusal(
            "literal record spreads exceed the finite constructor alternative limit",
            place,
        ));
    }
    Ok(count)
}

fn prepare(entries: Vec<SRecordEntry>) -> Result<(Vec<Step>, u32)> {
    let mut count = 1;
    let mut steps = Vec::new();
    for entry in entries {
        let SRecordEntry {
            name,
            spread,
            value,
        } = entry;
        if !spread {
            let name = name.expect("parsed named property");
            if name
                .parse::<u32>()
                .is_ok_and(|index| index < u32::MAX && index.to_string() == name)
            {
                return Err(refusal(
                    "constructor spreads need named fields with a stable declaration order",
                    value.span,
                ));
            }
            steps.push(Step::Field(name, Box::new(value)));
            continue;
        }
        let place = value.span;
        let (spread, alternatives) = prepare_operand(value)?;
        count = check_count(count * alternatives, place)?;
        steps.extend(spread);
    }
    Ok((steps, count))
}

fn prepare_operand(value: SExpr) -> Result<(Vec<Step>, u32)> {
    match value.node {
        SNode::Record {
            entries,
            prototype_definition: None,
            ..
        } => prepare(entries),
        SNode::If {
            cond,
            then,
            otherwise,
        } => {
            let (yes, yes_count) = prepare_operand(*then)?;
            let (no, no_count) = prepare_operand(*otherwise)?;
            let count = check_count(yes_count + no_count, value.span)?;
            Ok((
                vec![Step::Choice {
                    condition: cond,
                    yes,
                    no,
                }],
                count,
            ))
        }
        _ => Err(refusal(
            "constructor spreads require a complete tagged field projection from literal records",
            value.span,
        )),
    }
}

fn lower(
    tag: &str,
    steps: Vec<Step>,
    mut fields: Vec<(String, SExpr)>,
    bound: usize,
    fresh_name: &mut impl FnMut() -> String,
    place: Span,
) -> Result<SExpr> {
    let mut steps = steps.into_iter();
    while let Some(step) = steps.next() {
        let (condition, mut yes, mut no) = match step {
            Step::Field(name, value) => {
                if fields.iter().any(|(field, _)| *field == name) {
                    return Err(type_error(
                        format!("duplicate field {name}"),
                        value.span.or(Some(place)),
                    ));
                }
                fields.push((name, *value));
                continue;
            }
            Step::Choice { condition, yes, no } => (condition, yes, no),
        };
        let bindings = fields
            .iter_mut()
            .skip(bound)
            .map(|(_, value)| {
                let name = fresh_name();
                let reference = SExpr::new(
                    SNode::Name {
                        path: vec![name.clone()],
                    },
                    value.span,
                );
                (name, std::mem::replace(value, reference))
            })
            .collect::<Vec<_>>();
        let next_bound = fields.len();
        let rest = steps.collect::<Vec<_>>();
        yes.extend_from_slice(&rest);
        no.extend(rest);
        let then = lower(tag, yes, fields.clone(), next_bound, fresh_name, place)?;
        let otherwise = lower(tag, no, fields, next_bound, fresh_name, place)?;
        let mut body = node(
            SNode::If {
                cond: condition,
                then: Box::new(then),
                otherwise: Box::new(otherwise),
            },
            place,
        );
        for (name, value) in bindings.into_iter().rev() {
            body = node(
                SNode::Let {
                    name,
                    ty: None,
                    value: Box::new(value),
                    body: Box::new(body),
                },
                place,
            );
        }
        return Ok(body);
    }
    Ok(node(
        SNode::CtorObject {
            tag: tag.to_owned(),
            fields,
            frozen: false,
            prototype_definition: None,
        },
        place,
    ))
}

pub(super) fn expand_constructor_spreads(
    tag: SExpr,
    entries: Vec<SRecordEntry>,
    mut fresh_name: impl FnMut() -> String,
    place: Span,
) -> Result<SExpr> {
    let SNode::Str { value } = tag.node else {
        return Err(refusal(
            "literal record spreads currently require a single literal tag",
            tag.span,
        ));
    };
    lower(
        &value,
        prepare(entries)?.0,
        Vec::new(),
        0,
        &mut fresh_name,
        place,
    )
}
