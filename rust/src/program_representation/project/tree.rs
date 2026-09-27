//! Syntax tree over a program's source mappings, shared by the project-aware
//! analyses. The JavaScript runtime builds the same tree
//! (js/src/program-project-tree.js) with the same parent choice.

use super::super::ProgramSourceMapping;

/// Index of a node in [`SyntaxTree::nodes`].
pub(super) type NodeId = usize;

#[derive(Clone, Debug)]
pub(super) struct Node {
    pub(super) term: String,
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) parent: Option<NodeId>,
    pub(super) children: Vec<NodeId>,
    leaf_index: Option<usize>,
}

#[derive(Clone, Debug)]
pub(super) struct SyntaxTree {
    pub(super) source: String,
    pub(super) nodes: Vec<Node>,
    pub(super) roots: Vec<NodeId>,
    pub(super) leaves: Vec<NodeId>,
}

impl SyntaxTree {
    /// Builds a containment tree from source mappings. Mappings are ordered by
    /// start, then by descending end, then by mapping index: the Rust network
    /// lists a node before its children, so of two mappings with the same
    /// range the earlier one (the outer node of a wrapper pair) becomes the
    /// parent.
    pub(super) fn build(mappings: &[ProgramSourceMapping], source: &str) -> Self {
        let mut order = (0..mappings.len()).collect::<Vec<_>>();
        order.sort_by(|left, right| {
            let (left_range, right_range) = (mappings[*left].range, mappings[*right].range);
            left_range
                .start()
                .cmp(&right_range.start())
                .then(right_range.end().cmp(&left_range.end()))
                .then(left.cmp(right))
        });
        let mut nodes = order
            .iter()
            .map(|index| Node {
                term: mappings[*index].term.clone(),
                start: mappings[*index].range.start(),
                end: mappings[*index].range.end(),
                parent: None,
                children: Vec::new(),
                leaf_index: None,
            })
            .collect::<Vec<_>>();
        let mut stack: Vec<NodeId> = Vec::new();
        for id in 0..nodes.len() {
            while let Some(&top) = stack.last() {
                if nodes[top].start <= nodes[id].start && nodes[id].end <= nodes[top].end {
                    break;
                }
                stack.pop();
            }
            if let Some(&parent) = stack.last() {
                nodes[id].parent = Some(parent);
                nodes[parent].children.push(id);
            }
            stack.push(id);
        }
        let leaves = (0..nodes.len())
            .filter(|id| nodes[*id].children.is_empty() && nodes[*id].end > nodes[*id].start)
            .collect::<Vec<_>>();
        for (index, leaf) in leaves.iter().enumerate() {
            nodes[*leaf].leaf_index = Some(index);
        }
        let roots = (0..nodes.len())
            .filter(|id| nodes[*id].parent.is_none())
            .collect();
        Self {
            source: source.to_string(),
            nodes,
            roots,
            leaves,
        }
    }

    pub(super) fn term(&self, id: NodeId) -> &str {
        &self.nodes[id].term
    }

    pub(super) fn text(&self, id: NodeId) -> &str {
        &self.source[self.nodes[id].start..self.nodes[id].end]
    }

    pub(super) fn span(&self, id: NodeId) -> (usize, usize) {
        (self.nodes[id].start, self.nodes[id].end)
    }

    pub(super) fn parent(&self, id: NodeId) -> Option<NodeId> {
        self.nodes[id].parent
    }

    pub(super) fn parent_term(&self, id: NodeId) -> Option<&str> {
        self.parent(id).map(|parent| self.term(parent))
    }

    pub(super) fn children(&self, id: NodeId) -> &[NodeId] {
        &self.nodes[id].children
    }

    pub(super) fn child(&self, id: NodeId, index: usize) -> Option<NodeId> {
        self.nodes[id].children.get(index).copied()
    }

    pub(super) fn last_child(&self, id: NodeId) -> Option<NodeId> {
        self.nodes[id].children.last().copied()
    }

