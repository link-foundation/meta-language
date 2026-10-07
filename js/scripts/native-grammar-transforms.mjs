// Reviewed source-grammar transformations. They generate ordinary grammar
// expressions; neither executor needs a language-specific callback.
export function transformNativeSource(source, transformations = []) {
  const grammar = structuredClone(source);
  for (const decision of transformations) {
    if (decision.family !== 'nullable-pattern-extras') throw new TypeError(`unknown native grammar transformation ${decision.family}`);
    const { rule, pattern, extraAlternative, helper, whitespace } = decision;
    if (!/^[A-Za-z_][A-Za-z0-9_]*$/u.test(helper) || !helper.startsWith('_') || Object.hasOwn(grammar.rules, helper)) throw new TypeError('a nullable-pattern extra needs a unique hidden helper');
    const choice = grammar.rules[rule];
    if (choice?.type !== 'CHOICE' || !Number.isInteger(extraAlternative) || !choice.members[extraAlternative]) throw new TypeError('the nullable-pattern extra alternative is absent');
    grammar.rules[helper] = structuredClone(choice.members[extraAlternative]);
    let replaced = 0;
    const rewrite = (node) => {
      if (Array.isArray(node)) return node.map(rewrite);
      if (!node || typeof node !== 'object') return node;
      const alias = node.type === 'FIELD' ? node.content : node;
      if (alias?.type === 'ALIAS' && alias.content?.type === 'PATTERN' && alias.content.value === pattern) {
        replaced += 1;
        const token = { ...alias, content: { type: 'IMMEDIATE_TOKEN', content: alias.content } };
        return { type: 'SEQ', members: [
          { type: 'REPEAT', content: { type: 'SEQ', members: [
            { type: 'IMMEDIATE_TOKEN', content: { type: 'PATTERN', value: whitespace } },
            { type: 'ALIAS', named: true, value: rule, content: { type: 'SYMBOL', name: helper } },
          ] } },
          node.type === 'FIELD' ? { ...node, content: token } : token,
        ] };
      }
      return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, rewrite(value)]));
    };
    grammar.rules[rule] = rewrite(choice);
    if (replaced !== 1) throw new TypeError('the nullable-pattern extra must identify exactly one aliased pattern');
  }
  return grammar;
}
