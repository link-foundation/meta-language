// Imports a tree-sitter grammar.json as an executable native grammar: every
// construct becomes a native expression the executor runs, with no regular
// expression or foreign grammar text left. Unlike importTreeSitterJson, which
// keeps the source's structure for interchange, this import keeps tree-sitter's
// parse: the trees the native executor builds are the trees tree-sitter builds
// (docs/grammar/native-grammars.md, "Imported grammars").
//
// tree-sitter    native
// SYMBOL         (ref NAME)
// STRING         (literal TEXT); a keyword is closed by (not (ref WORD))
// PATTERN        the compiled pattern: classes, sequences, choices, repeats
// SEQ/CHOICE     (seq ...) / (choice unordered ...); BLANK makes (optional ...)
// REPEAT(1)      (repeat0 ...) / (repeat1 ...)
// PREC*          (precedence LEVEL none|left|right ...); in a token
//                (lexicalPrecedence LEVEL ...)
// PREC_DYNAMIC   (dynamicPrecedence LEVEL ...)
// TOKEN          (token ...); IMMEDIATE_TOKEN (immediateToken ...)
// FIELD          (capture labeled NAME ...)
// ALIAS          (alias NAME ...); an anonymous alias is named 'TEXT
// extras         (extra ...); conflicts (conflict ...)
// word           keyword extraction: the word rule does not take a keyword's
//                text unless no keyword parse exists (dynamic precedence -1)
import { compileGrammar } from '../grammar.js';
import { parseGrammarLinks } from '../grammar-links.js';
import { parseError } from './common.js';

const FORMAT = 'tree-sitter';

