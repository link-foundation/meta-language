// Source-coordinate and data adapters only; shared definitions own all Unicode
// scalar, property and normalization decisions. Loading is lazy for bootstrapping.
import { readRuntimeProjectData } from '../project-data-loader.js';
import { TranslationError } from './diagnostics.js';
import { acceptSourceIdentifierScalar, decodeUnicodeSourceScalar, normalizeUnicodeScalars,
  readUnicodeSourceScalarWidth, acceptRustSourceLifetimeToken } from './unicode-scalar-rules.js';
import { decodeJavaScriptIdentifierEscape } from './javascript-identifier-rules.js';

export const SOURCE_IDENTIFIER_UNICODE_VERSION = '17.0.0';
let tables;
const empty = [];
function sourceTables() {
  tables ??= readRuntimeProjectData(new URL('../data/unicode-source-tables.lino', import.meta.url));
  if (tables.schemaVersion !== 1 || tables.unicode !== SOURCE_IDENTIFIER_UNICODE_VERSION)
    throw new TypeError('uncovered source identifier Unicode data version');
  return tables;
}
export function readSourceIdentifierScalar(source, index) {
  return decodeUnicodeSourceScalar(source.charCodeAt(index), source.charCodeAt(index + 1));
}
export function readSourceIdentifierWidth(source, index) {
  return readUnicodeSourceScalarWidth(readSourceIdentifierScalar(source, index));
}
export function acceptSourceIdentifierAt(source, index, language, first) {
  const scalar = readSourceIdentifierScalar(source, index);
  return acceptSourceIdentifierPoint(scalar, language, first);
}
export function acceptSourceIdentifierPoint(scalar, language, first) {
  if (scalar < 128) return acceptSourceIdentifierScalar(language, first, scalar, empty, empty);
  const data = sourceTables(), prefix = language === 'Rust' ? 'XID_' : 'ID_';
  return acceptSourceIdentifierScalar(language, first, scalar,
    data.properties[prefix + 'Start'], data.properties[prefix + 'Continue']);
}
export function readJavaScriptSourceIdentifier(source, start, encoding) {
  let end = start, segment = start, value = '';
  while (end < source.length) {
    if (source[end] === '\\') {
      encoding.units ??= Array.from({ length: source.length }, (_, index) => source.charCodeAt(index));
      const decoded = decodeJavaScriptIdentifierEscape(encoding.units, end);
      if (decoded[0] < 0 || !acceptSourceIdentifierPoint(decoded[1], 'JavaScript', end === start))
        throw new TranslationError('syntax', 'invalid Unicode escape in a JavaScript identifier',
          { start: end, end: decoded[0] < 0 ? decoded[1] : decoded[0] });
      value += source.slice(segment, end) + String.fromCodePoint(decoded[1]); end = decoded[0]; segment = end;
    } else {
      if (!acceptSourceIdentifierAt(source, end, 'JavaScript', end === start)) break;
      end += readSourceIdentifierWidth(source, end);
    }
  }
  value += source.slice(segment, end);
  return { kind: 'identifier', value, raw: source.slice(start, end), start, end };
}
export function acceptRustSourceLifetimeAt(source, index) {
  if (source[index] !== "'") return false;
  const first = acceptSourceIdentifierAt(source, index + 1, 'Rust', true);
  let end = index + 1;
  if (first) {
    end += readSourceIdentifierWidth(source, end);
    while (end < source.length && acceptSourceIdentifierAt(source, end, 'Rust', false)) end += readSourceIdentifierWidth(source, end);
  }
  return acceptRustSourceLifetimeToken(first, source[end] === "'");
}
export function acceptRustSourceIdentifierName(value) {
  if (typeof value !== 'string' || value.length === 0) return false;
  for (let index = 0; index < value.length; index += readSourceIdentifierWidth(value, index))
    if (!acceptSourceIdentifierAt(value, index, 'Rust', index === 0)) return false;
  return true;
}
export function normalizeRustSourceIdentifier(source, span) {
  if (/^[\x00-\x7f]*$/u.test(source)) return source;
  const input = Array.from(source, character => character.codePointAt(0));
  const data = sourceTables();
  let output;
  try { output = normalizeUnicodeScalars(input, data.classes, data.decompositions, data.values, data.pairs, 16777216); }
  catch (error) { throw new TranslationError('syntax', error.message, span); }
  return output.map(scalar => String.fromCodePoint(scalar)).join('');
}
/** Canonical Unicode 17 NFC with a bounded, shared scalar definition. */
export function normalizeUnicodeText(source) {
  if (typeof source !== 'string') throw new TypeError('Unicode normalization requires source text');
  return normalizeRustSourceIdentifier(source, { start: 0, end: source.length });
}
