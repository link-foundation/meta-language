// Writes js_number_test.v: the helper plus `Eval vm_compute` of ml_js_number
// over the first N patterns of bits.txt, written as exact hex float literals.
//   node gen-rocq.mjs [N]
import { readFileSync, writeFileSync } from 'node:fs';
const count = Number(process.argv[2] ?? 2000);
const bits = readFileSync('bits.txt', 'utf8').trim().split('\n').slice(0, count).map(BigInt);
export function rocqFloat(b) {
  const negative = b >> 63n === 1n;
  const exponent = Number((b >> 52n) & 0x7ffn);
  const fraction = b & ((1n << 52n) - 1n);
  if (exponent === 0x7ff) return fraction ? 'nan' : negative ? 'neg_infinity' : 'infinity';
  if (exponent === 0 && fraction === 0n) return negative ? 'neg_zero' : '0x0p0%float';
  const hex = fraction.toString(16).padStart(13, '0');
  const body = exponent === 0 ? `0x0.${hex}p-1022` : `0x1.${hex}p${exponent - 1023}`;
  return negative ? `(-${body})%float` : `${body}%float`;
}
const head = readFileSync('js-number-rocq.v.head', 'utf8');
writeFileSync('js_number_test.v', `${head}\nImport ListNotations.\nEval vm_compute in List.map ml_js_number [${bits.map(rocqFloat).join('; ')}].\n`);
writeFileSync('expected-rocq.txt', readFileSync('expected.txt', 'utf8').trim().split('\n').slice(0, count).join('\n') + '\n');
