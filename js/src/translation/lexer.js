// Tokenizer shared by the portable-core translation frontends. Every token
// keeps its UTF-16 offsets so diagnostics and source mappings can point at
// the original bytes; comments are retained separately because JSDoc types are
// part of the JavaScript frontend's input.

import { TranslationError } from './diagnostics.js';

const OPERATORS = {
  JavaScript: ['===', '!==', '...', '**', '&&', '||', '==', '!=', '<=', '>=', '=>', '+=', '-=', '*=', '++', '--', '??'],
  Rust: ['..=', '::', '->', '=>', '==', '!=', '<=', '>=', '&&', '||', '+=', '-=', '*=', '/=', '%=', '..'],
  Lean: [':=', '++', '->', '=>', '==', '!=', '<=', '>=', '&&', '||', '<;>', '<|', '|>', '::', '..', '←', '→', '∀', '∧', '≤', '≥', '≠', '×'],
  Rocq: [':=', '=>', '->', '<->', '<=?', '<?', '=?', '<=', '>=', '<>', '/\\', '\\/', '&&', '||', '++', '::', '%'],
};

const COMMENTS = {
  JavaScript: { line: '//', block: ['/*', '*/'], nested: false },
  Rust: { line: '//', block: ['/*', '*/'], nested: true },
  Lean: { line: '--', block: ['/-', '-/'], nested: true },
  Rocq: { line: null, block: ['(*', '*)'], nested: true },
};

export function tokenize(source, language) {
  const operators = [...OPERATORS[language]].sort((left, right) => right.length - left.length);
  const comments = COMMENTS[language];
  const tokens = [];
  const commentList = [];
  let index = 0;
  while (index < source.length) {
    const char = source[index];
    if (/\s/u.test(char)) {
      index += 1;
      continue;
    }
    if (comments.line && source.startsWith(comments.line, index)) {
      const end = source.indexOf('\n', index);
      const stop = end === -1 ? source.length : end;
      commentList.push({ text: source.slice(index, stop), start: index, end: stop });
      index = stop;
      continue;
    }
    if (source.startsWith(comments.block[0], index)) {
      const stop = blockCommentEnd(source, index, comments);
      commentList.push({ text: source.slice(index, stop), start: index, end: stop });
      index = stop;
      continue;
    }
    const start = index;
    if (language === 'JavaScript' && char === '`') {
      tokens.push(templateToken(source, index));
      index = tokens.at(-1).end;
      continue;
    }
    if (char === '"' || (language === 'JavaScript' && char === "'")) {
      const token = stringToken(source, index, language);
      tokens.push(token);
      index = token.end;
      continue;
    }
    if (language === 'Lean' && source.startsWith('s!"', index)) {
      const token = stringToken(source, index + 2, language);
      tokens.push({ ...token, kind: 'interpolation', start, raw: source.slice(start, token.end) });
      index = token.end;
      continue;
    }
    if (/[0-9]/u.test(char)) {
      const match = /^[0-9][0-9_]*(?:[a-z][a-z0-9]*)?/u.exec(source.slice(index));
      const raw = match[0];
      const digits = /^[0-9_]*/u.exec(raw)[0];
      if (digits.endsWith('_')) throw new TranslationError('syntax', `malformed number ${raw}`, { start, end: start + raw.length });
      tokens.push({
        kind: 'number',
        value: digits.replaceAll('_', ''),
        suffix: raw.slice(digits.length),
        raw,
        start,
        end: start + raw.length,
      });
      index += raw.length;
      continue;
    }
    if (isIdentifierStart(char, language)) {
      let stop = index + char.length;
      for (;;) {
        while (stop < source.length && isIdentifierPart(source[stop], language)) stop += 1;
        // Lean and Rocq qualified names (`Tree.node`, `Nat.add`) are single tokens.
        const qualified = (language === 'Lean' || language === 'Rocq')
          && source[stop] === '.' && stop + 1 < source.length && isIdentifierStart(source[stop + 1], language);
        if (!qualified) break;
        stop += 2;
      }
      if (language === 'Rust' && source[stop] === '!' && source[stop + 1] !== '=') {
        tokens.push({ kind: 'macro', value: source.slice(index, stop), start, end: stop + 1, raw: source.slice(index, stop + 1) });
        index = stop + 1;
        continue;
      }
      tokens.push({ kind: 'identifier', value: source.slice(index, stop), start, end: stop, raw: source.slice(index, stop) });
      index = stop;
      continue;
    }
    const operator = operators.find((candidate) => source.startsWith(candidate, index));
    const text = operator ?? String.fromCodePoint(source.codePointAt(index));
    tokens.push({ kind: 'punct', value: text, start, end: start + text.length, raw: text });
    index += text.length;
  }
  tokens.push({ kind: 'eof', value: '', start: source.length, end: source.length, raw: '' });
  return { tokens, comments: commentList };
}

function blockCommentEnd(source, index, comments) {
  const [open, close] = comments.block;
  let depth = 0;
  let cursor = index;
  while (cursor < source.length) {
    if (source.startsWith(open, cursor)) {
      depth += 1;
      cursor += open.length;
      if (!comments.nested && depth > 1) depth = 1;
      continue;
    }
    if (source.startsWith(close, cursor)) {
      depth -= 1;
      cursor += close.length;
      if (depth === 0) return cursor;
      continue;
    }
    cursor += 1;
  }
  throw new TranslationError('syntax', 'unterminated block comment', { start: index, end: source.length });
}

