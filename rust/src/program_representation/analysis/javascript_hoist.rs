use super::{BTreeSet, Declaration, ProgramScope, ProgramSourceMapping, SemanticToken};

pub(super) fn hoist_var_declarations(
    declarations: &mut [Declaration],
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    scopes: &[ProgramScope],
) {
    let function_bodies = syntax
        .iter()
        .filter(|fact| {
            matches!(
                fact.term.as_str(),
                "function_declaration"
                    | "function_expression"
                    | "generator_function_declaration"
                    | "generator_function"
                    | "arrow_function"
                    | "method_definition"
            )
        })
        .flat_map(|function| {
            syntax.iter().filter_map(move |block| {
                (block.term == "statement_block"
                    && block.range.start >= function.range.start
                    && block.range.end == function.range.end)
                    .then_some(block.range.start + 1)
            })
        })
        .collect::<BTreeSet<_>>();

    for declaration in declarations.iter_mut().filter(|item| item.kind == "var") {
        let mut scope = tokens[declaration.token].scope;
        while let Some(parent) = scopes[scope].parent.as_ref() {
            if function_bodies.contains(&scopes[scope].range.start) {
                break;
            }
            scope = scopes
                .iter()
                .position(|candidate| &candidate.id == parent)
                .expect("parent scope exists");
        }
        declaration.scope = scope;
    }
}
