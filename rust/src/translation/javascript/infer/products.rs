//! Product expression observations from the existing inference engine.

use super::{
    Env, HashMap, HashSet, Inference, ROOT, Result, SExpr, SItem, SNode, SProgram, Span, Term,
    Type, collect_functions, fill_arrays, fill_fields, fill_functions, type_error,
};
use crate::translation::frontend_rules::{
    accept_argument_count, accept_java_script_optional_tuple_context,
};
use crate::translation::javascript::product_source_rules::accept_java_script_array_tuple_context;

type ExpressionTypes = HashMap<*const SExpr, Type>;
type InferredProgram = (SProgram, ExpressionTypes);

/// Fills in the missing parameter and result types of the functions of a parsed JavaScript program.
pub(in super::super) fn infer_javascript_types(program: SProgram) -> Result<SProgram> {
    infer_javascript_types_with_observations(program, false).map(|(program, _)| program)
}

/// Preserve the existing engine's inferred expression types for source admission.
pub(in super::super) fn infer_javascript_product_types(
    program: SProgram,
) -> Result<InferredProgram> {
    infer_javascript_types_with_observations(program, true)
}

fn infer_javascript_types_with_observations(
    mut program: SProgram,
    observe: bool,
) -> Result<InferredProgram> {
    let mut functions = Vec::new();
    collect_functions(&program.items, &[ROOT.to_owned()], &mut functions);
    let untyped = program.items.iter().any(|item| {
        matches!(item, SItem::Data(data) if data.ctors.iter().any(|ctor| {
            ctor.fields.iter().any(|field| field.ty == Type::Literal)
        }))
    });
    if !observe
        && !untyped
        && !super::callables::has_structural_inference(
            &serde_json::to_value(&program).expect("a surface program is serializable"),
        )
        && functions.iter().all(|(_, function)| {
            function.ret.is_some() && function.params.iter().all(|param| param.ty.is_some())
        })
    {
        let mut observed = HashMap::new();
        super::super::reference_analysis::mark_shared_reference_types(&mut program, &mut observed)?;
        return Ok((program, observed));
    }
    let mut inference = Inference::new(&program.items);
    inference.record_expressions = observe;
    let inferred = inference.run(&functions, &program)?;
    let mut expression_types = if observe {
        inference
            .expression_terms
            .iter()
            .map(|(node, (term, place))| (*node, inference.surface(term, *place)))
            .collect()
    } else {
        HashMap::new()
    };
    let mut arrays: HashMap<*const SExpr, Type> = inference
        .empty_arrays
        .iter()
        .map(|(node, (term, place))| (*node, inference.surface(term, *place)))
        .collect();
    arrays.extend(inference.lambdas.iter().map(|(node, term)| {
        let (place, places) = &inference.lambda_places[node];
        let Term::Function { params, ret } = term else {
            unreachable!("an observed Lambda retains its Function term")
        };
        (
            *node,
            Type::Function {
                params: params
                    .iter()
                    .zip(places)
                    .map(|(parameter, place)| inference.surface(parameter, *place))
                    .collect(),
                ret: Box::new(inference.surface(ret, *place)),
            },
        )
    }));
    arrays.extend(
        inference
            .collections
            .iter()
            .map(|(node, (term, place))| (*node, inference.surface(term, *place))),
    );
    arrays.extend(
        inference
            .cells
            .iter()
            .map(|(node, (term, place))| (*node, inference.surface(term, *place))),
    );
    arrays.extend(
        inference
            .records
            .iter()
            .map(|(node, term)| (*node, inference.surface(term, None))),
    );
    let mut inferred = inferred.into_iter();
    fill_functions(&mut program.items, &mut inferred);
    fill_fields(&mut program.items, &inference);
    if !arrays.is_empty() {
        fill_arrays(&mut program, &arrays);
    }
    super::methods::fill_methods(
        &mut program,
        &inference.array_includes,
        &mut expression_types,
    );
    super::super::reference_analysis::mark_shared_reference_types(
        &mut program,
        &mut expression_types,
    )?;
    Ok((program, expression_types))
}

impl Inference {
    /// Connect actual iterator owners to the complete generated iterable plan.
    pub(super) fn solve_iterables(&mut self) -> Result<bool> {
        let mut progress = false;
        let mut waiting = Vec::new();
        for constraint in std::mem::take(&mut self.iterables) {
            let object = self.resolve(&constraint.object);
            if let Some(element) = self.array_from_iterable_element(&object) {
                self.solve_array_from_iterable(&constraint, element)?;
                progress = true;
            } else {
                waiting.push(constraint);
            }
        }
        self.iterables = waiting;
        Ok(progress)
    }