function stringToken(source, index, language) {
  const quote = source[index];
  let cursor = index + 1;
  let value = '';
  while (cursor < source.length) {
    const char = source[cursor];
    if (char === quote) {
      if (language === 'Rocq' && source[cursor + 1] === '"') {
        value += '"';
        cursor += 2;
        continue;
      }
      return { kind: 'string', value, start: index, end: cursor + 1, raw: source.slice(index, cursor + 1) };
    }
    if (char === '\\' && language !== 'Rocq') {
      const escaped = source[cursor + 1];
      const simple = { n: '\n', t: '\t', r: '\r', '\\': '\\', '"': '"', "'": "'", 0: '\0', '{': '\\{' };
      if (!(escaped in simple)) {
        throw new TranslationError('syntax', `unsupported string escape \\${escaped}`, { start: cursor, end: cursor + 2 });
      }
      value += simple[escaped];
      cursor += 2;
      continue;
    }
    if (char === '\n' && language === 'JavaScript') break;
    value += char;
    cursor += 1;
  }
  throw new TranslationError('syntax', 'unterminated string literal', { start: index, end: source.length });
}

function templateToken(source, index) {
  const parts = [];
  let cursor = index + 1;
  let text = '';
  while (cursor < source.length) {
    const char = source[cursor];
    if (char === '`') {
      parts.push({ text });
      return { kind: 'template', parts, start: index, end: cursor + 1, raw: source.slice(index, cursor + 1) };
    }
    if (char === '\\') {
      const escaped = source[cursor + 1];
      const simple = { n: '\n', t: '\t', '\\': '\\', '`': '`', $: '$' };
      if (!(escaped in simple)) {
        throw new TranslationError('syntax', `unsupported template escape \\${escaped}`, { start: cursor, end: cursor + 2 });
      }
      text += simple[escaped];
      cursor += 2;
      continue;
    }
    if (source.startsWith('${', cursor)) {
      const close = matchingBrace(source, cursor + 1);
      parts.push({ text, expression: { source: source.slice(cursor + 2, close), offset: cursor + 2 } });
      text = '';
      cursor = close + 1;
      continue;
    }
    text += char;
    cursor += 1;
  }
  throw new TranslationError('syntax', 'unterminated template literal', { start: index, end: source.length });
}

export function matchingBrace(source, open) {
  let depth = 0;
  for (let cursor = open; cursor < source.length; cursor += 1) {
    if (source[cursor] === '{') depth += 1;
    if (source[cursor] === '}') {
      depth -= 1;
      if (depth === 0) return cursor;
    }
  }
  throw new TranslationError('syntax', 'unbalanced braces', { start: open, end: source.length });
}

function isIdentifierStart(char, language) {
  if (/[A-Za-z_]/u.test(char)) return true;
  if (language === 'JavaScript' && char === '$') return true;
  return (language === 'Lean' || language === 'Rocq') && /[\p{L}]/u.test(char) && !'∀→∧≤≥≠λ×'.includes(char);
}

function isIdentifierPart(char, language) {
  if (/[A-Za-z0-9_]/u.test(char)) return true;
  if (language === 'JavaScript' && char === '$') return true;
  if (language === 'Rocq' && char === "'") return true;
  if (language === 'Lean' && (char === "'" || char === '!' || char === '?')) return true;
  return (language === 'Lean' || language === 'Rocq') && /[\p{L}\p{N}]/u.test(char) && !'∀→∧≤≥≠λ×'.includes(char);
}

/** Cursor over a token list with the helpers every frontend needs. */
export class TokenCursor {
  constructor(tokens, language) {
    this.tokens = tokens;
    this.language = language;
    this.index = 0;
  }

  peek(offset = 0) {
    return this.tokens[Math.min(this.index + offset, this.tokens.length - 1)];
  }

  next() {
    const token = this.peek();
    if (token.kind !== 'eof') this.index += 1;
    return token;
  }

  is(value, offset = 0) {
    const token = this.peek(offset);
    return (token.kind === 'punct' || token.kind === 'identifier') && token.value === value;
  }

  isKind(kind, offset = 0) {
    return this.peek(offset).kind === kind;
  }

  eat(value) {
    if (this.is(value)) return this.next();
    return undefined;
  }

  expect(value, context) {
    const token = this.peek();
    if (this.is(value)) return this.next();
    throw new TranslationError(
      'syntax',
      `expected ${value}${context ? ` in ${context}` : ''} but found ${describe(token)}`,
      token,
    );
  }

  identifier(context) {
    const token = this.peek();
    if (token.kind !== 'identifier') {
      throw new TranslationError('syntax', `expected identifier${context ? ` in ${context}` : ''} but found ${describe(token)}`, token);
    }
    return this.next();
  }

  atEnd() {
    return this.peek().kind === 'eof';
  }
}

export function describe(token) {
  return token.kind === 'eof' ? 'end of input' : `${JSON.stringify(token.raw ?? token.value)}`;
}
