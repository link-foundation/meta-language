// Generate external scanners as executable Links Notation data. Delimiters
// and token names are parameters; the executors need no language callbacks.
import { parseTreeSitterPattern, renderTreeSitterPattern } from '../src/grammar-importers/tree-sitter-native.js';
import { templateContextScanner } from './template-context-scanner.mjs';
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

/** Opening and closing tags agree by text, rather than just delimiter count. */
export function rememberedDelimiterScanner({ name, startToken, contentToken, endToken, tagPattern }) {
  for (const value of [name, startToken, contentToken, endToken]) identifier(value);
  if (new Set([startToken, contentToken, endToken]).size !== 3 || typeof tagPattern !== 'string' || !tagPattern) throw new TypeError('remembered delimiters need distinct tokens and a nonempty tag pattern');
  const tag = renderTreeSitterPattern(parseTreeSitterPattern(tagPattern));
  if (new RegExp(`^(?:${tagPattern})$`, 'u').test('')) throw new TypeError('a remembered delimiter tag must consume input');
  const labels = `${name}_labels`;
  const closing = `(predicate ${tag} (equal (matched) (top ${labels})))`;
  return `(scanner ${name} (tokens ${contentToken}) (operations (if (not (valid ${contentToken})) (then fail)) (if (equal (depth ${labels}) (integer 0)) (then fail)) (while (not atEnd) (do (if (next ${closing}) (then mark (emit ${contentToken}))) advance)) fail))\n`
    + `(rule ${startToken} token ${tag} (action (push ${labels} (matched))))\n`
    + `(rule ${endToken} token ${closing} (action (pop ${labels})))\n`;
}

/** A whole tagged literal, with separate helper rules for remembered labels. */
export function rememberedLiteralScanner({ name, token, startToken, contentToken, endToken, tagPattern, excludedLabels = null }) {
  identifier(token);
  if ([startToken, contentToken, endToken].includes(token)) throw new TypeError('a remembered literal needs a distinct whole token');
  if (excludedLabels !== null) identifier(excludedLabels);
  let generated = rememberedDelimiterScanner({ name, startToken, contentToken, endToken, tagPattern });
  if (excludedLabels !== null) {
    const tag = renderTreeSitterPattern(parseTreeSitterPattern(tagPattern));
    generated = generated.replace(`(rule ${startToken} token ${tag} `, `(rule ${startToken} token (predicate ${tag} (not (equal (matched) (top ${excludedLabels})))) `);
  }
  return `${generated}(rule ${token} token (seq (ref ${startToken}) (ref ${contentToken}) (ref ${endToken})))\n`;
}

/** One delimiter token serves both ends, with grammar-owned surrounding text. */
export function rememberedContentScanner({ name, delimiterToken, contentToken, delimiterPattern, closingPrefix, closingSuffix, allowEmpty = false, allowEnd = false }) {
  for (const value of [name, delimiterToken, contentToken]) identifier(value);
  if (delimiterToken === contentToken || typeof delimiterPattern !== 'string' || !delimiterPattern || typeof allowEmpty !== 'boolean' || typeof allowEnd !== 'boolean') throw new TypeError('remembered content needs distinct tokens, a pattern and boolean policies');
  const prefix = delimiter(closingPrefix);
  const suffix = delimiter(closingSuffix);
  const pattern = renderTreeSitterPattern(parseTreeSitterPattern(delimiterPattern));
  if (new RegExp(`^(?:${delimiterPattern})$`, 'u').test('')) throw new TypeError('a remembered content delimiter must consume input');
  const labels = `${name}_labels`;
  const absent = `(equal (depth ${labels}) (integer 0))`;
  const present = `(greater (depth ${labels}) (integer 0))`;
  const closingLabel = `(predicate ${pattern} (all ${present} (equal (matched) (top ${labels}))))`;
  const closing = `(choice unordered (seq ${prefix} ${closingLabel} ${suffix})${allowEmpty ? ` (predicate (seq ${prefix} ${suffix}) ${absent})` : ''})`;
  return `(scanner ${name} (tokens ${contentToken}) (operations (if (not (valid ${contentToken})) (then fail)) (while (not atEnd) (do (if (next ${closing}) (then mark (emit ${contentToken}))) advance)) ${allowEnd ? `mark (emit ${contentToken})` : 'fail'}))\n`
    + `(rule ${delimiterToken} token (choice ordered ${closingLabel} (predicate ${pattern} ${absent})) (action (if ${absent} (then (push ${labels} (matched))) (else (pop ${labels})))))\n`;
}

