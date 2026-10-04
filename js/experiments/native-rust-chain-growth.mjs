// How the native Rust parse time grows with right-nested chains the oracle
// parses clean: `()=()=...` assignments and `..=..` ranges.
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const grammar = new URL('../../parity/grammars/native/rust.lino', import.meta.url);
const parser = compileGrammar(parseGrammarLinks(readFileSync(grammar, 'utf8')));
const shapes = {
  assign: (n) => `fn f() { let v: () = ${'()='.repeat(n)}(); }`,
  range: (n) => `fn f() { ${'..=..'.repeat(1)}${' ..'.repeat(n)} }`,
  rangeEq: (n) => `fn f() { ${Array.from({ length: n }, () => '..=..').join(' ')} }`,
  binary: (n) => `fn f() { ${'a+'.repeat(n)}a; }`,
  compare: (n) => `fn f() { ${'a=='.repeat(n)}a; }`,
};
const only = process.argv[2];
for (const [name, shape] of Object.entries(shapes)) {
  if (only && name !== only) continue;
  const row = [];
  for (const n of [1, 2, 4, 6, 8, 10, 12, 16, 24]) {
    const started = performance.now();
    const outcome = parser.parseTree(shape(n), { stepLimit: 5_000_000 });
    row.push(`${n}:${outcome.ok ? '' : 'X'}${Math.round(performance.now() - started)}ms`);
    if (performance.now() - started > 4000) break;
  }
  console.log(name, row.join(' '));
}
