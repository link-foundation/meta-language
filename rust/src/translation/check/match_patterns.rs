impl Checker {
    pub(super) fn literal_pattern(
        &mut self,
        value: SExpr,
        ty: &Type,
        span: Option<Span>,
    ) -> Result<NPat> {
        let checked = self.expr(&value, &Env::default(), &[], Some(ty), false)?;
        if !checked.ty.same(ty) {
            return Err(type_error(
                format!(
                    "literal pattern of type {} on {}",
                    checked.ty.key(),
                    ty.key()
                ),
                span,
            ));
        }
        let text = match &checked.node {
            Node::Lit { value } => value.text(),
            _ => String::new(),
        };
        Ok(NPat::Lit {
            value: Box::new(value),
            key: format!("{}:{text}", ty.key()),
        })
    }

    pub(super) fn match_cases(
        &mut self,
        scrutinee: &SExpr,
        cases: &[SCase],
        span: Option<Span>,
        env: &Env,
        path: &[String],
        expected: ExpectedType<'_>,
    ) -> Result<Expr> {
        let variable = env.local(scrutinee).map(str::to_owned);
        // Links may retain a nested match inside a source-authoring projection.
        // Check its cases without running the source-only specialization again.
        if !self.checked_projection
            && let Some(variable) = &variable
            && let Some(fact) = env.known.get(variable)
        {
            let fact = fact.clone();
            return self.known_match(cases, span, &fact, variable, env, path, expected);
        }
        let scrutinee = self.expr(scrutinee, env, path, None, false)?;
        let ty = scrutinee.ty.clone();
        let mut pending = Vec::new();
        for kase in cases {
            let mut inner = env.clone();
            let pattern = self.pattern(&kase.pattern, &ty, &mut inner, kase.span.or(span))?;
            if let (Some(variable), Pattern::Ctor { ctor, binds, .. }) = (&variable, &pattern)
                && binds.iter().all(Option::is_some)
                && !binds.iter().flatten().any(|bind| bind == variable)
            {
                inner.known.insert(
                    variable.clone(),
                    Fact {
                        ctor: ctor.clone(),
                        binds: binds.iter().flatten().cloned().collect(),
                    },
                );
            }
            pending.push((pattern, &kase.body, inner));
        }
        let partial_callable =
            crate::translation::partial_callable_rules::require_partial_callable_result_inference(
                expected.kind(),
                expected.result().is_some(),
            );
        let mut result_type = expected.concrete().cloned();
        let mut untyped_absence: Option<String> = None;
        let mut checked = Vec::new();
        for (pattern, surface, inner) in pending {
            let context = if partial_callable {
                expected
            } else {
                result_type
                    .as_ref()
                    .map_or(expected, |ty| ExpectedType::Concrete(Some(ty)))
            };
            let body = self.expr_with_context(surface, &inner, path, context, true)?;
            if !body.ty.is_literal() && result_type.is_none() {
                if self.language == Language::JavaScript
                    && let Node::None { absence } = &body.node
                    && matches!(&body.ty, Type::Option { element, .. } if **element == Type::Unit)
                {
                    untyped_absence = Some(match untyped_absence {
                        Some(previous) => merge_optional_absence(&previous, absence),
                        None => absence.clone(),
                    });
                } else if let Some(absence) = untyped_absence.take() {
                    result_type = Some(match &body.ty {
                        Type::Option {
                            element,
                            absence: present_absence,
                        } => Type::Option {
                            element: element.clone(),
                            absence: merge_optional_absence(present_absence, &absence),
                        },
                        ty => Type::Option {
                            element: Box::new(ty.clone()),
                            absence,
                        },
                    });
                } else {
                    result_type = Some(body.ty.clone());
                }
            }
            checked.push((pattern, body, surface, inner));
        }
        let result_type = result_type.unwrap_or_else(|| match untyped_absence {
            Some(absence) => Type::Option {
                element: Box::new(Type::Unit),
                absence,
            },
            None => self.default_number(),
        });
        let mut final_cases = Vec::new();
        for (pattern, body, surface, inner) in checked {
            let body = if body.ty.is_literal() {
                self.expr(surface, &inner, path, Some(&result_type), false)?
            } else {
                body
            };
            let body = self.coerce(body, &result_type, surface.span.or(span))?;
            final_cases.push(Case { pattern, body });
        }
        self.check_exhaustive(&ty, &final_cases, span)?;
        Ok(Expr::new(
            Node::Match {
                scrutinee: Box::new(scrutinee),
                cases: final_cases,
            },
            result_type,
        ))
    }

    /// Inside a case that matched `variable` against a constructor, a nested
    /// match on the same variable can only take that constructor's case: it is
    /// the case's body with the fields bound to the outer case's locals. The
    /// value is unchanged, and structural recursion stays visible to Lean and
    /// Rocq, which do not relate the inner match to the outer one.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn known_match(
        &mut self,
        cases: &[SCase],
        span: Option<Span>,
        fact: &Fact,
        variable: &str,
        env: &Env,
        path: &[String],
        expected: ExpectedType<'_>,
    ) -> Result<Expr> {
        let Some(kase) = cases.iter().find(|candidate| match &candidate.pattern {
            SCasePattern::Ctor { path, .. } => path.last() == Some(&fact.ctor),
            SCasePattern::Wild | SCasePattern::Bind { .. } => true,
            _ => false,
        }) else {
            return Err(type_error(
                format!("non-exhaustive match: no case for {}", fact.ctor),
                span,
            ));
        };
        let alias_span = kase.span.or(span);
        let alias = |name: &str, value: &str, body: SExpr| {
            if name == "_" || name == value {
                body
            } else {
                surface_let(name, value, body, alias_span)
            }
        };
        let mut body = kase.body.clone();
        if let SCasePattern::Bind { name } = &kase.pattern {
            body = alias(name, variable, body);
        }
        if let SCasePattern::Ctor { binds, .. } = &kase.pattern {
            if binds.len() != fact.binds.len() {
                return Err(type_error(
                    format!(
                        "{} pattern binds {} of {} fields",
                        fact.ctor,
                        binds.len(),
                        fact.binds.len()
                    ),
                    alias_span,
                ));
            }
            // Simultaneous binding: every alias reads an outer local before any is shadowed.
            let inner: Vec<(String, String)> = binds
                .iter()
                .zip(&fact.binds)
                .filter_map(|(name, value)| {
                    name.as_ref()
                        .filter(|name| !name.is_empty() && *name != "_" && *name != value)
                        .map(|name| (name.clone(), value.clone()))
                })
                .collect();
            if inner
                .iter()
                .any(|(_, value)| inner.iter().any(|(name, _)| name == value))
            {
                let temporaries: Vec<(String, String)> = inner
                    .iter()
                    .map(|(_, value)| (self.fresh_name("ml_k"), value.clone()))
                    .collect();
                for (index, (name, _)) in inner.iter().enumerate().rev() {
                    body = alias(name, &temporaries[index].0, body);
                }
                for (name, value) in temporaries.iter().rev() {
                    body = alias(name, value, body);
                }
            } else {
                for (name, value) in inner.iter().rev() {
                    body = alias(name, value, body);
                }
            }
        }
        // `knownMatch` passes the caller's `allowLiteral`; `match` is only
        // reached with the default `false`.
        self.expr_with_context(&body, env, path, expected, false)
    }

    pub(super) fn pattern(
        &self,
        pattern: &SCasePattern,
        ty: &Type,
        env: &mut Env,
        span: Option<Span>,
    ) -> Result<Pattern> {
        match pattern {
            SCasePattern::Wild => Ok(Pattern::Wild),
            SCasePattern::Bind { name } => {
                env.bind_local(name, ty.clone());
                Ok(Pattern::Bind { name: name.clone() })
            }
            SCasePattern::NatZero | SCasePattern::NatSucc { .. } => {
                if *ty != Type::Nat {
                    return Err(type_error(
                        format!("natural-number pattern on {}", ty.key()),
                        span,
                    ));
                }
                if let SCasePattern::NatSucc { name } = pattern {
                    env.bind_local(name, ty.clone());
                    return Ok(Pattern::NatSucc { name: name.clone() });
                }
                Ok(Pattern::NatZero)
            }
            SCasePattern::Ctor { path, binds } => {
                let Type::Data { name: data_name } = ty else {
                    return Err(type_error(
                        format!("constructor pattern on {}", ty.key()),
                        span,
                    ));
                };
                let ctor_name = path.last().cloned().unwrap_or_default();
                let Some(ctor) = self
                    .data_decl(data_name)?
                    .ctors
                    .iter()
                    .find(|ctor| ctor.name == ctor_name)
                    .cloned()
                else {
                    return Err(type_error(
                        format!("{data_name} has no constructor {ctor_name}"),
                        span,
                    ));
                };
                if binds.len() != ctor.fields.len() {
                    return Err(type_error(
                        format!(
                            "{ctor_name} pattern binds {} of {} fields",
                            binds.len(),
                            ctor.fields.len()
                        ),
                        span,
                    ));
                }
                let mut checked = Vec::new();
                for (bind, field) in binds.iter().zip(&ctor.fields) {
                    match bind {
                        Some(name) if name != "_" => {
                            env.bind_local(name, field.ty.clone());
                            checked.push(Some(name.clone()));
                        }
                        _ => checked.push(None),
                    }
                }
                Ok(Pattern::Ctor {
                    data: data_name.clone(),
                    ctor: ctor_name,
                    binds: checked,
                })
            }
        }
    }

    pub(super) fn check_exhaustive(
        &self,
        ty: &Type,
        cases: &[Case],
        span: Option<Span>,
    ) -> Result<()> {
        let patterns: Vec<&Pattern> = cases.iter().map(|kase| &kase.pattern).collect();
        if patterns
            .iter()
            .any(|pattern| matches!(pattern, Pattern::Wild | Pattern::Bind { .. }))
        {
            return Ok(());
        }
        if let Type::Data { name } = ty {
            let covered: HashSet<&str> = patterns
                .iter()
                .filter_map(|pattern| match pattern {
                    Pattern::Ctor { ctor, .. } => Some(ctor.as_str()),
                    _ => None,
                })
                .collect();
            let missing: Vec<&str> = self
                .data_decl(name)?
                .ctors
                .iter()
                .map(|ctor| ctor.name.as_str())
                .filter(|ctor| !covered.contains(ctor))
                .collect();
            if !missing.is_empty() {
                return Err(type_error(
                    format!("match on {name} misses {}", missing.join(", ")),
                    span,
                ));
            }
            return Ok(());
        }
        if ty.is_natural()
            && patterns
                .iter()
                .any(|pattern| matches!(pattern, Pattern::NatZero))
            && patterns
                .iter()
                .any(|pattern| matches!(pattern, Pattern::NatSucc { .. }))
        {
            return Ok(());
        }
        Err(type_error(
            format!("non-exhaustive match on {}", ty.key()),
            span,
        ))
    }
}