/** A complete delimiter token, optionally nesting and escaping its delimiters. */
export function delimitedScanner({ name, token, opening, closing, nested = false, escape = null, openingLookahead = null, rejectOpeningLookahead = false, rejected = [] }) {
  identifier(name);
  identifier(token);
  const open = delimiter(opening);
  const close = delimiter(closing);
  if (typeof rejectOpeningLookahead !== 'boolean' || !Array.isArray(rejected)) throw new TypeError('delimiter lookahead and rejection policies must be valid');
  const guard = openingLookahead === null ? '' : `(if ${rejectOpeningLookahead ? `(next ${delimiter(openingLookahead)})` : `(not (next ${delimiter(openingLookahead)}))`} (then fail)) `;
  const reject = rejected.length ? `(if (some ${rejected.map((text) => `(next ${delimiter(text)})`).join(' ')}) (then fail)) ` : '';
  if (nested && opening === closing) throw new TypeError('nested delimiters must differ');
  const escaped = escape === null ? '' : `(if (next ${delimiter(escape)}) (then (consume ${literal(escape)}) advance) (else `;
  const nesting = nested ? `(if (next ${open}) (then (consume ${open}) (push levels (integer 1))) (else advance))` : 'advance';
  const body = `(if (next ${close}) (then (consume ${close}) (pop levels)) (else ${nesting}))`;
  return `(scanner ${name} (tokens ${token}) (operations (if (not (valid ${token})) (then fail)) (consume ${open}) ${guard}(push levels (integer 1)) (while (greater (depth levels) (integer 0)) (do (if atEnd (then fail)) ${reject}${escaped}${body}${escaped ? '))' : ''})) (emit ${token})))\n`;
}

/** A nonempty fragment with paired prefix characters and explicit invalid boundaries. */
export function fragmentScanner({ name, token, stops, rejected = ['\0'], pairedPrefixes = [], requiredMarker = null, allowEnd = false }) {
  identifier(name);
  identifier(token);
  if (!Array.isArray(stops) || !stops.length || !Array.isArray(rejected) || !Array.isArray(pairedPrefixes) || typeof allowEnd !== 'boolean') throw new TypeError('fragment boundaries and policies must be valid');
  const stop = `(some atEnd ${stops.map((text) => `(next ${delimiter(text)})`).join(' ')})`;
  const reject = rejected.length ? `(if (some ${rejected.map((text) => `(next ${delimiter(text)})`).join(' ')}) (then fail)) ` : '';
  let advance = 'advance';
  for (const { prefix, except } of pairedPrefixes.toReversed()) {
    if (!Array.isArray(except) || !except.length) throw new TypeError('paired prefixes need following delimiters');
    const exceptions = `(some atEnd ${except.map((text) => `(next ${delimiter(text)})`).join(' ')})`;
    advance = `(if (next ${delimiter(prefix)}) (then (consume ${delimiter(prefix)}) (if (not ${exceptions}) (then advance))) (else ${advance}))`;
  }
  const marker = requiredMarker === null ? '' : `(if (next ${delimiter(requiredMarker)}) (then (push encountered (integer 1)))) `;
  const require = requiredMarker === null ? '' : '(if (equal (depth encountered) (integer 0)) (then fail)) (while (greater (depth encountered) (integer 0)) (do (pop encountered))) ';
  return `(scanner ${name} (tokens ${token}) (operations (if (not (valid ${token})) (then fail)) (if ${stop} (then fail)) (while (not ${stop}) (do ${reject}${marker}${advance})) ${allowEnd ? '' : '(if atEnd (then fail)) '}${require}(emit ${token})))\n`;
}

