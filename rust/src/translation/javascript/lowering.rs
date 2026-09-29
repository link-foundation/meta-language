//! Lowering statement lists into expressions and tagged-union switches into matches.

use super::{
    node, type_error, unsupported, wild, AssertionKind, BinaryOp, CaseTest, HashSet, Result,
    SComparison, SData, SExpr, SNode, SParam, SPattern, SPatternNode, SProp, SPropNode, SRow,
    STagTest, Span, Stmt, Switch, TranslationError, UnaryOp, INT, OBJECT_PROTOTYPE, ROOT,
};

/// What `assert.method` asserts: the lookup is in plain objects, so the names
/// of `Object.prototype` find an inherited function and read as `notDeep`.
pub(super) fn assertion_kind(method: &str, strict: bool) -> Option<AssertionKind> {
    let kind = match method {
        "strictEqual" => AssertionKind::Eq,
        "notStrictEqual" => AssertionKind::Ne,
        "deepStrictEqual" => AssertionKind::Deep,
        "notDeepStrictEqual" => AssertionKind::NotDeep,
        _ if OBJECT_PROTOTYPE.contains(&method) => AssertionKind::NotDeep,
        "equal" if strict => AssertionKind::Eq,
        "notEqual" if strict => AssertionKind::Ne,
        "deepEqual" if strict => AssertionKind::Deep,
        "notDeepEqual" if strict => AssertionKind::NotDeep,
        _ => return None,
    };
    Some(kind)
}

/// Statements to one expression: `const` is `let`, `if` with a returning
/// branch selects between the branch and the rest, and every path must end
/// in `return` or `throw`.
pub(super) fn lower(statements: &[Stmt], place: Span) -> Result<SExpr> {
    let list: Vec<&Stmt> = statements
        .iter()
        .flat_map(|statement| match statement {
            Stmt::Block {
                body, inline: true, ..
            } => body.iter().collect(),
            other => vec![other],
        })
        .filter(|statement| !matches!(statement, Stmt::Empty))
        .collect();
    lower_at(&list, 0, place)
}

pub(super) const fn statement_span(statement: &Stmt) -> Option<Span> {
    match statement {
        Stmt::Const { span, .. }
        | Stmt::Return { span, .. }
        | Stmt::Throw { span, .. }
        | Stmt::Block { span, .. }
        | Stmt::If { span, .. }
        | Stmt::Let { span, .. }
        | Stmt::Assign { span, .. }
        | Stmt::While { span, .. }
        | Stmt::DoWhile { span, .. }
        | Stmt::For { span, .. }
        | Stmt::Break { span }
        | Stmt::Continue { span }
        | Stmt::Expr { span, .. }
        | Stmt::Print { span, .. } => Some(*span),
        Stmt::Switch(node) => Some(node.span),
        Stmt::Empty => None,
    }
}

pub(super) fn unreachable(list: &[&Stmt], index: usize) -> Result<()> {
    if index + 1 < list.len() {
        return Err(unsupported(
            "unreachable statement",
            "statements after return, throw or a complete if are never executed",
            statement_span(list[index + 1]),
        ));
    }
    Ok(())
}

