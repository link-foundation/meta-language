//! Transport checked lexical cell occurrences into immutable formal bindings.

use std::borrow::Cow;
use std::collections::{BTreeSet, HashMap};

use crate::translation::Language;
use crate::translation::callable_emission_rules::accept_formal_local_cell_element;
use crate::translation::ir::{Decl, Expr, FnDecl, Node, Program, rename_function};
use crate::translation::source_identifiers::{
    collect_source_identifier_texts, read_source_identifier_supply,
};
use crate::translation::{Span, types::Type};

fn contains_cell(expression: &Expr) -> bool {
    let mut pending = vec![expression];
    while let Some(value) = pending.pop() {
        if matches!(&value.ty, Type::Cell { .. })
            || matches!(
                &value.node,
                Node::CellAllocate { .. } | Node::CellRead { .. } | Node::CellWrite { .. }
            )
        {
            return true;
        }
        pending.extend(value.children());
    }
    false
}

struct LocalCells {
    state: HashMap<String, Expr>,
    bindings: Vec<(String, Expr, Option<Span>)>,
    prefix: String,
    next: usize,
}

impl LocalCells {
    fn bind(&mut self, name: String, value: Expr, span: Option<Span>) -> Expr {
        let variable = Expr::var(name.clone(), value.ty.clone()).with_span(span);
        self.bindings.push((name, value, span));
        variable
    }

    fn read(&self, expression: &Expr) -> Option<Expr> {
        match &expression.node {
            Node::CellRead { cell } => return self.state.get(cell.var_name()?).cloned(),
            Node::CellAllocate { .. } | Node::CellWrite { .. } => return None,
            Node::Lambda { body, .. } => {
                return (!contains_cell(body)).then(|| expression.clone());
            }
            _ if matches!(&expression.ty, Type::Cell { .. }) => return None,
            _ => {}
        }
        let mut complete = true;
        let projected = expression.map_children(&mut |child| {
            self.read(child).unwrap_or_else(|| {
                complete = false;
                child.clone()
            })
        });
        complete.then_some(projected)
    }

    fn lower(&mut self, expression: &Expr) -> Option<Expr> {
        match &expression.node {
            Node::Let { name, value, body } => {
                if let Node::CellAllocate { value: initializer } = &value.node {
                    let Type::Cell { element } = &value.ty else {
                        return None;
                    };
                    if !accept_formal_local_cell_element(element.kind()) {
                        return None;
                    }
                    let initial = self.lower(initializer)?;
                    let variable = self.bind(name.clone(), initial, expression.span);
                    self.state.insert(name.clone(), variable);
                    let result = self.lower(body);
                    self.state.remove(name);
                    result
                } else {
                    let value = self.lower(value)?;
                    self.bind(name.clone(), value, expression.span);
                    self.lower(body)
                }
            }
            Node::CellWrite { cell, value, body } => {
                let name = cell.var_name()?;
                if !self.state.contains_key(name) {
                    return None;
                }
                let value = self.lower(value)?;
                self.next += 1;
                let variable = self.bind(
                    format!("{}localCell{}", self.prefix, self.next),
                    value,
                    expression.span,
                );
                self.state.insert(name.to_owned(), variable);
                self.lower(body)
            }
            _ => self.read(expression),
        }
    }
}

fn lower_function(function: &FnDecl) -> Option<FnDecl> {
    let (params, body) = rename_function(function, &|name| name.to_owned(), &BTreeSet::new());
    let names = serde_json::json!({ "params": params, "body": body });
    let mut cells = LocalCells {
        state: HashMap::new(),
        bindings: Vec::new(),
        prefix: read_source_identifier_supply(collect_source_identifier_texts(&names)),
        next: 0,
    };
    let mut body = cells.lower(&body)?;
    for (name, value, span) in cells.bindings.into_iter().rev() {
        let ty = body.ty.clone();
        body = Expr::new(
            Node::Let {
                name,
                value: Box::new(value),
                body: Box::new(body),
            },
            ty,
        )
        .with_span(span);
    }
    Some(FnDecl {
        params,
        body,
        ..function.clone()
    })
}

pub(crate) fn prepare(program: &Program) -> Cow<'_, Program> {
    if program.source_language != Language::JavaScript {
        return Cow::Borrowed(program);
    }
    let mut lowered = None;
    for (index, entry) in program.declarations.iter().enumerate() {
        if let Decl::Fn(function) = entry
            && contains_cell(&function.body)
            && let Some(function) = lower_function(function)
        {
            lowered.get_or_insert_with(|| program.clone()).declarations[index] = Decl::Fn(function);
        }
    }
    lowered.map_or(Cow::Borrowed(program), Cow::Owned)
}
