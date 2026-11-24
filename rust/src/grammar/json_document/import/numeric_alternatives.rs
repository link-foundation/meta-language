impl<'schema> Importer<'schema> {
    fn exclusive_collection_numeric_variants(
        &mut self,
        schemas: &[&'schema Value],
        sibling: &'schema Value,
        pointer: &str,
        remaining_work: &mut usize,
    ) -> Option<Vec<CollectionValueVariant<'schema>>> {
        let guard_layers = reference_layers(self.root, sibling)?;
        let guard = guard_layers.iter().fold(ALL_JSON_TYPES, |types, layer| {
            possible_type_membership(layer, types)
        });
        if guard & NUMBER_TYPE == 0 {
            return None;
        }
        let string = type_membership("string");
        let mut domains = Vec::new();
        let mut branches = Vec::new();
        let mut constrained_strings = false;
        let mut work = guard_layers.len().checked_add(
            JSON_TYPES
                .len()
                .checked_mul(schemas.len().checked_add(1)?)?,
        )?;
        for (index, &schema) in schemas.iter().enumerate() {
            let layers = reference_layers(self.root, schema)?;
            work = work.checked_add(layers.len())?;
            if work > *remaining_work {
                return None;
            }
            let domain = layers
                .iter()
                .fold(guard, |types, layer| possible_type_membership(layer, types));
            let other_types = domain & !(NUMBER_TYPE | string);
            let assertions: Vec<_> = guard_layers.iter().copied().chain(layers).collect();
            if other_types != 0
                && assertions.iter().any(|layer| {
                    layer.as_object().is_some_and(|fields| {
                        fields
                            .keys()
                            .any(|key| active_conjunction_keyword(key, 0, None, other_types))
                    })
                })
            {
                return None;
            }
            constrained_strings |= (domain & string) != 0
                && assertions.iter().any(|layer| {
                    layer.as_object().is_some_and(|fields| {
                        fields
                            .keys()
                            .any(|key| active_conjunction_keyword(key, 0, None, string))
                    })
                });
            domains.push(domain & !NUMBER_TYPE);
            let numeric_types = domain & NUMBER_TYPE;
            if numeric_types != 0 {
                let (name, limits) = combine_numeric_constraints(&assertions, numeric_types, None)?;
                branches.push((index, name, limits));
            }
        }
        if !branches.iter().any(|(_, name, limits)| {
            *name == "integer" || limits.as_object().is_some_and(|bounds| !bounds.is_empty())
        }) {
            return None;
        }
        *remaining_work -= work;
        let numeric = partition_numeric_ranges(&branches, remaining_work)?;
        let mut groups = if constrained_strings {
            self.exclusive_collection_pattern_variant_groups(
                schemas,
                sibling,
                remaining_work,
                ALL_JSON_TYPES & !NUMBER_TYPE,
            )?
        } else {
            Self::collection_type_variant_groups(&domains, guard & !NUMBER_TYPE)
        };
        if numeric
            .iter()
            .any(|(_, expression)| !matches!(expression, Expression::NonTerminal(_)))
            && self
                .collection_numeric_diagnostics
                .insert(pointer.to_owned())
        {
            self.report(pointer, "type", "The exclusive numeric member decoder uses decimal spelling. Validation also accepts equivalent exponent spellings.");
        }
        for (membership, expression) in numeric {
            Self::add_collection_variant(&mut groups, membership, expression);
        }
        Some(Self::collection_variant_schemas(groups))
    }

    fn alternative_numeric_expression(
        &mut self,
        schema: &'schema Value,
        branches: &'schema [Value],
        allowed_types: u8,
        keyword: &str,
        pointer: &str,
    ) -> Option<Option<Expression>> {
        let mut alternatives = Vec::new();
        let mut finite_work = 16_384;
        for (index, branch) in branches.iter().enumerate() {
            if possible_type_membership(branch, allowed_types) == 0 {
                continue;
            }
            let mut layers = reference_layers(self.root, branch)?;
            let membership = layers.iter().fold(allowed_types, |types, layer| {
                possible_type_membership(layer, types)
            });
            if membership == 0 {
                continue;
            }
            if keyword == "oneOf"
                && membership & NUMBER_TYPE == membership
                && let Some((candidates, _)) = finite_schema_candidates(
                    self.root,
                    branch,
                    &mut HashSet::new(),
                    &mut finite_work,
                    0,
                    ALL_JSON_TYPES,
                )
            {
                finite_work = finite_work.checked_sub(candidates.len())?;
                let (_, common) =
                    combine_numeric_constraints(&[schema], NUMBER_TYPE, Some(keyword))?;
                // Distinct values within one branch contribute disjoint point intervals.
                let mut seen = HashSet::new();
                for value in candidates {
                    let Some(number) = value.as_f64() else {
                        continue;
                    };
                    let identity = if number == 0.0 { 0 } else { number.to_bits() };
                    if membership & value_type_membership(&value) != 0
                        && seen.insert(identity)
                        && super::validation::accepts_constant(self.root, [branch, &common], &value)
                    {
                        let point = serde_json::json!({ "minimum": value, "maximum": value });
                        alternatives.push((index, "number", point));
                    }
                }
                continue;
            }
            layers.insert(0, schema);
            let (name, limits) = combine_numeric_constraints(&layers, membership, Some(keyword))?;
            alternatives.push((index, name, limits));
        }
        if keyword == "oneOf" {
            let number_limits: Vec<_> = alternatives
                .iter()
                .filter_map(|(_, name, limits)| (*name == "number").then_some(limits))
                .collect();
            if !number_limits.is_empty() {
                let mixed = number_limits.len() != alternatives.len();
                let mut decimal = match exclusive_decimal_range(number_limits, mixed) {
                    DecimalRange::Unsupported => return None,
                    DecimalRange::Empty => None,
                    DecimalRange::Allowed(expression) => Some(expression),
                };
                if mixed {
                    // Both schema types count for an integer, including zero-padded decimals.
                    let integer = exclusive_integer_range(
                        alternatives
                            .iter()
                            .map(|(_, _, limits)| integer_bounds(limits)),
                    )?
                    .map(|expression| {
                        EXPRESSION.seq([
                            expression,
                            EXPRESSION.opt(EXPRESSION.seq([
                                EXPRESSION.term("."),
                                EXPRESSION.repeat(EXPRESSION.term("0"), 1, None),
                            ])),
                        ])
                    });
                    let entries: Vec<_> = decimal.into_iter().chain(integer).collect();
                    decimal = (!entries.is_empty()).then(|| EXPRESSION.choice_unordered(entries));
                }
                if let Some(expression) = &decimal
                    && !matches!(expression, Expression::NonTerminal(_))
                {
                    self.report(pointer, keyword, "The exclusive number decoder uses decimal spelling. Validation also accepts equivalent exponent spellings.");
                }
                return Some(decimal);
            }
            let integer = exclusive_integer_range(
                alternatives
                    .iter()
                    .map(|(_, _, limits)| integer_bounds(limits)),
            )?;
            if integer.is_some() {
                self.report(pointer, keyword, "The exclusive integer decoder uses integer spelling. Validation also accepts equivalent decimal or exponent spellings.");
            }
            return Some(integer);
        }
        let location = pointer_child(pointer, keyword);
        let entries: Vec<_> = alternatives
            .into_iter()
            .filter_map(|(index, name, limits)| {
                self.number(name, &limits, &pointer_child(&location, &index.to_string()))
            })
            .collect();
        Some((!entries.is_empty()).then(|| EXPRESSION.choice_unordered(entries)))
    }

    fn alternative_conjunction_expression(
        &mut self,
        schema: &'schema Value,
        branches: &'schema [Value],
        allowed_types: u8,
        keyword: &str,
        pointer: &str,
    ) -> Option<Option<Expression>> {
        let mut alternatives = Vec::new();
        let location = pointer_child(pointer, keyword);
        for (index, branch) in branches.iter().enumerate() {
            if possible_type_membership(branch, allowed_types) == 0 {
                continue;
            }
            let mut layers = reference_layers(self.root, branch)?;
            layers.insert(0, schema);
            let location = pointer_child(&location, &index.to_string());
            let entries =
                self.conjunction_entries(&layers, allowed_types, Some(keyword), &location, 0)?;
            alternatives.push((location, entries));
        }
        let entries: Vec<_> = alternatives
            .into_iter()
            .filter_map(|(location, entries)| self.lower_conjunction(entries, &location))
            .collect();
        Some((!entries.is_empty()).then(|| EXPRESSION.choice_unordered(entries)))
    }
}
