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

/** Delimiter runs shorter or longer than a terminator remain content. */
export function delimiterRunScanner({ name, contentToken, endToken, delimiter: character, count = 3 }) {
  for (const value of [name, contentToken, endToken]) identifier(value);
  if (!Number.isSafeInteger(count) || count < 2) throw new TypeError('a delimiter run needs a terminator count of at least two');
  const item = delimiter(character);
  let tail = `(if (next ${item}) (then (emit ${contentToken})) (else mark (emit ${endToken})))`;
  for (let index = 1; index < count; index += 1) tail = `(if (next ${item}) (then (consume ${item}) ${tail}) (else mark (emit ${contentToken})))`;
  return `(scanner ${name} (tokens ${contentToken} ${endToken}) (operations (consume ${item}) mark ${tail}))\n`;
}

/** A zero-width line boundary, after optional horizontal space and CR. */
export function lineBoundaryScanner({ name, token }) {
  identifier(name);
  identifier(token);
  const space = '(class plain (char %20) (char %09))';
  return `(scanner ${name} (tokens ${token}) (operations (if (not (valid ${token})) (then fail)) (while (next ${space}) (do (skip ${space}))) (if (next (literal %0D)) (then (skip (literal %0D)))) (if (some atEnd (next (literal %0A))) (then (emit ${token}))) fail))\n`;
}

const characterClass = (characters, ranges = []) => {
  if (!Array.isArray(characters) || !Array.isArray(ranges) || characters.length + ranges.length === 0) throw new TypeError('a character class must be nonempty');
  const character = (value) => {
    if (typeof value !== 'string' || [...value].length !== 1) throw new TypeError('a character class item must be one code point');
    return encode(value);
  };
  const items = characters.map((value) => `(char ${character(value)})`);
  for (const range of ranges) {
    if (!Array.isArray(range) || range.length !== 2 || range[0].codePointAt(0) > range[1].codePointAt(0)) throw new TypeError('a character range must have ordered endpoints');
    items.push(`(range ${character(range[0])} ${character(range[1])})`);
  }
  return `(class plain ${items.join(' ')})`;
};

/** A zero-width token selected by lookahead, without consuming that context. */
export function lookaheadBoundaryScanner({ name, token, whitespace = [' ', '\t', '\r', '\v', '\f'], before, allowEnd = true }) {
  identifier(name);
  identifier(token);
  if (!Array.isArray(before) || before.length === 0 || typeof allowEnd !== 'boolean') throw new TypeError('a lookahead boundary needs terminators and an end policy');
  const spaces = characterClass(whitespace);
  const terminators = before.map((text) => `(next ${delimiter(text)})`).join(' ');
  return `(scanner ${name} (tokens ${token}) (operations (if (not (valid ${token})) (then fail)) mark (while (next ${spaces}) (do advance)) (if (some ${allowEnd ? 'atEnd ' : ''}${terminators}) (then (emit ${token}))) fail))\n`;
}

/** A marked token whose acceptance depends on the following lexical context. */
export function contextTokenScanner({ name, token, opening = '', whitespace = [' ', '\t', '\n', '\r', '\v', '\f'], requireWhitespace = false, rejectAfter = [], immediateCharacters = [], immediateRanges = [], contextPrefix = '', rejectContextWhitespace = false, target, stops, allowEnd = false, comments = false, advanceBeforeCheck = false, blockedTokens = [] }) {
  identifier(name);
  identifier(token);
  blockedTokens.forEach(identifier);
  if (![requireWhitespace, rejectContextWhitespace, allowEnd, comments, advanceBeforeCheck].every((value) => typeof value === 'boolean')) throw new TypeError('context token policies must be booleans');
  if (!Array.isArray(stops) || stops.length === 0) throw new TypeError('a context token needs stopping delimiters');
  const spaces = characterClass(whitespace);
  const stop = `(some ${stops.map((text) => `(next ${delimiter(text)})`).join(' ')})`;
  const blocked = blockedTokens.length ? `(if (some ${blockedTokens.map((item) => `(expected (ref ${item}))`).join(' ')}) (then fail)) ` : '';
  const openingCode = opening ? `(consume ${delimiter(opening)}) ` : '';
  const rejected = rejectAfter.length ? `(if (some ${rejectAfter.map((text) => `(next ${delimiter(text)})`).join(' ')}) (then fail)) ` : '';
  const immediate = immediateCharacters.length + immediateRanges.length ? `(if (next ${characterClass(immediateCharacters, immediateRanges)}) (then (emit ${token}))) ` : '';
  const prefix = contextPrefix ? `(consume ${delimiter(contextPrefix)}) ` : '';
  const rejectSpace = rejectContextWhitespace ? `(if (next ${spaces}) (then fail)) ` : '';
  const accepted = `(if (all (next ${delimiter(target)})${comments ? ' (equal (depth comment) (integer 0))' : ''}) (then (emit ${token})))`;
  const commentCode = comments ? `(if (all (equal (depth comment) (integer 0)) (next (literal %2F%2A))) (then (consume (literal %2F%2A)) (push comment (integer 1))) (else (if (all (greater (depth comment) (integer 0)) (next (literal %2A%2F))) (then (consume (literal %2A%2F)) (pop comment)) (else advance))))` : 'advance';
  const followingComment = comments ? `(if (all (equal (depth comment) (integer 0)) (next (literal %2F))) (then advance (if (next (literal %2A)) (then (push comment (integer 1))))) (else (if (all (greater (depth comment) (integer 0)) (next (literal %2A))) (then advance (if (next (literal %2F)) (then (pop comment)))))))` : '';
  const loop = advanceBeforeCheck ? `advance ${accepted} ${followingComment}` : `${accepted} ${commentCode}`;
  const clear = comments ? '(while (greater (depth comment) (integer 0)) (do (pop comment))) ' : '';
  return `(scanner ${name} (tokens ${token}) (operations (if (not (valid ${token})) (then fail)) ${blocked}${requireWhitespace ? `(if (not (next ${spaces})) (then fail)) ` : ''}(while (next ${spaces}) (do (skip ${spaces}))) ${openingCode}${rejected}mark ${immediate}${prefix}${rejectSpace}(while (not atEnd) (do (if ${stop} (then fail)) ${loop})) ${allowEnd ? `${clear}(emit ${token})` : 'fail'}))\n`;
}

