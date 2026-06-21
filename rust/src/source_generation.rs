use std::collections::BTreeSet;

use crate::link_network::{Link, LinkId, LinkMetadata, LinkNetwork, LinkType};

impl LinkNetwork {
    /// Inserts a source token that can be rendered without an original source span.
    ///
    /// This is the construction-side counterpart to parser-created token
    /// links: the token text is stored in the link term, but no byte range is
    /// required.
    #[must_use]
    pub fn insert_source_token(&mut self, language: &str, text: &str) -> LinkId {
        self.insert_link(
            [],
            LinkMetadata::new()
                .with_link_type(LinkType::Token)
                .with_named(!text.trim().is_empty())
                .with_term(text)
                .with_language(language),
        )
    }

    /// Inserts a syntax node whose ordered references are renderable children.
    ///
    /// The `kind` should match the grammar node vocabulary used by the
    /// corresponding parser where possible. Rendering only emits descendant
    /// token text; the kind is metadata for queries and round-trip validation.
    #[must_use]
    pub fn insert_syntax_node<const N: usize>(
        &mut self,
        language: &str,
        kind: &str,
        children: [LinkId; N],
    ) -> LinkId {
        self.insert_link(
            children,
            LinkMetadata::new()
                .with_link_type(LinkType::Syntax)
                .with_named(true)
                .with_term(kind)
                .with_language(language),
        )
    }

    /// Renders source text for `language` from a parsed or constructed network.
    ///
    /// Parsed networks are rendered from their document root. Hand-built
    /// networks without document links are rendered from top-level syntax or
    /// token links whose metadata language matches `language`.
    #[must_use]
    pub fn render_source(&self, language: &str) -> String {
        if let Some(source) = self.render_source_from_document(language) {
            return source;
        }

        self.render_source_roots(language)
    }

    /// Renders source text from a specific syntax, document, region, or token link.
    #[must_use]
    pub fn render_source_from(&self, root: LinkId, language: &str) -> String {
        let mut visiting = BTreeSet::new();
        self.render_link(root, language, &mut visiting)
    }

    /// Renders source text from document links matching `language`.
    ///
    /// Returns `None` when the network has no matching document link, which is
    /// common for programmatically constructed syntax fragments.
    #[must_use]
    pub fn render_source_from_document(&self, language: &str) -> Option<String> {
        let mut documents = self
            .links()
            .filter(|link| {
                link.metadata().link_type() == Some(LinkType::Document)
                    && language_matches(link.metadata().language(), language)
            })
            .map(Link::id)
            .peekable();

        documents.peek()?;

        let mut source = String::new();
        for document in documents {
            source.push_str(&self.render_source_from(document, language));
        }
        Some(source)
    }

    fn render_source_roots(&self, language: &str) -> String {
        let child_ids = self.renderable_child_ids(language);
        let mut roots = self
            .links()
            .filter(|link| renderable_root(link, language))
            .filter(|link| !child_ids.contains(&link.id()))
            .map(Link::id)
            .collect::<Vec<_>>();
        roots.sort_unstable_by_key(|id| id.as_u64());

        let mut source = String::new();
        for root in roots {
            source.push_str(&self.render_source_from(root, language));
        }
        source
    }

    fn renderable_child_ids(&self, language: &str) -> BTreeSet<LinkId> {
        let mut child_ids = BTreeSet::new();
        for link in self
            .links()
            .filter(|link| renderable_container(link, language))
        {
            for child in self.render_children(link, language) {
                child_ids.insert(child);
            }
        }
        child_ids
    }

    fn render_link(&self, id: LinkId, language: &str, visiting: &mut BTreeSet<LinkId>) -> String {
        let Some(link) = self.link(id) else {
            return String::new();
        };
        if !renderable_link(link, language) || link.metadata().flags().is_missing() {
            return String::new();
        }
        if link.metadata().link_type() == Some(LinkType::Token) {
            return link.metadata().term().unwrap_or_default().to_string();
        }
        if !visiting.insert(id) {
            return String::new();
        }

        let mut source = String::new();
        for child in self.render_children(link, language) {
            source.push_str(&self.render_link(child, language, visiting));
        }
        visiting.remove(&id);
        source
    }

