// Bounded draft validation. Native Rust formatting parity is intentionally
// pending CI; this experiment establishes JS-oracle agreement and automatic
// checked emission of the shared algorithm, without compiling any Rust.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { inspect } from 'node:util';
import { formatBinaryDoublePrecisionValue } from './issue-199-binary-double-precision-format.mjs';
import { renderFiniteBinaryDoublePrecision } from './issue-199-decimal-number-rules.mjs';
import { parseJavaScript } from '../js/src/translation/javascript.js';
import { checkProgram } from '../js/src/translation/check.js';
import { emitRust } from '../js/src/translation/emit-rust.js';

const values = [0, -0, NaN, Infinity, -Infinity, Number.MIN_VALUE, Number.MAX_VALUE,
  2 ** -1022, 0.1, 1e-6, 1e-7, 1e20, 1e21, 1 + 2 ** -17];
const bits = new DataView(new ArrayBuffer(8));
let state = 0x195201n;
for (let index = 0; index < 64; index += 1) {
  state = BigInt.asUintN(64, state * 6364136223846793005n + 1442695040888963407n);
  bits.setBigUint64(0, state, false);
  values.push(bits.getFloat64(0, false));
}
for (const value of values) {
  assert.equal(formatBinaryDoublePrecisionValue(value, 'ecmascript-number-text'), String(value));
  assert.equal(formatBinaryDoublePrecisionValue(value, 'node-console-number-text'), inspect(value));
}
assert.throws(() => formatBinaryDoublePrecisionValue('1'), TypeError);
assert.throws(() => formatBinaryDoublePrecisionValue(1, 'foreign-model'), TypeError);
for (const [mantissa, exponent] of [[-1n, -1074n], [1n, 0n], [0n, 0n], [1n, -1075n], [9007199254740992n, 0n]]) {
  assert.equal(renderFiniteBinaryDoublePrecision(mantissa, exponent, false, 'ecmascript-number-text'), '');
}
const source = readFileSync(new URL('./issue-199-decimal-number-rules.mjs', import.meta.url), 'utf8');
const emitted = emitRust(checkProgram(parseJavaScript(source, { referenceSemantics: false })));
assert.ok(emitted.definitions.some(definition => definition.includes('render_finite_binary_double_precision')));
console.log(`${values.length} numeric bit patterns agree with both independent JS format oracles; emitted ${emitted.definitions.length} checked Rust definitions. Native parity remains pending.`);
