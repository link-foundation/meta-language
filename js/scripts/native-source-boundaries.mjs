// A source continuation across registered trivia belongs to its containing
// production. The assertion inspects those extras without consuming them.
export function transformTriviaContinuationBoundary(grammar, {
  rule, pattern, triviaRules, horizontalPattern, continuationLiteral,
  continuationPattern, continuationFlags = '', count,
}) {
  const registered = new Set((grammar.extras ?? []).filter(extra => extra.type === 'SYMBOL').map(extra => extra.name));
  if (!Object.hasOwn(grammar.rules, rule) || !Array.isArray(triviaRules) || !triviaRules.length
    || new Set(triviaRules).size !== triviaRules.length
    || triviaRules.some(name => !registered.has(name) || !Object.hasOwn(grammar.rules, name))
    || ![pattern, horizontalPattern, continuationLiteral, continuationPattern].every(value => typeof value === 'string' && value)
    || typeof continuationFlags !== 'string' || !Number.isSafeInteger(count) || count <= 0) {
    throw new TypeError('trivia continuation boundaries need existing source rules, registered trivia, patterns and an exact count');
  }
  const sequence = (...members) => ({ type: 'SEQ', members });
  const horizontal = { type: 'REPEAT', content: { type: 'PATTERN', value: horizontalPattern } };
  const newline = { type: 'PATTERN', value: pattern };
  const blankLines = { type: 'REPEAT', content: sequence(horizontal, newline) };
  const commentLines = { type: 'REPEAT1', content: sequence(horizontal,
    { type: 'CHOICE', members: triviaRules.map(name => ({ type: 'SYMBOL', name })) }, newline, blankLines) };
  const keyword = { type: 'NATIVE_WORD_BOUNDARY', content: { type: 'STRING', value: continuationLiteral },
    word: { type: 'PATTERN', value: continuationPattern, ...(continuationFlags ? { flags: continuationFlags } : {}) } };
  const prefixes = sequence(newline, blankLines, commentLines, horizontal, keyword);
  let changes = 0;
  const rewrite = node => {
    if (Array.isArray(node)) return node.map(rewrite);
    if (!node || typeof node !== 'object') return node;
    if (node.type === 'PATTERN' && node.value === pattern) {
      changes += 1;
      return { type: 'NATIVE_PREFIX_EXCLUSION', content: node, prefixes, immediate: true };
    }
    return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, rewrite(value)]));
  };
  grammar.rules[rule] = rewrite(grammar.rules[rule]);
  if (changes !== count) throw new TypeError('trivia continuation boundaries must match the recorded source pattern count');
}
