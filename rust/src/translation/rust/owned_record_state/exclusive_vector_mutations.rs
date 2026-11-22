//! Bind an actual Vec place to the shared pop, clear and truncate surfaces.
use serde_json::{Value, json};

use super::{
    CollectionPlace, Result, RustParser, SExpr, SNode, SourceOrigin, Span, Token, Type, fail, name,
    rules,
};
use crate::translation::{
    rust_exclusive_source_vector_rules as vector_rules,
    rust_vector_mutation_helper_surfaces as templates,
    source_loop_folds::SourceFoldSpan,
    source_signatures::SourceOwner,
    source_vector_mutations::{MODEL, RustVectorMutation},
    surface::SItem,
};

fn fill_template(value: &mut Value, bindings: &Value) -> Option<()> {
    if let Some(parameter) = value
        .get("sourceVectorMutationTemplateParameter")
        .and_then(Value::as_str)
    {
        *value = bindings.get(parameter)?.clone();
        return Some(());
    }
    match value {
        Value::Array(values) => {
            for value in values {
                fill_template(value, bindings)?;
            }
        }
        Value::Object(values) => {
            for value in values.values_mut() {
                fill_template(value, bindings)?;
            }
        }
        _ => {}
    }
    Some(())
}

impl RustParser {
    pub(in super::super) fn read_exclusive_owned_vector_mutation(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        let arity = vector_rules::read_rust_exclusive_source_vector_method_arity(&method.value);
        if !self.exclusive_source_enabled || arity < 0.0 {
            return Ok(None);
        }
        let actual = self.origins.origin(receiver, path);
        let Some(Type::Array {
            element,
            reference: false,
        }) = actual.ty.as_ref()
        else {
            return Ok(None);
        };
        let owner = actual
            .expanded_source_owner
            .as_ref()
            .or(actual.source_owner.as_ref());
        let child = owner
            .filter(|owner| {
                vector_rules::accept_rust_exclusive_source_vector_receiver(
                    "array",
                    actual.array_source_model.unwrap_or(""),
                    actual.wrappers.len() as f64,
                    actual.proven || self.origins.collecting,
                    &owner.kind,
                    &owner.path.join("::"),
                    owner.arguments.len() as f64,
                )
            })
            .and_then(|owner| owner.arguments.first());
        let Some(child) = child else {
            return Err(fail(
                "Vec mutation needs its actual direct standard owned Vec receiver",
                receiver.span,
            ));
        };
        let place = super::super::owned_record_loops::place(self, receiver).ok_or_else(|| {
            fail(
                "Vec mutation needs its actual mutable lexical place",
                receiver.span,
            )
        })?;
        let slot = place.slot.clone();
        let layouts =
            self.record_state_layout_indices(&slot.origin, &place.indices, receiver.span)?;
        let target = rules::read_rust_owned_record_state_field_identity(
            &slot.root,
            &place
                .indices
                .iter()
                .map(|index| *index as f64)
                .collect::<Vec<_>>(),
        );
        let element_type = self.record_source_type(element, &actual.path);
        let mut item = super::super::owned_vector_iteration::item_origin(
            element_type.clone(),
            child,
            path,
            receiver.span,
            0,
        )?;
        let ownership = self.origins.standard_sum_ownership_facts(&item);
        if !self.origins.collecting && !matches!(ownership.profile.as_str(), "copy" | "owned") {
            return Err(fail(
                "Vec mutation requires an actual closed destructor-free element",
                receiver.span,
            ));
        }
        item.copy_evidence = (ownership.profile == "copy").then_some(());

        self.cursor.expect("(", Some("Vec mutation"))?;
        let mut bound = SourceOrigin::with_type(
            Type::Fixed {
                signed: false,
                bits: 64,
            },
            path,
            Vec::new(),
        );
        bound.copy_evidence = Some(());
        bound.source_owner = Some(SourceOwner {
            kind: "primitive".into(),
            path: vec!["usize".into()],
            arguments: Vec::new(),
        });
        let operand = if arity == 1.0 {
            let mut operand = self.read_expected_copy_closure(path, Some(&bound))?;
            self.check_reserved_place_operand_effects(receiver, &operand)?;
            if let SNode::Num { ty, .. } = &mut operand.node
                && ty.is_none()
            {
                *ty = bound.ty.clone();
                self.origins.mark(&operand, bound.clone());
            }
            let supplied = self.origins.origin(&operand, path);
            self.origins
                .argument_with_transfer(&supplied, &bound, operand.span, false)?;
            let supplied_owner = supplied
                .expanded_source_owner
                .as_ref()
                .or(supplied.source_owner.as_ref());
            if !self.origins.collecting
                && (!supplied.proven
                    || !supplied.wrappers.is_empty()
                    || supplied_owner != bound.source_owner.as_ref())
            {
                return Err(fail(
                    "Vec truncate needs its actual direct usize argument",
                    operand.span,
                ));
            }
            self.cursor.eat(",");
            Some(operand)
        } else {
            None
        };
        let end = self.cursor.expect(")", Some("Vec mutation"))?;
        let span = Span::new(
            receiver.span.map_or(method.start, |site| site.start),
            end.end,
        );
        if !self.origins.collecting
            && (slot.callable != self.owned_record_state.current_callable
                || self.record_state_overlaps(&slot, &target)
                || super::super::owned_record_updates::field_state(
                    &self.origins.snapshot_owned_bindings(),
                    &target,
                ) != "available")
        {
            return Err(fail(
                "Vec mutation needs an available place without overlapping loans",
                Some(span),
            ));
        }
        let completion = if method.value == "pop" {
            self.instantiate_standard_sum_source(
                "std::option::Option",
                vec![item.clone()],
                Some(span),
            )?
        } else {
            let mut unit = SourceOrigin::with_type(Type::Unit, path, Vec::new());
            unit.copy_evidence = Some(());
            unit.source_owner = Some(SourceOwner {
                kind: "unit".into(),
                path: Vec::new(),
                arguments: Vec::new(),
            });
            unit
        };
        let result_type = self.record_source_type(
            completion.ty.as_ref().ok_or_else(|| {
                fail(
                    "the Vec helper needs its actual source result type",
                    Some(span),
                )
            })?,
            &completion.path,
        );
        let vector_type =
            self.record_source_type(actual.ty.as_ref().expect("actual Vec type"), &actual.path);
        let packet_type = Type::Tuple {
            elements: vec![result_type, vector_type.clone()],
        };
        let helper = self.fresh_product_binding();
        let vector_parameter = self.fresh_product_binding();
        let bound_parameter = operand.as_ref().map(|_| self.fresh_product_binding());
        let mut parameters = vec![json!({"name": vector_parameter, "type": vector_type})];
        if let Some(parameter) = &bound_parameter {
            parameters.push(json!({"name": parameter, "type": bound.ty}));
        }
        let sum = completion.standard_sum_source.as_deref();
        let template = match method.value.as_str() {
            "clear" => templates::CLEAR,
            "pop" => templates::POP,
            "truncate" => templates::TRUNCATE,
            _ => {
                return Err(fail(
                    "the Vec operation has no authored helper body",
                    Some(span),
                ));
            }
        };
        let mut body = serde_json::from_str(template)
            .map_err(|_| fail("the Vec helper surface is malformed", Some(span)))?;
        fill_template(
            &mut body,
            &json!({
                "span": span, "vector": vector_type, "packet": packet_type, "element": element_type,
                "vectorParameter": vector_parameter, "boundParameter": bound_parameter,
                "accumulator": self.fresh_product_binding(), "item": self.fresh_product_binding(),
                "prior": self.fresh_product_binding(), "held": self.fresh_product_binding(),
                "nonePath": sum.and_then(|sum| sum.constructor_paths.get("None")),
                "somePath": sum.and_then(|sum| sum.constructor_paths.get("Some"))
            }),
        )
        .ok_or_else(|| {
            fail(
                "the Vec helper lacks its complete actual template inputs",
                Some(span),
            )
        })?;
        let declaration = json!({
            "k": "fn", "name": helper, "generated": true, "span": span,
            "params": parameters, "ret": packet_type, "body": body
        });
        let mut declaration: SItem = serde_json::from_value(declaration).map_err(|_| {
            fail(
                "the Vec helper could not retain its actual complete source declaration",
                Some(span),
            )
        })?;
        let SItem::Fn(function) = &mut declaration else {
            return Err(fail(
                "the Vec helper requires an actual function declaration",
                Some(span),
            ));
        };
        if !self.origins.collecting && self.source_regions_composed {
            let mut parameters = vec![self.retain_source_region_position(&actual, Some(span))?];
            if operand.is_some() {
                parameters.push(self.retain_source_region_position(&bound, Some(span))?);
            }
            function.source_vector_mutation = Some(Box::new(RustVectorMutation {
                model: MODEL.into(),
                operation: method.value.clone(),
                parameters,
                element: self.retain_source_region_position(&item, Some(span))?,
                result: self.retain_source_region_position(&completion, Some(span))?,
                span: SourceFoldSpan {
                    start: span.start,
                    end: span.end,
                },
            }));
        }
        self.retain_owned_vector_mutation_helper(declaration)?;

        let read = self.owned_record_effect_read(slot.clone(), Some(span))?;
        let selected = self.project_record_state(&layouts, read, Some(span));
        let argument = operand.as_ref().map(|_| self.fresh_product_binding());
        let mut args = vec![selected];
        args.extend(argument.as_deref().map(name));
        let mut packet = SExpr::new(
            SNode::App {
                func: Box::new(SExpr::new(
                    SNode::Name {
                        path: vec!["crate".into(), helper],
                    },
                    None,
                )),
                args,
            },
            Some(span),
        );
        if let (Some(operand), Some(argument)) = (operand, argument) {
            packet = SExpr::new(
                SNode::Let {
                    name: argument,
                    ty: bound.ty.map(Into::into),
                    value: Box::new(operand),
                    body: Box::new(packet),
                },
                Some(span),
            );
        }
        self.owned_vector_mutation_packet(
            CollectionPlace {
                slot,
                layouts,
                target,
            },
            packet_type,
            packet,
            completion,
        )
        .map(Some)
    }
}
