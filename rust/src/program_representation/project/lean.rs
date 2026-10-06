//! Project-aware Lean semantics: the Lake manifest (lakefile.toml) and the
//! toolchain pin, `import A.B` resolved to `A/B.lean` within a declared
//! library, declarations qualified by their namespaces, `open` namespaces,
//! attributes, tactic references, and prefix notations expanded to their
//! definition. The JavaScript runtime implements the same analysis in
//! js/src/program-project-lean.js.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use super::tree::{NodeId, SyntaxTree, Target, declaration, toml_entries};
use super::{ParsedFile, ProjectContext};

const DEFINITION_KEYWORDS: [&str; 7] = [
    "def", "theorem", "lemma", "abbrev", "instance", "opaque", "axiom",
];
const DOTTED: [&str; 2] = ["identifier", "."];
const NOT_REFERENCES: [&str; 3] = ["import", "attributes", "notation"];

struct Name {
    text: String,
    start: usize,
    end: usize,
}

struct Open {
    name: String,
    start: usize,
    end: usize,
}

struct Notation {
    atom: String,
    variables: Vec<String>,
    namespace: String,
    visibility: String,
    target: Target,
    file: Rc<ParsedFile>,
    body: NodeId,
}

impl Notation {
    fn expand(&self, arguments: &[String]) -> String {
        let tree = &self.file.tree;
        let (start, end) = tree.span(self.body);
        let mut text = String::new();
        let mut cursor = start;
        for leaf in tree.leaves_of(self.body) {
            let index = (tree.term(leaf) == "identifier")
                .then(|| {
                    self.variables
                        .iter()
                        .position(|name| name == tree.text(leaf))
                })
                .flatten();
            let Some(index) = index else {
                continue;
            };
            let (leaf_start, leaf_end) = tree.span(leaf);
            text.push_str(&tree.source[cursor..leaf_start]);
            text.push_str(arguments.get(index).map_or("undefined", String::as_str));
            cursor = leaf_end;
        }
        text.push_str(&tree.source[cursor..end]);
        text
    }
}

