// Generate split raw literals with remembered quote, bracket and marker count.
export function quotedCountedScanner({ name, startToken, contentToken, endToken,
  prefixes = ['r', 'R'], quotes = ['"', "'"], marker = '-',
  pairs = [{ opening: '(', closing: ')' }, { opening: '[', closing: ']' }, { opening: '{', closing: '}' }],
  maximum = 255 }) {
  for (const value of [name, startToken, contentToken, endToken]) {
    if (typeof value !== 'string' || !/^[A-Za-z_][A-Za-z0-9_]*$/u.test(value)) throw new TypeError('invalid quoted counted scanner identifier');
  }
  if (new Set([startToken, contentToken, endToken]).size !== 3 || !Number.isSafeInteger(maximum) || maximum < 0) throw new TypeError('quoted counted delimiters need distinct tokens and a nonnegative maximum');
  const encode = (text) => [...Buffer.from(text)].map((byte) => (
    (byte >= 65 && byte <= 90) || (byte >= 97 && byte <= 122)
    || (byte >= 48 && byte <= 57) || [45, 46, 95].includes(byte)
      ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`
  )).join('');
  const literal = (text) => {
    if (typeof text !== 'string' || !text) throw new TypeError('quoted counted delimiters must be nonempty');
    return `(literal ${encode(text)})`;
  };
  for (const choices of [prefixes, quotes, pairs]) if (!Array.isArray(choices) || !choices.length) throw new TypeError('quoted counted choices must be nonempty');
  for (const text of [...quotes, ...pairs.flatMap(({ opening, closing }) => [opening, closing])]) {
    if (typeof text !== 'string' || [...text].length !== 1) throw new TypeError('quoted counted quotes and brackets must be one code point');
  }
  if (new Set(prefixes).size !== prefixes.length || new Set(quotes).size !== quotes.length
    || new Set(pairs.map(({ opening }) => opening)).size !== pairs.length) throw new TypeError('quoted counted choices must be distinct');
  const prefix = `(choice unordered ${prefixes.map(literal).join(' ')})`;
  const repeated = literal(marker);
  const quoteStack = `${name}_quotes`;
  const bracketStack = `${name}_brackets`;
  const countStack = `${name}_delimiters`;
  const candidateStack = `${name}_candidate`;
  const contentVariable = `${name}_content`;
  const branch = (condition, body) => `(if ${condition} (then ${body}))`;
  const clear = (stack) => `(while (greater (depth ${stack}) (integer 0)) (do (pop ${stack})))`;
  // Select exactly one branch: the state value changes before later tests run.
  const select = (values, stack, textOf) => values.toReversed().reduce((tail, item) => {
    const index = values.indexOf(item);
    return `(if (next ${literal(textOf(item))}) (then (consume ${literal(textOf(item))}) (push ${stack} (integer ${index + 1}))) (else ${tail}))`;
  }, 'fail');
  const closing = `(some ${pairs.map(({ closing }, index) => `(all (equal (top ${bracketStack}) (integer ${index + 1})) (next ${literal(closing)}))`).join(' ')})`;
  const quote = `(some ${quotes.map((text, index) => `(all (equal (top ${quoteStack}) (integer ${index + 1})) (next ${literal(text)}))`).join(' ')})`;
  const readMarkers = `(while (next ${repeated}) (do (consume ${repeated}) (push ${candidateStack} (integer 1))))`;
  const matched = `(all (equal (depth ${candidateStack}) (depth ${countStack})) ${quote})`;
  const reset = `${clear(candidateStack)} ${clear(countStack)} (pop ${quoteStack}) (pop ${bracketStack})`;
  return `(scanner ${name} (tokens ${startToken} ${contentToken} ${endToken}) (operations `
    + branch(`(valid ${startToken})`, `(while (next (class plain (char %20) (char %09) (char %0A) (char %0B) (char %0C) (char %0D))) (do (skip (class plain (char %20) (char %09) (char %0A) (char %0B) (char %0C) (char %0D))))) (consume ${prefix}) ${select(quotes, quoteStack, (text) => text)} (while (next ${repeated}) (do (if (not (less (depth ${countStack}) (integer ${maximum}))) (then fail)) (consume ${repeated}) (push ${countStack} (integer 1)))) ${select(pairs, bracketStack, ({ opening }) => opening)} (emit ${startToken})`) + ' '
    + branch(`(valid ${contentToken})`, `(set ${contentVariable} (integer 0)) (while (not atEnd) (do (if ${closing} (then mark advance ${readMarkers} ${branch(matched, `(if (equal (variable ${contentVariable}) (integer 0)) (then fail)) ${clear(candidateStack)} (emit ${contentToken})`)} ${clear(candidateStack)} (set ${contentVariable} (integer 1))) (else advance (set ${contentVariable} (integer 1)))))) fail`) + ' '
    + branch(`(valid ${endToken})`, `(if (not ${closing}) (then fail)) advance ${readMarkers} (if (not ${matched}) (then fail)) advance ${reset} (emit ${endToken})`) + ' fail))\n';
}
