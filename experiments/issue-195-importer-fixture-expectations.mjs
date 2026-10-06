// Prints the rendered declared rules and the same-format emitted text for each
// shared importer corpus case, used to author the fixture expectations.
import { readFileSync } from 'node:fs';
import * as ml from '../js/src/index.js';
import { renderGrammarRule } from '../js/tests/support/render-grammar-expression.js';

const corpus = JSON.parse(readFileSync(new URL('../parity/fixtures/grammar-importers.json', import.meta.url)));
const pairs = {
  abnf: [ml.importAbnf, ml.emitAbnf],
  bnf: [ml.importBnf, ml.emitBnf],
  ebnf: [ml.importEbnf, ml.emitEbnf],
  pest: [ml.importPest, ml.emitPest],
  'tree-sitter-json': [ml.importTreeSitterJson, ml.emitTreeSitterJson],
};
const output = {};
for (const fixture of corpus.cases) {
  const [importer, emitter] = pairs[fixture.format];
  const grammar = importer(fixture.source);
  output[fixture.format] = {
    expressions: Object.fromEntries(fixture.rules.map((name) => [name, renderGrammarRule(grammar.rule(name))])),
    emitted: emitter(grammar),
  };
}
console.log(JSON.stringify(output, null, 2));
