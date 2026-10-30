//! Generic abstract trees with actual concrete source graph references.
use crate::syntax_projection_rules::{
    accept_abstract_syntax_node, accept_concrete_syntax_child_role,
    accept_concrete_syntax_node_type, read_syntax_projection_term,
};
use crate::{
    DecoratorSet, LinkId, LinkMetadata, LinkNetwork, LinkType, MetaLanguageArchiveLimits,
    StructuredDataLinksLimits, StructuredDataValue,
    decode_structured_data_links_ordered_with_limits,
    encode_structured_data_links_ordered_with_limits,
};
use std::collections::{BTreeMap, BTreeSet};

mod model;
mod read;
use model::{Edge, budget, child_edges, fail, island, node_record, source_root};
pub use read::read_abstract_syntax_links;

/// Bounds for complete concrete preservation and abstract construction.
#[derive(Clone, Copy, Debug)]
pub struct SyntaxProjectionLimits {
    /// Full graph node, reference and byte limits.
    pub graph: MetaLanguageArchiveLimits,
    /// Maximum source and abstract tree depth.
    pub depth_limit: usize,
}
impl Default for SyntaxProjectionLimits {
    fn default() -> Self {
        Self {
            graph: MetaLanguageArchiveLimits::default(),
            depth_limit: 256,
        }
    }
}
impl SyntaxProjectionLimits {
    fn data(self) -> StructuredDataLinksLimits {
        StructuredDataLinksLimits {
            node_limit: self.graph.node_limit,
            byte_limit: self.graph.byte_limit,
            depth_limit: self.depth_limit,
        }
    }
}

/// Explicit source-tree selection and inspectable cst-to-ast decorators.
#[derive(Clone, Debug, Default)]
pub struct AbstractSyntaxOptions {
    /// Property path within a complete structured concrete graph.
    pub tree_path: Vec<String>,
    /// Declared decorator operations, preserved in the result graph.
    pub decorators: DecoratorSet,
    /// Finite resource bounds.
    pub limits: SyntaxProjectionLimits,
}
/// Complete graph containing both trees and actual provenance edges.
#[derive(Clone, Debug)]
pub struct AbstractSyntaxLinks {
    /// Original concrete identities plus projected nodes and settings.
    pub network: LinkNetwork,
    /// The explicit abstract projection root.
    pub root: LinkId,
    /// Unchanged structured concrete schema root.
    pub concrete_root: LinkId,
}
/// A reloaded projected child with its actual concrete container and position.
#[derive(Clone, Debug)]
pub struct AbstractSyntaxChild {
    /// Source relationship, including embedding alternatives.
    pub role: String,
    /// Actual original concrete array or property pair.
    pub container: LinkId,
    /// Reference position in that container.
    pub index: usize,
    /// Projected child with its original source identity.
    pub node: std::sync::Arc<AbstractSyntaxNode>,
}
/// A projected node whose concrete data stays directly reachable.
#[derive(Clone, Debug)]
pub struct AbstractSyntaxNode {
    /// Actual projected graph identity.
    pub id: LinkId,
    /// Actual original concrete object identity.
    pub origin: LinkId,
    /// Ordered exact UTF-16 string fields after declared decoration.
    pub properties: std::sync::Arc<StructuredDataValue>,
    /// Ordered projected children with concrete relationship provenance.
    pub children: Vec<AbstractSyntaxChild>,
}
/// A reloaded graph view; no semantic equivalence is implied by this reader.
#[derive(Clone, Debug)]
pub struct ReadAbstractSyntaxLinks {
    /// Original complete structured concrete graph root.
    pub concrete_root: LinkId,
    /// Optional selected abstract root, absent when explicitly dropped.
    pub tree: Option<std::sync::Arc<AbstractSyntaxNode>>,
    /// Actual stored projection selection and decorator records.
    pub settings: StructuredDataValue,
}

