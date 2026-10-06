// Converts a tree-sitter grammar.json into a native Links Notation grammar
// that the native executor runs, with no regular expressions left: every
// PATTERN is compiled into native classes, sequences, choices and repeats.
//   node experiments/tree-sitter-native-convert.mjs GRAMMAR.json > out.lino
//
// tree-sitter    native
// SYMBOL         (ref NAME)
// STRING         (literal TEXT), a keyword closed by (not (ref WORD-PATTERN))
// PATTERN        compiled pattern expression
// SEQ/CHOICE     (seq ...) / (choice unordered ...), BLANK as (optional ...)
// REPEAT(1)      (repeat0 ...) / (repeat1 ...)
// PREC*          (precedence LEVEL none|left|right ...); in a token
//                (lexicalPrecedence LEVEL ...)
// PREC_DYNAMIC   (dynamicPrecedence LEVEL ...)
// TOKEN          (token ...), IMMEDIATE_TOKEN (immediateToken ...)
// FIELD          (capture labeled NAME ...)
// ALIAS          (alias NAME ...)
// extras         (extra ...), conflicts (conflict ...)
// word           keyword extraction: the word rule does not take a keyword's
//                text unless no keyword parse exists (dynamic precedence -1)
import { readFileSync } from 'node:fs';

const enc = (text) => [...new TextEncoder().encode(text)]
  .map((byte) => (/[A-Za-z0-9._-]/u.test(String.fromCharCode(byte)) ? String.fromCharCode(byte) : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`))
  .join('') || '%';

export class UnsupportedPattern extends Error {}

// ---------------------------------------------------------------- patterns

// tree-sitter patterns are JavaScript regular expression sources. The parser
// below reads the subset grammars use into {alt|seq|repeat|class|literal|any}.
const ID_START = ['Lu', 'Ll', 'Lt', 'Lm', 'Lo', 'Nl'];
const ID_CONTINUE = [...ID_START, 'Mn', 'Mc', 'Nd', 'Pc'];
const PROPERTY_CATEGORIES = {
  XID_Start: ID_START, ID_Start: ID_START, XID_Continue: ID_CONTINUE, ID_Continue: ID_CONTINUE,
  Alphabetic: [...ID_START], Letter: ['L'], L: ['L'], Lu: ['Lu'], Ll: ['Ll'], Lt: ['Lt'], Lm: ['Lm'], Lo: ['Lo'],
  N: ['N'], Nd: ['Nd'], Nl: ['Nl'], No: ['No'], Number: ['N'], M: ['M'], Mn: ['Mn'], Mc: ['Mc'], Me: ['Me'],
  P: ['P'], Pc: ['Pc'], Pd: ['Pd'], Ps: ['Ps'], Pe: ['Pe'], Pi: ['Pi'], Pf: ['Pf'], Po: ['Po'],
  S: ['S'], Sm: ['Sm'], Sc: ['Sc'], Sk: ['Sk'], So: ['So'], Z: ['Z'], Zs: ['Zs'], Zl: ['Zl'], Zp: ['Zp'],
  Uppercase_Letter: ['Lu'], Lowercase_Letter: ['Ll'], Decimal_Number: ['Nd'], White_Space: ['Zs', 'Zl', 'Zp'],
  Emoji: ['So'], Emoji_Presentation: ['So'], Extended_Pictographic: ['So'],
};
const SCRIPTS = new Set(['Greek', 'Latin', 'Cyrillic', 'Han', 'Hiragana', 'Katakana', 'Arabic', 'Hebrew', 'Hangul', 'Thai', 'Devanagari']);
const SPACE_ITEMS = [...' \t\n\r\f\v ﻿  '].map((c) => ({ kind: 'char', value: c }))
  .concat([{ kind: 'category', value: 'Zs' }]);
const DIGIT_ITEMS = [{ kind: 'range', start: '0', end: '9' }];
const WORD_ITEMS = [{ kind: 'range', start: 'a', end: 'z' }, { kind: 'range', start: 'A', end: 'Z' }, ...DIGIT_ITEMS, { kind: 'char', value: '_' }];

export function parsePattern(source, flags = '') {
  let index = 0;
  const peek = () => source[index];
  const eat = (c) => (source[index] === c ? (index += 1, true) : false);
  const fail = (why) => { throw new UnsupportedPattern(`${why} at ${index} in /${source}/`); };

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
    if (!eat('{')) fail('expected {');
    const end = source.indexOf('}', index);
    let name = source.slice(index, end);
    index = end + 1;
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
    let first = true;
    while (index < source.length && (peek() !== ']' || (first && false))) {
      first = false;
      let item;
      if (eat('\\')) item = escape(true);
      else if (peek() === '[' && source[index + 1] === ':') fail('posix class');
      else item = { kind: 'char', value: codePoint() };
      if (item.kind === 'char' && peek() === '-' && source[index + 1] !== ']' && source[index + 1] !== undefined) {
        index += 1;
        let end;
        if (eat('\\')) end = escape(true);
        else end = { kind: 'char', value: codePoint() };
        if (end.kind !== 'char') fail('range to a class');
        items.push({ kind: 'range', start: item.value, end: end.value });
      } else if (item.kind === 'char') items.push(item);
      else if (item.negated) nested.push(item);
      else items.push(...item.items);
    }
    if (!eat(']')) fail('unterminated class');
    if (nested.length === 0) return { kind: 'class', negated, items };
    // [x\S] is x or not-space; [^x\S] would be an intersection.
    if (negated) fail('negated class with a negated escape');
    const alternatives = nested.slice();
    if (items.length > 0) alternatives.unshift({ kind: 'class', negated: false, items });
    return { kind: 'alt', items: alternatives };
  };
  const atom = () => {
    const c = peek();
    if (c === '(') {
      index += 1;
      if (eat('?')) {
        if (eat(':')) { /* group */ } else if (peek() === '=' || peek() === '!') {
          const negative = source[index] === '!';
          index += 1;
          const inner = alternation();
          if (!eat(')')) fail('unterminated lookahead');
          return { kind: negative ? 'not' : 'and', item: inner };
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
      eat('?'); // lazy quantifiers match the same language
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
    default: throw new Error(item.kind);
  }
}

/** The native expression text of a parsed pattern. Adjacent characters join into one literal. */
export function renderPattern(node) {
  switch (node.kind) {
    case 'char': return `(literal ${enc(node.value)})`;
    case 'class': return `(class ${node.negated ? 'negated' : 'plain'} ${node.items.map(classItem).join(' ')})`;
    case 'seq': {
      const parts = [];
      let text = '';
      for (const item of node.items) {
        if (item.kind === 'char') { text += item.value; continue; }
        if (text) { parts.push(`(literal ${enc(text)})`); text = ''; }
        parts.push(renderPattern(item));
      }
      if (text) parts.push(`(literal ${enc(text)})`);
      if (parts.length === 0) return 'empty';
      return parts.length === 1 ? parts[0] : `(seq ${parts.join(' ')})`;
    }
    case 'alt': {
      const items = node.items.map(renderPattern);
      return `(choice unordered ${items.join(' ')})`;
    }
    case 'repeat': {
      const item = renderPattern(node.item);
      if (node.min === 0 && node.max === null) return `(repeat0 ${item})`;
      if (node.min === 1 && node.max === null) return `(repeat1 ${item})`;
      if (node.min === 0 && node.max === 1) return `(optional ${item})`;
      return `(repeat ${node.min} ${node.max === null ? 'unbounded' : node.max} ${item})`;
    }
    case 'and': return `(and ${renderPattern(node.item)})`;
    case 'not': return `(not ${renderPattern(node.item)})`;
    default: throw new Error(node.kind);
  }
}

// ---------------------------------------------------------------- grammar

const unwrapPrecedence = (node) => (node.type.startsWith('PREC') ? unwrapPrecedence(node.content) : node);
const isLexicalBody = (node) => ['STRING', 'PATTERN', 'TOKEN', 'IMMEDIATE_TOKEN'].includes(unwrapPrecedence(node).type);

function strings(node, out = new Set()) {
  if (!node || typeof node !== 'object') return out;
  if (node.type === 'STRING') out.add(node.value);
  for (const value of Object.values(node)) {
    if (Array.isArray(value)) value.forEach((item) => strings(item, out));
    else if (value && typeof value === 'object') strings(value, out);
  }
  return out;
}

export const WORD_PATTERN_RULE = '_word_pattern';

/**
 * Converts the parsed grammar.json `grammar` and returns
 * `{ text, report }`: the links text and what the conversion approximated.
 */
export function convertTreeSitterGrammar(grammar, { externals: externalRules = {} } = {}) {
  const report = { approximations: [], unsupported: [] };
  const ruleNames = Object.keys(grammar.rules);
  // Supertypes and inlined rules never appear in tree-sitter's trees.
  const hiddenRules = new Set([...(grammar.supertypes ?? []), ...(grammar.inline ?? [])].map((member) => member.name ?? member));
  const syntacticKind = (name) => (name.startsWith('_') || hiddenRules.has(name) ? 'silent' : 'normal');
  const externals = (grammar.externals ?? []).map((e) => e.name ?? e.value);
  const namedLevels = new Map();
  for (const list of grammar.precedences ?? []) {
    list.forEach((entry, position) => {
      if (entry.type === 'STRING' && !namedLevels.has(entry.value)) namedLevels.set(entry.value, 1000 * (list.length - position));
    });
  }
  const level = (value) => {
    if (typeof value === 'number') return value;
    if (namedLevels.has(value)) return namedLevels.get(value);
    report.approximations.push(`named precedence ${value} has no order; level 0`);
    return 0;
  };

  // Keyword extraction.
  const word = grammar.word ?? null;
  let keywordTest = null;
  let keywords = [];
  if (word) {
    const body = unwrapPrecedence(grammar.rules[word]);
    const wordSource = body.type === 'PATTERN' ? body.value : (body.type === 'TOKEN' && body.content.type === 'PATTERN' ? body.content.value : null);
    if (wordSource) {
      const fixed = wordSource.replace(/\\p\{XID_/gu, '\\p{ID_');
      const regex = new RegExp(`^(?:${fixed})$`, 'u');
      keywordTest = (text) => regex.test(text);
      keywords = [...strings({ rules: grammar.rules, extras: grammar.extras })].filter((text) => keywordTest(text)).sort();
    } else report.approximations.push(`word rule ${word} is not a pattern; no keyword extraction`);
  }
  const isKeyword = (text) => keywordTest !== null && keywords.includes(text);

  const expr = (node, inToken) => {
    switch (node.type) {
      case 'SYMBOL':
        if (externals.includes(node.name) && !externalRules[node.name]) report.unsupported.push(`external ${node.name}`);
        return `(ref ${enc(node.name)})`;
      case 'STRING':
        if (!inToken && isKeyword(node.value)) return `(token (seq (literal ${enc(node.value)}) (not (ref ${WORD_PATTERN_RULE}))))`;
        return `(literal ${enc(node.value)})`;
      case 'PATTERN': {
        const text = renderPattern(parsePattern(node.value, node.flags ?? ''));
        return inToken ? text : `(token ${text})`;
      }
      case 'BLANK': return 'empty';
      case 'SEQ': {
        const items = node.members.map((member) => expr(member, inToken)).filter((item) => item !== 'empty');
        if (items.length === 0) return 'empty';
        return items.length === 1 ? items[0] : `(seq ${items.join(' ')})`;
      }
      case 'CHOICE': {
        const blank = node.members.some((member) => member.type === 'BLANK');
        const items = node.members.filter((member) => member.type !== 'BLANK').map((member) => expr(member, inToken));
        const choice = items.length === 1 ? items[0] : `(choice unordered ${items.join(' ')})`;
        return blank ? `(optional ${choice})` : choice;
      }
      case 'REPEAT': return `(repeat0 ${expr(node.content, inToken)})`;
      case 'REPEAT1': return `(repeat1 ${expr(node.content, inToken)})`;
      case 'PREC': case 'PREC_LEFT': case 'PREC_RIGHT': {
        const inner = expr(node.content, inToken);
        if (inToken) return `(lexicalPrecedence ${level(node.value)} ${inner})`;
        const associativity = { PREC: 'none', PREC_LEFT: 'left', PREC_RIGHT: 'right' }[node.type];
        return `(precedence ${level(node.value)} ${associativity} ${inner})`;
      }
      case 'PREC_DYNAMIC': return `(dynamicPrecedence ${level(node.value)} ${expr(node.content, inToken)})`;
      case 'TOKEN': return inToken ? expr(node.content, true) : `(token ${expr(node.content, true)})`;
      case 'IMMEDIATE_TOKEN': return inToken ? expr(node.content, true) : `(immediateToken ${expr(node.content, true)})`;
      case 'FIELD': return `(capture labeled ${enc(node.name)} ${expr(node.content, inToken)})`;
      case 'ALIAS': {
        if (!node.named) {
          // An anonymous alias is a leaf whose kind is the text.
          if (!inToken) report.approximations.push(`anonymous alias ${node.value}`);
          return `(alias ${enc(`'${node.value}`)} ${expr(node.content, inToken)})`;
        }
        return `(alias ${enc(node.value)} ${expr(node.content, inToken)})`;
      }
      case 'RESERVED': return expr(node.content, inToken);
      default: throw new UnsupportedPattern(`node type ${node.type}`);
    }
  };

  const lines = [`(grammar (format tree-sitter) (start ${enc(ruleNames[0])}))`];
  for (const extra of grammar.extras ?? []) lines.push(`(extra ${extra.type === 'SYMBOL' ? `(ref ${enc(extra.name)})` : expr(extra, true)})`);
  for (const group of grammar.conflicts ?? []) {
    const names = group.map((member) => member.name ?? member);
    lines.push(`(conflict ${names.map(enc).join(' ')})`);
  }
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
          body = `(alias ${enc(name)} (immediateToken ${expr(bare.content, true)}))`;
        } else {
          kind = 'token';
          body = expr(bare.type === 'TOKEN' ? bare.content : bare, true);
          if (wrapped !== null) body = `(lexicalPrecedence ${level(wrapped)} ${body})`;
        }
        if (name === word && keywords.length > 0) {
          const keywordSet = `(choice unordered ${keywords.map((text) => `(literal ${enc(text)})`).join(' ')})`;
          const exact = `(seq ${keywordSet} (not (ref ${WORD_PATTERN_RULE})))`;
          body = `(choice unordered (seq (not ${exact}) (ref ${WORD_PATTERN_RULE})) (dynamicPrecedence -1 (ref ${WORD_PATTERN_RULE})))`;
        }
      } else {
        kind = syntacticKind(name);
        body = expr(node, false);
      }
    } catch (error) {
      if (!(error instanceof UnsupportedPattern)) throw error;
      report.unsupported.push(`${name}: ${error.message}`);
      kind = syntacticKind(name);
      body = '(not empty)';
    }
    lines.push(`(rule ${enc(name)} ${kind} ${body} (source-names (tree-sitter ${enc(name)})))`);
  }
  if (word && keywords.length > 0) {
    const bare = unwrapPrecedence(grammar.rules[word]);
    lines.push(`(rule ${WORD_PATTERN_RULE} token ${expr(bare.type === 'TOKEN' ? bare.content : bare, true)})`);
  }
  for (const name of externals) {
    if (ruleNames.includes(name)) continue;
    lines.push(`(rule ${enc(name)} ${name.startsWith('_') ? 'silent' : 'token'} ${externalRules[name] ?? '(not empty)'})`);
  }
  return { text: `${lines.join('\n')}\n`, report, keywords };
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const grammar = JSON.parse(readFileSync(process.argv[2], 'utf8'));
  const { text, report } = convertTreeSitterGrammar(grammar);
  process.stdout.write(text);
  console.error(JSON.stringify(report, null, 1));
}
