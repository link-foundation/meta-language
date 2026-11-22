//! A private ordered collection head and its ordinary loop completion.
use super::{
    FiniteRecordLoopState, LoopEntry, Options, Result, RustParser, SArrayItem, SExpr, SNode, Slot,
    SourceDeclaration, SourceOrigin, Span, invariant, rules,
};
use crate::translation::source_loop_folds::{RustSourceLoopFold, SourceFoldSpan};
use serde_json::Value;
use std::{collections::HashSet, sync::Arc};

pub(in super::super) struct AdmittedOrderedCollectionLoop {
    pub(in super::super) input_type: super::Type,
    pub(in super::super) item_type: super::Type,
    pub(in super::super) element: String,
    pub(in super::super) slots: Vec<Slot>,
    pub(in super::super) control_owner: Vec<String>,
    pub(in super::super) source_fold: Option<RustSourceLoopFold>,
    pub(in super::super) returns: usize,
    pub(super) writes: Vec<String>,
}

enum Pattern {
    Wildcard,
    Name(String, bool, Option<Span>, Span),
    Tuple(Vec<Pattern>, Option<Span>),
    Reference(Box<Pattern>, Option<Span>),
}

fn fail(reason: &str, span: Option<Span>) -> super::super::TranslationError {
    super::unsupported("Rust ordered collection loop", reason, span)
}

fn pattern(parser: &mut RustParser, path: &[String], depth: usize) -> Result<Pattern> {
    let span = Some(parser.cursor.peek().span());
    if depth > 4 {
        return Err(fail(
            "the ordered item pattern exceeds its finite depth",
            span,
        ));
    }
    let mutable = parser.cursor.eat("mut").is_some();
    if mutable
        && (parser.cursor.peek().kind != super::super::TokenKind::Identifier
            || parser.cursor.is("_"))
    {
        return Err(fail(
            "a mutable item pattern requires a direct lexical binding",
            span,
        ));
    }
    if parser.cursor.eat("&").is_some() {
        return Ok(Pattern::Reference(
            Box::new(pattern(parser, path, depth + 1)?),
            span,
        ));
    }
    if parser.cursor.eat("(").is_some() {
        if parser.cursor.eat(")").is_some() {
            return Ok(Pattern::Tuple(Vec::new(), span));
        }
        let first = pattern(parser, path, depth + 1)?;
        if parser.cursor.eat(",").is_none() {
            parser.cursor.expect(")", Some("ordered item pattern"))?;
            return Ok(first);
        }
        let mut items = vec![first];
        while !parser.cursor.is(")") {
            items.push(pattern(parser, path, depth + 1)?);
            if parser.cursor.eat(",").is_none() {
                break;
            }
        }
        parser.cursor.expect(")", Some("ordered tuple pattern"))?;
        return Ok(Pattern::Tuple(items, span));
    }
    if parser.cursor.eat("_").is_some() {
        return Ok(Pattern::Wildcard);
    }
    let binder = parser.cursor.identifier(Some("ordered item binding"))?;
    let error = crate::translation::rust_borrowed_iterator_rules::read_rust_borrowed_iterator_parameter_error(&binder.value, &binder.raw);
    if !error.is_empty() {
        return Err(fail(&error, Some(binder.span())));
    }
    if parser
        .origins
        .lookup_value_owner_ref(std::slice::from_ref(&binder.value), path)
        .is_some_and(|(_, owner)| {
            matches!(
                owner,
                SourceDeclaration::Constructor { .. } | SourceDeclaration::Constant { .. }
            )
        })
    {
        return Err(fail(
            "the ordered item pattern requires a true lexical binding",
            Some(binder.span()),
        ));
    }
    let binder_span = binder.span();
    Ok(Pattern::Name(binder.value, mutable, span, binder_span))
}

struct Binding {
    name: String,
    value: SExpr,
    origin: SourceOrigin,
    span: Option<Span>,
}

