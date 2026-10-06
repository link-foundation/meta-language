// Parse time and steps of the first N lines of rustc's punch_card body
// (weird-exprs.rs), a chain of bare, prefix and postfix ranges and
// assignments, against its length, to see how the executor scales on it.
// Usage: node experiments/native-rust-punch-growth.mjs [maximum lines]
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL('../../parity/grammars/native/rust.lino', import.meta.url), 'utf8')));
const lines = [
  '..=..=.. ..    .. .. .. ..    .. .. .. ..    .. ..=.. ..',
  '..=.. ..=..    .. .. .. ..    .. .. .. ..    ..=..=..=..',
  '..=.. ..=..    ..=.. ..=..    .. ..=..=..    .. ..=.. ..',
  '..=..=.. ..    ..=.. ..=..    ..=.. .. ..    .. ..=.. ..',
  '..=.. ..=..    ..=.. ..=..    .. ..=.. ..    .. ..=.. ..',
  '..=.. ..=..    ..=.. ..=..    .. .. ..=..    .. ..=.. ..',
  '..=.. ..=..    .. ..=..=..    ..=..=.. ..    .. ..=.. ..',
];
const words = lines.join(' ').split(/\s+/u);
for (let count = 4; count <= Math.min(words.length, Number(process.argv[2] ?? 12)); count += 4) {
  const source = `fn f() {\n    ${words.slice(0, count).join(' ')}\n}\n`;
  const started = performance.now();
  const outcome = parser.parseTree(source, { stepLimit: 1e9 });
  console.log(count, 'words', source.length, 'bytes', outcome.ok, Math.round(performance.now() - started), 'ms', outcome.steps ?? '');
}
