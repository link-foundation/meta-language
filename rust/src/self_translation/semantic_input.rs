//! Native item views over the automatically translated shared grouping plan.
use super::{Group, GroupKind, Item, line_break, source_lines};
use crate::translation::semantic_input_rules::{
    plan_semantic_source_groups, read_semantic_source_role,
};

pub(super) fn group_semantic_items<'a>(
    items: &[Item<'a>],
    text: &'a str,
    source_language: &str,
) -> Vec<Group<'a>> {
    let roles: Vec<f64> = items
        .iter()
        .map(|item| read_semantic_source_role(item.comment, item.text))
        .collect();
    let line_breaks: Vec<bool> = items.iter().map(|item| line_break(item.after)).collect();
    let plan = plan_semantic_source_groups(&roles, &line_breaks);
    plan.chunks_exact(3)
        .map(|record| {
            // These positions are produced by the complete shared source algorithm,
            // bounded by the actual native item slice supplied above.
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let first = record[0] as usize;
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let last = record[1] as usize;
            let selected = &items[first..=last];
            let kind = if record[2] == 0.0 {
                GroupKind::Provenance
            } else if record[2] == 1.0 {
                GroupKind::Carried {
                    language: source_language.to_owned(),
                    lines: source_lines(&items[first + 1..=last]),
                }
            } else if record[2] == 2.0 {
                GroupKind::Comment {
                    text: text[items[first].start..items[last].end].to_owned(),
                    term: items[last].term.clone(),
                }
            } else {
                GroupKind::Item {
                    text: text[items[first].start..items[last].end].to_owned(),
                    term: items[last].term.clone(),
                }
            };
            Group {
                kind,
                items: selected.to_vec(),
                after: items[last].after,
            }
        })
        .collect()
}
