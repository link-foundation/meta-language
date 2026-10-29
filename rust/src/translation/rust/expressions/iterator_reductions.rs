//! Actual standard iterator syntax/source facts into shared reduction plans.
use super::{
    BOOL, Options, Result, RustParser, SExpr, SNode, SourceOrigin, Span, Token, Type, unsupported,
};
use crate::translation::rust_array_index_rules::read_rust_scalar_array_index_receiver_error;
use crate::translation::rust_array_lookup_rules::read_rust_array_lookup_element_owner_error;
use crate::translation::rust_iterator_reduction_rules as rules;
use crate::translation::source_signatures::SourceOwner;
use std::collections::BTreeMap;
fn count(value: usize) -> f64 {
    value as f64
}
fn fail(reason: &str, range: Option<Span>) -> crate::translation::diagnostics::TranslationError {
    unsupported("Rust iterator reduction", reason, range)
}
fn boolean(path: &[String]) -> SourceOrigin {
    let mut origin = SourceOrigin::with_type(BOOL, path, Vec::new());
    origin.source_owner = Some(SourceOwner {
        kind: "primitive".to_owned(),
        path: vec!["bool".to_owned()],
        arguments: Vec::new(),
    });
    origin
}
impl RustParser {
    pub(super) fn iterator_reduction(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        args: &[SExpr],
        mut range: Option<Span>,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        if !rules::accept_rust_iterator_reduction_start(
            &method.value,
            count(args.len()),
            self.ordinary_completion_context(),
        ) {
            return Ok(None);
        }
        let actual = self.origins.origin(receiver, path);
        let source = actual.static_byte_array_read_view().unwrap_or(actual);
        let element = match &source.ty {
            Some(Type::Array { element, .. }) => Some(element.as_ref()),
            _ => None,
        };
        let owner = super::array_lookups::source_owners::element_owner(
            source
                .expanded_source_owner
                .as_ref()
                .or(source.source_owner.as_ref()),
        );
        let known_receiver = read_rust_scalar_array_index_receiver_error(
            source.ty.as_ref().map_or("", Type::kind),
            element.map_or("", Type::kind),
            source.array_source_model.unwrap_or(""),
            source.proven,
            source
                .wrappers
                .iter()
                .all(|layer| layer == "reference" || layer == "box"),
            source.array_element_copy_evidence,
        )
        .is_empty();
        let known_owner = if let (Some(element), Some(owner)) = (element, owner) {
            let (bits, signed) = match element {
                Type::Fixed { bits, signed } => (bits.to_string(), *signed),
                _ => (String::new(), false),
            };
            owner.arguments.is_empty()
                && (owner.kind == "primitive" || owner.kind == "unit" && owner.path.is_empty())
                && read_rust_array_lookup_element_owner_error(
                    element.kind(),
                    &bits,
                    signed,
                    &owner.path.join("::"),
                )
                .is_empty()
        } else {
            false
        };
        let error = rules::read_rust_iterator_reduction_receiver_error(known_receiver, known_owner);
        if !error.is_empty() {
            return Err(fail(&error, range));
        }
        let element = element.unwrap().clone();
        let owner = owner.unwrap().clone();
        if self.cursor.eat(".").is_none() || !self.cursor.is_kind(super::TokenKind::Identifier) {
            return Err(fail(
                "an iterator requires its immediate copied or cloned reduction",
                range,
            ));
        }
        let first = self.cursor.advance();
        let borrowed = !rules::accept_rust_iterator_copy_adapter(&first.value);
        let consumer = if borrowed {
            first
        } else {
            self.cursor.expect("(", Some("iterator copy adapter"))?;
            self.cursor.expect(")", Some("iterator copy adapter"))?;
            if self.cursor.eat(".").is_none() || !self.cursor.is_kind(super::TokenKind::Identifier)
            {
                return Err(fail(
                    "an iterator requires its complete immediate reduction",
                    range,
                ));
            }
            self.cursor.advance()
        };
        let operation = rules::read_rust_iterator_reduction_operation(&consumer.value);
        if operation.is_empty() {
            return Err(fail(
                "the complete standard iterator reduction needs a registered operation",
                Some(consumer.span()),
            ));
        }
        self.cursor.expect("(", Some("iterator reduction"))?;
        let mut initial = None;
        let mut initial_origin = None;
        if consumer.value == "fold" {
            let value = self.expr(path, Options::default())?;
            let origin = self.origins.origin(&value, path);
            let error = rules::read_rust_iterator_reduction_accumulator_error(
                origin.ty.is_some() && origin.proven,
                origin.direct_copy_value(),
                count(origin.wrappers.len()),
            );
            if !error.is_empty() {
                return Err(fail(&error, value.span));
            }
            self.cursor.expect(",", Some("iterator fold"))?;
            initial = Some(value);
            initial_origin = Some(origin);
        }
        let mut item = SourceOrigin::with_type(element, path, Vec::new());
        item.source_owner = Some(owner);
        if borrowed {
            item.wrappers.push("reference".to_owned());
            item.source_lifetimes.push(None);
        }
        let result = initial_origin.clone().unwrap_or_else(|| boolean(path));
        let parameters = if let Some(accumulator) = &initial_origin {
            vec![accumulator.clone(), item]
        } else {
            vec![item]
        };
        let mut expected = SourceOrigin::with_type(
            Type::Function {
                params: parameters
                    .iter()
                    .map(|parameter| parameter.ty.clone().unwrap())
                    .collect(),
                ret: Box::new(result.ty.clone().unwrap()),
            },
            path,
            Vec::new(),
        );
        expected.copy_evidence = Some(());
        expected.callable_parameters = Some(parameters.clone());
        expected.callable_result = Some(Box::new(result.clone()));
        let callback = self.read_expected_copy_closure(path, Some(&expected))?;
        self.cursor.eat(",");
        let close = self.cursor.expect(")", Some("iterator reduction"))?;
        range = Some(Span::new(
            receiver.span.map_or(method.start, |span| span.start),
            close.end,
        ));
        let callable = self.origins.origin(&callback, path);
        let actual_parameters = callable.callable_parameters.as_deref().unwrap_or(&[]);
        let known = actual_parameters
            .iter()
            .map(SourceOrigin::copy_closure_parameter)
            .collect::<Vec<_>>();
        let count_parameters = match &callable.ty {
            Some(Type::Function { params, .. }) => count(params.len()),
            _ => -1.0,
        };
        let error = rules::read_rust_iterator_reduction_callback_error(
            &consumer.value,
            callable.ty.as_ref().map_or("", Type::kind),
            count_parameters,
            count(callable.wrappers.len()),
            &known,
            callable
                .callable_result
                .as_ref()
                .is_some_and(|result| result.proven && result.ty.is_some()),
        );
        if !error.is_empty() {
            return Err(fail(&error, callback.span));
        }
        for (actual, expected) in actual_parameters.iter().zip(&parameters) {
            self.origins.argument(actual, expected, callback.span)?;
        }
        self.origins.argument(
            callable.callable_result.as_deref().unwrap(),
            &result,
            callback.span,
        )?;
        let mut values = BTreeMap::from([
            (
                "receiver".to_owned(),
                (receiver.clone(), source.ty.clone().unwrap()),
            ),
            ("callback".to_owned(), (callback, callable.ty.unwrap())),
        ]);
        if let (Some(initial), Some(origin)) = (initial, initial_origin) {
            values.insert("initial".to_owned(), (initial, origin.ty.unwrap()));
        }
        let roles = rules::read_rust_iterator_reduction_capture_roles(&consumer.value);
        let mut captures = Vec::new();
        let mut names = BTreeMap::new();
        for role in &roles {
            let name = self.fresh_product_binding();
            let (value, ty) = values.remove(role).ok_or_else(|| {
                fail(
                    "a reduction capture plan lost an actual source operand",
                    range,
                )
            })?;
            names.insert(role.clone(), name.clone());
            captures.push((name, ty, value));
        }
        let local = |role: &str| {
            SExpr::new(
                SNode::Name {
                    path: vec![names[role].clone()],
                },
                range,
            )
        };
        let mut arguments = vec![Some(local("callback"))];
        if consumer.value == "fold" {
            arguments.push(Some(local("initial")));
        }
        let mut body = SExpr::new(
            SNode::Method {
                object: Box::new(local("receiver")),
                op: operation,
                args: arguments,
            },
            range,
        );
        for (name, ty, value) in captures.into_iter().rev() {
            body = SExpr::new(
                SNode::Let {
                    name,
                    ty: Some(ty.into()),
                    value: Box::new(value),
                    body: Box::new(body),
                },
                range,
            );
        }
        self.origins.mark(&body, result);
        Ok(Some(body))
    }
}
