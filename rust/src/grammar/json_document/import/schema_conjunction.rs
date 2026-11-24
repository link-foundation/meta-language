impl<'schema> Importer<'schema> {
    fn combine_child_schemas(
        &mut self,
        schemas: Vec<&'schema Value>,
        pointer: &str,
        mut depth: usize,
    ) -> Option<CollectionChild<'schema>> {
        if let Some(schema) = compatible_child_schema(schemas.iter().copied()) {
            return Some(CollectionChild::Schema(schema));
        }
        let root = self.root;
        let mut work = 16_384;
        for schema in &schemas {
            let Some((candidates, keyword)) = finite_schema_candidates(
                root,
                schema,
                &mut HashSet::new(),
                &mut work,
                0,
                ALL_JSON_TYPES,
            ) else {
                continue;
            };
            if candidates.len() > work / schemas.len() {
                continue;
            }
            let entries: Vec<_> = candidates
                .into_iter()
                .filter(|value| {
                    super::validation::accepts_constant(root, schemas.iter().copied(), value)
                })
                .map(|value| self.constant(value.as_ref(), pointer, keyword))
                .collect();
            if entries.is_empty() {
                return Some(CollectionChild::Schema(&PROHIBITED_SCHEMA));
            }
            return Some(CollectionChild::Conjunction(vec![
                ConjunctionEntry::Expression(EXPRESSION.choice_unordered(entries)),
            ]));
        }
        let mut layers = Vec::new();
        for schema in schemas {
            if schema == &UNRESTRICTED_SCHEMA {
                continue;
            }
            let reference = reference_layers(self.root, schema)?;
            if layers.len() + reference.len() > 128 {
                return None;
            }
            layers.extend(reference);
        }
        let membership = layers.iter().fold(ALL_JSON_TYPES, |types, layer| {
            possible_type_membership(layer, types)
        });
        let collections = membership & (type_membership("object") | type_membership("array"));
        // Recursive definitions can revisit the same collection through different member paths.
        if collections != 0
            && layers.iter().any(|layer| {
                layer.as_object().is_some_and(|fields| {
                    fields.keys().any(|key| {
                        ASSERTION_TYPES
                            .get(key.as_str())
                            .is_some_and(|mask| mask & collections != 0)
                    })
                })
            })
        {
            if depth >= NESTED_COLLECTION_LIMITS[0] || self.remaining_nested_collections == 0 {
                return None;
            }
            self.remaining_nested_collections -= 1;
            depth += 1;
        }
        let entries = self.conjunction_entries(&layers, membership, None, pointer, depth)?;
        if entries.is_empty() {
            return Some(CollectionChild::Schema(&PROHIBITED_SCHEMA));
        }
        Some(CollectionChild::Conjunction(entries))
    }

    fn collection_expression(
        &mut self,
        constraints: CollectionConstraints<'schema>,
        pointer: &str,
    ) -> Option<Expression> {
        match constraints {
            CollectionConstraints::Object {
                properties,
                additional,
                required,
            } => self.object_with_properties(properties, additional, required.into_iter(), pointer),
            CollectionConstraints::Array {
                prefix,
                items,
                minimum,
                maximum,
            } => self.array_with_length(
                prefix,
                items,
                usize::try_from(minimum).ok()?,
                maximum.and_then(|max| usize::try_from(max).ok()),
                pointer,
            ),
        }
    }

    fn referenced_expression(
        &mut self,
        schema: &'schema Value,
        allowed_types: u8,
        pointer: &str,
    ) -> Option<Option<Expression>> {
        let layers = reference_layers(self.root, schema)?;
        let entries = self.conjunction_entries(&layers, allowed_types, None, pointer, 0)?;
        Some(self.lower_conjunction(entries, pointer))
    }

    fn lower_conjunction(
        &mut self,
        entries: Vec<ConjunctionEntry<'schema>>,
        pointer: &str,
    ) -> Option<Expression> {
        let expressions: Vec<_> = entries
            .into_iter()
            .filter_map(|entry| match entry {
                ConjunctionEntry::Expression(expression) => Some(expression),
                ConjunctionEntry::Number(name, limits) => self.number(name, &limits, pointer),
                ConjunctionEntry::Collection(constraints) => {
                    self.collection_expression(constraints, pointer)
                }
            })
            .collect();
        (!expressions.is_empty()).then(|| EXPRESSION.choice_unordered(expressions))
    }

    fn conjunction_entries(
        &mut self,
        layers: &[&'schema Value],
        allowed_types: u8,
        combination: Option<&str>,
        pointer: &str,
        depth: usize,
    ) -> Option<Vec<ConjunctionEntry<'schema>>> {
        let membership = layers.iter().fold(allowed_types, |types, layer| {
            possible_type_membership(layer, types)
        });
        if membership & NUMBER_TYPE == FRACTIONAL_TYPE {
            return None;
        }
        let mut entries = Vec::new();
        for (name, mask) in JSON_TYPES {
            if membership & mask != mask || (name == "integer" && membership & FRACTIONAL_TYPE != 0)
            {
                continue;
            }
            let entry = match name {
                "object" | "array" => ConjunctionEntry::Collection(collection_constraints(
                    layers,
                    mask,
                    combination,
                    pointer,
                    |schemas, location| self.combine_child_schemas(schemas, location, depth),
                )?),
                "number" | "integer" => {
                    let (name, limits) = combine_numeric_constraints(layers, mask, combination)?;
                    ConjunctionEntry::Number(name, limits)
                }
                "string" => {
                    let Some(conjunction) =
                        self.regular_string_chain(layers, combination, false)?
                    else {
                        continue;
                    };
                    let expression = if conjunction.is_empty() {
                        reference_expression(name)
                    } else {
                        let Some(expression) =
                            self.string_branch_product(&[conjunction.iter().collect()])?
                        else {
                            continue;
                        };
                        expression
                    };
                    ConjunctionEntry::Expression(expression)
                }
                _ => {
                    let residual = layers.iter().enumerate().any(|(index, layer)| {
                        layer.as_object().is_some_and(|fields| {
                            fields.keys().any(|key| {
                                active_conjunction_keyword(key, index, combination, mask)
                            })
                        })
                    });
                    if residual {
                        return None;
                    }
                    ConjunctionEntry::Expression(reference_expression(name))
                }
            };
            entries.push(entry);
        }
        Some(entries)
    }
}
