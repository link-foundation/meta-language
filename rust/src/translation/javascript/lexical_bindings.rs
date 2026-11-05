//! Actual parser origins transported to the shared temporal-binding decision.

use super::callables::{number, position};
use super::tag_context::TagReceiverFact;
use super::{JavaScriptParser, Result, Span, Token, unsupported};
use crate::translation::{callable_source_rules, frontend_rules};
use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct Frame {
    parent: Option<usize>,
    boundary: u8,
    bindings: Vec<(String, Option<usize>)>,
    tag_bindings: HashMap<String, TagReceiverFact>,
    declaration_counts: HashMap<String, usize>,
    parameters: HashSet<String>,
}

struct Declaration {
    scope: usize,
    name: String,
    kind: u8,
    span: Span,
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
    declarations: Vec<Declaration>,
}

impl Default for Trace {
    fn default() -> Self {
        Self {
            frames: vec![Frame::default()],
            current: 0,
            reads: Vec::new(),
            declarations: Vec::new(),
        }
    }
}

impl Trace {
    pub(super) const fn read_count(&self) -> usize {
        self.reads.len()
    }

    /// Actual assignment observations; the shared profile decides admission.
    pub(super) fn assigned_names_since(&self, start: usize) -> impl Iterator<Item = &str> {
        self.reads[start..]
            .iter()
            .filter(|read| read.assignment)
            .map(|read| read.name.as_str())
    }

    pub(super) fn declaration_counts(&mut self, counts: HashMap<String, usize>) {
        self.frames[self.current].declaration_counts = counts;
    }

    pub(super) fn duplicate_declaration(&self, name: &str) -> bool {
        self.frames[self.current]
            .declaration_counts
            .get(name)
            .is_some_and(|count| *count > 1)
    }

    pub(super) fn declaration(&mut self, name: &str, span: Span, kind: u8) {
        self.declarations.push(Declaration {
            scope: self.current,
            name: name.to_owned(),
            span,
            kind,
        });
    }

    pub(super) fn parameter(&mut self, name: &str) {
        self.frames[self.current].parameters.insert(name.to_owned());
        self.binding(name, None);
    }

    pub(super) fn parent_parameter(&self, name: &str) -> bool {
        self.frames[self.current]
            .parent
            .is_some_and(|parent| self.frames[parent].parameters.contains(name))
    }

    pub(super) fn tag_binding(&mut self, name: &str, fact: TagReceiverFact) {
        self.frames[self.current]
            .tag_bindings
            .insert(name.to_owned(), fact);
    }

    pub(super) fn tag_fact(&self, name: &str) -> Option<TagReceiverFact> {
        let mut current = Some(self.current);
        while let Some(index) = current {
            let frame = &self.frames[index];
            let step =
                crate::translation::javascript_tag_context_rules::read_java_script_tag_binding_step(
                    frame.bindings.iter().any(|(bound, _)| bound == name),
                    f64::from(frame.boundary),
                );
            match frontend_rules::ml_array::number_index(step) {
                Some(1) => return frame.tag_bindings.get(name).cloned(),
                Some(2) => return None,
                _ => current = frame.parent,
            }
        }
        None
    }

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
        let parents = self
            .frames
            .iter()
            .map(|frame| frame.parent.map_or(-1.0, number))
            .collect::<Vec<_>>();
        let boundaries = self
            .frames
            .iter()
            .map(|frame| f64::from(frame.boundary))
            .collect::<Vec<_>>();
        let mut parameter_scopes = Vec::new();
        let mut parameter_names = Vec::new();
        for (scope, frame) in self.frames.iter().enumerate() {
            for name in &frame.parameters {
                parameter_scopes.push(number(scope));
                parameter_names.push(name.clone());
            }
        }
        let scopes = self
            .declarations
            .iter()
            .map(|row| number(row.scope))
            .collect::<Vec<_>>();
        let names = self
            .declarations
            .iter()
            .map(|row| row.name.clone())
            .collect::<Vec<_>>();
        let kinds = self
            .declarations
            .iter()
            .map(|row| f64::from(row.kind))
            .collect::<Vec<_>>();
        let plan=crate::translation::lexical_declaration_rules::find_java_script_lexical_declaration_conflict(&parents,&boundaries,&parameter_scopes,&parameter_names,&scopes,&names,&kinds);
        if let Some(index) = plan.first() {
            let declaration = &self.declarations[position(*index)];
            return Err(super::TranslationError::syntax(
                format!("duplicate lexical binding {}", declaration.name),
                Some(declaration.span),
            ));
        }
        self.validate_reads(None)
    }

    pub(super) fn validate_temporal(&self, token: &Token) -> Result<()> {
        self.validate_reads(Some(token))
    }

    fn validate_reads(&self, temporal: Option<&Token>) -> Result<()> {
        let selected = temporal.map(|token| Read {
            scope: self.current,
            name: token.value.clone(),
            start: token.start,
            span: super::span(token, token),
            assignment: false,
        });
        let reads = selected
            .as_ref()
            .map_or(self.reads.as_slice(), std::slice::from_ref);
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
                ends.push(
                    temporal
                        .filter(|token| token.value == *name)
                        .map_or_else(|| end.map_or(-1.0, number), |token| number(token.start + 1)),
                );
            }
        }
        let scopes = reads
            .iter()
            .map(|read| number(read.scope))
            .collect::<Vec<_>>();
        let read_names = reads
            .iter()
            .map(|read| read.name.clone())
            .collect::<Vec<_>>();
        let starts = reads
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
        let read = &reads[position(*index)];
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
            ..Frame::default()
        });
        for name in names {
            self.lexical.binding(name, None);
        }
        let result = parse(self);
        self.lexical.current = outer;
        result
    }
}