/** A quote run closed by a matching run after a line break and padding. */
export function lineCountedDelimiterScanner({ name, token, delimiter: character = '"', minimum = 3, whitespace = [' ', '\t', '\r'], prefix = '', optionalPrefixCharacters = [], countModulo = null }) {
  identifier(name);
  identifier(token);
  if (!Number.isSafeInteger(minimum) || minimum < 2 || (countModulo !== null && (!Number.isSafeInteger(countModulo) || countModulo < minimum))) throw new TypeError('line delimiter counts must be ordered integers');
  const item = delimiter(character);
  const padding = characterClass(whitespace);
  const linePadding = `(all (next ${padding}) (not (next (literal %0A))))`;
  const optionalPrefix = optionalPrefixCharacters.length ? `(if (next ${characterClass(optionalPrefixCharacters)}) (then advance)) ` : '';
  const clear = (stack) => `(while (greater (depth ${stack}) (integer 0)) (do (pop ${stack})))`;
  const increment = '(push delimiters (integer 1))' + (countModulo === null ? '' : ` (if (equal (depth delimiters) (integer ${countModulo})) (then ${clear('delimiters')}))`);
  const initial = Array.from({ length: minimum }, () => `(consume ${item}) ${increment}`).join(' ');
  const candidate = `(while (all (next ${item}) (less (depth candidate) (depth delimiters))) (do (consume ${item}) (push candidate (integer 1))))`;
  return `(scanner ${name} (tokens ${token}) (operations (if (not (valid ${token})) (then fail)) (while (next ${padding}) (do (skip ${padding}))) ${prefix ? `(consume ${delimiter(prefix)}) ${optionalPrefix}` : ''}${initial} (while (next ${item}) (do (consume ${item}) ${increment})) (while ${linePadding} (do advance)) (consume (literal %0A)) (while (not atEnd) (do (if (next (literal %0A)) (then advance (while ${linePadding} (do advance)) ${candidate} (if (equal (depth candidate) (depth delimiters)) (then ${clear('candidate')} ${clear('delimiters')} (emit ${token}))) ${clear('candidate')}) (else advance)))) fail))\n`;
}

/** Generate each scanner from a JSON family descriptor, rejecting unknown families. */
export function scannerFamilies(descriptors) {
  const names = new Set();
  const tokens = new Set();
  return descriptors.map(({ family, ...options }) => {
    const declared = family === 'split-counted-delimiter' ? [options.startToken, options.contentToken, options.endToken]
      : family === 'delimiter-run' ? [options.contentToken, options.endToken]
        : [options.token, ...(options.closeToken ? [options.closeToken] : [])];
    if (names.has(options.name) || new Set(declared).size !== declared.length || declared.some((token) => tokens.has(token))) throw new TypeError('duplicate scanner name or token');
    names.add(options.name);
    for (const token of declared) tokens.add(token);
    if (family === 'delimited') return delimitedScanner(options);
    if (family === 'content') return contentScanner(options);
    if (family === 'counted-delimiter') return countedDelimiterScanner(options);
    if (family === 'split-counted-delimiter') return splitCountedDelimiterScanner(options);
    if (family === 'delimiter-run') return delimiterRunScanner(options);
    if (family === 'line-boundary') return lineBoundaryScanner(options);
    if (family === 'lookahead-boundary') return lookaheadBoundaryScanner(options);
    if (family === 'context-token') return contextTokenScanner(options);
    if (family === 'line-counted-delimiter') return lineCountedDelimiterScanner(options);
    throw new TypeError(`unknown scanner family ${JSON.stringify(family)}`);
  }).join('');
}
