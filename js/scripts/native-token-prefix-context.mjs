// Recorded raw prefix exclusions belong inside the selected source tokens.
// Retain the token wrapper and every original lexical precedence wrapper.
export function transformTokenPrefixExclusion(grammar, {
  rule, pattern = null, prefixSymbol = null, immediate = false, count,
}) {
  const declared = prefixSymbol !== null && (Object.hasOwn(grammar.rules, prefixSymbol)
    || (grammar.externals ?? []).some(node => node.type === 'SYMBOL' && node.name === prefixSymbol));
  if (typeof rule !== 'string' || !Object.hasOwn(grammar.rules, rule)
    || (pattern === null) === (prefixSymbol === null)
    || pattern !== null && (typeof pattern !== 'string' || !pattern)
    || prefixSymbol !== null && (typeof prefixSymbol !== 'string' || !declared)
    || typeof immediate !== 'boolean' || !Number.isSafeInteger(count) || count <= 0) {
    throw new TypeError('token prefix exclusions need an existing rule, one declared prefix or pattern and an exact token count');
  }
  const prefixes = prefixSymbol === null ? { type: 'PATTERN', value: pattern } : { type: 'SYMBOL', name: prefixSymbol };
  let changes = 0;
  const insidePrecedence = node => {
    if (['PREC', 'PREC_LEFT', 'PREC_RIGHT', 'PREC_DYNAMIC'].includes(node.type)) {
      return { ...node, content: insidePrecedence(node.content) };
    }
    return { type: 'NATIVE_PREFIX_EXCLUSION', content: node, prefixes: structuredClone(prefixes), ...(immediate ? { immediate } : {}) };
  };
  const rewrite = node => {
    if (Array.isArray(node)) return node.map(rewrite);
    if (!node || typeof node !== 'object') return node;
    if (['TOKEN', 'IMMEDIATE_TOKEN'].includes(node.type)) {
      changes += 1;
      return { ...node, content: insidePrecedence(node.content) };
    }
    return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, rewrite(value)]));
  };
  const replacement = rewrite(grammar.rules[rule]);
  if (changes !== count) throw new TypeError('token prefix exclusions must select exactly their recorded token occurrences');
  grammar.rules[rule] = replacement;
}