    /// Whether a node is the first child of its parent.
    pub(super) fn is_first_child(&self, id: NodeId) -> bool {
        self.parent(id)
            .is_some_and(|parent| self.child(parent, 0) == Some(id))
    }

    /// Named children of a node with one of the given terms.
    pub(super) fn children_of(&self, id: NodeId, terms: &[&str]) -> Vec<NodeId> {
        self.nodes[id]
            .children
            .iter()
            .copied()
            .filter(|child| terms.contains(&self.term(*child)))
            .collect()
    }

    pub(super) fn first_child(&self, id: NodeId, terms: &[&str]) -> Option<NodeId> {
        self.nodes[id]
            .children
            .iter()
            .copied()
            .find(|child| terms.contains(&self.term(*child)))
    }

    /// Every node under (and including) a node with one of the given terms, in source order.
    pub(super) fn descendants(&self, id: NodeId, terms: &[&str]) -> Vec<NodeId> {
        let mut found = Vec::new();
        let mut stack = vec![id];
        while let Some(current) = stack.pop() {
            if terms.contains(&self.term(current)) {
                found.push(current);
            }
            stack.extend(self.nodes[current].children.iter().rev());
        }
        found
    }

    /// Every node of the tree with one of the given terms, in tree order.
    pub(super) fn nodes_with(&self, terms: &[&str]) -> Vec<NodeId> {
        (0..self.nodes.len())
            .filter(|id| terms.contains(&self.term(*id)))
            .collect()
    }

    pub(super) fn leaves_of(&self, id: NodeId) -> Vec<NodeId> {
        let (start, end) = self.span(id);
        self.leaves
            .iter()
            .copied()
            .filter(|leaf| self.nodes[*leaf].start >= start && self.nodes[*leaf].end <= end)
            .collect()
    }

    pub(super) fn ancestor(&self, id: NodeId, terms: &[&str]) -> Option<NodeId> {
        let mut current = self.parent(id);
        while let Some(node) = current {
            if terms.contains(&self.term(node)) {
                return Some(node);
            }
            current = self.parent(node);
        }
        None
    }

    /// The innermost leaf at exactly the given range.
    pub(super) fn leaf_at(&self, start: usize, end: usize) -> Option<NodeId> {
        self.leaves
            .iter()
            .copied()
            .find(|leaf| self.nodes[*leaf].start == start && self.nodes[*leaf].end == end)
    }

    fn smallest_common(&self, first: NodeId, second: NodeId) -> Option<NodeId> {
        let mut current = Some(first);
        while let Some(node) = current {
            if self.nodes[node].start <= self.nodes[second].start
                && self.nodes[second].end <= self.nodes[node].end
            {
                return Some(node);
            }
            current = self.parent(node);
        }
        None
    }

    /// The operand written right after a leaf: the largest node starting at the next leaf.
    pub(super) fn operand_after(&self, leaf: NodeId) -> Option<NodeId> {
        let next = *self.leaves.get(self.nodes[leaf].leaf_index? + 1)?;
        let Some(enclosing) = self.smallest_common(leaf, next) else {
            return Some(next);
        };
        let mut candidate = next;
        let mut current = self.parent(next);
        while let Some(node) = current.filter(|node| *node != enclosing) {
            if self.nodes[node].start == self.nodes[next].start
                && self.nodes[node].end <= self.nodes[enclosing].end
            {
                candidate = node;
            }
            current = self.parent(node);
        }
        Some(candidate)
    }

    /// The operand written right before a leaf: the largest node ending at the previous leaf.
    pub(super) fn operand_before(&self, leaf: NodeId) -> Option<NodeId> {
        let previous = *self
            .leaves
            .get(self.nodes[leaf].leaf_index?.checked_sub(1)?)?;
        let enclosing = self.smallest_common(leaf, previous);
        let mut candidate = previous;
        let mut current = self.parent(previous);
        while let Some(node) = current.filter(|node| Some(*node) != enclosing) {
            if self.nodes[node].end == self.nodes[previous].end
                && enclosing.map_or(true, |outer| {
                    self.nodes[node].start >= self.nodes[outer].start
                })
            {
                candidate = node;
            }
            current = self.parent(node);
        }
        Some(candidate)
    }
}

