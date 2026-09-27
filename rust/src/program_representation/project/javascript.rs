//! Project-aware JavaScript semantics: package.json dialect, name and import
//! map; ECMAScript module requests resolved to project files; named,
//! namespace and default imports, JSON modules with their import attributes,
//! tagged templates expanded into the call they denote, and project
//! references checked by `node:assert` assertions (runtime-checked
//! propositions: JavaScript has no proof terms). The JavaScript runtime
//! implements the same analysis in js/src/program-project-javascript.js.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use super::tree::{declaration, dirname, join_path, json_string, NodeId, SyntaxTree, Target};
use super::ProjectContext;

const ASSERT_MODULES: [&str; 4] = [
    "node:assert",
    "node:assert/strict",
    "assert",
    "assert/strict",
];

type Exports = Rc<BTreeMap<String, Target>>;

struct Local {
    path: String,
    name: String,
    exports: Exports,
}

#[derive(Default)]
struct Manifest {
    imports: BTreeMap<String, String>,
}

pub(super) fn analyze(context: &mut ProjectContext<'_>) {
    let manifest = read_package_manifest(context);
    let entry = Rc::clone(&context.entry);
    let tree = &entry.tree;
    let mut locals: BTreeMap<String, Local> = BTreeMap::new();
    let mut assertions: BTreeSet<String> = BTreeSet::new();
    let statements = tree
        .roots
        .iter()
        .flat_map(|root| tree.children_of(*root, &["import_statement"]))
        .collect::<Vec<_>>();
    for statement in statements {
        let Some(fragment) = tree
            .first_child(statement, &["string"])
            .and_then(|specifier| tree.first_child(specifier, &["string_fragment"]))
        else {
            continue;
        };
        let request = tree.text(fragment);
        let clause = tree.first_child(statement, &["import_clause"]);
        if ASSERT_MODULES.contains(&request) {
            for name in imported_locals(tree, clause) {
                assertions.insert(tree.text(name).to_string());
            }
            continue;
        }
        let Some(path) = resolve_request(request, &entry.path, &manifest) else {
            continue;
        };
        if !context.has(&path) {
            continue;
        }
        let range = tree.span(statement);
        context.module(request, &path, &path, range);
        let exports = Rc::new(module_exports(context, &path));
        let attribute = tree.first_child(statement, &["import_attribute"]);
        let kind = attribute.and_then(|attribute| import_attribute_type(tree, attribute));
        let json = is_json(&path);
        if json && kind != Some("json") {
            context.diagnose("missing-import-attribute", request.to_string(), range);
        }
        if kind == Some("json") && !json {
            context.diagnose("import-attribute-mismatch", request.to_string(), range);
        }
        if let (Some(attribute), Some("json"), true) = (attribute, kind, json) {
            if let Some(target) = exports.get("default") {
                context.reference("attribute", "type: json", tree.span(attribute), target);
            }
        }
        let Some(clause) = clause else {
            continue;
        };
        for child in tree.children(clause).to_vec() {
            match tree.term(child) {
                "identifier" => {
                    let local = tree.text(child);
                    import_name(
                        context,
                        &exports,
                        "default",
                        child,
                        local,
                        &mut locals,
                        &path,
                    );
                }
                "namespace_import" => {
                    let namespace = Target {
                        symbol: format!("{path}#*"),
                        kind: "namespace".to_string(),
                        traits: Vec::new(),
                        file: path.clone(),
                        start: 0,
                        end: context.text(&path).map_or(0, str::len),
                    };
                    context.reference("import", "*", tree.span(child), &namespace);
                    if let Some(local) = tree.first_child(child, &["identifier"]) {
                        locals.insert(
                            tree.text(local).to_string(),
                            Local {
                                path: path.clone(),
                                name: "*".to_string(),
                                exports: Rc::clone(&exports),
                            },
                        );
                    }
                }
                "named_imports" => {
                    for item in tree.children_of(child, &["import_specifier"]) {
                        let names = tree.children_of(item, &["identifier"]);
                        let (Some(first), Some(last)) = (names.first(), names.last()) else {
                            continue;
                        };
                        let (name, local) = (tree.text(*first), tree.text(*last));
                        import_name(context, &exports, name, item, local, &mut locals, &path);
                    }
                }
                _ => {}
            }
        }
    }
    let bindings = context.bindings;
    for binding in bindings.iter().filter(|binding| binding.kind == "import") {
        let Some(local) = locals.get(&binding.name) else {
            continue;
        };
        for range in &binding.references {
            if let Some(leaf) = tree.leaf_at(range.start(), range.end()) {
                let role = if inside_assertion(tree, leaf, &assertions) {
                    "assertion"
                } else {
                    "reference"
                };
                reference_use(context, tree, local, leaf, role);
            }
        }
    }
}

