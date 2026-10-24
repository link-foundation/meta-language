// Tokenizer shared by the portable-core translation frontends. Every token
// keeps its UTF-16 offsets so diagnostics and source mappings can point at
// the original bytes; comments are retained separately because JSDoc types are
// part of the JavaScript frontend's input.

import { TranslationError } from './diagnostics.js';
import { acceptControlParenthesis, acceptStatementBrace, decodeUnicodeEscape, readRustNumberBounds, regularExpressionEnd, startRegularExpression } from './frontend-rules.js';
import { readRustSourceLineBreakEnd, readRustStringContinuationEnd } from './rust-string-rules.js';
import { acceptRustCookedStringEscapePrefix, readRustAsciiEscapeValue } from './rust-cooked-string-rules.js';
import { decodeRustUnicodeEscape } from './rust-unicode-escape-rules.js';
import { decodeRustRawString } from './rust-raw-string-rules.js';
import { decodeRustCharacterLiteral } from './rust-character-rules.js';
import { findJavaScriptLineTerminator } from './javascript-line-terminators.js';
import { acceptRustSourceWhitespace, acceptRustSourceByteOrderMark, acceptRustSourceNumberDigitBoundary } from './rust-source-whitespace-rules.js';
import { acceptRustSourceKeywordToken } from './rust-record-rules.js';
import { readRustSourceRawIdentifierError } from './unicode-scalar-rules.js';
import { acceptSourceIdentifierAt, normalizeRustSourceIdentifier, readSourceIdentifierWidth, readJavaScriptSourceIdentifier, acceptRustSourceLifetimeAt } from './unicode-identifiers.js';
import { acceptJavaScriptSourceKeywordToken, acceptJavaScriptBindingIdentifierContext, readJavaScriptSourceTokenGoalValue, acceptJavaScriptNumberIdentifierBoundary } from './javascript-identifier-rules.js';
import { requireJavaScriptIdentifierRole } from './javascript-identifier-roles.js';
import {decodeJavaScriptStringUnicodeEscape,requireJavaScriptStringUnitTransport} from './javascript-string-unit-rules.js';
export { SOURCE_IDENTIFIER_UNICODE_VERSION } from './unicode-identifiers.js';

