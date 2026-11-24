fn integer(value: f64) -> Option<i64> {
    value.is_finite().then(|| {
        format!("{value:.0}")
            .parse()
            .expect("safe integer spelling")
    })
}

fn safe_integer_bound(value: f64) -> bool {
    value.abs() <= 9_007_199_254_740_991.0
}

fn integer_bounds(schema: &Value) -> (f64, f64) {
    let number = |key: &str, fallback| schema[key].as_f64().unwrap_or(fallback);
    let minimum = number("minimum", f64::NEG_INFINITY)
        .ceil()
        .max(number("exclusiveMinimum", f64::NEG_INFINITY).floor() + 1.0);
    let maximum = number("maximum", f64::INFINITY)
        .floor()
        .min(number("exclusiveMaximum", f64::INFINITY).ceil() - 1.0);
    (minimum, maximum)
}

fn exclusive_integer_range(
    intervals: impl IntoIterator<Item = (f64, f64)>,
) -> Option<Option<Expression>> {
    let mut boundaries = BTreeMap::new();
    let mut coverage = 0.0;
    for (first, last) in intervals {
        if first > last {
            continue;
        }
        if first == f64::NEG_INFINITY {
            coverage += 1.0;
        } else {
            if !safe_integer_bound(first) || !safe_integer_bound(first - 1.0) {
                return None;
            }
            *boundaries.entry(integer(first)?).or_insert(0.0) += 1.0;
        }
        if last != f64::INFINITY {
            if !safe_integer_bound(last) || !safe_integer_bound(last + 1.0) {
                return None;
            }
            *boundaries.entry(integer(last + 1.0)?).or_insert(0.0) -= 1.0;
        }
    }
    let mut expressions = Vec::new();
    let mut first = None;
    for (point, change) in boundaries {
        let next = coverage + change;
        let boundary = exclusive_integer_boundary(coverage, next);
        if boundary < 0.0 {
            expressions.push(signed_integer_range(first, Some(point - 1)));
        }
        if boundary > 0.0 {
            first = Some(point);
        }
        coverage = next;
    }
    if coverage == 1.0 {
        expressions.push(signed_integer_range(first, None));
    }
    Some((!expressions.is_empty()).then(|| EXPRESSION.choice_unordered(expressions)))
}

fn reference_keyword(keyword: &str) -> bool {
    matches!(keyword, "$ref" | "$defs") || keywords("annotation").any(|name| name == keyword)
}

fn applies_to_type(keyword: &str, membership: u8) -> bool {
    ASSERTION_TYPES
        .get(keyword)
        .is_none_or(|&mask| mask & membership != 0)
}

fn active_conjunction_keyword(
    keyword: &str,
    index: usize,
    combination: Option<&str>,
    membership: u8,
) -> bool {
    keyword != "type"
        && !reference_keyword(keyword)
        && !(index == 0 && Some(keyword) == combination)
        && applies_to_type(keyword, membership)
}

fn required_property_names(schema: &Value) -> impl Iterator<Item = &str> {
    schema["required"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
}

fn property_schemas(schema: &Value) -> impl Iterator<Item = (&str, &Value)> {
    schema["properties"]
        .as_object()
        .into_iter()
        .flat_map(|fields| fields.iter().map(|(name, rule)| (name.as_str(), rule)))
}

fn additional_property_schema(schema: &Value) -> &Value {
    schema
        .get("additionalProperties")
        .unwrap_or(&UNRESTRICTED_SCHEMA)
}

fn object_property_schema<'schema>(schema: &'schema Value, name: &str) -> &'schema Value {
    schema["properties"]
        .get(name)
        .unwrap_or_else(|| additional_property_schema(schema))
}

fn prefix_item_schemas(schema: &Value) -> &[Value] {
    schema["prefixItems"].as_array().map_or(&[], Vec::as_slice)
}

fn tail_item_schema(schema: &Value) -> &Value {
    schema.get("items").unwrap_or(&UNRESTRICTED_SCHEMA)
}

