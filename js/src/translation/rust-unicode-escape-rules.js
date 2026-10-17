// Generated from the shared linked Rust Unicode scalar escape policy. Do not edit.
import {readRustAsciiEscapeDigit} from './rust-cooked-string-rules.js';
/** @typedef {{ $: "scalar", code: number, end: number } | { $: "malformed" } | { $: "unsupported", end: number, escapeLength: number }} RustUnicodeEscape */
/** Read one Rust Unicode escape in the original full UTF-16 source.
 * Result offsets are relative to start, the original backslash.
 * @param {number[]} units @param {number} start @returns {RustUnicodeEscape} */
export function decodeRustUnicodeEscape(units,start) {
  if (start < 0 || start >= units.length || start % 1 !== 0
      || units[start] !== 92
      || units[start + 1] !== 117
      || units[start + 2] !== 123) return { $: "malformed" };
  let index = start + 3;
  let code = 0;
  let digits = 0;
  while (index < units.length) {
    if (units[index] === 125) {
      if (digits < 1) return { $: "malformed" };
      const end = index + 1 - start;
      if (code > 1114111 || (code >= 55296 && code <= 57343)) return { $: "unsupported", end, escapeLength: end };
      return { $: "scalar", code, end };
    }
    if (units[index] === 95) {
      if (digits < 1) return { $: "malformed" };
      index += 1;
      continue;
    }
    const digit = readRustAsciiEscapeDigit(units[index] ?? -1);
    if (digit < 0 || digits >= 6) return { $: "malformed" };
    code = code * 16 + digit;
    digits += 1;
    index += 1;
  }
  return { $: "malformed" };
}
