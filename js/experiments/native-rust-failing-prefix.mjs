// Finds the shortest prefix of a Rust source, cut at top-level item starts,
// that the native Rust grammar rejects while each item alone parses: the
// item that ends that prefix is where the whole-file parse goes wrong.
// Usage: node experiments/native-rust-failing-prefix.mjs <file.rs>
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const grammar = new URL('../../parity/grammars/native/rust.lino', import.meta.url);
const parser = compileGrammar(parseGrammarLinks(readFileSync(grammar, 'utf8')));
const source = readFileSync(process.argv[2], 'utf8');
const lines = source.split('\n');
const starts = lines.flatMap((line, index) => (/^(?:#|fn |pub |use |mod |struct |enum |impl |trait |macro_rules|const |static |type |extern )/u.test(line) ? [index] : []));
starts.push(lines.length);
let [low, high] = [1, starts.length - 1];
// The prefixes ending at starts[low - 1] parse; find the first that fails.
while (low < high) {
  const middle = Math.floor((low + high) / 2);
  if (parser.parseTree(lines.slice(0, starts[middle]).join('\n')).ok) low = middle + 1;
  else high = middle;
}
const ok = parser.parseTree(lines.slice(0, starts[low]).join('\n')).ok;
console.log(`first failing prefix ends at line ${starts[low]} (ok ${ok}); last item from line ${starts[low - 1] + 1}:`);
console.log(lines.slice(starts[low - 1], starts[low]).join('\n').slice(0, 3000));
