//! Calls use the actual Rust body and the checked callback completion packet.

use super::{
    Result, RustParser, SExpr, SNode, SourceOrigin, Span, Token, Type,
    borrowed_vectors::source_type_position, fn_mut_source_declarations::FnMutSourceDeclaration,
    fn_mut_source_headers::FnMutSourceInstance, owned_record_effects::CallablePlace, span,
    type_error, unsupported,
};
use crate::translation::{
    source_fn_mut_specializations::{Callback, Capture, Parameter, RustFnMutSpecialization},
    source_signatures::SourceOwner,
};

pub(super) fn fn_mut_result_packet(
    result: &SourceOrigin,
    states: &[SourceOrigin],
    path: &[String],
) -> Result<SourceOrigin> {
    let positions = std::iter::once(result).chain(states).map(source_type_position).collect::<Result<Vec<_>>>()?;
    let mut packet = SourceOrigin::with_type(
        Type::Tuple {
            elements: positions.iter().map(|position| position.ty.clone()).collect(),
        },
        path,
        Vec::new(),
    );
    packet.source_owner = Some(SourceOwner {
        kind: "tuple".to_owned(),
        path: Vec::new(),
        arguments: positions.into_iter().map(|position| position.owner).collect(),
    });
    Ok(packet)
}

fn pack(value: SExpr) -> SExpr {
    let location = value.span;
    SExpr::new(
        SNode::Tuple {
            items: vec![value],
            frozen: false,
            product_model: None,
        },
        location,
    )
}

impl RustParser {
    fn unpack_fn_mut_result(
        &mut self,
        call: SExpr,
        mut result: SourceOrigin,
        path: &[String],
    ) -> SExpr {
        let location = call.span;
        let node = SExpr::new(
            SNode::TupleIndex {
                object: Box::new(call),
                index: 0,
            },
            location,
        );
        result.path = path.to_vec();
        result.source_owned_result_proof = true;
        self.origins.mark(&node, result);
        node
    }

    pub(super) fn is_fn_mut_source_callback(&self, path: &[String]) -> bool {
        let [name] = path else { return false };
        self.origins
            .scope
            .get(name)
            .is_some_and(|origin| origin.source_fn_mut_parameter)
    }

    pub(super) fn read_fn_mut_source_callback_invocation(
        &mut self,
        callee: &SExpr,
        arguments: &[SExpr],
        path: &[String],
        location: Option<Span>,
    ) -> Result<SExpr> {
        let actual = self.origins.origin(callee, path);
        let header = self.fn_mut_source_header_read.as_ref().filter(|read| {
            read.body_replay
                && read.callback_binding == actual.source_lexical_binding_identity
                && (read.callback_binding.is_some() || self.origins.collecting)
        });
        let Some(header) = header else {
            return Err(unsupported(
                "Rust source FnMut callback",
                "a source callback call needs its actual lexical formal",
                location,
            ));
        };
        if arguments.len() != header.callback.parameters.len() {
            return Err(type_error(
                "source callback requires its complete actual operands",
                location,
            ));
        }
        let result = header.callback.result.clone();
        let mut parameters = header.callback.parameters.clone();
        let states = header.state_parameters.clone();
        let mut call = self
            .read_copy_closure_call(callee, arguments, path, location)?
            .ok_or_else(|| type_error("the original source formal must be callable", location))?;
        let SNode::App { args: inputs, .. } = &mut call.node else {
            return Err(type_error("the actual callback must retain its application", location));
        };
        let mut places = Vec::new();
        for slot in states {
            let input = SExpr::new(SNode::Name { path: vec![slot.current_name.clone()] }, None);
            places.push(CallablePlace { parameter: inputs.len(), slot: slot.clone(), layouts: Vec::new(), position: slot.origin.clone() });
            parameters.push(slot.origin);
            inputs.push(input);
        }
        if places.is_empty() {
            Ok(self.unpack_fn_mut_result(call, result, path))
        } else {
            self.retain_callable_packet(call, parameters, places, result)
        }
    }

