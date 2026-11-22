import { readFileSync } from 'node:fs';
import { Grammar, GrammarBuilder, canonicalRepeat, choice, sequence } from '../grammar.js';
import { parseGrammarLinks } from '../grammar-links.js';
import {
  Cursor,
  grammarFromRules,
  parseError,
  rule,
  unsupportedError,
} from './common.js';
import { decorateGrammar } from '../grammar-decorators.js';
import { allowPestImplicitTrivia } from './pest-policy.js';

const FORMAT = 'peg';
const STACK_RULES = new Set(['PUSH', 'PUSH_LITERAL', 'PEEK', 'PEEK_ALL', 'POP', 'POP_ALL', 'DROP']);
const TRIVIA_RULES = ['WHITESPACE', 'COMMENT'];
let builtInSources;

export function pestBuiltInSource(name) {
  builtInSources ??= new Map(readFileSync(new URL('../data/pest/built-ins.lino', import.meta.url), 'utf8')
    .split('\n').slice(1).filter(Boolean).map(line => [line.slice(6, line.indexOf(' ', 6)), line]));
  return builtInSources.get(name);
}

/** Imports pest PEG grammar source into the shared grammar representation. */
export function importPest(source, options = {}) {
  return decorateGrammar(readPest(source), options.decorators, 'importer');
}

function readPest(source) {
  const definitions = new PestParser(String(source)).parseRules();
  const rules = new Map(definitions.map(definition => [definition.name, definition]));
  const start = definitions[0]?.name;
  if (!start) return grammarFromRules(FORMAT, []);
  const grammar = new Grammar(start, rules, FORMAT);
  const builtIns = grammar.undefinedNonterminals().map(name => [name, pestBuiltInSource(name)]);
  const undefinedRule = builtIns.find(([, definition]) => !definition);
  if (undefinedRule) throw parseError(FORMAT, `undefined non-terminal ${undefinedRule[0]}`);
  if (builtIns.length > 0) {
    const links = [`(grammar (start ${builtIns[0][0]}))`, ...builtIns.map(([, definition]) => definition)];
    for (const [name, definition] of parseGrammarLinks(links.join('\n')).rules) rules.set(name, definition);
  }
  return grammarFromRules(FORMAT, lowerImplicitTrivia(rules), start);
}

function lowerImplicitTrivia(rules) {
  const [whitespace, comment] = TRIVIA_RULES.map(name => rules.has(name) ? GrammarBuilder.ref(name) : null);
  if (!whitespace && !comment) return [...rules.values()];
  const spaces = whitespace && GrammarBuilder.repeat0(whitespace);
  const comments = comment && GrammarBuilder.repeat0(sequence([comment, spaces].filter(Boolean)));
  const trivia = { kind: 'token', item: sequence([spaces, comments].filter(Boolean)) };
  const lowered = new Map();
  const variants = new Map();
  const pending = [];

  function referencedRule(name, allowsTrivia) {
    const definition = rules.get(name);
    if (allowsTrivia || !definition || definition.modifier === undefined ||
        ['@', '$', '!'].includes(definition.modifier) || TRIVIA_RULES.includes(name)) return name;
    if (!variants.has(name)) {
      let variant = `${name}_pest_atomic`;
      while (rules.has(variant)) variant += '_';
      variants.set(name, variant);
      pending.push([variant, definition]);
    }
    return variants.get(name);
  }

  function withTrivia(expression, allowsTrivia) {
    switch (expression.kind) {
      case 'ref': return GrammarBuilder.ref(referencedRule(expression.name, allowsTrivia));
      case 'seq': {
        const items = expression.items.map(item => withTrivia(item, allowsTrivia));
        return sequence(allowsTrivia ? items.flatMap((item, index) => index ? [trivia, item] : [item]) : items);
      }
      case 'choice': return choice(expression.items.map(item => withTrivia(item, allowsTrivia)), expression.ordered);
      case 'repeat': case 'repeat0': case 'repeat1': {
        const item = withTrivia(expression.item, allowsTrivia);
        if (!allowsTrivia) return { ...expression, item };
        const min = expression.min ?? (expression.kind === 'repeat1' ? 1 : 0);
        const max = expression.max ?? null;
        if (max === 0) return GrammarBuilder.empty();
        const rest = max === 1 ? GrammarBuilder.empty()
          : canonicalRepeat(sequence([trivia, item]), Math.max(min - 1, 0), max === null ? null : max - 1);
        const repeated = sequence([item, rest]);
        return min ? repeated : GrammarBuilder.optional(repeated);
      }
      case 'optional': case 'and': case 'not': return { ...expression, item: withTrivia(expression.item, allowsTrivia) };
      default: return expression;
    }
  }

  function lower(name, definition, inheritedTrivia) {
    const allowsTrivia = allowPestImplicitTrivia(inheritedTrivia, TRIVIA_RULES.includes(definition.name),
      ['@', '$'].includes(definition.modifier), definition.modifier === '!');
    const expression = withTrivia(definition.expression, allowsTrivia);
    const result = rule(name, expression, definition.kind);
    if (name !== definition.name && definition.kind !== 'silent') {
      result.action = [{ operation: 'buildNode', kind: definition.name }];
    }
    lowered.set(name, result);
  }

  for (const [name, definition] of rules) lower(name, definition, true);
  for (const [name, definition] of pending) lower(name, definition, false);
  return [...lowered.values()];
}