pub(super) fn analyze(context: &mut ProjectContext<'_>) {
    let libraries = read_lake_manifest(context);
    let entry = Rc::clone(&context.entry);
    let tree = &entry.tree;
    let mut environment = Environment {
        context,
        declarations: BTreeMap::new(),
        namespaces: BTreeMap::new(),
        notations: Vec::new(),
        loaded: BTreeSet::new(),
    };
    let body = module_node(tree);
    let children =
        |terms: &[&str]| body.map_or_else(Vec::new, |body| tree.children_of(body, terms));
    for node in children(&["import"]) {
        for name in import_names(tree, node) {
            match environment.import_file(&name, libraries.as_ref()) {
                Import::Absent => {}
                Import::OutsideLibraries => {
                    let library = name.split('.').next().unwrap_or_default().to_string();
                    environment.context.diagnose(
                        "missing-project-library",
                        library,
                        tree.span(node),
                    );
                }
                Import::Found(path) => {
                    environment
                        .context
                        .module(&name, &path, &path, tree.span(node));
                }
            }
        }
    }
    let mut opens = Vec::new();
    let open_nodes = body.map_or_else(Vec::new, |body| tree.descendants(body, &["open"]));
    for node in open_nodes {
        if tree.children(node).is_empty() {
            continue;
        }
        let scoped = tree.first_child(node, &["in"]).is_some();
        for name in dotted_names(tree, node) {
            let Some(namespace) = environment.namespaces.get(&name.text) else {
                continue;
            };
            environment
                .context
                .reference("import", &name.text, (name.start, name.end), namespace);
            let (start, end) = tree.span(node);
            opens.push(Open {
                name: name.text,
                start: if scoped { start } else { end },
                end: if scoped { end } else { tree.source.len() },
            });
        }
    }
    let Environment {
        context,
        declarations,
        notations,
        ..
    } = environment;
    let resolve = |name: &str, position: usize| {
        std::iter::once(name.to_string())
            .chain(
                opens
                    .iter()
                    .filter(|open| open.start <= position && position <= open.end)
                    .map(|open| format!("{}.{name}", open.name)),
            )
            .find_map(|candidate| declarations.get(&candidate))
    };
    let role = |node: NodeId| {
        if tree.ancestor(node, &["by"]).is_some() {
            "tactic"
        } else {
            "reference"
        }
    };
    let notation_at = |node: NodeId| {
        let parent = tree.parent(node)?;
        if tree.term(parent) != "application" || !tree.is_first_child(node) {
            return None;
        }
        let name = tree.text(node);
        let start = tree.span(node).0;
        notations.iter().find(|notation| {
            notation.atom == name
                && notation.visibility != "local"
                && (notation.visibility != "scoped"
                    || opens.iter().any(|open| {
                        open.name == notation.namespace && open.start <= start && start <= open.end
                    }))
                && tree.children(parent).len() - 1 == notation.variables.len()
        })
    };
    let unresolved = context.unresolved;
    for fact in unresolved {
        let (start, end) = (fact.range.start(), fact.range.end());
        let Some(leaf) = tree.leaf_at(start, end) else {
            continue;
        };
        if tree.parent_term(leaf) == Some("projection")
            || tree.ancestor(leaf, &NOT_REFERENCES).is_some()
            || tree.parent_term(leaf) == Some("open")
            || tree
                .parent(leaf)
                .is_some_and(|parent| is_attribute_command(tree, parent))
            || notation_at(leaf).is_some()
        {
            continue;
        }
        if let Some(target) = resolve(&fact.name, start) {
            context.reference(role(leaf), &fact.name, (start, end), target);
        }
    }
    for node in tree.nodes_with(&["projection"]) {
        if tree.parent_term(node) == Some("projection")
            || !tree
                .leaves_of(node)
                .into_iter()
                .all(|leaf| DOTTED.contains(&tree.term(leaf)))
            || tree.ancestor(node, &NOT_REFERENCES).is_some()
        {
            continue;
        }
        let (start, end) = tree.span(node);
        if let Some(target) = resolve(tree.text(node), start) {
            context.reference(role(node), tree.text(node), (start, end), target);
        }
    }
    let commands = (0..tree.nodes.len()).filter(|node| is_attribute_command(tree, *node));
    for node in commands {
        let children = tree.children(node);
        let close = children.iter().position(|child| tree.term(*child) == "]");
        // Without `]`, as `Array.prototype.slice(0, -1)` and `slice(0)` do.
        let (before, after) = close.map_or_else(
            || (&children[..children.len().saturating_sub(1)], children),
            |close| (&children[..close], &children[close + 1..]),
        );
        let identifiers = |nodes: &[NodeId]| {
            nodes
                .iter()
                .copied()
                .filter(|child| tree.term(*child) == "identifier")
                .collect::<Vec<_>>()
        };
        let names = identifiers(before)
            .into_iter()
            .map(|leaf| tree.text(leaf))
            .collect::<Vec<_>>()
            .join(" ");
        for leaf in identifiers(after) {
            if let Some(target) = resolve(tree.text(leaf), tree.span(leaf).0) {
                context.reference("attribute", &names, tree.span(node), target);
            }
        }
    }
    for leaf in tree
        .leaves
        .iter()
        .copied()
        .filter(|leaf| tree.term(*leaf) == "identifier")
    {
        let Some(notation) = notation_at(leaf) else {
            continue;
        };
        let Some(application) = tree.parent(leaf) else {
            continue;
        };
        let range = tree.span(application);
        context.reference("notation", &notation.atom, range, &notation.target);
        let arguments = tree.children(application)[1..]
            .iter()
            .map(|argument| {
                let text = tree.text(*argument);
                if tree.leaves_of(*argument).len() > 1 && tree.term(*argument) != "parenthesized" {
                    format!("({text})")
                } else {
                    text.to_string()
                }
            })
            .collect::<Vec<_>>();
        let expansion = notation.expand(&arguments);
        context.expansion(
            &notation.atom,
            "notation",
            range,
            expansion,
            &notation.target.symbol,
        );
    }
}

fn is_attribute_command(tree: &SyntaxTree, node: NodeId) -> bool {
    tree.term(node) == "attribute"
        && tree
            .child(node, 0)
            .is_some_and(|child| tree.term(child) == "attribute")
}

fn module_node(tree: &SyntaxTree) -> Option<NodeId> {
    let file = *tree.roots.first()?;
    Some(tree.first_child(file, &["module"]).unwrap_or(file))
}

/// `import A.B C` names the modules A.B and C.
fn import_names(tree: &SyntaxTree, node: NodeId) -> Vec<String> {
    dotted_names(tree, node)
        .into_iter()
        .map(|name| name.text)
        .collect()
}

/// The dotted names written directly in a command, before an `in` body.
fn dotted_names(tree: &SyntaxTree, node: NodeId) -> Vec<Name> {
    let mut names: Vec<Name> = Vec::new();
    for child in tree.children(node).iter().skip(1) {
        if !DOTTED.contains(&tree.term(*child)) {
            break;
        }
        let (start, end) = tree.span(*child);
        match names.last_mut() {
            Some(last) if last.end == start => {
                last.end = end;
                last.text = tree.source[last.start..last.end].to_string();
            }
            _ => names.push(Name {
                text: tree.text(*child).to_string(),
                start,
                end,
            }),
        }
    }
    names
}

