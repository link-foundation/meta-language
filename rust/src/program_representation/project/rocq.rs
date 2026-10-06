//! Project-aware Rocq semantics: the `_CoqProject` load paths (`-Q`/`-R`
//! physical directory to logical prefix) and listed files, `Require Import`
//! resolved through them, declarations qualified by their logical module,
//! hint databases targeted by attributes, Ltac tactics, and notations
//! expanded to their definition. The JavaScript runtime implements the same
//! analysis in js/src/program-project-rocq.js.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use super::tree::{NodeId, SyntaxTree, Target, declaration, split_words};
use super::{ParsedFile, ProjectContext};

const NON_REFERENCE_CONTEXTS: [&str; 3] = ["require_command", "notation_command", "attributes"];

fn declaration_command(term: &str) -> Option<&'static str> {
    Some(match term {
        "definition_command" => "definition",
        "fixpoint_command" => "fixpoint",
        "theorem_command" => "theorem",
        "inductive_command" => "inductive",
        _ => return None,
    })
}

fn proof_keyword(keyword: &str) -> &'static str {
    match keyword {
        "Lemma" => "lemma",
        _ => "theorem",
    }
}

struct LoadPath {
    directory: String,
    prefix: String,
}

struct Declaration {
    module: String,
    qualified: String,
    target: Target,
}

struct Notation {
    module: String,
    atom: String,
    before: bool,
    after: bool,
    variables: Vec<String>,
    target: Target,
    file: Rc<ParsedFile>,
    body: NodeId,
}

impl Notation {
    fn expand(&self, operands: &[String]) -> String {
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
            text.push_str(operands.get(index).map_or("undefined", String::as_str));
            cursor = leaf_end;
        }
        text.push_str(&tree.source[cursor..end]);
        text
    }
}

struct Match {
    start: usize,
    end: usize,
    operands: Vec<String>,
}

pub(super) fn analyze(context: &mut ProjectContext<'_>) {
    let load_paths = read_coq_project(context);
    let entry = Rc::clone(&context.entry);
    let tree = &entry.tree;
    let mut environment = Environment {
        context,
        load_paths,
        declarations: Vec::new(),
        hint_databases: BTreeMap::new(),
        notations: Vec::new(),
        loaded: BTreeSet::new(),
        imported: BTreeSet::new(),
    };
    for node in tree.nodes_with(&["require_command"]) {
        for (name, imported) in require_names(tree, node) {
            if let Some(path) = environment.require(&name, imported) {
                environment
                    .context
                    .module(&name, &path, &path, tree.span(node));
            }
        }
    }
    let role = |node: NodeId| {
        if tree
            .ancestor(node, &["tactic_invocation", "ltac_definition"])
            .is_some()
        {
            "tactic"
        } else {
            "reference"
        }
    };
    let unresolved = environment.context.unresolved;
    for fact in unresolved {
        let (start, end) = (fact.range.start(), fact.range.end());
        let Some(leaf) = tree.leaf_at(start, end) else {
            continue;
        };
        if tree.ancestor(leaf, &NON_REFERENCE_CONTEXTS).is_some() {
            continue;
        }
        let database = tree.ancestor(leaf, &["hint_command"]).and_then(|hint| {
            tree.children(hint)
                .iter()
                .find(|child| tree.term(**child) == ":")
                .copied()
        });
        if database.is_some_and(|colon| start > tree.span(colon).0) {
            continue;
        }
        if let Some(target) = environment.resolve(&fact.name).cloned() {
            environment
                .context
                .reference(role(leaf), &fact.name, (start, end), &target);
        }
    }
    for sentence in tree.nodes_with(&["sentence"]) {
        let (Some(attributes), Some(hint)) = (
            tree.first_child(sentence, &["attributes"]),
            tree.first_child(sentence, &["hint_command"]),
        ) else {
            continue;
        };
        let children = tree.children(hint);
        let colon = children.iter().position(|child| tree.term(*child) == ":");
        let names = tree
            .descendants(attributes, &["identifier"])
            .into_iter()
            .map(|leaf| tree.text(leaf))
            .collect::<Vec<_>>()
            .join(" ");
        let databases = colon.map_or(&[][..], |colon| &children[colon + 1..]);
        for database in databases {
            if let Some(target) = environment.hint_databases.get(tree.text(*database)) {
                environment
                    .context
                    .reference("attribute", &names, tree.span(attributes), target);
            }
        }
    }
    let Environment {
        context,
        notations,
        imported,
        ..
    } = environment;
    for notation in notations
        .iter()
        .filter(|notation| imported.contains(&notation.module))
    {
        for found in notation_matches(tree, notation) {
            let range = (found.start, found.end);
            context.reference("notation", &notation.atom, range, &notation.target);
            let expansion = notation.expand(&found.operands);
            context.expansion(
                &notation.atom,
                "notation",
                range,
                expansion,
                &notation.target.symbol,
            );
        }
    }
}