const OPERATORS = {
  JavaScript: [
    '>>>=', '===', '!==', '...', '**=', '<<=', '>>=', '>>>', '&&=', '||=', '??=',
    '**', '&&', '||', '==', '!=', '<=', '>=', '=>', '+=', '-=', '*=', '/=', '%=', '&=', '|=', '^=', '++', '--', '??', '?.', '<<', '>>',
  ],
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
const javascriptGoalValue = token => readJavaScriptSourceTokenGoalValue(token?.kind ?? '', token?.value ?? '', token?.raw ?? '');

export function tokenize(source, language) {
  return tokenizeRegion(source, language, 0, false, { units: null });
}

// Reuse the ordinary lexical decisions when finding a template substitution.
// Opaque string, comment, regex and nested-template tokens cannot close it.
function tokenizeRegion(source, language, startIndex, braced, encoding) {
  const operators = [...OPERATORS[language]].sort((left, right) => right.length - left.length);
  const comments = COMMENTS[language];
  const tokens = [];
  const commentList = [];
  // Encode once, on the first regular expression. Copying each remaining
  // suffix makes a module with many literals take quadratic time and space.
  const scopes = [];
  const braces = [];
  let closed = false;
  let declarationBody = false;
  let depth = 0;
  let index = startIndex;
  while (index < source.length) {
    const char = source[index];
    if (braced && char === '}' && depth === 0) break;
    // Classify the current one-unit character without another source-unit read.
    // Full-source encoding remains lazy and shared by the escape algorithms.
    const rustUnit = language === 'Rust' ? char.codePointAt(0) : -1;
    if (language === 'Rust'
      ? acceptRustSourceWhitespace(rustUnit) || acceptRustSourceByteOrderMark(rustUnit, index === 0)
      : /\s/u.test(char)) {
      index += 1;
      continue;
    }
    if (comments.line && source.startsWith(comments.line, index)) {
      const end = language === 'JavaScript' ? findJavaScriptLineTerminator(source, index) : source.indexOf('\n', index);
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
    if (language === 'JavaScript' && char === '/' && (closed || startRegularExpression(['.', '?.'].includes(tokens.at(-2)?.value) ? 'member' : tokens.at(-1)?.kind ?? '', javascriptGoalValue(tokens.at(-1))))) {
      encoding.units ??= Array.from({ length: source.length }, (_, offset) => source.charCodeAt(offset));
      const end = regularExpressionEnd(encoding.units, index);
      if (end < 0) throw new TranslationError('syntax', 'unterminated regular expression literal', { start, end: source.length });
      tokens.push({ kind: 'regex', value: '/', raw: source.slice(index, end), start, end });
      index = end;
      closed = false;
      continue;
    }
    const controlClosed = closed;
    closed = false;
    if (language === 'JavaScript' && char === '`') {
      tokens.push(templateToken(source, index, encoding));
      index = tokens.at(-1).end;
      continue;
    }
    if (language === 'Rust' && source.startsWith('r#', index) && isIdentifierStartAt(source, index + 2, language)) {
      let end=index+2+readSourceIdentifierWidth(source,index+2);
      while(end<source.length&&isIdentifierPartAt(source,end,language))end+=readSourceIdentifierWidth(source,end);
      const name=normalizeRustSourceIdentifier(source.slice(index+2,end),{start,end}),message=readRustSourceRawIdentifierError(name,true);
      if(message)throw new TranslationError('syntax',message,{start,end});
      const macro=source[end]==='!'&&source[end+1]!=='=';
      if(macro)end+=1;
      tokens.push({kind:macro?'macro':'identifier',value:name,raw:source.slice(index,end),start,end});
      index=end;continue;
    }
    if (language === 'Rust' && char === 'r') {
      encoding.units ??= Array.from({ length: source.length }, (_, offset) => source.charCodeAt(offset));
      const literal = decodeRustRawString(encoding.units, index);
      if (literal.$ === 'tooManyDelimiters') throw new TranslationError('syntax', 'Rust raw string exceeds 255 delimiter hashes', { start, end: source.length });
      if (literal.$ === 'unterminated') throw new TranslationError('syntax', 'unterminated Rust raw string literal', { start, end: source.length });
      if (literal.$ === 'carriageReturn') throw new TranslationError('syntax', 'isolated carriage return in a Rust string', { start: literal.at, end: literal.at + 1 });
      if (literal.$ === 'nonScalar') throw new TranslationError('syntax', 'unpaired Unicode surrogate in a Rust raw string', { start: literal.at, end: literal.at + 1 });
      if (literal.$ === 'literal') {
        let value = '';
        let cursor = literal.contentStart;
        while (cursor < literal.contentEnd) {
          const end = readRustSourceLineBreakEnd(encoding.units, cursor);
          if (end >= 0) { value += '\n'; cursor = end; }
          else { value += source[cursor]; cursor += 1; }
        }
        tokens.push({ kind: 'string', value, start, end: literal.end, raw: source.slice(start, literal.end) });
        index = literal.end;
        continue;
      }
    }
    // Select only Rust apostrophe/byte-apostrophe prefixes. A notCharacter
    // result consumes nothing, so existing lifetime and label paths stay exact.
    if (language === 'Rust' && (char === "'" || (char === 'b' && source[index + 1] === "'")) && !acceptRustSourceLifetimeAt(source, index)) {
      encoding.units ??= Array.from({ length: source.length }, (_, offset) => source.charCodeAt(offset));
      const literal = decodeRustCharacterLiteral(encoding.units, index);
      if (literal.$ === 'malformed') throw new TranslationError('syntax', 'malformed Rust character literal', { start: literal.at, end: literal.end });
      if (literal.$ === 'unterminated') throw new TranslationError('syntax', 'unterminated Rust character literal', { start: literal.at, end: literal.end });
      if (literal.$ === 'nonScalar') throw new TranslationError('unsupported', 'Rust character literal is not a Unicode scalar value', { start: literal.at, end: literal.end });
      if (literal.$ === 'nonAscii') throw new TranslationError('syntax', 'non-ASCII source in a Rust byte character literal', { start: literal.at, end: literal.end });
      if (literal.$ === 'unicodeByteEscape') throw new TranslationError('syntax', 'Unicode escape in a Rust byte character literal', { start: literal.at, end: literal.end });
      if (literal.$ === 'unsupportedSuffix') throw new TranslationError('unsupported', 'Rust character literal suffix is outside the supported lexical profile', { start: literal.at, end: literal.end });
      if (literal.$ === 'char' || literal.$ === 'byteChar') {
        // Token.value is text in both runtimes. Distinct opaque kinds never
        // enter the existing String or Number semantic admission branches.
        tokens.push({ kind: literal.$, value: String(literal.value), start, end: literal.end, raw: source.slice(start, literal.end) });
        index = literal.end;
        continue;
      }
    }
    if (char === '"' || (language === 'JavaScript' && char === "'")) {
      const token = stringToken(source, index, language, encoding);
      tokens.push(token);
      index = token.end;
      continue;
    }
    if (language === 'Lean' && source.startsWith('s!"', index)) {
      const token = stringToken(source, index + 2, language, encoding);
      tokens.push({ ...token, kind: 'interpolation', start, raw: source.slice(start, token.end) });
      index = token.end;
      continue;
    }
    if (language === 'JavaScript' && (/[0-9]/u.test(char) || (char === '.' && /[0-9]/u.test(source[index + 1] ?? '')))) {
      const token = javascriptNumber(source, index);
      tokens.push(token);
      index = token.end;
      continue;
    }
    if (/[0-9]/u.test(char)) {
      if (language === 'Rust') {
        encoding.units ??= Array.from({ length: source.length }, (_, offset) => source.charCodeAt(offset));
        const [end, suffixStart, floating] = readRustNumberBounds(encoding.units, index);
        const raw = source.slice(index, end);
        const digits = source.slice(index, suffixStart);
        if (!acceptRustSourceNumberDigitBoundary(floating, digits.endsWith('_'), source.slice(suffixStart, end), isIdentifierPartAt(source, end, 'Rust'), end < source.length ? source.charCodeAt(end) : -1)) throw new TranslationError('syntax', `malformed number ${raw}`, { start, end });
        if (floating && !Number.isFinite(Number(digits.replaceAll('_', '')))) throw new TranslationError('syntax', `floating-point literal out of range ${raw}`, { start, end });
        tokens.push({ kind: 'number', ...(floating && { floating: true }), value: floating ? String(Number(digits.replaceAll('_', ''))) : digits.replaceAll('_', ''), suffix: source.slice(suffixStart, end), raw, start, end });
        index = end;
        continue;
      }
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
    if (language === 'JavaScript' && (char === '\\' || isIdentifierStartAt(source, index, language))) {
      const token = readJavaScriptSourceIdentifier(source, index, encoding);
      tokens.push(token); index = token.end; continue;
    }
    if (isIdentifierStartAt(source, index, language)) {
      let stop = index + (language === 'Rust' || language === 'JavaScript' ? readSourceIdentifierWidth(source, index) : char.length);
      for (;;) {
        while (stop < source.length && isIdentifierPartAt(source, stop, language)) stop += language === 'Rust' || language === 'JavaScript' ? readSourceIdentifierWidth(source, stop) : 1;
        // Lean and Rocq qualified names (`Tree.node`, `Nat.add`) are single tokens.
        const qualified = (language === 'Lean' || language === 'Rocq')
          && source[stop] === '.' && stop + 1 < source.length && isIdentifierStartAt(source, stop + 1, language);
        if (!qualified) break;
        stop += 2;
      }
      if (language === 'Rust' && source[stop] === '!' && source[stop + 1] !== '=') {
        tokens.push({ kind: 'macro', value: normalizeRustSourceIdentifier(source.slice(index, stop), { start, end: stop }), start, end: stop + 1, raw: source.slice(index, stop + 1) });
        index = stop + 1;
        continue;
      }
      tokens.push({ kind: 'identifier', value: language === 'Rust' ? normalizeRustSourceIdentifier(source.slice(index, stop), { start, end: stop }) : source.slice(index, stop), start, end: stop, raw: source.slice(index, stop) });
      index = stop;
      continue;
    }
    const operator = operators.find((candidate) => source.startsWith(candidate, index));
    const text = operator ?? String.fromCodePoint(source.codePointAt(index));
    if (braced && text === '{') depth += 1;
    if (braced && text === '}') depth -= 1;
    if (language === 'JavaScript') {
      if (text === '(') {
        const declaration = javascriptGoalValue(tokens.at(-2)) === 'function' && (tokens.length === 2 || [';', '{', '}', 'export', 'async'].includes(javascriptGoalValue(tokens.at(-3))));
        scopes.push({ control: acceptControlParenthesis(tokens.at(-1)?.kind ?? '', javascriptGoalValue(tokens.at(-1)), ['.', '?.'].includes(tokens.at(-2)?.value)), declaration });
      }
      if (text === ')') {
        const scope = scopes.pop();
        closed = scope?.control ?? false;
        declarationBody = scope?.declaration ?? false;
      } else if (text === '{') {
        braces.push(acceptStatementBrace(tokens.at(-1)?.kind ?? (braced ? 'expression' : ''), javascriptGoalValue(tokens.at(-1)), controlClosed, declarationBody));
        declarationBody = false;
      } else if (text === '}') closed = braces.pop() ?? false;
      else declarationBody = false;
    }
    tokens.push({ kind: 'punct', value: text, start, end: start + text.length, raw: text });
    index += text.length;
  }
  tokens.push({ kind: 'eof', value: '', start: index, end: index, raw: '' });
  return { tokens, comments: commentList };
}

const DIGITS = { 16: '[0-9a-fA-F]', 8: '[0-7]', 2: '[01]', 10: '[0-9]' };
const run = (digit) => `${digit}(?:_?${digit})*`;
const JS_NUMBER = new RegExp(
  `^(?:0[xX]${run(DIGITS[16])}|0[oO]${run(DIGITS[8])}|0[bB]${run(DIGITS[2])}`
  + `|(?:(?:0|[1-9](?:_?${run(DIGITS[10])})?)(?:\\.(?:${run(DIGITS[10])})?)?|\\.${run(DIGITS[10])})`
  + `(?:[eE][+-]?${run(DIGITS[10])})?)n?`,
  'u',
);

/**
 * An ECMAScript numeric literal. A BigInt (`5n`, `0x1fn`) keeps its decimal
 * digits as `value` with suffix `n`; a Number keeps `String(Number(raw))`, the
 * value JavaScript prints, with an empty suffix.
 */
function javascriptNumber(source, index) {
  const match = JS_NUMBER.exec(source.slice(index));
  const raw = match?.[0] ?? source[index];
  const end = index + raw.length;
  const next = end < source.length ? String.fromCodePoint(source.codePointAt(end)) : '';
  const malformed = () => new TranslationError('syntax', `malformed number ${source.slice(index, end + next.length)}`, { start: index, end: end + next.length });
  // `08`, `1_`, `3in` and `1.5n` are SyntaxErrors in JavaScript.
  if (!match || !acceptJavaScriptNumberIdentifierBoundary(acceptSourceIdentifierAt(source, end, 'JavaScript', true), end < source.length ? source.charCodeAt(end) : -1)) throw malformed();
  const text = raw.replaceAll('_', '');
  if (text.endsWith('n')) {
    if (/[.eE]/u.test(text) && !/^0[xX]/u.test(text)) throw malformed();
    return { kind: 'number', value: BigInt(text.slice(0, -1)).toString(), suffix: 'n', raw, start: index, end };
  }
  return { kind: 'number', value: String(Number(text)), suffix: '', raw, start: index, end };
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

// A \u escape at `at`: \u{X..} in JavaScript and Rust, and \uXXXX in
// JavaScript, where a surrogate pair of two such escapes is one code point.
// JavaScript strings retain isolated surrogate units; Rust requires scalars.
function unicodeEscape(source, at, language, encoding = { units: null }) {
  if(language==='JavaScript'){
    encoding.units??=Array.from({length:source.length},(_,index)=>source.charCodeAt(index));
    const escape=decodeJavaScriptStringUnicodeEscape(encoding.units,at);
    if(escape.$==='malformed')throw new TranslationError('syntax','malformed unicode escape',{start:at,end:at+2});
    return{text:escape.units.map(unit=>String.fromCharCode(unit)).join(''),end:at+escape.end};
  }
  let escape;
  if (language === 'Rust') {
    encoding.units ??= Array.from({ length: source.length }, (_, offset) => source.charCodeAt(offset));
    escape = decodeRustUnicodeEscape(encoding.units, at);
  } else {
    const units = [];
    for (let index = at; index < Math.min(source.length, at + 12); index += 1) units.push(source.charCodeAt(index));
    escape = decodeUnicodeEscape(units, language === 'JavaScript');
  }
  if (escape.$ === 'malformed') throw new TranslationError('syntax', 'malformed unicode escape', { start: at, end: at + 2 });
  if (escape.$ === 'unsupported') {
    throw new TranslationError('unsupported', `unicode escape ${source.slice(at, at + escape.escapeLength)} is not a scalar value; Rust strings hold scalar values only`, { start: at, end: at + escape.end });
  }
  return { text: String.fromCodePoint(escape.code), end: at + escape.end };
}

function stringToken(source, index, language, encoding) {
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
      const stringUnits=Array.from({length:value.length},(_,at)=>value.charCodeAt(at));
      const transport=language==='JavaScript'&&requireJavaScriptStringUnitTransport(stringUnits);
      return { kind: 'string', value:transport?value.toWellFormed():value,...(transport?{stringUnits}:{}),start: index, end: cursor + 1, raw: source.slice(index, cursor + 1) };
    }
    if (char === '\\' && language !== 'Rocq') {
      const escaped = source[cursor + 1];
      if (language === 'Rust') {
        encoding.units ??= Array.from({ length: source.length }, (_, offset) => source.charCodeAt(offset));
        const end = readRustStringContinuationEnd(encoding.units, cursor);
        if (end >= 0) { cursor = end; continue; }
        if (!acceptRustCookedStringEscapePrefix(encoding.units[cursor + 1] ?? -1)) {
          throw new TranslationError('syntax', `unsupported string escape \\${escaped}`, { start: cursor, end: cursor + 2 });
        }
        if (escaped === 'x') {
          const valueUnit = readRustAsciiEscapeValue(encoding.units[cursor + 2] ?? -1, encoding.units[cursor + 3] ?? -1);
          if (valueUnit < 0) throw new TranslationError('syntax', 'malformed Rust ASCII escape', { start: cursor, end: cursor + 2 });
          value += String.fromCharCode(valueUnit);
          cursor += 4;
          continue;
        }
      }
      if (language === 'JavaScript' && (/[1-9]/u.test(escaped ?? '') || (escaped === '0' && /[0-9]/u.test(source[cursor + 2] ?? '')))) {
        throw new TranslationError('syntax', 'legacy octal string escape', { start: cursor, end: cursor + 2 });
      }
      if (language === 'JavaScript' && ['\n', '\r', '\u2028', '\u2029'].includes(escaped)) {
        cursor += escaped === '\r' && source[cursor + 2] === '\n' ? 3 : 2;
        continue;
      }
      const simple = { n: '\n', t: '\t', r: '\r', b: '\b', f: '\f', v: '\v', '\\': '\\', '"': '"', "'": "'", 0: '\0', '{': language === 'JavaScript' ? '{' : '\\{' };
      if (escaped === 'u' || (language === 'JavaScript' && escaped === 'x')) {
        const escape = unicodeEscape(source, cursor, language, encoding);
        value += escape.text;
        cursor = escape.end;
        continue;
      }
      if (!(escaped in simple) && (language !== 'JavaScript' || escaped === undefined)) {
        throw new TranslationError('syntax', `unsupported string escape \\${escaped}`, { start: cursor, end: cursor + 2 });
      }
      value += simple[escaped] ?? escaped;
      cursor += 2;
      continue;
    }
    if (language === 'Rust' && char === '\r') {
      encoding.units ??= Array.from({ length: source.length }, (_, offset) => source.charCodeAt(offset));
      const end = readRustSourceLineBreakEnd(encoding.units, cursor);
      if (end >= 0) { value += '\n'; cursor = end; continue; }
      throw new TranslationError('syntax', 'isolated carriage return in a Rust string', { start: cursor, end: cursor + 1 });
    }
    if ((char === '\n' || char === '\r') && language === 'JavaScript') break;
    value += char;
    cursor += 1;
  }
  throw new TranslationError('syntax', 'unterminated string literal', { start: index, end: source.length });
}

function javascriptTemplatePart(text) {
  const stringUnits=Array.from({length:text.length},(_,at)=>text.charCodeAt(at));
  return requireJavaScriptStringUnitTransport(stringUnits)?{text:text.toWellFormed(),stringUnits}:{text};
}

function templateToken(source, index, encoding) {
  const parts = [];
  let cursor = index + 1;
  let text = '';
  while (cursor < source.length) {
    const char = source[cursor];
    if (char === '`') {
      parts.push(javascriptTemplatePart(text));
      return { kind: 'template', parts, start: index, end: cursor + 1, raw: source.slice(index, cursor + 1) };
    }
    if (char === '\\') {
      const escaped = source[cursor + 1];
      if (['\n', '\r', '\u2028', '\u2029'].includes(escaped)) {
        cursor += escaped === '\r' && source[cursor + 2] === '\n' ? 3 : 2;
        continue;
      }
      const simple = { n: '\n', t: '\t', r: '\r', b: '\b', f: '\f', v: '\v', 0: '\0', '\\': '\\', '`': '`', $: '$' };
      if (escaped === 'u' || escaped === 'x') {
        const escape = unicodeEscape(source, cursor, 'JavaScript');
        text += escape.text;
        cursor = escape.end;
        continue;
      }
      if (/[1-9]/u.test(escaped ?? '') || (escaped === '0' && /[0-9]/u.test(source[cursor + 2] ?? ''))) {
        throw new TranslationError('syntax', 'legacy octal template escape', { start: cursor, end: cursor + 3 });
      }
      if (escaped === undefined) {
        throw new TranslationError('syntax', `unsupported template escape \\${escaped}`, { start: cursor, end: cursor + 2 });
      }
      text += simple[escaped] ?? escaped;
      cursor += 2;
      continue;
    }
    if (char === '\r') {
      text += '\n';
      cursor += source[cursor + 1] === '\n' ? 2 : 1;
      continue;
    }
    if (source.startsWith('${', cursor)) {
      const close = matchingBrace(source, cursor + 1, encoding);
      parts.push({ ...javascriptTemplatePart(text), expression: { source: source.slice(cursor + 2, close), offset: cursor + 2 } });
      text = '';
      cursor = close + 1;
      continue;
    }
    text += char;
    cursor += 1;
  }
  throw new TranslationError('syntax', 'unterminated template literal', { start: index, end: source.length });
}

export function matchingBrace(source, open, encoding = { units: null }) {
  const close = tokenizeRegion(source, 'JavaScript', open + 1, true, encoding).tokens.at(-1).end;
  if (close < source.length) return close;
  throw new TranslationError('syntax', 'unbalanced braces', { start: open, end: source.length });
}

function isIdentifierStartAt(source, index, language) {
  if (language === 'Rust' || language === 'JavaScript') return acceptSourceIdentifierAt(source, index, language, true);
  const char = source[index] ?? '';
  if (/[A-Za-z_]/u.test(char)) return true;
  if (language === 'JavaScript' && char === '$') return true;
  return (language === 'Lean' || language === 'Rocq') && /[\p{L}]/u.test(char) && !'∀→∧≤≥≠λ×'.includes(char);
}

function isIdentifierPartAt(source, index, language) {
  if (language === 'Rust' || language === 'JavaScript') return acceptSourceIdentifierAt(source, index, language, false);
  const char = source[index] ?? '';
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
    return (token.kind === 'punct' || token.kind === 'identifier') && token.value === value
      && (this.language!=='Rust'||acceptRustSourceKeywordToken(token.value,token.raw))
      && (this.language!=='JavaScript'||acceptJavaScriptSourceKeywordToken(token.value,token.raw));
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
    if (this.language === 'JavaScript' && acceptJavaScriptBindingIdentifierContext(context ?? '')) requireJavaScriptIdentifierRole(token, true);
    return this.next();
  }

  atEnd() {
    return this.peek().kind === 'eof';
  }
}

export function describe(token) {
  return token.kind === 'eof' ? 'end of input' : `${JSON.stringify(token.raw ?? token.value)}`;
}
