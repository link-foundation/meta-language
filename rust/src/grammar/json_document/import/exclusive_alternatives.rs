type CollectionVariantGroups = Vec<(Vec<usize>, Vec<Expression>)>;

impl<'schema> Importer<'schema> {
    fn alternative_string_expression(
        &mut self,
        schema: &Value,
        branches: &'schema [Value],
        allowed_types: u8,
        keyword: &str,
    ) -> Option<Option<Expression>> {
        let string = type_membership("string");
        let Some(guard) = self.regular_string_constraints(schema, Some(keyword))? else {
            return Some(None);
        };
        let mut choices = Vec::new();
        for branch in branches {
            if possible_type_membership(branch, allowed_types) == 0 {
                continue;
            }
            let layers = reference_layers(self.root, branch)?;
            let membership = layers.iter().fold(allowed_types, |types, layer| {
                possible_type_membership(layer, types)
            });
            if membership == 0 {
                continue;
            }
            if membership != string {
                return None;
            }
            if let Some(constraints) = self.regular_string_chain(&layers, None)? {
                choices.push(constraints);
            }
        }
        let any_string = reference_expression("string");
        let choices: Vec<_> = choices
            .iter()
            .map(|constraints| {
                if guard.is_empty() && constraints.is_empty() {
                    vec![&any_string]
                } else {
                    guard.iter().chain(constraints).collect()
                }
            })
            .collect();
        if keyword == "oneOf" {
            return self.string_branch_product(&choices);
        }
        // Distribute the common assertions to bound each intersection independently.
        let mut entries = Vec::new();
        for choice in choices {
            if let Some(expression) = self.string_branch_product(&[choice])? {
                entries.push(expression);
            }
        }
        Some((!entries.is_empty()).then(|| EXPRESSION.choice_unordered(entries)))
    }

    fn lower_exclusive_array_alternatives(
        &mut self,
        schema: &'schema Value,
        branches: &'schema [Value],
        allowed_types: u8,
        pointer: &str,
    ) -> Option<Option<Expression>> {
        let array = type_membership("array");
        let guard = array_length_profile(&[schema], Some("oneOf"))?;
        let mut profiles = Vec::new();
        for branch in branches {
            if possible_type_membership(branch, allowed_types) == 0 {
                continue;
            }
            let layers = reference_layers(self.root, branch)?;
            let membership = layers.iter().fold(allowed_types, |types, layer| {
                possible_type_membership(layer, types)
            });
            if membership == 0 {
                continue;
            }
            if membership != array {
                return None;
            }
            let profile = array_length_profile(&layers, None)?;
            let minimum = guard.minimum.max(profile.minimum);
            let maximum = guard.maximum.into_iter().chain(profile.maximum).min();
            if maximum.is_some_and(|maximum| minimum > maximum) {
                continue;
            }
            profiles.push(ArrayLengthProfile {
                minimum,
                maximum,
                layout: profile.layout,
            });
        }
        self.lower_exclusive_array_lengths(guard.layout, &profiles, pointer)
            .or_else(|| self.lower_exclusive_array_items(guard.layout, &profiles, pointer))
    }

