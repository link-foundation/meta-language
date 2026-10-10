// Generate quoted and line-tagged templates as executable scanner data.
// Context and label stacks belong to the descriptor, so nested templates and
// interpolation expressions share no mutable host scanner state.
import { parseTreeSitterPattern, renderTreeSitterPattern } from '../src/grammar-importers/tree-sitter-native.js';

export function templateContextScanner(options) {
  const { name, quotedStartToken, quotedEndToken, contentToken,
    interpolationStartToken, interpolationEndToken, directiveStartToken,
    directiveEndToken, delimiterToken, labelPattern,
    escapeCharacters = ['"', 'n', 'r', 't', '\\'],
    hexadecimalEscapes = [{ prefix: 'u', digits: 4 }, { prefix: 'U', digits: 8 }],
  } = options;
  const tokens = [quotedStartToken, quotedEndToken, contentToken,
    interpolationStartToken, interpolationEndToken, directiveStartToken,
    directiveEndToken, delimiterToken];
  for (const value of [name, ...tokens]) {
    if (typeof value !== 'string' || !/^[A-Za-z_][A-Za-z0-9_]*$/u.test(value)) throw new TypeError('invalid template scanner identifier');
  }
  if (new Set(tokens).size !== tokens.length || typeof labelPattern !== 'string' || !labelPattern || new RegExp(`^(?:${labelPattern})$`, 'u').test('')) throw new TypeError('template scanners need distinct tokens and a nonempty label pattern');
  if (!Array.isArray(escapeCharacters) || escapeCharacters.some((value) => typeof value !== 'string' || [...value].length !== 1)
    || !Array.isArray(hexadecimalEscapes) || hexadecimalEscapes.some(({ prefix, digits }) => typeof prefix !== 'string' || [...prefix].length !== 1 || !Number.isSafeInteger(digits) || digits <= 0)
    || new Set([...escapeCharacters, ...hexadecimalEscapes.map(({ prefix }) => prefix)]).size !== escapeCharacters.length + hexadecimalEscapes.length) throw new TypeError('template escapes need distinct characters and positive hexadecimal widths');
  const pattern = renderTreeSitterPattern(parseTreeSitterPattern(labelPattern));
  const contexts = `${name}_contexts`;
  const labels = `${name}_labels`;
  const newline = `${name}_newline`;
  const literal = (text) => `(literal ${[...Buffer.from(text)].map((byte) => (
    (byte >= 65 && byte <= 90) || (byte >= 97 && byte <= 122)
    || (byte >= 48 && byte <= 57) || [45, 46, 95].includes(byte)
      ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`
  )).join('')})`;
  const inContext = (kind) => `(equal (top ${contexts}) (text ${kind}))`;
  const enter = (kind) => `(push ${contexts} (text ${kind}))`;
  const whitespace = '(class plain (char %20) (char %09) (char %0A) (char %0B) (char %0C) (char %0D))';
  const branch = (condition, body) => `(if ${condition} (then ${body}))`;
  const hexadecimal = hexadecimalEscapes.map(({ prefix, digits }) => branch(`(next ${literal(prefix)})`, `(consume ${literal(prefix)}) (consume ${renderTreeSitterPattern(parseTreeSitterPattern(`[0-9a-fA-F]{${digits}}`))}) (emit ${contentToken})`)).join(' ');
  const quoted = inContext('quoted');
  const heredoc = inContext('heredoc');
  const content = `(valid ${contentToken})`;
  const fragments = [['$', 'interpolation', interpolationStartToken, interpolationEndToken], ['%', 'directive', directiveStartToken, directiveEndToken]].map(([prefix, kind, start, end]) => [
    branch(`(all (valid ${start}) (not ${inContext(kind)}))`, `(consume ${literal(prefix + '{')}) ${enter(kind)} (emit ${start})`),
    branch(`(all (valid ${end}) ${inContext(kind)})`, `(consume ${literal('}')}) (pop ${contexts}) (emit ${end})`),
    branch(`(all ${content} (next ${literal(prefix)}))`, `advance (if (next ${literal('{')}) (then fail)) (if (next ${literal(prefix)}) (then advance (if (next ${literal('{')}) (then advance)))) (emit ${contentToken})`),
  ].join(' ')).join(' ');
  return `(scanner ${name} (tokens ${tokens.join(' ')}) (operations `
    + `(set ${newline} (integer 0)) (while (next ${whitespace}) (do ${branch(`(next ${literal('\n')})`, `(set ${newline} (integer 1))`)} (skip ${whitespace}))) `
    + `(if (some atEnd (next ${literal('\0')})) (then fail)) `
    + branch(`(all (valid ${quotedStartToken}) (not ${quoted}))`, `(consume ${literal('"')}) ${enter('quoted')} (emit ${quotedStartToken})`) + ' '
    + branch(`(all (valid ${quotedEndToken}) ${quoted})`, `(consume ${literal('"')}) (pop ${contexts}) (emit ${quotedEndToken})`) + ' '
    + fragments + ' '
    + branch(`(all (valid ${delimiterToken}) (not ${heredoc}))`, `(consume ${pattern}) (push ${labels} matched) ${enter('heredoc')} (emit ${delimiterToken})`) + ' '
    + branch(`(all (valid ${delimiterToken}) ${heredoc} (equal (variable ${newline}) (integer 1)))`, `(consume ${pattern}) (if (not (equal matched (top ${labels}))) (then fail)) mark (while (all (next ${whitespace}) (not (next ${literal('\n')}))) (do advance)) (if (not (next ${literal('\n')})) (then fail)) (pop ${labels}) (pop ${contexts}) (emit ${delimiterToken})`) + ' '
    + branch(`(all ${content} ${quoted} (next ${literal('"')}))`, 'fail') + ' '
    + branch(`(all ${content} ${quoted} (next ${literal('\\')}))`, `advance ${hexadecimal} (if (not (some ${escapeCharacters.map((value) => `(next ${literal(value)})`).join(' ')})) (then fail)) advance (emit ${contentToken})`) + ' '
    + branch(`(all ${content} (some ${quoted} ${heredoc}))`, `advance (emit ${contentToken})`) + ' fail))\n';
}
