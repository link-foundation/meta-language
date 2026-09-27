// Imports each shared corpus grammar, emits it back into its source notation,
// re-imports the emitted text, and checks that declared rules survive.
import { readFileSync } from 'node:fs';
import { isDeepStrictEqual } from 'node:util';
import * as ml from '../js/src/index.js';

const corpus = JSON.parse(readFileSync(new URL('../parity/fixtures/grammar-importers.json', import.meta.url)));
const pairs = {
  abnf: [ml.importAbnf, ml.emitAbnf],
  bnf: [ml.importBnf, ml.emitBnf],
  ebnf: [ml.importEbnf, ml.emitEbnf],
  pest: [ml.importPest, ml.emitPest],
  'tree-sitter-json': [ml.importTreeSitterJson, ml.emitTreeSitterJson],
};
const rulesOf = (grammar) => Object.fromEntries(grammar.normalized().rules.map((r) => [r.name, r]));
let failures = 0;
for (const fixture of corpus.cases ?? corpus) {
  const [importer, emitter] = pairs[fixture.format];
  const grammar = importer(fixture.source);
  const { source, report } = emitter(grammar);
  const reimported = importer(source);
  const before = rulesOf(grammar);
  const after = rulesOf(reimported);
  const declared = fixture.rules;
  const same = declared.every((name) => isDeepStrictEqual(before[name], after[name]));
  const fixpoint = emitter(reimported).source === source;
  console.log(`== ${fixture.format}: declared-equal=${same} fixpoint=${fixpoint} lossy=${JSON.stringify(report.lossy)}`);
  console.log(source);
  if (!same || !fixpoint) {
    failures += 1;
    for (const name of declared) {
      if (!isDeepStrictEqual(before[name], after[name])) {
        console.log('  before', name, JSON.stringify(before[name]));
        console.log('  after ', name, JSON.stringify(after[name]));
      }
    }
  }
}
process.exitCode = failures ? 1 : 0;
