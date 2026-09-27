//! Name lookup for the four-language program analyzer: the deepest visible
//! declaration for plain names, where Rust `use` aliases stand for the item
//! they import and `$name` only sees macro metavariables, and qualified Rust
//! paths through module bodies.

use std::cmp::Ordering;

use super::extents::{is_rust_item, rust_use_aliases, Visibility};
use super::{
    BTreeSet, Declaration, ProgramFact, ProgramRange, ProgramScope, ProgramSourceMapping,
    SemanticToken, TokenKind,
};

const RUST_PATH_ROOTS: &[&str] = &["self", "super", "crate"];

struct Entry<'a> {
    name: &'a str,
    scope: usize,
    start: usize,
    visibility: Visibility,
    target: usize,
}

struct Resolver<'a> {
    tokens: &'a [SemanticToken],
    language: &'a str,
    declarations: &'a [Declaration],
    parents: Vec<Option<usize>>,
    depths: Vec<usize>,
    entries: Vec<Entry<'a>>,
    module_bodies: BTreeSet<usize>,
}

/// References of each declaration (sorted by source position) and the names
/// that resolve to no declaration.
pub(super) fn resolve_references(
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    language: &str,
    scopes: &[ProgramScope],
    declarations: &[Declaration],
) -> (Vec<Vec<ProgramRange>>, Vec<ProgramFact>) {
    let mut resolver = Resolver {
        tokens,
        language,
        declarations,
        parents: scopes
            .iter()
            .map(|scope| {
                scope
                    .parent
                    .as_deref()
                    .and_then(|parent| scopes.iter().position(|candidate| candidate.id == parent))
            })
            .collect(),
        depths: scopes.iter().map(|scope| scope.depth).collect(),
        entries: declarations
            .iter()
            .enumerate()
            .map(|(index, declaration)| Entry {
                name: &tokens[declaration.token].text,
                scope: declaration.scope,
                start: tokens[declaration.token].range.start,
                visibility: declaration.visibility,
                target: index,
            })
            .collect(),
        module_bodies: declarations
            .iter()
            .filter_map(|declaration| declaration.visibility.body)
            .collect(),
    };
    if language == "Rust" {
        let aliases = rust_use_aliases(tokens, syntax)
            .into_iter()
            .filter_map(|index| {
                let token = &tokens[index];
                resolver.path_target(index).map(|target| Entry {
                    name: &token.text,
                    scope: token.scope,
                    start: token.range.start,
                    visibility: Visibility::default(),
                    target,
                })
            })
            .collect::<Vec<_>>();
        resolver.entries.extend(aliases);
    }
    let ignored_terms: &[&str] = match language {
        "JavaScript" => &["property_identifier", "private_property_identifier"],
        "Rust" => &["field_identifier", "fragment_specifier"],
        _ => &[],
    };
    let ignored = syntax
        .iter()
        .filter(|fact| ignored_terms.contains(&fact.term.as_str()))
        .map(|fact| fact.range)
        .collect::<BTreeSet<_>>();
    let declared = declarations
        .iter()
        .map(|declaration| declaration.token)
        .collect::<BTreeSet<_>>();

    let mut references = vec![Vec::new(); declarations.len()];
    let mut unresolved = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        if token.kind != TokenKind::Identifier
            || declared.contains(&index)
            || ignored.contains(&token.range)
        {
            continue;
        }
        if language == "Rust" && index > 0 && tokens[index - 1].text == "::" {
            // Qualified paths resolve through module bodies; paths into
            // external crates or types are outside the analyzed program.
            if let Some(target) = resolver.path_target(index) {
                references[target].push(token.range);
            }
            continue;
        }
        if let Some(target) = resolver.lexical(index) {
            references[target].push(token.range);
        } else {
            unresolved.push(ProgramFact::new("unresolved", &token.text, token.range));
        }
    }
    (references, unresolved)
}

