// ANTLR v4 importer. The Rust twin lives in rust/src/grammar/import/antlr.rs;
// both lower the same structural `.g4` subset to the same grammar IR.
import { Grammar, GrammarBuilder } from '../grammar.js';
import {
  FORMAT,
  debugString,
  errorAt,
  tokenText,
  tokenizeAntlr,
} from './antlr-lexer.js';
import { parseError, unsupportedError } from './common.js';
import { decorateGrammar } from '../grammar-decorators.js';

const SEQUENCE_END = new Set(['pipe', 'semicolon', 'arrow', 'rparen', 'hash']);
const SUFFIXES = {
  question: GrammarBuilder.optional,
  star: GrammarBuilder.repeat0,
  plus: GrammarBuilder.repeat1,
};

/**
 * Imports ANTLR v4 `.g4` grammar text into the shared grammar representation.
 *
 * Alternatives lower to unordered choices. Lexer rules (uppercase names) become
 * token rules and `fragment` rules become silent rules. Lexer commands, dropped
 * actions and predicates, and comments before a rule or its `:` are kept in
 * the rule's `doc`; `-> skip` and `-> channel(NAME)` also put the rule on that
 * channel, which makes it trivia. `EOF` is the end of the input, and character
 * sets read `\uXXXX`, `\u{X...}` and `\p{NAME}` for a general category or
 * script. `options`, `tokens`, `channels`, `import` and `mode` declarations are
 * skipped, and references to undefined rules stay visible on the grammar.
 */
export function importAntlr(source, options = {}) {
  return decorateGrammar(readAntlr(source), options.decorators, 'importer');
}

function readAntlr(source) {
  return new AntlrParser(tokenizeAntlr(String(source))).parseGrammar();
}

class AntlrParser {
  constructor(tokens) {
    this.tokens = tokens;
    this.cursor = 0;
    this.pendingComments = [];
  }

  parseGrammar() {
    const rules = [];
    while (!this.isEnd()) {
      this.collectComments();
      if (this.isEnd()) break;
      if (this.parseHeader() || this.parseSkippedDirective()) continue;
      rules.push(this.parseRule());
    }

    const startRule = rules.find((rule) => rule.kind === 'normal') ?? rules[0];
    if (!startRule) throw parseError(FORMAT, 'ANTLR grammar does not contain rules');
    return buildGrammar(startRule.name, rules);
  }

  parseHeader() {
    if (this.checkKeyword('grammar')) {
      this.advance();
    } else if ((this.checkKeyword('lexer') || this.checkKeyword('parser')) &&
      this.checkNextKeyword('grammar')) {
      this.advance();
      this.advance();
    } else {
      return false;
    }
    this.expectIdent('grammar name');
    this.expectKind('semicolon', "';'");
    this.pendingComments = [];
    return true;
  }

  parseSkippedDirective() {
    if (this.checkAnyKeyword(['options', 'tokens', 'channels'])) {
      this.advance();
      if (this.peekKind() === 'action') {
        this.advance();
        this.tryConsume('semicolon');
      } else {
        this.skipUntilSemicolon();
      }
      this.pendingComments = [];
      return true;
    }
    if (this.checkAnyKeyword(['import', 'mode'])) {
      this.advance();
      this.skipUntilSemicolon();
      this.pendingComments = [];
      return true;
    }
    return false;
  }

  parseRule() {
    const comments = this.pendingComments;
    this.pendingComments = [];
    const fragment = this.tryConsumeKeyword('fragment');
    const name = this.expectIdent('rule name');
    // A comment between the name and its ':' documents the rule too.
    this.collectComments();
    comments.push(...this.pendingComments);
    this.pendingComments = [];
    this.rejectRulePrelude();
    this.expectKind('colon', "':'");

    const notes = [];
    const expression = this.parseChoice(notes);
    const command = this.tryConsume('arrow') ? this.parseLexerCommand() : null;
    this.expectKind('semicolon', "';'");

    let kind = 'normal';
    if (fragment) kind = 'silent';
    else if (/^[A-Z]/.test(name)) kind = 'token';
    return { name, kind, expression, doc: ruleDoc(comments, notes, command), channel: commandChannel(command) };
  }

  rejectRulePrelude() {
    if (this.peekKind() === 'colon') return;
    const keyword = this.peekKind() === 'ident' ? this.peek().value : null;
    if (['locals', 'returns', 'throws', 'options'].includes(keyword)) {
      throw unsupportedError(FORMAT, `rule prelude ${keyword}`);
    }
    if (this.peekKind() === 'charSet') throw unsupportedError(FORMAT, 'rule arguments');
    throw this.expected("':' before rule body");
  }

