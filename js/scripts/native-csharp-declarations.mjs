// Source-selected declaration preferences generate ordinary shared grammar
// expressions. Runtime executors do not need a language-specific callback.
export function applyDeclarationSourceTransform(grammar, decision) {
  if (decision.family === 'optional-prefix-preference') {
    const { rule, field, count } = decision;
    if (!Object.hasOwn(grammar.rules, rule) || typeof field !== 'string' || field.length === 0
      || !Number.isSafeInteger(count) || count <= 0) {
      throw new TypeError('optional prefix preferences need an existing rule, field selector and exact occurrence count');
    }
    const contains = (node) => node?.type === 'FIELD' && node.name === field
      || Array.isArray(node) && node.some(contains)
      || node && typeof node === 'object' && Object.values(node).some(contains);
    let changes = 0;
    const rewrite = (node) => {
      if (Array.isArray(node)) return node.map(rewrite);
      if (!node || typeof node !== 'object') return node;
      const prefix = node.type === 'SEQ' ? node.members[0] : undefined;
      if (prefix?.type === 'CHOICE' && prefix.members.length === 2
        && prefix.members.some((member) => member.type === 'BLANK')
        && prefix.members.some(contains)) {
        changes += 1;
        const present = prefix.members.find((member) => member.type !== 'BLANK');
        const absent = prefix.members.find((member) => member.type === 'BLANK');
        return { type: 'NATIVE_ORDERED_CHOICE', members: [
          { ...node, members: [present, ...node.members.slice(1)] },
          { ...node, members: [absent, ...node.members.slice(1)] },
        ] };
      }
      return Object.fromEntries(Object.entries(node).map(([key, child]) => [key, rewrite(child)]));
    };
    grammar.rules[rule] = rewrite(grammar.rules[rule]);
    if (changes !== count) throw new TypeError('optional prefix preferences must match exactly the recorded declaration occurrences');
    return true;
  }
  if (decision.family !== 'alias-dynamic-precedence') return false;
  const { rules, alias, value, count } = decision;
  if (!Array.isArray(rules) || rules.length === 0 || new Set(rules).size !== rules.length
    || rules.some((rule) => !Object.hasOwn(grammar.rules, rule))
    || typeof alias !== 'string' || alias.length === 0
    || !Number.isSafeInteger(value) || value <= 0
    || !Number.isSafeInteger(count) || count <= 0) {
    throw new TypeError('alias preferences need distinct existing rules, a named alias, positive precedence and an exact occurrence count');
  }
  let changes = 0;
  const rewrite = (node) => {
    if (Array.isArray(node)) return node.map(rewrite);
    if (!node || typeof node !== 'object') return node;
    if (node.type === 'ALIAS' && node.named === true && node.value === alias) {
      changes += 1;
      return { type: 'PREC_DYNAMIC', value, content: node };
    }
    return Object.fromEntries(Object.entries(node).map(([key, child]) => [key, rewrite(child)]));
  };
  for (const rule of rules) grammar.rules[rule] = rewrite(grammar.rules[rule]);
  if (changes !== count) throw new TypeError('alias preferences must match exactly the recorded declaration occurrences');
  return true;
}
