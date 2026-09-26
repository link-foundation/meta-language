import { Grammar } from '../grammar.js';

export class GrammarImportError extends Error {
  constructor(format, kind, detail) {
    const description = kind === 'unsupported'
      ? `unsupported construct: ${detail}`
      : `parse error: ${detail}`;
    super(`${format} import ${description}`);
    this.name = 'GrammarImportError';
    this.format = format;
    this.kind = kind;
    if (kind === 'unsupported') this.construct = detail;
  }
}

export function parseError(format, detail) {
  return new GrammarImportError(format, 'parse', detail);
}

export function unsupportedError(format, detail) {
  return new GrammarImportError(format, 'unsupported', detail);
}

export function grammarFromRules(format, rules, start = null, allowed = []) {
  if (rules.length === 0) throw parseError(format, 'grammar contains no rules');
  const mapped = new Map();
  for (const rule of rules) {
    if (mapped.has(rule.name)) throw parseError(format, `duplicate rule ${rule.name}`);
    mapped.set(rule.name, rule);
  }
  const grammar = new Grammar(start ?? rules[0].name, mapped, format);
  const missing = grammar.undefinedNonterminals(allowed);
  if (missing.length > 0) throw parseError(format, `undefined non-terminal ${missing[0]}`);
  return grammar;
}

export function rule(name, expression, kind = 'normal') {
  return { name, kind, expression };
}

export function stripLineComment(line, marker = ';') {
  let quote = null;
  let escaped = false;
  for (let index = 0; index < line.length; index += 1) {
    const character = line[index];
    if (escaped) {
      escaped = false;
      continue;
    }
    if (quote && character === '\\') {
      escaped = true;
      continue;
    }
    if (quote === character) quote = null;
    else if (!quote && (character === '"' || character === "'")) quote = character;
    else if (!quote && line.startsWith(marker, index)) return line.slice(0, index);
  }
  return line;
}

export function decodeQuoted(text, quote, format) {
  let result = '';
  while (!text.eof()) {
    const character = text.take();
    if (character === quote) return result;
    if (character !== '\\') {
      result += character;
      continue;
    }
    if (text.eof()) throw parseError(format, 'unterminated string escape');
    const escaped = text.take();
    result += ({ n: '\n', r: '\r', t: '\t' })[escaped] ?? escaped;
  }
  throw parseError(format, 'unterminated string literal');
}

export class Cursor {
  constructor(source, format) {
    this.source = source;
    this.format = format;
    this.offset = 0;
  }

  eof() {
    return this.offset >= this.source.length;
  }

  peek(length = 1) {
    return this.source.slice(this.offset, this.offset + length);
  }

  take() {
    const character = this.source[this.offset];
    this.offset += 1;
    return character;
  }

  skipSpace() {
    while (/\s/.test(this.peek())) this.offset += 1;
  }

  consume(value) {
    this.skipSpace();
    if (!this.source.startsWith(value, this.offset)) {
      throw parseError(this.format, `expected ${JSON.stringify(value)} at offset ${this.offset}`);
    }
    this.offset += value.length;
  }

  tryConsume(value) {
    this.skipSpace();
    if (!this.source.startsWith(value, this.offset)) return false;
    this.offset += value.length;
    return true;
  }

  identifier(pattern = /[A-Za-z_][A-Za-z0-9_-]*/y) {
    this.skipSpace();
    pattern.lastIndex = this.offset;
    const match = pattern.exec(this.source);
    if (!match) throw parseError(this.format, `expected identifier at offset ${this.offset}`);
    this.offset = pattern.lastIndex;
    return match[0];
  }

  quoted() {
    this.skipSpace();
    const quote = this.take();
    if (quote !== '"' && quote !== "'") {
      throw parseError(this.format, `expected quoted literal at offset ${this.offset - 1}`);
    }
    return decodeQuoted(this, quote, this.format);
  }

  error(message) {
    throw parseError(this.format, `${message} at offset ${this.offset}`);
  }
}