  parseChoice(notes) {
    const alternatives = [];
    pushChoiceAlternative(alternatives, this.parseAlternative(notes));
    while (this.tryConsume('pipe')) pushChoiceAlternative(alternatives, this.parseAlternative(notes));
    return finishChoice(alternatives);
  }

  // An alternative and its `# Label`, which names the context class ANTLR
  // generates for it, not syntax: the label joins the rule's doc.
  parseAlternative(notes) {
    const sequence = this.parseSequence(notes);
    if (this.tryConsume('hash')) {
      notes.push(`alternative ${this.expectIdent('alternative label')}`);
      this.skipInlineComments();
    }
    return sequence;
  }

  parseSequence(notes) {
    const items = [];
    while (true) {
      this.skipInlineComments();
      if (this.isEnd() || SEQUENCE_END.has(this.peekKind())) break;
      pushSequenceItem(items, this.parseElement(notes));
    }
    return finishSequence(items);
  }

  parseElement(notes) {
    this.skipInlineComments();
    if (this.peekKind() === 'action') return this.parseAction(notes);

    const label = this.labelAhead();
    if (label !== null) {
      this.advance();
      this.advance();
      return GrammarBuilder.capture(label, this.parsePrefixed(notes));
    }
    return this.parsePrefixed(notes);
  }

  parsePrefixed(notes) {
    this.skipInlineComments();
    const expression = this.tryConsume('tilde')
      ? negateExpression(this.parseAtom(notes))
      : this.parseAtom(notes);
    return this.parseSuffixes(expression);
  }

  parseAtom(notes) {
    this.skipInlineComments();
    const token = this.peek();
    if (!token) throw this.expected('expression element');

    switch (token.kind) {
      case 'ident':
        this.advance();
        // ANTLR reserves EOF for the end of the input.
        if (token.value === 'EOF') return GrammarBuilder.not(GrammarBuilder.any());
        return GrammarBuilder.ref(token.value);
      case 'string': {
        this.advance();
        if (!this.tryConsume('range')) return GrammarBuilder.literal(token.value);
        const endText = this.expectString('range end');
        const start = singleChar(token.value, 'range start', token.offset);
        const end = singleChar(endText, 'range end', token.offset);
        if (start.codePointAt(0) > end.codePointAt(0)) {
          throw errorAt(token.offset, 'literal range start exceeds end');
        }
        return GrammarBuilder.charRange(start, end);
      }
      case 'charSet':
        this.advance();
        return lowerCharSet(token.value, token.offset);
      case 'dot':
        this.advance();
        return GrammarBuilder.any();
      case 'lparen': {
        this.advance();
        const expression = this.parseChoice(notes);
        this.expectKind('rparen', "')'");
        return expression;
      }
      case 'action':
        return this.parseAction(notes);
      default:
        throw this.expected('expression element');
    }
  }

  parseAction(notes) {
    this.advance();
    notes.push(this.tryConsume('question') ? 'dropped predicate' : 'dropped action');
    return GrammarBuilder.empty();
  }

  parseSuffixes(initial) {
    let expression = initial;
    while (Object.hasOwn(SUFFIXES, this.peekKind())) {
      expression = SUFFIXES[this.advance().kind](expression);
      if (this.tryConsume('question')) expression = GrammarBuilder.capture('non_greedy', expression);
    }
    return expression;
  }

  parseLexerCommand() {
    const tokens = [];
    while (!this.isEnd() && this.peekKind() !== 'semicolon') {
      const token = this.advance();
      if (token.kind !== 'comment') tokens.push(token);
    }
    if (tokens.length === 0) throw this.expected('lexer command');
    return formatCommand(tokens);
  }

  skipUntilSemicolon() {
    while (!this.isEnd()) {
      if (this.tryConsume('semicolon')) return;
      this.advance();
    }
    throw this.expected("';' after directive");
  }

  collectComments() {
    while (this.peekKind() === 'comment') this.pendingComments.push(this.advance().value);
  }

  skipInlineComments() {
    while (this.peekKind() === 'comment') this.advance();
  }

  labelAhead() {
    if (this.peekKind() !== 'ident') return null;
    const next = this.tokens[this.cursor + 1]?.kind;
    return next === 'equal' || next === 'plusEqual' ? this.peek().value : null;
  }

  expectIdent(role) {
    if (this.peekKind() !== 'ident') throw this.expected(role);
    return this.advance().value;
  }

  expectString(role) {
    if (this.peekKind() !== 'string') throw this.expected(role);
    return this.advance().value;
  }