pub(super) fn lower_at(list: &[&Stmt], index: usize, place: Span) -> Result<SExpr> {
    // A function that finishes without returning returns undefined, the unit value.
    let Some(statement) = list.get(index) else {
        return Ok(node(SNode::Unit, place));
    };
    match statement {
        Stmt::Const { name, value, span } => Ok(node(
            SNode::Let {
                name: name.clone(),
                ty: None,
                value: Box::new(value.clone()),
                body: Box::new(lower_at(list, index + 1, place)?),
            },
            *span,
        )),
        Stmt::Return { expr, .. } => {
            unreachable(list, index)?;
            Ok(expr.clone())
        }
        Stmt::Throw { message, span } => {
            unreachable(list, index)?;
            Ok(node(
                SNode::Abort {
                    message: message.clone(),
                },
                *span,
            ))
        }
        Stmt::Block { body, span, .. } => {
            if !terminates(body) {
                return Err(unsupported(
                    "block without a result",
                    "the block must end in return or throw",
                    Some(*span),
                ));
            }
            unreachable(list, index)?;
            lower(body, *span)
        }
        Stmt::If {
            cond,
            then,
            otherwise,
            span,
        } => {
            if !terminates(then) || otherwise.as_ref().is_some_and(|branch| !terminates(branch)) {
                return Err(unsupported(
                    "if branch without a result",
                    "every branch must end in return or throw; statements without effects are outside the portable core",
                    Some(*span),
                ));
            }
            let then = lower(then, *span)?;
            if let Some(test) = tag_condition(cond) {
                if otherwise.is_some() {
                    unreachable(list, index)?;
                }
                let other = match otherwise {
                    Some(branch) => lower(branch, *span)?,
                    None => lower_at(list, index + 1, place)?,
                };
                return lower_tag_if(test, then, other, *span);
            }
            let other = if let Some(branch) = otherwise {
                unreachable(list, index)?;
                lower(branch, *span)?
            } else {
                lower_at(list, index + 1, place)?
            };
            Ok(node(
                SNode::If {
                    cond: Box::new(cond.clone()),
                    then: Box::new(then),
                    otherwise: Box::new(other),
                },
                *span,
            ))
        }
        Stmt::Switch(switch) => lower_switch(switch, list, index, place),
        Stmt::Expr { span, .. } => Err(unsupported(
            "expression statement",
            "statements with effects are outside the portable core in function bodies",
            Some(*span),
        )),
        Stmt::Empty => Err(TranslationError::syntax("unknown statement empty", None)),
        // Bodies with these statements are lowered by `imperative`.
        Stmt::Let { span, .. }
        | Stmt::Assign { span, .. }
        | Stmt::While { span, .. }
        | Stmt::DoWhile { span, .. }
        | Stmt::For { span, .. }
        | Stmt::Break { span }
        | Stmt::Continue { span }
        | Stmt::Print { span, .. } => Err(TranslationError::syntax(
            "internal: an imperative statement in a pure body",
            Some(*span),
        )),
    }
}

pub(super) fn terminates(statements: &[Stmt]) -> bool {
    let last = statements
        .iter()
        .rev()
        .find(|statement| !matches!(statement, Stmt::Empty | Stmt::Block { inline: true, .. }));
    match last {
        Some(Stmt::Return { .. } | Stmt::Throw { .. }) => true,
        Some(Stmt::Block { body, .. }) => terminates(body),
        Some(Stmt::If {
            then,
            otherwise: Some(otherwise),
            ..
        }) => terminates(then) && terminates(otherwise),
        Some(Stmt::Switch(node)) => switch_terminates(node),
        _ => false,
    }
}

pub(super) fn switch_terminates(node: &Switch) -> bool {
    if !node.clauses.iter().all(|clause| terminates(&clause.body)) {
        return false;
    }
    if node.clauses.iter().any(|clause| {
        clause
            .tests
            .iter()
            .any(|test| matches!(test, CaseTest::Default { .. }))
    }) {
        return true;
    }
    let Some((_, data)) = &node.tag else {
        return false;
    };
    let tags = switch_tags(node);
    data.ctors
        .iter()
        .all(|ctor| tags.contains(&ctor.name.as_str()))
}

/// The tags a switch's cases name.
pub(super) fn switch_tags(node: &Switch) -> Vec<&str> {
    node.clauses
        .iter()
        .flat_map(|clause| &clause.tests)
        .filter_map(|test| match test {
            CaseTest::Tag { tag, .. } => Some(tag.as_str()),
            _ => None,
        })
        .collect()
}

/// The subject, data type and covered tags of a tag switch or tag test.
pub(super) struct TagSwitch<'a> {
    pub(super) subject: &'a str,
    pub(super) data: &'a SData,
    pub(super) covered: Vec<&'a str>,
}

