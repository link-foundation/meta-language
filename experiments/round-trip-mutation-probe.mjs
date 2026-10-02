// Probes whether adding a literal alternative to the start rule survives
// emit -> re-import for each shared interchange format.
import { readFileSync } from 'node:fs';
import * as ml from '../js/src/index.js';
import { renderGrammarRule } from '../js/tests/support/render-grammar-expression.js';

const corpus = JSON.parse(readFileSync(new URL('../parity/fixtures/grammar-importers.json', import.meta.url)));
const fns = {
  abnf: [ml.importAbnf, ml.emitAbnf], bnf: [ml.importBnf, ml.emitBnf], ebnf: [ml.importEbnf, ml.emitEbnf],
  pest: [ml.importPest, ml.emitPest], 'tree-sitter-json': [ml.importTreeSitterJson, ml.emitTreeSitterJson],
};
for (const c of corpus.cases) {
  const [imp, emit] = fns[c.format];
  const g = imp(c.source);
  for (const ordered of [false, true]) {
    const rules = new Map(g.rules);
    const start = g.startRule();
    const expr = ordered ? { kind: 'choice', items: [start.expression, { kind: 'literal', value: 'mutated' }], ordered: true }
      : { kind: 'choice', items: [start.expression, { kind: 'literal', value: 'mutated' }], ordered: false };
    rules.set(start.name, { ...start, expression: expr });
    const m = new ml.Grammar(g.start, rules, g.sourceFormat);
    try {
      const e = emit(m);
      const r = imp(e.source);
      const same = renderGrammarRule(r.rule(start.name)) === renderGrammarRule(m.rule(start.name));
      let acc; try { ml.parseWithGrammar(r, 'mutated'); acc = true; } catch { acc = false; }
      console.log(c.id, ordered ? 'ordered' : 'unordered', 'lossy', JSON.stringify(e.report.lossy), 'same', same, 'accepts', acc,
        same ? '' : renderGrammarRule(r.rule(start.name)));
    } catch (err) { console.log(c.id, ordered, 'ERR', err.message); }
  }
}
