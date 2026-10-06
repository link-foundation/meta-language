// Bisects the step budget one repair round of the native Rust grammar needs for a source,
// given as a JSON string body or with -f FILE, against the default budget, to tell a
// recovery that ran out of steps (a whole-input ERROR) from a recovery choice:
//   node experiments/native-recovery-step-budget.mjs 'let x;\n \npub fn a() {}\n'
//   node experiments/native-recovery-step-budget.mjs -f input.rs
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { nativeGrammarText } from '../src/native-grammar-parser.js';

const parser = compileGrammar(parseGrammarLinks(nativeGrammarText('native-rust')));
const [flag, value] = process.argv.slice(2);
const source = flag === '-f' ? readFileSync(value, 'utf8') : JSON.parse(`"${flag}"`);
const completes = (stepLimit) => parser.parseTree(source, { errorRecovery: true, recovery: 'accept', stepLimit }).tree !== null;
let [low, high] = [1000, 50_000_000];
while (high - low > 1000) {
  const middle = Math.floor((low + high) / 2);
  if (completes(middle)) high = middle;
  else low = middle;
}
const bytes = Buffer.byteLength(source);
console.log(`${bytes} bytes: a round needs about ${high} steps; the default budget is ${100_000 + 1000 * bytes}`);
