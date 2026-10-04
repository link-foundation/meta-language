//! Recursion analysis and structural natural-number decomposition.

use super::{
    BinaryOp, Case, Decimal, Decl, Expr, FnDecl, HashMap, HashSet, LitValue, NAT, Node, Pattern,
    Semantics, Type, binary_node, text_lit,
};

/// Inside `match v with | succ k => …`, the value `k + 1` is `v` itself.
/// Rewriting it to `v` keeps recursion such as `fib (n + 1) + fib n`
/// structural for Lean and Rocq without changing any value.
pub(super) fn reuse_successors(expr: &Expr, predecessors: &HashMap<String, String>) -> Expr {
    match &expr.node {
        Node::Binary {
            op: BinaryOp::Add,
            left,
            right,
            ..
        } if expr.ty == Type::Nat
            && left
                .var_name()
                .is_some_and(|name| predecessors.contains_key(name))
            && matches!(&right.node, Node::Lit { value: LitValue::Text(one) } if one == "1") =>
        {
            let name = &predecessors[left.var_name().unwrap_or_default()];
            Expr::var(name.clone(), expr.ty.clone()).with_span(expr.span)
        }
        Node::Let { name, value, body } => {
            let inner = without(predecessors, &[name.as_str()]);
            Expr {
                node: Node::Let {
                    name: name.clone(),
                    value: Box::new(reuse_successors(value, predecessors)),
                    body: Box::new(reuse_successors(body, &inner)),
                },
                ..expr.clone()
            }
        }
        Node::Match { scrutinee, cases } => {
            let cases = cases
                .iter()
                .map(|kase| {
                    let bound: Vec<&str> = match &kase.pattern {
                        Pattern::NatSucc { name } => vec![name.as_str()],
                        Pattern::Ctor { binds, .. } => {
                            binds.iter().flatten().map(String::as_str).collect()
                        }
                        _ => Vec::new(),
                    };
                    let mut inner = without(predecessors, &bound);
                    if let (Pattern::NatSucc { name }, Some(variable)) =
                        (&kase.pattern, scrutinee.var_name())
                        && !bound.contains(&variable)
                    {
                        inner.insert(name.clone(), variable.to_owned());
                    }
                    Case {
                        pattern: kase.pattern.clone(),
                        body: reuse_successors(&kase.body, &inner),
                    }
                })
                .collect();
            Expr {
                node: Node::Match {
                    scrutinee: Box::new(reuse_successors(scrutinee, predecessors)),
                    cases,
                },
                ..expr.clone()
            }
        }
        _ => expr.map_children(&mut |child| reuse_successors(child, predecessors)),
    }
}

pub(super) fn without(map: &HashMap<String, String>, names: &[&str]) -> HashMap<String, String> {
    map.iter()
        .filter(|(key, value)| !names.contains(&key.as_str()) && !names.contains(&value.as_str()))
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect()
}

/// Marks each function recursive or not and, for recursive functions, finds a
/// structurally decreasing parameter. Without one, Lean writes a partial def and
/// Rocq a Definition over `ml_fix`; the other targets accept general recursion.
pub(super) fn analyse_recursion(declarations: &mut [Decl]) {
    for decl in declarations.iter_mut() {
        let Decl::Fn(entry) = decl else { continue };
        let mut calls = Vec::new();
        collect_calls(&entry.body, &mut calls);
        entry.recursive = calls.contains(&entry.full_name);
        entry.decreasing = if entry.recursive {
            structural_parameter(entry)
        } else {
            None
        };
        if entry.recursive && entry.decreasing.is_none() {
            expose_natural_cases(entry);
        }
    }
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for decl in declarations.iter() {
        let Decl::Fn(entry) = decl else { continue };
        let mut calls = Vec::new();
        collect_calls(&entry.body, &mut calls);
        graph.insert(
            entry.full_name.clone(),
            calls
                .into_iter()
                .filter(|call| *call != entry.full_name)
                .collect(),
        );
    }
    for decl in declarations.iter_mut() {
        let Decl::Fn(entry) = decl else { continue };
        let targets = &graph[&entry.full_name];
        if targets
            .iter()
            .any(|target| reaches(&graph, target, &entry.full_name, &mut HashSet::new()))
        {
            entry.mutual = true;
        }
    }
}

