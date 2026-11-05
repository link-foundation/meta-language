//! Dense transport of actual CST field links; shared rules decide module roles.
use super::{
    BTreeMap, LinkNetwork, LinkType, ProgramRange, ProgramRepresentationError, ProgramSourceMapping,
};
use crate::translation::module_binding_rules as rules;

fn number(value: usize) -> f64 {
    serde_json::Number::from(value)
        .as_f64()
        .expect("a bounded source coordinate is a Number")
}

pub(super) fn read_fields(
    network: &LinkNetwork,
    syntax: &[ProgramSourceMapping],
    language: &str,
) -> Result<Vec<f64>, ProgramRepresentationError> {
    if language != "JavaScript" {
        return Ok(Vec::new());
    }
    let positions = syntax
        .iter()
        .enumerate()
        .map(|(index, node)| (node.link_id, index))
        .collect::<BTreeMap<_, _>>();
    let mut fields = vec![Vec::new(); syntax.len()];
    for link in network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Field))
    {
        let references = link.references();
        if references.len() != 3 {
            continue;
        }
        let (Some(parent), Some(child)) = (
            positions.get(&references[0].as_u64()),
            positions.get(&references[2].as_u64()),
        ) else {
            continue;
        };
        let Some(label) = network
            .link(references[1])
            .and_then(|label| label.metadata().term())
        else {
            continue;
        };
        fields[*parent].push((label.to_owned(), *child));
    }
    let mut starts = Vec::new();
    let mut counts = Vec::new();
    let mut labels = Vec::new();
    let mut targets = Vec::new();
    for row in fields {
        starts.push(number(labels.len()));
        counts.push(number(row.len()));
        for (label, child) in row {
            labels.push(label);
            targets.push(number(child));
        }
    }
    let terms = syntax
        .iter()
        .map(|node| node.term.clone())
        .collect::<Vec<_>>();
    let node_starts = syntax
        .iter()
        .map(|node| number(node.range.start))
        .collect::<Vec<_>>();
    let node_ends = syntax
        .iter()
        .map(|node| number(node.range.end))
        .collect::<Vec<_>>();
    let plan = rules::plan_java_script_module_binding_fields(
        &terms,
        &node_starts,
        &node_ends,
        &starts,
        &counts,
        &labels,
        &targets,
    );
    if plan.first() == Some(&-1.0) {
        return Err(ProgramRepresentationError::InvalidModuleBindings(
            rules::read_source_module_binding_error(plan[1]),
        ));
    }
    Ok(plan)
}
pub(super) fn role(plan: &[f64], range: ProgramRange) -> f64 {
    rules::read_source_module_binding_range_role(plan, number(range.start), number(range.end))
}
pub(super) fn affixes(
    role: f64,
    shorthand: bool,
    original: &str,
) -> Result<Vec<String>, ProgramRepresentationError> {
    if role == 3.0 {
        return Err(ProgramRepresentationError::InvalidModuleBindings(
            rules::read_source_module_binding_error(5.0),
        ));
    }
    Ok(rules::read_source_binding_rename_affixes(
        role, shorthand, original,
    ))
}
