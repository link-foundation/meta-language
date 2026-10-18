//! Test-only grammar composition transport and public API adapters.
//! All decisions remain in the production/generated implementation.
pub(super) use std::collections::HashMap;

pub(super) use links_notation::{ParserConfig, parse_lino_to_links_with_config};
pub(super) use meta_language::grammar::interchange::{deserialize_grammar, serialize_grammar};
pub(super) use meta_language::grammar::{FeatureExpr, FeatureForm, FieldValue};
pub(super) use meta_language::{
    Decorator, DecoratorAction, DecoratorLevel, DecoratorSet, FeatureParseOptions,
    GlobalConceptOptions, GlobalConceptSource, Grammar, GrammarExpr, GrammarKind, GrammarRule,
    GrammarSourceName, ParseOutcome, check_grammar_selection_mapped_tree, compile_feature_grammar,
    create_global_concept_space, create_global_concept_space_parser, decorate_grammar,
    global_concept_space_to_links, parse_grammar_links, parse_links_expression,
    percent_encode_links_text, render_grammar_links, render_links_expression,
};
pub(super) use serde_json::{Value, json};

use super::{bundle, physical};

#[path = "../../src/translation/grammar_rule_composition_rules.rs"]
pub(super) mod rules;

pub(super) fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

pub(super) fn json_grammar() -> Grammar {
    parse_grammar_links(include_str!("../../src/data/native-grammars/json.lino")).unwrap()
}

pub(super) fn field_candidate(name: &str, first: i64) -> GrammarExpr {
    parse_links_expression(&format!(
        "(grammar-field-candidate {} field-ranges (source-field-selector value include {first} 1))",
        percent_encode_links_text(name),
    ))
    .unwrap()
}

pub(super) fn json_candidate(name: &str) -> GrammarExpr {
    parse_links_expression(&format!(
        "(grammar-candidate {} json-string)",
        percent_encode_links_text(name),
    ))
    .unwrap()
}

pub(super) fn author(id: &str, level: DecoratorLevel, name: &str, value: &str) -> Decorator {
    Decorator::new(
        id,
        level,
        0,
        vec![("name".to_owned(), name.to_owned())],
        vec![DecoratorAction::Set {
            field: "composition".to_owned(),
            value: value.to_owned(),
        }],
    )
    .unwrap()
}

pub(super) fn policy(grammar: &Grammar, candidates: Vec<GrammarExpr>, mode: &str) -> DecoratorSet {
    let template = GrammarExpr::feature(FeatureExpr::Form(FeatureForm::new(
        mode,
        vec![
            FieldValue::Expression(GrammarExpr::Empty),
            FieldValue::Expressions(candidates),
        ],
    )));
    DecoratorSet::new(vec![author(
        "compose",
        DecoratorLevel::GrammarRule,
        &grammar.start_rule().unwrap().name,
        &render_links_expression(&template),
    )])
    .unwrap()
}

pub(super) fn composed(grammar: &Grammar, candidates: Vec<GrammarExpr>, mode: &str) -> Grammar {
    decorate_grammar(
        grammar,
        &policy(grammar, candidates, mode),
        DecoratorLevel::GrammarRule,
    )
    .unwrap()
}

pub(super) fn form(expression: &GrammarExpr) -> &FeatureForm {
    let GrammarExpr::Feature(feature) = expression else {
        panic!("an actual feature expression")
    };
    let FeatureExpr::Form(form) = feature.as_ref() else {
        panic!("an actual feature form")
    };
    form
}

pub(super) fn form_mut(expression: &mut GrammarExpr) -> &mut FeatureForm {
    let GrammarExpr::Feature(feature) = expression else {
        panic!("an actual feature expression")
    };
    let FeatureExpr::Form(form) = feature.as_mut() else {
        panic!("an actual feature form")
    };
    form
}

pub(super) fn item(expression: &GrammarExpr) -> &GrammarExpr {
    let FieldValue::Expression(item) = &form(expression).fields[0] else {
        panic!("the original item")
    };
    item
}

pub(super) fn candidates(expression: &GrammarExpr) -> &[GrammarExpr] {
    let FieldValue::Expressions(items) = &form(expression).fields[1] else {
        panic!("ordered candidates")
    };
    items
}

pub(super) fn replaced(grammar: &Grammar, name: &str, replacement: &GrammarRule) -> Grammar {
    let mut output = Grammar::new().with_declarations(grammar.declarations().clone());
    if let Some(start) = grammar.start() {
        output.set_start(start);
    }
    if let Some(format) = grammar.source_format() {
        output.set_source_format(format);
    }
    output.set_kinds(grammar.kinds().to_vec());
    for rule in grammar.rules() {
        output.add_rule(if rule.name == name {
            replacement.clone()
        } else {
            rule.clone()
        });
    }
    output
}

pub(super) fn parse(
    grammar: &Grammar,
    sources: &HashMap<String, Grammar>,
    source: &str,
) -> ParseOutcome {
    let resolve = |name: &str| sources.get(name).cloned();
    compile_feature_grammar(grammar, Some(&resolve), FeatureParseOptions::default())
        .unwrap()
        .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
        .unwrap()
}

pub(super) fn mapped(outcome: &ParseOutcome, source: &str) -> Value {
    assert!(outcome.ok, "{:?}", outcome.rejection);
    let result = bundle(outcome);
    assert_eq!(physical(&result["tree"]), source.as_bytes());
    check_grammar_selection_mapped_tree(&result, &FeatureParseOptions::default()).unwrap();
    result
}

pub(super) fn embeddings<'a>(tree: &'a Value, output: &mut Vec<&'a Value>) {
    if tree["type"] == "embed" {
        output.push(tree);
    }
    for role in ["children", "sourceChildren"] {
        if let Some(children) = tree[role].as_array() {
            for child in children {
                embeddings(child, output);
            }
        }
    }
    if let Some(root) = tree.get("root") {
        embeddings(root, output);
    }
}

pub(super) fn embedding<'a>(tree: &'a Value, mapping: &str) -> &'a Value {
    let mut nodes = Vec::new();
    embeddings(tree, &mut nodes);
    nodes
        .into_iter()
        .find(|node| node["selection"]["mapping"] == mapping)
        .unwrap()
}

// Test-only removal of mapped wrappers, preserving all original physical
// nodes, trivia, fields, metadata and spans. It performs no grammar validation.
pub(super) fn original_trees(tree: &Value) -> Vec<Value> {
    if tree["type"] == "embed" && tree.get("sourceChildren").is_some() {
        return tree["sourceChildren"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(original_trees)
            .collect();
    }
    let mut copy = tree.clone();
    if tree["type"] == "node" {
        copy["children"] = json!(
            tree["children"]
                .as_array()
                .unwrap()
                .iter()
                .flat_map(original_trees)
                .collect::<Vec<_>>()
        );
    }
    vec![copy]
}

pub(super) fn first_field(tree: &Value) -> Option<&Value> {
    if tree["field"] == "value" && tree["trivia"] != true {
        return Some(tree);
    }
    tree["children"].as_array()?.iter().find_map(first_field)
}
