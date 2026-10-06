// Probes, per shared importer case, whether the same-format emitted text
// re-imports to the same declared rules and whether emission is a fixpoint.
import { readFileSync } from 'node:fs';
import * as ml from '../js/src/index.js';
import { renderGrammarRule } from '../js/tests/support/render-grammar-expression.js';

const corpus = JSON.parse(readFileSync(new URL('../parity/fixtures/grammar-importers.json', import.meta.url), 'utf8'));
const pairs = {
  abnf: [ml.importAbnf, ml.emitAbnf],
  bnf: [ml.importBnf, ml.emitBnf],
  ebnf: [ml.importEbnf, ml.emitEbnf],
  pest: [ml.importPest, ml.emitPest],
  'tree-sitter-json': [ml.importTreeSitterJson, ml.emitTreeSitterJson],
};
for (const fixture of corpus.cases) {
  const [importer, emitter] = pairs[fixture.format];
  const first = emitter(importer(fixture.source)).source;
  const reimported = importer(first);
  const diffs = fixture.rules.filter((name) =>
    renderGrammarRule(reimported.rule(name)) !== fixture.expressions[name]);
  const second = emitter(reimported).source;
  const third = emitter(importer(second)).source;
  console.log(fixture.id, {
    declaredEqual: diffs.length === 0,
    diffs: diffs.map((name) => [fixture.expressions[name], renderGrammarRule(reimported.rule(name))]),
    fixpoint: first === second,
    stabilizes: second === third,
    start: reimported.startRule()?.name,
  });
}
