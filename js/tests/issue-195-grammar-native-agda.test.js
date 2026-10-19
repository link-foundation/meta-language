import { readProjectData, resolveProjectDataPath } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { LinkNetwork, compileGrammar, parseGrammarLinks, renderGrammarLinks, renderSyntaxTree } from '../src/index.js';
import { NATIVE_GRAMMARS, buildNativeGrammarCorpusSources, buildNativeGrammarFixture, fixturePath, renderFixture } from '../scripts/generate-native-grammar-fixtures.mjs';
import { nativeRows, oracleRows, oracleRecovers, nativeCorpusFailure } from '../scripts/native-grammar-rows.mjs';
import { agdaLayoutOracleSnapshot } from '../scripts/agda-layout-oracle.mjs';
import { checkAgdaLayoutOracleSources } from '../scripts/build-agda-layout-oracle.mjs';
import { readAgdaCurrentOracleFixture, selectAgdaReleasedOracleAssertion, AGDA_HISTORICAL_FIXTURE_FILE } from '../scripts/agda-oracle-profiles.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const read = (file) => readFileSync(resolveProjectDataPath(new URL(`../../${file}`, import.meta.url)), 'utf8');
const readData = (file) => readProjectData(new URL(`../../${file}`, import.meta.url));
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'agda');
const fixture = readData(fixturePath(entry));
const listing = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(listing));
const text = (tree) => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;
const observe = (assertions, testName) => recordIssue195Observations({
  requirementId: 'I195-GRAMMAR-NATIVE-AGDA', suffix: 'behavior',
  fixtureId: 'planned:repository-directive:i195-grammar-native-agda',
  fixtureFile: AGDA_HISTORICAL_FIXTURE_FILE, assertions, testName,
});

test('native Agda grammar and scanners are canonical generated Links Notation', (context) => {
  selectAgdaReleasedOracleAssertion();
  assert.equal(renderGrammarLinks(parseGrammarLinks(listing)), listing);
  assert.deepEqual(fixture, buildNativeGrammarFixture(entry));
  checkAgdaLayoutOracleSources();
  observe(['nativeAgdaGrammarIsCanonicalLinks'], context.name);
});

test('native Agda focused trees match the independent oracle and preserve every byte', (context) => {
  selectAgdaReleasedOracleAssertion();
  for (const { source, rows } of fixture.matches) {
    assert.equal(oracleRecovers(source, 'Agda'), false, JSON.stringify(source));
    assert.deepEqual(oracleRows(source, 'Agda'), rows);
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true, JSON.stringify(source));
    assert.deepEqual(outcome.ambiguities, []);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, JSON.stringify(source));
    assert.equal(text(outcome.tree), source);
    const network = LinkNetwork.parse(source, entry.language);
    assert.equal(network.reconstructText(), source);
    assert.ok(network.parseGrammars().some(({ id }) => id === `native-${entry.id}`));
  }
  observe(['nativeAgdaTreesMatchOracle', 'nativeAgdaTreesLossless'], context.name);
});

test('native Agda rejects and losslessly recovers invalid focused sources', (context) => {
  selectAgdaReleasedOracleAssertion();
  for (const { source, recovered } of fixture.rejections) {
    assert.equal(oracleRecovers(source, 'Agda'), true, JSON.stringify(source));
    assert.equal(parser.parseTree(source).ok, false, JSON.stringify(source));
    const outcome = parser.parseTree(source, { errorRecovery: true });
    assert.equal(outcome.rejection.reason, 'recovered');
    assert.equal(renderSyntaxTree(outcome.tree), recovered);
    assert.equal(text(outcome.tree), source);
  }
  observe(['nativeAgdaRejectsInvalidInput'], context.name);
});

// CI executes the upstream corpus; local checks select the focused tests.
test('native Agda matches the independent oracle on every pinned upstream corpus input', (context) => {
  const file = 'parity/fixtures/native-grammars/agda-corpus.json';
  assert.equal(read(file), renderFixture(buildNativeGrammarCorpusSources('agda')));
  const { cases } = readData(file);
  assert.ok(cases.length > 0);
  const failures = [];
  for (const { file: sourceFile, title, source } of cases) {
    const label = `${sourceFile}: ${title}`;
    try {
      const outcome = parser.parseTree(source);
      const oracle = agdaLayoutOracleSnapshot(source);
      if (oracle.recovers) assert.equal(outcome.ok, false, label);
      else {
        assert.equal(outcome.ok, true, label);
        assert.deepEqual(outcome.ambiguities, [], label);
        assert.deepEqual(nativeRows(outcome.tree, source, fixture), oracle.rows, label);
        assert.equal(text(outcome.tree), source, label);
      }
    } catch (error) { failures.push(nativeCorpusFailure(label, error)); }
  }
  assert.deepEqual(failures, []);
  observe(['nativeAgdaUpstreamCorpusMatchesOracle'], context.name);
});

test('native Agda focused layout divergences remain explicit and lossless', (context) => {
  selectAgdaReleasedOracleAssertion();
  for (const { source, rows } of fixture.divergences) {
    assert.equal(oracleRecovers(source, 'Agda'), true);
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true, source);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows);
    assert.equal(text(outcome.tree), source);
  }
  observe(['nativeAgdaDeclaredDivergencesLossless'], context.name);
});

test('Agda complete layout oracle cleanly matches all retained current focused rows under its exact source profile', () => {
  const current=readAgdaCurrentOracleFixture();
  for(const {source,rows} of current.matches){
    const oracle=agdaLayoutOracleSnapshot(source);
    assert.equal(oracle.recovers,false,JSON.stringify(source));
    assert.deepEqual(oracle.rows,rows,JSON.stringify(source));
    const outcome=parser.parseTree(source);
    assert.equal(outcome.ok,true,JSON.stringify(source));
    assert.deepEqual(outcome.ambiguities,[]);
    assert.deepEqual(nativeRows(outcome.tree,source,fixture),rows,JSON.stringify(source));
    assert.equal(text(outcome.tree),source);
  }
  for(const {source} of current.rejections)assert.equal(agdaLayoutOracleSnapshot(source).recovers,true,JSON.stringify(source));
});
