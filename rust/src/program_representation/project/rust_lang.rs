//! Project-aware Rust semantics: the Cargo manifest, the crate's module tree
//! loaded from `mod name;` declarations (`name.rs` or `name/mod.rs`), use
//! trees and paths resolved with item visibility, `#[macro_use]` modules, and
//! `macro_rules!` invocations expanded by their matching rule, and project
//! references checked by `assert!` macros: compile-time evaluated inside a
//! `const` or `static` item, runtime-checked elsewhere (Rust has no proof
//! terms). The JavaScript runtime implements the same analysis in
//! js/src/program-project-rust.js.

use std::collections::BTreeMap;
use std::rc::Rc;

use super::tree::{NodeId, SyntaxTree, Target, declaration, dirname, join_path, toml_entries};
use super::{ParsedFile, ProjectContext};

const PATH_SEGMENTS: [&str; 5] = ["identifier", "type_identifier", "crate", "self", "super"];
const SCOPED: [&str; 2] = ["scoped_identifier", "scoped_type_identifier"];
const NON_REFERENCE_CONTEXTS: [&str; 5] = [
    "use_declaration",
    "mod_item",
    "attribute_item",
    "inner_attribute_item",
    "macro_definition",
];
const ASSERT_MACROS: [&str; 6] = [
    "assert",
    "assert_eq",
    "assert_ne",
    "debug_assert",
    "debug_assert_eq",
    "debug_assert_ne",
];

/// The declaration kind and name term of an item node.
fn item_kind(term: &str) -> Option<(&'static str, &'static str)> {
    Some(match term {
        "function_item" => ("function", "identifier"),
        "struct_item" => ("struct", "type_identifier"),
        "enum_item" => ("enum", "type_identifier"),
        "union_item" => ("union", "type_identifier"),
        "trait_item" => ("trait", "type_identifier"),
        "type_item" => ("type", "type_identifier"),
        "const_item" => ("constant", "identifier"),
        "static_item" => ("static", "identifier"),
        "mod_item" => ("module", "identifier"),
        "macro_definition" => ("macro", "identifier"),
        _ => return None,
    })
}

pub(super) fn analyze(context: &mut ProjectContext<'_>) {
    read_cargo_manifest(context);
    let entry = Rc::clone(&context.entry);
    let tree = &entry.tree;
    let mut krate = Crate {
        context,
        records: Vec::new(),
        modules: BTreeMap::new(),
    };
    krate.load_file("crate", &entry.path, true);
    for node in tree.nodes_with(&["mod_item"]) {
        if tree.first_child(node, &["declaration_list"]).is_some() {
            continue;
        }
        let Some(name) = tree.first_child(node, &["identifier"]) else {
            continue;
        };
        let name = tree.text(name);
        krate.context.request(name, tree.span(node));
        let qualified = format!("{}::{name}", krate.module_of(node));
        if let Some(record) = krate.modules.get(&qualified) {
            let file = krate.records[*record].file.path.clone();
            krate.context.module(name, &file, &file, tree.span(node));
        }
    }
    let mut locals: BTreeMap<String, Local> = BTreeMap::new();
    let role = |node: NodeId| {
        let Some(invocation) = tree.ancestor(node, &["macro_invocation"]) else {
            return "reference";
        };
        let asserts = tree.child(invocation, 0).is_some_and(|name| {
            tree.term(name) == "identifier" && ASSERT_MACROS.contains(&tree.text(name))
        });
        if !asserts {
            "reference"
        } else if tree
            .ancestor(invocation, &["const_item", "static_item"])
            .is_some()
        {
            "const-assertion"
        } else {
            "assertion"
        }
    };
    for node in tree.nodes_with(&["use_declaration"]) {
        krate.use_declaration(node, &mut locals);
    }
    for node in tree.nodes_with(&["attribute_item"]) {
        krate.attribute(node);
    }
    for node in tree.nodes_with(&SCOPED) {
        if tree
            .parent_term(node)
            .is_some_and(|term| SCOPED.contains(&term))
            || tree.ancestor(node, &NON_REFERENCE_CONTEXTS).is_some()
        {
            continue;
        }
        let segments = path_segments(tree, node);
        let from = krate.module_of(node);
        if let Some(Resolved::Found {
            target: Some(target),
            ..
        }) = krate.resolve(&segments, &from, &locals)
        {
            krate
                .context
                .reference(role(node), tree.text(node), tree.span(node), &target);
        }
    }
    let unresolved = krate.context.unresolved;
    for fact in unresolved {
        let Some(leaf) = tree.leaf_at(fact.range.start(), fact.range.end()) else {
            continue;
        };
        if tree.ancestor(leaf, &NON_REFERENCE_CONTEXTS).is_some()
            || tree
                .parent_term(leaf)
                .is_some_and(|term| SCOPED.contains(&term))
            || (tree.parent_term(leaf) == Some("macro_invocation") && tree.is_first_child(leaf))
        {
            continue;
        }
        if let Some(target) = locals
            .get(&fact.name)
            .and_then(|local| local.target.as_ref())
        {
            let range = (fact.range.start(), fact.range.end());
            krate
                .context
                .reference(role(leaf), &fact.name, range, target);
        }
    }
    for node in tree.nodes_with(&["macro_invocation"]) {
        krate.macro_invocation(node);
    }
}