    pub(super) fn read_fn_mut_source_invocation(
        &mut self,
        declaration: &FnMutSourceDeclaration,
        start: &Token,
        path: &[String],
    ) -> Result<SExpr> {
        if self.fn_mut_source_header_read.is_some() || !declaration.generics.is_empty() {
            return Err(unsupported(
                "Rust source FnMut function",
                "generic and recursive callback specialization needs actual binder and recursion admission",
                Some(start.span()),
            ));
        }
        let preview = self.preview_fn_mut_source_header(declaration, Vec::new())?;
        if !preview.resolved.exclusive_parameters.is_empty() {
            return Err(unsupported(
                "Rust source FnMut function",
                "an exclusive original formal needs ordered caller writeback",
                Some(start.span()),
            ));
        }
        let callback_formal = &preview.callback.owner.callback;
        self.cursor.expect("(", Some("source FnMut call"))?;
        let mut arguments = Vec::new();
        let mut captures = Vec::new();
        for (index, parameter) in preview.resolved.parameters.iter().enumerate() {
            if index > 0 {
                self.cursor.expect(",", Some("source FnMut arguments"))?;
            }
            let value = if index == callback_formal.index {
                if callback_formal.mode == "exclusive" {
                    self.cursor.expect("&", Some("immediate FnMut callback"))?;
                    self.cursor
                        .expect("mut", Some("immediate FnMut callback"))?;
                }
                let mut depth = 0;
                while self.cursor.is_at("(", depth) {
                    depth += 1;
                }
                if !self.cursor.is_at("|", depth) && !self.cursor.is_at("||", depth) {
                    return Err(unsupported(
                        "Rust source FnMut callback",
                        "the selected operand must be an immediate non-move closure",
                        Some(self.peek().span()),
                    ));
                }
                self.begin_fn_mut_source_capture_read()?;
                let read = if preview
                    .callback
                    .parameters
                    .iter()
                    .all(SourceOrigin::copy_closure_parameter)
                    && preview.callback.result.direct_copy_value()
                {
                    let mut expected = parameter.clone();
                    expected.copy_evidence = Some(());
                    self.read_expected_copy_closure(path, Some(&expected))
                } else {
                    self.read_owned_callback_closure(
                        path,
                        &preview.callback.parameters,
                        Some(&preview.callback.result),
                    )
                };
                let selected = self.fn_mut_source_capture_read.take().expect("the selected source callback retains its capture reader");
                let mut closure = read?;
                captures = selected.captures.ok_or_else(|| type_error("the selected callback must finish its original closure body", closure.span))?;
                self.origins.argument(
                    &self.origins.origin(&closure, path),
                    parameter,
                    closure.span,
                )?;
                let SNode::Lambda { params, ret, body, .. } = &mut closure.node else {
                    return Err(type_error(
                        "the actual callback needs its checked body",
                        closure.span,
                    ));
                };
                let states = captures.iter().map(|capture| capture.alias.origin.clone()).collect::<Vec<_>>();
                *ret = fn_mut_result_packet(&preview.callback.result, &states, path)?.ty;
                for capture in &captures {
                    params.push(self.callback_state_parameter(&capture.alias, closure.span)?);
                    self.retain_callable_state_parameter(capture.alias.current_name.clone(), capture.alias.clone(), true)?;
                }
                if captures.is_empty() {
                    **body = pack((**body).clone());
                }
                closure
            } else {
                let argument =
                    self.read_standard_string_view_argument(path, Some(parameter), None)?;
                self.origins.argument(
                    &self.origins.origin(&argument, path),
                    parameter,
                    argument.span,
                )?;
                self.origins
                    .prepare_owned_argument(&argument, Some(parameter), path)?;
                argument
            };
            arguments.push(value);
        }
        self.cursor.eat(",");
        let close = self.cursor.expect(")", Some("source FnMut call"))?;
        let actual = self.replay_fn_mut_source_declaration(declaration, Vec::new(), true, captures.clone())?;
        if captures.len() != actual.state_parameters.len() {
            return Err(type_error(
                "source FnMut replay must retain every ordered callback state parameter",
                span(start, &close),
            ));
        }
        let name = self.fresh_product_binding();
        let source_version = self
            .fn_mut_source_declarations
            .as_ref()
            .expect("an actual callback declaration retains its original source inventory")
            .source_version
            .clone();
        let result = actual.resolved.result;
        let mut parameters = actual
            .resolved
            .parameters
            .iter()
            .enumerate()
            .map(|(index, origin)| {
                Ok(Parameter {
                    mode: if index == callback_formal.index {
                        "callback"
                    } else if !origin.wrappers.is_empty() {
                        "shared"
                    } else {
                        "value"
                    }
                    .to_owned(),
                    position: source_type_position(origin)?,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        let mut parameter_origins = actual.resolved.parameters.clone();
        let mut places = Vec::new();
        let mut captured_parameters = Vec::new();
        let mut function = actual.function;
        for (capture, slot) in captures.into_iter().zip(&actual.state_parameters) {
            let parameter = parameters.len();
            let position = source_type_position(&slot.origin)?;
            if let Some(binder) = capture.binder {
                captured_parameters.push(Capture { parameter, binder, position: position.clone() });
            }
            parameters.push(Parameter { mode: "exclusive".to_owned(), position });
            parameter_origins.push(slot.origin.clone());
            function.params.push(self.callback_state_parameter(slot, span(start, &close))?);
            arguments.push(SExpr::new(SNode::Name { path: vec![capture.outer.current_name.clone()] }, span(start, &close)));
            places.push(CallablePlace { parameter, slot: capture.outer, layouts: Vec::new(), position: slot.origin.clone() });
        }
        let callback = Callback {
            parameter: callback_formal.index,
            parameters: actual
                .callback
                .parameters
                .iter()
                .map(source_type_position)
                .collect::<Result<_>>()?,
            result: source_type_position(&actual.callback.result)?,
            captures: captured_parameters,
        };
        function.name = name.clone();
        let states = actual.state_parameters.iter().map(|slot| slot.origin.clone()).collect::<Vec<_>>();
        function.ret = fn_mut_result_packet(&result, &states, &declaration.owner)?.ty;
        if places.is_empty() {
            function.body = pack(function.body);
        }
        function.source_signature = None;
        function.source_fn_mut_specialization = Some(Box::new(RustFnMutSpecialization {
            model: "Rust closed source FnMut specializations 1".to_owned(),
            language: "Rust".to_owned(),
            source_version,
            declaration: actual.callback.owner,
            type_arguments: Vec::new(),
            callback,
            parameters,
            result: source_type_position(&result)?,
            packet: places.iter().map(|place| place.parameter).collect(),
        }));
        self.fn_mut_source_instances.push(FnMutSourceInstance {
            owner: declaration.owner.clone(),
            declaration: declaration.declaration.clone(),
            function,
        });
        let callee = std::iter::once("crate".to_owned())
            .chain(declaration.owner.iter().cloned())
            .chain(std::iter::once(name))
            .collect();
        let location = span(start, &close);
        let call = SExpr::new(
            SNode::App {
                func: Box::new(SExpr::new(SNode::Name { path: callee }, location)),
                args: arguments,
            },
            location,
        );
        if places.is_empty() {
            Ok(self.unpack_fn_mut_result(call, result, path))
        } else {
            self.retain_callable_packet(call, parameter_origins, places, result)
        }
    }
}