/// A switch is a match. In a tag switch, `x.field` of the subject in a case
/// is a pattern variable of that case's constructor; the variables get
/// names that nothing in the case body uses, so no reference is captured.
pub(super) fn lower_switch(
    node: &Switch,
    list: &[&Stmt],
    index: usize,
    place: Span,
) -> Result<SExpr> {
    for clause in &node.clauses {
        if !terminates(&clause.body) {
            return Err(unsupported(
                "case without a result",
                "every case must end in return or throw; falling through is outside the portable core",
                Some(clause.span),
            ));
        }
    }
    let tag_switch = node.tag.as_ref().map(|(subject, data)| TagSwitch {
        subject,
        data,
        covered: switch_tags(node),
    });
    let mut rows = Vec::new();
    // `default` applies only when no case matches, wherever it stands.
    let mut defaults = Vec::new();
    let mut has_default = false;
    for clause in &node.clauses {
        for test in &clause.tests {
            let body = lower(&clause.body, clause.span)?;
            match (test, &tag_switch) {
                (CaseTest::Default { span }, tag_switch) => {
                    has_default = true;
                    defaults = match tag_switch {
                        Some(tag_switch) => default_rows(tag_switch, body, clause.span)?,
                        None => vec![SRow {
                            patterns: vec![wild(*span)],
                            body,
                            span: Some(clause.span),
                        }],
                    };
                }
                (CaseTest::Tag { tag, span }, Some(tag_switch)) => {
                    rows.push(tag_row(tag_switch, tag, *span, &body, clause.span)?);
                }
                (
                    CaseTest::NumLit {
                        value,
                        negative,
                        span,
                    },
                    _,
                ) => rows.push(SRow {
                    patterns: vec![SPattern {
                        node: SPatternNode::NumLit {
                            value: value.clone(),
                            negative: *negative,
                        },
                        span: Some(*span),
                    }],
                    body,
                    span: Some(clause.span),
                }),
                (CaseTest::BoolLit { value, span }, _) => rows.push(SRow {
                    patterns: vec![SPattern {
                        node: SPatternNode::BoolLit {
                            value: *value,
                            negative: false,
                        },
                        span: Some(*span),
                    }],
                    body,
                    span: Some(clause.span),
                }),
                (CaseTest::Tag { .. }, None) => {}
            }
        }
    }
    rows.extend(defaults);
    let at_end = index + 1 == list.len();
    if switch_terminates(node) {
        unreachable(list, index)?;
    } else if !at_end && !has_default {
        rows.push(SRow {
            patterns: vec![wild(node.span)],
            body: lower_at(list, index + 1, place)?,
            span: Some(node.span),
        });
    } else if has_default {
        unreachable(list, index)?;
    }
    Ok(SExpr::new(
        SNode::Match {
            scrutinees: vec![node.scrutinee.clone()],
            rows,
        },
        Some(node.span),
    ))
}

/// The `default` of a tag switch covers the remaining alternatives; when it
/// reads fields of the subject, it is one row per remaining alternative.
pub(super) fn default_rows(tag_switch: &TagSwitch, body: SExpr, place: Span) -> Result<Vec<SRow>> {
    if !reads_fields(&body, tag_switch.subject) {
        return Ok(vec![SRow {
            patterns: vec![wild(place)],
            body,
            span: Some(place),
        }]);
    }
    tag_switch
        .data
        .ctors
        .iter()
        .filter(|ctor| !tag_switch.covered.contains(&ctor.name.as_str()))
        .map(|ctor| tag_row(tag_switch, &ctor.name, place, &body, place))
        .collect()
}

/// `x.$ === 'tag'` on a local `x`: the data type and the tag, when it narrows `x`.
pub(super) fn tag_condition(cond: &SExpr) -> Option<&STagTest> {
    cond.tag_test
        .as_deref()
        .filter(|test| test.object.simple_name().is_some())
}

/// `if (x.$ === 'tag') A else B` is a match in which `A` reads the fields of `tag`.
pub(super) fn lower_tag_if(
    test: &STagTest,
    then: SExpr,
    otherwise: SExpr,
    place: Span,
) -> Result<SExpr> {
    let tag_switch = TagSwitch {
        subject: test.object.simple_name().unwrap_or_default(),
        data: &test.data,
        covered: vec![test.tag.as_str()],
    };
    let (matching, other) = if test.negated {
        (otherwise, then)
    } else {
        (then, otherwise)
    };
    let mut rows = vec![tag_row(&tag_switch, &test.tag, place, &matching, place)?];
    rows.extend(default_rows(&tag_switch, other, place)?);
    Ok(node(
        SNode::Match {
            scrutinees: vec![test.object.clone()],
            rows,
        },
        place,
    ))
}