/// Cargo.toml: the package name and edition and the declared dependencies.
fn read_cargo_manifest(context: &mut ProjectContext<'_>) {
    let Some(source) = context.text("Cargo.toml").map(str::to_string) else {
        return;
    };
    context.fact(
        "project-manifest",
        "Cargo.toml",
        "Cargo.toml",
        (0, source.len()),
    );
    for entry in toml_entries(&source) {
        let range = (entry.start, entry.end);
        if entry.table == "package" && entry.key == "name" {
            context.fact("project-package", &entry.value, "Cargo.toml", range);
        }
        if entry.table == "package" && entry.key == "edition" {
            context.fact("project-edition", &entry.value, "Cargo.toml", range);
        }
        if ["dependencies", "dev-dependencies", "build-dependencies"]
            .contains(&entry.table.as_str())
        {
            context.fact("project-dependency", &entry.key, "Cargo.toml", range);
        }
    }
}

fn path_segments(tree: &SyntaxTree, node: NodeId) -> Vec<String> {
    tree.leaves_of(node)
        .into_iter()
        .filter(|leaf| PATH_SEGMENTS.contains(&tree.term(*leaf)))
        .map(|leaf| tree.text(leaf).to_string())
        .collect()
}

fn collapse_whitespace(text: &str) -> String {
    let mut result = String::new();
    let mut space = false;
    for character in text.chars() {
        if matches!(character, ' ' | '\t' | '\n' | '\r') {
            space = !result.is_empty();
            continue;
        }
        if space {
            result.push(' ');
        }
        result.push(character);
        space = false;
    }
    result
}

#[derive(Clone, Debug)]
struct Item {
    name: String,
    kind: &'static str,
    public: bool,
    attributes: Vec<String>,
    node: NodeId,
    module: Option<usize>,
    target: Option<Target>,
    /// The module record declaring a macro.
    record: usize,
}

#[derive(Debug)]
struct ModuleRecord {
    qualified: String,
    file: Rc<ParsedFile>,
    items: Vec<Item>,
    macros: Vec<Item>,
    module: Target,
}

impl ModuleRecord {
    fn item(&self, name: &str) -> Option<&Item> {
        self.items.iter().find(|item| item.name == name)
    }
}

/// An imported local: the declaration it names and, for modules, the module.
#[derive(Clone, Debug)]
struct Local {
    target: Option<Target>,
    module: Option<usize>,
}

enum Resolved {
    Found {
        target: Option<Target>,
        module: Option<usize>,
    },
    Missing(String),
    Inaccessible(String),
}

struct UseEntry {
    segments: Vec<String>,
    alias: String,
    node: NodeId,
    wildcard: bool,
}

struct Crate<'c, 'a> {
    context: &'c mut ProjectContext<'a>,
    records: Vec<ModuleRecord>,
    modules: BTreeMap<String, usize>,
}

impl Crate<'_, '_> {
    /// Loads a module file and, recursively, the modules it declares.
    fn load_file(&mut self, qualified: &str, file: &str, root: bool) -> Option<usize> {
        let loaded = self.context.load(file)?;
        let tree = &loaded.tree;
        let container = tree
            .roots
            .iter()
            .copied()
            .find(|root| tree.term(*root) == "source_file")
            .or_else(|| tree.roots.first().copied());
        let stem = file[dirname(file).len()..].trim_start_matches('/');
        let stem = stem.strip_suffix(".rs").unwrap_or(stem);
        let directory = if root || stem == "mod" {
            dirname(file).to_string()
        } else {
            join_path(dirname(file), stem)
        };
        let module = declaration(file, qualified, "module", &[], (0, loaded.source.len()));
        Some(self.load_module(qualified, &loaded, container, &directory, module))
    }

