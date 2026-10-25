//! Existing whole-program constraint scheduling, mechanically relocated.

use super::{
    BOOL, Env, FunctionResult, Inference, ROOT, Result, SEffect, SFn, SProgram, Signature, Term,
    Type,
};

type InferredSignature = (Vec<Type>, Type, Option<Type>);

impl Inference {
    pub(super) fn run(
        &mut self,
        functions: &[(Vec<String>, &SFn)],
        program: &SProgram,
    ) -> Result<Vec<InferredSignature>> {
        // The other items of the module keep the types their own translation checked.
        for external in &program.externals {
            let (params, ret) = external.signature();
            let signature = Signature {
                defaults: vec![false; params.len()],
                optional_params: vec![false; params.len()],
                params: params
                    .into_iter()
                    .map(|ty| self.declared(Some(ty)))
                    .collect(),
                ret: self.declared(Some(ret)),
                declared_ret: None,
            };
            self.signatures
                .insert(format!("{ROOT}.{}", external.name()), signature);
        }
        for (path, function) in functions {
            let params = function
                .params
                .iter()
                .map(|param| self.declared(param.ty.as_ref()))
                .collect();
            let ret = self.fresh(false);
            let declared_ret = function.ret.as_ref().map(|ty| self.declared(Some(ty)));
            if let Some(declared) = &declared_ret {
                self.unify(&ret, declared, function.span)?;
            }
            let defaults = function
                .params
                .iter()
                .map(|param| param.default_value.is_some() || param.optional)
                .collect();
            self.signatures.insert(
                path.join("."),
                Signature {
                    params,
                    defaults,
                    optional_params: function.params.iter().map(|param| param.optional).collect(),
                    ret,
                    declared_ret,
                },
            );
        }
        for (path, function) in functions {
            let signature = self.signatures[&path.join(".")].clone();
            self.context = path[..path.len() - 1].to_vec();
            let mut initialization = Env::new();
            for (param, term) in function.params.iter().zip(&signature.params) {
                if let Some(default_value) = &param.default_value {
                    let value = self.expr_expected(default_value, &initialization, Some(term))?;
                    self.unify(&value, term, param.span)?;
                }
                initialization.insert(param.name.clone(), term.clone());
            }
        }
        for (path, function) in functions {
            let signature = self.signatures[&path.join(".")].clone();
            let ret = signature.ret.clone();
            let env: Env = function
                .params
                .iter()
                .zip(&signature.params)
                .map(|(param, term)| (param.name.clone(), term.clone()))
                .collect();
            self.context = path[..path.len() - 1].to_vec();
            let body = self.expr_expected(&function.body, &env, Some(&ret))?;
            self.unify(&body, &ret, function.span)?;
            self.results.push(FunctionResult {
                result: ret.clone(),
                declared: signature.declared_ret.clone(),
                body: body.clone(),
            });
            if function.ret.is_none()
                && let Term::Var(variable) = ret
                && matches!(self.resolve(&body), Term::Option { .. })
            {
                self.bindings[variable] = Some(body);
            }
        }
        self.context = vec![ROOT.to_owned()];
        let mut env = Env::new();
        for effect in program.main.iter().flat_map(|main| &main.effects) {
            match effect {
                SEffect::Print { expr, .. } => {
                    self.expr(expr, &env)?;
                }
                SEffect::Let { name, value, .. } => {
                    let term = self.expr(value, &env)?;
                    env.insert(name.clone(), term);
                }
                SEffect::Assert { prop, .. } => self.prop(prop, &env)?,
            }
        }
        self.solve()?;
        Ok(functions
            .iter()
            .enumerate()
            .map(|(index, (path, function))| {
                let signature = &self.signatures[&path.join(".")];
                let params = signature
                    .params
                    .iter()
                    .map(|term| self.surface(term, function.span))
                    .collect();
                let result = &self.results[index];
                let semantic_ret = result
                    .declared
                    .as_ref()
                    .filter(|declared| self.additional_absence(&result.body, declared))
                    .map(|_| self.surface(&result.body, function.span));
                (
                    params,
                    self.surface(&signature.ret, function.span),
                    semantic_ret,
                )
            })
            .collect())
    }