fn array_length_bounds(layers: &[&Value]) -> (u64, Option<u64>) {
    let minimum = layers
        .iter()
        .filter_map(|layer| layer["minItems"].as_u64())
        .max()
        .unwrap_or(0);
    let maximum = layers
        .iter()
        .filter_map(|layer| {
            let bound = layer["maxItems"].as_u64();
            if tail_item_schema(layer) == &PROHIBITED_SCHEMA {
                let prefix = prefix_item_schemas(layer).len() as u64;
                return Some(bound.map_or(prefix, |bound| bound.min(prefix)));
            }
            bound
        })
        .min();
    (minimum, maximum)
}

fn array_length_profile<'schema>(
    layers: &[&'schema Value],
    combination: Option<&str>,
) -> Option<ArrayLengthProfile<'schema>> {
    const MAXIMUM_SAFE_INTEGER: u64 = 9_007_199_254_740_991;
    if !collection_keywords_are_supported(
        layers,
        type_membership("array"),
        "arrayAssertion",
        combination,
    ) {
        return None;
    }
    let (minimum, maximum) = array_length_bounds(layers);
    if minimum > MAXIMUM_SAFE_INTEGER
        || maximum.is_some_and(|maximum| maximum >= MAXIMUM_SAFE_INTEGER)
    {
        return None;
    }
    let minimum = usize::try_from(minimum).ok()?;
    let maximum = match maximum {
        Some(maximum) => Some(usize::try_from(maximum).ok()?),
        None => None,
    };
    let mut layout = None;
    for &layer in layers {
        let prefix = prefix_item_schemas(layer);
        // A closed tail already contributes its limit to the length interval.
        let items = match tail_item_schema(layer) {
            Value::Bool(false) => &UNRESTRICTED_SCHEMA,
            items => items,
        };
        if prefix.is_empty() && items == &UNRESTRICTED_SCHEMA {
            continue;
        }
        let candidate = ArrayItemLayout { prefix, items };
        if layout.is_some_and(|layout| layout != candidate) {
            return None;
        }
        layout = Some(candidate);
    }
    Some(ArrayLengthProfile {
        minimum,
        maximum,
        layout,
    })
}

fn compatible_child_schema<'schema>(
    schemas: impl Iterator<Item = &'schema Value>,
) -> Option<&'schema Value> {
    let mut rule = &UNRESTRICTED_SCHEMA;
    let mut agrees = true;
    let mut membership = ALL_JSON_TYPES;
    for candidate in schemas {
        match candidate {
            Value::Bool(false) => return Some(candidate),
            Value::Bool(true) => (),
            _ if rule == &UNRESTRICTED_SCHEMA => rule = candidate,
            _ => agrees &= rule == candidate,
        }
        membership &= declared_type_membership(candidate);
    }
    if membership == 0 {
        return Some(&PROHIBITED_SCHEMA);
    }
    agrees.then_some(rule)
}

fn collection_keywords_are_supported(
    layers: &[&Value],
    membership: u8,
    category: &str,
    combination: Option<&str>,
) -> bool {
    layers.iter().enumerate().all(|(index, layer)| {
        !layer.as_object().is_some_and(|fields| {
            fields.keys().any(|key| {
                active_conjunction_keyword(key, index, combination, membership)
                    && !keywords(category).any(|name| name == key.as_str())
            })
        })
    })
}

