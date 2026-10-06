// Writes float_rem_test.v: the Rocq remainder helper on the V8 cases of
// rem-cases.txt, counting the cases where it disagrees (SameValue).
//   node gen-rem-rocq.mjs && rocq c float_rem_test.v
import { readFileSync, writeFileSync } from 'node:fs';

const view = new DataView(new ArrayBuffer(8));
const fromBits = (b) => { view.setBigUint64(0, b); return view.getFloat64(0); };
/** An exact Rocq float literal: an integer mantissa in hexadecimal times a power of two. */
export function rocqFloat(x) {
  if (Number.isNaN(x)) return 'nan';
  if (x === Infinity) return 'infinity';
  if (x === -Infinity) return 'neg_infinity';
  if (x === 0) return Object.is(x, -0) ? '(-0)' : '0';
  view.setFloat64(0, x);
  const b = view.getBigUint64(0);
  const exponent = Number((b >> 52n) & 0x7ffn);
  const fraction = b & ((1n << 52n) - 1n);
  const [mantissa, power] = exponent === 0 ? [fraction, -1074] : [fraction | (1n << 52n), exponent - 1075];
  const text = `0x${mantissa.toString(16)}p${power}`;
  return x < 0 ? `(-${text})` : text;
}
const helper = readFileSync(new URL('./float-rem.v.head', import.meta.url), 'utf8');
const cases = readFileSync(new URL('./rem-cases.txt', import.meta.url), 'utf8').trim().split('\n')
  .map((line) => line.split(' ').map((field) => fromBits(BigInt(field))));
const list = cases.map(([x, y, r]) => `(${rocqFloat(x)}, ${rocqFloat(y)}, ${rocqFloat(r)})`).join(';\n  ');
writeFileSync(new URL('./float_rem_test.v', import.meta.url), `${helper}
Open Scope float_scope.
Definition cases : list (float * float * float) := [
  ${list}].
Definition bad := List.filter (fun c => let '(x, y, r) := c in negb (ml_float_same (ml_float_rem x y) r)) cases.
Eval vm_compute in (List.length cases, List.length bad).
Eval vm_compute in List.firstn 5 bad.
`);
