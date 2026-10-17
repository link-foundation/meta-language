//! Mechanical transport of consecutive source guards to the existing switch.

use super::{CaseTest, Span, Stmt, Switch, statement_span, tag_condition, terminates};
use crate::translation::javascript::Clause;
use crate::translation::javascript::surface_rules::{
    accept_java_script_tag_guard_sequence, accept_java_script_terminal_tag_guard,
};

/// Keep the complete common remainder until existing switch field projection.
pub(in crate::translation::javascript) fn tag_guard_sequence<'a>(
    statements: impl DoubleEndedIterator<Item = &'a Stmt> + Clone,
) -> Option<Stmt> {
    let first = statements.clone().next()?;
    let Stmt::If { cond, .. } = first else {
        return None;
    };
    let subject = tag_condition(cond)?;
    let last = statements.clone().next_back()?;
    let first_span = statement_span(first)?;
    let last_span = statement_span(last)?;
    let mut statements = statements.peekable();
    let mut clauses = Vec::new();
    let mut covered = std::collections::HashSet::new();
    while let Some(Stmt::If {
        cond,
        then,
        otherwise,
        span,
    }) = statements.peek().copied()
    {
        let Some(test) = tag_condition(cond) else {
            break;
        };
        // Parsed tag tests clone their actual declaration, including its
        // source span and constructor fields. Equal spellings alone cannot
        // let a different declaration or source binding borrow exclusions.
        let same_binding =
            test.object.simple_name() == subject.object.simple_name() && test.data == subject.data;
        if !accept_java_script_terminal_tag_guard(
            !test.negated,
            otherwise.is_some(),
            terminates(then),
            same_binding,
            covered.contains(test.tag.as_str()),
        ) {
            break;
        }
        covered.insert(test.tag.as_str());
        clauses.push(Clause {
            tests: vec![CaseTest::Tag {
                tag: test.tag.clone(),
                span: cond.span?,
            }],
            body: then.clone(),
            span: *span,
        });
        statements.next();
    }
    let remainder: Vec<Stmt> = statements.cloned().collect();
    if !accept_java_script_tag_guard_sequence(clauses.len() > 1, terminates(&remainder)) {
        return None;
    }
    let tail = Span {
        start: statement_span(remainder.first()?)?.start,
        end: last_span.end,
    };
    clauses.push(Clause {
        tests: vec![CaseTest::Default { span: tail }],
        body: remainder,
        span: tail,
    });
    Some(Stmt::Switch(Switch {
        scrutinee: subject.object.clone(),
        tag: Some((
            subject.object.simple_name()?.to_owned(),
            subject.data.clone(),
        )),
        clauses,
        span: Span {
            start: first_span.start,
            end: last_span.end,
        },
    }))
}