fn collection_constraints<'schema>(
    layers: &[&'schema Value],
    membership: u8,
    combination: Option<&str>,
    pointer: &str,
    mut combine: impl FnMut(Vec<&'schema Value>, &str) -> Option<CollectionChild<'schema>>,
) -> Option<CollectionConstraints<'schema>> {
    if membership == type_membership("object") {
        if !collection_keywords_are_supported(layers, membership, "objectAssertion", combination) {
            return None;
        }
        let additional = combine(
            layers
                .iter()
                .copied()
                .map(additional_property_schema)
                .collect(),
            &pointer_child(pointer, "additionalProperties"),
        )?;
        let mut seen = HashSet::new();
        let keys = layers
            .iter()
            .flat_map(|&layer| property_schemas(layer).map(|(name, _)| name))
            .filter(|&name| seen.insert(name));
        let mut properties = Vec::new();
        let location = pointer_child(pointer, "properties");
        for name in keys {
            // Defaults apply only in the layers that leave this property undeclared.
            let schemas = layers
                .iter()
                .map(|&layer| object_property_schema(layer, name))
                .collect();
            let rule = combine(schemas, &pointer_child(&location, name))?;
            properties.push((name, rule));
        }
        let required = layers
            .iter()
            .flat_map(|&layer| required_property_names(layer))
            .collect();
        return Some(CollectionConstraints::Object {
            properties,
            additional,
            required,
        });
    }
    if membership == type_membership("array") {
        if !collection_keywords_are_supported(layers, membership, "arrayAssertion", combination) {
            return None;
        }
        let (minimum, maximum) = array_length_bounds(layers);
        let layouts: Vec<_> = layers
            .iter()
            .copied()
            .filter(|layer| {
                maximum.is_none_or(|limit| limit != 0 && limit >= minimum)
                    && (!prefix_item_schemas(layer).is_empty()
                        || !tail_item_schema(layer).is_boolean())
            })
            .collect();
        if layouts.len() < 2 {
            let layout = layouts.first().copied().unwrap_or(&UNRESTRICTED_SCHEMA);
            return Some(CollectionConstraints::Array {
                prefix: prefix_item_schemas(layout)
                    .iter()
                    .map(CollectionChild::Schema)
                    .collect(),
                items: CollectionChild::Schema(tail_item_schema(layout)),
                minimum,
                maximum,
            });
        }
        let prefix_length = layouts
            .iter()
            .map(|&layer| prefix_item_schemas(layer).len())
            .max()
            .unwrap_or(0);
        let prefix_end = maximum
            .and_then(|limit| usize::try_from(limit).ok())
            .map_or(prefix_length, |limit| prefix_length.min(limit));
        let mut prefix = Vec::new();
        let location = pointer_child(pointer, "prefixItems");
        for index in 0..prefix_end {
            let schemas = layouts
                .iter()
                .map(|&layer| {
                    prefix_item_schemas(layer)
                        .get(index)
                        .unwrap_or_else(|| tail_item_schema(layer))
                })
                .collect();
            let item = combine(schemas, &pointer_child(&location, &index.to_string()))?;
            // An array cannot reach any position after a forbidden item.
            if matches!(item, CollectionChild::Schema(Value::Bool(false))) {
                return Some(CollectionConstraints::Array {
                    prefix,
                    items: CollectionChild::Schema(&UNRESTRICTED_SCHEMA),
                    minimum,
                    maximum: Some(index as u64),
                });
            }
            prefix.push(item);
        }
        let items = if maximum.is_some_and(|limit| limit <= prefix_length as u64) {
            CollectionChild::Schema(&UNRESTRICTED_SCHEMA)
        } else {
            combine(
                layouts.iter().copied().map(tail_item_schema).collect(),
                &pointer_child(pointer, "items"),
            )?
        };
        return Some(CollectionConstraints::Array {
            prefix,
            items,
            minimum,
            maximum,
        });
    }
    None
}

fn combine_numeric_constraints(
    layers: &[&Value],
    membership: u8,
    combination: Option<&str>,
) -> Option<(&'static str, Value)> {
    let name = match membership {
        INTEGER_TYPE => "integer",
        NUMBER_TYPE => "number",
        _ => return None,
    };
    let mut limits = serde_json::Map::new();
    for (index, layer) in layers.iter().enumerate() {
        let Some(fields) = layer.as_object() else {
            continue;
        };
        for (key, bound) in fields {
            let key = key.as_str();
            if !active_conjunction_keyword(key, index, combination, membership) {
                continue;
            }
            if !keywords("limit").any(|name| name == key) {
                return None;
            }
            let value = bound.as_f64().filter(|value| value.is_finite())?;
            let lower = matches!(key, "minimum" | "exclusiveMinimum");
            if limits
                .get(key)
                .and_then(Value::as_f64)
                .is_none_or(|previous| {
                    if lower {
                        value > previous
                    } else {
                        value < previous
                    }
                })
            {
                limits.insert(key.to_owned(), bound.clone());
            }
        }
    }
    Some((name, Value::Object(limits)))
}

fn finite_constants(schema: &Value) -> Option<Vec<&Value>> {
    if schema == &Value::Bool(false) {
        Some(Vec::new())
    } else if let Some(value) = schema.get("const") {
        Some(vec![value])
    } else {
        schema["enum"]
            .as_array()
            .map(|values| values.iter().collect())
    }
}

