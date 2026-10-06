// The step count and time the native Rust grammar needs for each item of a
// file (split as in native-rust-first-failure.mjs) and for the whole file,
// against the default budget of 100000 + 1000 per byte.
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const grammar = new URL('../../parity/grammars/native/rust.lino', import.meta.url);
const parser = compileGrammar(parseGrammarLinks(readFileSync(grammar, 'utf8')));
const source = readFileSync(process.argv[2], 'utf8');
function needed(text) {
  let low = 0;
  let high = 100_000 + 1000 * Buffer.byteLength(text);
  let started = performance.now();
  let outcome = parser.parseTree(text, { stepLimit: high });
  let ms = performance.now() - started;
  while (outcome.rejection?.reason === 'stepLimit' && high < 2e8) {
    low = high;
    high *= 4;
    started = performance.now();
    outcome = parser.parseTree(text, { stepLimit: high });
    ms = performance.now() - started;
  }
  return { ok: outcome.ok, budget: 100_000 + 1000 * Buffer.byteLength(text), within: high, ms: Math.round(ms) };
}
const lines = source.split('\n');
const starts = lines.flatMap((line, index) => (/^(?:#|fn |pub |use |mod |struct |enum |impl |trait |macro_rules|const |static |type |extern )/u.test(line) ? [index] : []));
for (let i = 0; i < starts.length; i += 1) {
  const piece = lines.slice(starts[i], starts[i + 1] ?? lines.length).join('\n');
  const result = needed(piece);
  if (result.within > result.budget || result.ms > 200) console.log(`line ${starts[i] + 1}`, JSON.stringify(result), lines[starts[i]].slice(0, 50));
}
console.log('whole', JSON.stringify(needed(source)));