impl Resolver<'_> {
    fn contains(&self, outer: usize, inner: usize) -> bool {
        let mut current = Some(inner);
        while let Some(scope) = current {
            if scope == outer {
                return true;
            }
            current = self.parents[scope];
        }
        false
    }

    // Candidate order: deepest scope, then the latest declaration before the
    // use, then the earliest after it.
    fn prefer(&self, left: &Entry<'_>, right: &Entry<'_>, start: usize) -> Ordering {
        self.depths[left.scope]
            .cmp(&self.depths[right.scope])
            .then_with(|| {
                let left_before = left.start <= start;
                let right_before = right.start <= start;
                left_before.cmp(&right_before).then_with(|| {
                    if left_before {
                        left.start.cmp(&right.start)
                    } else {
                        right.start.cmp(&left.start)
                    }
                })
            })
    }

    /// The declaration a plain name refers to, through `use` aliases.
    fn lexical(&self, index: usize) -> Option<usize> {
        let token = &self.tokens[index];
        let sigil = self.language == "Rust"
            && index > 0
            && self.tokens[index - 1].text == "$"
            && self.tokens[index - 1].range.end == token.range.start;
        self.entries
            .iter()
            .filter(|entry| {
                entry.name == token.text
                    && entry.visibility.sigil == sigil
                    && entry.visibility.from <= token.range.start
                    && token.range.start < entry.visibility.until
                    && self.contains(entry.scope, token.scope)
            })
            // `max_by` keeps the last of equal candidates; reversing keeps
            // the first, like the JavaScript runtime's stable sort.
            .rev()
            .max_by(|left, right| self.prefer(left, right, token.range.start))
            .map(|entry| entry.target)
    }

    fn nearest_module(&self, scope: usize) -> usize {
        let mut current = scope;
        while let Some(parent) = self.parents[current] {
            if self.module_bodies.contains(&current) {
                break;
            }
            current = parent;
        }
        current
    }

    fn parent_module(&self, module: usize) -> Option<usize> {
        self.parents[module].map(|parent| self.nearest_module(parent))
    }

    fn member(&self, module: usize, name: &str) -> Option<usize> {
        self.declarations.iter().position(|declaration| {
            declaration.scope == module
                && self.tokens[declaration.token].text == name
                && is_rust_item(&declaration.kind)
        })
    }

    fn body(&self, declaration: Option<usize>) -> Option<usize> {
        declaration.and_then(|declaration| self.declarations[declaration].visibility.body)
    }

    /// Resolves the last segment of a Rust `a::b::name` path through module
    /// bodies; `crate`, `self`, and `super` name the root, enclosing, and
    /// parent modules. `None` for paths outside the analyzed modules.
    fn path_target(&self, index: usize) -> Option<usize> {
        let tokens = self.tokens;
        let mut segments = vec![index];
        let mut cursor = index;
        while cursor >= 2
            && tokens[cursor - 1].text == "::"
            && (tokens[cursor - 2].kind == TokenKind::Identifier
                || RUST_PATH_ROOTS.contains(&tokens[cursor - 2].text.as_str()))
        {
            cursor -= 2;
            segments.insert(0, cursor);
        }
        if segments.len() < 2 || (cursor > 0 && tokens[cursor - 1].text == "::") {
            return None;
        }
        let head = &tokens[segments[0]];
        let mut module = match head.text.as_str() {
            "crate" => Some(0),
            "self" => Some(self.nearest_module(head.scope)),
            "super" => self.parent_module(self.nearest_module(head.scope)),
            _ if head.kind == TokenKind::Identifier => self.body(self.lexical(segments[0])),
            _ => None,
        };
        for &segment in &segments[1..segments.len() - 1] {
            let current = module?;
            module = if tokens[segment].text == "super" {
                self.parent_module(current)
            } else {
                self.body(self.member(current, &tokens[segment].text))
            };
        }
        self.member(module?, &tokens[index].text)
    }
}
