// Pairs of doubles and the bits of `x % y` in V8, for checking the Lean and
// Rocq remainder helpers.  node gen-rem.mjs > rem-cases.txt
const view = new DataView(new ArrayBuffer(8));
const bits = (x) => { view.setFloat64(0, x); return view.getBigUint64(0); };
const fromBits = (b) => { view.setBigUint64(0, b); return view.getFloat64(0); };
let seed = 0x9e3779b97f4a7c15n;
const next = () => { seed ^= seed << 13n; seed &= (1n << 64n) - 1n; seed ^= seed >> 7n; seed ^= seed << 17n; seed &= (1n << 64n) - 1n; return seed; };
const specials = [0, -0, 1, -1, 1.5, -2.5, 3, 7, 1e308, -1e308, 5e-324, -5e-324, 2.2250738585072014e-308, Infinity, -Infinity, NaN, 0.1, 1e21, 2 ** 53, 2 ** 53 + 2, 1e-310];
const pairs = [];
for (const x of specials) for (const y of specials) pairs.push([x, y]);
for (let i = 0; i < 400; i += 1) pairs.push([fromBits(next()), fromBits(next())]);
for (let i = 0; i < 200; i += 1) pairs.push([Number(next() % 100000n) / 7 - 5000, Number(next() % 1000n) / 3 - 100]);
for (const [x, y] of pairs) console.log(`${bits(x)} ${bits(y)} ${bits(x % y)}`);
