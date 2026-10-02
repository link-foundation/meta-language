// The native links form of a grammar: Links Notation that both runtimes write
// byte for byte the same and read back without the source the grammar came
// from. A `(grammar (format TAG) (start NAME))` link is followed by one
// `(rule NAME KIND EXPRESSION)` link per rule in grammar order, with an
// optional trailing `(doc TEXT)`. Expressions are nested links headed by the
// native listing's words: `empty`, `any`, `(literal TEXT)`,
// `(literalInsensitive TEXT)`, `(range FROM TO)`, `(class plain|negated
// ITEM...)` with `(char C)` and `(range FROM TO)` items, `(ref NAME)`,
// `(choice ordered|unordered ITEM...)`, `(seq ITEM...)`, `(optional ITEM)`,
// `(repeat0 ITEM)`, `(repeat1 ITEM)`, `(repeat MIN MAX|unbounded ITEM)`,
// `(and ITEM)`, `(not ITEM)` and `(capture labeled LABEL ITEM)` or
// `(capture unlabeled ITEM)`. Names, texts and characters are percent-encoded
// as `LinkNetwork` LiNo terms are (ASCII letters, digits and `-_.` kept, every
// other UTF-8 byte as `%XX`, the empty text as `%`). It mirrors
// rust/src/grammar/interchange/links.rs.
import { Parser } from 'links-notation';
import { Grammar } from './grammar.js';
import { ruleDoc } from './grammar-emitters/structural.js';
import { GrammarImportError } from './grammar-importers.js';

const RULE_KINDS = new Set(['normal', 'atomic', 'silent', 'token']);
const SOURCE_FORMATS = new Set(['meta-language', 'bnf', 'ebnf', 'abnf', 'peg', 'antlr', 'lark', 'gbnf', 'tree-sitter', 'inferred']);
const UNARY = new Set(['optional', 'repeat0', 'repeat1', 'and', 'not']);
const MAX_BOUND_DIGITS = 9;

function linksError(detail) {
  return new GrammarImportError('meta-language', 'parse', `links: ${detail}`);
}

/** Percent-encodes `value` as `LinkNetwork.toLino` terms are in the Rust runtime. */
export function percentEncodeLinksText(value) {
  if (value.length === 0) return '%';
  let encoded = '';
  for (const byte of new TextEncoder().encode(value)) {
    const char = String.fromCharCode(byte);
    encoded += /[A-Za-z0-9._-]/u.test(char) ? char : `%${byte.toString(16).toUpperCase().padStart(2, '0')}`;
  }
  return encoded;
}

/** Reverses `percentEncodeLinksText`, throwing a `GrammarImportError` on a malformed text. */
export function percentDecodeLinksText(value) {
  if (value === '%') return '';
  const bytes = [];
  for (let index = 0; index < value.length; index += 1) {
    const char = value[index];
    if (char === '%') {
      const hex = value.slice(index + 1, index + 3);
      if (!/^[0-9A-F]{2}$/u.test(hex)) throw linksError(`invalid percent escape in ${value}`);
      bytes.push(Number.parseInt(hex, 16));
      index += 2;
    } else if (/[A-Za-z0-9._-]/u.test(char)) {
      bytes.push(char.charCodeAt(0));
    } else {
      throw linksError(`unescaped character in ${value}`);
    }
  }
  try {
    return new TextDecoder('utf-8', { fatal: true }).decode(new Uint8Array(bytes));
  } catch {
    throw linksError(`${value} is not UTF-8`);
  }
}

/** Renders the native links form of `grammar`, one link per line. */
export function renderGrammarLinks(grammar) {
  const header = ['grammar'];
  if (grammar.sourceFormat) header.push(`(format ${grammar.sourceFormat})`);
  const start = grammar.startRule();
  if (start) header.push(`(start ${percentEncodeLinksText(start.name)})`);
  const lines = [`(${header.join(' ')})`];
  for (const rule of grammar.rules.values()) lines.push(renderRuleLink(grammar, rule));
  return lines.map((line) => `${line}\n`).join('');
}

/** Renders the `(rule NAME KIND EXPRESSION [(doc TEXT)])` link of one rule of `grammar`. */
export function renderRuleLink(grammar, rule) {
  const parts = ['rule', percentEncodeLinksText(rule.name), rule.kind, renderLinksExpression(rule.expression)];
  const doc = ruleDoc(grammar, rule);
  if (doc !== null) parts.push(`(doc ${percentEncodeLinksText(doc)})`);
  return `(${parts.join(' ')})`;
}

