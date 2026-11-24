import { GrammarBuilder } from '../grammar.js';
import { CharStream, errorAt, readEscape } from './lark-lexer.js';
import {
  acceptLarkRegexFlag, acceptLarkRegexLiteralEscape, acceptLarkRegexMetaCharacter,
  acceptLarkRegexQuantifier,
} from './lark-regex-policy.js';

const escapeControls = { a: '\x07', v: '\x0b' };
const character = (value) => ({ kind: 'char', value });
const range = (start, end) => ({ kind: 'range', start, end });
const category = (value) => ({ kind: 'category', value });
const classes = {
  d: [category('Nd')],
  s: [range('\x09', '\x0d'), range('\x1c', '\x20'), character('\x85'), character('\xa0'),
    character('\u1680'), range('\u2000', '\u200a'), range('\u2028', '\u2029'),
    character('\u202f'), character('\u205f'), character('\u3000')],
  w: [category('L'), category('N'), character('_')],
};
const caseGroups = ['Iiİı', 'KkK', 'Ssſ'];

/** Lowers the supported, greedy, scalar-based Lark regex terminals. */
export function lowerLarkRegex(value, flags, offset) {
  const stream = new CharStream(value);
  const base = offset + 1;
  const flagsStart = base + stream.bytes.at(-1) + 1;
  for (const [index, flag] of Array.from(flags).entries()) {
    if (!acceptLarkRegexFlag(flag)) {
      throw errorAt(flagsStart + index, `unsupported regex flag ${flag}`);
    }
  }
  let insensitive = flags.includes('i');
  const dotAll = flags.includes('s');
  const scoped = stream.startsWith('(?i:') || stream.startsWith('(?:');
  if (scoped) {
    insensitive ||= stream.startsWith('(?i:');
    stream.cursor += stream.startsWith('(?i:') ? 4 : 3;
  }

  const items = [];
  let variable = false;
  let required = false;
  while (!stream.isEnd() && !(scoped && stream.peekChar() === ')')) {
    const position = base + stream.offset;
    const head = stream.advanceChar();
    let item;
    if (head === '[') {
      item = readClass(stream, base, position, insensitive);
    } else if (head === '.') {
      item = GrammarBuilder.charClass(dotAll ? [range('\0', '\u{10ffff}')]
        : [range('\0', '\x09'), range('\x0b', '\u{10ffff}')]);
    } else if (head === '\\') {
      const escape = readRegexEscape(stream, base, position, false);
      item = escape.items
        ? GrammarBuilder.charClass(escape.items, escape.negated)
        : literal(escape.value, insensitive, position);
    } else {
      if (acceptLarkRegexMetaCharacter(head)) {
        throw errorAt(position, `unsupported regex construct ${head}`);
      }
      item = literal(head, insensitive, position);
    }

    const quantifier = stream.peekChar() ?? '';
    if (acceptLarkRegexQuantifier(quantifier)) {
      const quantifierOffset = base + stream.offset;
      if (variable) throw errorAt(quantifierOffset, 'unsupported second regex quantifier');
      variable = true;
      stream.advanceChar();
      item = quantifier === '+' ? GrammarBuilder.repeat1(item)
        : quantifier === '*' ? GrammarBuilder.repeat0(item) : GrammarBuilder.optional(item);
      required ||= quantifier === '+';
    } else {
      required = true;
    }
    items.push(item);
  }
  if (scoped && (stream.advanceChar() !== ')' || !stream.isEnd())) {
    throw errorAt(base + stream.offset, 'unsupported regex group boundary');
  }
  if (scoped && items.length === 0) return GrammarBuilder.empty();
  if (!required) throw errorAt(offset, 'regex terminal must consume input');
  const expression = GrammarBuilder.seq(...items);
  // A regex terminal is one token even when it contains several grammar
  // expressions. Only the longest result is visible to the enclosing rule.
  return items.length === 1 && !variable ? expression : { kind: 'token', item: expression };
}

function readClass(stream, base, position, insensitive) {
  const negated = stream.peekChar() === '^';
  if (negated) stream.advanceChar();
  const items = [];
  while (!stream.isEnd() && stream.peekChar() !== ']') {
    const startPosition = base + stream.offset;
    const start = readClassItem(stream, base, startPosition);
    if (stream.peekChar() === '-' && ![']', undefined].includes(stream.peekChar(1))) {
      stream.advanceChar();
      const end = readClassItem(stream, base, base + stream.offset);
      if (start.items || end.items || start.value.codePointAt(0) > end.value.codePointAt(0)) {
        throw errorAt(startPosition, 'invalid regex character class range');
      }
      items.push(...caseItems(start.value, end.value, insensitive, startPosition, true));
    } else if (start.items) {
      items.push(...start.items);
    } else {
      items.push(...caseItems(start.value, start.value, insensitive, startPosition));
    }
  }
  if (items.length === 0 || stream.advanceChar() !== ']') {
    throw errorAt(position, 'empty or unterminated regex character class');
  }
  return GrammarBuilder.charClass(items, negated);
}

function readClassItem(stream, base, position) {
  const value = stream.advanceChar();
  if (value === '[') throw errorAt(position, 'unsupported nested regex character class');
  return value === '\\' ? readRegexEscape(stream, base, position, true) : { value };
}

function readRegexEscape(stream, base, position, insideClass) {
  const escape = stream.peekChar();
  if (escape === undefined) throw errorAt(position, 'unterminated regex escape');
  const lower = escape.toLowerCase();
  if (Object.hasOwn(classes, lower)) {
    stream.advanceChar();
    if (escape !== lower && insideClass) {
      throw errorAt(position, `unsupported complemented class escape \\${escape}`);
    }
    return { items: classes[lower], negated: escape !== lower };
  }
  if (Object.hasOwn(escapeControls, escape)) {
    stream.advanceChar();
    return { value: escapeControls[escape] };
  }
  if ('nrtfxuU'.includes(escape) || (insideClass && escape === 'b')
    || acceptLarkRegexLiteralEscape(escape.codePointAt(0))) {
    return { value: readEscape(stream, position, 'unterminated regex escape', base) };
  }
  throw errorAt(position, `unsupported regex escape \\${escape}`);
}

function literal(value, insensitive, position) {
  if (!insensitive) return GrammarBuilder.literal(value);
  return GrammarBuilder.charClass(caseItems(value, value, true, position));
}

function caseItems(start, end, insensitive, position, explicitRange = false) {
  const original = explicitRange ? range(start, end) : character(start);
  if (!insensitive) return [original];
  if (end.codePointAt(0) > 127) throw errorAt(position, 'unsupported non-ASCII case-insensitive regex range or literal');
  const items = [original];
  for (const [first, last, delta] of [[65, 90, 32], [97, 122, -32]]) {
    const lower = Math.max(start.codePointAt(0), first);
    const upper = Math.min(end.codePointAt(0), last);
    if (lower === upper) items.push(character(String.fromCodePoint(lower + delta)));
    else if (lower < upper) items.push(range(String.fromCodePoint(lower + delta), String.fromCodePoint(upper + delta)));
  }
  for (const group of caseGroups) {
    if (Array.from(group.slice(0, 2)).some((member) => start <= member && member <= end)) {
      for (const member of group.slice(2)) items.push(character(member));
    }
  }
  return items;
}
