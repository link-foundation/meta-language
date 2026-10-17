import { readProjectData, resolveProjectDataPath } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { rubyLiteralCIRegressionFixture } from '../scripts/generate-ruby-literal-ci-regressions.mjs';

const read = file => readFileSync(resolveProjectDataPath(new URL(`../../${file}`, import.meta.url)), 'utf8');
const readData = file => readProjectData(new URL(`../../${file}`, import.meta.url));
const parser = compileGrammar(parseGrammarLinks(read('parity/grammars/native/ruby.lino')));
const fixture = readData('parity/fixtures/native-grammars/ruby-profile.json');
const regression = readData('parity/fixtures/native-grammars/ruby-literal-ci-regressions.json');
const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;

// These exact pinned inputs failed CI run 38227158183. Keep their entire
// context: individual numeric terms and quoted heredocs already passed while
// the original multi-statement inputs exhausted the budget or failed later.
test('Ruby literal CI regression fixture retains exact pinned cases and independent oracle rows', () => {
  assert.deepEqual(regression, rubyLiteralCIRegressionFixture());
});

for (const entry of regression.cases) {
  test(`Ruby literal CI regression: ${entry.name}`, () => {
    const oracle = oracleSnapshot(entry.source, regression.language);
    assert.equal(oracle.recovers, entry.recovers);
    assert.deepEqual(oracle.rows, entry.rows);
    const native = parser.parseTree(entry.source);
    if (entry.recovers) {
      assert.equal(native.ok, false);
      const recovered = parser.parseTree(entry.source, { errorRecovery: true });
      assert.equal(recovered.rejection.reason, 'recovered');
      assert.equal(text(recovered.tree), entry.source);
      return;
    }
    assert.equal(native.ok, true, JSON.stringify(native.rejection));
    assert.deepEqual(native.ambiguities, []);
    assert.deepEqual(nativeRows(native.tree, entry.source, fixture), oracle.rows);
    assert.equal(text(native.tree), entry.source);
  });
}
