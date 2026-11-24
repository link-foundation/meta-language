impl<'schema> Importer<'schema> {
    fn object(&mut self, schema: &'schema Value, pointer: &str) -> Option<Expression> {
        self.object_with_properties(
            property_schemas(schema)
                .map(|(name, schema)| (name, CollectionChild::Schema(schema)))
                .collect(),
            CollectionChild::Schema(additional_property_schema(schema)),
            required_property_names(schema),
            pointer,
        )
    }

    fn object_with_properties(
        &mut self,
        fields: Vec<(&'schema str, CollectionChild<'schema>)>,
        additional: CollectionChild<'schema>,
        required: impl Iterator<Item = &'schema str>,
        pointer: &str,
    ) -> Option<Expression> {
        let mut keys: Vec<_> = fields.iter().map(|(name, _)| *name).collect();
        let mut fields: HashMap<_, _> = fields.into_iter().collect();
        let mut required_names = HashSet::new();
        for key in required {
            if required_names.insert(key) && !fields.contains_key(key) {
                keys.push(key);
            }
        }
        if keys.is_empty() {
            let body = match additional {
                CollectionChild::Schema(Value::Bool(false)) => EXPRESSION.empty(),
                CollectionChild::Schema(Value::Bool(true)) => {
                    return Some(reference_expression("object"));
                }
                child => self
                    .lower_child(child, &pointer_child(pointer, "additionalProperties"))
                    .map_or_else(
                        || EXPRESSION.empty(),
                        |item| {
                            bounded_tail(
                                member(reference_expression("string"), item),
                                0,
                                None,
                                false,
                            )
                        },
                    ),
            };
            return Some(collection("{", "}", body));
        }
        let additional =
            self.lower_child(additional, &pointer_child(pointer, "additionalProperties"));
        let additional_member = additional.clone()
            .and_then(|child| {
                let lengths: Vec<_> = keys.iter().map(|key| key.encode_utf16().count()).collect();
                if lengths.iter().sum::<usize>() > 1024
                    || lengths.iter().any(|length| *length > 128)
                {
                    self.report(pointer, "additionalProperties", "The decoder bounds declared key spellings for additional properties. Validation checks all permitted names.");
                    return None;
                }
                let name = format!("additional_property_{}", self.next());
                let expression = member(additional_property_name(&keys), child);
                self.grammar.add_rule(GrammarRule::new(&name, expression));
                Some(reference_expression(&name))
            });
        let mut members = Vec::new();
        for key in keys {
            let item = match fields.remove(key) {
                Some(child) => self.lower_child(
                    child,
                    &pointer_child(&pointer_child(pointer, "properties"), key),
                ),
                None => additional.clone(),
            };
            let is_required = required_names.contains(key);
            if let Some(item) = item {
                members.push(Member {
                    required: is_required,
                    expression: property(key, item),
                });
            } else if is_required {
                return None;
            }
        }
        Some(self.object_members(&members, additional_member, pointer, "properties"))
    }

    fn object_members(
        &mut self,
        fields: &[Member],
        additional: Option<Expression>,
        pointer: &str,
        keyword: &str,
    ) -> Expression {
        let members = ObjectMembers {
            fields,
            additional,
            group: self.next(),
        };
        let mut states = HashMap::new();
        let body = if members.fields.len() > 12 {
            self.report(pointer, keyword, "The decoder emits large objects in schema field order. Validation accepts all field orders.");
            self.ordered_members(&members, 0, false, &mut states)
        } else {
            let required = members
                .fields
                .iter()
                .enumerate()
                .filter(|(_, member)| member.required)
                .fold(0, |bits, (index, _)| bits | ((1_usize) << index));
            self.unordered_members(&members, 0, false, required, &mut states)
        };
        collection("{", "}", body)
    }

    fn ordered_members(
        &mut self,
        members: &ObjectMembers<'_>,
        index: usize,
        started: bool,
        states: &mut HashMap<usize, String>,
    ) -> Expression {
        let state = index * 2 + usize::from(started);
        if let Some(name) = states.get(&state) {
            return reference_expression(name);
        }
        let name = format!("object_{}_{index}_{}", members.group, usize::from(started));
        states.insert(state, name.clone());
        let mut alternatives = Vec::new();
        if let Some(additional) = &members.additional {
            alternatives.push(EXPRESSION.seq([
                append_member(additional, started),
                self.ordered_members(members, index, true, states),
            ]));
        }
        if let Some(member) = members.fields.get(index) {
            alternatives.push(EXPRESSION.seq([
                append_member(&member.expression, started),
                self.ordered_members(members, index + 1, true, states),
            ]));
            if !member.required {
                alternatives.push(self.ordered_members(members, index + 1, started, states));
            }
        } else {
            alternatives.push(EXPRESSION.empty());
        }
        self.grammar.add_rule(GrammarRule::new(
            &name,
            EXPRESSION.choice_unordered(alternatives),
        ));
        reference_expression(&name)
    }

    fn unordered_members(
        &mut self,
        members: &ObjectMembers<'_>,
        seen: usize,
        started: bool,
        required: usize,
        states: &mut HashMap<usize, String>,
    ) -> Expression {
        let state = seen * 2 + usize::from(started);
        if let Some(name) = states.get(&state) {
            return reference_expression(name);
        }
        let name = format!("object_{}_{seen}_{}", members.group, usize::from(started));
        states.insert(state, name.clone());
        let mut alternatives = Vec::new();
        if seen & required == required {
            alternatives.push(EXPRESSION.empty());
        }
        if let Some(additional) = &members.additional {
            alternatives.push(EXPRESSION.seq([
                append_member(additional, started),
                self.unordered_members(members, seen, true, required, states),
            ]));
        }
        for (index, member) in members.fields.iter().enumerate() {
            let bit = 1_usize << index;
            if seen & bit == 0 {
                alternatives.push(EXPRESSION.seq([
                    append_member(&member.expression, started),
                    self.unordered_members(members, seen | bit, true, required, states),
                ]));
            }
        }
        self.grammar.add_rule(GrammarRule::new(
            &name,
            EXPRESSION.choice_unordered(alternatives),
        ));
        reference_expression(&name)
    }

    fn array(&mut self, schema: &'schema Value, pointer: &str) -> Option<Expression> {
        let min = schema["minItems"]
            .as_u64()
            .map_or(Some(0), |value| usize::try_from(value).ok())?;
        let max = schema["maxItems"]
            .as_u64()
            .and_then(|value| usize::try_from(value).ok());
        self.array_with_length(
            prefix_item_schemas(schema)
                .iter()
                .map(CollectionChild::Schema),
            CollectionChild::Schema(tail_item_schema(schema)),
            min,
            max,
            pointer,
        )
    }

    fn array_with_length(
        &mut self,
        prefix: impl IntoIterator<Item = CollectionChild<'schema>>,
        items: CollectionChild<'schema>,
        min: usize,
        max: Option<usize>,
        pointer: &str,
    ) -> Option<Expression> {
        if max.is_some_and(|max| max < min) {
            return None;
        }
        if max == Some(0) {
            return Some(collection("[", "]", EXPRESSION.empty()));
        }
        let location = pointer_child(pointer, "prefixItems");
        let prefix: Vec<_> = prefix
            .into_iter()
            .enumerate()
            .map(|(index, item)| {
                self.lower_child(item, &pointer_child(&location, &index.to_string()))
            })
            .collect();
        let tail = self.lower_child(items, &pointer_child(pointer, "items"));
        let mut alternatives = Vec::new();
        let mut members = Vec::new();
        for length in 0..=max.unwrap_or(prefix.len()).min(prefix.len()) {
            if length == prefix.len()
                && let Some(item) = &tail
            {
                let rest = bounded_tail(
                    item.clone(),
                    min.saturating_sub(length),
                    max.map(|limit| limit - length),
                    length != 0,
                );
                alternatives
                    .push(EXPRESSION.seq(members.iter().cloned().chain(std::iter::once(rest))));
                break;
            }
            if length >= min {
                alternatives.push(EXPRESSION.seq(members.clone()));
            }
            let Some(Some(item)) = prefix.get(length) else {
                break;
            };
            members.push(if length == 0 {
                item.clone()
            } else {
                separated(item.clone())
            });
        }
        (!alternatives.is_empty())
            .then(|| collection("[", "]", EXPRESSION.choice_unordered(alternatives)))
    }
}