pub(super) fn tag_row(
    tag_switch: &TagSwitch,
    tag: &str,
    test_span: Span,
    case_body: &SExpr,
    place: Span,
) -> Result<SRow> {
    let subject = tag_switch.subject;
    let data = tag_switch.data;
    let Some(ctor) = data.ctors.iter().find(|candidate| candidate.name == tag) else {
        return Err(type_error(
            format!("{} has no tag {tag}", data.name),
            Some(test_span),
        ));
    };
    let has_field = |field: &str| {
        ctor.fields
            .iter()
            .any(|candidate| candidate.name.as_deref() == Some(field))
    };
    // Field names to pattern variables, in order.
    let mut binders: Vec<(String, String)> = Vec::new();
    // `const { left, value } = t;` at the start of the case names the pattern variables itself.
    let mut rest = case_body;
    let mut direct: Vec<&str> = Vec::new();
    while let SNode::Let {
        name, value, body, ..
    } = &rest.node
    {
        let SNode::Field { object, field } = &value.node else {
            break;
        };
        if object.simple_name() != Some(subject)
            || !has_field(field)
            || binders.iter().any(|(bound, _)| bound == field)
            || name == subject
            || direct.contains(&name.as_str())
        {
            break;
        }
        binders.push((field.clone(), name.clone()));
        direct.push(name);
        rest = body;
    }
    let mut rebound = HashSet::new();
    collect_binders(rest, &mut rebound);
    if direct.iter().any(|name| rebound.contains(*name)) {
        binders.clear();
        rest = case_body;
    }
    let mut body = rest.clone();
    let mut used = HashSet::new();
    collect_names(&body, &mut used);
    for (_, name) in &binders {
        used.insert(name.clone());
    }
    let mut binder_for = |field: &str| -> Result<String> {
        if !has_field(field) {
            return Err(type_error(
                format!(
                    "the {tag} alternative of {} has no field {field}",
                    data.name
                ),
                Some(place),
            ));
        }
        if let Some((_, name)) = binders.iter().find(|(bound, _)| bound == field) {
            return Ok(name.clone());
        }
        let mut name = if used.contains(field) || field == subject {
            format!("{subject}_{field}")
        } else {
            field.to_owned()
        };
        let mut index = 2;
        while used.contains(&name) {
            name = format!("{subject}_{field}{index}");
            index += 1;
        }
        used.insert(name.clone());
        binders.push((field.to_owned(), name.clone()));
        Ok(name)
    };
    substitute_fields(&mut body, subject, &mut binder_for)?;
    let args = ctor
        .fields
        .iter()
        .map(|field| {
            let bound = binders
                .iter()
                .find(|(name, _)| Some(name.as_str()) == field.name.as_deref());
            SPattern {
                node: bound.map_or(SPatternNode::Wild, |(_, name)| SPatternNode::BindOrCtor {
                    name: name.clone(),
                }),
                span: Some(test_span),
            }
        })
        .collect();
    Ok(SRow {
        patterns: vec![SPattern {
            node: SPatternNode::Ctor {
                path: vec![ROOT.to_owned(), data.name.clone(), tag.to_owned()],
                args,
            },
            span: Some(test_span),
        }],
        body,
        span: Some(place),
    })
}

/// The subexpressions the JavaScript frontend's generic traversals visit.
/// They skip keys named `span` and `type`, so constructor-object fields with
/// those names are skipped too.
pub(super) fn children(expr: &SExpr) -> Vec<&SExpr> {
    let mut out: Vec<&SExpr> = Vec::new();
    match &expr.node {
        SNode::App { func, args } => {
            out.push(func);
            out.extend(args);
        }
        SNode::Field { object, .. } | SNode::Length { object, .. } => out.push(object),
        SNode::Index { object, index } => {
            out.push(object);
            out.push(index);
        }
        SNode::Array { items, .. } | SNode::Math { args: items, .. } => {
            out.extend(items.iter().map(|item| &item.value));
        }
        SNode::Unary { arg, .. } | SNode::ToString { arg } | SNode::Show { arg, .. } => {
            out.push(arg);
        }
        SNode::Binary { left, right, .. } => {
            out.push(left);
            out.push(right);
        }
        SNode::If {
            cond,
            then,
            otherwise,
        } => {
            out.push(cond);
            out.push(then);
            out.push(otherwise);
        }
        SNode::Let { value, body, .. } => {
            out.push(value);
            out.push(body);
        }
        SNode::Match { scrutinees, rows } => {
            out.extend(scrutinees);
            out.extend(rows.iter().map(|row| &row.body));
        }
        SNode::CtorObject { fields, .. } => {
            out.extend(
                fields
                    .iter()
                    .filter(|(name, _)| name != "span" && name != "type")
                    .map(|(_, value)| value),
            );
        }
        _ => {}
    }
    if let Some(test) = &expr.tag_test {
        out.push(&test.object);
    }
    out
}