    pub(super) fn product_condition(&mut self, term: &Term, place: Option<Span>) {
        self.conditions.push((term.clone(), place));
    }

    /// Apply the author's positional expected type without turning a source array into owned evidence.
    pub(super) fn expr_expected(
        &mut self,
        node: &SExpr,
        env: &Env,
        expected: Option<&Term>,
    ) -> Result<Term> {
        let expected = expected.map(|term| self.resolve(term));
        let Some(mut expected) = expected else {
            return self.expr(node, env);
        };
        if let (SNode::Array { .. }, Term::Option { element, .. }) = (&node.node, &expected) {
            let present = self.resolve(element);
            let present_kind = match &present {
                Term::Tuple(_) => "tuple",
                Term::Known(ty) => ty.kind(),
                _ => "",
            };
            if accept_java_script_optional_tuple_context(
                "JavaScript",
                node.node.kind(),
                "option",
                present_kind,
            ) {
                expected = present;
            }
        }
        let term = match (&node.node, &expected) {
            (SNode::ArrayBindingDefault { .. }, _) => {
                self.array_binding_expr(node, env, Some(&expected))?
            }
            (SNode::ArrayFrom { args, .. }, _) => {
                self.array_from(node, args, env, Some(&expected))?
            }
            (SNode::Set { items, .. } | SNode::Map { entries: items, .. }, _) => {
                self.collection_construction(node, items, env, Some(&expected))?
            }
            (SNode::Record { entries, .. }, _) => {
                self.record_expr(node, entries, env, Some(&expected))?
            }
            (SNode::Lambda { .. } | SNode::CellWrite { .. }, _) => {
                self.callable_expr(node, env, Some(&expected))?
            }
            (SNode::Array { items, .. }, Term::Tuple(elements))
                if accept_java_script_array_tuple_context(
                    "array",
                    "tuple",
                    items.iter().any(|item| item.spread),
                ) =>
            {
                Term::Tuple(
                    items
                        .iter()
                        .enumerate()
                        .map(|(index, item)| {
                            self.expr_expected(&item.value, env, elements.get(index))
                        })
                        .collect::<Result<Vec<_>>>()?,
                )
            }
            (SNode::Array { items, .. }, Term::Array(wanted_element)) => {
                let element = self.fresh(false);
                self.empty_arrays
                    .insert(std::ptr::from_ref(node), (element.clone(), node.span));
                for item in items {
                    let wanted = if item.spread {
                        &expected
                    } else {
                        wanted_element.as_ref()
                    };
                    let value = self.expr_expected(&item.value, env, Some(wanted))?;
                    let want = if item.spread {
                        Term::Array(Box::new(element.clone()))
                    } else {
                        element.clone()
                    };
                    self.unify(&value, &want, item.value.span.or(node.span))?;
                    let contained = if item.spread {
                        match self.resolve(&value) {
                            Term::Array(element) => Some(*element),
                            _ => None,
                        }
                    } else {
                        Some(self.resolve(&value))
                    };
                    if let Some(contained) = contained
                        && matches!(self.resolve(&contained), Term::Option { .. })
                        && let Term::Var(variable) = &element
                    {
                        self.bindings[*variable] = Some(self.resolve(&contained));
                    }
                }
                Term::Array(Box::new(element))
            }
            (
                SNode::Let {
                    name, value, body, ..
                },
                _,
            ) => {
                let value = self.expr(value, env)?;
                let mut scope = env.clone();
                scope.insert(name.clone(), value);
                self.expr_expected(body, &scope, Some(&expected))?
            }
            (SNode::Print { expr, body, .. }, _) => {
                self.expr(expr, env)?;
                self.expr_expected(body, env, Some(&expected))?
            }
            (
                SNode::If {
                    cond,
                    then,
                    otherwise,
                },
                _,
            ) => {
                let condition = self.expr(cond, env)?;
                self.product_condition(&condition, cond.span.or(node.span));
                let yes = self.expr_expected(then, env, Some(&expected))?;
                let no = self.expr_expected(otherwise, env, Some(&expected))?;
                self.join(yes, no, node.span)
            }
            _ => return self.expr(node, env),
        };
        if self.record_expressions {
            self.expression_terms
                .insert(std::ptr::from_ref(node), (term.clone(), node.span));
        }
        Ok(term)
    }

