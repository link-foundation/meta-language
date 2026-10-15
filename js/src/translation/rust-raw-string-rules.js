// Generated from the shared linked Rust raw string literal policy. Do not edit.
import { readRustSourceLineBreakEnd } from './rust-string-rules.js';
import { readSourceTextScalar } from './source-text-byte-rules.js';
/** @typedef {{ $: "notRaw" } | { $: "literal", contentStart: number, contentEnd: number, end: number } | { $: "tooManyDelimiters" } | { $: "carriageReturn", at: number } | { $: "nonScalar", at: number } | { $: "unterminated" }} RustRawString */
/** @param {number[]} units @param {number} start @returns {RustRawString} */
export function decodeRustRawString(units, start) {
  if (start < 0 || start >= units.length || start % 1 !== 0 || units[start] !== 114) return { $: "notRaw" };
  let opening = start + 1;
  while (opening < units.length && units[opening] === 35) opening += 1;
  if (units[opening] !== 34) return { $: "notRaw" };
  const delimiters = opening - start - 1;
  if (delimiters > 255) return { $: "tooManyDelimiters" };
  let cursor = opening + 1;
  while (cursor < units.length) {
    if (units[cursor] === 34) {
      let closing = cursor + 1;
      let matched = 0;
      while (matched < delimiters && units[closing] === 35) {
        closing += 1;
        matched += 1;
      }
      if (matched === delimiters) return { $: "literal", contentStart: opening + 1, contentEnd: cursor, end: closing };
    }
    if (units[cursor] === 13) {
      const end = readRustSourceLineBreakEnd(units, cursor);
      if (end < 0) return { $: "carriageReturn", at: cursor };
      cursor = end;
      continue;
    }
    const scalar = readSourceTextScalar(units, cursor);
    if (scalar < 0) return { $: "nonScalar", at: cursor };
    cursor += scalar >= 65536 ? 2 : 1;
  }
  return { $: "unterminated" };
}
