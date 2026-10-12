// Prefer an omitted optional suffix when a complete syntactic boundary follows.
// Lexical prefix tests cannot recognize boundaries with rules that skip extras.
export function transformSyntacticBoundaryContext(grammar, { rule, symbol, occurrence, count, boundary }) {
  const existing = name => typeof name === 'string' && Object.hasOwn(grammar.rules, name);
  const positive = value => Number.isSafeInteger(value) && value > 0;
  if (!existing(rule) || !existing(symbol) || !positive(occurrence) || !positive(count) || occurrence > count
    || !boundary || typeof boundary !== 'object' || Array.isArray(boundary) || typeof boundary.type !== 'string') {
    throw new TypeError('syntactic boundary contexts need source rules, an exact optional-symbol occurrence and a boundary expression');
  }
  const declared = new Set([...Object.keys(grammar.rules), ...(grammar.externals ?? []).filter(node => node.type === 'SYMBOL').map(node => node.name)]);
  const checkBoundary = node => {
    if (Array.isArray(node)) return node.forEach(checkBoundary);
    if (!node || typeof node !== 'object') return;
    if (node.type === 'SYMBOL' && !declared.has(node.name)) throw new TypeError('syntactic boundary references need declared source symbols');
    Object.values(node).forEach(checkBoundary);
  };
  checkBoundary(boundary);
  const original = structuredClone(grammar.rules[rule]);
  const preferred = structuredClone(original);
  let observed = 0;
  let selected = null;
  const find = (node, parent = null) => {
    if (Array.isArray(node)) return node.forEach(child => find(child, parent));
    if (!node || typeof node !== 'object') return;
    if (node.type === 'SYMBOL' && node.name === symbol) {
      observed += 1;
      if (observed === occurrence) selected = { node, parent };
    }
    Object.values(node).forEach(child => find(child, node));
  };
  find(preferred);
  if (observed !== count || !selected || selected.parent?.type !== 'CHOICE' || selected.parent.members.length !== 2
    || !selected.parent.members.includes(selected.node) || !selected.parent.members.some(node => node.type === 'BLANK')) {
    throw new TypeError('syntactic boundary contexts must select exactly the recorded optional suffix');
  }
  const replace = node => {
    if (node === selected.parent) return { type: 'BLANK' };
    if (Array.isArray(node)) return node.map(replace);
    if (!node || typeof node !== 'object') return node;
    return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, replace(value)]));
  };
  grammar.rules[rule] = {
    type: 'NATIVE_SYNTACTIC_CONTEXT_VARIANT',
    preferred: replace(preferred), boundary: structuredClone(boundary), content: original,
  };
}
