//! Pattern-match compilation and exhaustiveness.

use super::expectation::ExpectedType;
use crate::translation::structured_type_rules::merge_optional_absence;

use super::{
    BinaryOp, Case, Checker, Column, Ctor, Decimal, Entry, Env, Expr, Fact, Field, HashSet,
    Language, NAT_PATTERN_UNFOLD, NPat, Node, Pattern, Result, Row, SCase, SCasePattern, SExpr,
    SNode, SPatternNode, SRow, Span, Type, decrement, surface_let, type_error, unsupported,
};

fn pattern_owners(pattern: &super::SPattern) -> Vec<String> {
    match &pattern.node {
        SPatternNode::Alias { pattern, .. } => pattern_owners(pattern),
        SPatternNode::IntegerRange { lower, upper, .. } => [lower, upper]
            .into_iter()
            .flatten()
            .flat_map(|pattern| pattern_owners(pattern))
            .collect(),
        SPatternNode::ByteLit { .. } => vec!["u8".to_owned()],
        SPatternNode::TypedNumLit { owner, .. } => vec![owner.clone()],
        _ => vec![String::new()],
    }
}

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
        mut checked_tag_receiver: Option<Expr>,
    ) -> Result<SExpr> {
        let mut lets = Vec::new();
        let mut columns = Vec::new();
        for (column, source) in scrutinees.iter().enumerate() {
            let owners = rows
                .iter()
                .filter_map(|row| row.patterns.get(column))
                .flat_map(pattern_owners)
                .collect::<Vec<_>>();
            let (kind, typed) = match &source.node {
                SNode::Num { ty, .. } => ("num", ty.is_some()),
                _ => ("", false),
            };
            let inference_error=crate::translation::rust_numeric_pattern_rules::read_rust_source_numeric_pattern_inference_error(self.language.as_str(),kind,typed,&owners);
            if !inference_error.is_empty() {
                return Err(type_error(&inference_error, source.span.or(span)));
            }
            let owner=crate::translation::rust_numeric_pattern_rules::read_rust_source_numeric_pattern_inference(self.language.as_str(),kind,typed,&owners);
            let inferred;
            let scrutinee = if !owner.is_empty() {
                let mut value = source.clone();
                if let SNode::Num { ty, .. } = &mut value.node {
                    *ty = if owner == "f64" {
                        Some(crate::translation::types::FLOAT)
                    } else {
                        crate::translation::types::rust_fixed_type(&owner)
                    };
                }
                inferred = value;
                &inferred
            } else {
                source
            };
            let checked = if column == 0
                && let Some(receiver) = checked_tag_receiver.take()
            {
                receiver
            } else {
                self.expr(scrutinee, env, path, None, false)?
            };
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
            normalised.push(super::pattern_aliases::project_pattern_alias_row(
                Row { patterns, ..row },
                columns,
                span,
            )?);
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
        if matches!(column.ty, Type::Fixed { .. })
            && normalised
                .iter()
                .any(|row| matches!(row.patterns[refutable], NPat::Interval { .. }))
        {
            return self.compile_integer_range_column(columns, &normalised, refutable, span);
        }
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
        if matches!(pivot, NPat::Tuple { .. }) {
            return self.compile_tuple_pattern_column(columns, &normalised, refutable, span);
        }
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
                            right: pivot_value.clone(),
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
                        optional: false,
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
            SPatternNode::IntegerRange {
                lower,
                upper,
                inclusive,
            } => self.normalise_integer_range(
                lower.map(|value| *value),
                upper.map(|value| *value),
                inclusive,
                ty,
                at,
            ),
            SPatternNode::Alias {
                name,
                pattern: inner,
            } => Ok(NPat::Alias {
                name,
                pattern: Box::new(self.normalise_pattern(NPat::Surface(*inner), ty, at)?),
            }),
            SPatternNode::Wild => Ok(NPat::Wild),
            SPatternNode::Tuple { args, rest } => {
                let width = match ty {
                    Type::Tuple { elements } => Some(elements.len()),
                    Type::Unit => Some(0),
                    _ => None,
                };
                let count = |value| {
                    f64::from(
                        u32::try_from(value)
                            .expect("source tuple width fits the shared array limit"),
                    )
                };
                let plan =
                    crate::translation::rust_source_render_interfaces::plan_tuple_pattern_fields(
                        width.map_or(-1.0, count),
                        count(args.len()),
                        rest.map_or(-1.0, count),
                    );
                if plan.first() != Some(&0.0) {
                    return Err(type_error(
                        "tuple pattern has the wrong type or number of fields",
                        at,
                    ));
                }
                let Type::Tuple { elements } = ty else {
                    return Ok(NPat::Wild);
                };
                let mut normalised = Vec::new();
                for (position, index) in plan.into_iter().skip(1).enumerate() {
                    normalised.push(if index < 0.0 {
                        NPat::Wild
                    } else {
                        let source_index = index.to_string().parse::<usize>().map_err(|_| {
                            type_error("tuple pattern plan has an invalid source index", at)
                        })?;
                        self.normalise_pattern(
                            NPat::Surface(args[source_index].clone()),
                            &elements[position],
                            span,
                        )?
                    });
                }
                Ok(NPat::Tuple { args: normalised })
            }
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
                let context_error=crate::translation::rust_numeric_pattern_rules::read_rust_source_integer_pattern_context_error(self.language.as_str(),if *ty==Type::Float{"float"}else{""});
                if !context_error.is_empty() {
                    return Err(type_error(&context_error, span));
                }

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
            SPatternNode::TypedNumLit {
                value,
                ty: literal_type,
                owner,
                negative,
            } => {
                let (kind, bits, signed) = match &literal_type {
                    Type::Fixed { bits, signed } => ("fixed", bits.to_string(), *signed),
                    Type::Float => ("float", String::new(), false),
                    _ => ("", String::new(), false),
                };
                let error=crate::translation::rust_numeric_pattern_rules::read_rust_source_numeric_pattern_type_error(&owner,kind,&bits,signed);
                let error = if error.is_empty() {
                    crate::translation::rust_numeric_pattern_rules::read_rust_source_numeric_pattern_negation_error(&owner,negative)
                } else {
                    error
                };
                if !error.is_empty() {
                    return Err(type_error(&error, span));
                }
                self.literal_pattern(
                    SExpr::new(
                        SNode::Num {
                            value,
                            ty: Some(literal_type),
                            negative,
                            unit: false,
                        },
                        None,
                    ),
                    ty,
                    span,
                )
            }
            SPatternNode::ByteLit { value } => {
                let (kind, bits, signed) = match ty {
                    Type::Fixed { bits, signed } => ("fixed", f64::from(*bits), *signed),
                    _ => ("", 0.0, false),
                };
                let error=crate::translation::rust_byte_pattern_rules::read_rust_source_byte_pattern_type_error(kind,bits,signed);
                if !error.is_empty() {
                    return Err(type_error(&error, span));
                }
                self.literal_pattern(
                    SExpr::new(
                        SNode::Num {
                            value,
                            ty: Some(Type::Fixed {
                                bits: 8,
                                signed: false,
                            }),
                            negative: false,
                            unit: false,
                        },
                        None,
                    ),
                    ty,
                    span,
                )
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
}

include!("match_patterns.rs");