struct Builder<'a> {
    network: LinkNetwork,
    options: &'a AbstractSyntaxOptions,
    references: usize,
    active: BTreeSet<LinkId>,
    memo: BTreeMap<LinkId, Option<LinkId>>,
}
impl Builder<'_> {
    fn append(&mut self, references: &[LinkId], metadata: LinkMetadata) -> Result<LinkId, String> {
        self.references = self
            .references
            .checked_add(references.len())
            .ok_or_else(|| fail("graph resource limit exceeded"))?;
        if self.network.len() >= self.options.limits.graph.node_limit
            || self.references > self.options.limits.graph.reference_limit
            || self.network.next_id > 9_007_199_254_740_991
        {
            return Err(fail("graph resource limit exceeded"));
        }
        let id = self.network.insert_link([], metadata);
        self.network.set_references(id, references);
        Ok(id)
    }
    fn node(
        &mut self,
        role: &str,
        references: &[LinkId],
        definition: Option<&str>,
    ) -> Result<LinkId, String> {
        let mut metadata = LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(true)
            .with_language(read_syntax_projection_term("language"))
            .with_term(read_syntax_projection_term(role));
        if let Some(value) = definition {
            metadata = metadata.with_definition(value);
        }
        self.append(references, metadata)
    }
    fn data(&mut self, value: &StructuredDataValue) -> Result<LinkId, String> {
        let encoded =
            encode_structured_data_links_ordered_with_limits(value, self.options.limits.data())?;
        let mut mapping = BTreeMap::new();
        for link in encoded.network.links() {
            mapping.insert(link.id(), self.append(&[], link.metadata().clone())?);
        }
        for link in encoded.network.links() {
            let references: Vec<_> = link.references().iter().map(|id| mapping[id]).collect();
            self.references = self
                .references
                .checked_add(references.len())
                .ok_or_else(|| fail("graph resource limit exceeded"))?;
            if self.references > self.options.limits.graph.reference_limit {
                return Err(fail("graph resource limit exceeded"));
            }
            self.network
                .set_references(mapping[&link.id()], &references);
        }
        Ok(mapping[&encoded.root])
    }
    fn project(&mut self, origin: LinkId, depth: usize) -> Result<Option<LinkId>, String> {
        if depth > self.options.limits.depth_limit {
            return Err(fail("tree depth resource limit exceeded"));
        }
        if self.active.contains(&origin) {
            return Err(fail("cyclic concrete tree"));
        }
        if let Some(value) = self.memo.get(&origin) {
            return Ok(*value);
        }
        let (record, trivia) = node_record(&self.network, origin)?;
        let kind = model::record_type(&record)?;
        let decorated = model::decorate(&record, &self.options.decorators)?;
        if !accept_abstract_syntax_node(&kind, trivia, decorated.is_none()) {
            self.memo.insert(origin, None);
            return Ok(None);
        }
        self.active.insert(origin);
        let mut edges = Vec::new();
        for Edge {
            container,
            index,
            target,
            role,
        } in child_edges(&self.network, origin, &kind)?
        {
            if let Some(target) = self.project(target, depth + 1)? {
                let position = self.node("index", &[], Some(&index.to_string()))?;
                edges.push(self.node("edge", &[container, position, target], Some(&role))?);
            }
        }
        let properties =
            self.data(&decorated.ok_or_else(|| fail("missing projected properties"))?)?;
        let children = self.node("children", &edges, None)?;
        let result = self.node("node", &[origin, properties, children], None)?;
        self.active.remove(&origin);
        self.memo.insert(origin, Some(result));
        Ok(Some(result))
    }
}

/// Construct a generic AST while retaining every original concrete graph link.
/// Nontrivia operators, recovery records and nested grammar boundaries remain.
///
/// # Errors
/// Refuses invalid concrete graphs, unsupported decorators or exceeded budgets.
pub fn create_abstract_syntax_links(
    network: &LinkNetwork,
    concrete_root: LinkId,
    options: &AbstractSyntaxOptions,
) -> Result<AbstractSyntaxLinks, String> {
    budget(network, options.limits)?;
    decode_structured_data_links_ordered_with_limits(
        network,
        concrete_root,
        options.limits.data(),
    )?;
    if options.tree_path.len() > options.limits.depth_limit {
        return Err(fail("invalid concrete tree path"));
    }
    let selected = source_root(network, concrete_root, &options.tree_path)?;
    let mut builder = Builder {
        network: network.clone(),
        options,
        active: BTreeSet::new(),
        memo: BTreeMap::new(),
        references: network.links().map(|link| link.references().len()).sum(),
    };
    let tree = builder.project(selected, 0)?;
    let settings = builder.data(&model::settings(options)?)?;
    let roots = builder.node("children", &tree.into_iter().collect::<Vec<_>>(), None)?;
    let root = builder.node(
        "root",
        &[concrete_root, roots, settings, selected],
        Some("1"),
    )?;
    budget(&builder.network, options.limits)?;
    Ok(AbstractSyntaxLinks {
        network: builder.network,
        root,
        concrete_root,
    })
}