/// `From P Require Import A B` requires P.A and P.B; `Require Import A.B`
/// requires A.B. Import and Export also make the short names visible.
fn require_names(tree: &SyntaxTree, node: NodeId) -> Vec<(String, bool)> {
    let from = tree
        .child(node, 0)
        .filter(|first| tree.term(*first) == "From")
        .and_then(|_| tree.child(node, 1))
        .map(|name| tree.text(name));
    let Some(mode) = tree
        .children(node)
        .iter()
        .copied()
        .find(|child| matches!(tree.term(*child), "Import" | "Export"))
    else {
        return Vec::new();
    };
    let words = &tree.source[tree.span(mode).1..tree.span(node).1];
    split_words(words)
        .into_iter()
        .map(|name| {
            let name = from.map_or_else(|| name.to_string(), |from| format!("{from}.{name}"));
            (name, true)
        })
        .collect()
}

/// `_CoqProject`: `-Q dir Prefix` and `-R dir Prefix` load paths and listed files.
fn read_coq_project(context: &mut ProjectContext<'_>) -> Vec<LoadPath> {
    let mut load_paths = Vec::new();
    let Some(source) = context.text("_CoqProject").map(str::to_string) else {
        return load_paths;
    };
    context.fact(
        "project-manifest",
        "_CoqProject",
        "_CoqProject",
        (0, source.len()),
    );
    let mut words = Vec::new();
    let mut offset = 0;
    let space = |byte: u8| matches!(byte, b' ' | b'\t' | b'\r');
    for line in source.split('\n') {
        let bytes = line.as_bytes();
        let mut index = 0;
        while index < bytes.len() && bytes[index] != b'#' {
            if space(bytes[index]) {
                index += 1;
                continue;
            }
            let start = index;
            while index < bytes.len() && !space(bytes[index]) {
                index += 1;
            }
            words.push((&line[start..index], offset + start, offset + index));
        }
        offset += line.len() + 1;
    }
    let mut index = 0;
    while index < words.len() {
        let (text, start, end) = words[index];
        if (text == "-Q" || text == "-R") && index + 2 < words.len() {
            let (directory, directory_start, directory_end) = words[index + 1];
            let directory = directory.strip_prefix("./").unwrap_or(directory);
            let directory = directory.strip_suffix('/').unwrap_or(directory);
            let prefix = words[index + 2].0;
            load_paths.push(LoadPath {
                directory: if directory == "." { "" } else { directory }.to_string(),
                prefix: prefix.to_string(),
            });
            context.fact(
                "project-load-path",
                prefix,
                "_CoqProject",
                (directory_start, directory_end),
            );
            index += 2;
        } else if is_vernacular(text) {
            context.fact("project-source", text, "_CoqProject", (start, end));
        }
        index += 1;
    }
    load_paths
}

struct Environment<'c, 'a> {
    context: &'c mut ProjectContext<'a>,
    load_paths: Vec<LoadPath>,
    declarations: Vec<Declaration>,
    hint_databases: BTreeMap<String, Target>,
    notations: Vec<Notation>,
    loaded: BTreeSet<String>,
    imported: BTreeSet<String>,
}

