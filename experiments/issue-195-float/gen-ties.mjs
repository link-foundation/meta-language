// Values N + j/2^s near 2^46..2^52, where shortest digits often tie: bits to
// ties-bits.txt and V8's String(x) to ties-expected.txt.
import { writeFileSync } from 'node:fs';
const view = new DataView(new ArrayBuffer(8));
const toBits = (x) => { view.setFloat64(0, x); return view.getBigUint64(0); };
let seed = 7n;
const next = () => { seed = (seed * 6364136223846793005n + 1442695040888963407n) & ((1n << 64n) - 1n); return seed; };
const values = [];
for (let i = 0; i < 20000; i += 1) {
  const shift = 40 + Number(next() % 13n);
  const base = Number(next() >> BigInt(64 - shift));
  const scale = 2 ** Number(next() % 7n);
  values.push((base + Number(next() % BigInt(scale)) / scale) * (i % 2 ? -1 : 1) / 10 ** Number(next() % 3n) * (i % 3 ? 1 : 1e-30));
}
writeFileSync('ties-bits.txt', values.map((x) => `${toBits(x)}\n`).join(''));
writeFileSync('ties-expected.txt', values.map((x) => `${String(x)}\n`).join(''));
