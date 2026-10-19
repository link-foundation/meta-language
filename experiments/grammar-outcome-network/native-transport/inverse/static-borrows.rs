//! Typed host storage only; shared JavaScript owns selection and admission.
#[path = "../../../../rust/src/translation/owned_outcome_static_borrow_rules.rs"]
mod rules;

pub fn resolve_owned_outcome_static_borrow(
    text: &str,
    registry: &[&'static str],
) -> Result<&'static str, String> {
    let limit = || rules::read_owned_outcome_static_borrow_failure(-3.0);
    let entries = u32::try_from(registry.len()).map_err(|_| limit())?;
    if !rules::accept_owned_outcome_static_borrow_resources(f64::from(entries), 0.0) {
        return Err(limit());
    }
    let mut bytes = u32::try_from(text.len()).map_err(|_| limit())?;
    for value in registry {
        bytes = bytes
            .checked_add(u32::try_from(value.len()).map_err(|_| limit())?)
            .ok_or_else(limit)?;
    }
    if !rules::accept_owned_outcome_static_borrow_resources(f64::from(entries), f64::from(bytes)) {
        return Err(limit());
    }
    let labels = registry
        .iter()
        .map(|value| (*value).to_owned())
        .collect::<Vec<_>>();
    let selected = rules::find_owned_outcome_static_borrow_index(text, &labels, f64::from(bytes));
    let Some(index) = rules::ml_array::number_index(selected) else {
        return Err(rules::read_owned_outcome_static_borrow_failure(selected));
    };
    registry
        .get(index)
        .copied()
        .ok_or_else(|| rules::read_owned_outcome_static_borrow_failure(-4.0))
}