class PestParser extends Cursor {
  constructor(source) {
    super(source, FORMAT);
  }

  skipSpace() {
    while (true) {
      while (/\s/.test(this.peek())) this.offset += 1;
      if (this.source.startsWith('//', this.offset)) {
        const end = this.source.indexOf('\n', this.offset + 2);
        this.offset = end < 0 ? this.source.length : end + 1;
      } else if (this.source.startsWith('/*', this.offset)) {
        const end = this.source.indexOf('*/', this.offset + 2);
        if (end < 0) this.error('unterminated block comment');
        this.offset = end + 2;
      } else {
        return;
      }
    }
  }

  parseRules() {
    const rules = [];
    while (true) {
      this.skipSpace();
      if (this.eof()) break;
      const name = this.identifier();
      if (rules.some((candidate) => candidate.name === name)) {
        throw parseError(FORMAT, `duplicate rule ${name}`);
      }
      this.consume('=');
      this.skipSpace();
      const modifier = ['_', '@', '$', '!'].includes(this.peek()) ? this.take() : '';
      const kind = { _: 'silent', '@': 'atomic', $: 'token' }[modifier] ?? 'normal';
      this.consume('{');
      const expression = this.alternation();
      this.consume('}');
      rules.push({ ...rule(name, expression, kind), modifier });
    }
    return rules;
  }

  alternation() {
    const alternatives = [this.sequence()];
    while (this.tryConsume('|')) alternatives.push(this.sequence());
    return choice(alternatives, true);
  }

  sequence() {
    const items = [this.prefix()];
    while (this.tryConsume('~')) items.push(this.prefix());
    return sequence(items);
  }

  prefix() {
    if (this.tryConsume('&')) return GrammarBuilder.and(this.prefix());
    if (this.tryConsume('!')) return GrammarBuilder.not(this.prefix());
    return this.postfix();
  }

  postfix() {
    let expression = this.atom();
    if (this.tryConsume('?')) return GrammarBuilder.optional(expression);
    if (this.tryConsume('*')) return GrammarBuilder.repeat0(expression);
    if (this.tryConsume('+')) return GrammarBuilder.repeat1(expression);
    this.skipSpace();
    const counted = /^\{\s*(\d*)\s*(?:,\s*(\d*)\s*)?\}/.exec(this.source.slice(this.offset));
    if (counted && (counted[1] !== '' || counted[2] !== undefined)) {
      this.offset += counted[0].length;
      const hasComma = counted[0].includes(',');
      const min = counted[1] === '' ? 0 : Number(counted[1]);
      const max = hasComma
        ? counted[2] === '' ? null : Number(counted[2])
        : min;
      expression = canonicalRepeat(expression, min, max);
    }
    return expression;
  }

  atom() {
    this.skipSpace();
    if (this.tryConsume('(')) {
      const expression = this.alternation();
      this.consume(')');
      return expression;
    }
    if (this.peek() === '^') {
      this.offset += 1;
      if (this.peek() !== '"') this.error('case-insensitive marker must precede a string');
      return GrammarBuilder.literalInsensitive(this.quoted());
    }
    if (this.peek() === '"') return GrammarBuilder.literal(this.quoted());
    if (this.peek() === "'") {
      const start = this.quoted();
      this.consume('..');
      const end = this.quoted();
      if ([...start].length !== 1 || [...end].length !== 1) {
        throw parseError(FORMAT, 'character range endpoints must be one character');
      }
      return GrammarBuilder.charRange(start, end);
    }
    const name = this.identifier();
    this.skipSpace();
    if (name === 'PUSH' && this.peek() === '(') throw unsupportedError(FORMAT, 'Push');
    if (name === 'PEEK' && this.peek() === '[') throw unsupportedError(FORMAT, 'PeekSlice');
    if (STACK_RULES.has(name)) throw unsupportedError(FORMAT, `${name} stack rule`);
    return name === 'ANY' ? GrammarBuilder.any() : GrammarBuilder.ref(name);
  }
}
