// Generate external scanners as executable Links Notation data. Delimiters
// and token names are parameters; the executors need no language callbacks.
const encode = (text) => [...Buffer.from(text)].map((byte) => (
  (byte >= 65 && byte <= 90) || (byte >= 97 && byte <= 122)
  || (byte >= 48 && byte <= 57) || [45, 46, 95].includes(byte)
    ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`
)).join('') || '%';
const literal = (text) => `(literal ${encode(text)})`;
const identifier = (name) => {
  if (typeof name !== 'string' || !/^[A-Za-z_][A-Za-z0-9_]*$/u.test(name)) {
    throw new TypeError(`invalid scanner identifier ${JSON.stringify(name)}`);
  }
  return name;
};
const delimiter = (text) => {
  if (typeof text !== 'string' || text.length === 0) throw new TypeError('a scanner delimiter must be nonempty text');
  return literal(text);
};

/** A complete delimiter token, optionally nesting and escaping its delimiters. */
export function delimitedScanner({ name, token, opening, closing, nested = false, escape = null }) {
  identifier(name);
  identifier(token);
  const open = delimiter(opening);
  const close = delimiter(closing);
  if (nested && opening === closing) throw new TypeError('nested delimiters must differ');
  const escaped = escape === null ? '' : `(if (next ${delimiter(escape)}) (then (consume ${literal(escape)}) advance) (else `;
  const nesting = nested ? `(if (next ${open}) (then (consume ${open}) (push levels (integer 1))) (else advance))` : 'advance';
  const body = `(if (next ${close}) (then (consume ${close}) (pop levels)) (else ${nesting}))`;
  return `(scanner ${name} (tokens ${token}) (operations (if (not (valid ${token})) (then fail)) (consume ${open}) (push levels (integer 1)) (while (greater (depth levels) (integer 0)) (do (if atEnd (then fail)) ${escaped}${body}${escaped ? '))' : ''})) (emit ${token})))\n`;
}

/** Content between grammar-owned delimiters, including escaped code points. */
export function contentScanner({ name, token, closing, escape = null, stops = [], closeToken = null, allowEnd = true }) {
  identifier(name);
  identifier(token);
  if (closeToken !== null) identifier(closeToken);
  const endings = [closing, ...stops].map((text) => `(next ${delimiter(text)})`);
  const stop = `(some atEnd ${endings.join(' ')})`;
  const body = escape === null ? 'advance'
    : `(if (next ${delimiter(escape)}) (then (consume ${literal(escape)}) advance) (else advance))`;
  const content = `(if (valid ${token}) (then (if ${stop} (then fail)) (while (not ${stop}) (do ${body}))${allowEnd ? '' : ' (if atEnd (then fail))'} (emit ${token})))`;
  const close = closeToken === null ? '' : ` (if (valid ${closeToken}) (then (consume ${literal(closing)}) (emit ${closeToken})))`;
  return `(scanner ${name} (tokens ${token}${closeToken === null ? '' : ` ${closeToken}`}) (operations ${content}${close} fail))\n`;
}

/** A raw token whose closing delimiter repeats the opening marker count. */
export function countedDelimiterScanner({ name, token, prefix = '', marker, opening, closing, suffix = '' }) {
  identifier(name);
  identifier(token);
  const repeated = delimiter(marker);
  const open = delimiter(opening);
  const close = delimiter(closing);
  const beginning = prefix ? `(consume ${delimiter(prefix)}) ` : '';
  const ending = suffix ? `(consume ${delimiter(suffix)}) ` : '';
  const suffixCondition = suffix ? ` (next ${literal(suffix)})` : '';
  const clear = (stack) => `(while (greater (depth ${stack}) (integer 0)) (do (pop ${stack})))`;
  return `(scanner ${name} (tokens ${token}) (operations (if (not (valid ${token})) (then fail)) ${beginning}(while (next ${repeated}) (do (consume ${repeated}) (push delimiters (integer 1)))) (consume ${open}) (while (not atEnd) (do (if (next ${close}) (then (consume ${close}) (while (all (next ${repeated}) (less (depth candidate) (depth delimiters))) (do (consume ${repeated}) (push candidate (integer 1)))) (if (all (equal (depth candidate) (depth delimiters))${suffixCondition}) (then ${ending}${clear('candidate')} ${clear('delimiters')} (emit ${token}))) ${clear('candidate')}) (else advance)))) fail))\n`;
}

/** Separate opening, content and closing tokens with shared delimiter state. */
export function splitCountedDelimiterScanner({ name, startToken, contentToken, endToken, prefix = '', marker, opening, closing, suffix = '', skipWhitespace = false, contentStops = [], countModulo = null }) {
  for (const value of [name, startToken, contentToken, endToken]) identifier(value);
  if (countModulo !== null && (!Number.isSafeInteger(countModulo) || countModulo < 2)) throw new TypeError('a scanner count modulus must be an integer of at least two');
  const repeated = delimiter(marker);
  const open = delimiter(opening);
  const close = delimiter(closing);
  const beginning = prefix ? `(consume ${delimiter(prefix)}) ` : '';
  const ending = suffix ? `(consume ${delimiter(suffix)}) ` : '';
  const suffixCondition = suffix ? ` (next ${literal(suffix)})` : '';
  const clear = (stack) => `(while (greater (depth ${stack}) (integer 0)) (do (pop ${stack})))`;
  const push = (stack) => `(push ${stack} (integer 1))${countModulo === null ? '' : ` (if (equal (depth ${stack}) (integer ${countModulo})) (then ${clear(stack)}))`}`;
  const stop = contentStops.length === 0 ? '' : `(if (some ${contentStops.map((text) => `(next ${delimiter(text)})`).join(' ')}) (then fail)) `;
  const markers = '(depth delimiters)';
  const seen = '(depth candidate)';
  const skip = skipWhitespace ? '(while (next (class plain (char %20) (char %09) (char %0A) (char %0B) (char %0C) (char %0D))) (do (skip (class plain (char %20) (char %09) (char %0A) (char %0B) (char %0C) (char %0D))))) ' : '';
  const collect = `(while (next ${repeated}) (do (consume ${repeated}) ${push('candidate')}))`;
  const start = `(if (valid ${startToken}) (then ${skip}${beginning}(while (next ${repeated}) (do (consume ${repeated}) ${push('delimiters')})) (consume ${open}) (emit ${startToken})))`;
  const content = `(if (valid ${contentToken}) (then (while (not atEnd) (do ${stop}(if (next ${close}) (then mark (consume ${close}) ${collect} (if (all (equal ${seen} ${markers})${suffixCondition}) (then ${clear('candidate')} (emit ${contentToken}))) ${clear('candidate')}) (else advance)))) fail))`;
  const end = `(if (valid ${endToken}) (then (consume ${close}) ${collect} (if (not (equal ${seen} ${markers})) (then fail)) ${ending}${clear('candidate')} ${clear('delimiters')} (emit ${endToken})))`;
  return `(scanner ${name} (tokens ${startToken} ${contentToken} ${endToken}) (operations ${start} ${content} ${end} fail))\n`;
}

/** Generate each scanner from a JSON family descriptor, rejecting unknown families. */
export function scannerFamilies(descriptors) {
  const names = new Set();
  const tokens = new Set();
  return descriptors.map(({ family, ...options }) => {
    const declared = family === 'split-counted-delimiter' ? [options.startToken, options.contentToken, options.endToken] : [options.token, ...(options.closeToken ? [options.closeToken] : [])];
    if (names.has(options.name) || new Set(declared).size !== declared.length || declared.some((token) => tokens.has(token))) throw new TypeError('duplicate scanner name or token');
    names.add(options.name);
    for (const token of declared) tokens.add(token);
    if (family === 'delimited') return delimitedScanner(options);
    if (family === 'content') return contentScanner(options);
    if (family === 'counted-delimiter') return countedDelimiterScanner(options);
    if (family === 'split-counted-delimiter') return splitCountedDelimiterScanner(options);
    throw new TypeError(`unknown scanner family ${JSON.stringify(family)}`);
  }).join('');
}