fn import_name(
    context: &mut ProjectContext<'_>,
    exports: &Exports,
    name: &str,
    node: NodeId,
    local: &str,
    locals: &mut BTreeMap<String, Local>,
    path: &str,
) {
    let range = context.entry.tree.span(node);
    let Some(target) = exports.get(name) else {
        context.diagnose("missing-project-symbol", format!("{path}#{name}"), range);
        return;
    };
    context.reference("import", name, range, target);
    locals.insert(
        local.to_string(),
        Local {
            path: path.to_string(),
            name: name.to_string(),
            exports: Rc::clone(exports),
        },
    );
}

/// One use of an imported local: a namespace or JSON member access, a tagged
/// template, or a plain reference.
fn reference_use(
    context: &mut ProjectContext<'_>,
    tree: &SyntaxTree,
    local: &Local,
    leaf: NodeId,
    role: &'static str,
) {
    let parent = tree.parent(leaf);
    let first = tree.is_first_child(leaf);
    let member = parent
        .filter(|parent| first && tree.term(*parent) == "member_expression")
        .and_then(|parent| tree.first_child(parent, &["property_identifier"]));
    if let (Some(member), Some(parent), true) =
        (member, parent, local.name == "*" || local.name == "default")
    {
        let name = if local.name == "*" {
            tree.text(member).to_string()
        } else {
            format!("default.{}", tree.text(member))
        };
        match local.exports.get(&name) {
            Some(target) => context.reference(role, &name, tree.span(parent), target),
            None => context.diagnose(
                "missing-project-symbol",
                format!("{}#{name}", local.path),
                tree.span(parent),
            ),
        }
        return;
    }
    let Some(target) = local.exports.get(&local.name) else {
        return;
    };
    let template = parent
        .filter(|parent| first && tree.term(*parent) == "call_expression")
        .and_then(|parent| tree.first_child(parent, &["template_string"]));
    if let (Some(template), Some(parent)) = (template, parent) {
        context.reference("template-tag", &local.name, tree.span(parent), target);
        if let Some(expansion) = template_call(tree, leaf, template) {
            context.expansion(
                &local.name,
                "tagged-template",
                tree.span(parent),
                expansion,
                &target.symbol,
            );
        }
        return;
    }
    context.reference(role, &local.name, tree.span(leaf), target);
}

/// The local identifiers an import clause binds.
fn imported_locals(tree: &SyntaxTree, clause: Option<NodeId>) -> Vec<NodeId> {
    let Some(clause) = clause else {
        return Vec::new();
    };
    tree.children(clause)
        .iter()
        .flat_map(|child| match tree.term(*child) {
            "identifier" => vec![*child],
            "namespace_import" => tree.children_of(*child, &["identifier"]),
            "named_imports" => tree
                .children_of(*child, &["import_specifier"])
                .into_iter()
                .filter_map(|item| tree.children_of(item, &["identifier"]).last().copied())
                .collect(),
            _ => Vec::new(),
        })
        .collect()
}

/// Whether a node is an argument of `assert(...)`, `assert.method(...)` for an
/// imported assertion module, or `console.assert(...)`.
fn inside_assertion(tree: &SyntaxTree, node: NodeId, assertions: &BTreeSet<String>) -> bool {
    let mut current = node;
    while let Some(call) = tree.parent(current) {
        if tree.term(call) == "call_expression" && tree.term(current) == "arguments" {
            let callee = tree.child(call, 0);
            let object = callee.and_then(|callee| {
                if tree.term(callee) == "member_expression" {
                    tree.child(callee, 0)
                } else {
                    Some(callee)
                }
            });
            if object.is_some_and(|object| {
                tree.term(object) == "identifier" && assertions.contains(tree.text(object))
            }) {
                return true;
            }
            if callee.is_some_and(|callee| {
                tree.term(callee) == "member_expression" && tree.text(callee) == "console.assert"
            }) {
                return true;
            }
        }
        current = call;
    }
    false
}