fn bind(
    parser: &mut RustParser,
    pattern: Pattern,
    value: SExpr,
    origin: SourceOrigin,
    path: &[String],
    bindings: &mut Vec<Binding>,
    names: &mut HashSet<String>,
) -> Result<()> {
    match pattern {
        Pattern::Wildcard => Ok(()),
        Pattern::Reference(inner, span) => {
            if origin.wrappers.as_slice() != ["reference"]
                || !matches!(
                    origin.ty,
                    Some(super::Type::Fixed { .. } | super::Type::Bool)
                )
            {
                return Err(fail(
                    "a reference pattern requires its actual primitive Copy referent",
                    span,
                ));
            }
            let mut copied = origin;
            copied.wrappers.clear();
            copied.source_lifetimes.clear();
            copied.source_borrowed_place = false;
            copied.source_value_borrow_bindings.clear();
            copied.source_loan_binding_identities.clear();
            copied.source_place_loan_binding_identities.clear();
            bind(parser, *inner, value, copied, path, bindings, names)
        }
        Pattern::Tuple(items, span) => {
            if !origin.wrappers.is_empty() {
                return Err(fail(
                    "a tuple pattern requires its actual direct tuple positions",
                    span,
                ));
            }
            if items.is_empty() && origin.ty == Some(super::super::UNIT) {
                return Ok(());
            }
            let fields = origin
                .fields
                .filter(|fields| fields.len() == items.len())
                .ok_or_else(|| {
                    fail(
                        "the ordered item pattern must preserve every actual tuple position",
                        span,
                    )
                })?;
            for (index, (item, field)) in items.into_iter().zip(fields).enumerate() {
                let projected = SExpr::new(
                    SNode::TupleIndex {
                        object: Box::new(value.clone()),
                        index,
                    },
                    span,
                );
                bind(parser, item, projected, field, path, bindings, names)?;
            }
            Ok(())
        }
        Pattern::Name(name, mutable, span, binder_span) => {
            if !names.insert(name.clone()) {
                return Err(fail("ordered item pattern bindings must be distinct", span));
            }
            parser.origins.mark(&value, origin.clone());
            parser.origins.bind(&name, &value, path, Some(&origin))?;
            let name = if mutable {
                if !origin.wrappers.is_empty() || origin.source_borrowed_place {
                    return Err(fail(
                        "a mutable item binding requires its actual owned position",
                        span,
                    ));
                }
                parser.register_owned_record_mutable_binding(&name, span, binder_span)?
            } else {
                name
            };
            bindings.push(Binding {
                name,
                value,
                origin,
                span,
            });
            Ok(())
        }
    }
}

impl RustParser {
    pub(in super::super) fn read_ordered_collection_loop(
        &mut self,
        path: &[String],
    ) -> Result<SExpr> {
        let start = self.cursor.advance();
        let location = Some(start.span());
        let selected = pattern(self, path, 0)?;
        self.cursor.expect("in", Some("ordered collection loop"))?;
        let depth = self
            .owned_record_state
            .effects
            .get_or_insert_with(Default::default)
            .loops
            .depth;
        let error = rules::read_rust_owned_record_nested_loop_depth_error((depth + 1) as f64);
        if !error.is_empty() {
            return Err(fail(&error, location));
        }
        self.owned_record_state
            .effects
            .as_mut()
            .expect("actual effect inventory")
            .loops
            .depth = depth + 1;
        let previous_scope = self.origins.enter(None);
        let temporaries = self.origins.owned_temporary_frame();
        let result_origin = self.standard_sum_source.return_origin.clone();
        let has_callable_result = result_origin
            .as_ref()
            .is_some_and(|origin| origin.ty.is_some());
        let frame = self.begin_owned_record_state_block(false);
        let result =
            self.read_ordered_collection_loop_scope(path, selected, location, has_callable_result);
        self.owned_record_state
            .effects
            .as_mut()
            .expect("actual effect inventory")
            .loops
            .depth = depth;
        self.standard_sum_source.return_origin = result_origin;
        self.origins.scope = previous_scope;
        self.origins.finish_ordered_iteration_loans(&temporaries);
        self.origins.restore_owned_temporary_frame(temporaries);
        self.end_owned_record_state_block(frame)?;
        result
    }

    fn ordered_collection_loop_state_roots(
        &self,
        body: &Value,
        span: Option<Span>,
    ) -> Result<HashSet<String>> {
        let mut roots = HashSet::new();
        let mut pending = vec![body];
        let mut work = 0;
        while let Some(node) = pending.pop() {
            work += 1;
            if work > crate::translation::rust_owned_record_effect_rules::read_rust_owned_record_effect_limit("inspection") as usize {
                return Err(fail("the ordered loop state roots exceed their source budget", span));
            }
            if super::text(node, "k") == "math" {
                let op = super::text(node, "op");
                let ordinal = super::text(node, "name");
                let arity = node["args"].as_array().map_or(0, Vec::len);
                if let Some(effect) = self.owned_record_effect_source_parts(op, ordinal, arity) {
                    let slot = match effect {
                        super::EffectSource::Read(slot)
                        | super::EffectSource::Write { slot, .. } => slot,
                    };
                    roots.insert(slot.lexical.clone());
                }
            }
            match node {
                Value::Array(children) => pending.extend(children),
                Value::Object(children) => pending.extend(children.values()),
                _ => {}
            }
        }
        Ok(roots)
    }