    fn load_module(
        &mut self,
        qualified: &str,
        file: &Rc<ParsedFile>,
        container: Option<NodeId>,
        directory: &str,
        module: Target,
    ) -> usize {
        let record = self.records.len();
        self.records.push(ModuleRecord {
            qualified: qualified.to_string(),
            file: Rc::clone(file),
            items: Vec::new(),
            macros: Vec::new(),
            module,
        });
        self.modules.insert(qualified.to_string(), record);
        let tree = &file.tree;
        let mut attributes = Vec::new();
        let children = container.map_or(&[][..], |container| tree.children(container));
        for child in children {
            let term = tree.term(*child);
            if term == "attribute_item" {
                let attribute = tree.first_child(*child, &["attribute"]).unwrap_or(*child);
                if let Some(name) = tree.first_child(attribute, &["identifier"]) {
                    attributes.push(tree.text(name).to_string());
                }
                continue;
            }
            if term == "line_comment" || term == "block_comment" {
                continue;
            }
            let item = self.item(record, *child, directory, std::mem::take(&mut attributes));
            if let Some(item) = item {
                let items = &mut self.records[record].items;
                match items.iter_mut().find(|existing| existing.name == item.name) {
                    Some(existing) => *existing = item,
                    None => items.push(item),
                }
            }
        }
        record
    }

    fn item(
        &mut self,
        record: usize,
        node: NodeId,
        directory: &str,
        attributes: Vec<String>,
    ) -> Option<Item> {
        let file = Rc::clone(&self.records[record].file);
        let qualified = self.records[record].qualified.clone();
        let tree = &file.tree;
        let (kind, name_term) = item_kind(tree.term(node))?;
        let name_node = tree.first_child(node, &[name_term])?;
        let name = tree.text(name_node).to_string();
        let mut item = Item {
            name: name.clone(),
            kind,
            public: tree.first_child(node, &["visibility_modifier"]).is_some(),
            attributes,
            node,
            module: None,
            target: None,
            record,
        };
        if kind == "module" {
            let child = format!("{qualified}::{name}");
            if let Some(body) = tree.first_child(node, &["declaration_list"]) {
                let module = declaration(&file.path, &child, "module", &[], tree.span(node));
                let directory = join_path(directory, &name);
                item.module = Some(self.load_module(&child, &file, Some(body), &directory, module));
            } else {
                let candidates = [
                    join_path(directory, &format!("{name}.rs")),
                    join_path(directory, &format!("{name}/mod.rs")),
                ];
                item.module = candidates
                    .iter()
                    .find(|candidate| self.context.has(candidate))
                    .cloned()
                    .and_then(|path| self.load_file(&child, &path, false));
            }
            item.target = item
                .module
                .map(|module| self.records[module].module.clone());
            return Some(item);
        }
        if kind == "macro" {
            let symbol = format!("{qualified}::{name}!");
            item.target = Some(declaration(
                &file.path,
                &symbol,
                "macro",
                &[],
                tree.span(node),
            ));
            self.records[record].macros.push(item);
            return None;
        }
        let mut traits = Vec::new();
        if let Some(modifiers) = tree.first_child(node, &["function_modifiers"]) {
            if tree.first_child(modifiers, &["async"]).is_some() {
                traits.push("async");
            }
            if tree.first_child(modifiers, &["const"]).is_some() {
                traits.push("const");
            }
        }
        let recursive = tree.first_child(node, &["block"]).is_some_and(|body| {
            tree.descendants(body, &["identifier"])
                .into_iter()
                .any(|leaf| tree.text(leaf) == name)
        });
        if recursive {
            traits.push("recursive");
        }
        let symbol = format!("{qualified}::{name}");
        item.target = Some(declaration(
            &file.path,
            &symbol,
            kind,
            &traits,
            tree.span(name_node),
        ));
        Some(item)
    }

