// Runs the mutation-guarded round trip over the shared importer corpus.
import { readFileSync } from 'node:fs';
import * as ml from '../js/src/index.js';

const corpus = JSON.parse(readFileSync(new URL('../parity/fixtures/grammar-importers.json', import.meta.url)));
const fns = {
  abnf: [ml.importAbnf, ml.emitAbnf], bnf: [ml.importBnf, ml.emitBnf], ebnf: [ml.importEbnf, ml.emitEbnf],
  pest: [ml.importPest, ml.emitPest], 'tree-sitter-json': [ml.importTreeSitterJson, ml.emitTreeSitterJson],
};
for (const c of corpus.cases) {
  const [importGrammar, emitGrammar] = fns[c.format];
  const r = ml.checkGrammarRoundTrip(c.source, { importGrammar, emitGrammar, accepts: c.accepts, rejects: c.rejects });
  console.log(c.id, r.status, JSON.stringify(r.failures));
}
