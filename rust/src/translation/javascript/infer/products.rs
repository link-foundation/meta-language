//! Product expression observations from the existing inference engine.

use super::{
    BOOL, Env, HashMap, HashSet, Inference, ROOT, Result, SExpr, SItem, SNode, SProgram, Span,
    Term, Type, collect_functions, fill_arrays, fill_fields, fill_functions, type_error,
};
use crate::translation::frontend_rules::accept_argument_count;

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
        && functions.iter().all(|(_, function)| {
            function.ret.is_some() && function.params.iter().all(|param| param.ty.is_some())
        })
    {
        return Ok((program, HashMap::new()));
    }
    let mut inference = Inference::new(&program.items);
    inference.record_expressions = observe;
    let inferred = inference.run(&functions, &program)?;
    let expression_types = if observe {
        inference
            .expression_terms
            .iter()
            .map(|(node, (term, place))| (*node, inference.surface(term, *place)))
            .collect()
    } else {
        HashMap::new()
    };
    let arrays: HashMap<*const SExpr, Type> = inference
        .empty_arrays
        .iter()
        .map(|(node, (term, place))| (*node, inference.surface(term, *place)))
        .collect();
    let mut inferred = inferred.into_iter();
    fill_functions(&mut program.items, &mut inferred);
    fill_fields(&mut program.items, &inference);
    if !arrays.is_empty() {
        fill_arrays(&mut program, &arrays);
    }
    Ok((program, expression_types))
}

impl Inference {
    pub(super) fn product_condition(&mut self, term: &Term, place: Option<Span>) -> Result<()> {
        // The source fact collector records whole product truthiness as its own obligation.
        if self.record_expressions && matches!(self.resolve(term), Term::Tuple(_)) {
            return Ok(());
        }
        self.unify(term, &Term::Known(BOOL), place)
    }

    /// Apply the author's positional expected type without turning a source array into owned evidence.
    pub(super) fn expr_expected(
        &mut self,
        node: &SExpr,
        env: &Env,
        expected: Option<&Term>,
    ) -> Result<Term> {
        let expected = expected.map(|term| self.resolve(term));
        let Some(expected) = expected.filter(|_| self.record_expressions) else {
            return self.expr(node, env);
        };
        let term = match (&node.node, &expected) {
            (SNode::Array { items, .. }, Term::Tuple(elements))
                if items.len() == elements.len() && items.iter().all(|item| !item.spread) =>
            {
                Term::Tuple(
                    items
                        .iter()
                        .zip(elements)
                        .map(|(item, element)| self.expr_expected(&item.value, env, Some(element)))
                        .collect::<Result<Vec<_>>>()?,
                )
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
                self.product_condition(&condition, cond.span.or(node.span))?;
                let yes = self.expr_expected(then, env, Some(&expected))?;
                let no = self.expr_expected(otherwise, env, Some(&expected))?;
                self.unify(&yes, &no, node.span)?;
                yes
            }
            _ => return self.expr(node, env),
        };
        self.expression_terms
            .insert(std::ptr::from_ref(node), (term.clone(), node.span));
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
        for ((term, param), arg) in arg_terms.iter().zip(&signature.params).zip(args) {
            self.unify(term, param, arg.span.or(place))?;
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
            Term::Array(element) => self.contains(element, variable, seen),
            Term::Tuple(elements) => elements
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
                    self.unify(&constraint.result, &element, constraint.place)?;
                    progress = true;
                }
                object => {
                    let element = self.fresh(false);
                    self.unify(
                        &object,
                        &Term::Array(Box::new(element.clone())),
                        constraint.place,
                    )?;
                    self.unify(&constraint.result, &element, constraint.place)?;
                    progress = true;
                }
            }
        }
        self.indices = waiting;
        Ok(progress)
    }
}
