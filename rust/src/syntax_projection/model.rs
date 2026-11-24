use super::*;
use crate::{DecoratorAction, DecoratorLevel};
use serde_json::json;

pub(super) fn fail(message: &str) -> String {
    format!("syntax projection: {message}")
}
pub(super) fn budget(network: &LinkNetwork, limits: SyntaxProjectionLimits) -> Result<(), String> {
    let values = [
        limits.graph.node_limit,
        limits.graph.reference_limit,
        limits.graph.byte_limit,
        limits.depth_limit,
    ];
    if values
        .iter()
        .any(|value| *value == 0 || *value as u128 > 9_007_199_254_740_991)
    {
        return Err(fail("invalid resource limit"));
    }
    let mut references = 0usize;
    let mut bytes = 0usize;
    let mut maximum = 0u64;
    for link in network.links() {
        let id = link.id().as_u64();
        if id == 0 || id > 9_007_199_254_740_991 {
            return Err(fail("invalid graph identity"));
        }
        maximum = maximum.max(id);
        references = references.saturating_add(link.references().len());
        if link
            .references()
            .iter()
            .any(|id| network.link(*id).is_none())
        {
            return Err(fail("dangling graph reference"));
        }
        for text in [
            link.metadata().term(),
            link.metadata().language(),
            link.metadata().definition(),
        ]
        .into_iter()
        .flatten()
        {
            bytes = bytes.saturating_add(text.len());
        }
    }
    if network.next_id > 9_007_199_254_740_991 || network.next_id <= u128::from(maximum) {
        return Err(fail("invalid allocator state"));
    }
    if network.len() > limits.graph.node_limit
        || references > limits.graph.reference_limit
        || bytes > limits.graph.byte_limit
    {
        return Err(fail("graph resource limit exceeded"));
    }
    Ok(())
}
pub(super) fn island(
    network: &LinkNetwork,
    root: LinkId,
    limits: SyntaxProjectionLimits,
) -> Result<LinkNetwork, String> {
    crate::meta_language_archive::select_network(network, root, limits.graph.node_limit)
}
pub(super) fn fields(
    network: &LinkNetwork,
    root: LinkId,
) -> Result<BTreeMap<String, (LinkId, LinkId)>, String> {
    let link = network
        .link(root)
        .filter(|link| link.metadata().term() == Some("object value"))
        .ok_or_else(|| fail("expected a concrete object"))?;
    link.references()
        .iter()
        .map(|pair| {
            let references = network
                .link(*pair)
                .ok_or_else(|| fail("invalid concrete property"))?
                .references();
            let [name, value] = references else {
                return Err(fail("invalid concrete property"));
            };
            let name = network
                .link(*name)
                .filter(|link| link.metadata().term() == Some("string value"))
                .ok_or_else(|| fail("a concrete property name must be Unicode scalar text"))?;
            Ok((
                name.metadata().definition().unwrap_or("").to_owned(),
                (*pair, *value),
            ))
        })
        .collect()
}
fn scalar(network: &LinkNetwork, root: LinkId) -> Result<Vec<u16>, String> {
    let link = network
        .link(root)
        .ok_or_else(|| fail("expected a scalar concrete property"))?;
    let value = link.metadata().definition().unwrap_or("");
    match link.metadata().term() {
        Some("string value") => Ok(value.encode_utf16().collect()),
        Some("Unicode code unit string") => link
            .references()
            .iter()
            .map(|id| {
                network
                    .link(*id)
                    .and_then(|link| link.metadata().definition())
                    .and_then(|value| value.parse::<u16>().ok())
                    .ok_or_else(|| fail("invalid concrete code unit"))
            })
            .collect(),
        Some("number value") => value
            .parse::<f64>()
            .map(|number| {
                crate::translation::js_number::js_number(number)
                    .encode_utf16()
                    .collect()
            })
            .map_err(|_| fail("invalid concrete number")),
        Some("boolean value") => Ok(value.encode_utf16().collect()),
        Some("null value") => Ok("null".encode_utf16().collect()),
        _ => Err(fail("expected a scalar concrete property")),
    }
}
pub(super) fn node_record(
    network: &LinkNetwork,
    root: LinkId,
) -> Result<(StructuredDataValue, bool), String> {
    let fields = fields(network, root)?;
    let mut record = Vec::new();
    for key in [
        "type", "kind", "field", "language", "text", "hex", "start", "end",
    ] {
        if let Some((_, value)) = fields.get(key) {
            record.push((
                key.encode_utf16().collect(),
                StructuredDataValue::String(scalar(network, *value)?),
            ));
        }
    }
    let record = StructuredDataValue::Object(record);
    let kind = record_type(&record)?;
    if !accept_concrete_syntax_node_type(&kind) {
        return Err(fail("unknown concrete syntax node type"));
    }
    let trivia = fields
        .get("trivia")
        .and_then(|(_, value)| network.link(*value))
        .is_some_and(|link| {
            link.metadata().term() == Some("boolean value")
                && link.metadata().definition() == Some("true")
        });
    Ok((record, trivia))
}
pub(super) fn record_type(record: &StructuredDataValue) -> Result<String, String> {
    let StructuredDataValue::Object(fields) = record else {
        return Err(fail("invalid abstract properties"));
    };
    fields
        .iter()
        .find(|(name, _)| name == &"type".encode_utf16().collect::<Vec<_>>())
        .and_then(|(_, value)| {
            if let StructuredDataValue::String(value) = value {
                String::from_utf16(value).ok()
            } else {
                None
            }
        })
        .ok_or_else(|| fail("unknown concrete syntax node type"))
}
pub(super) fn decorate(
    record: &StructuredDataValue,
    set: &DecoratorSet,
) -> Result<Option<StructuredDataValue>, String> {
    if !set.has(DecoratorLevel::CstToAst) {
        return Ok(Some(record.clone()));
    }
    let StructuredDataValue::Object(fields) = record else {
        return Err(fail("invalid abstract properties"));
    };
    let mut names = Vec::new();
    let mut current = BTreeMap::new();
    for (name, value) in fields {
        let StructuredDataValue::String(value) = value else {
            return Err(fail("invalid abstract properties"));
        };
        let name = String::from_utf16(name).map_err(|_| fail("invalid abstract property name"))?;
        let value = pack_units(value)?;
        names.push(name.clone());
        current.insert(name, value);
    }
    for decorator in set.for_level(DecoratorLevel::CstToAst) {
        // One scalar in the private transport represents exactly one UTF-16
        // unit. Existing equality and nonempty substring replacement therefore
        // retain their actual unit boundaries, including isolated surrogates.
        let conditions = decorator
            .when()
            .iter()
            .map(|(field, value)| Ok((field.clone(), pack_text(value)?)))
            .collect::<Result<_, String>>()?;
        let actions = decorator
            .actions()
            .iter()
            .map(|action| {
                Ok(match action {
                    DecoratorAction::Set { field, value } => DecoratorAction::Set {
                        field: field.clone(),
                        value: pack_text(value)?,
                    },
                    DecoratorAction::Replace { field, from, to } => DecoratorAction::Replace {
                        field: field.clone(),
                        from: pack_text(from)?,
                        to: pack_text(to)?,
                    },
                    DecoratorAction::Drop => DecoratorAction::Drop,
                })
            })
            .collect::<Result<_, String>>()?;
        let encoded = crate::Decorator::new(
            decorator.id(),
            decorator.level(),
            decorator.order(),
            conditions,
            actions,
        )
        .map_err(|error| fail(&error.to_string()))?;
        let one = DecoratorSet::new(vec![encoded]).map_err(|error| fail(&error.to_string()))?;
        let Some(next) = one.decorate(DecoratorLevel::CstToAst, &current) else {
            return Ok(None);
        };
        for action in decorator.actions() {
            let field = match action {
                DecoratorAction::Set { field, .. } | DecoratorAction::Replace { field, .. } => {
                    Some(field)
                }
                DecoratorAction::Drop => None,
            };
            if let Some(field) =
                field.filter(|field| next.contains_key(*field) && !names.contains(*field))
            {
                names.push(field.clone());
            }
        }
        current = next;
    }
    let fields = names
        .into_iter()
        .map(|name| {
            Ok((
                name.encode_utf16().collect(),
                StructuredDataValue::String(unpack_units(&current[&name])?),
            ))
        })
        .collect::<Result<_, String>>()?;
    Ok(Some(StructuredDataValue::Object(fields)))
}
fn pack_text(value: &str) -> Result<String, String> {
    pack_units(&value.encode_utf16().collect::<Vec<_>>())
}
fn pack_units(value: &[u16]) -> Result<String, String> {
    value
        .iter()
        .map(|unit| {
            char::from_u32(0x10000 + u32::from(*unit))
                .ok_or_else(|| fail("invalid code unit transport"))
        })
        .collect()
}
fn unpack_units(value: &str) -> Result<Vec<u16>, String> {
    value
        .chars()
        .map(|unit| {
            u32::from(unit)
                .checked_sub(0x10000)
                .and_then(|unit| u16::try_from(unit).ok())
                .ok_or_else(|| fail("invalid code unit transport"))
        })
        .collect()
}
#[derive(Clone)]
pub(super) struct Edge {
    pub container: LinkId,
    pub index: usize,
    pub target: LinkId,
    pub role: String,
}
fn array_edges(
    network: &LinkNetwork,
    selected: Option<&(LinkId, LinkId)>,
    role: &str,
    edges: &mut Vec<Edge>,
) -> Result<(), String> {
    let Some((_, container)) = selected else {
        return Ok(());
    };
    let values = network
        .link(*container)
        .filter(|link| link.metadata().term() == Some("array value"))
        .ok_or_else(|| fail("expected concrete child array"))?;
    edges.extend(
        values
            .references()
            .iter()
            .enumerate()
            .map(|(index, target)| Edge {
                container: *container,
                index,
                target: *target,
                role: role.to_owned(),
            }),
    );
    Ok(())
}
pub(super) fn child_edges(
    network: &LinkNetwork,
    origin: LinkId,
    kind: &str,
) -> Result<Vec<Edge>, String> {
    let fields = fields(network, origin)?;
    let mut edges = Vec::new();
    if accept_concrete_syntax_child_role(kind, "children") {
        array_edges(network, fields.get("children"), "children", &mut edges)?;
    }
    if accept_concrete_syntax_child_role(kind, "root") {
        let (container, target) = fields
            .get("root")
            .ok_or_else(|| fail("an embedding needs its concrete root"))?;
        edges.push(Edge {
            container: *container,
            target: *target,
            index: 1,
            role: "root".to_owned(),
        });
    }
    if accept_concrete_syntax_child_role(kind, "sourceChildren") {
        array_edges(
            network,
            fields.get("sourceChildren"),
            "sourceChildren",
            &mut edges,
        )?;
    }
    if accept_concrete_syntax_child_role(kind, "alternative root") {
        if let Some((_, selection)) = fields.get("selection") {
            if let Some((_, alternatives)) = self::fields(network, *selection)?.get("alternatives")
            {
                let list = network
                    .link(*alternatives)
                    .filter(|link| link.metadata().term() == Some("array value"))
                    .ok_or_else(|| fail("expected concrete alternatives"))?;
                for alternative in list.references() {
                    let selected = self::fields(network, *alternative)?;
                    let (container, target) = selected
                        .get("root")
                        .ok_or_else(|| fail("an alternative needs its concrete root"))?;
                    edges.push(Edge {
                        container: *container,
                        index: 1,
                        target: *target,
                        role: "alternative root".to_owned(),
                    });
                }
            }
        }
    }
    Ok(edges)
}
pub(super) fn source_root(
    network: &LinkNetwork,
    root: LinkId,
    path: &[String],
) -> Result<LinkId, String> {
    let schema = network
        .link(root)
        .filter(|link| {
            link.metadata().term() == Some("structured data") && link.references().len() == 1
        })
        .ok_or_else(|| fail("expected a complete structured concrete graph"))?;
    let mut selected = schema.references()[0];
    for name in path {
        selected = fields(network, selected)?
            .get(name)
            .ok_or_else(|| fail("missing concrete tree path"))?
            .1;
    }
    Ok(selected)
}
pub(super) fn settings(options: &AbstractSyntaxOptions) -> Result<StructuredDataValue, String> {
    crate::structured_data_links::from_json(
        &json!({"schemaVersion":1,"treePath":options.tree_path,
        "decorators":crate::decorator_data::write(&options.decorators)}),
        1,
    )
}
