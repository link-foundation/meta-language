// Which characters the pinned tree-sitter oracles skip as `\s` extras: each
// candidate (every character JavaScript's `\s` matches) is placed between two
// items, and the oracle's has-error flag says whether it was white space.
import { oracleRecovers } from '../scripts/native-grammar-rows.mjs';

const candidates = [0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x20, 0x85, 0xa0, 0x1680, ...Array.from({ length: 11 }, (_, i) => 0x2000 + i),
  0x2028, 0x2029, 0x202f, 0x205f, 0x3000, 0xfeff];
const samples = { Rust: (ws) => `fn a() {}${ws}fn b() {}\n`, C: (ws) => `int a;${ws}int b;\n` };
for (const [language, sample] of Object.entries(samples)) {
  const skipped = candidates.filter((code) => !oracleRecovers(sample(String.fromCodePoint(code)), language));
  console.log(language, 'skips', skipped.map((code) => `U+${code.toString(16).toUpperCase().padStart(4, '0')}`).join(' '));
}