/// A declaration for the reference and expansion tables.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Target {
    pub(super) symbol: String,
    pub(super) kind: String,
    pub(super) traits: Vec<String>,
    pub(super) file: String,
    pub(super) start: usize,
    pub(super) end: usize,
}

pub(super) fn declaration(
    file: &str,
    qualified: &str,
    kind: &str,
    traits: &[&str],
    (start, end): (usize, usize),
) -> Target {
    Target {
        symbol: format!("{file}#{qualified}"),
        kind: kind.to_string(),
        traits: traits.iter().map(ToString::to_string).collect(),
        file: file.to_string(),
        start,
        end,
    }
}

pub(super) fn dirname(path: &str) -> &str {
    path.rfind('/').map_or("", |index| &path[..index])
}

/// Joins a relative path onto a directory, resolving `.` and `..` segments.
pub(super) fn join_path(directory: &str, relative: &str) -> String {
    let joined = format!("{directory}/{relative}");
    let mut parts: Vec<&str> = Vec::new();
    for part in joined.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            _ => parts.push(part),
        }
    }
    parts.join("/")
}

/// Encodes a string as a JSON string literal, as `JSON.stringify` does.
pub(super) fn json_string(value: &str) -> String {
    serde_json::Value::from(value).to_string()
}

/// Words separated by spaces, tabs and line breaks.
pub(super) fn split_words(text: &str) -> Vec<&str> {
    text.split([' ', '\t', '\n', '\r'])
        .filter(|word| !word.is_empty())
        .collect()
}

/// One key/value line of a TOML manifest.
#[derive(Clone, Debug)]
pub(super) struct TomlEntry {
    pub(super) table: String,
    pub(super) key: String,
    pub(super) value: String,
    pub(super) start: usize,
    pub(super) end: usize,
}

const fn toml_space(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\r')
}

/// Reads the key/value lines of a TOML manifest: `[table]` and `[[array]]`
/// headers, `key = "string"` and other single-line values. A string value's
/// range covers its content only.
pub(super) fn toml_entries(source: &str) -> Vec<TomlEntry> {
    let mut entries = Vec::new();
    let mut table = String::new();
    let mut offset = 0;
    for line in source.split('\n') {
        let line_start = offset;
        offset += line.len() + 1;
        let bytes = line.as_bytes();
        let mut from = 0;
        while from < bytes.len() && toml_space(bytes[from]) {
            from += 1;
        }
        let mut to = bytes.len();
        while to > from && toml_space(bytes[to - 1]) {
            to -= 1;
        }
        let text = &line[from..to];
        if text.is_empty() || text.starts_with('#') {
            continue;
        }
        if text.len() >= 4 && text.starts_with("[[") && text.ends_with("]]") {
            text[2..text.len() - 2].trim().clone_into(&mut table);
            continue;
        }
        if text.len() >= 2 && text.starts_with('[') && text.ends_with(']') {
            text[1..text.len() - 1].trim().clone_into(&mut table);
            continue;
        }
        let Some(equals) = text.find('=') else {
            continue;
        };
        let mut key = text[..equals].trim();
        if key.len() >= 2
            && (key.starts_with('"') || key.starts_with('\''))
            && key.ends_with(&key[..1])
        {
            key = &key[1..key.len() - 1];
        }
        let mut value_from = from + equals + 1;
        while value_from < to && toml_space(bytes[value_from]) {
            value_from += 1;
        }
        let quote = bytes.get(value_from).copied();
        let close = match quote {
            Some(quote @ (b'"' | b'\'')) => line[value_from + 1..]
                .bytes()
                .position(|byte| byte == quote)
                .map(|index| value_from + 1 + index),
            _ => None,
        };
        let (start, end) = close.map_or((value_from, to), |close| (value_from + 1, close));
        entries.push(TomlEntry {
            table: table.clone(),
            key: key.to_string(),
            value: line[start..end].to_string(),
            start: line_start + start,
            end: line_start + end,
        });
    }
    entries
}
