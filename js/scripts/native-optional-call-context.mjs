// Optional argument lists have a separator policy only when arguments are
// present. The absent branch retains the source target and every later suffix.
export function transformOptionalCallContext(grammar, { rule, field, optionalSymbol, pattern, count }) {
  if (!Object.hasOwn(grammar.rules, rule) || !Object.hasOwn(grammar.rules, optionalSymbol)
    || typeof field !== 'string' || !field || typeof pattern !== 'string' || !pattern
    || !Number.isSafeInteger(count) || count <= 0) {
    throw new TypeError('optional call contexts need existing rules, a field, pattern and exact count');
  }
  const selectsSymbol = node => {
    while (['ALIAS', 'FIELD', 'PREC', 'PREC_LEFT', 'PREC_RIGHT', 'PREC_DYNAMIC'].includes(node?.type)) node = node.content;
    return node?.type === 'SYMBOL' && node.name === optionalSymbol;
  };
  let changes = 0;
  const rewrite = node => {
    if (Array.isArray(node)) return node.map(rewrite);
    if (!node || typeof node !== 'object') return node;
    const rewritten = Object.fromEntries(Object.entries(node).map(([key, value]) => [key, rewrite(value)]));
    if (rewritten.type !== 'SEQ') return rewritten;
    const targets = rewritten.members.map((member, index) => member.type === 'FIELD' && member.name === field ? index : -1).filter(index => index >= 0);
    const matches = targets.filter(index => {
      const optional = rewritten.members[index + 1];
      return optional?.type === 'CHOICE' && optional.members.length === 2
        && optional.members.some(member => member.type === 'BLANK')
        && optional.members.some(selectsSymbol);
    });
    if (!matches.length) return rewritten;
    if (matches.length !== 1) throw new TypeError('one optional call production must have one selected argument suffix');
    changes += 1;
    const index = matches[0];
    const optional = rewritten.members[index + 1];
    const present = structuredClone(rewritten.members);
    present.splice(index + 1, 1,
      { type: 'NATIVE_PATTERN_LOOKAHEAD', content: { type: 'BLANK' }, pattern, immediate: true },
      structuredClone(optional.members.find(selectsSymbol)));
    const absent = structuredClone(rewritten.members);
    absent[index + 1] = { type: 'BLANK' };
    return { type: 'CHOICE', members: [
      { ...rewritten, members: present },
      { ...rewritten, members: absent },
    ] };
  };
  const rewritten = rewrite(grammar.rules[rule]);
  if (changes !== count) throw new TypeError('optional call contexts must match their recorded occurrences');
  grammar.rules[rule] = rewritten;
}
