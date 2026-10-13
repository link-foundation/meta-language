// Bounded cooked-string adapter for exercising the portable lexical decisions.
// It does not replace the production lexer or admit raw, byte or C strings.
import { TranslationError } from '../js/src/translation/diagnostics.js';
import { tokenize } from '../js/src/translation/lexer.js';
import { acceptRustCookedStringEscapePrefix, readRustSourceLineBreakEnd, readRustStringContinuationEnd } from './issue-199-rust-string-continuation-rules.mjs';

export function readRustStringContinuationDraft(source, start = 0) {
  if (typeof source !== 'string') throw new TypeError('source must be text');
  if (!Number.isInteger(start) || start < 0 || source[start] !== '"') throw new TypeError('a cooked string starts at a quotation mark');
  for (const character of source) {
    const scalar = character.codePointAt(0);
    if (scalar >= 0xd800 && scalar <= 0xdfff) throw new TypeError('UTF-8 source requires Unicode scalar values');
  }
  const units = Array.from({ length: source.length }, (_, index) => source.charCodeAt(index));
  const pieces = [];
  let byteCursor = Buffer.byteLength(source.slice(0, start), 'utf8');
  const utf8Start = byteCursor;
  const append = (kind, begin, end) => {
    const raw = source.slice(begin, end);
    const utf8End = byteCursor + Buffer.byteLength(raw, 'utf8');
    pieces.push({ kind, start: begin, end, utf8Start: byteCursor, utf8End, raw });
    byteCursor = utf8End;
  };
  let unchangedStart = start;
  let cursor = start + 1;
  while (cursor < source.length) {
    if (units[cursor] === 34) {
      append('unchanged source', unchangedStart, cursor + 1);
      const normalized = pieces.map(piece => piece.kind === 'string continuation' ? ''
        : piece.kind === 'source line break' ? '\n' : piece.raw).join('');
      const token = tokenize(normalized, 'Rust').tokens[0];
      return { kind: 'string', value: token.value, raw: source.slice(start, cursor + 1),
        start, end: cursor + 1, utf8Start, utf8End: byteCursor, pieces };
    }
    if (units[cursor] === 92) {
      const end = readRustStringContinuationEnd(units, cursor);
      if (end >= 0) {
        append('unchanged source', unchangedStart, cursor);
        append('string continuation', cursor, end);
        cursor = end;
        unchangedStart = end;
        continue;
      }
      // Preserve all other escapes, including escaped quotes and backslashes,
      // for the existing shared scalar decoder. Do not scan their second unit
      // as the beginning of another escape.
      if (!acceptRustCookedStringEscapePrefix(units[cursor + 1])) {
        throw new TranslationError('syntax', 'unsupported string escape in the cooked Rust profile', { start: cursor, end: cursor + 2 });
      }
      cursor += 2;
      continue;
    }
    const end = readRustSourceLineBreakEnd(units, cursor);
    if (end >= 0) {
      append('unchanged source', unchangedStart, cursor);
      append('source line break', cursor, end);
      cursor = end;
      unchangedStart = end;
      continue;
    }
    if (units[cursor] === 13) throw new TranslationError('syntax', 'isolated carriage return in a Rust string', { start: cursor, end: cursor + 1 });
    cursor += units[cursor] >= 0xd800 && units[cursor] <= 0xdbff ? 2 : 1;
  }
  throw new TranslationError('syntax', 'unterminated string literal', { start, end: source.length });
}
