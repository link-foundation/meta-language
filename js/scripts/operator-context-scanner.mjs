// Prefix decisions are data-driven tries. An incomplete longer operator never
// falls back to a shorter operator after contextual validation failed.
const literal = value => `(literal ${[...Buffer.from(value)].map(byte => /[A-Za-z0-9_.-]/u.test(String.fromCharCode(byte)) ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`).join('') || '%'})`;
const branch = (condition, yes, no = '') => `(if ${condition} (then ${yes})${no ? ` (else ${no})` : ''})`;
const loop = (condition, body) => `(while ${condition} (do ${body}))`;
const next = value => `(next ${literal(value)})`;
const identifiers = values => {
  if (values.some(value => typeof value !== 'string' || !/^[A-Za-z_][A-Za-z0-9_]*$/u.test(value))
    || new Set(values).size !== values.length) throw new TypeError('invalid operator-context token identities');
};

/** A marked line run whose following operator is checked without consuming it. */
export function operatorLineScanner({ name, keywordToken, operatorToken, commentToken,
  keyword = 'do', commentPrefix = '#', operators, wordOperators = [],
  separatedOperator = null, rejectedSuffixes = {}, tokenTerminators,
  whitespace = [' ', '\t', '\n', '\r'], horizontal = [' ', '\t'],
  keywordSuffix = ':', arityPrefix = '/' }) {
  identifiers([name, keywordToken, operatorToken, commentToken]);
  if (!Array.isArray(operators) || !operators.length || operators.some(value => typeof value !== 'string' || !value)
    || !Array.isArray(wordOperators) || wordOperators.some(value => typeof value !== 'string' || !value)
    || !Array.isArray(tokenTerminators) || !tokenTerminators.length
    || typeof keyword !== 'string' || !keyword || typeof commentPrefix !== 'string' || !commentPrefix) throw new TypeError('invalid operator-context profile');
  const spaces = `(choice unordered ${whitespace.map(literal).join(' ')})`;
  const inline = `(choice unordered ${horizontal.map(literal).join(' ')})`;
  const boundary = `(choice unordered ${tokenTerminators.map(literal).join(' ')} ${spaces})`;
  const suffix = `${branch(next(keywordSuffix), `advance ${branch(`(next ${spaces})`, 'fail')} (emit ${operatorToken})`)} ${loop(`(next ${inline})`, 'advance')} ${branch(next(arityPrefix), `advance ${loop(`(next ${spaces})`, 'advance')} ${branch('(next (class plain (range 0 9)))', 'fail')}`)} (emit ${operatorToken})`;
  const root = { children: new Map(), accepted: false, prefix: '' };
  for (const operator of [...operators, ...wordOperators]) {
    let node = root;
    for (const character of operator) {
      if (!node.children.has(character)) node.children.set(character, { children: new Map(), accepted: false, prefix: node.prefix + character });
      node = node.children.get(character);
    }
    node.accepted = true;
  }
  const code = node => {
    const accept = node.accepted ? `${(rejectedSuffixes[node.prefix] ?? []).map(value => branch(next(value), 'fail')).join(' ')} ${wordOperators.includes(node.prefix) ? `${branch(`(not (next ${boundary}))`, 'fail')} ` : ''}${suffix}` : 'fail';
    return [...node.children.entries()].reduceRight((fallback, [character, child]) => branch(next(character), `advance ${code(child)}`, fallback), accept);
  };
  const separated = separatedOperator === null ? '' : `${branch(next(separatedOperator[0][0]), `(consume ${literal(separatedOperator[0])}) ${loop(`(next ${inline})`, 'advance')} (consume ${literal(separatedOperator[1])}) ${branch(`(not (next ${boundary}))`, 'fail')} ${suffix}`)}`;
  const operation = `(if (not (some (valid ${keywordToken}) (valid ${operatorToken}) (valid ${commentToken}))) (then fail)) ${loop(`(next ${inline})`, `(skip ${inline})`)} (if (not (next (choice unordered (literal %0A) (literal %0D)))) (then fail)) advance ${loop(`(next ${spaces})`, 'advance')} mark ${branch(next(commentPrefix), `(emit ${commentToken})`)} ${branch(`(all (valid ${keywordToken}) ${next(keyword[0])})`, `(consume ${literal(keyword)}) ${branch(`(next ${boundary})`, `(emit ${keywordToken})`)} fail`)} ${branch(`(valid ${operatorToken})`, `${separated} ${code(root)}`)} fail`;
  return `(scanner ${name} (tokens ${keywordToken} ${operatorToken} ${commentToken}) (operations ${operation}))\n`;
}

/** A zero-width unary marker retains the source's preceding whitespace policy. */
export function unaryOperatorBoundaryScanner({ name, token, operators, whitespace = [' ', '\t', '\n', '\r'], horizontal = [' ', '\t'] }) {
  identifiers([name, token]);
  if (!Array.isArray(operators) || !operators.length || operators.some(({ text, rejectedAfter }) => typeof text !== 'string' || [...text].length !== 1 || !Array.isArray(rejectedAfter))) throw new TypeError('invalid unary marker profile');
  const spaces = `(choice unordered ${whitespace.map(literal).join(' ')})`;
  const inline = `(choice unordered ${horizontal.map(literal).join(' ')})`;
  const branches = operators.map(({ text, rejectedAfter }) => branch(next(text), `advance ${branch(`(some (next ${spaces}) ${rejectedAfter.map(value => next(value)).join(' ')})`, 'fail')} (emit ${token})`)).join(' ');
  return `(scanner ${name} (tokens ${token}) (operations (if (not (valid ${token})) (then fail)) (if (not (next ${inline})) (then fail)) ${loop(`(next ${inline})`, `(skip ${inline})`)} mark ${branches} fail))\n`;
}
