// Select field alternatives by their actual source structure. Ordering keeps
// every production, alias, field and lexical wrapper in the imported graph.
export function transformOrderedFieldAlternatives(grammar, { rule, field, variants, count,
  literalContinuationPattern = null, literalContinuationFlags = '' }) {
  if (!Object.hasOwn(grammar.rules, rule) || typeof field !== 'string' || !field
    || !Array.isArray(variants) || variants.length < 2 || !Number.isSafeInteger(count) || count <= 0
    || literalContinuationPattern !== null && (typeof literalContinuationPattern !== 'string' || !literalContinuationPattern)
    || typeof literalContinuationFlags !== 'string') {
    throw new TypeError('ordered field alternatives need an existing rule, field, selectors and exact count');
  }
  const unwrap = node => {
    while (['PREC', 'PREC_LEFT', 'PREC_RIGHT', 'PREC_DYNAMIC', 'TOKEN', 'IMMEDIATE_TOKEN'].includes(node.type)) node = node.content;
    return node;
  };
  const matches = (member, selector) => {
    if (!selector || typeof selector !== 'object' || Array.isArray(selector)) throw new TypeError('field alternatives need structural selectors');
    const node = unwrap(member);
    if (Object.hasOwn(selector, 'symbol')) {
      if (Object.keys(selector).length !== 1 || typeof selector.symbol !== 'string' || !Object.hasOwn(grammar.rules, selector.symbol)) throw new TypeError('field symbol selectors must name one existing rule');
      return node.type === 'SYMBOL' && node.name === selector.symbol;
    }
    if (typeof selector.alias !== 'string' || !selector.alias || typeof selector.named !== 'boolean'
      || (Object.hasOwn(selector, 'contentSymbol') ? Object.hasOwn(selector, 'literals') : !Object.hasOwn(selector, 'literals'))
      || Object.keys(selector).length !== 3) throw new TypeError('field alias selectors need a name, named identity and one exact content selector');
    if (node.type !== 'ALIAS' || node.value !== selector.alias || node.named !== selector.named) return false;
    const content = unwrap(node.content);
    if (Object.hasOwn(selector, 'contentSymbol')) {
      if (typeof selector.contentSymbol !== 'string' || !Object.hasOwn(grammar.rules, selector.contentSymbol)) throw new TypeError('field alias content must name an existing source rule');
      return content.type === 'SYMBOL' && content.name === selector.contentSymbol;
    }
    if (!Array.isArray(selector.literals) || !selector.literals.length || selector.literals.some(value => typeof value !== 'string')) throw new TypeError('field alias literals must be an exact nonempty source list');
    const members = content.type === 'CHOICE' ? content.members : [content];
    return members.length === selector.literals.length && members.every((member, index) => unwrap(member).type === 'STRING' && unwrap(member).value === selector.literals[index]);
  };
  let changes = 0;
  const rewrite = node => {
    if (Array.isArray(node)) return node.map(rewrite);
    if (!node || typeof node !== 'object') return node;
    if (node.type === 'FIELD' && node.name === field && node.content.type === 'CHOICE') {
      const members = node.content.members;
      const indices = variants.map(selector => {
        const selected = members.map((member, index) => matches(member, selector) ? index : -1).filter(index => index >= 0);
        if (selected.length !== 1) throw new TypeError('ordered field selectors must each match exactly one source alternative');
        return selected[0];
      });
      if (indices.length !== members.length || new Set(indices).size !== members.length) throw new TypeError('ordered field selectors must preserve every source alternative exactly once');
      changes += 1;
      const selected = indices.map((index, position) => {
        if (literalContinuationPattern === null || !Object.hasOwn(variants[position], 'literals')) return members[index];
        const boundary = member => ['PREC', 'PREC_LEFT', 'PREC_RIGHT', 'PREC_DYNAMIC', 'TOKEN', 'IMMEDIATE_TOKEN'].includes(member.type)
          ? { ...member, content: boundary(member.content) }
          : { ...member, content: { type: 'NATIVE_WORD_BOUNDARY', content: member.content,
            word: { type: 'PATTERN', value: literalContinuationPattern, ...(literalContinuationFlags ? { flags: literalContinuationFlags } : {}) } } };
        return boundary(members[index]);
      });
      return { ...node, content: { ...node.content, type: 'NATIVE_ORDERED_CHOICE', members: selected } };
    }
    return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, rewrite(value)]));
  };
  grammar.rules[rule] = rewrite(grammar.rules[rule]);
  if (changes !== count) throw new TypeError('ordered field alternatives must match the recorded field count');
}
