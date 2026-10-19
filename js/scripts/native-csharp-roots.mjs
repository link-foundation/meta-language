// Scoped source decisions generate ordinary precedence and complete-choice
// expressions. Runtime packages consume the same resulting Links grammar.
export function applyRootSourceTransform(grammar, decision) {
  if (!['repetition-member-precedence', 'nested-complete-choice'].includes(decision.family)) return false;
  const { rule, count } = decision;
  if (!Object.hasOwn(grammar.rules, rule) || !Number.isSafeInteger(count) || count <= 0) throw new TypeError('root source contexts need an existing rule and exact occurrence count');
  let changes = 0;
  if (decision.family === 'repetition-member-precedence') {
    const { precedences } = decision;
    if (!Array.isArray(precedences) || !precedences.length || new Set(precedences.map(({ symbol }) => symbol)).size !== precedences.length
      || precedences.some(({ symbol, value }) => !Object.hasOwn(grammar.rules, symbol) || !Number.isSafeInteger(value))) throw new TypeError('repetition precedence needs unique existing symbols and integer priorities');
    const rewrite = node => {
      if (Array.isArray(node)) return node.map(rewrite);
      if (!node || typeof node !== 'object') return node;
      if (node.type === 'REPEAT' && node.content?.type === 'CHOICE'
        && precedences.every(({ symbol }) => node.content.members.filter(member => member.type === 'SYMBOL' && member.name === symbol).length === 1)) {
        changes += 1;
        return { ...node, content: { ...node.content, members: node.content.members.map(member => {
          const selected = precedences.find(({ symbol }) => member.type === 'SYMBOL' && member.name === symbol);
          return selected ? { type: 'PREC_DYNAMIC', value: selected.value, content: member } : member;
        }) } };
      }
      return Object.fromEntries(Object.entries(node).map(([name, child]) => [name, rewrite(child)]));
    };
    const rewritten = rewrite(grammar.rules[rule]);
    if (changes !== count) throw new TypeError('repetition precedence must match exactly the recorded member context');
    grammar.rules[rule] = rewritten;
    return true;
  }
  const { preferred, alternatives, boundary } = decision;
  const selector = node => node.type === 'SYMBOL' ? node.name
    : node.type === 'SEQ' && node.members[0]?.type === 'SYMBOL' ? `prefix:${node.members[0].name}` : null;
  if (!Array.isArray(alternatives) || alternatives.length < 2 || new Set(alternatives).size !== alternatives.length
    || !alternatives.includes(preferred) || typeof boundary !== 'string' || !boundary) throw new TypeError('complete nested choices need distinct alternatives, one preferred selector and a literal boundary');
  const rewrite = node => {
    if (Array.isArray(node)) return node.map(rewrite);
    if (!node || typeof node !== 'object') return node;
    if (node.type === 'CHOICE' && node.members.length === alternatives.length
      && alternatives.every(name => node.members.filter(member => selector(member) === name).length === 1)) {
      changes += 1;
      return { type: 'NATIVE_COMPLETE_CONTEXT_VARIANT', preferred: node.members.find(member => selector(member) === preferred), boundary, content: node };
    }
    return Object.fromEntries(Object.entries(node).map(([name, child]) => [name, rewrite(child)]));
  };
  const rewritten = rewrite(grammar.rules[rule]);
  if (changes !== count) throw new TypeError('complete nested choices must match exactly the recorded alternative context');
  grammar.rules[rule] = rewritten;
  return true;
}