/** Renders one expression of the native links form. */
export function renderLinksExpression(expression) {
  const text = percentEncodeLinksText;
  const link = (...parts) => `(${parts.join(' ')})`;
  const items = (list) => list.map(renderLinksExpression);
  switch (expression.kind) {
    case 'empty': case 'any': return expression.kind;
    case 'literal': case 'literalInsensitive': return link(expression.kind, text(expression.value));
    case 'charRange': return link('range', text(expression.start), text(expression.end));
    case 'charClass':
      return link('class', expression.negated ? 'negated' : 'plain', ...expression.items.map((item) => (item.kind === 'range'
        ? link('range', text(item.start), text(item.end))
        : link('char', text(item.value)))));
    case 'ref': return link('ref', text(expression.name));
    case 'choice': return link('choice', expression.ordered ? 'ordered' : 'unordered', ...items(expression.items));
    case 'seq': return link('seq', ...items(expression.items));
    case 'optional': case 'repeat0': case 'repeat1': case 'and': case 'not':
      return link(expression.kind, renderLinksExpression(expression.item));
    case 'repeat':
      return link('repeat', String(expression.min), expression.max === null || expression.max === undefined ? 'unbounded' : String(expression.max), renderLinksExpression(expression.item));
    case 'capture':
      return expression.label === null || expression.label === undefined
        ? link('capture', 'unlabeled', renderLinksExpression(expression.item))
        : link('capture', 'labeled', text(expression.label), renderLinksExpression(expression.item));
    default: throw new TypeError(`unknown grammar expression kind ${expression.kind}`);
  }
}

/** A parsed value as `[head, arguments]`: a bare reference is a head without arguments. */
function parts(value) {
  if (value.values.length === 0) {
    if (value.id === null) throw linksError('empty link');
    return [value.id, []];
  }
  if (value.id !== null) throw linksError(`unexpected identified link ${value.id}`);
  const [head, ...rest] = value.values;
  if (head.values.length !== 0 || head.id === null) throw linksError('a link must start with a word');
  return [head.id, rest];
}

function word(value, what) {
  if (value === undefined || value.values.length !== 0 || value.id === null) throw linksError(`expected ${what}`);
  return value.id;
}

function decodedWord(value, what) {
  return percentDecodeLinksText(word(value, what));
}

function character(value, what) {
  const decoded = decodedWord(value, what);
  if ([...decoded].length !== 1) throw linksError(`${what} must be one code point`);
  return decoded;
}

function arity(head, args, count) {
  if (args.length !== count) throw linksError(`${head} takes ${count} value(s), not ${args.length}`);
}

function bound(value, allowUnbounded) {
  const text = word(value, 'a repetition bound');
  if (allowUnbounded && text === 'unbounded') return null;
  if (!/^[0-9]+$/u.test(text)) throw linksError(`invalid repetition bound ${text}`);
  if (text.length > MAX_BOUND_DIGITS) throw linksError(`repetition bound ${text} is too large`);
  return Number(text);
}

function flag(value, yes, no) {
  const text = word(value, `${yes} or ${no}`);
  if (text !== yes && text !== no) throw linksError(`expected ${yes} or ${no}, not ${text}`);
  return text === yes;
}

