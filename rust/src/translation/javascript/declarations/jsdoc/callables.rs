//! Materialize the original source slices selected by the generated callable plan.

use super::{HashMap, HashSet, Result, Span, Type, read_jsdoc_type, unsupported};
use crate::translation::callable_source_rules::plan_callable_type_syntax;
use crate::translation::frontend_rules::ml_array::number_index;

fn position(value: f64) -> usize {
    number_index(value).expect("a callable syntax plan retains an exact source position")
}

fn source_slice(units: &[u16], start: f64, end: f64, range: Span) -> Result<String> {
    units
        .get(position(start)..position(end))
        .and_then(|slice| String::from_utf16(slice).ok())
        .ok_or_else(|| {
            unsupported(
                "JSDoc type source boundary",
                "a selected type range must preserve complete UTF16 characters",
                Some(range),
            )
        })
}

pub(super) fn read_callable_type(
    name: &str,
    range: Span,
    aliases: &HashMap<String, String>,
    expanding: &mut HashSet<String>,
) -> Result<Option<Type>> {
    let units = name.encode_utf16().collect::<Vec<_>>();
    let numeric = units.iter().copied().map(f64::from).collect::<Vec<_>>();
    let plan = plan_callable_type_syntax(&numeric);
    let Some(action) = plan.first() else {
        return Ok(None);
    };
    if action.to_bits() == (-1.0_f64).to_bits() {
        return Err(unsupported(
            &format!("JSDoc type {{{name}}}"),
            "a function type needs a return type",
            Some(range),
        ));
    }
    match position(*action) {
        0 => read_jsdoc_type(
            &source_slice(&units, plan[1], plan[2], range)?,
            range,
            aliases,
            expanding,
        )
        .map(Some),
        1 => {
            let mut params = Vec::new();
            let mut at = 4;
            for _ in 0..position(plan[3]) {
                let count = position(plan[at]);
                at += 1;
                let mut pieces = Vec::new();
                for _ in 0..count {
                    pieces.push(source_slice(&units, plan[at], plan[at + 1], range)?);
                    at += 2;
                }
                params.push(read_jsdoc_type(
                    &pieces.join(":"),
                    range,
                    aliases,
                    expanding,
                )?);
            }
            assert_eq!(
                at,
                plan.len(),
                "a callable syntax plan consumes every source slice"
            );
            let ret = read_jsdoc_type(
                &source_slice(&units, plan[1], plan[2], range)?,
                range,
                aliases,
                expanding,
            )?;
            Ok(Some(Type::Function {
                params,
                ret: Box::new(ret),
            }))
        }
        _ => unreachable!("a generated callable syntax action has a declared constructor"),
    }
}
