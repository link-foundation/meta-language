// Syntax tree over a program's source mappings, shared by the project-aware
// analyses. The Rust runtime builds the same tree
// (rust/src/program_representation/project/tree.rs) with the same ordering.

/**
 * Builds a containment tree from source mappings ({term, start, end}).
 * Mappings are ordered by start, then by descending end, then by descending
 * mapping index, so of two mappings with the same range the later one (the
 * outer node of a wrapper pair) becomes the parent.
 */
export function syntaxTree(mappings, source) {
  const order = mappings.map((_, index) => index).sort((left, right) =>
    mappings[left].start - mappings[right].start ||
    mappings[right].end - mappings[left].end ||
    right - left);
  const nodes = order.map((index) => ({
    term: mappings[index].term,
    start: mappings[index].start,
    end: mappings[index].end,
    parent: null,
    children: [],
  }));
  const stack = [];
  for (const node of nodes) {
    while (stack.length && !(stack.at(-1).start <= node.start && node.end <= stack.at(-1).end)) stack.pop();
    if (stack.length) {
      node.parent = stack.at(-1);
      node.parent.children.push(node);
    }
    stack.push(node);
  }
  const leaves = nodes.filter((node) => node.children.length === 0 && node.end > node.start);
  leaves.forEach((leaf, index) => { leaf.leafIndex = index; });
  return { source, nodes, roots: nodes.filter((node) => !node.parent), leaves };
}

export function nodeText(tree, node) {
  return tree.source.slice(node.start, node.end);
}

/** Named children of a node with one of the given terms. */
export function childrenOf(node, ...terms) {
  return node.children.filter((child) => terms.includes(child.term));
}

export function firstChild(node, ...terms) {
  return node.children.find((child) => terms.includes(child.term));
}

/** Every node under (and including) a node with one of the given terms, in source order. */
export function descendants(node, ...terms) {
  const found = [];
  const visit = (current) => {
    if (terms.includes(current.term)) found.push(current);
    current.children.forEach(visit);
  };
  visit(node);
  return found;
}

export function leavesOf(tree, node) {
  return tree.leaves.filter((leaf) => leaf.start >= node.start && leaf.end <= node.end);
}

export function ancestor(node, ...terms) {
  for (let current = node.parent; current; current = current.parent) {
    if (terms.includes(current.term)) return current;
  }
  return undefined;
}

/** The innermost leaf at exactly the given range. */
export function leafAt(tree, start, end) {
  return tree.leaves.find((leaf) => leaf.start === start && leaf.end === end);
}

/** The outermost node starting at a leaf and ending no later than a bound. */
function largestNodeFrom(tree, leaf, bound) {
  let candidate = leaf;
  for (let current = leaf.parent; current && current !== bound; current = current.parent) {
    if (current.start === leaf.start && current.end <= bound.end) candidate = current;
  }
  return candidate;
}

function smallestCommon(first, second) {
  for (let current = first; current; current = current.parent) {
    if (current.start <= second.start && second.end <= current.end) return current;
  }
  return undefined;
}

/** The operand written right after a leaf: the largest node starting at the next leaf. */
export function operandAfter(tree, leaf) {
  const next = tree.leaves[leaf.leafIndex + 1];
  if (!next) return undefined;
  const enclosing = smallestCommon(leaf, next);
  return enclosing ? largestNodeFrom(tree, next, enclosing) : next;
}

/** The operand written right before a leaf: the largest node ending at the previous leaf. */
export function operandBefore(tree, leaf) {
  const previous = tree.leaves[leaf.leafIndex - 1];
  if (!previous) return undefined;
  const enclosing = smallestCommon(leaf, previous);
  let candidate = previous;
  for (let current = previous.parent; current && current !== enclosing; current = current.parent) {
    if (current.end === previous.end && current.start >= enclosing.start) candidate = current;
  }
  return candidate;
}

/** A declaration record for the reference and expansion tables. */
export function declaration(file, qualified, kind, traits, node) {
  return { symbol: `${file}#${qualified}`, kind, traits, file, start: node.start, end: node.end };
}

export function dirname(path) {
  const index = path.lastIndexOf('/');
  return index < 0 ? '' : path.slice(0, index);
}

/** Joins a relative path onto a directory, resolving `.` and `..` segments. */
export function joinPath(directory, relative) {
  const parts = [];
  for (const part of `${directory}/${relative}`.split('/')) {
    if (part === '' || part === '.') continue;
    if (part === '..') parts.pop();
    else parts.push(part);
  }
  return parts.join('/');
}

/** Encodes a string as a JSON string literal, as JSON.stringify does. */
export function jsonString(value) {
  return JSON.stringify(value);
}
