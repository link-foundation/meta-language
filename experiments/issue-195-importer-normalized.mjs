// Prints the JavaScript importers' normalized grammar for each shared corpus case.
import { readFileSync } from 'node:fs';
import { importAbnf, importBnf, importEbnf, importPest, importTreeSitterJson } from '../js/src/index.js';

const corpus = JSON.parse(readFileSync(new URL('../parity/fixtures/grammar-importers.json', import.meta.url)));
const importers = { abnf: importAbnf, bnf: importBnf, ebnf: importEbnf, pest: importPest, 'tree-sitter-json': importTreeSitterJson };
for (const fixture of corpus.cases) {
  const grammar = importers[fixture.format](fixture.source);
  console.log(`== ${fixture.format}`);
  for (const rule of grammar.normalized().rules) console.log(rule.name, rule.kind, JSON.stringify(rule.expression));
}
