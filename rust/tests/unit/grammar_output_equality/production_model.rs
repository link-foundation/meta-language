// Generated from complete owned Rust CST declarations. Do not edit by hand.
#[path = "production_model/forking.rs"]
mod forking;
#[path = "production_model/grammar_selection.rs"]
mod grammar_selection;
#[path = "production_model/grammar_selection_rules.rs"]
mod grammar_selection_rules;
#[path = "production_model/operations.rs"]
mod operations;
#[path = "production_model/program.rs"]
mod program;
#[path = "production_model/results.rs"]
mod results;
#[path = "production_model/walk.rs"]
mod walk;
// Bind every copied declaration without relaxing native lints.
const _: fn() = || {
    let _ = program::Associativity::Left;
    let _ = program::Associativity::Right;
    let _ = program::Associativity::None;
    let _ = |value: &program::PrecedenceTag| {
        let _ = (&value.level, &value.name, &value.associativity, &value.rule);
    };
    let _ = |value: &program::TokenRank| {
        let _ = (&value.priority, &value.specificity, &value.order);
    };
    let _ = |value: &grammar_selection::AcceptedSelection| {
        let _ = (
            &value.language,
            &value.root,
            &value.program,
            &value.provenance,
        );
    };
    let _ = |value: &grammar_selection::SelectionMetadata| {
        let _ = (
            &value.mode,
            &value.mapping,
            &value.start,
            &value.end,
            &value.candidates,
            &value.provenance,
            &value.alternatives,
            &value.ambiguous,
        );
    };
    let _ = grammar_selection_rules::accept_grammar_selection_mode;
    let _ = grammar_selection_rules::accept_grammar_selection_mapping;
    let _ = grammar_selection_rules::accept_grammar_selection_count;
    let _ = grammar_selection_rules::report_grammar_selection_ambiguity;
    let _ = grammar_selection_rules::read_grammar_tree_equality_fields;
    let _ = grammar_selection_rules::read_grammar_tree_recursion_plan;
    let _ = grammar_selection_rules::read_grammar_selection_equality_fields;
    let _ = forking::Lead::Literal;
    let _ = forking::Lead::Ref;
    let _ = |value: &forking::Lead| match value {
        forking::Lead::Literal(field_0) => {
            let _ = field_0;
        }
        forking::Lead::Ref(field_0) => {
            let _ = field_0;
        }
    };
    let _ = results::TreeType::Node;
    let _ = results::TreeType::Token;
    let _ = results::TreeType::Error;
    let _ = results::TreeType::Missing;
    let _ = results::TreeType::Embed;
    let _ = |value: &results::ChildList| {
        let _ = (&value.count, &value.flat, &value.link);
    };
    let _ = results::children_of;
    let _ = results::join_children;
    let _ = |value: &results::Tree| {
        let _ = (
            &value.ty,
            &value.kind,
            &value.projection_name,
            &value.projection_anonymous,
            &value.rule,
            &value.field,
            &value.trivia,
            &value.start,
            &value.end,
            &value.children,
            &value.attributes,
            &value.lexical,
            &value.precedence,
            &value.tail,
            &value.priority,
            &value.reduced,
            &value.closes,
            &value.reduced_to,
            &value.forked_to,
            &value.alone,
            &value.before,
            &value.dynamic,
            &value.ambiguous,
            &value.literal,
            &value.scanned,
            &value.lexed,
            &value.plain,
            &value.rank,
            &value.language,
            &value.root,
            &value.program,
            &value.selection,
        );
    };
    let _ = results::Tree::new;
    let _ = results::Tree::node;
    let _ = results::Tree::trivia_leaf;
    let _ = results::no_children;
    let _ = walk::Walk::Part;
    let _ = walk::Walk::Item;
    let _ = walk::Walk::End;
    let _ = |value: &walk::Walk| match value {
        walk::Walk::Part(field_0) => {
            let _ = field_0;
        }
        walk::Walk::Item(field_0) => {
            let _ = field_0;
        }
        walk::Walk::End(field_0) => {
            let _ = field_0;
        }
    };
    let _ = walk::open_part;
    let _ = walk::next_item;
    let _ = walk::open_node;
    let _ = walk::items;
    let _ = walk::meaningful;
    let _ = walk::same_tree;
    let _ = walk::same_tree_in;
    let _ = walk::same_children;
    let _ = walk::same_output;
    let _ = |value: &walk::GrammarTreeEqualityPlan| {
        let _ = (&value.fields, &value.recursion);
    };
    let _ = walk::first_meaningful;
    let _ = walk::first_leaf_start;
    let _ = walk::leftmost_chain;
};