fn charge_collection_value(
    value: &Value,
    copies: usize,
    work: &mut usize,
    depth: usize,
) -> Option<()> {
    let size = 1 + value.as_str().map_or(0, str::len);
    *work = work.checked_sub(copies.checked_mul(size)?)?;
    if depth > 128 {
        return None;
    }
    if let Some(items) = value.as_array() {
        for item in items {
            charge_collection_value(item, copies, work, depth + 1)?;
        }
    }
    if let Some(fields) = value.as_object() {
        for (key, item) in fields {
            *work = work.checked_sub(copies.checked_mul(key.len() + 1)?)?;
            charge_collection_value(item, copies, work, depth + 1)?;
        }
    }
    Some(())
}

fn extend_collection_values(
    prefixes: Vec<Value>,
    members: &[Cow<'_, Value>],
    keep_prefixes: bool,
    work: &mut usize,
    mut append: impl FnMut(&Value, &Value) -> Value,
) -> Option<Vec<Value>> {
    let size = prefixes
        .len()
        .checked_mul(members.len().checked_add(usize::from(keep_prefixes))?)?;
    if size > FINITE_COLLECTION_LIMITS[2] {
        return None;
    }
    *work = work.checked_sub(size)?;
    if !members.is_empty() {
        for prefix in &prefixes {
            charge_collection_value(prefix, members.len(), work, 0)?;
        }
        for member in members {
            charge_collection_value(member, prefixes.len(), work, 0)?;
        }
    }
    let mut extended = Vec::with_capacity(size);
    for prefix in &prefixes {
        for member in members {
            extended.push(append(prefix, member.as_ref()));
        }
    }
    if keep_prefixes {
        let mut prefixes = prefixes;
        prefixes.extend(extended);
        return Some(prefixes);
    }
    Some(extended)
}

fn finite_collection_values<'schema>(
    schema: &'schema Value,
    membership: u8,
    work: &mut usize,
    mut collect_member: impl FnMut(&'schema Value, &mut usize) -> Option<Vec<Cow<'schema, Value>>>,
) -> Option<Vec<Value>> {
    let [maximum_properties, maximum_length, maximum_values] = *FINITE_COLLECTION_LIMITS;
    if membership == type_membership("object")
        && schema["additionalProperties"] == PROHIBITED_SCHEMA
    {
        let fields = schema["properties"].as_object();
        let required: HashSet<_> = schema["required"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .collect();
        if required
            .iter()
            .any(|name| !fields.is_some_and(|fields| fields.contains_key(*name)))
        {
            return Some(Vec::new());
        }
        if fields.is_some_and(|fields| fields.len() > maximum_properties) {
            return None;
        }
        let mut values = vec![Value::Object(serde_json::Map::new())];
        for (name, child) in fields.into_iter().flatten() {
            let members = collect_member(child, work)?;
            let key_work = values
                .len()
                .checked_mul(members.len())?
                .checked_mul(name.len() + 1)?;
            *work = work.checked_sub(key_work)?;
            values = extend_collection_values(
                values,
                &members,
                !required.contains(name.as_str()),
                work,
                |prefix, member| {
                    let mut fields = prefix.as_object().expect("object candidate").clone();
                    fields.insert(name.clone(), member.clone());
                    Value::Object(fields)
                },
            )?;
            if values.is_empty() {
                break;
            }
        }
        return Some(values);
    }
    if membership == type_membership("array") {
        let (minimum, maximum) = array_length_bounds(&[schema]);
        let maximum = maximum?;
        if maximum < minimum {
            return Some(Vec::new());
        }
        if maximum > maximum_length as u64 {
            return None;
        }
        let mut values = vec![Value::Array(Vec::new())];
        let mut candidates = Vec::new();
        let prefix = prefix_item_schemas(schema);
        for length in 0..=maximum {
            if length >= minimum {
                if candidates.len() + values.len() > maximum_values {
                    return None;
                }
                for value in &values {
                    charge_collection_value(value, 1, work, 0)?;
                }
                candidates.extend(values.iter().cloned());
            }
            if length == maximum {
                break;
            }
            let child = prefix
                .get(length as usize)
                .unwrap_or_else(|| tail_item_schema(schema));
            let members = collect_member(child, work)?;
            values = extend_collection_values(values, &members, false, work, |prefix, member| {
                let mut items = prefix.as_array().expect("array candidate").clone();
                items.push(member.clone());
                Value::Array(items)
            })?;
            if values.is_empty() {
                break;
            }
        }
        return Some(candidates);
    }
    None
}

fn finite_schema_candidates<'schema>(
    root: &'schema Value,
    schema: &'schema Value,
    active: &mut HashSet<*const Value>,
    work: &mut usize,
    depth: usize,
    allowed_types: u8,
) -> Option<(Vec<Cow<'schema, Value>>, &'static str)> {
    *work = work.checked_sub(1)?;
    let identity = std::ptr::from_ref(schema);
    if depth > 128 || active.contains(&identity) {
        return None;
    }
    if let Some(values) = finite_constants(schema) {
        let keyword = if schema.get("const").is_some() {
            "const"
        } else {
            "enum"
        };
        return Some((values.into_iter().map(Cow::Borrowed).collect(), keyword));
    }
    let membership = allowed_types & declared_type_membership(schema);
    if membership == 0 {
        return Some((Vec::new(), "type"));
    }
    if membership & FINITE_SCALAR_TYPES == membership {
        let values = FINITE_SCALAR_VALUES
            .iter()
            .filter(|value| value_type_membership(value) & membership != 0)
            .map(Cow::Borrowed)
            .collect();
        return Some((values, "type"));
    }
    let rules = schema.as_object()?;
    active.insert(identity);
    let result = (|| {
        if let Some(target) = rules
            .get("$ref")
            .and_then(Value::as_str)
            .and_then(|name| reference(root, name))
            && let Some((values, _)) =
                finite_schema_candidates(root, target, active, work, depth + 1, membership)
        {
            return Some((values, "$ref"));
        }
        for keyword in ["anyOf", "oneOf"] {
            let Some(branches) = rules.get(keyword).and_then(Value::as_array) else {
                continue;
            };
            let values = branches.iter().try_fold(Vec::new(), |mut values, child| {
                let (candidates, _) =
                    finite_schema_candidates(root, child, active, work, depth + 1, membership)?;
                *work = work.checked_sub(candidates.len())?;
                values.extend(candidates);
                Some(values)
            });
            if let Some(values) = values {
                return Some((values, keyword));
            }
        }
        let values = finite_collection_values(schema, membership, work, |child, work| {
            let (values, _) =
                finite_schema_candidates(root, child, active, work, depth + 1, ALL_JSON_TYPES)?;
            Some(values)
        })?;
        let keyword = if membership == type_membership("object") {
            "properties"
        } else {
            "items"
        };
        Some((values.into_iter().map(Cow::Owned).collect(), keyword))
    })();
    active.remove(&identity);
    result
}

fn declared_type_membership(schema: &Value) -> u8 {
    schema.get("type").map_or(ALL_JSON_TYPES, |declared| {
        types(declared)
            .iter()
            .filter_map(Value::as_str)
            .fold(0, |membership, name| membership | type_membership(name))
    })
}

fn type_membership(name: &str) -> u8 {
    JSON_TYPES
        .iter()
        .find_map(|&(keyword, membership)| (keyword == name).then_some(membership))
        .unwrap_or(0)
}

fn value_type_membership(value: &Value) -> u8 {
    type_membership(match value {
        Value::Array(_) => "array",
        Value::Bool(_) => "boolean",
        Value::Null => "null",
        Value::Number(number) => {
            return if NumberSpelling::new(&number.to_string()).is_integer() {
                INTEGER_TYPE
            } else {
                FRACTIONAL_TYPE
            };
        }
        Value::Object(_) => "object",
        Value::String(_) => "string",
    })
}

fn possible_type_membership(schema: &Value, allowed_types: u8) -> u8 {
    let declared = allowed_types & declared_type_membership(schema);
    finite_constants(schema).map_or(declared, |constants| {
        declared
            & constants
                .into_iter()
                .fold(0, |types, value| types | value_type_membership(value))
    })
}

fn reference_layers<'schema>(
    root: &'schema Value,
    mut schema: &'schema Value,
) -> Option<Vec<&'schema Value>> {
    let mut layers = Vec::new();
    let mut visited = HashSet::new();
    while schema.is_boolean() || schema.is_object() {
        layers.push(schema);
        let Some(name) = schema.get("$ref") else {
            return Some(layers);
        };
        let name = name.as_str()?;
        if !visited.insert(name) || layers.len() > 128 {
            return None;
        }
        schema = reference(root, name)?;
    }
    None
}

