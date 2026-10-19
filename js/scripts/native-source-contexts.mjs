// Reviewed context boundaries preserve the source productions and generate
// ordinary Links lookahead expressions shared by the runtime packages.
export function transformFieldFollowingContext(grammar, { rule, field, pattern, count }) {
  if (!Object.hasOwn(grammar.rules, rule) || typeof field !== 'string' || !field
    || typeof pattern !== 'string' || !pattern || !Number.isSafeInteger(count) || count <= 0) {
    throw new TypeError('following field contexts need a rule, field, pattern and exact count');
  }
  let changes = 0;
  const rewrite = node => {
    if (Array.isArray(node)) return node.map(rewrite);
    if (!node || typeof node !== 'object') return node;
    if (node.type === 'FIELD' && node.name === field) {
      changes += 1;
      return { ...node, content: { type: 'SEQ', members: [node.content,
        { type: 'NATIVE_PATTERN_LOOKAHEAD', content: { type: 'BLANK' }, pattern, immediate: true }] } };
    }
    return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, rewrite(value)]));
  };
  grammar.rules[rule] = rewrite(grammar.rules[rule]);
  if (changes !== count) throw new TypeError('following field contexts must match their recorded occurrences');
}