/** An external lexical rule with optional following context, expressed entirely in grammar data. */
export function patternTokenScanner({ name, token, pattern, before = null, excludedBefore = null, excludedAtStart = null }) {
  identifier(name);
  identifier(token);
  if (typeof pattern !== 'string' || !pattern || new RegExp(`^(?:${pattern})$`, 'u').test('')) throw new TypeError('lexical tokens need a consuming pattern');
  const body = renderTreeSitterPattern(parseTreeSitterPattern(pattern));
  const context = [before === null ? '' : `(and ${renderTreeSitterPattern(parseTreeSitterPattern(before))})`, excludedBefore === null ? '' : `(not ${renderTreeSitterPattern(parseTreeSitterPattern(excludedBefore))})`].filter(Boolean);
  const prefix = excludedAtStart === null ? '' : `(not ${renderTreeSitterPattern(parseTreeSitterPattern(excludedAtStart))}) `;
  return `(rule ${token} token ${context.length || prefix ? `(seq ${prefix}${body}${context.length ? ` ${context.join(' ')}` : ''})` : body})\n`;
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
export function splitCountedDelimiterScanner({ name, startToken, contentToken, endToken, prefix = '', marker, opening, closing, suffix = '', skipWhitespace = false, contentStops = [], countModulo = null, allowEnd = false, skipContentWhitespace = false }) {
  for (const value of [name, startToken, contentToken, endToken]) identifier(value);
  if (countModulo !== null && (!Number.isSafeInteger(countModulo) || countModulo < 2)) throw new TypeError('a scanner count modulus must be an integer of at least two');
  if (typeof allowEnd !== 'boolean' || typeof skipContentWhitespace !== 'boolean') throw new TypeError('counted content policies must be boolean');
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
  const whitespace = '(while (next (class plain (char %20) (char %09) (char %0A) (char %0B) (char %0C) (char %0D))) (do (skip (class plain (char %20) (char %09) (char %0A) (char %0B) (char %0C) (char %0D))))) ';
  const skip = skipWhitespace ? whitespace : '';
  const contentSkip = skipContentWhitespace ? whitespace : '';
  const collect = `(while (next ${repeated}) (do (consume ${repeated}) ${push('candidate')}))`;
  const start = `(if (valid ${startToken}) (then ${skip}${beginning}(while (next ${repeated}) (do (consume ${repeated}) ${push('delimiters')})) (consume ${open}) (emit ${startToken})))`;
  const content = `(if (valid ${contentToken}) (then ${contentSkip}(while (not atEnd) (do ${stop}(if (next ${close}) (then mark (consume ${close}) ${collect} (if (all (equal ${seen} ${markers})${suffixCondition}) (then ${clear('candidate')} (emit ${contentToken}))) ${clear('candidate')}) (else advance)))) ${allowEnd ? `mark (emit ${contentToken})` : 'fail'}))`;
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

/** Zero-width layout tokens retain bounded indentation stacks across lines. */
export function indentationScanner({ name, newlineToken, indentToken, dedentToken, tabWidth = 8, countModulo = 65536, commentPrefix = '#', continuation = '\\', resetCharacters = ['\r', '\f'], commentTokens = [], commentLiterals = [], bracketClosers = [')', ']', '}'], stringStartToken = null, stringContentToken = null, quoteCharacters = ['"', "'", '`'], stringPrefixCharacters = [], interpolationVariable = null }) {
  for (const value of [name, newlineToken, indentToken, dedentToken, ...commentTokens]) identifier(value);
  for (const value of [stringStartToken, stringContentToken, interpolationVariable]) if (value !== null) identifier(value);
  if (new Set([newlineToken, indentToken, dedentToken]).size !== 3 || !Number.isSafeInteger(tabWidth) || tabWidth < 1 || !Number.isSafeInteger(countModulo) || countModulo <= tabWidth) throw new TypeError('indentation needs distinct tokens and positive bounded widths');
  const resets = characterClass(resetCharacters);
  const quotes = characterClass(quoteCharacters);
  const stringOpening = stringPrefixCharacters.length ? `(seq (repeat0 ${characterClass(stringPrefixCharacters)}) ${quotes})` : quotes;
  const comment = delimiter(commentPrefix);
  const escape = delimiter(continuation);
  const available = (token) => [newlineToken, indentToken, dedentToken].includes(token) ? `(valid ${token})` : `(expected (ref ${token}))`;
  const stack = `${name}_indentations`;
  const width = `${name}_indentation`;
  const found = `${name}_line_end`;
  const firstComment = `${name}_comment_indentation`;
  const value = (variable) => `(variable ${variable})`;
  const set = (variable, expression) => `(set ${variable} ${expression})`;
  const zero = '(integer 0)';
  const commentsAvailable = `(some ${[indentToken, dedentToken, newlineToken, ...commentTokens].map(available).join(' ')}${commentLiterals.map((text) => ` (expected ${delimiter(text)})`).join('')})`;
  const commentAhead = `(all (next ${comment}) ${commentsAvailable})`;
  const whitespace = `(some (next (literal %0A)) (next (literal %20)) (next (literal %09)) (next ${resets}) ${commentAhead} (next ${escape}))`;
  const increment = (amount) => `${set(width, `(add ${value(width)} (integer ${amount}))`)} (if (not (less ${value(width)} (integer ${countModulo}))) (then ${set(width, `(subtract ${value(width)} (integer ${countModulo}))`)})) advance`;
  const newline = `${set(found, '(integer 1)')} ${set(width, zero)} advance`;
  const commentBody = `(if (equal ${value(found)} ${zero}) (then fail)) (if (equal ${value(firstComment)} (integer -1)) (then ${set(firstComment, value(width))})) (while (all (not atEnd) (not (next (literal %0A)))) (do advance)) (if (not atEnd) (then advance)) ${set(width, zero)}`;
  const continuationBody = `(consume ${escape}) (if (next (literal %0D)) (then advance)) (if (not atEnd) (then (if (not (next (literal %0A))) (then fail)) advance))`;
  const scan = `(while (all (not atEnd) ${whitespace}) (do (if (next (literal %0A)) (then ${newline}) (else (if (next (literal %20)) (then ${increment(1)}) (else (if (next (literal %09)) (then ${increment(tabWidth)}) (else (if (next ${resets}) (then ${set(width, zero)} advance) (else (if ${commentAhead} (then ${commentBody}) (else ${continuationBody}))))))))))))`;
  const brackets = bracketClosers.length ? `(some ${bracketClosers.map((text) => `(expected ${delimiter(text)})`).join(' ')})` : '(equal (integer 0) (integer 1))';
  const startsString = stringStartToken === null ? '(equal (integer 0) (integer 1))' : `(all ${available(stringStartToken)} (next ${stringOpening}))`;
  const recovery = stringContentToken === null ? '' : ` (not (all ${available(stringContentToken)} ${available(indentToken)}))`;
  const outsideInterpolation = interpolationVariable === null ? '' : ` (equal (variable ${interpolationVariable}) (integer 0))`;
  const dedentAvailable = `(some ${available(dedentToken)} (all (not ${available(newlineToken)}) (not ${startsString}) (not ${brackets})))`;
  const emit = `(if (greater ${value(found)} ${zero}) (then (if (all ${available(indentToken)} (greater ${value(width)} (top ${stack}))) (then (push ${stack} ${value(width)}) (emit ${indentToken}))) (if (all ${dedentAvailable} (greater (depth ${stack}) ${zero}) (less ${value(width)} (top ${stack})) (less ${value(firstComment)} (top ${stack}))${outsideInterpolation}) (then (pop ${stack}) (emit ${dedentToken}))) (if (all ${available(newlineToken)}${recovery}) (then (emit ${newlineToken})))))`;
  return `(scanner ${name} (tokens ${newlineToken} ${indentToken} ${dedentToken}) (operations mark ${set(found, zero)} ${set(width, zero)} ${set(firstComment, '(integer -1)')} ${scan} (if atEnd (then ${set(found, '(integer 1)')} ${set(width, zero)})) ${emit} fail))\n`;
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

/** Prefix flags and quote counts remain inspectable scanner state. */
export function prefixedQuotedScanner({ name, startToken, contentToken, endToken, interpolationEscapeToken, quotes = [{ text: "'", triple: true }, { text: '"', triple: true }, { text: '`', triple: false }], rawPrefixes = ['r', 'R'], bytesPrefixes = ['b', 'B'], interpolationPrefixes = ['f', 'F', 't', 'T'], ignoredPrefixes = ['u', 'U'], interpolationCharacters = ['{', '}'], ordinaryEscapePrefixes = ['N', 'u', 'U'], interpolationVariable = null, recoveryTokens = [] }) {
  for (const value of [name, startToken, contentToken, endToken, interpolationEscapeToken, ...recoveryTokens]) identifier(value);
  if (new Set([startToken, contentToken, endToken, interpolationEscapeToken]).size !== 4 || !Array.isArray(quotes) || !quotes.length || new Set(quotes.map(({ text }) => text)).size !== quotes.length) throw new TypeError('prefixed quotes need distinct tokens and delimiters');
  for (const quote of quotes) if (typeof quote.triple !== 'boolean' || typeof quote.text !== 'string' || [...quote.text].length !== 1) throw new TypeError('quote delimiters need one character and a triple policy');
  if (interpolationVariable !== null) identifier(interpolationVariable);
  const prefixCharacters = [...rawPrefixes, ...bytesPrefixes, ...interpolationPrefixes, ...ignoredPrefixes];
  if (new Set(prefixCharacters).size !== prefixCharacters.length) throw new TypeError('prefix flag characters must be disjoint');
  const characters = (items) => items.length ? `(next ${characterClass(items)})` : '(equal (integer 0) (integer 1))';
  const raw = `${name}_raw`;
  const bytes = `${name}_bytes`;
  const formatted = `${name}_formatted`;
  const triple = `${name}_triple`;
  const endings = `${name}_quotes`;
  const hasContent = `${name}_content`;
  const set = (variable, number) => `(set ${variable} (integer ${number}))`;
  const flag = (stack) => `(equal (top ${stack}) (integer 1))`;
  const close = `(some ${quotes.map(({ text }) => `(all (equal (top ${endings}) (text ${encode(text)})) (next ${literal(text)}))`).join(' ')})`;
  const pop = [raw, bytes, formatted, triple, endings].map((stack) => `(pop ${stack})`).join(' ');
  const content = `(if (equal (variable ${hasContent}) (integer 0)) (then fail)) mark (emit ${contentToken})`;
  // Prefix flags are local to the opener; leaving them behind makes completed
  // strings with different prefixes look like different scanner states. Restore
  // the enclosing interpolation flag after closing a nested string.
  const finish = `${pop} ${[raw, bytes, formatted, triple].map((variable) => set(variable, 0)).join(' ')}${interpolationVariable === null ? '' : ` (set ${interpolationVariable} (top ${formatted}))`} mark (emit ${endToken})`;
  const recovery = recoveryTokens.length ? `(some ${recoveryTokens.map((token) => `(expected (ref ${token}))`).join(' ')})` : '(equal (integer 0) (integer 1))';
  const escapedInterpolation = interpolationCharacters.map((text) => `(if (next ${literal(text + text)}) (then (consume ${literal(text + text)}) mark (emit ${interpolationEscapeToken})))`).join(' ');
  const escape = `(if (valid ${interpolationEscapeToken}) (then (if (all (greater (depth ${endings}) (integer 0)) ${flag(formatted)} (not ${recovery})) (then ${escapedInterpolation})) fail))`;
  const prefixes = prefixCharacters.length ? `(while ${characters(prefixCharacters)} (do (if ${characters(rawPrefixes)} (then ${set(raw, 1)})) (if ${characters(bytesPrefixes)} (then ${set(bytes, 1)})) (if ${characters(interpolationPrefixes)} (then ${set(formatted, 1)})) advance))` : '';
  // An empty quoted literal has exactly two quotes: only consume additional
  // opening delimiters after looking ahead for the full *pair* that makes a
  // triple quote. Otherwise the closing quote is swallowed into the opener.
  const opening = quotes.map(({ text, triple: canTriple }) => `(if (next ${literal(text)}) (then (push ${endings} (text ${encode(text)})) advance mark ${canTriple ? `(if (next ${literal(text + text)}) (then (consume ${literal(text + text)}) mark ${set(triple, 1)})) ` : ''}${[raw, bytes, formatted, triple].map((stack) => `(push ${stack} (variable ${stack}))`).join(' ')}${interpolationVariable === null ? '' : ` (set ${interpolationVariable} (variable ${formatted}))`} (emit ${startToken})))`).join(' ');
  const start = `(if (valid ${startToken}) (then ${[raw, bytes, formatted, triple].map((variable) => set(variable, 0)).join(' ')} ${prefixes} ${opening} fail))`;
  const rawEscape = `advance (if (some ${close} (next (literal %5C))) (then advance)) (if (next (literal %0D)) (then advance (if (next (literal %0A)) (then advance))) (else (if (next (literal %0A)) (then advance))))`;
  const bytesEscape = `mark advance (if ${characters(ordinaryEscapePrefixes)} (then advance advance ${set(hasContent, 1)}) (else ${content.replace(' mark (emit', ' (emit')}))`;
  const closing = `(if ${flag(triple)} (then mark advance (if ${close} (then advance (if ${close} (then (if (greater (variable ${hasContent}) (integer 0)) (then (emit ${contentToken})) (else advance ${finish}))) (else mark (emit ${contentToken})))) (else mark (emit ${contentToken})))) (else (if (greater (variable ${hasContent}) (integer 0)) (then mark (emit ${contentToken})) (else advance ${finish}))))`;
  const body = `(if (all (some (valid ${contentToken}) (valid ${endToken})) (greater (depth ${endings}) (integer 0)) (not ${recovery})) (then ${set(hasContent, 0)} (while (all (not atEnd) (not (next (literal %00)))) (do (if (all ${flag(formatted)} ${characters(interpolationCharacters)}) (then ${content})) (if (next (literal %5C)) (then (if ${flag(raw)} (then ${rawEscape}) (else (if ${flag(bytes)} (then ${bytesEscape}) (else ${content}))))) (else (if ${close} (then ${closing}) (else (if (all (next (literal %0A)) (greater (variable ${hasContent}) (integer 0)) (not ${flag(triple)})) (then fail)) advance ${set(hasContent, 1)})))))) fail))`;
  return `(scanner ${name} (tokens ${startToken} ${contentToken} ${endToken} ${interpolationEscapeToken}) (operations ${escape} ${body} ${start} fail))\n`;
}

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
  const generated = descriptors.map(({ family, ...options }) => {
    const declared = family === 'template-context' ? [options.quotedStartToken, options.quotedEndToken, options.contentToken, options.interpolationStartToken, options.interpolationEndToken, options.directiveStartToken, options.directiveEndToken, options.delimiterToken]
      : family === 'prefixed-quoted' ? [options.startToken, options.contentToken, options.endToken, options.interpolationEscapeToken]
      : family === 'indentation' ? [options.newlineToken, options.indentToken, options.dedentToken]
      : family === 'remembered-content' ? [options.delimiterToken, options.contentToken]
      : family === 'remembered-literal' ? [options.token, options.startToken, options.contentToken, options.endToken]
      : ['split-counted-delimiter', 'remembered-delimiter'].includes(family) ? [options.startToken, options.contentToken, options.endToken]
      : family === 'delimiter-run' ? [options.contentToken, options.endToken]
        : [options.token, ...(options.closeToken ? [options.closeToken] : [])];
    if (names.has(options.name) || new Set(declared).size !== declared.length || declared.some((token) => tokens.has(token))) throw new TypeError('duplicate scanner name or token');
    names.add(options.name);
    for (const token of declared) tokens.add(token);
    if (family === 'template-context') return templateContextScanner(options);
    if (family === 'remembered-delimiter') return rememberedDelimiterScanner(options);
    if (family === 'remembered-literal') return rememberedLiteralScanner(options);
    if (family === 'remembered-content') return rememberedContentScanner(options);
    if (family === 'delimited') return delimitedScanner(options);
    if (family === 'fragment') return fragmentScanner(options);
    if (family === 'pattern-token') return patternTokenScanner(options);
    if (family === 'content') return contentScanner(options);
    if (family === 'counted-delimiter') return countedDelimiterScanner(options);
    if (family === 'split-counted-delimiter') return splitCountedDelimiterScanner(options);
    if (family === 'delimiter-run') return delimiterRunScanner(options);
    if (family === 'line-boundary') return lineBoundaryScanner(options);
    if (family === 'indentation') return indentationScanner(options);
    if (family === 'prefixed-quoted') return prefixedQuotedScanner(options);
    if (family === 'lookahead-boundary') return lookaheadBoundaryScanner(options);
    if (family === 'context-token') return contextTokenScanner(options);
    if (family === 'line-counted-delimiter') return lineCountedDelimiterScanner(options);
    throw new TypeError(`unknown scanner family ${JSON.stringify(family)}`);
  }).join('');
  const lines = generated.split('\n').filter(Boolean);
  return [...lines.filter((line) => !line.startsWith('(rule ')), ...lines.filter((line) => line.startsWith('(rule '))].map((line) => `${line}\n`).join('');
}
