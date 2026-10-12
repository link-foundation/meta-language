// Recorded complete-production preferences preserve every original alternative.
// References and fields select source grammar nodes; the executor receives only
// ordinary aliases, ordered choices and zero-width boundary assertions.
export function applySourceContextPreference(grammar, decision) {
  if (decision.family === 'rule-context-copy') {
    const { rule, helper } = decision;
    if (!Object.hasOwn(grammar.rules, rule) || typeof helper !== 'string' || !/^_[A-Za-z_]+$/u.test(helper) || Object.hasOwn(grammar.rules, helper)) throw new TypeError('a contextual rule copy needs an existing source and unique hidden helper');
    grammar.rules[helper] = structuredClone(grammar.rules[rule]);
    return true;
  }
  if (decision.family !== 'symbol-complete-context') return false;
  const { rules, symbol, preferredSymbol, preferredField = null, fieldSymbol = null,
    sourceField = null, alias = null, boundaries, count, helper = null, wholeRule = false, fieldAlias = null } = decision;
  if (!Array.isArray(rules) || !rules.length || rules.some(rule => !Object.hasOwn(grammar.rules, rule))
    || !Object.hasOwn(grammar.rules, symbol) || !Object.hasOwn(grammar.rules, preferredSymbol)
    || !Array.isArray(boundaries) || !boundaries.length || boundaries.some(boundary => typeof boundary !== 'string' || !boundary)
    || !Number.isSafeInteger(count) || count <= 0 || (sourceField !== null && typeof sourceField !== 'string')
    || (alias !== null && (typeof alias !== 'string' || !alias))
    || (fieldAlias !== null && (typeof fieldAlias !== 'string' || !fieldAlias || preferredField === null))
    || typeof wholeRule !== 'boolean'
    || (helper !== null && (typeof helper !== 'string' || !/^_[A-Za-z_]+$/u.test(helper)))
    || (preferredField === null) !== (fieldSymbol === null)
    || (fieldSymbol !== null && !Object.hasOwn(grammar.rules, fieldSymbol))) throw new TypeError('complete symbol contexts need existing rules, symbols, fields and exact counts');
  let preferred = { type: 'SYMBOL', name: preferredSymbol };
  let helperBody = null;
  if (preferredField !== null) {
    let fields = 0;
    const replaceField = node => {
      if (Array.isArray(node)) return node.map(replaceField);
      if (!node || typeof node !== 'object') return node;
      if (node.type === 'FIELD' && node.name === preferredField && node.content.type === 'SYMBOL') {
        fields += 1;
        const content = { type: 'SYMBOL', name: fieldSymbol };
        return { ...node, content: fieldAlias === null ? content : { type: 'ALIAS', named: true, value: fieldAlias, content } };
      }
      return Object.fromEntries(Object.entries(node).map(([key, child]) => [key, replaceField(child)]));
    };
    const body = replaceField(structuredClone(grammar.rules[preferredSymbol]));
    if (fields !== 1) throw new TypeError('a preferred production must identify exactly one source field');
    if (helper === null) throw new TypeError('a restricted production needs an explicit hidden source helper');
    if (Object.hasOwn(grammar.rules, helper) && JSON.stringify(grammar.rules[helper]) !== JSON.stringify(body)) throw new TypeError('a preferred production helper conflicts with an existing rule');
    preferred = { type: 'ALIAS', named: true, value: alias ?? preferredSymbol,
      content: { type: 'SYMBOL', name: helper } };
    helperBody = body;
  } else if (alias !== null) preferred = { type: 'ALIAS', named: true, value: alias, content: preferred };
  const originals = rules.map(rule => [rule, grammar.rules[rule]]);
  let changes = 0;
  const replace = (node, field = null) => {
    if (Array.isArray(node)) return node.map(child => replace(child, field));
    if (!node || typeof node !== 'object') return node;
    if (node.type === 'SYMBOL' && node.name === symbol && (sourceField === null || field === sourceField)) {
      changes += 1;
      return boundaries.reduce((content, boundary) => ({ type: 'NATIVE_COMPLETE_CONTEXT_VARIANT',
        content, boundary, preferred: structuredClone(preferred) }), node);
    }
    return Object.fromEntries(Object.entries(node).map(([key, child]) => [key,
      replace(child, node.type === 'FIELD' && key === 'content' ? node.name : field)]));
  };
  const updated = originals.map(([rule, node]) => {
    if (!wholeRule) return [rule, replace(node)];
    changes += 1;
    return [rule, boundaries.reduce((content, boundary) => ({ type: 'NATIVE_COMPLETE_CONTEXT_VARIANT', content, boundary, preferred: structuredClone(preferred) }), node)];
  });
  if (changes !== count) throw new TypeError('complete symbol contexts must select exactly their recorded source occurrences');
  if (helperBody !== null) grammar.rules[helper] = helperBody;
  for (const [rule, node] of updated) grammar.rules[rule] = node;
  return true;
}
