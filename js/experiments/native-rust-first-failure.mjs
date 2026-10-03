// Finds where the native Rust grammar stops on a source the oracle parses
// clean: parses the whole file without recovery and prints the rejection
// (the furthest position and the expected items), then tries each top-level
// item alone (split at lines starting a new item) to name the failing items.
// Usage: node experiments/native-rust-first-failure.mjs <file.rs>
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { oracleRecovers } from '../scripts/native-grammar-rows.mjs';

const grammar = new URL('../../parity/grammars/native/rust.lino', import.meta.url);
const parser = compileGrammar(parseGrammarLinks(readFileSync(grammar, 'utf8')));
const source = readFileSync(process.argv[2], 'utf8');
const outcome = parser.parseTree(source);
console.log('whole file ok:', outcome.ok);
if (!outcome.ok) console.log(JSON.stringify(outcome.rejection, null, 1).slice(0, 1500));
const lines = source.split('\n');
const starts = lines.flatMap((line, index) => (/^(?:#|\/\/|fn |pub |use |mod |struct |enum |impl |trait |macro_rules|const |static |type |extern )/u.test(line) && !/^\/\//u.test(line) ? [index] : []));
for (let i = 0; i < starts.length; i += 1) {
  const piece = lines.slice(starts[i], starts[i + 1] ?? lines.length).join('\n');
  if (!parser.parseTree(piece).ok && !oracleRecovers(piece, 'Rust')) {
    console.log(`--- item at line ${starts[i] + 1} fails natively, oracle clean:`);
    console.log(piece.slice(0, 600));
  }
}
