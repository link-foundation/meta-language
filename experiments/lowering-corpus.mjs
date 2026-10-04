// Lowers every grammar the importer fixture records (each case's source,
// imported in its own format) into every notation and checks the package.
// Prints the status per case and target; VERBOSE=1 logs the failures.
import { readFileSync } from 'node:fs';
import { GRAMMAR_LOWERING_FORMATS, checkGrammarLowering, grammarImporter } from '../js/src/index.js';

const fixture = JSON.parse(readFileSync(new URL('../parity/fixtures/grammar-importers.json', import.meta.url), 'utf8'));
const table = {};
let broken = 0;
for (const entry of [...fixture.cases, ...fixture.reverse]) {
  const grammar = grammarImporter(entry.format)(entry.source);
  for (const format of GRAMMAR_LOWERING_FORMATS) {
    let verdict;
    try {
      const result = checkGrammarLowering(grammar, format, { accepts: entry.accepts ?? [], rejects: entry.rejects ?? [] });
      verdict = result.status;
      if (result.status === 'broken') {
        broken += 1;
        if (process.env.VERBOSE) console.error(entry.id, format, result.failures, result.lowering.executable, result.lowering.metadata);
      }
    } catch (error) {
      broken += 1;
      verdict = `ERR ${error.message.slice(0, 40)}`;
      if (process.env.VERBOSE) console.error(entry.id, format, error);
    }
    (table[entry.id] ??= {})[format] = verdict;
  }
}
console.table(table);
console.log(`broken: ${broken}`);
