//! Calls use the actual Rust body and the checked callback completion packet.

use super::{
    Result, RustParser, SExpr, SNode, SourceOrigin, Span, Token, Type,
    borrowed_vectors::source_type_position, fn_mut_source_declarations::FnMutSourceDeclaration,
    fn_mut_source_headers::FnMutSourceInstance, span, type_error, unsupported,
};
use crate::translation::{
    source_fn_mut_specializations::{Callback, Parameter, RustFnMutSpecialization},
    source_signatures::SourceOwner,
};

pub(super) fn fn_mut_result_packet(result: &SourceOrigin, path: &[String]) -> Result<SourceOrigin> {
    let position = source_type_position(result)?;
    let mut packet = SourceOrigin::with_type(
        Type::Tuple {
            elements: vec![position.ty],
        },
        path,
        Vec::new(),
    );
    packet.source_owner = Some(SourceOwner {
        kind: "tuple".to_owned(),
        path: Vec::new(),
        arguments: vec![position.owner],
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
        let call = self
            .read_copy_closure_call(callee, arguments, path, location)?
            .ok_or_else(|| type_error("the original source formal must be callable", location))?;
        Ok(self.unpack_fn_mut_result(call, result, path))
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
                let mut closure = if preview
                    .callback
                    .parameters
                    .iter()
                    .all(SourceOrigin::copy_closure_parameter)
                    && preview.callback.result.direct_copy_value()
                {
                    let mut expected = parameter.clone();
                    expected.copy_evidence = Some(());
                    self.read_expected_copy_closure(path, Some(&expected))?
                } else {
                    self.read_owned_callback_closure(
                        path,
                        &preview.callback.parameters,
                        Some(&preview.callback.result),
                    )?
                };
                self.origins.argument(
                    &self.origins.origin(&closure, path),
                    parameter,
                    closure.span,
                )?;
                let SNode::Lambda { ret, body, .. } = &mut closure.node else {
                    return Err(type_error(
                        "the actual callback needs its checked body",
                        closure.span,
                    ));
                };
                *ret = fn_mut_result_packet(&preview.callback.result, path)?.ty;
                **body = pack((**body).clone());
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
        let actual = self.replay_fn_mut_source_declaration(declaration, Vec::new(), true)?;
        let name = self.fresh_product_binding();
        let source_version = self
            .fn_mut_source_declarations
            .as_ref()
            .expect("an actual callback declaration retains its original source inventory")
            .source_version
            .clone();
        let result = actual.resolved.result;
        let parameters = actual
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
            .collect::<Result<_>>()?;
        let callback = Callback {
            parameter: callback_formal.index,
            parameters: actual
                .callback
                .parameters
                .iter()
                .map(source_type_position)
                .collect::<Result<_>>()?,
            result: source_type_position(&actual.callback.result)?,
            captures: Vec::new(),
        };
        let mut function = actual.function;
        function.name = name.clone();
        function.ret = fn_mut_result_packet(&result, &declaration.owner)?.ty;
        function.body = pack(function.body);
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
            packet: Vec::new(),
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
        Ok(self.unpack_fn_mut_result(call, result, path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::translation::rust::{SItem, SourceProfiles, parse_rust_with_selected_profiles};

    #[test]
    fn direct_callback_calls_replay_every_call_in_the_original_declaration() {
        let source = "
            fn transform(value: u8, mut map: impl FnMut(u8) -> u8) -> u8 {
                map(map(value))
            }
            fn main() {
                assert!(transform(2u8, |value| value + 1u8) == 4u8);
            }
        ";
        let surface = parse_rust_with_selected_profiles(
            source,
            SourceProfiles {
                source_version: "1.93.2",
                exclusive_source: "Rust exclusive source loans 1",
                fn_mut_source: "Rust closed source FnMut specializations 1",
                ..SourceProfiles::default()
            },
        )
        .unwrap();
        let function = surface.items.iter().find_map(|item| match item {
            SItem::Fn(function) if function.source_fn_mut_specialization.is_some() => {
                Some(function)
            }
            _ => None,
        });
        let function = function.expect("the selected original declaration is specialized");
        let facet = function.source_fn_mut_specialization.as_ref().unwrap();
        assert_eq!(facet.source_version, "1.93.2");
        assert_eq!(facet.declaration.path, ["transform"]);
        assert_eq!(facet.callback.parameter, 1);
        assert!(facet.packet.is_empty());
        assert!(matches!(&function.body.node, SNode::Tuple { items, .. } if items.len() == 1));
        assert!(!surface.items.iter().any(|item| item.name() == "transform"));
    }
}
