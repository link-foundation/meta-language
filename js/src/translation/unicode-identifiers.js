// Source-coordinate and data adapters only; shared definitions own all Unicode
// scalar, property and normalization decisions. Loading is lazy for bootstrapping.
import { readRuntimeProjectData } from '../project-data-loader.js';
import { TranslationError } from './diagnostics.js';
import { acceptSourceIdentifierScalar, decodeUnicodeSourceScalar, normalizeUnicodeScalars,
  readUnicodeSourceScalarWidth } from './unicode-scalar-rules.js';

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
  if (scalar < 128) return acceptSourceIdentifierScalar(language, first, scalar, empty, empty);
  const data = sourceTables(), prefix = language === 'Rust' ? 'XID_' : 'ID_';
  return acceptSourceIdentifierScalar(language, first, scalar,
    data.properties[prefix + 'Start'], data.properties[prefix + 'Continue']);
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