/** Percent-encodes a Links Notation word: bytes outside `[A-Za-z0-9._-]` as %XX. */
export const encodeLinksWord = (text) => [...new TextEncoder().encode(text)]
  .map((byte) => (/[A-Za-z0-9._-]/u.test(String.fromCharCode(byte)) ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`))
  .join('') || '%';
const enc = encodeLinksWord;

/** A pattern construct the native form has no counterpart for. */
export class UnsupportedTreeSitterPattern extends Error {}

// ---------------------------------------------------------------- patterns

// tree-sitter patterns are JavaScript regular expression sources. The parser
// below reads the subset grammars use into {alt|seq|repeat|class|char|and|not}.
const ID_START = ['Lu', 'Ll', 'Lt', 'Lm', 'Lo', 'Nl'];
const ID_CONTINUE = [...ID_START, 'Mn', 'Mc', 'Nd', 'Pc'];
const PROPERTY_CATEGORIES = {
  XID_Start: ID_START, ID_Start: ID_START, XID_Continue: ID_CONTINUE, ID_Continue: ID_CONTINUE,
  Alphabetic: [...ID_START], Letter: ['L'], L: ['L'], Lu: ['Lu'], Ll: ['Ll'], Lt: ['Lt'], Lm: ['Lm'], Lo: ['Lo'],
  Uppercase_Letter: ['Lu'], Lowercase_Letter: ['Ll'], Titlecase_Letter: ['Lt'], Modifier_Letter: ['Lm'], Other_Letter: ['Lo'],
  Uppercase: ['Lu'], Lowercase: ['Ll'],
  N: ['N'], Nd: ['Nd'], Nl: ['Nl'], No: ['No'], Number: ['N'], Decimal_Number: ['Nd'], Letter_Number: ['Nl'], Other_Number: ['No'],
  M: ['M'], Mn: ['Mn'], Mc: ['Mc'], Me: ['Me'], Mark: ['M'], Nonspacing_Mark: ['Mn'], Spacing_Mark: ['Mc'], Enclosing_Mark: ['Me'],
  P: ['P'], Pc: ['Pc'], Pd: ['Pd'], Ps: ['Ps'], Pe: ['Pe'], Pi: ['Pi'], Pf: ['Pf'], Po: ['Po'], Punctuation: ['P'],
  Connector_Punctuation: ['Pc'], Dash_Punctuation: ['Pd'],
  S: ['S'], Sm: ['Sm'], Sc: ['Sc'], Sk: ['Sk'], So: ['So'], Symbol: ['S'], Math_Symbol: ['Sm'], Currency_Symbol: ['Sc'],
  Z: ['Z'], Zs: ['Zs'], Zl: ['Zl'], Zp: ['Zp'], Separator: ['Z'], Space_Separator: ['Zs'], White_Space: ['Zs', 'Zl', 'Zp'],
  C: ['C'], Cc: ['Cc'], Cf: ['Cf'], Co: ['Co'], Cs: ['Cs'], Cn: ['Cn'], Control: ['Cc'], Format: ['Cf'], Other: ['C'],
  Emoji: ['So'], Emoji_Presentation: ['So'], Extended_Pictographic: ['So'], EMod: ['Sk'], Emoji_Modifier: ['Sk'],
};
const SCRIPTS = new Set([
  'Arabic', 'Armenian', 'Bengali', 'Cyrillic', 'Devanagari', 'Georgian', 'Greek', 'Han', 'Hangul', 'Hebrew', 'Hiragana',
  'Katakana', 'Latin', 'Thai',
]);
const SPACE_ITEMS = [...' \t\n\r\f\v ﻿  '].map((c) => ({ kind: 'char', value: c }))
  .concat([{ kind: 'category', value: 'Zs' }]);
const DIGIT_ITEMS = [{ kind: 'range', start: '0', end: '9' }];
const WORD_ITEMS = [{ kind: 'range', start: 'a', end: 'z' }, { kind: 'range', start: 'A', end: 'Z' }, ...DIGIT_ITEMS, { kind: 'char', value: '_' }];

/** Parses the regular expression `source` (a tree-sitter PATTERN) into a pattern tree. */
export function parseTreeSitterPattern(source, flags = '') {
  let index = 0;
  const peek = () => source[index];
  const eat = (c) => (source[index] === c ? (index += 1, true) : false);
  const fail = (why) => { throw new UnsupportedTreeSitterPattern(`${why} at ${index} in /${source}/`); };

  const codePoint = () => {
    const c = source.codePointAt(index);
    index += c > 0xffff ? 2 : 1;
    return String.fromCodePoint(c);
  };
  const hexDigits = (count) => {
    const text = source.slice(index, index + count);
    if (!/^[0-9a-fA-F]+$/u.test(text) || text.length !== count) fail('bad hex escape');
    index += count;
    return String.fromCodePoint(Number.parseInt(text, 16));
  };
  const property = (negated) => {
    let name;
    if (eat('{')) {
      const end = source.indexOf('}', index);
      if (end < 0) fail('unterminated property');
      name = source.slice(index, end);
      index = end + 1;
    } else {
      // \pL, \pN: a one-letter general category.
      name = codePoint();
    }
    name = name.replace(/^(?:General_Category|gc)=/u, '');
    const scriptName = name.replace(/^(?:Script|sc|Script_Extensions|scx)=/u, '');
    let items;
    if (SCRIPTS.has(scriptName)) items = [{ kind: 'script', value: scriptName }];
    else if (PROPERTY_CATEGORIES[name]) items = PROPERTY_CATEGORIES[name].map((value) => ({ kind: 'category', value }));
    else fail(`unknown property ${name}`);
    return { kind: 'class', negated, items };
  };
  // An escape outside or inside a class: a single character, or a class.
  const escape = (inClass) => {
    const c = codePoint();
    switch (c) {
      case 'd': return { kind: 'class', negated: false, items: DIGIT_ITEMS };
      case 'D': return { kind: 'class', negated: true, items: DIGIT_ITEMS };
      case 'w': return { kind: 'class', negated: false, items: WORD_ITEMS };
      case 'W': return { kind: 'class', negated: true, items: WORD_ITEMS };
      case 's': return { kind: 'class', negated: false, items: SPACE_ITEMS };
      case 'S': return { kind: 'class', negated: true, items: SPACE_ITEMS };
      case 'p': return property(false);
      case 'P': return property(true);
      case 'n': return { kind: 'char', value: '\n' };
      case 'r': return { kind: 'char', value: '\r' };
      case 't': return { kind: 'char', value: '\t' };
      case 'f': return { kind: 'char', value: '\f' };
      case 'v': return { kind: 'char', value: '\v' };
      case '0': return { kind: 'char', value: '\0' };
      case 'a': return { kind: 'char', value: '\x07' };
      case 'e': return { kind: 'char', value: '\x1b' };
      case 'b': if (inClass) return { kind: 'char', value: '\b' }; return fail('word boundary');
      case 'x': return { kind: 'char', value: hexDigits(2) };
      case 'u':
        if (eat('{')) {
          const end = source.indexOf('}', index);
          const value = String.fromCodePoint(Number.parseInt(source.slice(index, end), 16));
          index = end + 1;
          return { kind: 'char', value };
        }
        return { kind: 'char', value: hexDigits(4) };
      case 'c': return { kind: 'char', value: String.fromCharCode(codePoint().charCodeAt(0) % 32) };
      default:
        if (/[A-Za-z]/u.test(c) && !inClass) return fail(`escape \\${c}`);
        return { kind: 'char', value: c };
    }
  };
  const classBody = () => {
    const negated = eat('^');
    const items = [];
    const nested = [];
    while (index < source.length && peek() !== ']') {
      if (peek() === '&' && source[index + 1] === '&') fail('class intersection');
      let item;
      if (eat('\\')) item = escape(true);
      else if (peek() === '[' && source[index + 1] === ':') fail('posix class');
      else if (peek() === '[') { index += 1; item = classBody(); }
      else item = { kind: 'char', value: codePoint() };
      if (item.kind === 'char' && peek() === '-' && source[index + 1] !== ']' && source[index + 1] !== undefined) {
        index += 1;
        let end;
        if (eat('\\')) end = escape(true);
        else end = { kind: 'char', value: codePoint() };
        if (end.kind !== 'char') fail('range to a class');
        items.push({ kind: 'range', start: item.value, end: end.value });
      } else if (item.kind === 'char') items.push(item);
      else if (item.kind === 'class' && !item.negated) items.push(...item.items);
      else nested.push(item);
    }
    if (!eat(']')) fail('unterminated class');
    if (nested.length === 0) return { kind: 'class', negated, items };
    // [x\S] is x or not-space; [^x\S] is neither x nor not-space.
    const alternatives = nested.slice();
    if (items.length > 0) alternatives.unshift({ kind: 'class', negated: false, items });
    const union = alternatives.length === 1 ? alternatives[0] : { kind: 'alt', items: alternatives };
    if (!negated) return union;
    return { kind: 'seq', items: [{ kind: 'not', item: union }, { kind: 'class', negated: true, items: [] }] };
  };
  const atom = () => {
    const c = peek();
    if (c === '(') {
      index += 1;
      if (eat('?')) {
        if (eat(':')) { /* a group */ } else if (peek() === '=' || peek() === '!') {
          const negative = source[index] === '!';
          index += 1;
          const inner = alternation();
          if (!eat(')')) fail('unterminated lookahead');
          return { kind: negative ? 'not' : 'and', item: inner };
        } else if (peek() === '<' && source[index + 1] !== '=' && source[index + 1] !== '!') {
          // A named group (?<name>...) matches as a group.
          const end = source.indexOf('>', index);
          index = end + 1;
        } else fail('group flag');
      }
      const inner = alternation();
      if (!eat(')')) fail('unterminated group');
      return inner;
    }
    if (c === '[') { index += 1; return classBody(); }
    if (c === '.') { index += 1; return { kind: 'class', negated: true, items: [{ kind: 'char', value: '\n' }] }; }
    if (c === '\\') { index += 1; return escape(false); }
    if (c === '^' || c === '$') fail('anchor');
    return { kind: 'char', value: codePoint() };
  };
  const quantified = () => {
    let item = atom();
    for (;;) {
      let min; let max;
      if (eat('*')) { min = 0; max = null; } else if (eat('+')) { min = 1; max = null; } else if (eat('?')) { min = 0; max = 1; } else if (peek() === '{' && /^\{\d+(,\d*)?\}/u.test(source.slice(index))) {
        const match = /^\{(\d+)(,(\d*))?\}/u.exec(source.slice(index));
        index += match[0].length;
        min = Number(match[1]);
        max = match[2] === undefined ? min : (match[3] === '' ? null : Number(match[3]));
      } else break;
      eat('?'); // a lazy quantifier matches the same language
      item = { kind: 'repeat', min, max, item };
    }
    return item;
  };
  const sequence = () => {
    const items = [];
    while (index < source.length && peek() !== '|' && peek() !== ')') items.push(quantified());
    return items.length === 1 ? items[0] : { kind: 'seq', items };
  };
  function alternation() {
    const items = [sequence()];
    while (eat('|')) items.push(sequence());
    return items.length === 1 ? items[0] : { kind: 'alt', items };
  }
  const tree = alternation();
  if (index !== source.length) fail('trailing text');
  return flags.includes('i') ? caseFold(tree) : tree;
}

function caseFold(node) {
  switch (node.kind) {
    case 'char': {
      const lower = node.value.toLowerCase();
      const upper = node.value.toUpperCase();
      if (lower === upper) return node;
      return { kind: 'class', negated: false, items: [{ kind: 'char', value: lower }, { kind: 'char', value: upper }] };
    }
    case 'class': {
      const items = node.items.flatMap((item) => {
        if (item.kind === 'char') return [...new Set([item.value, item.value.toLowerCase(), item.value.toUpperCase()])].map((value) => ({ kind: 'char', value }));
        if (item.kind === 'range' && /^[a-z]$/u.test(item.start) && /^[a-z]$/u.test(item.end)) return [item, { kind: 'range', start: item.start.toUpperCase(), end: item.end.toUpperCase() }];
        if (item.kind === 'range' && /^[A-Z]$/u.test(item.start) && /^[A-Z]$/u.test(item.end)) return [item, { kind: 'range', start: item.start.toLowerCase(), end: item.end.toLowerCase() }];
        return [item];
      });
      return { ...node, items };
    }
    case 'seq': case 'alt': return { ...node, items: node.items.map(caseFold) };
    case 'repeat': case 'and': case 'not': return { ...node, item: caseFold(node.item) };
    default: return node;
  }
}

function classItem(item) {
  switch (item.kind) {
    case 'char': return `(char ${enc(item.value)})`;
    case 'range': return `(range ${enc(item.start)} ${enc(item.end)})`;
    case 'category': return `(category ${item.value})`;
    case 'script': return `(script ${item.value})`;
    default: throw new Error(`unknown class item ${item.kind}`);
  }
}

/** The native expression text of a pattern tree. Adjacent characters join into one literal. */
export function renderTreeSitterPattern(node) {
  switch (node.kind) {
    case 'char': return `(literal ${enc(node.value)})`;
    case 'class':
      if (node.items.length === 0) return node.negated ? 'any' : '(not empty)';
      return `(class ${node.negated ? 'negated' : 'plain'} ${node.items.map(classItem).join(' ')})`;
    case 'seq': {
      const parts = [];
      let text = '';
      for (const item of node.items) {
        if (item.kind === 'char') { text += item.value; continue; }
        if (text) { parts.push(`(literal ${enc(text)})`); text = ''; }
        parts.push(renderTreeSitterPattern(item));
      }
      if (text) parts.push(`(literal ${enc(text)})`);
      if (parts.length === 0) return 'empty';
      return parts.length === 1 ? parts[0] : `(seq ${parts.join(' ')})`;
    }
    case 'alt': return `(choice unordered ${node.items.map(renderTreeSitterPattern).join(' ')})`;
    case 'repeat': {
      const item = renderTreeSitterPattern(node.item);
      if (node.min === 0 && node.max === null) return `(repeat0 ${item})`;
      if (node.min === 1 && node.max === null) return `(repeat1 ${item})`;
      if (node.min === 0 && node.max === 1) return `(optional ${item})`;
      return `(repeat ${node.min} ${node.max === null ? 'unbounded' : node.max} ${item})`;
    }
    case 'and': return `(and ${renderTreeSitterPattern(node.item)})`;
    case 'not': return `(not ${renderTreeSitterPattern(node.item)})`;
    default: throw new Error(`unknown pattern node ${node.kind}`);
  }
}

// ---------------------------------------------------------------- grammar

const unwrapPrecedence = (node) => (node.type.startsWith('PREC') ? unwrapPrecedence(node.content) : node);
const isLexicalBody = (node) => ['STRING', 'PATTERN', 'TOKEN', 'IMMEDIATE_TOKEN'].includes(unwrapPrecedence(node).type);
const memberName = (member) => member.name ?? member.value ?? member;

// The finite set of texts a lexical expression matches, or null when it is
// not finite (or too large to list).
function finiteTexts(node, limit = 64) {
  switch (node.type) {
    case 'STRING': return [node.value];
    case 'BLANK': return [''];
    case 'TOKEN': case 'IMMEDIATE_TOKEN': case 'PREC': case 'PREC_LEFT': case 'PREC_RIGHT': case 'PREC_DYNAMIC':
    case 'FIELD': case 'ALIAS': case 'RESERVED':
      return finiteTexts(node.content, limit);
    case 'CHOICE': {
      const out = [];
      for (const member of node.members) {
        const texts = finiteTexts(member, limit);
        if (texts === null) return null;
        out.push(...texts);
      }
      return out.length > limit ? null : [...new Set(out)];
    }
    case 'SEQ': {
      let out = [''];
      for (const member of node.members) {
        const texts = finiteTexts(member, limit);
        if (texts === null) return null;
        out = out.flatMap((head) => texts.map((tail) => head + tail));
        if (out.length > limit) return null;
      }
      return out;
    }
    default: return null;
  }
}

// The tokens of the grammar: each STRING outside a token and each token
// (a TOKEN, an IMMEDIATE_TOKEN or a whole lexical rule) as one unit.
function lexicalUnits(node, out) {
  if (!node || typeof node !== 'object') return out;
  if (node.type === 'STRING' || node.type === 'TOKEN' || node.type === 'IMMEDIATE_TOKEN') {
    out.push(node);
    return out;
  }
  for (const value of Object.values(node)) {
    if (Array.isArray(value)) value.forEach((item) => lexicalUnits(item, out));
    else if (value && typeof value === 'object') lexicalUnits(value, out);
  }
  return out;
}

/**
 * Imports the parsed or textual grammar.json `source` as native rules.
 *
 * `nameOf(name)` gives the native name of a rule or a named alias
 * (default: the name itself); `externals` maps an external token's tree-sitter
 * name to the native expression that scans it; `wordRule` names the helper
 * rule keyword extraction adds. Returns `{ start, extras, conflicts, rules,
 * keywords, report }`: `rules` lists `{ name, sourceName, kind, body }` in
 * source order, followed by the helper rules; `report` lists the
 * approximations and the constructs left unsupported.
 */
export function importTreeSitterNative(source, options = {}) {
  let grammar;
  try {
    grammar = typeof source === 'string' ? JSON.parse(source) : source;
  } catch (error) {
    throw parseError(FORMAT, error.message);
  }
  if (!grammar || typeof grammar.rules !== 'object') throw parseError(FORMAT, 'grammar.json must contain object rules');
  const nameOf = options.nameOf ?? ((name) => name);
  const externalBodies = options.externals ?? {};
  const wordRule = options.wordRule ?? 'word_characters';
  const report = { approximations: [], unsupported: [] };
  const ruleNames = Object.keys(grammar.rules);
  if (ruleNames.includes(wordRule)) throw parseError(FORMAT, `the helper rule name ${wordRule} is a rule of the grammar`);
  // Supertypes and inlined rules never appear in tree-sitter's trees.
  const hiddenRules = new Set([...(grammar.supertypes ?? []), ...(grammar.inline ?? [])].map(memberName));
  const syntacticKind = (name) => (name.startsWith('_') || hiddenRules.has(name) ? 'silent' : 'normal');
  const externals = (grammar.externals ?? []).map(memberName);
  const namedLevels = new Map();
  for (const list of grammar.precedences ?? []) {
    list.forEach((entry, position) => {
      const name = entry.type === 'STRING' ? entry.value : (entry.type === 'SYMBOL' ? entry.name : null);
      if (name !== null && !namedLevels.has(name)) namedLevels.set(name, 1000 * (list.length - position));
    });
  }
  const level = (value) => {
    if (typeof value === 'number') return value;
    if (namedLevels.has(value)) return namedLevels.get(value);
    report.approximations.push(`named precedence ${value} has no order; level 0`);
    return 0;
  };
  const ref = (name) => `(ref ${enc(nameOf(name))})`;

  const expr = (node, inToken, keywords) => {
    switch (node.type) {
      case 'SYMBOL':
        if (externals.includes(node.name) && externalBodies[node.name] === undefined && !ruleNames.includes(node.name)) {
          report.unsupported.push(`external ${node.name}`);
        }
        return ref(node.name);
      case 'STRING':
        if (!inToken && keywords.has(node.value)) return `(token (seq (literal ${enc(node.value)}) (not (ref ${wordRule}))))`;
        return `(literal ${enc(node.value)})`;
      case 'PATTERN': {
        const text = renderTreeSitterPattern(parseTreeSitterPattern(node.value, node.flags ?? ''));
        return inToken ? text : `(token ${text})`;
      }
      case 'BLANK': return 'empty';
      case 'SEQ': {
        const items = node.members.map((member) => expr(member, inToken, keywords)).filter((item) => item !== 'empty');
        if (items.length === 0) return 'empty';
        return items.length === 1 ? items[0] : `(seq ${items.join(' ')})`;
      }
      case 'CHOICE': {
        const blank = node.members.some((member) => member.type === 'BLANK');
        const items = node.members.filter((member) => member.type !== 'BLANK').map((member) => expr(member, inToken, keywords));
        if (items.length === 0) return 'empty';
        const choice = items.length === 1 ? items[0] : `(choice unordered ${items.join(' ')})`;
        return blank ? `(optional ${choice})` : choice;
      }
      case 'REPEAT': return `(repeat0 ${expr(node.content, inToken, keywords)})`;
      case 'REPEAT1': return `(repeat1 ${expr(node.content, inToken, keywords)})`;
      case 'PREC': case 'PREC_LEFT': case 'PREC_RIGHT': {
        const inner = expr(node.content, inToken, keywords);
        if (inToken) return `(lexicalPrecedence ${level(node.value)} ${inner})`;
        const associativity = { PREC: 'none', PREC_LEFT: 'left', PREC_RIGHT: 'right' }[node.type];
        return `(precedence ${level(node.value)} ${associativity} ${inner})`;
      }
      case 'PREC_DYNAMIC': return `(dynamicPrecedence ${level(node.value)} ${expr(node.content, inToken, keywords)})`;
      case 'TOKEN': case 'IMMEDIATE_TOKEN': {
        if (inToken) return expr(node.content, true, keywords);
        const inner = expr(node.content, true, keywords);
        const guarded = keywordUnits.has(node) ? `(seq ${inner} (not (ref ${wordRule})))` : inner;
        return `(${node.type === 'TOKEN' ? 'token' : 'immediateToken'} ${guarded})`;
      }
      case 'FIELD': return `(capture labeled ${enc(node.name)} ${expr(node.content, inToken, keywords)})`;
      case 'ALIAS': {
        const inner = expr(node.content, inToken, keywords);
        // An anonymous alias is a leaf whose kind is its text.
        if (!node.named) return `(alias ${enc(`'${node.value}`)} ${inner})`;
        return `(alias ${enc(nameOf(node.value))} ${inner})`;
      }
      case 'RESERVED': return expr(node.content, inToken, keywords);
      default: throw new UnsupportedTreeSitterPattern(`node type ${node.type}`);
    }
  };

  // Keyword extraction: the strings the word token matches entirely, tested
  // on the word token compiled natively.
  const word = grammar.word ?? null;
  let keywords = new Set();
  const keywordUnits = new Set();
  let wordBody = null;
  if (word !== null) {
    const bare = unwrapPrecedence(grammar.rules[word]);
    try {
      wordBody = expr(bare.type === 'TOKEN' || bare.type === 'IMMEDIATE_TOKEN' ? bare.content : bare, true, keywords);
    } catch (error) {
      if (!(error instanceof UnsupportedTreeSitterPattern)) throw error;
      report.unsupported.push(`word ${word}: ${error.message}`);
    }
    if (wordBody !== null && isLexicalBody(grammar.rules[word])) {
      const matcher = compileGrammar(parseGrammarLinks(`(grammar (start word))\n(rule word token ${wordBody})\n`));
      const matches = (text) => { try { matcher.parse(text); return true; } catch { return false; } };
      // tree-sitter takes as a keyword each token every text of which the
      // word token matches.
      const units = [];
      for (const [name, node] of Object.entries(grammar.rules)) {
        if (name === word) continue;
        if (isLexicalBody(node)) units.push(node);
        else lexicalUnits(node, units);
      }
      for (const extra of grammar.extras ?? []) lexicalUnits(extra, units);
      const found = new Set();
      for (const unit of units) {
        const texts = finiteTexts(unit);
        if (texts === null || texts.length === 0 || texts.includes('')) continue;
        if (!texts.every((text) => found.has(text) || matches(text))) continue;
        texts.forEach((text) => found.add(text));
        keywordUnits.add(unit);
      }
      keywords = new Set([...found].sort());
    } else if (wordBody !== null) {
      report.approximations.push(`word rule ${word} is not a token; no keyword extraction`);
      wordBody = null;
    }
  }

  const extras = [];
  for (const extra of grammar.extras ?? []) {
    try {
      extras.push(extra.type === 'SYMBOL' ? ref(extra.name) : expr(extra, true, keywords));
    } catch (error) {
      if (!(error instanceof UnsupportedTreeSitterPattern)) throw error;
      report.unsupported.push(`extra: ${error.message}`);
    }
  }
  const conflicts = (grammar.conflicts ?? []).map((group) => group.map((member) => nameOf(memberName(member))));

  const rules = [];
  for (const name of ruleNames) {
    const node = grammar.rules[name];
    let body;
    let kind;
    try {
      if (isLexicalBody(node)) {
        const bare = unwrapPrecedence(node);
        const wrapped = node !== bare && node.type.startsWith('PREC') ? node.value : null;
        if (bare.type === 'IMMEDIATE_TOKEN') {
          kind = 'silent';
          body = `(alias ${enc(nameOf(name))} (immediateToken ${expr(bare.content, true, keywords)}))`;
        } else {
          kind = 'token';
          body = expr(bare.type === 'TOKEN' ? bare.content : bare, true, keywords);
          // A keyword rule is closed like a keyword.
          if (keywordUnits.has(node)) body = `(seq ${body} (not (ref ${wordRule})))`;
          if (wrapped !== null) body = `(lexicalPrecedence ${level(wrapped)} ${body})`;
        }
        if (name === word && keywords.size > 0) {
          const keywordSet = `(choice unordered ${[...keywords].map((text) => `(literal ${enc(text)})`).join(' ')})`;
          const exact = `(seq ${keywordSet} (not (ref ${wordRule})))`;
          body = `(choice unordered (seq (not ${exact}) (ref ${wordRule})) (dynamicPrecedence -1 (ref ${wordRule})))`;
        }
      } else {
        kind = syntacticKind(name);
        body = expr(node, false, keywords);
      }
    } catch (error) {
      if (!(error instanceof UnsupportedTreeSitterPattern)) throw error;
      report.unsupported.push(`${name}: ${error.message}`);
      kind = syntacticKind(name);
      body = '(not empty)';
    }
    rules.push({ name: nameOf(name), sourceName: name, kind, body });
  }
  if (wordBody !== null && keywords.size > 0) rules.push({ name: wordRule, sourceName: null, kind: 'token', body: wordBody });
  for (const name of externals) {
    if (ruleNames.includes(name)) continue;
    const body = externalBodies[name];
    if (body === undefined) report.unsupported.push(`external ${name} has no native scanner`);
    rules.push({ name: nameOf(name), sourceName: name, kind: name.startsWith('_') ? 'silent' : 'token', body: body ?? '(not empty)', external: true });
  }
  return {
    start: nameOf(ruleNames[0]),
    extras,
    conflicts,
    rules,
    keywords: [...keywords],
    report: { approximations: [...new Set(report.approximations)], unsupported: [...new Set(report.unsupported)] },
  };
}

/**
 * The Links Notation text of an imported grammar. `annotate(rule)` gives the
 * fields after a rule's body, such as `(concept ID)`; by default each rule
 * renamed from its source keeps the source name as `(source-names
 * (tree-sitter NAME))`.
 */
export function renderTreeSitterNative(imported, { annotate } = {}) {
  const fields = annotate ?? ((rule) => (rule.sourceName !== null && rule.sourceName !== rule.name
    ? [`(source-names (tree-sitter ${enc(rule.sourceName)}))`] : []));
  const lines = [`(grammar (start ${enc(imported.start)}))`];
  for (const extra of imported.extras) lines.push(`(extra ${extra})`);
  for (const group of imported.conflicts) lines.push(`(conflict ${group.map(enc).join(' ')})`);
  for (const rule of imported.rules) {
    lines.push(`(rule ${[enc(rule.name), rule.kind, rule.body, ...fields(rule)].join(' ')})`);
  }
  return `${lines.join('\n')}\n`;
}
