// Shared bounded Rust character and byte-character lexical decisions.
// The generator retains every actual dependency; typed character semantics stay refused.
import { readSourceTextScalar } from './source-text-byte-rules.js';
import { acceptRustCookedStringEscapePrefix, readRustAsciiEscapeDigit, readRustAsciiEscapeValue } from './rust-cooked-string-rules.js';
import { decodeRustUnicodeEscape } from './rust-unicode-escape-rules.js';
import { acceptRustSourceHeaderLifetimeCharacter } from './rust-source-header-emission-rules.js';
import { acceptRustSourceWhitespace } from './rust-source-whitespace-rules.js';

/** @typedef {{ $: 'notCharacter' } | { $: 'char', value: number, contentStart: number, contentEnd: number, end: number } | { $: 'byteChar', value: number, contentStart: number, contentEnd: number, end: number } | { $: 'malformed', at: number, end: number } | { $: 'unterminated', at: number, end: number } | { $: 'nonScalar', at: number, end: number } | { $: 'nonAscii', at: number, end: number } | { $: 'unicodeByteEscape', at: number, end: number } | { $: 'unsupportedSuffix', at: number, end: number }} RustCharacterLiteral */

// The existing digit classifier is canonical. Unlike the existing cooked
// character/string ASCII helper, this byte operation permits both digits 0..15.
/** @param {number} highUnit @param {number} lowUnit @returns {number} */
export function readRustCharacterByteEscapeValue(highUnit, lowUnit) {
  const high = readRustAsciiEscapeDigit(highUnit);
  const low = readRustAsciiEscapeDigit(lowUnit);
  if (high < 0 || low < 0) return -1;
  return high * 16 + low;
}

// Finite value mapping corresponding to the existing cooked-string prefix
// policy and stringToken's simple-value table. No second hex/scalar decoder.
/** @param {number} unit @returns {number} */
export function readRustCharacterSimpleEscapeValue(unit) {
  if (unit === 110) return 10;
  if (unit === 114) return 13;
  if (unit === 116) return 9;
  if (unit === 48) return 0;
  if (unit === 92 || unit === 39 || unit === 34) return unit;
  return -1;
}

/** @param {number[]} units @param {boolean} byte @param {number} value @param {number} contentStart @param {number} contentEnd @returns {RustCharacterLiteral} */
export function finishRustCharacterLiteral(units, byte, value, contentStart, contentEnd) {
  if (contentEnd >= units.length) return { $: 'unterminated', at: contentStart, end: units.length };
  if (units[contentEnd] !== 39) return { $: 'malformed', at: contentEnd, end: contentEnd + 1 };
  const end = contentEnd + 1;
  // The bounded draft recognizes unsuffixed literal tokens. Rust token syntax
  // permits identifier suffixes even on chars/bytes; those are NOT declared
  // malformed. Current source identifiers are ASCII, so known ASCII starts and
  // any unproved non-ASCII suffix-looking continuation refuse explicitly.
  const following = units[end] ?? -1;
  if (acceptRustSourceHeaderLifetimeCharacter(following, true)) {
    return { $: 'unsupportedSuffix', at: end, end: end + 1 };
  }
  if (following >= 128 && !acceptRustSourceWhitespace(following)) {
    const scalar = readSourceTextScalar(units, end);
    if (scalar < 0) return { $: 'nonScalar', at: end, end: end + 1 };
    return { $: 'unsupportedSuffix', at: end, end: end + (scalar >= 65536 ? 2 : 1) };
  }
  if (byte) return { $: 'byteChar', value, contentStart, contentEnd, end };
  return { $: 'char', value, contentStart, contentEnd, end };
}

/**
 * Read one unsuffixed character/byte-character token at its absolute start.
 * A plain apostrophe without exactly one scalar and a closing apostrophe
 * returns notCharacter, preserving the existing lifetime/label token path.
 * It does not authenticate that the remaining source is a valid lifetime.
 * @param {number[]} units @param {number} start @returns {RustCharacterLiteral}
 */
export function decodeRustCharacterLiteral(units, start) {
  if (!Number.isInteger(start) || start < 0 || start >= units.length) return { $: 'notCharacter' };
  const byte = units[start] === 98 && units[start + 1] === 39;
  if (!byte && units[start] !== 39) return { $: 'notCharacter' };
  const contentStart = start + (byte ? 2 : 1);
  if (contentStart >= units.length) return { $: 'unterminated', at: start, end: units.length };
  const first = units[contentStart] ?? -1;
  if (first === 39 || first === 9 || first === 10 || first === 13) {
    return { $: 'malformed', at: contentStart, end: contentStart + 1 };
  }
  if (first !== 92) {
    const scalar = readSourceTextScalar(units, contentStart);
    if (scalar < 0) return { $: 'nonScalar', at: contentStart, end: contentStart + 1 };
    const contentEnd = contentStart + (scalar >= 65536 ? 2 : 1);
    // Preserve every nonliteral apostrophe path, including Unicode spellings
    // outside today's identifier profile. This branch consumes nothing.
    if (!byte && units[contentEnd] !== 39) return { $: 'notCharacter' };
    if (byte && scalar > 127) return { $: 'nonAscii', at: contentStart, end: contentEnd };
    return finishRustCharacterLiteral(units, byte, scalar, contentStart, contentEnd);
  }
  const escaped = units[contentStart + 1] ?? -1;
  if (!acceptRustCookedStringEscapePrefix(escaped)) {
    return { $: 'malformed', at: contentStart, end: Math.min(contentStart + 2, units.length) };
  }
  if (escaped === 120) {
    const high = units[contentStart + 2] ?? -1;
    const low = units[contentStart + 3] ?? -1;
    const value = byte ? readRustCharacterByteEscapeValue(high, low) : readRustAsciiEscapeValue(high, low);
    if (value < 0) return { $: 'malformed', at: contentStart, end: Math.min(contentStart + 4, units.length) };
    return finishRustCharacterLiteral(units, byte, value, contentStart, contentStart + 4);
  }
  if (escaped === 117) {
    if (byte) return { $: 'unicodeByteEscape', at: contentStart, end: Math.min(contentStart + 2, units.length) };
    const scalar = decodeRustUnicodeEscape(units, contentStart);
    if (scalar.$ === 'malformed') return { $: 'malformed', at: contentStart, end: Math.min(contentStart + 2, units.length) };
    if (scalar.$ === 'unsupported') return { $: 'nonScalar', at: contentStart, end: contentStart + scalar.end };
    return finishRustCharacterLiteral(units, false, scalar.code, contentStart, contentStart + scalar.end);
  }
  const value = readRustCharacterSimpleEscapeValue(escaped);
  if (value < 0) return { $: 'malformed', at: contentStart, end: Math.min(contentStart + 2, units.length) };
  return finishRustCharacterLiteral(units, byte, value, contentStart, contentStart + 2);
}

/** @param {string} kind @returns {string} */
export function readRustCharacterExpressionType(kind) {
  return kind === 'byteChar' ? 'u8' : '';
}

/** @param {string} language @param {string} typeKind @param {boolean} signed @returns {boolean} */
export function acceptRustNumericLiteralNegation(language, typeKind, signed) {
  return language !== 'Rust' || typeKind !== 'fixed' || signed;
}