pub(super) fn reaches(
    graph: &HashMap<String, HashSet<String>>,
    from: &str,
    to: &str,
    seen: &mut HashSet<String>,
) -> bool {
    if from == to {
        return true;
    }
    if !seen.insert(from.to_owned()) {
        return false;
    }
    graph
        .get(from)
        .is_some_and(|targets| targets.iter().any(|next| reaches(graph, next, to, seen)))
}

/// The functions an expression calls, in evaluation order.
pub fn collect_calls(expr: &Expr, calls: &mut Vec<String>) {
    if let Node::Call { func, .. } = &expr.node {
        calls.push(func.clone());
    }
    for child in expr.children() {
        collect_calls(child, calls);
    }
}

pub(super) fn structural_parameter(entry: &FnDecl) -> Option<usize> {
    entry.params.iter().enumerate().position(|(index, param)| {
        decreases_on(
            &entry.body,
            &entry.full_name,
            index,
            &param.name,
            &HashSet::new(),
        )
    })
}

/// Every recursive call's argument at `index` must be a strict subterm of `name`.
pub(super) fn decreases_on(
    expr: &Expr,
    func: &str,
    index: usize,
    name: &str,
    subterms: &HashSet<String>,
) -> bool {
    match &expr.node {
        Node::Call { func: callee, args } => {
            if callee == func {
                let decreasing = args
                    .get(index)
                    .and_then(Expr::var_name)
                    .is_some_and(|arg| subterms.contains(arg));
                if !decreasing {
                    return false;
                }
            }
            args.iter()
                .all(|arg| decreases_on(arg, func, index, name, subterms))
        }
        Node::Match { scrutinee, cases } => {
            if !decreases_on(scrutinee, func, index, name, subterms) {
                return false;
            }
            let is_param = scrutinee
                .var_name()
                .is_some_and(|variable| variable == name || subterms.contains(variable));
            cases.iter().all(|kase| {
                let mut inner = subterms.clone();
                if is_param {
                    match &kase.pattern {
                        Pattern::NatSucc { name } => {
                            inner.insert(name.clone());
                        }
                        Pattern::Ctor { binds, .. } => {
                            inner.extend(binds.iter().flatten().cloned());
                        }
                        _ => {}
                    }
                }
                decreases_on(&kase.body, func, index, name, &inner)
            })
        }
        Node::Let {
            name: local,
            value,
            body,
        } => {
            if !decreases_on(value, func, index, name, subterms) {
                return false;
            }
            let mut inner = subterms.clone();
            inner.remove(local);
            // `let k := subterm` (introduced by pattern compilation) is itself a subterm.
            if value
                .var_name()
                .is_some_and(|variable| subterms.contains(variable))
            {
                inner.insert(local.clone());
            }
            decreases_on(body, func, index, name, &inner)
        }
        _ => expr
            .children()
            .into_iter()
            .all(|child| decreases_on(child, func, index, name, subterms)),
    }
}

/// `if n == 0 then a else b` on a natural or unsigned machine-integer
/// parameter is the case split `match n with 0 => a | succ p => b'`, where
/// `b'` reads `n - k` as `p - (k - 1)` and `n == k` as `p == k - 1`: in `b`,
/// `n` is at least 1, so both agree with truncated subtraction, with checked
/// machine subtraction, and with a checked conversion of `n - k` back to a
/// natural. A `let m := n` in `b` is an alias read the same way. Writing the
/// split explicitly makes recursion such as `n * fact (n - 1)` structural,
/// which Lean and Rocq require.
pub(super) fn expose_natural_cases(entry: &mut FnDecl) {
    for index in 0..entry.params.len() {
        let param = &entry.params[index];
        if !param.ty.is_natural() {
            continue;
        }
        let mut fresh = 0;
        let Some(body) = split_natural(&entry.body, &param.name, &mut fresh) else {
            continue;
        };
        if decreases_on(&body, &entry.full_name, index, &param.name, &HashSet::new()) {
            entry.body = body;
            entry.decreasing = Some(index);
            return;
        }
    }
}