/// lakefile.toml: the package name, its libraries and required packages, and
/// the lean-toolchain pin.
fn read_lake_manifest(context: &mut ProjectContext<'_>) -> Option<BTreeSet<String>> {
    let source = context.text("lakefile.toml").map(str::to_string);
    if let Some(toolchain) = context.text("lean-toolchain").map(str::to_string) {
        let trimmed = toolchain.trim();
        let start = toolchain.find(trimmed).unwrap_or_default();
        context.fact(
            "project-toolchain",
            trimmed,
            "lean-toolchain",
            (start, start + trimmed.len()),
        );
    }
    let source = source?;
    context.fact(
        "project-manifest",
        "lakefile.toml",
        "lakefile.toml",
        (0, source.len()),
    );
    let mut libraries = BTreeSet::new();
    for entry in toml_entries(&source) {
        let range = (entry.start, entry.end);
        if entry.key != "name" {
            continue;
        }
        match entry.table.as_str() {
            "" => context.fact("project-package", &entry.value, "lakefile.toml", range),
            "lean_lib" => {
                context.fact("project-library", &entry.value, "lakefile.toml", range);
                libraries.insert(entry.value);
            }
            "require" => context.fact("project-dependency", &entry.value, "lakefile.toml", range),
            _ => {}
        }
    }
    Some(libraries)
}

enum Import {
    /// The project has no such file.
    Absent,
    /// The file is outside every declared library.
    OutsideLibraries,
    Found(String),
}

struct Environment<'c, 'a> {
    context: &'c mut ProjectContext<'a>,
    declarations: BTreeMap<String, Target>,
    namespaces: BTreeMap<String, Target>,
    notations: Vec<Notation>,
    loaded: BTreeSet<String>,
}

impl Environment<'_, '_> {
    /// The file of an imported module, loading its declarations and imports.
    fn import_file(&mut self, name: &str, libraries: Option<&BTreeSet<String>>) -> Import {
        let path = format!("{}.lean", name.split('.').collect::<Vec<_>>().join("/"));
        if !self.context.has(&path) || path == self.context.entry.path {
            return Import::Absent;
        }
        let library = name.split('.').next().unwrap_or_default();
        if libraries.is_some_and(|libraries| !libraries.contains(library)) {
            return Import::OutsideLibraries;
        }
        if !self.loaded.insert(path.clone()) {
            return Import::Found(path);
        }
        let Some(file) = self.context.load(&path) else {
            return Import::Absent;
        };
        let tree = &file.tree;
        let body = module_node(tree);
        if let Some(body) = body {
            for node in tree.children_of(body, &["import"]) {
                for imported in import_names(tree, node) {
                    self.import_file(&imported, libraries);
                }
            }
            self.declare(&path, &file, body);
        }
        Import::Found(path)
    }

    fn declare(&mut self, path: &str, file: &Rc<ParsedFile>, body: NodeId) {
        let tree = &file.tree;
        let mut scopes: Vec<String> = Vec::new();
        let mut universes = BTreeSet::new();
        let prefix = |scopes: &[String]| {
            scopes
                .iter()
                .filter(|scope| !scope.is_empty())
                .cloned()
                .collect::<Vec<_>>()
                .join(".")
        };
        let mut visibility = "global";
        for child in tree.children(body).iter().copied() {
            let term = tree.term(child);
            let has_children = !tree.children(child).is_empty();
            if term == "scoped" || term == "local" {
                visibility = if term == "scoped" { "scoped" } else { "local" };
                continue;
            }
            match term {
                "namespace" if has_children => {
                    for name in dotted_names(tree, child) {
                        scopes.push(name.text);
                        let namespace = prefix(&scopes);
                        self.namespaces
                            .entry(namespace)
                            .or_insert_with_key(|namespace| {
                                declaration(path, namespace, "namespace", &[], tree.span(child))
                            });
                    }
                }
                "section" if has_children => scopes.push(String::new()),
                "end" if has_children => {
                    scopes.pop();
                }
                "universe" if has_children => {
                    for name in tree.children_of(child, &["identifier"]) {
                        universes.insert(tree.text(name).to_string());
                    }
                }
                "declaration" => {
                    let inner = tree.first_child(child, &["definition", "structure", "inductive"]);
                    if let Some(inner) = inner {
                        self.definition(path, tree, inner, &prefix(&scopes), &universes);
                    }
                }
                "definition" | "structure" | "inductive" => {
                    self.definition(path, tree, child, &prefix(&scopes), &universes);
                }
                "notation" if has_children => {
                    self.notation(path, file, child, prefix(&scopes), visibility);
                }
                _ => {}
            }
            visibility = "global";
        }
    }