/// The subexpressions of `children`, to rewrite.
pub(super) fn children_mut(expr: &mut SExpr) -> Vec<&mut SExpr> {
    let mut out: Vec<&mut SExpr> = Vec::new();
    match &mut expr.node {
        SNode::App { func, args } => {
            out.push(func);
            out.extend(args);
        }
        SNode::Field { object, .. } | SNode::Length { object, .. } => out.push(object),
        SNode::Index { object, index } => {
            out.push(object);
            out.push(index);
        }
        SNode::Array { items, .. } | SNode::Math { args: items, .. } => {
            out.extend(items.iter_mut().map(|item| &mut item.value));
        }
        SNode::Unary { arg, .. } | SNode::ToString { arg } | SNode::Show { arg, .. } => {
            out.push(arg);
        }
        SNode::Binary { left, right, .. } => {
            out.push(left);
            out.push(right);
        }
        SNode::If {
            cond,
            then,
            otherwise,
        } => {
            out.push(cond);
            out.push(then);
            out.push(otherwise);
        }
        SNode::Let { value, body, .. } => {
            out.push(value);
            out.push(body);
        }
        SNode::Match { scrutinees, rows } => {
            out.extend(scrutinees);
            out.extend(rows.iter_mut().map(|row| &mut row.body));
        }
        SNode::CtorObject { fields, .. } => {
            out.extend(
                fields
                    .iter_mut()
                    .filter(|(name, _)| name != "span" && name != "type")
                    .map(|(_, value)| value),
            );
        }
        _ => {}
    }
    if let Some(test) = &mut expr.tag_test {
        out.push(&mut test.object);
    }
    out
}

/// The names patterns bind.
pub(super) fn pattern_names(pattern: &SPattern, names: &mut HashSet<String>) {
    match &pattern.node {
        SPatternNode::BindOrCtor { name } | SPatternNode::Bind { name } => {
            names.insert(name.clone());
        }
        SPatternNode::Ctor { args, .. } => {
            for arg in args {
                pattern_names(arg, names);
            }
        }
        _ => {}
    }
}

pub(super) fn row_pattern_names(expr: &SExpr, names: &mut HashSet<String>) {
    if let SNode::Match { rows, .. } = &expr.node {
        for pattern in rows.iter().flat_map(|row| &row.patterns) {
            pattern_names(pattern, names);
        }
    }
}

/// Every name a surface expression binds or mentions.
pub(super) fn collect_names(expr: &SExpr, names: &mut HashSet<String>) {
    match &expr.node {
        SNode::Name { path } if path.len() == 1 => {
            names.insert(path[0].clone());
        }
        SNode::Let { name, .. } => {
            names.insert(name.clone());
        }
        _ => {}
    }
    row_pattern_names(expr, names);
    for child in children(expr) {
        collect_names(child, names);
    }
}

/// Every name a surface expression binds.
pub(super) fn collect_binders(expr: &SExpr, names: &mut HashSet<String>) {
    if let SNode::Let { name, .. } = &expr.node {
        names.insert(name.clone());
    }
    row_pattern_names(expr, names);
    for child in children(expr) {
        collect_binders(child, names);
    }
}

/// Whether an expression reads a field of `subject`.
pub(super) fn reads_fields(expr: &SExpr, subject: &str) -> bool {
    if let SNode::Field { object, .. } = &expr.node {
        if object.simple_name() == Some(subject) {
            return true;
        }
    }
    children(expr)
        .into_iter()
        .any(|child| reads_fields(child, subject))
}