/** Reads one expression of the native links form. */
export function parseLinksExpression(value) {
  const [head, args] = parts(value);
  switch (head) {
    case 'empty': case 'any':
      arity(head, args, 0);
      return { kind: head };
    case 'literal': case 'literalInsensitive':
      arity(head, args, 1);
      return { kind: head, value: decodedWord(args[0], 'a text') };
    case 'range':
      arity(head, args, 2);
      return { kind: 'charRange', start: character(args[0], 'a range start'), end: character(args[1], 'a range end') };
    case 'class': {
      if (args.length === 0) throw linksError('class needs plain or negated');
      const negated = flag(args[0], 'negated', 'plain');
      const items = args.slice(1).map((item) => {
        const [itemHead, itemArgs] = parts(item);
        if (itemHead === 'char') {
          arity(itemHead, itemArgs, 1);
          return { kind: 'char', value: character(itemArgs[0], 'a class character') };
        }
        if (itemHead === 'range') {
          arity(itemHead, itemArgs, 2);
          return { kind: 'range', start: character(itemArgs[0], 'a range start'), end: character(itemArgs[1], 'a range end') };
        }
        throw linksError(`unknown class item ${itemHead}`);
      });
      return { kind: 'charClass', negated, items };
    }
    case 'ref':
      arity(head, args, 1);
      return { kind: 'ref', name: decodedWord(args[0], 'a rule name') };
    case 'choice': {
      if (args.length === 0) throw linksError('choice needs ordered or unordered');
      const ordered = flag(args[0], 'ordered', 'unordered');
      return { kind: 'choice', items: args.slice(1).map(parseLinksExpression), ordered };
    }
    case 'seq': return { kind: 'seq', items: args.map(parseLinksExpression) };
    case 'repeat': {
      arity(head, args, 3);
      const min = bound(args[0], false);
      const max = bound(args[1], true);
      if (max !== null && max < min) throw linksError(`repetition bounds ${min}, ${max} are reversed`);
      return { kind: 'repeat', item: parseLinksExpression(args[2]), min, max };
    }
    case 'capture': {
      if (args.length === 0) throw linksError('capture needs labeled or unlabeled');
      if (flag(args[0], 'labeled', 'unlabeled')) {
        arity(head, args, 3);
        return { kind: 'capture', label: decodedWord(args[1], 'a capture label'), item: parseLinksExpression(args[2]) };
      }
      arity(head, args, 2);
      return { kind: 'capture', label: null, item: parseLinksExpression(args[1]) };
    }
    default:
      if (!UNARY.has(head)) throw linksError(`unknown expression ${head}`);
      arity(head, args, 1);
      return { kind: head, item: parseLinksExpression(args[0]) };
  }
}

/**
 * Reads the native links form written by `renderGrammarLinks`, throwing a
 * `GrammarImportError` of the `meta-language` format.
 */
export function parseGrammarLinks(source) {
  let statements;
  try {
    statements = new Parser({ comments: false }).parse(source);
  } catch (error) {
    throw linksError(error.message);
  }
  if (statements.length === 0) throw linksError('the links define no grammar');
  const [headerHead, headerArgs] = parts(statements[0]);
  if (headerHead !== 'grammar') throw linksError('the first link must be the grammar link');
  let sourceFormat = null;
  let start = null;
  for (const field of headerArgs) {
    const [key, values] = parts(field);
    arity(key, values, 1);
    if (key === 'format' && sourceFormat === null) {
      sourceFormat = word(values[0], 'a format');
      if (!SOURCE_FORMATS.has(sourceFormat)) throw linksError(`unknown source format ${sourceFormat}`);
    } else if (key === 'start' && start === null) {
      start = decodedWord(values[0], 'a start rule');
    } else {
      throw linksError(`unexpected grammar field ${key}`);
    }
  }
  const rules = new Map();
  const docs = new Map();
  for (const statement of statements.slice(1)) {
    const [head, args] = parts(statement);
    if (head !== 'rule') throw linksError(`unexpected link ${head}`);
    if (args.length !== 3 && args.length !== 4) throw linksError('rule takes a name, a kind, an expression and an optional doc');
    const name = decodedWord(args[0], 'a rule name');
    const kind = word(args[1], 'a rule kind');
    if (!RULE_KINDS.has(kind)) throw linksError(`unknown rule kind ${kind}`);
    if (rules.has(name)) throw linksError(`rule ${name} is defined twice`);
    rules.set(name, { kind, expression: parseLinksExpression(args[2]) });
    if (args.length === 4) {
      const [docHead, docArgs] = parts(args[3]);
      if (docHead !== 'doc') throw linksError(`unexpected rule field ${docHead}`);
      arity(docHead, docArgs, 1);
      docs.set(name, decodedWord(docArgs[0], 'a doc text'));
    }
  }
  if (rules.size === 0) throw linksError('the links define no rules');
  if (start !== null && !rules.has(start)) throw linksError(`start rule ${start} is not defined`);
  const grammar = new Grammar(start ?? rules.keys().next().value, rules, sourceFormat);
  for (const [name, doc] of docs) grammar.rules.set(name, Object.freeze({ ...grammar.rules.get(name), doc }));
  return grammar;
}