    /// The module an entry node belongs to: the crate root plus enclosing inline modules.
    fn module_of(&self, node: NodeId) -> String {
        let tree = &self.context.entry.tree;
        let mut names = Vec::new();
        let mut current = tree.parent(node);
        while let Some(id) = current {
            if tree.term(id) == "declaration_list" && tree.parent_term(id) == Some("mod_item") {
                let module = tree
                    .parent(id)
                    .and_then(|module| tree.first_child(module, &["identifier"]));
                names.push(module.map_or("", |name| tree.text(name)));
            }
            current = tree.parent(id);
        }
        names.push("crate");
        names.reverse();
        names.join("::")
    }

    /// Resolves a path from a module. Returns `None` for paths outside the
    /// crate (other crates, the prelude), the accessible item, or the symbol
    /// that is missing or inaccessible.
    fn resolve(
        &self,
        segments: &[String],
        from: &str,
        locals: &BTreeMap<String, Local>,
    ) -> Option<Resolved> {
        let mut module = self.modules.get(from).copied();
        let mut index = 0;
        let head = segments.first().map(String::as_str);
        if head == Some("crate") {
            module = self.modules.get("crate").copied();
            index = 1;
        } else if head == Some("self") {
            index = 1;
        } else if head == Some("super") {
            while segments.get(index).map(String::as_str) == Some("super") {
                module = module.and_then(|module| {
                    let qualified = &self.records[module].qualified;
                    let parent = qualified.rfind("::").map_or("", |at| &qualified[..at]);
                    self.modules.get(parent).copied()
                });
                index += 1;
            }
        } else if !module.is_some_and(|module| {
            head.is_some_and(|head| self.records[module].item(head).is_some())
        }) {
            module = Some(locals.get(head?)?.module?);
            index = 1;
        }
        let mut module = module?;
        if index == segments.len() {
            return Some(Resolved::Found {
                target: Some(self.records[module].module.clone()),
                module: Some(module),
            });
        }
        while index < segments.len() {
            let record = &self.records[module];
            let symbol = format!(
                "{}#{}::{}",
                record.file.path, record.qualified, segments[index]
            );
            let Some(item) = record.item(&segments[index]) else {
                return Some(Resolved::Missing(symbol));
            };
            let inside =
                from == record.qualified || from.starts_with(&format!("{}::", record.qualified));
            if !item.public && !inside {
                return Some(Resolved::Inaccessible(symbol));
            }
            if index == segments.len() - 1 {
                return Some(Resolved::Found {
                    target: item.target.clone(),
                    module: item.module,
                });
            }
            let Some(next) = item.module else {
                if item.kind == "module" {
                    return None;
                }
                return Some(Resolved::Missing(format!(
                    "{symbol}::{}",
                    segments[index + 1]
                )));
            };
            module = next;
            index += 1;
        }
        None
    }

    /// Flattens a use tree into imported locals and links each to its declaration.
    fn use_declaration(&mut self, node: NodeId, locals: &mut BTreeMap<String, Local>) {
        let entry = Rc::clone(&self.context.entry);
        let tree = &entry.tree;
        let from = self.module_of(node);
        let first = tree.child(node, 1);
        if let Some(first) = first {
            let head = tree.leaves_of(first).first().copied();
            if let Some(head) = head.filter(|head| PATH_SEGMENTS.contains(&tree.term(*head))) {
                let name = tree.text(head);
                let segments = [name.to_string()];
                if let Some(Resolved::Found {
                    module: Some(module),
                    ..
                }) = self.resolve(&segments, &from, &BTreeMap::new())
                {
                    let file = self.records[module].file.path.clone();
                    self.context.module(name, &file, &file, tree.span(node));
                }
            }
        }
        for entry in Self::use_tree(tree, first, &[]) {
            let range = tree.span(entry.node);
            match self.resolve(&entry.segments, &from, &BTreeMap::new()) {
                None => {}
                Some(Resolved::Missing(symbol)) => {
                    self.context
                        .diagnose("missing-project-symbol", symbol, range);
                }
                Some(Resolved::Inaccessible(symbol)) => {
                    self.context
                        .diagnose("inaccessible-project-symbol", symbol, range);
                }
                Some(Resolved::Found { target, module }) => {
                    if let Some(target) = &target {
                        self.context
                            .reference("import", tree.text(entry.node), range, target);
                    }
                    if entry.wildcard {
                        let items =
                            module.map_or(&[][..], |module| &self.records[module].items[..]);
                        for item in items
                            .iter()
                            .filter(|item| item.public && item.target.is_some())
                        {
                            locals.insert(
                                item.name.clone(),
                                Local {
                                    target: item.target.clone(),
                                    module: item.module,
                                },
                            );
                        }
                    } else {
                        locals.insert(entry.alias, Local { target, module });
                    }
                }
            }
        }
    }