    fn read_ordered_collection_loop_scope(
        &mut self,
        path: &[String],
        selected: Pattern,
        location: Option<Span>,
        callable_result: bool,
    ) -> Result<SExpr> {
        let previous = self.ordered_iteration_head.replace(Vec::new());
        let receiver = self.with_standard_sum_source_context(None, |parser| {
            parser.expr(
                path,
                Options {
                    no_struct: true,
                    ..Options::default()
                },
            )
        });
        let heads = std::mem::replace(&mut self.ordered_iteration_head, previous)
            .expect("actual ordered head inventory");
        let receiver = receiver?;
        let (input, item, owned_vector) = match heads.as_slice() {
            [] => {
                if let Some((input, item, actual)) =
                    self.owned_vector_source_iteration(&receiver, path)?
                {
                    (input, item, Some(actual))
                } else {
                    let method = if self.origins.origin(&receiver, path).wrappers.is_empty() {
                        "into_iter"
                    } else {
                        "iter"
                    };
                    let (input, item) = self.standard_collection_iteration(&receiver, method, path)?.ok_or_else(|| {
                        fail(
                            if self.exclusive_source_enabled {
                                "for requires its actual selected standard collection or owned Vec receiver"
                            } else {
                                "for requires its actual standard BTree receiver"
                            },
                            receiver.span,
                        )
                    })?;
                    (input, item, None)
                }
            }
            [head]
                if head.expression.node == receiver.node
                    && self
                        .cursor
                        .tokens
                        .get(head.end_token..self.cursor.index)
                        .is_some_and(|remaining| {
                            remaining.iter().all(|token| token.value == ")")
                        })
                    && self.origins.origin(&receiver, path).wrappers.is_empty() =>
            {
                (
                    head.expression.clone(),
                    head.item.clone(),
                    head.owned_vector.clone(),
                )
            }
            _ => {
                return Err(fail(
                    "a private ordered iterator must be the complete loop head",
                    receiver.span,
                ));
            }
        };
        let iteration_origin = self.origins.origin(&input, path);
        let iterator_regions = &iteration_origin.source_value_borrow_bindings;
        if !iterator_regions.is_empty() {
            // The receiver remains borrowed even when the item pattern copies.
            let sentinel = self.fresh_product_binding();
            self.origins
                .scope
                .insert(sentinel, iteration_origin.clone());
        }
        let before = self.ordinary_completion_snapshot()?;
        let slots = self.record_state_active_slots();
        let input_type = iteration_origin
            .ty
            .ok_or_else(|| fail("ordered iteration needs its actual input type", input.span))?;
        let item_type = item
            .ty
            .clone()
            .ok_or_else(|| fail("ordered iteration needs its actual item type", input.span))?;
        let element = self.fresh_product_binding();
        let value = SExpr::new(
            SNode::Name {
                path: vec![element.clone()],
            },
            location,
        );
        let mut bindings = Vec::new();
        bind(
            self,
            selected,
            value,
            item.clone(),
            path,
            &mut bindings,
            &mut HashSet::new(),
        )?;
        if !callable_result {
            self.standard_sum_source.return_origin = Some(
                self.origins
                    .origin(&SExpr::new(SNode::Unit, location), path),
            );
        }
        let result_origin = self.standard_sum_source.return_origin.clone();
        let completion = self.ordinary_loop_completion(path, None, location)?;
        if !callable_result && completion.returns != 0 {
            return Err(fail(
                "function return requires its actual enclosing callable result",
                location,
            ));
        }
        let span = location
            .map(|start| Span::new(start.start, self.cursor.tokens[self.cursor.index - 1].end));
        let mut body = completion
            .expression
            .expect("actual ordered loop completion");
        for binding in bindings.into_iter().rev() {
            body = SExpr::new(
                SNode::Let {
                    name: binding.name,
                    ty: binding.origin.ty.map(Into::into),
                    value: Box::new(binding.value),
                    body: Box::new(body),
                },
                binding.span,
            );
        }
        let mut return_paths = Vec::new();
        let admission = if self.origins.collecting {
            self.origins.standard_sum_source_needs_final_admission = true;
            None
        } else {
            for exit in &completion.paths {
                if rules::test_rust_owned_record_loop_continuing_exit(&exit.kind) {
                    invariant(&before, &exit.ownership, &slots, span)?;
                }
                if exit.kind == "return"
                    && (!exit.origin.wrappers.is_empty()
                        || super::super::copy_closure_lifetimes::value_borrows(&exit.origin)
                            .iter()
                            .any(|region| {
                                iterator_regions.is_empty() || iterator_regions.contains(region)
                            }))
                {
                    return Err(fail(
                        "ordered loop return needs its actual result without a loop loan",
                        span,
                    ));
                }
            }
            let outer = before.keys().cloned().collect::<Vec<_>>();
            self.origins.restore_owned_bindings(before);
            let mut inspection = 0;
            for exit in completion.paths {
                if exit.kind == "break" {
                    let mut branch = std::collections::HashMap::new();
                    for (key, state) in exit.ownership {
                        for root in &outer {
                            inspection += 1;
                            if inspection > 1_000_000 {
                                return Err(fail(
                                    "ordered loop ownership inspection exceeds its source budget",
                                    span,
                                ));
                            }
                            if root == &key
                                || super::fields::test_rust_owned_record_field_descendant(
                                    root, &key,
                                )
                            {
                                branch.insert(key, state);
                                break;
                            }
                        }
                    }
                    let current = self.ordinary_completion_snapshot()?;
                    self.origins
                        .join_owned_bindings(&current, &branch, true, true);
                } else if exit.kind == "return" {
                    return_paths.push(exit);
                }
            }
            crate::translation::finite_record_loop_helpers::preflight_source(
                &(&input, &body),
                span,
            )?;
            let input_surface = serde_json::to_value(&input)
                .map_err(|_| fail("ordered loop input transport failed", span))?;
            let surface = serde_json::to_value(&body)
                .map_err(|_| fail("ordered loop source transport failed", span))?;
            let used = self.ordered_collection_loop_state_roots(&surface, span)?;
            let writes = super::write_footprint(self, &Value::Array(vec![input_surface, surface]))?;
            let slots: Vec<_> = slots
                .into_iter()
                .filter(|slot| used.contains(&slot.lexical))
                .collect();
            let mut control_owner = vec!["crate".to_owned()];
            control_owner.extend(completion.owner);
            let source_fold = if self.retain_source_region_state() {
                let actual = owned_vector.as_ref().ok_or_else(|| {
                    fail(
                        "shared collection loops require their actual collection region interface",
                        span,
                    )
                })?;
                let span = span
                    .ok_or_else(|| fail("a source fold needs its complete original span", None))?;
                let result = result_origin.as_ref().ok_or_else(|| {
                    fail(
                        "a source fold needs its actual enclosing result",
                        Some(span),
                    )
                })?;
                Some(RustSourceLoopFold {
                    model: "Rust owned vector source fold 1".into(),
                    input: self.retain_source_region_position(actual, Some(span))?,
                    item: self.retain_source_region_position(&item, Some(span))?,
                    result: self.retain_source_region_position(result, Some(span))?,
                    control: super::Type::Named {
                        path: control_owner.clone(),
                        span: None,
                    },
                    slots: slots
                        .iter()
                        .map(|slot| self.retain_source_fold_slot(slot))
                        .collect::<Result<_>>()?,
                    span: SourceFoldSpan {
                        start: span.start,
                        end: span.end,
                    },
                })
            } else {
                None
            };
            let inventory = &mut self
                .owned_record_state
                .effects
                .as_mut()
                .expect("actual effect inventory")
                .loops;
            inventory.footprint_nodes += writes.len();
            inventory.footprint_units += writes.iter().map(String::len).sum::<usize>();
            Some(Arc::new(AdmittedOrderedCollectionLoop {
                input_type,
                item_type,
                element,
                slots,
                control_owner,
                source_fold,
                returns: completion.returns,
                writes,
            }))
        };
        let FiniteRecordLoopState { entries, .. } = &mut self
            .owned_record_state
            .effects
            .as_mut()
            .expect("actual effect inventory")
            .loops;
        let mut node = SExpr::new(
            SNode::Math {
                op: rules::read_rust_ordered_collection_loop_operation(),
                name: entries.len().to_string(),
                args: vec![SArrayItem::new(input, false), SArrayItem::new(body, false)],
            },
            span,
        );
        entries.push(LoopEntry {
            admission: None,
            ordered: admission,
            destination: None,
            return_paths,
        });
        node.block = true;
        self.origins.mark(
            &node,
            SourceOrigin::with_type(super::super::UNIT, path, Vec::new()),
        );
        Ok(node)
    }
}
