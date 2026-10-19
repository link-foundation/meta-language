// Generate contextual line endings and matching bracket scopes as scanner data.
import { parseTreeSitterPattern, renderTreeSitterPattern } from '../src/grammar-importers/tree-sitter-native.js';

export function scopedLayoutScanner({ name, startToken, newlineToken, separatorToken,
  separator = ';', continuationToken, continuation = 'else',
  identifierContinuation = '[A-Za-z0-9_.\\u0080-\\u{10ffff}]',
  commentPrefix = '#', pairs, whitespace = [' ', '\t', '\n', '\r', '\v', '\f'],
  recoveryToken = null, triviaToken = null }) {
  if (!Array.isArray(pairs) || !pairs.length) throw new TypeError('scoped layout needs bracket pairs');
  const tokens = [startToken, newlineToken, separatorToken, continuationToken,
    ...(triviaToken === null ? [] : [triviaToken]),
    ...pairs.flatMap(({ openToken, closeToken }) => [openToken, closeToken]),
    ...(recoveryToken === null ? [] : [recoveryToken])];
  for (const value of [name, ...tokens]) {
    if (typeof value !== 'string' || !/^[A-Za-z_][A-Za-z0-9_]*$/u.test(value)) throw new TypeError('invalid scoped layout identifier');
  }
  if (new Set(tokens).size !== tokens.length || !Array.isArray(pairs) || !pairs.length) throw new TypeError('scoped layout needs distinct tokens and bracket pairs');
  const encode = (text) => [...Buffer.from(text)].map((byte) => (
    (byte >= 65 && byte <= 90) || (byte >= 97 && byte <= 122)
    || (byte >= 48 && byte <= 57) || [45, 46, 95].includes(byte)
      ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`
  )).join('');
  const literal = (text) => {
    if (typeof text !== 'string' || !text) throw new TypeError('scoped layout delimiters must be nonempty');
    return `(literal ${encode(text)})`;
  };
  for (const { opening, closing, ignoreNewlines, allowContinuation } of pairs) {
    literal(opening); literal(closing);
    if (typeof ignoreNewlines !== 'boolean' || typeof allowContinuation !== 'boolean') throw new TypeError('scoped layout policies must be booleans');
  }
  if (typeof identifierContinuation !== 'string' || !identifierContinuation || new RegExp(`^(?:${identifierContinuation})$`, 'u').test('')) throw new TypeError('identifier continuation must consume input');
  const following = renderTreeSitterPattern(parseTreeSitterPattern(identifierContinuation));
  const space = `(choice unordered ${whitespace.map(literal).join(' ')})`;
  const stack = `${name}_scopes`;
  const branch = (condition, body) => `(if ${condition} (then ${body}))`;
  const scope = (index) => `(equal (top ${stack}) (integer ${index + 1}))`;
  const anyScope = (policy) => {
    const selected = pairs.flatMap((pair, index) => pair[policy] ? [scope(index)] : []);
    return selected.length ? `(some ${selected.join(' ')})` : '(equal (integer 0) (integer 1))';
  };
  const scanContinuation = `(consume ${literal(continuation)}) (if (next ${following}) (then fail)) mark (emit ${continuationToken})`;
  const brackets = pairs.map(({ opening, closing, openToken, closeToken }, index) =>
    branch(`(all (valid ${openToken}) (next ${literal(opening)}))`, `(consume ${literal(opening)}) (push ${stack} (integer ${index + 1})) (emit ${openToken})`) + ' '
    + branch(`(all (valid ${closeToken}) ${scope(index)} (next ${literal(closing)}))`, `(consume ${literal(closing)}) (pop ${stack}) (emit ${closeToken})`)).join(' ');
  return `(scanner ${name} (tokens ${tokens.join(' ')}) (operations `
    + (recoveryToken === null ? '' : branch(`(valid ${recoveryToken})`, 'fail') + ' ')
    + branch(`(valid ${startToken})`, `(emit ${startToken})`) + ' '
    + (triviaToken === null ? '' : branch(`(all (valid ${triviaToken}) ${anyScope('ignoreNewlines')} (next ${space}))`, `(consume (repeat1 ${space})) (emit ${triviaToken})`) + ' ')
    + (triviaToken === null ? '' : branch(`(all (valid ${triviaToken}) ${anyScope('allowContinuation')} (expected (ref ${continuationToken})) (next ${space}))`, `(consume (repeat1 ${space})) (if (not (next ${literal(commentPrefix)})) (then fail)) (emit ${triviaToken})`) + ' ')
    + `(while (all (next ${space}) (some (not (next ${literal('\n')})) ${anyScope('ignoreNewlines')})) (do (skip ${space}))) `
    + brackets + ' '
    + branch(`(all (valid ${separatorToken}) (next ${literal(separator)}))`, `(consume ${literal(separator)}) (emit ${separatorToken})`) + ' '
    + branch(`(all (valid ${continuationToken}) (next ${literal(continuation)}))`, scanContinuation) + ' '
    + branch(`(all (valid ${continuationToken}) ${anyScope('allowContinuation')} (next ${literal('\n')}))`,
      `(while (next ${space}) (do (skip ${space}) mark)) (if (next ${literal(commentPrefix)}) (then fail)) `
      + branch(`(next ${literal(continuation)})`, scanContinuation) + ` (emit ${newlineToken})`) + ' '
    + branch(`(all (valid ${newlineToken}) ${anyScope('allowContinuation')} (expected (ref ${continuationToken})) (next (seq (repeat1 ${space}) ${literal(continuation)} (not ${following}))))`, 'fail') + ' '
    + branch(`(all (valid ${newlineToken}) (next ${literal('\n')}))`, `(consume ${literal('\n')}) (emit ${newlineToken})`) + ' fail))\n';
}
