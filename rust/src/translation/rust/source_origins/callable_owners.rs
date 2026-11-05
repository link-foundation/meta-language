//! Generic recursive origin-to-column transport; shared rules interpret it.
use super::SourceOrigin;
use crate::translation::rust_callable_owner_rules::{
    read_rust_callable_owner_projection_failure, read_rust_callable_owner_projection_limit,
};
fn count(value: usize) -> f64 {
    u32::try_from(value).map_or(f64::INFINITY, f64::from)
}
use crate::translation::types::Type;
pub(super) fn descriptor(origin: &SourceOrigin) -> Vec<String> {
    let mut pending = vec![(origin, String::new(), false, 0usize)];
    let mut rows = Vec::new();
    let mut visits = 0usize;
    while let Some((value, path, inherited, depth)) = pending.pop() {
        visits += 1;
        if count(visits) > read_rust_callable_owner_projection_limit("visits")
            || count(depth) > read_rust_callable_owner_projection_limit("depth")
        {
            return vec![read_rust_callable_owner_projection_failure(); 2];
        }
        let callable = matches!(&value.ty, Some(Type::Function { .. }));
        let active = inherited || callable;
        if active {
            let owner = if let Some(expanded) = &value.expanded_source_owner {
                SourceOrigin {
                    source_owner: Some(expanded.clone()),
                    integer_source_owner_constraint: value.integer_source_owner_constraint.clone(),
                    ..SourceOrigin::unknown()
                }
                .pointer_integer_owner()
            } else {
                value.pointer_integer_owner()
            };
            rows.push(path.clone());
            rows.push(owner);
        }
        if let Some(fields) = &value.fields {
            for (index, field) in fields.iter().enumerate().rev() {
                pending.push((field, format!("{path}/field/{index}"), active, depth + 1));
            }
        }
        if callable {
            if let Some(result) = &value.callable_result {
                pending.push((result, format!("{path}/result"), true, depth + 1));
            }
            if let Some(parameters) = &value.callable_parameters {
                for (index, parameter) in parameters.iter().enumerate().rev() {
                    pending.push((
                        parameter,
                        format!("{path}/parameter/{index}"),
                        true,
                        depth + 1,
                    ));
                }
            }
        }
        if count(rows.len()) > read_rust_callable_owner_projection_limit("cells") {
            return vec![read_rust_callable_owner_projection_failure(); 2];
        }
    }
    rows
}