/// A natural literal's value.
pub(super) fn nat_literal(expr: &Expr) -> Option<Decimal> {
    match &expr.node {
        Node::Lit {
            value: LitValue::Text(text),
        } if !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit()) => {
            Decimal::parse(text)
        }
        _ => None,
    }
}

/// `k - 1` for a positive canonical decimal `k`.
pub(super) fn decrement(value: &Decimal) -> String {
    let mut digits: Vec<u8> = value.digits.bytes().collect();
    for digit in digits.iter_mut().rev() {
        if *digit == b'0' {
            *digit = b'9';
        } else {
            *digit -= 1;
            break;
        }
    }
    let text = String::from_utf8(digits).unwrap_or_default();
    let trimmed = text.trim_start_matches('0');
    if trimmed.is_empty() {
        "0".to_owned()
    } else {
        trimmed.to_owned()
    }
}

/// `n == 0`, `0 == n`: whether the zero test selects the `then` branch.
pub(super) fn zero_test(cond: &Expr, name: &str) -> Option<bool> {
    let Node::Binary {
        op: op @ (BinaryOp::Eq | BinaryOp::Ne),
        left,
        right,
        domain: Some(domain),
        ..
    } = &cond.node
    else {
        return None;
    };
    if !domain.is_natural() {
        return None;
    }
    let zero_on = |a: &Expr, b: &Expr| {
        a.var_name() == Some(name) && nat_literal(b).is_some_and(|value| value.digits == "0")
    };
    (zero_on(left, right) || zero_on(right, left)).then_some(*op == BinaryOp::Eq)
}

/// The locals a match binds in any case.
pub(super) fn binds(cases: &[Case], name: &str) -> bool {
    cases
        .iter()
        .any(|kase| kase.pattern.bound().contains(&name))
}

/// Splits zero tests on `name` into natural case splits; `None` when nothing changed.
pub(super) fn split_natural(expr: &Expr, name: &str, fresh: &mut usize) -> Option<Expr> {
    if let Node::If {
        cond,
        then,
        otherwise,
    } = &expr.node
        && let Some(zero_then) = zero_test(cond, name)
    {
        let (zero, positive) = if zero_then {
            (then, otherwise)
        } else {
            (otherwise, then)
        };
        *fresh += 1;
        let pred = format!("ml_p{fresh}");
        let ty = match &cond.node {
            Node::Binary {
                domain: Some(domain),
                ..
            } => domain.clone(),
            _ => NAT,
        };
        let succ_body = predecessor_of(positive, name, &pred, &ty);
        let succ_body = split_or_same(&succ_body, &pred, fresh);
        let succ_body = split_or_same(&succ_body, name, fresh);
        let zero = split_or_same(zero, name, fresh);
        return Some(Expr {
            node: Node::Match {
                scrutinee: Box::new(Expr::var(name, ty).with_span(cond.span)),
                cases: vec![
                    Case {
                        pattern: Pattern::NatZero,
                        body: zero,
                    },
                    Case {
                        pattern: Pattern::NatSucc { name: pred },
                        body: succ_body,
                    },
                ],
            },
            ty: expr.ty.clone(),
            span: expr.span,
        });
    }
    match &expr.node {
        Node::Let { name: local, .. } if local == name => return None,
        Node::Match { cases, .. } if binds(cases, name) => return None,
        _ => {}
    }
    let mut changed = false;
    let copy = expr.map_children(&mut |child| {
        split_natural(child, name, fresh).map_or_else(
            || child.clone(),
            |next| {
                changed = true;
                next
            },
        )
    });
    changed.then_some(copy)
}

pub(super) fn split_or_same(expr: &Expr, name: &str, fresh: &mut usize) -> Expr {
    split_natural(expr, name, fresh).unwrap_or_else(|| expr.clone())
}

/// Rewrites uses of `name - k` and `name == k` (k ≥ 1) in terms of
/// `pred = name - 1`, including through `let` aliases of `name`.
pub(super) fn predecessor_of(expr: &Expr, name: &str, pred: &str, ty: &Type) -> Expr {
    let names = HashSet::from([name.to_owned()]);
    Predecessor { pred, ty }.visit(expr, &names)
}

pub(super) struct Predecessor<'a> {
    pred: &'a str,
    ty: &'a Type,
}

