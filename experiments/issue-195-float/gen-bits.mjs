// Prints N random binary64 bit patterns (decimal) to bits.txt and V8's
// String(x) of each to expected.txt, plus edge cases first.
//   node gen-bits.mjs [N] [seed]
import { writeFileSync } from 'node:fs';
const count = Number(process.argv[2] ?? 20000);
let seed = BigInt(process.argv[3] ?? 195);
const next = () => { seed = (seed * 6364136223846793005n + 1442695040888963407n) & ((1n << 64n) - 1n); return seed; };
const view = new DataView(new ArrayBuffer(8));
const toBits = (x) => { view.setFloat64(0, x); return view.getBigUint64(0); };
const fromBits = (b) => { view.setBigUint64(0, b); return view.getFloat64(0); };
const edge = [0, -0, NaN, Infinity, -Infinity, 5e-324, -5e-324, 2.2250738585072014e-308, 2.225073858507201e-308,
  1.7976931348623157e308, 1e21, 1e-7, 1e-6, 123e-9, 0.1 + 0.2, 0.1, 42, 6 * 7, 1 / 3, 2 / 3, 1e23, 9007199254740993,
  9007199254740992, 2 ** 53 + 2, 123456789012345680000, 1e20, 1.5, -1.5, 100, 0.000001, 0.0000001, 1e300, 4.35, 0.3,
  2 ** -1022, 2 ** -1074 * 3, 2 ** 1023, 5e-310, 1.2345678901234567e-300];
const bits = edge.map(toBits);
for (let i = 0; i < count; i += 1) {
  const r = next();
  // mix fully random patterns with random integers and short decimals
  if (i % 4 === 0) bits.push(toBits(Number((r >> 11n) % 100000n) / [1, 10, 100, 1000][Number(r & 3n)]));
  else if (i % 4 === 1) bits.push(toBits(Number(r >> 30n)));
  else bits.push(r);
}
writeFileSync('bits.txt', bits.map(String).join('\n') + '\n');
writeFileSync('expected.txt', bits.map((b) => String(fromBits(b))).join('\n') + '\n');
