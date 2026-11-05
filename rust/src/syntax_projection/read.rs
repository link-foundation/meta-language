use super::*;
use crate::decode_structured_data_links_with_limits;
use std::sync::Arc;

fn required(
    network: &LinkNetwork,
    id: LinkId,
    role: &str,
    arity: Option<usize>,
) -> Result<Vec<LinkId>, String> {
    let link = network
        .link(id)
        .ok_or_else(|| fail("invalid projection link"))?;
    let metadata = link.metadata();
    if metadata.language() != Some(read_syntax_projection_term("language").as_str())
        || metadata.term() != Some(read_syntax_projection_term(role).as_str())
        || metadata.link_type() != Some(LinkType::Syntax)
        || !metadata.is_named()
        || metadata.span().is_some()
        || metadata.flags() != crate::LinkFlags::default()
        || arity.is_some_and(|arity| link.references().len() != arity)
    {
        return Err(fail("invalid projection link"));
    }
    Ok(link.references().to_vec())
}
struct Reader<'a> {
    network: &'a LinkNetwork,
    limits: SyntaxProjectionLimits,
    active: BTreeSet<LinkId>,
    memo: BTreeMap<LinkId, Arc<AbstractSyntaxNode>>,
    heights: BTreeMap<LinkId, usize>,
    properties: BTreeMap<LinkId, Arc<StructuredDataValue>>,
    property_bytes: usize,
}
impl Reader<'_> {
    fn properties(&mut self, root: LinkId) -> Result<Arc<StructuredDataValue>, String> {
        if let Some(value) = self.properties.get(&root) {
            return Ok(Arc::clone(value));
        }
        let network = island(self.network, root, self.limits)?;
        let value =
            decode_structured_data_links_ordered_with_limits(&network, root, self.limits.data())?;
        let StructuredDataValue::Object(fields) = &value else {
            return Err(fail("invalid abstract properties"));
        };
        for (key, value) in fields {
            let StructuredDataValue::String(value) = value else {
                return Err(fail("invalid abstract properties"));
            };
            self.property_bytes = self
                .property_bytes
                .saturating_add(key.len().saturating_add(value.len()).saturating_mul(2));
        }
        if self.property_bytes > self.limits.graph.byte_limit {
            return Err(fail("materialized property byte limit exceeded"));
        }
        let value = Arc::new(value);
        self.properties.insert(root, Arc::clone(&value));
        Ok(value)
    }
    fn node(&mut self, id: LinkId, depth: usize) -> Result<Arc<AbstractSyntaxNode>, String> {
        if depth > self.limits.depth_limit {
            return Err(fail("tree depth resource limit exceeded"));
        }
        if self.active.contains(&id) {
            return Err(fail("cyclic abstract tree"));
        }
        if let Some(value) = self.memo.get(&id) {
            if depth.saturating_add(self.heights[&id]) > self.limits.depth_limit {
                return Err(fail("tree depth resource limit exceeded"));
            }
            return Ok(Arc::clone(value));
        }
        let refs = required(self.network, id, "node", Some(3))?;
        let (origin, properties, children) = (refs[0], refs[1], refs[2]);
        let (record, _) = node_record(self.network, origin)?;
        let expected: BTreeSet<_> =
            child_edges(self.network, origin, &model::record_type(&record)?)?
                .into_iter()
                .map(|edge| (edge.container, edge.index, edge.role, edge.target))
                .collect();
        self.active.insert(id);
        let mut result = Vec::new();
        let mut height = 0usize;
        for reference in required(self.network, children, "children", None)? {
            let refs = required(self.network, reference, "edge", Some(3))?;
            let (container, position, target) = (refs[0], refs[1], refs[2]);
            required(self.network, position, "index", Some(0))?;
            let spelling = self
                .network
                .link(position)
                .and_then(|link| link.metadata().definition())
                .unwrap_or("");
            let index = spelling
                .parse::<usize>()
                .map_err(|_| fail("invalid concrete child position"))?;
            if index as u128 > 9_007_199_254_740_991 || index.to_string() != spelling {
                return Err(fail("invalid concrete child position"));
            }
            let child = self.node(target, depth + 1)?;
            let role = self
                .network
                .link(reference)
                .and_then(|link| link.metadata().definition())
                .unwrap_or("");
            if !expected.contains(&(container, index, role.to_owned(), child.origin)) {
                return Err(fail("abstract edge lost its concrete origin"));
            }
            height = height.max(self.heights[&target].saturating_add(1));
            result.push(AbstractSyntaxChild {
                role: role.to_owned(),
                container,
                index,
                node: child,
            });
        }
        if depth.saturating_add(height) > self.limits.depth_limit {
            return Err(fail("tree depth resource limit exceeded"));
        }
        let properties = self.properties(properties)?;
        let value = Arc::new(AbstractSyntaxNode {
            id,
            origin,
            properties,
            children: result,
        });
        self.active.remove(&id);
        self.heights.insert(id, height);
        self.memo.insert(id, Arc::clone(&value));
        Ok(value)
    }
}

/// Reload edited AST links with actual source relationships and exact fields.
///
/// # Errors
/// Refuses malformed graph roles, source selections, provenance and budgets.
pub fn read_abstract_syntax_links(
    network: &LinkNetwork,
    root: LinkId,
    limits: SyntaxProjectionLimits,
) -> Result<ReadAbstractSyntaxLinks, String> {
    budget(network, limits)?;
    let refs = required(network, root, "root", Some(4))?;
    if network
        .link(root)
        .and_then(|link| link.metadata().definition())
        != Some("1")
    {
        return Err(fail("unknown projection schema"));
    }
    let (concrete_root, roots, settings_root, selected) = (refs[0], refs[1], refs[2], refs[3]);
    decode_structured_data_links_ordered_with_limits(
        &island(network, concrete_root, limits)?,
        concrete_root,
        limits.data(),
    )?;
    let settings_network = island(network, settings_root, limits)?;
    let settings = decode_structured_data_links_ordered_with_limits(
        &settings_network,
        settings_root,
        limits.data(),
    )?;
    let value =
        decode_structured_data_links_with_limits(&settings_network, settings_root, limits.data())?;
    let path: Vec<String> = serde_json::from_value(value["treePath"].clone())
        .map_err(|_| fail("invalid projection settings"))?;
    if value["schemaVersion"].as_f64() != Some(1.0)
        || path.len() > limits.depth_limit
        || !value["decorators"].is_array()
    {
        return Err(fail("invalid projection settings"));
    }
    crate::decorator_data::read(&value["decorators"])?;
    if source_root(network, concrete_root, &path)? != selected {
        return Err(fail("concrete root selection changed"));
    }
    let roots = required(network, roots, "children", None)?;
    if roots.len() > 1 {
        return Err(fail("invalid abstract root selection"));
    }
    let mut reader = Reader {
        network,
        limits,
        active: BTreeSet::new(),
        memo: BTreeMap::new(),
        heights: BTreeMap::new(),
        properties: BTreeMap::new(),
        property_bytes: 0,
    };
    let tree = roots
        .first()
        .map(|root| reader.node(*root, 0))
        .transpose()?;
    if tree.as_ref().is_some_and(|tree| tree.origin != selected) {
        return Err(fail("abstract root lost its concrete origin"));
    }
    Ok(ReadAbstractSyntaxLinks {
        concrete_root,
        tree,
        settings,
    })
}
