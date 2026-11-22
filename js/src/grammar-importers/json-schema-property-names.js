import { GrammarBuilder as Expression } from '../grammar.js';
import { intersectScalarIntervals, surrogatePairBoundaries } from './json-schema-pattern-policy.js';

const { charClass, seq, choice, literal, repeat, repeat0, empty } = Expression;
const range = (start, end) => ({ kind: 'range', start, end });
const character = value => ({ kind: 'char', value });
const hex = '0123456789abcdef';
const hexDigits = (first, last) => charClass(Array.from(hex.slice(first, last + 1)).flatMap(digit =>
  digit >= 'a' ? [character(digit), character(digit.toUpperCase())] : [character(digit)]));

const shortEscapes = new Map([
  ['"', '"'], ['\\', '\\'], ['/', '/'], ['\b', 'b'], ['\u000c', 'f'], ['\n', 'n'], ['\r', 'r'], ['\t', 't'],
]);

const directExclusions = [character('"'), character('\\'), range('\u0000', '\u001f')];

function hexUnit(unit) {
  return seq(literal('\\u'), ...Array.from(unit.toString(16).padStart(4, '0'), digit => {
    const index = hex.indexOf(digit);
    return hexDigits(index, index);
  }));
}

function exactCharacter(value) {
  const escaped = seq(...Array.from({ length: value.length }, (_, index) => hexUnit(value.charCodeAt(index))));
  const spellings = [escaped];
  if (value.codePointAt(0) >= 32 && value !== '"' && value !== '\\') spellings.unshift(literal(value));
  if (shortEscapes.has(value)) spellings.push(literal(`\\${shortEscapes.get(value)}`));
  return choice(...spellings);
}

function hexInterval(first, last, digits = 4) {
  if (first === 0 && last === 16 ** digits - 1) return repeat(hexDigits(0, 15), digits, digits);
  if (digits === 1) return hexDigits(first, last);
  const base = 16 ** (digits - 1);
  const lower = Math.floor(first / base);
  const upper = Math.floor(last / base);
  if (lower === upper) return seq(hexDigits(lower, lower), hexInterval(first % base, last % base, digits - 1));
  const alternatives = [seq(hexDigits(lower, lower), hexInterval(first % base, base - 1, digits - 1))];
  if (lower + 1 < upper) alternatives.push(seq(hexDigits(lower + 1, upper - 1), repeat(hexDigits(0, 15), digits - 1, digits - 1)));
  alternatives.push(seq(hexDigits(upper, upper), hexInterval(0, last % base, digits - 1)));
  return choice(...alternatives);
}

function unicodeInterval(first, last) {
  return seq(literal('\\u'), hexInterval(first, last));
}

function surrogatePairInterval(first, last) {
  const [firstHigh, firstLow, lastHigh, lastLow] = surrogatePairBoundaries(first, last);
  if (firstHigh === lastHigh) return seq(hexUnit(firstHigh), unicodeInterval(firstLow, lastLow));
  const alternatives = [
    seq(hexUnit(firstHigh), unicodeInterval(firstLow, 0xdfff)),
    seq(hexUnit(lastHigh), unicodeInterval(0xdc00, lastLow)),
  ];
  if (firstHigh + 1 < lastHigh) {
    alternatives.push(seq(unicodeInterval(firstHigh + 1, lastHigh - 1), unicodeInterval(0xdc00, 0xdfff)));
  }
  return choice(...alternatives);
}

/** Match every JSON spelling of one Unicode scalar in the given intervals. */
export function jsonStringCharacter(ranges) {
  const direct = [];
  const escaped = [];
  for (const [first, last] of ranges) {
    if (first === last) {
      escaped.push(exactCharacter(String.fromCodePoint(first)));
      continue;
    }
    for (const [low, high] of [[32, 33], [35, 91], [93, 0xd7ff], [0xe000, 0x10ffff]]) {
      const [start, end] = intersectScalarIntervals(first, last, low, high);
      if (start <= end) direct.push(range(String.fromCodePoint(start), String.fromCodePoint(end)));
    }
    for (const [low, high] of [[0, 0xd7ff], [0xe000, 0xffff], [0x10000, 0x10ffff]]) {
      const [start, end] = intersectScalarIntervals(first, last, low, high);
      if (start <= end) escaped.push(start < 0x10000 ? unicodeInterval(start, end) : surrogatePairInterval(start, end));
    }
    for (const [value, escape] of shortEscapes) {
      const point = value.codePointAt(0);
      if (first <= point && point <= last) escaped.push(literal(`\\${escape}`));
    }
  }
  if (direct.length) escaped.unshift(charClass(direct));
  return escaped.length ? choice(...escaped) : null;
}

function unitsExcept(first, last, excluded) {
  const intervals = [];
  let start = first;
  for (const unit of [...excluded].sort((left, right) => left - right)) {
    if (unit < start || unit > last) continue;
    if (unit > start) intervals.push(hexInterval(start, unit - 1));
    start = unit + 1;
  }
  if (start <= last) intervals.push(hexInterval(start, last));
  return intervals.length ? seq(literal('\\u'), choice(...intervals)) : null;
}

function differentCharacter(excluded) {
  const direct = charClass([...directExclusions, ...Array.from(excluded, character)], true);
  const abbreviations = [...shortEscapes].filter(([value]) => !excluded.has(value)).map(([, escape]) => character(escape));
  const basicPlane = new Set();
  const pairs = new Map();
  for (const value of excluded) {
    const high = value.charCodeAt(0);
    if (value.length === 1) basicPlane.add(high);
    else {
      if (!pairs.has(high)) pairs.set(high, new Set());
      pairs.get(high).add(value.charCodeAt(1));
    }
  }
  const alternatives = [direct, unitsExcept(0, 0xd7ff, basicPlane), unitsExcept(0xe000, 0xffff, basicPlane)];
  if (abbreviations.length) alternatives.push(seq(literal('\\'), charClass(abbreviations)));
  const otherHigh = unitsExcept(0xd800, 0xdbff, pairs.keys());
  if (otherHigh) alternatives.push(seq(otherHigh, unitsExcept(0xdc00, 0xdfff, [])));
  for (const [high, lows] of pairs) {
    const otherLow = unitsExcept(0xdc00, 0xdfff, lows);
    if (otherLow) alternatives.push(seq(hexUnit(high), otherLow));
  }
  return choice(...alternatives.filter(Boolean));
}

/** Match a JSON key that decodes to something other than any declared key. */
export function additionalPropertyName(keys) {
  const root = { terminal: false, children: new Map() };
  for (const key of keys) {
    let node = root;
    for (const value of key) {
      if (!node.children.has(value)) node.children.set(value, { terminal: false, children: new Map() });
      node = node.children.get(value);
    }
    node.terminal = true;
  }
  const arbitraryTail = repeat0(differentCharacter(new Set()));
  function remaining(node) {
    const alternatives = [seq(differentCharacter(new Set(node.children.keys())), arbitraryTail)];
    if (!node.terminal) alternatives.push(empty());
    for (const [value, child] of node.children) alternatives.push(seq(exactCharacter(value), remaining(child)));
    return choice(...alternatives);
  }
  return seq(literal('"'), remaining(root), literal('"'));
}
