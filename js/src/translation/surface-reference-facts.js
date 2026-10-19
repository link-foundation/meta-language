// Mechanical object/array transport for the portable reference analysis.
// Field labels and primitive values retain their original spellings. Decisions
// about scopes, types, syntax and representation live in the checked rules.
export function createSurfaceReferenceFacts(value, { retainValues = false } = {}) {
  const forms = [], texts = [], starts = [], counts = [], labels = [], targets = [];
  const values = [];
  const identities = new WeakMap();
  function insert(node) {
    if (node && typeof node === 'object' && identities.has(node)) return identities.get(node);
    const identity = forms.length;
    if (retainValues) values.push(node);
    const form = node === null ? 'null' : Array.isArray(node) ? 'array' : typeof node;
    forms.push(form);
    texts.push(form === 'object' || form === 'array' || form === 'undefined' || form === 'null' ? '' : String(node));
    starts.push(0);
    counts.push(0);
    if (node && typeof node === 'object') {
      identities.set(node, identity);
      const children = Object.entries(node).map(([label, child]) => [label, insert(child)]);
      starts[identity] = labels.length;
      counts[identity] = children.length;
      for (const [label, child] of children) { labels.push(label); targets.push(child); }
    }
    return identity;
  }
  const root = insert(value);
  return { forms, texts, starts, counts, labels, targets, root, ...(retainValues ? { values } : {}) };
}

/** Pack the same rows into two primitive columns for the checked kernel ABI. */
export function createSurfaceReferenceColumns(facts) {
  return {
    columns: facts.forms.concat(facts.texts, facts.labels),
    rows: facts.starts.concat(facts.counts, facts.targets),
    size: facts.forms.length,
  };
}
