//! Mechanical evaluation edges over the actual renamed checked bindings.
//! The automatically translated source decision determines final consumption.

use std::collections::{BTreeMap, BTreeSet};

use super::{Expr, Node, Type};

#[derive(Default)]
struct Facts<'a> {
    bindings: Vec<f64>,
    starts: Vec<f64>,
    counts: Vec<f64>,
    successors: Vec<f64>,
    names: BTreeMap<String, usize>,
    candidates: Vec<(&'a Expr, usize)>,
    recursive: Option<&'a str>,
}

fn fact(value: usize) -> f64 {
    f64::from(u32::try_from(value).expect("checked program occurrence limits fit in u32"))
}

impl<'a> Facts<'a> {
    fn name(&mut self, value: &str) -> usize {
        let next = self.names.len();
        *self.names.entry(value.to_owned()).or_insert(next)
    }

    fn event(&mut self, binding: usize, next: &[usize]) -> usize {
        let position = self.bindings.len();
        self.bindings.push(fact(binding));
        self.starts.push(fact(self.successors.len()));
        self.counts.push(fact(next.len()));
        self.successors.extend(next.iter().copied().map(fact));
        position
    }

    fn sequence(&mut self, nodes: &[&'a Expr], mut continuation: Vec<usize>) -> Vec<usize> {
        for child in nodes.iter().rev() {
            continuation = self.build(child, continuation);
        }
        continuation
    }

    fn build(&mut self, node: &'a Expr, continuation: Vec<usize>) -> Vec<usize> {
        match &node.node {
            Node::Var { name } => {
                let binding = self.name(name);
                let position = self.event(binding, &continuation);
                if matches!(
                    node.ty,
                    Type::Array {
                        reference: false,
                        ..
                    }
                ) {
                    self.candidates.push((node, position));
                }
                vec![position]
            }
            Node::Lambda { body, .. } => {
                // Closure creation clones its captured values. The closure
                // body later consumes those copies, not enclosing storage.
                let mut captured = BTreeSet::new();
                collect_names(body, &mut captured);
                let mut next = continuation;
                for name in captured.into_iter().rev() {
                    let binding = self.name(&name);
                    next = vec![self.event(binding, &next)];
                }
                next
            }
            Node::If {
                cond,
                then,
                otherwise,
            } => {
                let mut alternatives = self.build(then, continuation.clone());
                alternatives.extend(self.build(otherwise, continuation));
                self.build(cond, alternatives)
            }
            Node::Match { scrutinee, cases } => {
                let mut alternatives = Vec::new();
                for branch in cases {
                    alternatives.extend(self.build(&branch.body, continuation.clone()));
                }
                self.build(scrutinee, alternatives)
            }
            Node::Append { left, right } if left.var_name().is_some() => {
                let moved = self.build(left, continuation);
                self.build(right, moved)
            }
            Node::Call { func, args } if self.recursive == Some(func.as_str()) => {
                let mut next = continuation;
                for argument in args.iter().rev() {
                    next = self.build(argument, next);
                }
                next
            }
            _ => self.sequence(&node.children(), continuation),
        }
    }
}

fn collect_names(node: &Expr, names: &mut BTreeSet<String>) {
    if let Node::Var { name } = &node.node {
        names.insert(name.clone());
    } else {
        for child in node.children() {
            collect_names(child, names);
        }
    }
}

pub(super) fn final_uses(body: &Expr, recursive: Option<&str>) -> BTreeSet<*const Expr> {
    let mut facts = Facts {
        recursive,
        ..Facts::default()
    };
    facts.build(body, Vec::new());
    facts
        .candidates
        .iter()
        .filter_map(|(node, position)| {
            crate::translation::owned_array_append_rules::accept_owned_array_final_use(
                fact(*position),
                &facts.bindings,
                &facts.starts,
                &facts.counts,
                &facts.successors,
            )
            .then_some(std::ptr::from_ref(*node))
        })
        .collect()
}