    pub(super) fn call(
        &mut self,
        path: &[String],
        args: &[SExpr],
        env: &Env,
        place: Option<Span>,
    ) -> Result<Term> {
        let signature = self.signature(path);
        let arg_terms = args
            .iter()
            .enumerate()
            .map(|(index, arg)| {
                self.expr_expected(
                    arg,
                    env,
                    signature
                        .as_ref()
                        .and_then(|signature| signature.params.get(index)),
                )
            })
            .collect::<Result<Vec<_>>>()?;
        #[allow(clippy::cast_precision_loss)]
        let argument_count = args.len() as f64;
        let Some(signature) = signature
            .filter(|signature| accept_argument_count(signature.defaults.clone(), argument_count))
        else {
            return Ok(self.fresh(false));
        };
        self.calls.push(super::CallArguments {
            parameters: signature.params.clone(),
            arguments: arg_terms.clone(),
            defaults: signature.defaults.clone(),
            optional_params: signature.optional_params.clone(),
            place,
        });
        for (index, ((term, param), arg)) in arg_terms
            .iter()
            .zip(&signature.params)
            .zip(args)
            .enumerate()
        {
            let Some((value, _)) = self.parameter_default_argument_inference(
                signature.defaults[index],
                signature.optional_params[index],
                term,
            ) else {
                continue;
            };
            self.unify(&value, param, arg.span.or(place))?;
            if let Term::Var(variable) = param
                && matches!(self.resolve(&value), Term::Option { .. })
            {
                self.bindings[*variable] = Some(self.resolve(&value));
            }
        }
        Ok(signature.ret)
    }

    /// Inspect a raw variable before following aliases, as the JavaScript engine does.
    pub(super) fn contains(&self, term: &Term, variable: usize, seen: &mut HashSet<usize>) -> bool {
        match term {
            Term::Var(id) if *id == variable => true,
            Term::Var(id) => {
                seen.insert(*id)
                    && self.bindings[*id]
                        .as_ref()
                        .is_some_and(|next| self.contains(next, variable, seen))
            }
            Term::Array(element)
            | Term::Cell(element)
            | Term::Option { element, .. }
            | Term::Set { element, .. } => self.contains(element, variable, seen),
            Term::Record(owner) => owner
                .borrow()
                .fields
                .iter()
                .any(|(_, term)| self.contains(term, variable, seen)),
            Term::Function { .. } => self.callable_contains(term, variable, seen),
            Term::Map { key, value, .. } => {
                self.contains(key, variable, seen) || self.contains(value, variable, seen)
            }
            Term::Tuple(elements) | Term::Union(elements) => elements
                .iter()
                .any(|element| self.contains(element, variable, seen)),
            Term::Known(_) => false,
        }
    }

    pub(super) fn solve_product_indices(&mut self) -> Result<bool> {
        let mut progress = false;
        let mut waiting = Vec::new();
        for constraint in std::mem::take(&mut self.indices) {
            match self.resolve(&constraint.object) {
                Term::Var(_) => waiting.push(constraint),
                Term::Tuple(elements) => {
                    if constraint.literal {
                        let element = super::super::products::product_index_position(
                            constraint.index,
                            elements.len(),
                        )
                        .and_then(|index| elements.get(index))
                        .ok_or_else(|| type_error("tuple index out of range", constraint.place))?;
                        self.unify(&constraint.result, element, constraint.place)?;
                    }
                    progress = true;
                }
                Term::Array(element) => {
                    let value = if crate::translation::array_value_rules::select_optional_array_read(
                        "JavaScript",
                        constraint.optional,
                        constraint.proven_in_bounds,
                        false,
                    ) {
                        super::optional_values::optional(*element, "undefined")
                    } else {
                        *element
                    };
                    self.unify(&constraint.result, &value, constraint.place)?;
                    if let Term::Var(variable) = constraint.result {
                        self.bindings[variable] = Some(value);
                    }
                    progress = true;
                }
                _ => {
                    // A known nonarray receiver reaches the ordinary checker;
                    // inference does not claim it has an array layout.
                    progress = true;
                }
            }
        }
        self.indices = waiting;
        Ok(progress)
    }
}
