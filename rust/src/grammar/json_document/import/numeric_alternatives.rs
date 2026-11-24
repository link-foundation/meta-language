impl<'schema> Importer<'schema> {
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
