import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { LinkNetwork, compileGrammar, parseGrammarLinks, renderGrammarLinks, renderSyntaxTree } from '../src/index.js';
import { NATIVE_GRAMMARS, buildNativeGrammarCorpusSources, buildNativeGrammarFixture, fixturePath, renderFixture } from '../scripts/generate-native-grammar-fixtures.mjs';
import { nativeRows, oracleRows, oracleRecovers, nativeCorpusFailure } from '../scripts/native-grammar-rows.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const read = (file) => readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8');
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'toml');
const fixture = JSON.parse(read(fixturePath(entry)));
const listing = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(listing));
const text = (tree) => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;
const observe = (assertions, testName) => recordIssue195Observations({
  requirementId: 'I195-GRAMMAR-NATIVE-TOML', suffix: 'behavior',
  fixtureId: 'planned:repository-directive:i195-grammar-native-toml',
  fixtureFile: fixturePath(entry), assertions, testName,
});

test('native TOML grammar and scanners are canonical generated Links Notation', (context) => {
  assert.equal(renderGrammarLinks(parseGrammarLinks(listing)), listing);
  assert.deepEqual(fixture, buildNativeGrammarFixture(entry));
  observe(['nativeTomlGrammarIsCanonicalLinks'], context.name);
});

test('native TOML focused trees match the independent oracle and preserve every byte', (context) => {
  for (const { source, rows } of fixture.matches) {
    assert.equal(oracleRecovers(source, 'TOML'), false, JSON.stringify(source));
    assert.deepEqual(oracleRows(source, 'TOML'), rows);
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true, JSON.stringify(source));
    assert.deepEqual(outcome.ambiguities, []);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, JSON.stringify(source));
    assert.equal(text(outcome.tree), source);
    const network = LinkNetwork.parse(source, entry.language);
    assert.equal(network.reconstructText(), source);
    assert.ok(network.parseGrammars().some(({ id }) => id === `native-${entry.id}`));
  }
  observe(['nativeTomlTreesMatchOracle', 'nativeTomlTreesLossless'], context.name);
});

test('native TOML rejects and losslessly recovers invalid focused sources', (context) => {
  for (const { source, recovered } of fixture.rejections) {
    assert.equal(oracleRecovers(source, 'TOML'), true, JSON.stringify(source));
    assert.equal(parser.parseTree(source).ok, false, JSON.stringify(source));
    const outcome = parser.parseTree(source, { errorRecovery: true });
    assert.equal(outcome.rejection.reason, 'recovered');
    assert.equal(renderSyntaxTree(outcome.tree), recovered);
    assert.equal(text(outcome.tree), source);
  }
  observe(['nativeTomlRejectsInvalidInput'], context.name);
});

// CI executes the upstream corpus; local checks select the focused tests.
test('native TOML matches the independent oracle on every pinned upstream corpus input', (context) => {
  const file = 'parity/fixtures/native-grammars/toml-corpus.json';
  assert.equal(read(file), renderFixture(buildNativeGrammarCorpusSources('toml')));
  const { cases } = JSON.parse(read(file));
  assert.ok(cases.length > 0);
  const failures = [];
  for (const { file: sourceFile, title, source } of cases) {
    const label = `${sourceFile}: ${title}`;
    try {
      const outcome = parser.parseTree(source);
      if (oracleRecovers(source, 'TOML')) assert.equal(outcome.ok, false, label);
      else {
        assert.equal(outcome.ok, true, label);
        assert.deepEqual(outcome.ambiguities, [], label);
        assert.deepEqual(nativeRows(outcome.tree, source, fixture), oracleRows(source, 'TOML'), label);
        assert.equal(text(outcome.tree), source, label);
      }
    } catch (error) { failures.push(nativeCorpusFailure(label, error)); }
  }
  assert.deepEqual(failures, []);
  observe(['nativeTomlUpstreamCorpusMatchesOracle'], context.name);
});