    /// Deferred `+` and field constraints, until nothing more is known; then `+` of unknowns adds numbers.
    pub(super) fn solve(&mut self) -> Result<()> {
        let mut progress = true;
        while progress {
            progress = false;
            let mut waiting = Vec::new();
            for constraint in std::mem::take(&mut self.plus) {
                if self.plus_step(&constraint, false)? {
                    progress = true;
                } else {
                    waiting.push(constraint);
                }
            }
            self.plus = waiting;
            let mut waiting = Vec::new();
            for constraint in std::mem::take(&mut self.fields) {
                if self.field_step(&constraint)? {
                    progress = true;
                } else {
                    waiting.push(constraint);
                }
            }
            // Field refinement appends its actual present-value constraint,
            // just as the JavaScript snapshot iteration retains appended rows.
            waiting.extend(std::mem::take(&mut self.fields));
            self.fields = waiting;
            progress |= self.solve_methods()?;
            progress |= self.solve_includes(false)?;
            progress |= self.solve_optional()?;
            progress |= self.solve_array_formal_iterables()?;
            progress |= self.solve_recursive_binding_iterables()?;
            progress |= self.solve_record_formal_fields()?;
            progress |= self.solve_object_rest_fields()?;
            progress |= self.solve_iterables()?;
            progress |= self.solve_product_indices()?;
            progress |= self.solve_array_binding_defaults()?;
            progress |= self.solve_arithmetic(false)?;
            progress |= self.solve_joins_and_results()?;
            progress |= self.solve_record_spreads()?;
            progress |= self.solve_equalities(false)?;
            progress |= self.solve_enumerations()?;
            if !progress && !self.includes.is_empty() {
                progress |= self.solve_includes(true)?;
            }
            if !progress {
                progress |= self.force_method_owner()?;
            }
            // `.length` of a value nothing else fixes reads an array.
            let length = self.fields.iter().find(|constraint| {
                constraint.field == "length"
                    && matches!(self.resolve(&constraint.object), Term::Var(_))
            });
            if let (false, Some(length)) = (progress, length) {
                let (object, place) = (length.object.clone(), length.place);
                let array = Term::Array(Box::new(self.fresh(false)));
                self.unify(&object, &array, place)?;
                progress = true;
            }
            if !progress && !self.coalesces.is_empty() {
                let first = self.coalesces.remove(0);
                self.coalescing_step(&first, true)?;
                progress = true;
            }
            if !progress {
                let unknown = self
                    .array_formal_iterables
                    .iter()
                    .chain(self.recursive_binding_iterables.iter())
                    .chain(self.iterables.iter())
                    .find(|constraint| matches!(self.resolve(&constraint.object), Term::Var(_)));
                if let Some(iterable) = unknown {
                    let object = iterable.object.clone();
                    let place = iterable.place;
                    let element = self.fresh(false);
                    self.unify(&object, &Term::Array(Box::new(element)), place)?;
                    progress = true;
                }
            }
            if !progress {
                let unknown = self
                    .indices
                    .iter()
                    .find(|constraint| matches!(self.resolve(&constraint.object), Term::Var(_)));
                if let Some(indexed) = unknown {
                    let object = indexed.object.clone();
                    let place = indexed.place;
                    let element = self.fresh(false);
                    self.unify(&object, &Term::Array(Box::new(element)), place)?;
                    progress = true;
                }
            }
            if !progress && !self.plus.is_empty() {
                let first = self.plus.remove(0);
                self.plus_step(&first, true)?;
                progress = true;
            }
            if !progress && !self.arithmetic.is_empty() {
                progress |= self.solve_arithmetic(true)?;
            }
            if !progress && !self.equalities.is_empty() {
                progress |= self.solve_equalities(true)?;
            }
            if !progress
                && let Some((term, place)) = self
                    .conditions
                    .iter()
                    .find(|(term, _)| matches!(self.resolve(term), Term::Var(_)))
                    .cloned()
            {
                self.unify(&term, &Term::Known(BOOL), place)?;
                progress = true;
            }
        }
        if let Some(constraint) = self.record_spreads.first() {
            return Err(super::type_error(
                "an object spread needs a known record shape; declare its fields with JSDoc",
                constraint.place,
            ));
        }
        if let Some(constraint) = self.record_formal_fields.first() {
            return Err(super::type_error(
                "record formal default needs a known represented field domain",
                constraint.place,
            ));
        }
        if let Some(constraint) = self.recursive_binding_iterables.first() {
            return Err(super::type_error(
                "a recursive binding needs a represented iterable",
                constraint.place,
            ));
        }
        if let Some(constraint) = self.object_rest_fields.first() {
            return Err(super::type_error(
                "object rest needs a known complete own-field domain",
                constraint.place,
            ));
        }
        if let Some(constraint) = self.array_binding_defaults.first() {
            return Err(super::type_error(
                "array binding default needs a known array element type",
                constraint.place,
            ));
        }
        if let Some(constraint) = self.enumerations.first() {
            return Err(super::type_error(
                "object value enumeration needs its actual complete value domain",
                constraint.place,
            ));
        }
        Ok(())
    }
}