  expectKind(kind, role) {
    if (this.peekKind() !== kind) throw this.expected(role);
    this.advance();
  }

  expected(expected) {
    return errorAt(this.peek()?.offset ?? 0, `expected ${expected}`);
  }

  tryConsumeKeyword(keyword) {
    if (!this.checkKeyword(keyword)) return false;
    this.advance();
    return true;
  }

  tryConsume(kind) {
    if (this.peekKind() !== kind) return false;
    this.advance();
    return true;
  }

  checkKeyword(keyword) {
    return isKeyword(this.peek(), keyword);
  }

  checkNextKeyword(keyword) {
    return isKeyword(this.tokens[this.cursor + 1], keyword);
  }

  checkAnyKeyword(keywords) {
    return keywords.some((keyword) => this.checkKeyword(keyword));
  }

  isEnd() {
    return this.cursor >= this.tokens.length;
  }

  peek() {
    return this.tokens[this.cursor];
  }

  peekKind() {
    return this.tokens[this.cursor]?.kind;
  }

  advance() {
    const token = this.tokens[this.cursor];
    this.cursor += 1;
    return token;
  }
}

// Rust keeps rules in a list and resolves names to the first match; the
// JavaScript grammar is keyed by name, so a repeated rule keeps its first body.
function buildGrammar(start, rules) {
  const mapped = new Map();
  for (const rule of rules) if (!mapped.has(rule.name)) mapped.set(rule.name, rule);
  const grammar = new Grammar(start, mapped, FORMAT);
  for (const rule of mapped.values()) {
    if (rule.doc === null) continue;
    grammar.rules.set(rule.name, Object.freeze({ ...grammar.rules.get(rule.name), doc: rule.doc }));
  }
  return grammar;
}

function isKeyword(token, keyword) {
  return token?.kind === 'ident' && token.value === keyword;
}

function lowerCharSet(content, offset) {
  const scanner = new ClassScanner(content, offset);
  const negated = scanner.tryConsume('^');
  const items = [];
  while (!scanner.isEnd()) {
    const property = scanner.tryReadProperty();
    if (property !== null) {
      items.push(property);
      continue;
    }
    const start = scanner.readChar();
    if (scanner.tryConsumeRangeSeparator()) {
      const end = scanner.readChar();
      if (start.codePointAt(0) > end.codePointAt(0)) {
        throw errorAt(offset, 'character class range start exceeds end');
      }
      items.push({ kind: 'range', start, end });
    } else {
      items.push({ kind: 'char', value: start });
    }
  }
  if (items.length === 0) throw errorAt(offset, 'character class must not be empty');
  return GrammarBuilder.charClass(items, negated);
}

// Reads the raw text between `[` and `]`: `\uXXXX` and `\u{X...}` escape a
// code point, `\p{NAME}` names a Unicode general category or script, and any
// other unknown escape stands for the escaped character.
class ClassScanner {
  constructor(text, offset) {
    this.chars = Array.from(text);
    this.cursor = 0;
    this.offset = offset;
  }

  readChar() {
    if (this.isEnd()) throw errorAt(this.offset, 'unexpected end of character class');
    const character = this.advanceChar();
    return character === '\\' ? this.readEscape() : character;
  }

  readEscape() {
    if (this.isEnd()) throw errorAt(this.offset, 'unterminated character class escape');
    const character = this.advanceChar();
    if (character === 'u') return this.readCodePoint();
    return ({ n: '\n', r: '\r', t: '\t', b: '\u0008', f: '\u000c' })[character] ?? character;
  }

  // The code point of `\uXXXX` or `\u{X...}`, after the `u`.
  readCodePoint() {
    let digits = '';
    if (this.tryConsume('{')) {
      while (!this.isEnd() && this.chars[this.cursor] !== '}') digits += this.advanceChar();
      if (!this.tryConsume('}')) throw errorAt(this.offset, 'unterminated unicode escape');
    } else {
      for (let count = 0; count < 4 && !this.isEnd(); count += 1) digits += this.advanceChar();
    }
    const value = /^[0-9A-Fa-f]{1,6}$/u.test(digits) ? Number.parseInt(digits, 16) : -1;
    if (value < 0 || value > 0x10ffff || (value >= 0xd800 && value <= 0xdfff)) {
      throw errorAt(this.offset, 'invalid unicode escape');
    }
    return String.fromCodePoint(value);
  }