    fn use_tree(tree: &SyntaxTree, node: Option<NodeId>, prefix: &[String]) -> Vec<UseEntry> {
        let Some(node) = node else {
            return Vec::new();
        };
        let joined = |path: Vec<String>| [prefix, &path[..]].concat();
        let path_of = |child: Option<NodeId>, skip: &str| {
            child
                .filter(|child| tree.term(*child) != skip)
                .map_or_else(Vec::new, |child| path_segments(tree, child))
        };
        match tree.term(node) {
            "use_list" => tree
                .children(node)
                .iter()
                .flat_map(|child| Self::use_tree(tree, Some(*child), prefix))
                .collect(),
            "scoped_use_list" => {
                let path = joined(path_of(tree.child(node, 0), "use_list"));
                Self::use_tree(tree, tree.first_child(node, &["use_list"]), &path)
            }
            "use_as_clause" => {
                let alias = tree.last_child(node).map_or("", |alias| tree.text(alias));
                vec![UseEntry {
                    segments: joined(path_of(tree.child(node, 0), "")),
                    alias: alias.to_string(),
                    node,
                    wildcard: false,
                }]
            }
            "use_wildcard" => vec![UseEntry {
                segments: joined(path_of(tree.child(node, 0), "*")),
                alias: "*".to_string(),
                node,
                wildcard: true,
            }],
            "self" if !prefix.is_empty() => vec![UseEntry {
                segments: prefix.to_vec(),
                alias: prefix[prefix.len() - 1].clone(),
                node,
                wildcard: false,
            }],
            term if PATH_SEGMENTS.contains(&term) || term == "scoped_identifier" => {
                let segments = joined(path_segments(tree, node));
                let alias = segments.last().cloned().unwrap_or_default();
                vec![UseEntry {
                    segments,
                    alias,
                    node,
                    wildcard: false,
                }]
            }
            _ => Vec::new(),
        }
    }

    /// `#[macro_use] mod name;` brings the module's macros into textual scope.
    fn attribute(&mut self, node: NodeId) {
        let entry = Rc::clone(&self.context.entry);
        let tree = &entry.tree;
        let attribute = tree.first_child(node, &["attribute"]).unwrap_or(node);
        let Some(name) = tree.first_child(attribute, &["identifier"]) else {
            return;
        };
        if tree.text(name) != "macro_use" {
            return;
        }
        let next = tree.parent(node).and_then(|parent| {
            let children = tree.children(parent);
            let index = children.iter().position(|child| *child == node)?;
            children.get(index + 1).copied()
        });
        let Some(next) = next.filter(|next| tree.term(*next) == "mod_item") else {
            return;
        };
        let module = tree
            .first_child(next, &["identifier"])
            .map_or("", |name| tree.text(name));
        let qualified = format!("{}::{module}", self.module_of(node));
        if let Some(record) = self.modules.get(&qualified) {
            let target = self.records[*record].module.clone();
            self.context
                .reference("attribute", "macro_use", tree.span(node), &target);
        }
    }

    /// The macros in textual scope at an entry node.
    fn visible_macros(&self, node: NodeId) -> Vec<&Item> {
        fn collect<'r>(records: &'r [ModuleRecord], module: usize, macros: &mut Vec<&'r Item>) {
            macros.extend(records[module].macros.iter());
            for item in &records[module].items {
                if item.kind == "module"
                    && item.attributes.iter().any(|name| name == "macro_use")
                    && let Some(child) = item.module
                {
                    collect(records, child, macros);
                }
            }
        }
        let mut macros = Vec::new();
        if let Some(module) = self.modules.get(&self.module_of(node)) {
            collect(&self.records, *module, &mut macros);
        }
        let entry = &self.context.entry;
        let start = entry.tree.span(node).0;
        macros.retain(|item| {
            item.target
                .as_ref()
                .is_some_and(|target| target.file != entry.path)
                || self.records[item.record].file.tree.span(item.node).1 <= start
        });
        macros
    }

    fn macro_invocation(&mut self, node: NodeId) {
        let entry = Rc::clone(&self.context.entry);
        let tree = &entry.tree;
        let Some(name) = tree
            .child(node, 0)
            .filter(|name| tree.term(*name) == "identifier")
        else {
            return;
        };
        let Some(item) = self
            .visible_macros(node)
            .into_iter()
            .find(|candidate| candidate.name == tree.text(name))
            .cloned()
        else {
            return;
        };
        let Some(target) = item.target.as_ref() else {
            return;
        };
        self.context
            .reference("macro", &item.name, tree.span(node), target);
        let definition = Rc::clone(&self.records[item.record].file);
        let expansion = tree
            .first_child(node, &["token_tree"])
            .and_then(|argument| expand_macro(&definition.tree, item.node, tree, argument));
        if let Some(expansion) = expansion {
            self.context.expansion(
                &item.name,
                "macro-rules",
                tree.span(node),
                expansion,
                &target.symbol,
            );
        }
    }
}

