// One recorded occurrence can accept existing source layout alternatives.
// Other occurrences retain their original separator and continuation policy.
export function transformSymbolChoiceContext(grammar, { rule, symbol, occurrence, count, helper, alternatives }) {
  const existing = name => typeof name === 'string' && Object.hasOwn(grammar.rules, name);
  const declared = new Set([...Object.keys(grammar.rules), ...(grammar.externals ?? []).filter(node => node.type === 'SYMBOL').map(node => node.name)]);
  const positive = value => Number.isSafeInteger(value) && value > 0;
  if (!existing(rule) || !existing(symbol) || !positive(occurrence) || !positive(count) || occurrence > count
    || typeof helper !== 'string' || !/^_[A-Za-z_][A-Za-z0-9_]*$/u.test(helper) || declared.has(helper)
    || !Array.isArray(alternatives) || !alternatives.length || !alternatives.every(name => declared.has(name) && name !== symbol)
    || new Set(alternatives).size !== alternatives.length) {
    throw new TypeError('symbol choice contexts need source rules, a distinct hidden helper, declared alternatives and an exact occurrence');
  }
  let observed = 0;
  const rewrite = node => {
    if (Array.isArray(node)) return node.map(rewrite);
    if (!node || typeof node !== 'object') return node;
    if (node.type === 'SYMBOL' && node.name === symbol) {
      observed += 1;
      if (observed === occurrence) return { type: 'SYMBOL', name: helper };
    }
    return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, rewrite(value)]));
  };
  const rewritten = rewrite(grammar.rules[rule]);
  if (observed !== count) throw new TypeError('symbol choice contexts must match their recorded occurrence count');
  Object.assign(grammar.rules, {
    [rule]: rewritten,
    [helper]: { type: 'CHOICE', members: [symbol, ...alternatives].map(name => ({ type: 'SYMBOL', name })) },
  });
}
