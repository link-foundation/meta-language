//! Actual CST/source transport only. The normal generated JavaScript rule owns
//! every form decision; this adapter neither executes nor rewrites bodies.
use super::{dependency_rules, recursive_groups::source_number};
use crate::native_grammar_parser::{NativeNode, parse_native};
use crate::translation::frontend_rules::ml_array::number_index;

#[derive(Default)]
struct Facts<'a> {
    nodes: Vec<&'a NativeNode>,
    terms: Vec<&'a str>,
    fields: Vec<&'a str>,
    parents: Vec<f64>,
    starts: Vec<f64>,
    ends: Vec<f64>,
    texts: Vec<&'a str>,
    errors: Vec<f64>,
}

impl<'a> Facts<'a> {
    fn visit(
        &mut self,
        node: &'a NativeNode,
        field: &'a str,
        parent: f64,
        source: &'a str,
    ) -> Option<()> {
        let at = source_number(self.nodes.len())?;
        self.nodes.push(node);
        self.terms.push(&node.term);
        self.fields.push(field);
        self.parents.push(parent);
        self.starts.push(source_number(node.start)?);
        self.ends.push(source_number(node.end)?);
        self.texts.push(if node.children.is_empty() {
            source.get(node.start..node.end)?
        } else {
            ""
        });
        self.errors
            .push(if node.is_error || node.is_missing || node.has_error {
                1.0
            } else {
                0.0
            });
        for (child, field) in &node.children {
            self.visit(child, field.as_deref().unwrap_or(""), at, source)?;
        }
        Some(())
    }
}

pub(super) fn read_binding(source: &str) -> Option<(u8, String)> {
    let root = parse_native("native-javascript", source);
    let mut facts = Facts::default();
    facts.visit(&root, "", -1.0, source)?;
    let selected = dependency_rules::read_deferred_binding(
        &facts.terms,
        &facts.fields,
        &facts.parents,
        &facts.starts,
        &facts.ends,
        &facts.texts,
        &facts.errors,
        source_number(source.len())?,
    );
    let [kind, at] = selected.as_slice() else {
        return None;
    };
    let kind = u8::try_from(number_index(*kind)?).ok()?;
    if !matches!(kind, 1 | 2) {
        return None;
    }
    let at = number_index(*at)?;
    let node = facts.nodes.get(at)?;
    if node.term != "identifier" || !node.children.is_empty() || node.end <= node.start {
        return None;
    }
    let name = source.get(node.start..node.end)?;
    if name != *facts.texts.get(at)? {
        return None;
    }
    Some((kind, name.to_owned()))
}

/// Actual source/column transport to the independently generated inert-value rule.
pub fn read_inert_binding(source: &str) -> Option<(String, u8)> {
    if source.len() > 65_536 {
        return None;
    }
    let root = parse_native("native-javascript", source);
    let mut facts = Facts::default();
    facts.visit(&root, "", -1.0, source)?;
    let terms = facts
        .terms
        .iter()
        .map(|value| (*value).to_owned())
        .collect::<Vec<_>>();
    let fields = facts
        .fields
        .iter()
        .map(|value| (*value).to_owned())
        .collect::<Vec<_>>();
    let texts = facts
        .texts
        .iter()
        .map(|value| (*value).to_owned())
        .collect::<Vec<_>>();
    let mut selected = crate::translation::inert_module_binding_rules::read_inert_module_binding(
        &terms,
        &fields,
        &facts.parents,
        &facts.starts,
        &facts.ends,
        &texts,
        &facts.errors,
        source_number(source.len())?,
    );
    if selected.is_empty() {
        selected = crate::translation::inert_module_binding_rules::read_empty_standard_collection_module_binding(
            &terms,
            &fields,
            &facts.parents,
            &facts.starts,
            &facts.ends,
            &texts,
            &facts.errors,
            source_number(source.len())?,
        );
    }
    if selected.is_empty() {
        selected = crate::translation::inert_module_binding_rules::read_numeric_module_binding(
            &terms,
            &fields,
            &facts.parents,
            &facts.starts,
            &facts.ends,
            &texts,
            &facts.errors,
            source_number(source.len())?,
        );
    }
    let [at, owner] = selected.as_slice() else {
        return None;
    };
    let at = number_index(*at)?;
    let owner = u8::try_from(number_index(*owner)?).ok()?;
    let node = facts.nodes.get(at)?;
    if node.term != "identifier" || !node.children.is_empty() || node.end <= node.start {
        return None;
    }
    let name = source.get(node.start..node.end)?;
    if name != *facts.texts.get(at)? {
        return None;
    }
    Some((name.to_owned(), owner))
}

/// Only parser-owned fact/coordinate transport; the shared rule proves purity.
pub(super) fn read_numeric_binding_name(source: &str) -> Option<String> {
    if source.len() > 65_536 {
        return None;
    }
    let root = parse_native("native-javascript", source);
    let mut facts = Facts::default();
    facts.visit(&root, "", -1.0, source)?;
    let terms = facts
        .terms
        .iter()
        .map(|value| (*value).to_owned())
        .collect::<Vec<_>>();
    let fields = facts
        .fields
        .iter()
        .map(|value| (*value).to_owned())
        .collect::<Vec<_>>();
    let texts = facts
        .texts
        .iter()
        .map(|value| (*value).to_owned())
        .collect::<Vec<_>>();
    let selected = crate::translation::inert_module_binding_rules::read_numeric_module_binding(
        &terms,
        &fields,
        &facts.parents,
        &facts.starts,
        &facts.ends,
        &texts,
        &facts.errors,
        source_number(source.len())?,
    );
    let [at, owner] = selected.as_slice() else {
        return None;
    };
    if *owner != 0.0 {
        return None;
    }
    let at = number_index(*at)?;
    let node = facts.nodes.get(at)?;
    if node.term != "identifier" || !node.children.is_empty() || node.end <= node.start {
        return None;
    }
    let name = source.get(node.start..node.end)?;
    if name != *facts.texts.get(at)? {
        return None;
    }
    Some(name.to_owned())
}