/// The children of a delimited node without its delimiters.
fn inner(tree: &SyntaxTree, node: NodeId) -> &[NodeId] {
    let children = tree.children(node);
    if children.len() < 2 {
        return &[];
    }
    &children[1..children.len() - 1]
}

/// Expands a `macro_rules!` invocation by its first rule whose pattern
/// matches: literal tokens must match, and each `$name:fragment` binds the
/// tokens up to the next literal. Expression fragments of more than one token
/// keep their grouping in parentheses. Returns `None` when no rule applies.
fn expand_macro(
    definition_tree: &SyntaxTree,
    definition: NodeId,
    tree: &SyntaxTree,
    argument: NodeId,
) -> Option<String> {
    let tokens = inner(tree, argument);
    for rule in definition_tree.children_of(definition, &["macro_rule"]) {
        let (Some(pattern), Some(transcriber)) = (
            definition_tree.first_child(rule, &["token_tree_pattern"]),
            definition_tree.first_child(rule, &["token_tree"]),
        ) else {
            continue;
        };
        let Some(bindings) = match_pattern(
            definition_tree,
            inner(definition_tree, pattern),
            tree,
            tokens,
        ) else {
            continue;
        };
        let body = inner(definition_tree, transcriber);
        let (Some(first), Some(last)) = (body.first(), body.last()) else {
            return Some(String::new());
        };
        let source = &definition_tree.source;
        let mut text = String::new();
        let mut cursor = definition_tree.span(*first).0;
        for variable in definition_tree.descendants(transcriber, &["metavariable"]) {
            let binding = bindings.get(definition_tree.text(variable))?;
            let (start, end) = definition_tree.span(variable);
            text.push_str(&source[cursor..start]);
            text.push_str(binding);
            cursor = end;
        }
        text.push_str(&source[cursor..definition_tree.span(*last).1]);
        return Some(collapse_whitespace(&text));
    }
    None
}

fn match_pattern(
    definition_tree: &SyntaxTree,
    pattern: &[NodeId],
    tree: &SyntaxTree,
    tokens: &[NodeId],
) -> Option<BTreeMap<String, String>> {
    let mut bindings = BTreeMap::new();
    let mut position = 0;
    for (index, element) in pattern.iter().enumerate() {
        match definition_tree.term(*element) {
            "token_repetition_pattern" => return None,
            "token_binding_pattern" => {}
            _ => {
                if position >= tokens.len()
                    || tree.text(tokens[position]) != definition_tree.text(*element)
                {
                    return None;
                }
                position += 1;
                continue;
            }
        }
        let stop = pattern
            .get(index + 1)
            .map(|next| definition_tree.text(*next));
        let from = position;
        while position < tokens.len() && Some(tree.text(tokens[position])) != stop {
            position += 1;
        }
        if position == from {
            return None;
        }
        let text = &tree.source[tree.span(tokens[from]).0..tree.span(tokens[position - 1]).1];
        let fragment = definition_tree
            .first_child(*element, &["fragment_specifier"])
            .unwrap_or(*element);
        let grouped = if definition_tree.text(fragment) == "expr" && position - from > 1 {
            format!("({text})")
        } else {
            text.to_string()
        };
        let variable = definition_tree
            .first_child(*element, &["metavariable"])
            .map_or("", |variable| definition_tree.text(variable));
        bindings.insert(variable.to_string(), grouped);
    }
    (position == tokens.len()).then_some(bindings)
}