/// Replaces `subject.field` by the case's pattern variables, up to a rebinding of `subject`.
pub(super) fn substitute_fields(
    expr: &mut SExpr,
    subject: &str,
    binder_for: &mut dyn FnMut(&str) -> Result<String>,
) -> Result<()> {
    if let SNode::Field { object, field } = &expr.node {
        if object.simple_name() == Some(subject) {
            let name = binder_for(field)?;
            expr.node = SNode::Name { path: vec![name] };
            return Ok(());
        }
    }
    match &mut expr.node {
        SNode::Let {
            name, value, body, ..
        } => {
            substitute_fields(value, subject, binder_for)?;
            if name != subject {
                substitute_fields(body, subject, binder_for)?;
            }
        }
        SNode::Match { scrutinees, rows } => {
            for scrutinee in scrutinees {
                substitute_fields(scrutinee, subject, binder_for)?;
            }
            for row in rows {
                let mut bound = HashSet::new();
                for pattern in &row.patterns {
                    pattern_names(pattern, &mut bound);
                }
                if !bound.contains(subject) {
                    substitute_fields(&mut row.body, subject, binder_for)?;
                }
            }
        }
        _ => {
            for child in children_mut(expr) {
                substitute_fields(child, subject, binder_for)?;
            }
        }
    }
    Ok(())
}

/// A proposition from an asserted condition; `===` on objects compares identities.
pub(super) fn prop_of(expr: SExpr) -> SProp {
    let node = match expr.node {
        SNode::Binary {
            op, left, right, ..
        } if matches!(
            op,
            BinaryOp::Eq | BinaryOp::Ne | BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge
        ) =>
        {
            let comparison = SComparison {
                left: *left,
                right: *right,
                reference: matches!(op, BinaryOp::Eq | BinaryOp::Ne),
                same_value: false,
            };
            match op {
                BinaryOp::Eq => SPropNode::Eq(comparison),
                BinaryOp::Ne => SPropNode::Ne(comparison),
                BinaryOp::Lt => SPropNode::Lt(comparison),
                BinaryOp::Le => SPropNode::Le(comparison),
                BinaryOp::Gt => SPropNode::Gt(comparison),
                _ => SPropNode::Ge(comparison),
            }
        }
        SNode::Binary {
            op: BinaryOp::And,
            left,
            right,
            ..
        } => SPropNode::And {
            left: Box::new(prop_of(*left)),
            right: Box::new(prop_of(*right)),
        },
        SNode::Binary {
            op: BinaryOp::Or,
            left,
            right,
            ..
        } => SPropNode::Or {
            left: Box::new(prop_of(*left)),
            right: Box::new(prop_of(*right)),
        },
        SNode::Unary {
            op: UnaryOp::Not,
            arg,
        } => SPropNode::Not {
            arg: Box::new(prop_of(*arg)),
        },
        node => SPropNode::Bool {
            expr: SExpr { node, ..expr },
        },
    };
    SProp { node, span: None }
}

/// The parameter a leading `if (n < 0n) throw …` restricts to the naturals.
pub(super) fn guarded_parameter(statement: &Stmt, params: &[SParam]) -> Option<usize> {
    let Stmt::If {
        cond,
        then,
        otherwise: None,
        ..
    } = statement
    else {
        return None;
    };
    let then: Vec<&Stmt> = then
        .iter()
        .filter(|inner| !matches!(inner, Stmt::Empty))
        .collect();
    if !matches!(then.as_slice(), [Stmt::Throw { .. }]) {
        return None;
    }
    let SNode::Binary {
        op, left, right, ..
    } = &cond.node
    else {
        return None;
    };
    let is_zero = |expr: &SExpr| matches!(&expr.node, SNode::Num { value, .. } if value == "0");
    let (variable, zero) = match op {
        BinaryOp::Lt if is_zero(right) => (left, right),
        BinaryOp::Gt if is_zero(left) => (right, left),
        _ => return None,
    };
    let name = variable.simple_name()?;
    // An undeclared parameter compared with the BigInt 0n is a bigint.
    let bigint_zero = matches!(&zero.node, SNode::Num { ty: None, .. });
    params
        .iter()
        .position(|param| param.name == name)
        .filter(|&index| {
            params[index].ty == Some(INT) || (params[index].ty.is_none() && bigint_zero)
        })
}
