//! Typed storage transport for the shared grammar composition decisions.

use crate::decorators::DecoratorError;
use crate::grammar::feature::{
    check_grammar_selection_included_range_expression, check_grammar_selection_included_range_form,
};
use crate::grammar::interchange::{parse_links_expression, render_links_expression};
use crate::grammar::{FeatureExpr, FeatureForm, FieldValue, GrammarExpr, GrammarRule};
use crate::translation::grammar_rule_composition_rules::{
    accept_grammar_composition_candidate_profile, read_grammar_composition_candidate_issue,
    read_grammar_composition_context_issue, read_grammar_composition_disposition,
    read_grammar_composition_field_order,
};

fn refuse(message: impl AsRef<str>) -> DecoratorError {
    DecoratorError::new(format!("grammar composition: {}", message.as_ref()))
}

fn read_selection_fields(
    expression: &GrammarExpr,
) -> Result<(&FeatureForm, &GrammarExpr, &[GrammarExpr]), DecoratorError> {
    let GrammarExpr::Feature(feature) = expression else {
        return Err(refuse("selection needs exactly its declared fields"));
    };
    let FeatureExpr::Form(form) = &**feature else {
        return Err(refuse("selection needs exactly its declared fields"));
    };
    let fields = read_grammar_composition_field_order("selection");
    if form.fields.len() + 1 != fields.split('|').count() {
        return Err(refuse("selection needs exactly its declared fields"));
    }
    let [FieldValue::Expression(item), FieldValue::Expressions(items)] = form.fields.as_slice()
    else {
        return Err(refuse("selection needs enumerable own data fields"));
    };
    Ok((form, item, items))
}

fn read_candidate_facts(
    items: &[GrammarExpr],
    required: bool,
) -> Result<Option<String>, DecoratorError> {
    let mut kinds = Vec::new();
    let mut mappings = Vec::new();
    let mut names = Vec::new();
    let mut selectors = Vec::new();
    let mut keys = Vec::new();
    for value in items {
        let GrammarExpr::Feature(feature) = value else {
            return Err(refuse("candidate needs an inspectable expression"));
        };
        let FeatureExpr::Form(candidate) = &**feature else {
            return Err(refuse("candidate needs an inspectable expression"));
        };
        if !required
            && !accept_grammar_composition_candidate_profile(
                &candidate.head,
                candidate.text("mapping").unwrap_or_default(),
            )
        {
            return Ok(None);
        }
        let role = if candidate.head == "grammar-field-candidate" {
            "field-candidate"
        } else {
            "json-candidate"
        };
        let fields = read_grammar_composition_field_order(role);
        if candidate.fields.len() + 1 != fields.split('|').count() {
            return Err(refuse(format!("{role} needs exactly its declared fields")));
        }
        let (name, mapping, selector) = match candidate.fields.as_slice() {
            [FieldValue::Name(name), FieldValue::Name(mapping)] => (name, mapping, String::new()),
            [
                FieldValue::Name(name),
                FieldValue::Name(mapping),
                FieldValue::Expression(selector),
            ] => {
                if !check_grammar_selection_included_range_form(candidate).map_err(refuse)? {
                    return Err(refuse("field candidate is not a source-view form"));
                }
                (name, mapping, render_links_expression(selector))
            }
            _ => return Err(refuse("candidate source and mapping need text")),
        };
        kinds.push(candidate.head.clone());
        mappings.push(mapping.clone());
        names.push(name.clone());
        selectors.push(selector);
        keys.push(render_links_expression(value));
    }
    let issue = read_grammar_composition_candidate_issue(&kinds, &mappings, &names, &selectors);
    if !issue.is_empty() {
        return Err(refuse(issue));
    }
    serde_json::to_string(&keys)
        .map(Some)
        .map_err(|error| refuse(error.to_string()))
}

fn read_template(
    source: &str,
    rule_kind: &str,
    tree_decorators: bool,
) -> Result<(GrammarExpr, String), DecoratorError> {
    let template = parse_links_expression(source)
        .map_err(|error| refuse(format!("invalid authoring Links: {error}")))?;
    check_grammar_selection_included_range_expression(&template).map_err(refuse)?;
    let (selection, slot, items) = read_selection_fields(&template)?;
    let slot_kind = render_links_expression(slot);
    let issue = read_grammar_composition_context_issue(
        rule_kind,
        &selection.head,
        &slot_kind,
        tree_decorators,
    );
    if !issue.is_empty() {
        return Err(refuse(issue));
    }
    let candidates = read_candidate_facts(items, true)?
        .expect("required candidate facts retain their complete ordered key");
    Ok((template, candidates))
}

pub(super) fn check_grammar_composition_rule_data(
    rule: &GrammarRule,
) -> Result<(), DecoratorError> {
    check_grammar_selection_included_range_expression(&rule.expr).map_err(refuse)?;
    for action in rule.attributes.action.as_deref().unwrap_or_default() {
        check_grammar_selection_included_range_expression(&GrammarExpr::feature(
            FeatureExpr::Form(action.clone()),
        ))
        .map_err(refuse)?;
    }
    Ok(())
}

pub(super) fn compose_grammar_rule_expression(
    rule: &GrammarRule,
    source: &str,
    rule_kind: &str,
    tree_decorators: bool,
) -> Result<GrammarExpr, DecoratorError> {
    check_grammar_composition_rule_data(rule)?;
    let (mut template, candidates) = read_template(source, rule_kind, tree_decorators)?;
    let mut actual_mode = String::new();
    let mut actual_candidates = String::new();
    if let GrammarExpr::Feature(feature) = &rule.expr
        && let FeatureExpr::Form(form) = &**feature
        && matches!(form.head.as_str(), "any-grammar" | "all-grammar")
    {
        let (existing, _, items) = read_selection_fields(&rule.expr)?;
        if let Some(key) = read_candidate_facts(items, false)? {
            actual_mode.clone_from(&existing.head);
            actual_candidates = key;
        }
    }
    let (selection, _, _) = read_selection_fields(&template)?;
    if read_grammar_composition_disposition(
        &actual_mode,
        &actual_candidates,
        &selection.head,
        &candidates,
    )
    .to_bits()
        == 1f64.to_bits()
    {
        return Ok(rule.expr.clone());
    }
    let GrammarExpr::Feature(feature) = &mut template else {
        unreachable!("the checked template is a selection form");
    };
    let FeatureExpr::Form(form) = &mut **feature else {
        unreachable!("the checked template is a selection form");
    };
    form.fields[0] = FieldValue::Expression(rule.expr.clone());
    Ok(template)
}
