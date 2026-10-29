use super::{
    BTreeMap, BTreeSet, Declaration, JavaScriptFunctionScope, ProgramScope, ProgramSourceMapping,
    SemanticToken,
};

pub(super) fn hoist_var_declarations(
    declarations: &mut Vec<Declaration>,
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    scopes: &[ProgramScope],
    function_scopes: &BTreeMap<usize, JavaScriptFunctionScope>,
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

    let environments = function_scopes
        .values()
        .filter_map(|scope| scope.body_start.map(|start| (start, scope)))
        .collect::<BTreeMap<_, _>>();
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
        declaration.scope = environments
            .get(&scopes[scope].range.start)
            .filter(|environment| environment.simple)
            .map_or(scope, |environment| environment.parameter_scope);
    }
    let mut cells = declarations
        .iter()
        .filter(|item| item.kind == "parameter")
        .map(|item| (item.scope, tokens[item.token].text.clone()))
        .collect::<BTreeSet<_>>();
    declarations.retain(|item| {
        if item.kind!="var"{return true;}
        let key=(item.scope,tokens[item.token].text.clone());
        if crate::translation::source_library_binding_rules::test_java_script_source_variable_redeclaration(&item.kind,cells.contains(&key)){return false;}
        cells.insert(key);true
    });
}
