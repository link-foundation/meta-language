//! Pattern-match compilation and exhaustiveness.

use super::{
    BinaryOp, Case, Checker, Column, Ctor, Decimal, Entry, Env, Expr, Fact, Field, HashSet,
    Language, NAT_PATTERN_UNFOLD, NPat, Node, Pattern, Result, Row, SCase, SCasePattern, SExpr,
    SNode, SPatternNode, SRow, Span, Type, decrement, surface_let, type_error, unsupported,
};

impl Checker {
    /// Compiles a surface match (several scrutinees, nested and literal
    /// patterns) into single-level constructor matches and literal if-chains.
    /// Bind patterns name the constructor fields directly so structural
    /// recursion stays visible to the recursion analysis.
    pub(super) fn compile_match(
        &mut self,
        scrutinees: &[SExpr],
        rows: &[SRow],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<SExpr> {
        let mut lets = Vec::new();
        let mut columns = Vec::new();
        for scrutinee in scrutinees {
            let checked = self.expr(scrutinee, env, path, None, false)?;
            if let Some(name) = env.local(scrutinee) {
                columns.push(Column {
                    name: name.to_owned(),
                    ty: checked.ty,
                });
                continue;
            }
            let name = self.fresh_name("ml_s");
            lets.push((name.clone(), scrutinee.clone()));
            columns.push(Column {
                name,
                ty: checked.ty,
            });
        }
        let rows = rows
            .iter()
            .map(|row| Row {
                patterns: row.patterns.iter().cloned().map(NPat::Surface).collect(),
                body: row.body.clone(),
                binds: Vec::new(),
            })
            .collect();
        let mut body = self.compile_rows(&columns, rows, span)?;
        for (name, value) in lets.into_iter().rev() {
            body = SExpr::new(
                SNode::Let {
                    name,
                    ty: None,
                    value: Box::new(value),
                    body: Box::new(body),
                },
                span,
            );
        }
        Ok(body)
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn compile_rows(
        &mut self,
        columns: &[Column],
        rows: Vec<Row>,
        span: Option<Span>,
    ) -> Result<SExpr> {
        if rows.is_empty() {
            return Err(type_error("non-exhaustive match", span));
        }
        let mut normalised = Vec::with_capacity(rows.len());
        for row in rows {
            let mut patterns = Vec::with_capacity(row.patterns.len());
            for (index, pattern) in row.patterns.into_iter().enumerate() {
                let Some(column) = columns.get(index) else {
                    return Err(type_error("non-exhaustive match", span));
                };
                patterns.push(self.normalise_pattern(pattern, &column.ty, span)?);
            }
            normalised.push(Row { patterns, ..row });
        }
        let first = &normalised[0];
        let Some(refutable) = first
            .patterns
            .iter()
            .position(|pattern| !pattern.irrefutable())
        else {
            let mut binds = first.binds.clone();
            for (pattern, column) in first.patterns.iter().zip(columns) {
                if let NPat::Bind(name) = pattern {
                    binds.push((name.clone(), column.name.clone()));
                }
            }
            let mut body = first.body.clone();
            for (name, variable) in binds.iter().rev() {
                if name != variable {
                    body = surface_let(name, variable, body, span);
                }
            }
            return Ok(body);
        };
        let column = columns[refutable].clone();
        let pivot = first.patterns[refutable].clone();
        let rest = |patterns: &[NPat]| -> Vec<NPat> {
            patterns
                .iter()
                .enumerate()
                .filter(|(index, _)| *index != refutable)
                .map(|(_, pattern)| pattern.clone())
                .collect()
        };
        let bind_column = |row: &Row| -> Vec<(String, String)> {
            let mut binds = row.binds.clone();
            if let NPat::Bind(name) = &row.patterns[refutable] {
                binds.push((name.clone(), column.name.clone()));
            }
            binds
        };
        let remaining: Vec<Column> = columns
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != refutable)
            .map(|(_, column)| column.clone())
            .collect();
        // A natural column that also holds successor patterns splits on zero and
        // successor, which decrements its literal patterns.
        let nat = column.ty == Type::Nat
            && normalised
                .iter()
                .any(|row| matches!(row.patterns[refutable], NPat::Nat { .. }));
        if let (
            NPat::Lit {
                value: pivot_value,
                key: pivot_key,
            },
            false,
        ) = (&pivot, nat)
        {
            // Literal patterns on integers, booleans and strings become equality tests.
            let same = |row: &Row| row.patterns[refutable].lit_key() == Some(pivot_key.as_str());
            let matching: Vec<Row> = normalised
                .iter()
                .filter(|row| row.patterns[refutable].irrefutable() || same(row))
                .map(|row| Row {
                    patterns: rest(&row.patterns),
                    binds: bind_column(row),
                    body: row.body.clone(),
                })
                .collect();
            // Where a boolean column is not the pivot it is the other value, so
            // its remaining literal patterns always match there.
            let boolean = column.ty == Type::Bool;
            let others: Vec<Row> = normalised
                .iter()
                .filter(|row| !same(row))
                .map(|row| {
                    let mut row = row.clone();
                    if boolean && matches!(row.patterns[refutable], NPat::Lit { .. }) {
                        row.patterns[refutable] = NPat::Wild;
                    }
                    row
                })
                .collect();
            let then = self.compile_rows(&remaining, matching, span)?;
            if boolean && others.is_empty() {
                return Err(type_error("non-exhaustive match on bool", span));
            }
            let otherwise = self.compile_rows(columns, others, span)?;
            return Ok(SExpr::new(
                SNode::If {
                    cond: Box::new(SExpr::new(
                        SNode::Binary {
                            op: BinaryOp::Eq,
                            left: Box::new(SExpr::name(&column.name)),
                            right: Box::new(pivot_value.clone()),
                            rounding: None,
                        },
                        None,
                    )),
                    then: Box::new(then),
                    otherwise: Box::new(otherwise),
                },
                span,
            ));
        }
        let ctors = if nat {
            vec![
                Ctor {
                    name: "zero".to_owned(),
                    fields: Vec::new(),
                },
                Ctor {
                    name: "succ".to_owned(),
                    fields: vec![Field {
                        name: String::new(),
                        ty: column.ty.clone(),
                    }],
                },
            ]
        } else {
            let name = column.ty.data_name().unwrap_or_default().to_owned();
            self.data_decl(&name)?.ctors.clone()
        };
        let mut cases = Vec::new();
        for ctor in &ctors {
            let mut field_names = Vec::new();
            for field_index in 0..ctor.fields.len() {
                let named = normalised.iter().find_map(|row| {
                    let pattern = &row.patterns[refutable];
                    if pattern.ctor() != Some(ctor.name.as_str()) {
                        return None;
                    }
                    match pattern.args().get(field_index) {
                        Some(NPat::Bind(name)) => Some(name.clone()),
                        _ => None,
                    }
                });
                let name = named.unwrap_or_else(|| self.fresh_name("ml_f"));
                field_names.push(name);
            }
            let field_columns: Vec<Column> = ctor
                .fields
                .iter()
                .zip(&field_names)
                .map(|(field, name)| Column {
                    name: name.clone(),
                    ty: field.ty.clone(),
                })
                .collect();
            let mut specialised = Vec::new();
            for row in &normalised {
                let pattern = &row.patterns[refutable];
                if pattern.irrefutable() {
                    let mut patterns = vec![NPat::Wild; field_columns.len()];
                    patterns.extend(rest(&row.patterns));
                    specialised.push(Row {
                        patterns,
                        binds: bind_column(row),
                        body: row.body.clone(),
                    });
                } else if pattern.ctor() == Some(ctor.name.as_str()) {
                    let mut patterns = pattern.args().to_vec();
                    patterns.extend(rest(&row.patterns));
                    specialised.push(Row {
                        patterns,
                        ..row.clone()
                    });
                } else if let Some(value) = pattern.nat_value() {
                    let zero = value == Decimal::from_i128(0);
                    if ctor.name == "zero" && zero {
                        specialised.push(Row {
                            patterns: rest(&row.patterns),
                            ..row.clone()
                        });
                    }
                    if ctor.name == "succ" && !zero {
                        let predecessor = SExpr::new(
                            SNode::Num {
                                value: decrement(&value),
                                ty: None,
                                negative: false,
                                unit: false,
                            },
                            None,
                        );
                        let mut patterns =
                            vec![self.literal_pattern(predecessor, &column.ty, span)?];
                        patterns.extend(rest(&row.patterns));
                        specialised.push(Row {
                            patterns,
                            ..row.clone()
                        });
                    }
                }
            }
            let mut body_columns = field_columns;
            body_columns.extend(remaining.iter().cloned());
            let body = self.compile_rows(&body_columns, specialised, span)?;
            let pattern = if nat {
                if ctor.name == "zero" {
                    SCasePattern::NatZero
                } else {
                    SCasePattern::NatSucc {
                        name: field_names[0].clone(),
                    }
                }
            } else {
                SCasePattern::Ctor {
                    path: vec![ctor.name.clone()],
                    binds: field_names.into_iter().map(Some).collect(),
                }
            };
            cases.push(SCase {
                pattern,
                body,
                span: None,
            });
        }
        Ok(SExpr::new(
            SNode::Match1 {
                scrutinee: Box::new(SExpr::name(&column.name)),
                cases,
            },
            span,
        ))
    }

    /// Resolves a surface pattern against the column type: constructors, naturals, literals, binders.
    pub(super) fn normalise_pattern(
        &mut self,
        pattern: NPat,
        ty: &Type,
        span: Option<Span>,
    ) -> Result<NPat> {
        let NPat::Surface(pattern) = pattern else {
            return Ok(pattern);
        };
        let natural = *ty == Type::Nat;
        let at = pattern.span.or(span);
        let succ = |inner: NPat| NPat::Nat {
            succ: true,
            args: vec![inner],
        };
        let zero = || NPat::Nat {
            succ: false,
            args: Vec::new(),
        };
        match pattern.node {
            SPatternNode::Wild => Ok(NPat::Wild),
            SPatternNode::Bind { name } => Ok(NPat::Bind(name)),
            SPatternNode::BindOrCtor { name } => {
                if let Type::Data { name: data_name } = ty
                    && let Some(ctor) = self
                        .data_decl(data_name)?
                        .ctors
                        .iter()
                        .find(|ctor| ctor.name == name)
                {
                    if !ctor.fields.is_empty() {
                        return Err(type_error(
                            format!("{name} pattern needs {} fields", ctor.fields.len()),
                            at,
                        ));
                    }
                    return Ok(NPat::Data {
                        ctor: ctor.name.clone(),
                        args: Vec::new(),
                    });
                }
                if natural && self.language == Language::Rocq && name == "O" {
                    return Ok(zero());
                }
                if *ty == Type::Bool && (name == "true" || name == "false") {
                    let value = SExpr::new(
                        SNode::Bool {
                            value: name == "true",
                        },
                        None,
                    );
                    return self.literal_pattern(value, ty, span);
                }
                Ok(NPat::Bind(name))
            }
            SPatternNode::NumLit { value, negative } => {
                // A JavaScript `case 0n:` is an `===` test, which the recursion analysis reads as a zero test.
                // Numerals up to NAT_PATTERN_UNFOLD unfold to successor chains; larger
                // ones are equality tests, so a pattern never costs a node per unit.
                let count = Decimal::parse(&value)
                    .and_then(|value| value.to_i128())
                    .filter(|count| *count <= NAT_PATTERN_UNFOLD);
                if let (true, false, Some(count)) =
                    (natural, self.language == Language::JavaScript, count)
                {
                    let mut result = zero();
                    for _ in 0..count {
                        result = succ(result);
                    }
                    return Ok(result);
                }
                let value = SExpr::new(
                    SNode::Num {
                        value,
                        ty: None,
                        negative,
                        unit: false,
                    },
                    None,
                );
                self.literal_pattern(value, ty, span)
            }
            SPatternNode::BoolLit { value, .. } => {
                self.literal_pattern(SExpr::new(SNode::Bool { value }, None), ty, span)
            }
            SPatternNode::StrLit { value } => {
                self.literal_pattern(SExpr::new(SNode::Str { value }, None), ty, span)
            }
            SPatternNode::NatAdd { inner, add } => {
                if !natural {
                    return Err(unsupported(
                        "n + k pattern",
                        &format!(
                            "only natural-number scrutinees support n + k patterns, not {}",
                            ty.key()
                        ),
                        at,
                    ));
                }
                let Some(add) = add
                    .parse::<i128>()
                    .ok()
                    .filter(|add| *add <= NAT_PATTERN_UNFOLD)
                else {
                    return Err(unsupported(
                        "n + k pattern",
                        &format!("n + k patterns take offsets of at most {NAT_PATTERN_UNFOLD}"),
                        at,
                    ));
                };
                let mut result = self.normalise_pattern(NPat::Surface(*inner), ty, span)?;
                for _ in 0..add {
                    result = succ(result);
                }
                Ok(result)
            }
            SPatternNode::Ctor { path, args } => {
                let name = path.last().cloned().unwrap_or_default();
                if natural && args.len() == 1 && (name == "succ" || name == "S") {
                    let arg = args.into_iter().next().map_or(NPat::Wild, NPat::Surface);
                    return Ok(succ(self.normalise_pattern(arg, ty, span)?));
                }
                if natural && args.is_empty() && (name == "zero" || name == "O") {
                    return Ok(zero());
                }
                let Type::Data { name: data_name } = ty else {
                    return Err(type_error(
                        format!("constructor pattern {name} on {}", ty.key()),
                        at,
                    ));
                };
                let decl = self.data_decl(data_name)?;
                let module_path = decl.module_path.clone();
                let ctor = decl.ctors.iter().find(|ctor| ctor.name == name).cloned();
                if path.len() > 1 {
                    let owner = match self.lookup(&path, &[], at)? {
                        Some(owner) => Some(owner),
                        None => self.lookup(&path, &module_path, at)?,
                    };
                    if !matches!(&owner, Some(Entry::Ctor { data, .. }) if data == data_name) {
                        return Err(type_error(
                            format!("{} is not a constructor of {data_name}", path.join(".")),
                            at,
                        ));
                    }
                }
                let Some(ctor) = ctor else {
                    return Err(type_error(
                        format!("{data_name} has no constructor {name}"),
                        at,
                    ));
                };
                if args.len() != ctor.fields.len() {
                    return Err(type_error(
                        format!(
                            "{name} pattern has {} of {} fields",
                            args.len(),
                            ctor.fields.len()
                        ),
                        at,
                    ));
                }
                let mut normalised = Vec::new();
                for (arg, field) in args.into_iter().zip(&ctor.fields) {
                    normalised.push(self.normalise_pattern(NPat::Surface(arg), &field.ty, span)?);
                }
                Ok(NPat::Data {
                    ctor: name,
                    args: normalised,
                })
            }
        }
    }

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
            value,
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
        expected: Option<&Type>,
    ) -> Result<Expr> {
        let variable = env.local(scrutinee).map(str::to_owned);
        if let Some(variable) = &variable
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
        let mut result_type = expected.cloned();
        let mut checked = Vec::new();
        for (pattern, surface, inner) in pending {
            let body = self.expr(surface, &inner, path, result_type.as_ref(), true)?;
            if !body.ty.is_literal() && result_type.is_none() {
                result_type = Some(body.ty.clone());
            }
            checked.push((pattern, body, surface, inner));
        }
        let result_type = result_type.unwrap_or_else(|| self.default_number());
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
        expected: Option<&Type>,
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
        self.expr(&body, env, path, expected, false)
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