impl Predecessor<'_> {
    fn pred_var(&self) -> Expr {
        Expr::var(self.pred, self.ty.clone())
    }

    fn minus(&self, k: &Decimal, like: &Expr) -> Expr {
        if k.digits == "1" {
            return self.pred_var();
        }
        let Node::Binary {
            op,
            semantics,
            rounding,
            by_zero,
            ..
        } = &like.node
        else {
            return like.clone();
        };
        Expr {
            node: binary_node(
                *op,
                Box::new(self.pred_var()),
                Box::new(text_lit(self.ty.clone(), decrement(k))),
                Some(self.ty.clone()),
                *semantics,
                *rounding,
                *by_zero,
            ),
            ty: self.ty.clone(),
            span: like.span,
        }
    }

    fn visit(&self, node: &Expr, names: &HashSet<String>) -> Expr {
        let is_alias = |candidate: &Expr| {
            candidate
                .var_name()
                .is_some_and(|name| names.contains(name))
        };
        if let Node::Binary {
            op,
            left,
            right,
            domain: Some(domain),
            semantics,
            rounding,
            by_zero,
        } = &node.node
            && domain.is_natural()
            && is_alias(left)
            && let Some(k) = nat_literal(right).filter(|k| k.digits != "0")
        {
            if *op == BinaryOp::Sub {
                return self.minus(&k, node);
            }
            if matches!(op, BinaryOp::Eq | BinaryOp::Ne) {
                return Expr {
                    node: binary_node(
                        *op,
                        Box::new(self.pred_var()),
                        Box::new(text_lit(self.ty.clone(), decrement(&k))),
                        Some(domain.clone()),
                        *semantics,
                        *rounding,
                        *by_zero,
                    ),
                    ..node.clone()
                };
            }
        }
        // JavaScript's `n - 1n` on a natural `n` is the checked conversion of
        // an integer difference; with n ≥ 1 it is the natural predecessor.
        if let Node::Cast {
            arg, to: Type::Nat, ..
        } = &node.node
            && let Node::Binary {
                op: BinaryOp::Sub,
                left,
                right,
                ..
            } = &arg.node
            && let Node::Cast {
                arg: inner,
                from: Type::Nat,
                ..
            } = &left.node
            && is_alias(inner)
            && let Some(k) = nat_literal(right).filter(|k| k.digits != "0")
        {
            let like = Expr {
                node: binary_node(
                    BinaryOp::Sub,
                    Box::new(self.pred_var()),
                    Box::new(self.pred_var()),
                    None,
                    Some(Semantics::Truncated),
                    None,
                    None,
                ),
                ty: self.ty.clone(),
                span: node.span,
            };
            return self.minus(&k, &like);
        }
        match &node.node {
            Node::Let {
                name: local,
                value,
                body,
            } => {
                let mut inner = names.clone();
                inner.remove(local);
                let value_visited = self.visit(value, names);
                if local == self.pred {
                    return Expr {
                        node: Node::Let {
                            name: local.clone(),
                            value: Box::new(value_visited),
                            body: body.clone(),
                        },
                        ..node.clone()
                    };
                }
                if is_alias(value) {
                    inner.insert(local.clone());
                }
                Expr {
                    node: Node::Let {
                        name: local.clone(),
                        value: Box::new(value_visited),
                        body: Box::new(self.visit(body, &inner)),
                    },
                    ..node.clone()
                }
            }
            Node::Match { scrutinee, cases } => {
                let scrutinee = Box::new(self.visit(scrutinee, names));
                if binds(cases, self.pred) {
                    return Expr {
                        node: Node::Match {
                            scrutinee,
                            cases: cases.clone(),
                        },
                        ..node.clone()
                    };
                }
                let cases = cases
                    .iter()
                    .map(|kase| {
                        let bound = kase.pattern.bound();
                        let inner: HashSet<String> = names
                            .iter()
                            .filter(|candidate| !bound.contains(&candidate.as_str()))
                            .cloned()
                            .collect();
                        Case {
                            pattern: kase.pattern.clone(),
                            body: self.visit(&kase.body, &inner),
                        }
                    })
                    .collect();
                Expr {
                    node: Node::Match { scrutinee, cases },
                    ..node.clone()
                }
            }
            _ => node.map_children(&mut |child| self.visit(child, names)),
        }
    }
}
