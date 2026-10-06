// Counts the executor steps a native .lino grammar takes on each text:
//   LINO=GRAMMAR.lino node experiments/native-c-steps.mjs 'text' ...
import { readFileSync } from 'node:fs';
import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
const compiled = compileGrammar(parseGrammarLinks(readFileSync(process.env.LINO ?? '../parity/grammars/native/c.lino', 'utf8')));
for (const text of process.argv.slice(2)) {
  let low = 1_000;
  let high = 100_000_000;
  // The smallest step limit the text parses within, by bisection.
  while (low < high) {
    const mid = Math.floor((low + high) / 2);
    const out = compiled.parseTree(text, { errorRecovery: true, recovery: 'accept', stepLimit: mid });
    if (out.rejection?.reason === 'stepLimit') low = mid + 1; else high = mid;
  }
  console.log(`${low} steps  ${JSON.stringify(text)}`);
}
