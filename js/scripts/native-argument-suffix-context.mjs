// A bare argument can use the source call grammar without claiming a suffix
// reserved for its enclosing call. Parenthesized argument rules stay original.
export function transformArgumentSuffixContext(grammar, {
  argumentRule, expressionRule, suffixRule, expressionHelper, ruleHelpers,
  excludedRules = [], expressionCount, expressionCallCount, suffixCounts, exclusionCounts,
}) {
  const existing = name => typeof name === 'string' && Object.hasOwn(grammar.rules, name);
  const fresh = name => typeof name === 'string' && /^_[A-Za-z_][A-Za-z0-9_]*$/u.test(name)
    && !Object.hasOwn(grammar.rules, name) && !(grammar.externals ?? []).some(node => node.name === name);
  const positive = value => Number.isSafeInteger(value) && value > 0;
  if (![argumentRule, expressionRule, suffixRule].every(existing) || !fresh(expressionHelper)
    || !ruleHelpers || typeof ruleHelpers !== 'object' || Array.isArray(ruleHelpers)
    || !Object.keys(ruleHelpers).length || !Object.keys(ruleHelpers).every(existing)
    || !Object.values(ruleHelpers).every(fresh)
    || new Set([expressionHelper, ...Object.values(ruleHelpers)]).size !== Object.keys(ruleHelpers).length + 1
    || !Array.isArray(excludedRules) || !excludedRules.every(existing)
    || new Set(excludedRules).size !== excludedRules.length || excludedRules.some(name => Object.hasOwn(ruleHelpers, name))
    || !positive(expressionCount) || !positive(expressionCallCount)
    || !suffixCounts || typeof suffixCounts !== 'object' || Array.isArray(suffixCounts)
    || Object.keys(suffixCounts).length !== Object.keys(ruleHelpers).length
    || !Object.keys(ruleHelpers).every(name => Object.hasOwn(suffixCounts, name) && Number.isSafeInteger(suffixCounts[name]) && suffixCounts[name] >= 0)
    || !exclusionCounts || typeof exclusionCounts !== 'object' || Array.isArray(exclusionCounts)
    || Object.keys(exclusionCounts).length !== excludedRules.length
    || !excludedRules.every(name => Object.hasOwn(exclusionCounts, name) && positive(exclusionCounts[name]))) {
    throw new TypeError('argument suffix contexts need existing rules, distinct hidden helpers and exact occurrence counts');
  }
  const containsSuffix = node => node?.type === 'SYMBOL' && node.name === suffixRule
    || Array.isArray(node) && node.some(containsSuffix)
    || node && typeof node === 'object' && Object.values(node).some(containsSuffix);
  const selectedName = node => {
    while (['ALIAS', 'FIELD', 'PREC', 'PREC_LEFT', 'PREC_RIGHT', 'PREC_DYNAMIC'].includes(node?.type)) node = node.content;
    return node?.type === 'SYMBOL' ? node.name : null;
  };
  const excluded = new Map(excludedRules.map(name => [name, 0]));
  const hidden = new Set([...(grammar.inline ?? []), ...(grammar.supertypes ?? [])].map(node => typeof node === 'string' ? node : node.name));
  const mappedReference = name => name.startsWith('_') || hidden.has(name)
    ? { type: 'SYMBOL', name: ruleHelpers[name] }
    : { type: 'ALIAS', named: true, value: name, content: { type: 'SYMBOL', name: ruleHelpers[name] } };
  const cloneCalls = (node, counts, stripSuffix) => {
    if (Array.isArray(node)) return node.map(child => cloneCalls(child, counts, stripSuffix));
    if (!node || typeof node !== 'object') return node;
    if (node.type === 'SYMBOL' && Object.hasOwn(ruleHelpers, node.name)) {
      counts.calls += 1;
      return mappedReference(node.name);
    }
    const rewritten = Object.fromEntries(Object.entries(node).map(([key, value]) => [key, cloneCalls(value, counts, stripSuffix)]));
    if (rewritten.type !== 'CHOICE') return rewritten;
    if (stripSuffix && rewritten.members.length === 2
      && rewritten.members.some(member => member.type === 'BLANK')
      && rewritten.members.some(containsSuffix)) {
      counts.suffixes += 1;
      return { type: 'BLANK' };
    }
    const members = rewritten.members.filter(member => {
      const name = selectedName(member);
      if (!excluded.has(name)) return true;
      excluded.set(name, excluded.get(name) + 1);
      return false;
    });
    if (!members.length) throw new TypeError('an argument call context cannot exclude every source alternative');
    return { ...rewritten, members };
  };
  const additions = {};
  for (const [name, helper] of Object.entries(ruleHelpers)) {
    const counts = { calls: 0, suffixes: 0 };
    additions[helper] = cloneCalls(grammar.rules[name], counts, true);
    if (counts.suffixes !== suffixCounts[name]) throw new TypeError(`argument suffix count differs for ${name}`);
  }
  const counts = { calls: 0, suffixes: 0 };
  additions[expressionHelper] = cloneCalls(grammar.rules[expressionRule], counts, false);
  if (counts.calls !== expressionCallCount) throw new TypeError('argument expression call count differs');
  if (excludedRules.some(name => excluded.get(name) !== exclusionCounts[name])) throw new TypeError('argument call exclusion counts differ');
  let references = 0;
  const projectArguments = node => {
    if (Array.isArray(node)) return node.map(projectArguments);
    if (!node || typeof node !== 'object') return node;
    if (node.type === 'SYMBOL' && node.name === expressionRule) {
      references += 1;
      return { type: 'SYMBOL', name: expressionHelper };
    }
    return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, projectArguments(value)]));
  };
  const argumentsBody = projectArguments(grammar.rules[argumentRule]);
  if (references !== expressionCount) throw new TypeError('bare argument expression count differs');
  Object.assign(grammar.rules, additions, { [argumentRule]: argumentsBody });
}
