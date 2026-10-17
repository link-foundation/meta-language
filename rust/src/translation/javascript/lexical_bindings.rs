//! Actual parser origins transported to the shared temporal-binding decision.

use super::callables::{number, position};
use super::{JavaScriptParser, Result, Span, Token, unsupported};
use crate::translation::{callable_source_rules, frontend_rules};

#[derive(Default)]
struct Frame {
    parent: Option<usize>,
    boundary: u8,
    bindings: Vec<(String, Option<usize>)>,
}

struct Read {
    scope: usize,
    name: String,
    start: usize,
    span: Span,
    assignment: bool,
}

pub(super) struct Trace {
    frames: Vec<Frame>,
    current: usize,
    reads: Vec<Read>,
}

impl Default for Trace {
    fn default() -> Self {
        Self {
            frames: vec![Frame::default()],
            current: 0,
            reads: Vec::new(),
        }
    }
}

impl Trace {
    pub(super) fn binding(&mut self, name: &str, initialized_at: Option<usize>) {
        let bindings = &mut self.frames[self.current].bindings;
        if let Some((_, end)) = bindings.iter_mut().find(|(bound, _)| bound == name) {
            *end = initialized_at.or(*end);
        } else {
            bindings.push((name.to_owned(), initialized_at));
        }
    }

    pub(super) fn read(&mut self, token: &Token, assignment: bool, span: Span) {
        self.reads.push(Read {
            scope: self.current,
            name: token.value.clone(),
            start: token.start,
            span,
            assignment,
        });
    }

    pub(super) fn validate(&self) -> Result<()> {
        let mut parents = Vec::new();
        let mut bounds = Vec::new();
        let mut names = Vec::new();
        let mut ends = Vec::new();
        for frame in &self.frames {
            parents.push(frame.parent.map_or(-1.0, number));
            bounds.extend([
                number(names.len()),
                number(frame.bindings.len()),
                f64::from(frame.boundary),
            ]);
            for (name, end) in &frame.bindings {
                names.push(name.clone());
                ends.push(end.map_or(-1.0, number));
            }
        }
        let scopes = self
            .reads
            .iter()
            .map(|read| number(read.scope))
            .collect::<Vec<_>>();
        let read_names = self
            .reads
            .iter()
            .map(|read| read.name.clone())
            .collect::<Vec<_>>();
        let starts = self
            .reads
            .iter()
            .map(|read| number(read.start))
            .collect::<Vec<_>>();
        let plan = frontend_rules::read_java_script_temporal_binding_read(
            &parents,
            &bounds,
            &names,
            &ends,
            &scopes,
            &read_names,
            &starts,
        );
        let Some(index) = plan.first() else {
            return Ok(());
        };
        let read = &self.reads[position(*index)];
        let issue = frontend_rules::ml_array::number_index(plan[1]);
        if issue == Some(2) {
            return Err(unsupported(
                &format!("function reading {}", read.name),
                "a mutable module binding requires module-capture representation",
                Some(read.span),
            ));
        }
        if issue == Some(1) {
            return Err(unsupported(
                &format!("closure reading {} before its declaration", read.name),
                "the captured binding may still be in its temporal dead zone; invoking the closure after initialization needs an invocation-order proof",
                Some(read.span),
            ));
        }
        if read.assignment {
            return Err(unsupported(
                &format!("assignment of {} before its declaration", read.name),
                "the binding is in its temporal dead zone, where assigning it throws a ReferenceError",
                Some(read.span),
            ));
        }
        Err(unsupported(
            &callable_source_rules::read_callable_reference_construct(&read.name, true),
            &callable_source_rules::read_callable_reference_obligation(true, true),
            Some(read.span),
        ))
    }
}

impl JavaScriptParser {
    pub(super) fn with_lexical_scope<T>(
        &mut self,
        names: &[String],
        boundary: u8,
        parse: impl FnOnce(&mut Self) -> Result<T>,
    ) -> Result<T> {
        let outer = self.lexical.current;
        self.lexical.current = self.lexical.frames.len();
        self.lexical.frames.push(Frame {
            parent: Some(outer),
            boundary,
            bindings: Vec::new(),
        });
        for name in names {
            self.lexical.binding(name, None);
        }
        let result = parse(self);
        self.lexical.current = outer;
        result
    }
}