  // A `\p{NAME}` item at the cursor, or null.
  tryReadProperty() {
    const [backslash, letter, open] = this.chars.slice(this.cursor, this.cursor + 3);
    if (backslash !== '\\' || (letter !== 'p' && letter !== 'P')) return null;
    if (letter === 'P') throw unsupportedError(FORMAT, 'negated Unicode property in character set');
    if (open !== '{') throw errorAt(this.offset, 'Unicode property needs a {NAME}');
    const close = this.chars.indexOf('}', this.cursor);
    if (close < 0) throw errorAt(this.offset, 'unterminated Unicode property');
    const name = this.chars.slice(this.cursor + 3, close).join('');
    this.cursor = close + 1;
    return unicodePropertyItem(name);
  }

  tryConsume(expected) {
    if (this.chars[this.cursor] !== expected) return false;
    this.cursor += 1;
    return true;
  }

  tryConsumeRangeSeparator() {
    if (this.chars[this.cursor] !== '-' || this.cursor + 1 >= this.chars.length) return false;
    this.cursor += 1;
    return true;
  }

  isEnd() {
    return this.cursor >= this.chars.length;
  }

  advanceChar() {
    const character = this.chars[this.cursor];
    this.cursor += 1;
    return character;
  }
}

function negateExpression(expression) {
  if (expression.kind === 'charClass' && !expression.negated) {
    return GrammarBuilder.charClass(expression.items, true);
  }
  return GrammarBuilder.not(expression);
}

function finishSequence(items) {
  if (items.length === 0) return GrammarBuilder.empty();
  if (items.length === 1) return items[0];
  return { kind: 'seq', items };
}

function pushSequenceItem(items, item) {
  if (item.kind === 'empty') return;
  if (item.kind === 'seq') {
    for (const nested of item.items) pushSequenceItem(items, nested);
  } else {
    items.push(item);
  }
}

function finishChoice(alternatives) {
  if (alternatives.every((alternative) => alternative.kind === 'empty')) {
    return GrammarBuilder.empty();
  }
  if (alternatives.length === 1) return alternatives[0];
  return { kind: 'choice', items: alternatives, ordered: false };
}

function pushChoiceAlternative(alternatives, alternative) {
  if (alternative.kind === 'choice' && !alternative.ordered) alternatives.push(...alternative.items);
  else alternatives.push(alternative);
}

function singleChar(value, role, offset) {
  const characters = [...value];
  if (characters.length === 0) throw errorAt(offset, `${role} must contain one character`);
  if (characters.length > 1) {
    throw errorAt(offset, `${role} ${debugString(value)} must contain one character`);
  }
  return characters[0];
}

function ruleDoc(comments, notes, command) {
  const parts = [...comments.filter((comment) => comment !== ''), ...notes];
  if (command !== null) parts.push(command);
  return parts.length === 0 ? null : parts.join('; ');
}

// The channel a lexer command puts its token on: `skip` and `channel(NAME)`
// make the token trivia, which the runtime skips between tokens.
function commandChannel(command) {
  if (command === null) return null;
  const parts = command.slice('->'.length).split(',').map((part) => part.trim());
  for (const part of parts) {
    const channel = /^channel\(([A-Za-z_][A-Za-z_0-9]*)\)$/u.exec(part);
    if (channel !== null) return channel[1];
  }
  return parts.includes('skip') ? 'skip' : null;
}

const UNICODE_SCRIPTS = new Set([
  'Arabic', 'Armenian', 'Bengali', 'Cyrillic', 'Devanagari', 'Georgian', 'Greek', 'Han', 'Hangul', 'Hebrew', 'Hiragana',
  'Katakana', 'Latin', 'Thai',
]);

// The character class item `\p{NAME}` names: a general category such as `L` or
// `Nd`, or a script such as `Greek` or `Script=Greek`.
function unicodePropertyItem(name) {
  const property = name.replace(/^(?:General_Category|gc)=/u, '');
  if (/^[LMNPSZC][a-z]?$/u.test(property)) return { kind: 'category', value: property };
  const script = property.replace(/^(?:Script|sc)=/u, '');
  if (UNICODE_SCRIPTS.has(script)) return { kind: 'script', value: script };
  throw unsupportedError(FORMAT, `Unicode property ${name}`);
}

function formatCommand(tokens) {
  let output = '->';
  for (const token of tokens) {
    if (token.kind === 'lparen') {
      output += '(';
    } else if (token.kind === 'rparen') {
      output += ')';
    } else if (token.kind === 'comma') {
      output += ', ';
    } else {
      if (output === '->' || (!output.endsWith('(') && !output.endsWith(', '))) output += ' ';
      output += tokenText(token);
    }
  }
  return output;
}