    fn lower_exclusive_array_lengths(
        &mut self,
        guard: Option<ArrayItemLayout<'schema>>,
        profiles: &[ArrayLengthProfile<'schema>],
        pointer: &str,
    ) -> Option<Option<Expression>> {
        let mut groups = Vec::<(ArrayItemLayout<'schema>, BTreeMap<usize, i64>)>::new();
        let mut empty_coverage = 0;
        for &ArrayLengthProfile {
            minimum,
            maximum,
            layout: items,
        } in profiles
        {
            if minimum == 0 {
                empty_coverage += 1;
            }
            if maximum == Some(0) {
                continue;
            }
            if let (Some(guard_layout), Some(branch_layout)) = (guard, items)
                && guard_layout != branch_layout
            {
                return None;
            }
            let layout = guard.or(items).unwrap_or(ArrayItemLayout {
                prefix: &[],
                items: &UNRESTRICTED_SCHEMA,
            });
            let index = groups
                .iter()
                .position(|(candidate, _)| *candidate == layout)
                .unwrap_or_else(|| {
                    let index = groups.len();
                    groups.push((layout, BTreeMap::new()));
                    index
                });
            let boundaries = &mut groups[index].1;
            // Coverage changes at the first included length and the first length after the interval.
            *boundaries.entry(minimum.max(1)).or_default() += 1;
            if let Some(maximum) = maximum {
                *boundaries.entry(maximum.checked_add(1)?).or_default() -= 1;
            }
        }
        // The first item distinguishes nonempty arrays with disjoint first-item types.
        if groups.len() > 1
            && !branches_use_disjoint_types(
                self.root,
                groups
                    .iter()
                    .map(|(layout, _)| layout.prefix.first().unwrap_or(layout.items)),
                ALL_JSON_TYPES,
            )
        {
            return None;
        }
        let mut expressions = Vec::new();
        if empty_coverage == 1 {
            expressions.push(collection("[", "]", EXPRESSION.empty()));
        }
        for (layout, boundaries) in groups {
            let mut add_interval = |minimum, maximum| {
                if let Some(expression) = self.array_with_length(
                    layout.prefix.iter().map(CollectionChild::Schema),
                    CollectionChild::Schema(layout.items),
                    minimum,
                    maximum,
                    pointer,
                ) {
                    expressions.push(expression);
                }
            };
            let mut count = 0;
            let mut minimum = 0;
            for (length, change) in boundaries {
                let next_count = count + change;
                if count == 1 && next_count != 1 {
                    add_interval(minimum, Some(length - 1));
                }
                if count != 1 && next_count == 1 {
                    minimum = length;
                }
                count = next_count;
            }
            if count == 1 {
                add_interval(minimum, None);
            }
        }
        Some((!expressions.is_empty()).then(|| EXPRESSION.choice_unordered(expressions)))
    }

    fn lower_exclusive_array_items(
        &mut self,
        guard: Option<ArrayItemLayout<'schema>>,
        profiles: &[ArrayLengthProfile<'schema>],
        pointer: &str,
    ) -> Option<Option<Expression>> {
        let last_position = profiles.iter().fold(
            guard.map_or(1, |layout| layout.prefix.len().max(1)),
            |position, profile| {
                position
                    .max(profile.minimum)
                    .max(profile.maximum.unwrap_or(0))
                    .max(profile.layout.map_or(0, |layout| layout.prefix.len()))
            },
        );
        let mut work: usize = keywords("exclusiveArrayWorkLimit").next()?.parse().ok()?;
        let location = pointer_child(pointer, "items");
        let item_schema = |layout: Option<ArrayItemLayout<'schema>>, position: usize| {
            layout.map_or(&UNRESTRICTED_SCHEMA, |items| {
                items.prefix.get(position).unwrap_or(items.items)
            })
        };
        let branches: Vec<_> = (0..profiles.len()).collect();
        let mut indices = HashMap::from([((0, branches.clone()), 0)]);
        let mut states = vec![ArrayMembershipState {
            position: 0,
            branches,
            transitions: Vec::new(),
            expression: None,
        }];
        // After the final prefix or length boundary, only branch membership can change.
        let mut index = 0;
        while index < states.len() {
            let position = states[index].position;
            let branches = states[index].branches.clone();
            work = work.checked_sub(1 + branches.len())?;
            let children: Vec<_> = branches
                .iter()
                .map(|&branch| {
                    let profile = &profiles[branch];
                    if profile.maximum.is_some_and(|maximum| position >= maximum) {
                        &PROHIBITED_SCHEMA
                    } else {
                        item_schema(profile.layout, position)
                    }
                })
                .collect();
            let variants = self.select_exclusive_collection_value_variants(
                &children,
                item_schema(guard, position),
                &location,
                &mut work,
                true,
            )?;
            for variant in variants {
                let remaining: Vec<_> = variant
                    .branches
                    .iter()
                    .map(|&item| branches[item])
                    .collect();
                let position = (position + 1).min(last_position);
                let target = *indices
                    .entry((position, remaining.clone()))
                    .or_insert_with(|| {
                        let target = states.len();
                        states.push(ArrayMembershipState {
                            position,
                            branches: remaining,
                            transitions: Vec::new(),
                            expression: None,
                        });
                        target
                    });
                states[index].transitions.push((variant, target));
            }
            index += 1;
        }
        // Work backward from possible ends. The only cycles consume a member and stay in the same state.
        let mut ordered: Vec<_> = (0..states.len()).collect();
        ordered.sort_by_key(|&index| {
            let state = &states[index];
            (std::cmp::Reverse(state.position), state.branches.len())
        });
        for index in ordered {
            let state = &states[index];
            let position = state.position;
            let matching = state
                .branches
                .iter()
                .filter(|&&branch| position >= profiles[branch].minimum)
                .count();
            let mut expressions = Vec::new();
            if matching == 1 {
                expressions.push(EXPRESSION.empty());
            }
            let mut loops = Vec::new();
            for (variant, target) in std::mem::take(&mut states[index].transitions) {
                if target != index && states[target].expression.is_none() {
                    continue;
                }
                let item = if let Some(child) = variant.child {
                    self.lower_child(child, &location)
                } else {
                    Some(
                        EXPRESSION.choice_unordered(
                            variant
                                .values
                                .iter()
                                .map(|value| self.constant(value, &location, "oneOf")),
                        ),
                    )
                };
                let Some(item) = item else { continue };
                let member = if position == 0 { item } else { separated(item) };
                if let Some(target_expression) = &states[target].expression {
                    expressions.push(EXPRESSION.seq([member, target_expression.clone()]));
                } else {
                    loops.push(member);
                }
            }
            if expressions.is_empty() {
                continue;
            }
            let name = format!("json_exclusive_array_{}", self.next());
            let expression = reference_expression(&name);
            expressions.extend(
                loops
                    .into_iter()
                    .map(|item| EXPRESSION.seq([item, expression.clone()])),
            );
            self.grammar.add_rule(GrammarRule::new(
                &name,
                EXPRESSION.choice_unordered(expressions),
            ));
            states[index].expression = Some(expression);
        }
        Some(
            states[0]
                .expression
                .take()
                .map(|body| collection("[", "]", body)),
        )
    }

    fn exclusive_object_field_expression(
        &mut self,
        schema: &'schema Value,
        branches: &'schema [Value],
        allowed_types: u8,
        pointer: &str,
    ) -> Option<Option<Expression>> {
        let object = type_membership("object");
        if !collection_keywords_are_supported(&[schema], object, "objectAssertion", Some("oneOf")) {
            return None;
        }
        let has_sibling_assertions = schema
            .as_object()?
            .keys()
            .any(|key| active_conjunction_keyword(key, 0, Some("oneOf"), object));
        let mut layouts = Vec::new();
        for (index, branch) in branches.iter().enumerate() {
            if possible_type_membership(branch, allowed_types) == 0 {
                continue;
            }
            let layers = reference_layers(self.root, branch)?;
            let membership = layers.iter().fold(allowed_types, |types, layer| {
                possible_type_membership(layer, types)
            });
            if membership == 0 {
                continue;
            }
            let (&leaf, preceding) = layers.split_last()?;
            if membership != object
                || layers.iter().enumerate().any(|(index, layer)| {
                    layer.as_object().is_some_and(|rules| {
                        rules.keys().any(|key| {
                            active_conjunction_keyword(key, index, None, object)
                                && (index != preceding.len()
                                    || !keywords("objectAssertion").any(|name| name == key))
                        })
                    })
                })
            {
                return None;
            }
            let location = preceding
                .last()
                .and_then(|layer| layer["$ref"].as_str())
                .and_then(|name| name.strip_prefix('#'))
                .map_or_else(
                    || pointer_child(&pointer_child(pointer, "oneOf"), &index.to_string()),
                    str::to_owned,
                );
            layouts.push((leaf, location));
        }
        let [first, others @ ..] = layouts.as_slice() else {
            return None;
        };
        if others.is_empty() {
            return None;
        }
        let (base, location) = (first.0, &first.1);
        let properties: Vec<_> = property_schemas(base).collect();
        let additional = additional_property_schema(base);
        if others
            .iter()
            .any(|(layout, _)| additional_property_schema(layout) != additional)
        {
            return None;
        }
        let names: BTreeSet<_> = layouts
            .iter()
            .flat_map(|&(layout, _)| property_schemas(layout).map(|(name, _)| name))
            .collect();
        let differing: Vec<_> = names
            .iter()
            .copied()
            .filter(|&name| {
                others.iter().any(|(layout, _)| {
                    object_property_schema(base, name) != object_property_schema(layout, name)
                })
            })
            .collect();
        let branch_required: Vec<BTreeSet<_>> = layouts
            .iter()
            .map(|(layout, _)| required_property_names(layout).collect())
            .collect();
        let all_required: BTreeSet<_> = branch_required.iter().flatten().copied().collect();
        let differing_required: BTreeSet<_> = all_required
            .iter()
            .copied()
            .filter(|name| {
                branch_required
                    .iter()
                    .any(|required| !required.contains(name))
            })
            .collect();
        let mut affected = differing.clone();
        affected.extend(
            differing_required
                .iter()
                .copied()
                .filter(|name| !differing.contains(name)),
        );
        if affected.is_empty() {
            return Some(None);
        }
        let all_branches: Vec<_> = (0..layouts.len()).collect();
        let required: BTreeSet<_> = all_required
            .difference(&differing_required)
            .copied()
            .chain(required_property_names(schema))
            .collect();
        let mut work = 16_384;
        let mut fields = Vec::new();
        let properties_pointer = pointer_child(pointer, "properties");
        for name in affected {
            let mut variants = if differing.contains(&name) {
                let schemas: Vec<_> = layouts
                    .iter()
                    .map(|&(layout, _)| object_property_schema(layout, name))
                    .collect();
                let sibling = object_property_schema(schema, name);
                self.select_exclusive_collection_value_variants(
                    &schemas,
                    sibling,
                    &pointer_child(&properties_pointer, name),
                    &mut work,
                    false,
                )?
            } else {
                vec![CollectionValueVariant {
                    branches: all_branches.clone(),
                    use_common_schema: true,
                    child: None,
                    values: Vec::new(),
                    expression: None,
                }]
            };
            if !required.contains(name) {
                let branches: Vec<_> = branch_required
                    .iter()
                    .enumerate()
                    .filter_map(|(index, required)| (!required.contains(name)).then_some(index))
                    .collect();
                if !branches.is_empty() {
                    variants.push(CollectionValueVariant {
                        branches,
                        use_common_schema: false,
                        child: None,
                        values: Vec::new(),
                        expression: None,
                    });
                }
            }
            fields.push(ObjectFieldVariants { name, variants });
        }
        let mut selections = Vec::new();
        if differing_required.is_empty()
            && let [field] = fields.as_mut_slice()
        {
            let mut values = Vec::new();
            let mut children = Vec::new();
            for variant in field
                .variants
                .drain(..)
                .filter(|variant| variant.branches.len() == 1)
            {
                if let Some(child) = variant.child {
                    children.push(child);
                } else {
                    values.extend(variant.values);
                }
            }
            if children.is_empty() && values.is_empty() {
                return Some(None);
            }
            let child = if children.len() > 1 {
                let location = pointer_child(&properties_pointer, field.name);
                let entries = children
                    .into_iter()
                    .filter_map(|child| self.lower_child(child, &location))
                    .map(ConjunctionEntry::Expression)
                    .collect();
                Some(CollectionChild::Conjunction(entries))
            } else {
                children.pop()
            };
            field.variants.push(CollectionValueVariant {
                branches: Vec::new(),
                use_common_schema: false,
                child,
                values,
                expression: None,
            });
            selections.push(vec![0]);
        } else {
            // An absent field admits only branches that leave it optional. Each selected
            // combination has its own ordinary object grammar and budget cost.
            let names: HashSet<_> = names
                .iter()
                .copied()
                .chain(required.iter().copied())
                .chain(all_required.iter().copied())
                .chain(property_schemas(schema).map(|(name, _)| name))
                .collect();
            if names.len() > 12 {
                return None;
            }
            let work_per_object = ((1 << names.len()) + 1) * (2 + names.len());
            select_exclusive_object_fields(
                &fields,
                &all_branches,
                work_per_object,
                &mut work,
                &mut Vec::new(),
                &mut selections,
            )?;
        }
        if selections.is_empty() {
            return Some(None);
        }

        let (properties, additional, location) = if has_sibling_assertions {
            let selected_locations: HashSet<_> = differing
                .iter()
                .map(|name| pointer_child(&properties_pointer, name))
                .collect();
            let CollectionConstraints::Object {
                properties,
                additional,
                ..
            } = collection_constraints(
                &[schema, base],
                object,
                Some("oneOf"),
                pointer,
                |children, location| {
                    if selected_locations.contains(location) {
                        return Some(CollectionChild::Schema(&UNRESTRICTED_SCHEMA));
                    }
                    self.combine_child_schemas(children, location, 0)
                },
            )?
            else {
                return None;
            };
            (properties, additional, pointer)
        } else {
            (
                properties
                    .into_iter()
                    .map(|(name, value)| (name, CollectionChild::Schema(value)))
                    .collect(),
                CollectionChild::Schema(additional),
                location.as_str(),
            )
        };
        let mut expressions = Vec::new();
        for selection in selections {
            let mut overrides = HashMap::new();
            let mut required_names: Vec<_> = required.iter().copied().collect();
            for (field, selected) in fields.iter_mut().zip(selection) {
                let variant = &mut field.variants[selected];
                if variant.use_common_schema {
                    required_names.push(field.name);
                    continue;
                }
                if let Some(child) = &variant.child {
                    overrides.insert(field.name, child.clone());
                    required_names.push(field.name);
                    continue;
                }
                if variant.values.is_empty() {
                    overrides.insert(field.name, CollectionChild::Schema(&PROHIBITED_SCHEMA));
                    continue;
                }
                if variant.expression.is_none() {
                    let rule = format!("json_exclusive_property_{}", self.next());
                    let expression = EXPRESSION.choice_unordered(
                        variant
                            .values
                            .iter()
                            .map(|value| self.constant(value.as_ref(), pointer, "oneOf"))
                            .collect::<Vec<_>>(),
                    );
                    self.grammar.add_rule(GrammarRule::new(&rule, expression));
                    variant.expression = Some(reference_expression(&rule));
                }
                let expression = variant.expression.as_ref()?.clone();
                overrides.insert(
                    field.name,
                    CollectionChild::Conjunction(vec![ConjunctionEntry::Expression(expression)]),
                );
                required_names.push(field.name);
            }
            let mut selected_fields: Vec<_> = properties
                .iter()
                .map(|(name, child)| {
                    (
                        *name,
                        overrides.remove(name).unwrap_or_else(|| child.clone()),
                    )
                })
                .collect();
            selected_fields.extend(fields.iter().filter_map(|field| {
                overrides
                    .remove(field.name)
                    .map(|child| (field.name, child))
            }));
            if let Some(expression) = self.object_with_properties(
                selected_fields,
                additional.clone(),
                required_names.into_iter(),
                location,
            ) {
                expressions.push(expression);
            }
        }
        Some((!expressions.is_empty()).then(|| EXPRESSION.choice_unordered(expressions)))
    }

    fn exclusive_collection_type_variants(
        &self,
        schemas: &[&'schema Value],
        sibling: &'schema Value,
        remaining_work: &mut usize,
    ) -> Option<Vec<CollectionValueVariant<'schema>>> {
        let mut work = JSON_TYPES
            .len()
            .checked_mul(schemas.len().checked_add(1)?)?;
        let mut type_domain = |schema: &'schema Value| {
            let layers = reference_layers(self.root, schema)?;
            work = work.checked_add(layers.len())?;
            if work > *remaining_work
                || layers.iter().any(|layer| {
                    layer.as_object().is_some_and(|fields| {
                        fields.keys().any(|keyword| {
                            active_conjunction_keyword(keyword, 0, None, ALL_JSON_TYPES)
                        })
                    })
                })
            {
                return None;
            }
            Some(layers.iter().fold(ALL_JSON_TYPES, |types, layer| {
                possible_type_membership(layer, types)
            }))
        };
        let guard = type_domain(sibling)?;
        let mut domains = Vec::new();
        // The unrestricted number production includes both numeric types and their exponent spellings.
        for &schema in schemas {
            let types = type_domain(schema)?;
            if types & guard & NUMBER_TYPE == INTEGER_TYPE {
                return None;
            }
            domains.push(types);
        }
        *remaining_work -= work;
        Some(Self::collection_variant_schemas(
            Self::collection_type_variant_groups(&domains, guard),
        ))
    }

    fn collection_type_variant_groups(domains: &[u8], guard: u8) -> CollectionVariantGroups {
        let mut groups = CollectionVariantGroups::new();
        for (name, mask) in JSON_TYPES {
            if name == "integer" || guard & mask == 0 {
                continue;
            }
            let branches: Vec<_> = domains
                .iter()
                .enumerate()
                .filter_map(|(index, &types)| (types & mask == mask).then_some(index))
                .collect();
            if branches.is_empty() {
                continue;
            }
            Self::add_collection_variant(&mut groups, branches, reference_expression(name));
        }
        groups
    }

    fn add_collection_variant(
        groups: &mut CollectionVariantGroups,
        branches: Vec<usize>,
        expression: Expression,
    ) {
        if let Some((_, entries)) = groups.iter_mut().find(|(current, _)| *current == branches) {
            entries.push(expression);
        } else {
            groups.push((branches, vec![expression]));
        }
    }

    fn collection_variant_schemas(
        groups: CollectionVariantGroups,
    ) -> Vec<CollectionValueVariant<'schema>> {
        groups
            .into_iter()
            .map(|(branches, expressions)| CollectionValueVariant {
                branches,
                use_common_schema: false,
                child: Some(CollectionChild::Conjunction(
                    expressions
                        .into_iter()
                        .map(ConjunctionEntry::Expression)
                        .collect(),
                )),
                values: Vec::new(),
                expression: None,
            })
            .collect()
    }