/// `` tag`a${x}b` `` calls tag with the frozen strings array (carrying its raw
/// strings) followed by the substituted values.
fn template_call(tree: &SyntaxTree, tag: NodeId, template: NodeId) -> Option<String> {
    let mut strings = Vec::new();
    let mut values = Vec::new();
    let mut current = String::new();
    for child in tree.children(template) {
        match tree.term(*child) {
            "string_fragment" => current.push_str(tree.text(*child)),
            "escape_sequence" => return None,
            "template_substitution" => {
                strings.push(std::mem::take(&mut current));
                let [_, inner, _] = tree.children(*child) else {
                    return None;
                };
                values.push(tree.text(*inner).to_string());
            }
            _ => {}
        }
    }
    strings.push(current);
    let array = format!(
        "[{}]",
        strings
            .iter()
            .map(|value| json_string(value))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let argument =
        format!("Object.freeze(Object.assign({array}, {{ raw: Object.freeze({array}) }}))");
    let arguments = std::iter::once(argument)
        .chain(values)
        .collect::<Vec<_>>()
        .join(", ");
    Some(format!("{}({arguments})", tree.text(tag)))
}

fn import_attribute_type(tree: &SyntaxTree, attribute: NodeId) -> Option<&str> {
    for pair in tree.descendants(attribute, &["pair"]) {
        let key = tree.child(pair, 0);
        let fragment = tree
            .child(pair, 2)
            .filter(|value| tree.term(*value) == "string")
            .and_then(|value| tree.first_child(value, &["string_fragment"]));
        let name =
            key.map(|key| tree.text(tree.first_child(key, &["string_fragment"]).unwrap_or(key)));
        if let (Some(fragment), Some("type")) = (fragment, name) {
            return Some(tree.text(fragment));
        }
    }
    None
}

fn resolve_request(request: &str, entry: &str, manifest: &Manifest) -> Option<String> {
    if request.starts_with("./") || request.starts_with("../") {
        return Some(join_path(dirname(entry), request));
    }
    if request.starts_with('#') {
        return manifest
            .imports
            .get(request)
            .map(|target| join_path("", target));
    }
    None
}

/// package.json: the package name, its module dialect ("type"), the
/// package-internal import map and the declared dependencies.
fn read_package_manifest(context: &mut ProjectContext<'_>) -> Manifest {
    let mut manifest = Manifest::default();
    let Some(file) = context.load_as("package.json", "JSON") else {
        return manifest;
    };
    let tree = &file.tree;
    context.fact(
        "project-manifest",
        "package.json",
        "package.json",
        (0, file.source.len()),
    );
    let members = json_root(tree)
        .map(|root| json_members(tree, root))
        .unwrap_or_default();
    let text =
        |node: NodeId| tree.text(tree.first_child(node, &["string_content"]).unwrap_or(node));
    if let Some(name) = find(&members, "name").filter(|name| tree.term(*name) == "string") {
        context.fact(
            "project-package",
            text(name),
            "package.json",
            content_range(tree, name),
        );
    }
    let kind = find(&members, "type");
    let dialect = kind
        .filter(|kind| tree.term(*kind) == "string")
        .map_or("commonjs", text);
    let range = kind.map_or((0, 0), |kind| content_range(tree, kind));
    context.fact("project-dialect", dialect, "package.json", range);
    if let Some(imports) =
        find(&members, "imports").filter(|imports| tree.term(*imports) == "object")
    {
        for (key, value) in json_members(tree, imports) {
            if tree.term(value) != "string" {
                continue;
            }
            manifest
                .imports
                .insert(key.clone(), text(value).to_string());
            context.fact(
                "project-import-map",
                &key,
                "package.json",
                content_range(tree, value),
            );
        }
    }
    for section in ["dependencies", "devDependencies", "peerDependencies"] {
        let Some(dependencies) =
            find(&members, section).filter(|dependencies| tree.term(*dependencies) == "object")
        else {
            continue;
        };
        for (key, value) in json_members(tree, dependencies) {
            context.fact("project-dependency", &key, "package.json", tree.span(value));
        }
    }
    manifest
}

fn find(members: &[(String, NodeId)], key: &str) -> Option<NodeId> {
    // A later member with the same key replaces an earlier one.
    members
        .iter()
        .rev()
        .find(|(name, _)| name == key)
        .map(|(_, value)| *value)
}

fn content_range(tree: &SyntaxTree, node: NodeId) -> (usize, usize) {
    tree.first_child(node, &["string_content"]).map_or_else(
        || {
            let (start, end) = tree.span(node);
            (start + 1, end.saturating_sub(1))
        },
        |content| tree.span(content),
    )
}

fn json_root(tree: &SyntaxTree) -> Option<NodeId> {
    let document = tree
        .roots
        .iter()
        .copied()
        .find(|root| tree.term(*root) == "document")
        .or_else(|| tree.roots.first().copied())?;
    Some(tree.first_child(document, &["object"]).unwrap_or(document))
}

/// The members of a JSON object, keyed by their decoded-as-written key text,
/// in first-appearance order with the last value of a repeated key.
fn json_members(tree: &SyntaxTree, object: NodeId) -> Vec<(String, NodeId)> {
    let mut members: Vec<(String, NodeId)> = Vec::new();
    for pair in tree.children_of(object, &["pair"]) {
        let (Some(key), Some(value)) = (tree.child(pair, 0), tree.child(pair, 2)) else {
            continue;
        };
        let name = tree
            .first_child(key, &["string_content"])
            .map_or("", |content| tree.text(content))
            .to_string();
        if let Some(member) = members.iter_mut().find(|(existing, _)| *existing == name) {
            member.1 = value;
        } else {
            members.push((name, value));
        }
    }
    members
}

/// Whether a module path names a JSON module; as in Node.js, the extension is
/// compared case-sensitively.
#[allow(clippy::case_sensitive_file_extension_comparisons)]
fn is_json(path: &str) -> bool {
    path.ends_with(".json")
}

/// The exports of a project module: exported declarations of a JavaScript
/// module, or the default value and top-level members of a JSON module.
fn module_exports(context: &mut ProjectContext<'_>, path: &str) -> BTreeMap<String, Target> {
    let mut exports = BTreeMap::new();
    if is_json(path) {
        let Some(file) = context.load_as(path, "JSON") else {
            return exports;
        };
        let tree = &file.tree;
        let root = json_root(tree);
        let span = root.map_or((0, 0), |root| tree.span(root));
        exports.insert(
            "default".to_string(),
            declaration(path, "default", "json-value", &[], span),
        );
        if let Some(root) = root.filter(|root| tree.term(*root) == "object") {
            for pair in tree.children_of(root, &["pair"]) {
                let Some(key) = tree
                    .child(pair, 0)
                    .and_then(|key| tree.first_child(key, &["string_content"]))
                else {
                    continue;
                };
                let name = format!("default.{}", tree.text(key));
                let target = declaration(path, &name, "json-member", &[], tree.span(key));
                exports.insert(name, target);
            }
        }
        return exports;
    }
    let Some(file) = context.load(path) else {
        return exports;
    };
    let tree = &file.tree;
    let statements = tree
        .roots
        .iter()
        .flat_map(|root| tree.children_of(*root, &["export_statement"]))
        .collect::<Vec<_>>();
    for statement in statements {
        let declared = tree.first_child(
            statement,
            &[
                "function_declaration",
                "generator_function_declaration",
                "class_declaration",
                "lexical_declaration",
                "variable_declaration",
            ],
        );
        if tree.first_child(statement, &["default"]).is_some() {
            let span = tree.span(declared.unwrap_or(statement));
            exports.insert(
                "default".to_string(),
                declaration(path, "default", "default", &[], span),
            );
            continue;
        }
        let Some(declared) = declared else {
            continue;
        };
        let term = tree.term(declared);
        if term == "lexical_declaration" || term == "variable_declaration" {
            for declarator in tree.children_of(declared, &["variable_declarator"]) {
                if let Some(name) = tree.first_child(declarator, &["identifier"]) {
                    let text = tree.text(name);
                    let target = declaration(path, text, "variable", &[], tree.span(name));
                    exports.insert(text.to_string(), target);
                }
            }
            continue;
        }
        let Some(name) = tree.first_child(declared, &["identifier"]) else {
            continue;
        };
        let text = tree.text(name);
        let mut traits = Vec::new();
        if tree.first_child(declared, &["async"]).is_some() {
            traits.push("async");
        }
        if term == "generator_function_declaration" {
            traits.push("generator");
        }
        let recursive = tree
            .first_child(declared, &["statement_block", "class_body"])
            .is_some_and(|body| {
                tree.descendants(body, &["identifier"])
                    .into_iter()
                    .any(|node| tree.text(node) == text)
            });
        if recursive {
            traits.push("recursive");
        }
        let kind = if term == "class_declaration" {
            "class"
        } else {
            "function"
        };
        exports.insert(
            text.to_string(),
            declaration(path, text, kind, &traits, tree.span(name)),
        );
    }
    exports
}
