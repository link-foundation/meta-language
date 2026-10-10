import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { LinkNetwork, compileGrammar, parseGrammarLinks, renderGrammarLinks, renderSyntaxTree } from '../src/index.js';
import { R_SCANNERS } from '../experiments/build-r-scanner.mjs';
import { scannerFamilies } from '../scripts/scanner-families.mjs';
import { NATIVE_GRAMMARS, buildNativeGrammarCorpusSources, buildNativeGrammarFixture, fixturePath, renderFixture } from '../scripts/generate-native-grammar-fixtures.mjs';
import { nativeRows, oracleRows, oracleRecovers } from '../scripts/native-grammar-rows.mjs';
import { parseNative } from '../src/native-grammar-parser.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const read = (file) => readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8');
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'r');
const fixture = JSON.parse(read(fixturePath(entry)));
const listing = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(listing));
const text = (tree) => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;
const observe = (assertions, testName) => recordIssue195Observations({
  requirementId: 'I195-GRAMMAR-NATIVE-R', suffix: 'behavior',
  fixtureId: 'planned:repository-directive:i195-grammar-native-r',
  fixtureFile: fixturePath(entry), assertions, testName,
});

test('native R root spans include leading trivia as the independent oracle requires', () => {
  for (const source of ['\nx <- 1\n', '  x <- 1\n', '\n\t\n']) {
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), oracleRows(source, 'R'));
    assert.equal(parseNative('native-r', source).start, 0);
  }
});

test('native R and r use the ordinary catalog parser', (context) => {
  const source = 'x <- r"(body)"\n';
  for (const language of ['R', 'r']) {
    const network = LinkNetwork.parse(source, language);
    assert.equal(network.reconstructText(), source);
    assert.ok(network.links().some((link) => link.metadata().term === 'program'));
    assert.ok(network.links().some((link) => link.metadata().term === 'string'));
  }
  observe(['nativeRCatalogDispatch'], context.name);
});

test('native R grammar and scanners are canonical generated Links Notation', (context) => {
  assert.equal(renderGrammarLinks(parseGrammarLinks(listing)), listing);
  assert.equal(scannerFamilies(R_SCANNERS), read('parity/grammars/scanners/r.lino'));
  assert.deepEqual(fixture, buildNativeGrammarFixture(entry));
  observe(['nativeRGrammarIsCanonicalLinks'], context.name);
});

test('native R focused trees match the independent oracle and preserve every byte', (context) => {
  for (const { source, rows } of fixture.matches) {
    assert.equal(oracleRecovers(source, 'R'), false, JSON.stringify(source));
    assert.deepEqual(oracleRows(source, 'R'), rows);
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true, JSON.stringify(source));
    assert.deepEqual(outcome.ambiguities, []);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, JSON.stringify(source));
    assert.equal(text(outcome.tree), source);
  }
  observe(['nativeRTreesMatchOracle', 'nativeRTreesLossless'], context.name);
});

test('native R rejects and losslessly recovers invalid focused sources', (context) => {
  for (const { source, recovered } of fixture.rejections) {
    assert.equal(oracleRecovers(source, 'R'), true, JSON.stringify(source));
    assert.equal(parser.parseTree(source).ok, false, JSON.stringify(source));
    const outcome = parser.parseTree(source, { errorRecovery: true });
    assert.equal(outcome.rejection.reason, 'recovered');
    assert.equal(renderSyntaxTree(outcome.tree), recovered);
    assert.equal(text(outcome.tree), source);
  }
  observe(['nativeRRejectsInvalidInput'], context.name);
});

// CI executes the upstream corpus; local checks select the focused tests.
test('native R matches the independent oracle on every pinned upstream corpus input', (context) => {
  const file = 'parity/fixtures/native-grammars/r-corpus.json';
  assert.equal(read(file), renderFixture(buildNativeGrammarCorpusSources('r')));
  const { cases } = JSON.parse(read(file));
  assert.ok(cases.length > 0);
  const failures = [];
  for (const { file: sourceFile, title, source } of cases) {
    const label = `${sourceFile}: ${title}`;
    try {
      const outcome = parser.parseTree(source);
      if (oracleRecovers(source, 'R')) assert.equal(outcome.ok, false, label);
      else {
        assert.equal(outcome.ok, true, label);
        assert.deepEqual(outcome.ambiguities, [], label);
        assert.deepEqual(nativeRows(outcome.tree, source, fixture), oracleRows(source, 'R'), label);
        assert.equal(text(outcome.tree), source, label);
      }
    } catch (error) { failures.push(`${label}: ${error.message}`); }
  }
  assert.deepEqual(failures, []);
  observe(['nativeRUpstreamCorpusMatchesOracle'], context.name);
});