fn referenced_type_membership(root: &Value, branch: &Value, allowed_types: u8) -> u8 {
    reference_layers(root, branch).map_or_else(
        || possible_type_membership(branch, allowed_types),
        |layers| {
            layers.into_iter().fold(allowed_types, |types, layer| {
                possible_type_membership(layer, types)
            })
        },
    )
}

fn branches_use_disjoint_types<'schema>(
    root: &'schema Value,
    branches: impl ExactSizeIterator<Item = &'schema Value>,
    allowed_types: u8,
) -> bool {
    if branches.len() < 2 {
        return true;
    }
    let mut seen = 0;
    for branch in branches {
        if !branch.is_object() && !branch.is_boolean() {
            return false;
        }
        let membership = referenced_type_membership(root, branch, allowed_types);
        if seen & membership != 0 {
            return false;
        }
        seen |= membership;
    }
    true
}

fn finite_string_values(schema: &Value) -> Option<Vec<&str>> {
    finite_constants(schema)?
        .into_iter()
        .map(Value::as_str)
        .collect()
}

fn required_string_tags(schema: &Value) -> HashMap<&str, Vec<&str>> {
    if !matches!(schema.get("type").map(types), Some([value]) if value.as_str() == Some("object"))
        || ["$ref", "anyOf", "oneOf"]
            .iter()
            .any(|key| schema.get(*key).is_some())
    {
        return HashMap::new();
    }
    let Some(required) = schema["required"].as_array() else {
        return HashMap::new();
    };
    required
        .iter()
        .filter_map(|key| {
            let key = key.as_str()?;
            let values = finite_string_values(schema["properties"].get(key)?)?;
            Some((key, values))
        })
        .collect()
}