    fn exclusive_collection_pattern_variants(
        &mut self,
        schemas: &[&'schema Value],
        sibling: &'schema Value,
        remaining_work: &mut usize,
    ) -> Option<Vec<CollectionValueVariant<'schema>>> {
        let string = type_membership("string");
        let guard_layers = reference_layers(self.root, sibling)?;
        let guard = guard_layers.iter().fold(ALL_JSON_TYPES, |types, layer| {
            possible_type_membership(layer, types)
        });
        if guard & string == 0 {
            return None;
        }
        let guard_constraints = self.regular_string_chain(&guard_layers, None)?;
        let mut domains = Vec::new();
        let mut branches = Vec::new();
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
            let other_types = domain & !string;
            if (other_types & NUMBER_TYPE) == INTEGER_TYPE
                || (other_types != 0
                    && guard_layers.iter().chain(&layers).any(|layer| {
                        layer.as_object().is_some_and(|fields| {
                            fields
                                .keys()
                                .any(|key| active_conjunction_keyword(key, 0, None, other_types))
                        })
                    }))
            {
                return None;
            }
            domains.push(other_types);
            if (domain & string) != 0
                && guard_constraints.is_some()
                && let Some(constraints) = self.regular_string_chain(&layers, None)?
            {
                branches.push((index, constraints));
            }
        }
        *remaining_work -= work;
        let mut groups = Self::collection_type_variant_groups(&domains, guard & !string);
        if branches.is_empty() {
            return Some(Self::collection_variant_schemas(groups));
        }
        let guard_constraints = guard_constraints?;
        let any_string = reference_expression("string");
        let conjunctions: Vec<_> = branches
            .iter()
            .map(|(_, constraints)| {
                if guard_constraints.is_empty() && constraints.is_empty() {
                    vec![&any_string]
                } else {
                    guard_constraints.iter().chain(constraints).collect()
                }
            })
            .collect();
        let rule_prefix = format!("json_string_membership_{}", self.next());
        let product = partition_json_string_expressions(
            &self.grammar,
            &conjunctions,
            &rule_prefix,
            &mut self.collection_string_work,
        )?;
        let product_work: usize = product.iter().map(|variant| 1 + variant.rules.len()).sum();
        *remaining_work = remaining_work.checked_sub(product_work)?;
        for variant in product {
            for rule in variant.rules {
                self.grammar.add_rule(rule);
            }
            let membership = variant
                .branches
                .into_iter()
                .map(|index| branches[index].0)
                .collect();
            Self::add_collection_variant(&mut groups, membership, variant.expression);
        }
        Some(Self::collection_variant_schemas(groups))
    }

    fn select_exclusive_collection_value_variants(
        &mut self,
        schemas: &[&'schema Value],
        sibling: &'schema Value,
        pointer: &str,
        work: &mut usize,
        allow_shared_schema: bool,
    ) -> Option<Vec<CollectionValueVariant<'schema>>> {
        if sibling == &PROHIBITED_SCHEMA {
            return Some(Vec::new());
        }
        let allowed: Vec<_> = schemas
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, rule)| rule != &PROHIBITED_SCHEMA)
            .collect();
        let Some((&(_, first), rest)) = allowed.split_first() else {
            return Some(Vec::new());
        };
        if let Some(numeric) =
            self.exclusive_collection_numeric_variants(schemas, sibling, pointer, work)
        {
            return Some(numeric);
        }
        // A shared schema gives every permitted present value the same branch membership.
        if (allow_shared_schema || allowed.len() < schemas.len())
            && rest.iter().all(|&(_, rule)| rule == first)
        {
            let child = self.combine_child_schemas(vec![first, sibling], pointer, 0)?;
            if matches!(child, CollectionChild::Schema(Value::Bool(false))) {
                return Some(Vec::new());
            }
            return Some(vec![CollectionValueVariant {
                branches: allowed.iter().map(|&(index, _)| index).collect(),
                use_common_schema: false,
                child: Some(child),
                values: Vec::new(),
                expression: None,
            }]);
        }
        if let Some(typed) = self.exclusive_collection_type_variants(schemas, sibling, work) {
            return Some(typed);
        }
        self.finite_collection_value_variants(schemas, &allowed, sibling, work, allow_shared_schema)
            .or_else(|| self.exclusive_collection_pattern_variants(schemas, sibling, work))
    }

    fn finite_collection_value_variants(
        &self,
        schemas: &[&'schema Value],
        allowed: &[(usize, &'schema Value)],
        sibling: &'schema Value,
        work: &mut usize,
        allow_shared_schema: bool,
    ) -> Option<Vec<CollectionValueVariant<'schema>>> {
        let checks = schemas.len() + usize::from(sibling != &UNRESTRICTED_SCHEMA);
        let mut seen = HashSet::new();
        let mut candidates = Vec::new();
        let mut shared = if allow_shared_schema && sibling != &UNRESTRICTED_SCHEMA {
            finite_schema_candidates(
                self.root,
                sibling,
                &mut HashSet::new(),
                work,
                0,
                ALL_JSON_TYPES,
            )
        } else {
            None
        };
        let sources = if shared.is_some() {
            vec![sibling]
        } else {
            allowed.iter().map(|&(_, child)| child).collect()
        };
        for child in sources {
            let (values, _) = shared.take().or_else(|| {
                finite_schema_candidates(
                    self.root,
                    child,
                    &mut HashSet::new(),
                    work,
                    0,
                    ALL_JSON_TYPES,
                )
            })?;
            for value in values {
                charge_collection_value(value.as_ref(), checks, work, 0)?;
                if seen.insert(value.to_string()) {
                    candidates.push(value);
                }
            }
        }
        let mut signatures = HashMap::new();
        let mut variants = Vec::new();
        for value in candidates {
            if sibling != &UNRESTRICTED_SCHEMA
                && !super::validation::accepts_constant(self.root, [sibling], value.as_ref())
            {
                continue;
            }
            let branches: Vec<_> = schemas
                .iter()
                .enumerate()
                .filter_map(|(index, &schema)| {
                    super::validation::accepts_constant(self.root, [schema], value.as_ref())
                        .then_some(index)
                })
                .collect();
            if branches.is_empty() {
                continue;
            }
            let index = *signatures.entry(branches.clone()).or_insert_with(|| {
                let index = variants.len();
                variants.push(CollectionValueVariant {
                    branches,
                    use_common_schema: false,
                    child: None,
                    values: Vec::new(),
                    expression: None,
                });
                index
            });
            variants[index].values.push(value);
        }
        Some(variants)
    }
}