    fn definition(
        &mut self,
        path: &str,
        tree: &SyntaxTree,
        node: NodeId,
        prefix: &str,
        universes: &BTreeSet<String>,
    ) {
        let term = tree.term(node);
        let kind = if term == "definition" {
            tree.child(node, 0).map_or("", |keyword| tree.text(keyword))
        } else {
            term
        };
        if term == "definition" && !DEFINITION_KEYWORDS.contains(&kind) {
            return;
        }
        let Some(name) = dotted_names(tree, node).into_iter().next() else {
            return;
        };
        if tree.child(node, 1).map(|child| tree.span(child).0) != Some(name.start) {
            return;
        }
        let qualified = if prefix.is_empty() {
            name.text.clone()
        } else {
            format!("{prefix}.{}", name.text)
        };
        let mut traits = Vec::new();
        let short = name.text.rsplit('.').next().unwrap_or_default();
        let recursive = tree.leaves_of(node).into_iter().any(|leaf| {
            tree.span(leaf).0 >= name.end
                && tree.term(leaf) == "identifier"
                && [name.text.as_str(), short, qualified.as_str()].contains(&tree.text(leaf))
        });
        if recursive {
            traits.push("recursive");
        }
        let children = tree.children(node);
        let assign = children
            .iter()
            .position(|child| matches!(tree.term(*child), ":=" | "where"))
            .unwrap_or(children.len());
        let header = children[..assign]
            .iter()
            .flat_map(|child| tree.leaves_of(*child))
            .filter(|leaf| tree.term(*leaf) == "identifier")
            .map(|leaf| tree.text(leaf))
            .collect::<Vec<_>>();
        if header.contains(&"IO") {
            traits.push("io");
        }
        if header.iter().any(|leaf| universes.contains(*leaf)) {
            traits.push("universe-polymorphic");
        }
        let target = declaration(path, &qualified, kind, &traits, (name.start, name.end));
        self.declarations.insert(qualified.clone(), target);
        for constructor in tree.children_of(node, &["constructor"]) {
            let Some(constructor_name) = tree.first_child(constructor, &["identifier"]) else {
                continue;
            };
            let constructor_qualified = format!("{qualified}.{}", tree.text(constructor_name));
            let target = declaration(
                path,
                &constructor_qualified,
                "constructor",
                &[],
                tree.span(constructor_name),
            );
            self.declarations.insert(constructor_qualified, target);
        }
    }

    /// `notation "atom " x y => body`: a prefix notation with its variables.
    fn notation(
        &mut self,
        path: &str,
        file: &Rc<ParsedFile>,
        node: NodeId,
        namespace: String,
        visibility: &str,
    ) {
        let tree = &file.tree;
        let children = tree.children(node);
        let Some(arrow) = children.iter().position(|child| tree.term(*child) == "=>") else {
            return;
        };
        if arrow < 1 || arrow + 1 >= children.len() {
            return;
        }
        let elements = children[1..arrow]
            .iter()
            .copied()
            .filter(|child| !matches!(tree.term(*child), ":" | "number"))
            .flat_map(|element| flatten_application(tree, element))
            .collect::<Vec<_>>();
        let Some((first, variables)) = elements.split_first() else {
            return;
        };
        if tree.term(*first) != "string"
            || variables
                .iter()
                .any(|variable| tree.term(*variable) != "identifier")
        {
            return;
        }
        let quoted = tree.text(*first);
        let atom = quoted
            .get(1..quoted.len().saturating_sub(1))
            .unwrap_or_default()
            .trim()
            .to_string();
        let symbol = if namespace.is_empty() {
            format!("«{atom}»")
        } else {
            format!("{namespace}.«{atom}»")
        };
        let target = declaration(
            path,
            &format!("notation:{symbol}"),
            "notation",
            &[],
            tree.span(node),
        );
        self.notations.push(Notation {
            atom,
            variables: variables
                .iter()
                .map(|variable| tree.text(*variable).to_string())
                .collect(),
            namespace,
            visibility: visibility.to_string(),
            target,
            file: Rc::clone(file),
            body: children[arrow + 1],
        });
    }
}

fn flatten_application(tree: &SyntaxTree, node: NodeId) -> Vec<NodeId> {
    if tree.term(node) == "application" {
        tree.children(node)
            .iter()
            .flat_map(|child| flatten_application(tree, *child))
            .collect()
    } else {
        vec![node]
    }
}