fn required_prefix_tags<'schema>(
    layers: &[&'schema Value],
    minimum: u64,
) -> HashMap<usize, Vec<&'schema str>> {
    let mut tags = HashMap::new();
    let required = usize::try_from(minimum).unwrap_or(usize::MAX);
    for &layer in layers {
        for (position, schema) in prefix_item_schemas(layer).iter().take(required).enumerate() {
            if let Some(values) = finite_string_values(schema) {
                tags.entry(position).or_insert(values);
            }
        }
    }
    tags
}

fn tags_are_disjoint<Key: Eq + Hash>(
    left: &HashMap<Key, Vec<&str>>,
    right: &HashMap<Key, Vec<&str>>,
) -> bool {
    left.iter().any(|(key, values)| {
        right
            .get(key)
            .is_some_and(|other| string_sets_are_disjoint(values, other))
    })
}

fn branches_use_disjoint_collection_layouts(
    root: &Value,
    branches: &[Value],
    allowed_types: u8,
) -> bool {
    let references: Vec<_> = branches
        .iter()
        .map(|branch| reference_layers(root, branch))
        .collect();
    let tags: Vec<_> = references
        .iter()
        .map(|layers| {
            layers
                .as_ref()
                .and_then(|layers| layers.last().copied())
                .map(required_string_tags)
                .unwrap_or_default()
        })
        .collect();
    let arrays: Vec<_> = references
        .iter()
        .map(|layers| {
            let layers = layers.as_ref()?;
            let membership = layers.iter().fold(allowed_types, |types, layer| {
                possible_type_membership(layer, types)
            });
            (membership == type_membership("array")).then(|| {
                let (minimum, maximum) = array_length_bounds(layers);
                (minimum, maximum, required_prefix_tags(layers, minimum))
            })
        })
        .collect();
    branches.iter().enumerate().all(|(index, branch)| {
        branches[..index]
            .iter()
            .enumerate()
            .all(|(other, previous)| {
                branches_use_disjoint_types(root, [branch, previous].into_iter(), allowed_types)
                    || arrays[index]
                        .as_ref()
                        .zip(arrays[other].as_ref())
                        .is_some_and(
                            |(
                                (left_min, left_max, left_tags),
                                (right_min, right_max, right_tags),
                            )| {
                                let minimum = left_min.max(right_min);
                                left_max
                                    .iter()
                                    .chain(right_max)
                                    .any(|maximum| maximum < minimum)
                                    || tags_are_disjoint(left_tags, right_tags)
                            },
                        )
                    || tags_are_disjoint(&tags[index], &tags[other])
            })
    })
}