impl Environment<'_, '_> {
    /// The file of a required logical module, found through the load paths.
    fn require(&mut self, name: &str, imported: bool) -> Option<String> {
        let mut found = None;
        for load_path in &self.load_paths {
            let Some(rest) = name.strip_prefix(&format!("{}.", load_path.prefix)) else {
                continue;
            };
            let relative = format!("{}.v", rest.split('.').collect::<Vec<_>>().join("/"));
            let path = if load_path.directory.is_empty() {
                relative
            } else {
                format!("{}/{relative}", load_path.directory)
            };
            if !self.context.has(&path) || path == self.context.entry.path {
                continue;
            }
            found = Some(path);
            break;
        }
        let path = found?;
        if self.loaded.insert(name.to_string())
            && let Some(file) = self.context.load(&path)
        {
            for node in file.tree.nodes_with(&["require_command"]) {
                for (required, _) in require_names(&file.tree, node) {
                    self.require(&required, false);
                }
            }
            self.declare(name, &path, &file);
        }
        if imported {
            self.imported.insert(name.to_string());
        }
        Some(path)
    }

    /// A declaration by qualified name, or by short name from an imported module.
    fn resolve(&self, name: &str) -> Option<&Target> {
        self.declarations
            .iter()
            .find(|item| item.qualified == name)
            .or_else(|| {
                self.declarations.iter().find(|item| {
                    self.imported.contains(&item.module)
                        && item.qualified == format!("{}.{name}", item.module)
                })
            })
            .map(|item| &item.target)
    }

    fn declare(&mut self, module: &str, path: &str, file: &Rc<ParsedFile>) {
        let tree = &file.tree;
        let add = |declarations: &mut Vec<Declaration>,
                   name: &str,
                   kind: &str,
                   traits: &[&str],
                   node: NodeId| {
            let qualified = format!("{module}.{name}");
            let target = declaration(path, &qualified, kind, traits, tree.span(node));
            declarations.push(Declaration {
                module: module.to_string(),
                qualified,
                target,
            });
        };
        for sentence in tree.nodes_with(&["sentence"]) {
            let command = tree.children(sentence).iter().copied().find(|child| {
                let term = tree.term(*child);
                declaration_command(term).is_some()
                    || matches!(
                        term,
                        "ltac_definition" | "notation_command" | "create_hintdb_command"
                    )
            });
            let Some(command) = command else {
                continue;
            };
            match tree.term(command) {
                "ltac_definition" => {
                    if let Some(name) = tree.first_child(command, &["ident"]) {
                        add(
                            &mut self.declarations,
                            tree.text(name),
                            "tactic",
                            &["proof-state"],
                            name,
                        );
                    }
                }
                "create_hintdb_command" => {
                    if let Some(name) = tree.first_child(command, &["ident"]) {
                        let text = tree.text(name);
                        let target = declaration(
                            path,
                            &format!("hintdb:{text}"),
                            "hint-database",
                            &[],
                            tree.span(name),
                        );
                        self.hint_databases.insert(text.to_string(), target);
                    }
                }
                "notation_command" => self.notation(module, path, file, command),
                "inductive_command" => {
                    for body in tree.children_of(command, &["inductive_definition"]) {
                        let Some(name) = tree.first_child(body, &["ident"]) else {
                            continue;
                        };
                        add(
                            &mut self.declarations,
                            tree.text(name),
                            "inductive",
                            &[],
                            name,
                        );
                        for constructor in tree.descendants(body, &["constructor"]) {
                            if let Some(constructor_name) =
                                tree.first_child(constructor, &["ident"])
                            {
                                let text = tree.text(constructor_name);
                                add(
                                    &mut self.declarations,
                                    text,
                                    "constructor",
                                    &[],
                                    constructor_name,
                                );
                            }
                        }
                    }
                }
                term => {
                    let holder = tree
                        .first_child(command, &["ident_decl"])
                        .unwrap_or(command);
                    let Some(name) = tree.first_child(holder, &["ident"]) else {
                        continue;
                    };
                    let text = tree.text(name);
                    let kind = if term == "theorem_command" {
                        tree.child(command, 0)
                            .map_or("theorem", |keyword| proof_keyword(tree.text(keyword)))
                    } else {
                        declaration_command(term).unwrap_or_default()
                    };
                    let name_end = tree.span(name).1;
                    let recursive = tree.leaves_of(command).into_iter().any(|leaf| {
                        tree.span(leaf).0 >= name_end
                            && tree.term(leaf) == "identifier"
                            && tree.text(leaf) == text
                    });
                    let traits: &[&str] = if recursive { &["recursive"] } else { &[] };
                    add(&mut self.declarations, text, kind, traits, name);
                }
            }
        }
    }

