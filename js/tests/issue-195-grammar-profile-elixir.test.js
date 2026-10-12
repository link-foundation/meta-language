import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { compileGrammar, parseGrammarLinks, renderGrammarLinks } from '../src/index.js';
import { nativeRows, oracleSnapshot, nativeCorpusFailure } from '../scripts/native-grammar-rows.mjs';
import { nativeScannerProfileFixture, nativeScannerProfileCorpus } from '../scripts/generate-native-scanner-profile-fixtures.mjs';

const read = file => readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8');
const grammar = read('parity/grammars/native/elixir.lino');
const fixture = JSON.parse(read('parity/fixtures/native-grammars/elixir-profile.json'));
const parser = compileGrammar(parseGrammarLinks(grammar));
const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;

test('Elixir scanner profile focused fixtures are canonical and independently generated', () => {
  assert.equal(renderGrammarLinks(parseGrammarLinks(grammar)), grammar);
  assert.deepEqual(fixture, nativeScannerProfileFixture('elixir'));
});

test('Elixir scanner profile focused trees exactly match the independent oracle', () => {
  for (const { source, rows } of fixture.matches) {
    const oracle = oracleSnapshot(source, fixture.language);
    assert.equal(oracle.recovers, false, source);
    assert.deepEqual(oracle.rows, rows, source);
    const native = parser.parseTree(source);
    assert.equal(native.ok, true, source);
    assert.deepEqual(native.ambiguities, [], source);
    assert.deepEqual(nativeRows(native.tree, source, fixture), rows, source);
    assert.equal(text(native.tree), source);
  }
});

test('Elixir scanner profile focused invalid inputs reject and recover losslessly', () => {
  for (const { source } of fixture.rejections) {
    assert.equal(oracleSnapshot(source, fixture.language).recovers, true, source);
    assert.equal(parser.parseTree(source).ok, false, source);
    const native = parser.parseTree(source, { errorRecovery: true });
    assert.equal(native.rejection.reason, 'recovered', source);
    assert.equal(text(native.tree), source);
  }
});

// CI executes every pinned input. Local checks select the focused tests.
test('Elixir scanner profile complete pinned upstream corpus matches its oracle', () => {
  const corpus = JSON.parse(read('parity/fixtures/native-grammars/elixir-profile-corpus.json'));
  assert.deepEqual(corpus, nativeScannerProfileCorpus('elixir'));
  assert.ok(corpus.cases.length > 0);
  const failures = [];
  for (const { file, title, source } of corpus.cases) {
    const label = `${file}: ${title}`;
    try {
      const native = parser.parseTree(source);
      const oracle = oracleSnapshot(source, fixture.language);
      if (oracle.recovers) assert.equal(native.ok, false, label);
      else {
        assert.equal(native.ok, true, label);
        assert.deepEqual(native.ambiguities, [], label);
        assert.deepEqual(nativeRows(native.tree, source, fixture), oracle.rows, label);
        assert.equal(text(native.tree), source, label);
      }
    } catch (error) { failures.push(nativeCorpusFailure(label, error)); }
  }
  assert.deepEqual(failures, []);
});
