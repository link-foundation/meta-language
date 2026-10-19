// Stateful scanner families compile to the common operation language. Neither
// runtime needs a language-specific callback to execute these definitions.
import { parseTreeSitterPattern, renderTreeSitterPattern } from '../src/grammar-importers/tree-sitter-native.js';

const encode = (text) => [...Buffer.from(text)].map((byte) => /[A-Za-z0-9_.-]/u.test(String.fromCharCode(byte)) ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`).join('') || '%';
const literal = (text) => `(literal ${encode(text)})`;
const branch = (condition, operations) => `(if ${condition} (then ${operations}))`;
const integer = (number) => `(integer ${number})`;
const variable = (name) => `(variable ${name})`;
const set = (name, value) => `(set ${name} ${value})`;
const increment = (name) => set(name, `(add ${variable(name)} (integer 1))`);
const newline = '(choice unordered (literal %0D%0A) (literal %0A))';
const identifier = (name) => {
  if (typeof name !== 'string' || !/^[A-Za-z_][A-Za-z0-9_]*$/u.test(name)) throw new TypeError('invalid structured scanner identifier');
  return name;
};
const identifiers = (name, tokens) => {
  [name, ...tokens].forEach(identifier);
  if (new Set(tokens).size !== tokens.length) throw new TypeError('structured scanners need distinct tokens');
};
const text = (value) => {
  if (typeof value !== 'string' || !value) throw new TypeError('structured scanner delimiters must be nonempty');
  return literal(value);
};
const pattern = (value) => {
  if (typeof value !== 'string' || !value || new RegExp(`^(?:${value})$`, 'u').test('')) throw new TypeError('structured scanner patterns must consume input');
  return renderTreeSitterPattern(parseTreeSitterPattern(value));
};

/** Remember line-anchored labels, including quoted labels and FIFO bodies. */
export function hereDocumentScanner({ name, startToken, contentToken, endToken,
  opening = '<<', labelPattern = '[A-Za-z_][A-Za-z0-9_]*', quotes = ['"', "'"],
  indentation = '', openingWhitespace = '', closingSuffix = '', queue = false,
  allowEmpty = true }) {
  identifiers(name, [startToken, contentToken, endToken]);
  if (![queue, allowEmpty].every((value) => typeof value === 'boolean') || !Array.isArray(quotes) || new Set(quotes).size !== quotes.length
    || [indentation, openingWhitespace, closingSuffix].some((value) => typeof value !== 'string')
    || [...indentation, ...openingWhitespace].some((value) => ![' ', '\t'].includes(value))) throw new TypeError('invalid here-document policies');
  quotes.forEach(text);
  const label = pattern(labelPattern);
  const labels = `${name}_labels`;
  const temporary = `${name}_temporary`;
  const labelRule = `${name}_label`;
  if ([startToken, contentToken, endToken].includes(labelRule)) throw new TypeError('here-document helper name conflicts with a declared token');
  const seen = `${name}_content`;
  const padding = (characters) => `(choice unordered ${[...new Set(characters)].map(literal).join(' ')})`;
  const indented = indentation ? `(repeat0 ${padding(indentation)}) ` : '';
  const horizontal = openingWhitespace ? `(repeat0 ${padding(openingWhitespace)}) ` : '';
  const matching = `(predicate ${label} (equal (matched) (top ${labels})))`;
  const closing = `(seq ${indented}${matching}${closingSuffix ? ` (optional ${literal(closingSuffix)})` : ''} (and (choice unordered ${newline} (not (class negated)))))`;
  // The oldest label is brought to the top only for this scan and restored
  // before emission. A failed scan discards all speculative state mutations.
  const select = queue ? `(while (greater (depth ${labels}) (integer 1)) (do (push ${temporary} (top ${labels})) (pop ${labels}))) ` : '';
  const restore = queue ? `(while (greater (depth ${temporary}) (integer 0)) (do (push ${labels} (top ${temporary})) (pop ${temporary}))) ` : '';
  const boundary = `(all (equal column (integer 0)) (next ${closing}))`;
  const reset = set(seen, integer(0));
  const scan = `(if (equal (depth ${labels}) (integer 0)) (then fail)) ${select}`
    + branch(`(valid ${endToken})`, `(if (not ${boundary}) (then fail)) (consume (seq ${indented}${matching}${closingSuffix ? ` (optional ${literal(closingSuffix)})` : ''})) (pop ${labels}) ${restore}${reset} (emit ${endToken})`) + ' '
    + branch(`(valid ${contentToken})`, `${reset} (while (not atEnd) (do ${branch(boundary, `${allowEmpty ? '' : `(if (equal ${variable(seen)} (integer 0)) (then fail)) `}${restore}${reset} mark (emit ${contentToken})`)} (if (next (literal %00)) (then fail)) advance ${set(seen, integer(1))})) fail`);
  return `(scanner ${name} (tokens ${contentToken} ${endToken}) (operations ${scan} fail))\n`
    + `(rule ${labelRule} token ${label} (action (push ${labels} (matched))))\n`
    + `(rule ${startToken} token (seq ${text(opening)} ${horizontal}(choice ordered ${quotes.map((quote) => `(seq ${literal(quote)} (ref ${labelRule}) ${literal(quote)})`).join(' ')} (ref ${labelRule}))))\n`;
}

/** Quote contexts and interpolation braces remain balanced across token calls. */
export function interpolatedQuotedScanner({ name, startToken, contentToken, endToken,
  interpolationStartToken, interpolationEndToken, braceOpenToken, braceCloseToken,
  quotes = [{ opening: '"', closing: '"', interpolation: '${', multiline: false }],
  interpolationClosing = '}', braceOpening = '{', braceClosing = '}', escape = '\\' }) {
  const tokens = [startToken, contentToken, endToken, interpolationStartToken, interpolationEndToken, braceOpenToken, braceCloseToken];
  identifiers(name, tokens);
  [interpolationClosing, braceOpening, braceClosing].forEach(text);
  if (escape !== null) text(escape);
  if (!Array.isArray(quotes) || !quotes.length || new Set(quotes.map(({ opening }) => opening)).size !== quotes.length) throw new TypeError('interpolated quotes need distinct opening delimiters');
  for (const quote of quotes) {
    text(quote.opening); text(quote.closing);
    if (quote.interpolation !== null) text(quote.interpolation);
    if (quote.escape !== undefined && quote.escape !== null) text(quote.escape);
    if (typeof quote.multiline !== 'boolean') throw new TypeError('interpolated quotes need multiline policies');
  }
  const contexts = `${name}_contexts`;
  const seen = `${name}_content`;
  const top = `(top ${contexts})`;
  const inExpression = `(all (greater (depth ${contexts}) (integer 0)) (not (greater ${top} (integer 0))))`;
  const enter = (index) => `(push ${contexts} (integer ${index}))`;
  const emitContent = `(if (equal ${variable(seen)} (integer 0)) (then fail)) ${set(seen, integer(0))} mark (emit ${contentToken})`;
  const starts = quotes.toSorted((left, right) => right.opening.length - left.opening.length).map((quote) => {
    const index = quotes.indexOf(quote) + 1;
    return branch(`(all (valid ${startToken}) (next ${literal(quote.opening)}))`, `(consume ${literal(quote.opening)}) ${enter(index)} (emit ${startToken})`);
  }).join(' ');
  const bodies = quotes.map((quote, index) => {
    const closing = `(next ${literal(quote.closing)})`;
    const interpolation = quote.interpolation === null ? '(equal (integer 0) (integer 1))' : `(next ${literal(quote.interpolation)})`;
    const quoteEscape = quote.escape === undefined ? escape : quote.escape;
    const escapeCode = quoteEscape === null ? 'advance' : `(if (next ${literal(quoteEscape)}) (then (consume ${literal(quoteEscape)}) (if (some atEnd (next (literal %00))) (then fail)) advance) (else advance))`;
    return branch(`(equal ${top} (integer ${index + 1}))`,
      branch(`(valid ${endToken})`, `(consume ${literal(quote.closing)}) (pop ${contexts}) (emit ${endToken})`) + ' '
      + (quote.interpolation === null ? '' : branch(`(valid ${interpolationStartToken})`, `(consume ${literal(quote.interpolation)}) ${enter(0)} (emit ${interpolationStartToken})`) + ' ')
      + branch(`(valid ${contentToken})`, `${set(seen, integer(0))} (while (not atEnd) (do ${branch(`(some ${closing} ${interpolation})`, emitContent)} (if (some (next (literal %00))${quote.multiline ? '' : ` (next ${newline})`}) (then fail)) ${escapeCode} ${set(seen, integer(1))})) fail`));
  }).join(' ');
  return `(scanner ${name} (tokens ${tokens.join(' ')}) (operations `
    + branch(`(all (valid ${interpolationEndToken}) (equal ${top} (integer 0)) (greater (depth ${contexts}) (integer 0)))`, `(consume ${literal(interpolationClosing)}) (pop ${contexts}) (emit ${interpolationEndToken})`) + ' '
    + branch(`(all (valid ${braceOpenToken}) ${inExpression})`, `(consume ${literal(braceOpening)}) ${enter(-1)} (emit ${braceOpenToken})`) + ' '
    + branch(`(all (valid ${braceCloseToken}) (equal ${top} (integer -1)))`, `(consume ${literal(braceClosing)}) (pop ${contexts}) (emit ${braceCloseToken})`) + ' '
    + bodies + ' ' + starts + ' fail))\n';
}

/** CommonMark fenced blocks: matching marker, minimum width and indentation. */
export function fencedBlockScanner({ name, token, markers = ['`', '~'], minimum = 3, maximumIndentation = 3, allowEnd = true }) {
  identifiers(name, [token]);
  if (!Array.isArray(markers) || !markers.length || new Set(markers).size !== markers.length || markers.some((value) => typeof value !== 'string' || [...value].length !== 1)
    || !Number.isSafeInteger(minimum) || minimum < 1 || !Number.isSafeInteger(maximumIndentation) || maximumIndentation < 0 || typeof allowEnd !== 'boolean') throw new TypeError('invalid fenced block policies');
  const width = `${name}_width`;
  const candidate = `${name}_candidate`;
  const indent = `${name}_indentation`;
  const padding = '(class plain (char %20) (char %09))';
  const indentation = `${set(indent, integer(0))} (while (next (literal %20)) (do advance ${increment(indent)}))`;
  const clear = [width, candidate, indent].map((key) => set(key, integer(0))).join(' ');
  const cases = markers.map((marker) => {
    const item = literal(marker);
    return branch(`(next ${item})`, `${set(width, integer(0))} (while (next ${item}) (do advance ${increment(width)})) (if (less ${variable(width)} (integer ${minimum})) (then fail))`
      + ` (while (all (not atEnd) (not (next ${newline}))) (do ${marker === '`' ? branch(`(next ${item})`, 'fail') + ' ' : ''}(if (next (literal %00)) (then fail)) advance)) (if atEnd (then ${allowEnd ? `${clear} (emit ${token})` : 'fail'})) (consume ${newline})`
      + ` (while (not atEnd) (do ${branch('(equal column (integer 0))', `${indentation} ${set(candidate, integer(0))} (while (next ${item}) (do advance ${increment(candidate)})) (while (next ${padding}) (do advance)) ${branch(`(all (not (greater ${variable(indent)} (integer ${maximumIndentation}))) (not (less ${variable(candidate)} ${variable(width)})) (some atEnd (next ${newline})))`, `(if (not atEnd) (then (consume ${newline}))) ${clear} (emit ${token})`)}`)} (if atEnd (then ${allowEnd ? `${clear} (emit ${token})` : 'fail'})) (if (next (literal %00)) (then fail)) advance)) ${allowEnd ? `${clear} (emit ${token})` : 'fail'}`);
  }).join(' ');
  return `(scanner ${name} (tokens ${token}) (operations (if (not (valid ${token})) (then fail)) (if (not (equal column (integer 0))) (then fail)) ${indentation} (if (greater ${variable(indent)} (integer ${maximumIndentation})) (then fail)) ${cases} fail))\n`;
}

/** Offside blocks opened by grammar-owned keywords and closed by dedentation. */
export function offsideLayoutScanner({ name, openToken, separatorToken, closeToken,
  whitespace = [' ', '\t', '\n', '\r'], closing = [] }) {
  identifiers(name, [openToken, separatorToken, closeToken]);
  if (!Array.isArray(whitespace) || !whitespace.length || whitespace.some((value) => ![' ', '\t', '\n', '\r'].includes(value)) || !Array.isArray(closing)) throw new TypeError('invalid offside layout policies');
  closing.forEach(text);
  const spaces = `(choice unordered ${whitespace.map(literal).join(' ')})`;
  const columns = `${name}_columns`;
  const pending = `${name}_pending`;
  const boundary = closing.length ? `(some ${closing.map((value) => `(next ${literal(value)})`).join(' ')})` : '(equal (integer 0) (integer 1))';
  const padding = `(while (next ${spaces}) (do ${branch(`(next ${newline})`, set(pending, integer(1)))} (skip ${spaces})))`;
  return `(scanner ${name} (tokens ${openToken} ${separatorToken} ${closeToken}) (operations ${padding} `
    + branch(`(valid ${openToken})`, `(if (some atEnd ${boundary}) (then fail)) (push ${columns} column) ${set(pending, integer(0))} (emit ${openToken})`) + ' '
    + `(if (equal (depth ${columns}) (integer 0)) (then fail)) `
    + branch(`(all (valid ${closeToken}) (some atEnd ${boundary} (all (equal ${variable(pending)} (integer 1)) (less column (top ${columns})))))`, `(pop ${columns}) (emit ${closeToken})`) + ' '
    + branch(`(all (valid ${separatorToken}) (equal ${variable(pending)} (integer 1)) (equal column (top ${columns})) (not atEnd) (not ${boundary}))`, `${set(pending, integer(0))} (emit ${separatorToken})`) + ' fail))\n';
}