    /// `Notation "x ++2" := (body)`: pattern words that occur in the body are
    /// variables, the others are atoms; one atom with operands around it.
    fn notation(&mut self, module: &str, path: &str, file: &Rc<ParsedFile>, command: NodeId) {
        let tree = &file.tree;
        let Some(declaration_node) = tree.first_child(command, &["notation_declaration"]) else {
            return;
        };
        let pattern = tree.first_child(declaration_node, &["string"]);
        let children = tree.children(declaration_node);
        let body = children
            .iter()
            .position(|child| tree.term(*child) == ":=")
            .and_then(|assign| children.get(assign + 1).copied());
        let (Some(pattern), Some(body)) = (pattern, body) else {
            return;
        };
        let body_names = tree
            .leaves_of(body)
            .into_iter()
            .filter(|leaf| tree.term(*leaf) == "identifier")
            .map(|leaf| tree.text(leaf))
            .collect::<BTreeSet<_>>();
        let quoted = tree.text(pattern);
        let words = split_words(
            quoted
                .get(1..quoted.len().saturating_sub(1))
                .unwrap_or_default(),
        );
        let atoms = words
            .iter()
            .filter(|word| !body_names.contains(**word))
            .collect::<Vec<_>>();
        let [atom] = atoms[..] else {
            return;
        };
        if words.len() > 3 {
            return;
        }
        let atom_index = words
            .iter()
            .position(|word| word == atom)
            .unwrap_or_default();
        let target = declaration(
            path,
            &format!("notation:\"{}\"", words.join(" ")),
            "notation",
            &[],
            tree.span(command),
        );
        self.notations.push(Notation {
            module: module.to_string(),
            atom: (*atom).to_string(),
            before: atom_index > 0,
            after: atom_index + 1 < words.len(),
            variables: words
                .iter()
                .filter(|word| body_names.contains(**word))
                .map(|word| (*word).to_string())
                .collect(),
            target,
            file: Rc::clone(file),
            body,
        });
    }
}

/// Entry occurrences of a notation atom (possibly split over adjacent tokens)
/// with the operands written around it.
fn notation_matches(tree: &SyntaxTree, notation: &Notation) -> Vec<Match> {
    let mut matches = Vec::new();
    let leaves = &tree.leaves;
    let atom_length = notation.atom.encode_utf16().count();
    let mut index = 0;
    while index < leaves.len() {
        if tree
            .ancestor(leaves[index], &NON_REFERENCE_CONTEXTS)
            .is_some()
        {
            index += 1;
            continue;
        }
        let mut text = String::new();
        let mut last = index;
        while last < leaves.len()
            && text.encode_utf16().count() < atom_length
            && (last == index || tree.span(leaves[last]).0 == tree.span(leaves[last - 1]).1)
        {
            text.push_str(tree.text(leaves[last]));
            last += 1;
        }
        if text != notation.atom {
            index += 1;
            continue;
        }
        let (first, last_leaf) = (leaves[index], leaves[last - 1]);
        let before = notation
            .before
            .then(|| tree.operand_before(first))
            .flatten();
        let after = notation
            .after
            .then(|| tree.operand_after(last_leaf))
            .flatten();
        if (notation.before && before.is_none()) || (notation.after && after.is_none()) {
            index += 1;
            continue;
        }
        let operands = [before, after]
            .into_iter()
            .flatten()
            .map(|operand| {
                let text = tree.text(operand);
                if tree.leaves_of(operand).len() > 1
                    && !tree.term(operand).starts_with("parenthesized")
                {
                    format!("({text})")
                } else {
                    text.to_string()
                }
            })
            .collect();
        matches.push(Match {
            start: tree.span(before.unwrap_or(first)).0,
            end: tree.span(after.unwrap_or(last_leaf)).1,
            operands,
        });
        index = last;
    }
    matches
}

/// Whether a `_CoqProject` argument names a vernacular source file; as in
/// `coq_makefile`, the extension is compared case-sensitively.
#[allow(clippy::case_sensitive_file_extension_comparisons)]
fn is_vernacular(argument: &str) -> bool {
    argument.ends_with(".v")
}