    fn render_children(&self, link: &Link, language: &str) -> Vec<LinkId> {
        if uses_owned_child_links(link) {
            let owned_children = self.owned_render_children(link.id(), language);
            if !owned_children.is_empty() {
                return owned_children;
            }
        }

        if link.metadata().span().is_none() {
            let direct_children = self.direct_render_children(link, language);
            if !direct_children.is_empty() {
                return direct_children;
            }
        }

        self.field_render_children(link.id(), language)
    }

    fn owned_render_children(&self, parent: LinkId, language: &str) -> Vec<LinkId> {
        let mut children = self
            .links()
            .filter(|link| link.id() != parent)
            .filter(|link| {
                link.references()
                    .first()
                    .is_some_and(|reference| *reference == parent)
            })
            .filter(|link| renderable_child(link, language))
            .map(Link::id)
            .collect::<Vec<_>>();
        self.sort_children_by_source_order(&mut children);
        children
    }

    fn direct_render_children(&self, link: &Link, language: &str) -> Vec<LinkId> {
        let mut children = Vec::new();
        let mut seen = BTreeSet::new();
        for child in link.references().iter().copied() {
            if child == link.id() || !seen.insert(child) {
                continue;
            }
            let Some(child_link) = self.link(child) else {
                continue;
            };
            if renderable_child(child_link, language) {
                children.push(child);
            }
        }
        children
    }

    fn field_render_children(&self, parent: LinkId, language: &str) -> Vec<LinkId> {
        let mut fields = self
            .links()
            .filter(|link| link.metadata().link_type() == Some(LinkType::Field))
            .filter(|link| {
                link.references()
                    .first()
                    .is_some_and(|reference| *reference == parent)
            })
            .filter_map(|field| {
                field
                    .references()
                    .get(2)
                    .copied()
                    .map(|child| (field.id(), child))
            })
            .filter(|(_field, child)| {
                self.link(*child)
                    .is_some_and(|link| renderable_child(link, language))
            })
            .collect::<Vec<_>>();
        fields.sort_unstable_by_key(|(field, _child)| field.as_u64());

        let mut children = Vec::new();
        let mut seen = BTreeSet::new();
        for (_field, child) in fields {
            if seen.insert(child) {
                children.push(child);
            }
        }
        children
    }

    fn sort_children_by_source_order(&self, children: &mut [LinkId]) {
        children.sort_unstable_by_key(|id| {
            let span = self.link(*id).and_then(|link| link.metadata().span());
            (
                span.is_none(),
                span.map_or(usize::MAX, |span| span.byte_range().start()),
                id.as_u64(),
            )
        });
    }
}

fn renderable_root(link: &Link, language: &str) -> bool {
    matches!(
        link.metadata().link_type(),
        Some(LinkType::Syntax | LinkType::Token)
    ) && language_matches(link.metadata().language(), language)
        && !link.metadata().flags().is_missing()
}

fn renderable_container(link: &Link, language: &str) -> bool {
    matches!(
        link.metadata().link_type(),
        Some(LinkType::Document | LinkType::Region | LinkType::Syntax)
    ) && language_matches(link.metadata().language(), language)
        && !link.metadata().flags().is_missing()
}

fn renderable_child(link: &Link, language: &str) -> bool {
    matches!(
        link.metadata().link_type(),
        Some(LinkType::Syntax | LinkType::Token)
    ) && language_matches(link.metadata().language(), language)
        && !link.metadata().flags().is_missing()
}

fn renderable_link(link: &Link, language: &str) -> bool {
    matches!(
        link.metadata().link_type(),
        Some(LinkType::Document | LinkType::Region | LinkType::Syntax | LinkType::Token)
    ) && language_matches(link.metadata().language(), language)
}

const fn uses_owned_child_links(link: &Link) -> bool {
    link.metadata().span().is_some()
        || matches!(
            link.metadata().link_type(),
            Some(LinkType::Document | LinkType::Region)
        )
}

fn language_matches(source_language: Option<&str>, target_language: &str) -> bool {
    source_language.map_or(true, |source_language| {
        source_language.eq_ignore_ascii_case(target_language)
    })
}
