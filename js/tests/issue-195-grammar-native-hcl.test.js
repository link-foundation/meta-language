import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { LinkNetwork, compileGrammar, parseGrammarLinks, renderGrammarLinks, renderSyntaxTree } from '../src/index.js';
import { HCL_SCANNERS } from '../experiments/build-hcl-scanner.mjs';
import { scannerFamilies } from '../scripts/scanner-families.mjs';
import { NATIVE_GRAMMARS, buildNativeGrammarCorpusSources, buildNativeGrammarFixture, fixturePath, renderFixture } from '../scripts/generate-native-grammar-fixtures.mjs';
import { nativeRows, oracleRows, oracleRecovers } from '../scripts/native-grammar-rows.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const read = (file) => readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8');
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'hcl');
const fixture = JSON.parse(read(fixturePath(entry)));
const listing = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(listing));
const text = (tree) => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;
const observe = (assertions, testName) => recordIssue195Observations({
  requirementId: 'I195-GRAMMAR-NATIVE-HCL', suffix: 'behavior',
  fixtureId: 'planned:repository-directive:i195-grammar-native-hcl',
  fixtureFile: fixturePath(entry), assertions, testName,
});

test('native HCL and Terraform use the ordinary catalog parser', (context) => {
  const source = 'a=<<END\nbody\nEND\n';
  for (const language of ['HCL', 'terraform']) {
    const network = LinkNetwork.parse(source, language);
    assert.equal(network.reconstructText(), source);
    assert.ok(network.links().some((link) => link.metadata().term === 'config_file'));
    assert.ok(network.links().some((link) => link.metadata().term === 'heredoc_template'));
  }
  observe(['nativeHclCatalogDispatch'], context.name);
});

test('native HCL grammar and scanners are canonical generated Links Notation', (context) => {
  assert.equal(renderGrammarLinks(parseGrammarLinks(listing)), listing);
  assert.equal(scannerFamilies(HCL_SCANNERS), read('parity/grammars/scanners/hcl.lino'));
  assert.deepEqual(fixture, buildNativeGrammarFixture(entry));
  observe(['nativeHclGrammarIsCanonicalLinks'], context.name);
});

test('native HCL focused trees match the independent oracle and preserve every byte', (context) => {
  for (const { source, rows } of fixture.matches) {
    assert.equal(oracleRecovers(source, 'HCL'), false, JSON.stringify(source));
    assert.deepEqual(oracleRows(source, 'HCL'), rows);
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true, JSON.stringify(source));
    assert.deepEqual(outcome.ambiguities, []);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, JSON.stringify(source));
    assert.equal(text(outcome.tree), source);
  }
  observe(['nativeHclTreesMatchOracle', 'nativeHclTreesLossless'], context.name);
});

test('native HCL rejects and losslessly recovers invalid focused sources', (context) => {
  for (const { source, recovered } of fixture.rejections) {
    assert.equal(oracleRecovers(source, 'HCL'), true, JSON.stringify(source));
    assert.equal(parser.parseTree(source).ok, false, JSON.stringify(source));
    const outcome = parser.parseTree(source, { errorRecovery: true });
    assert.equal(outcome.rejection.reason, 'recovered');
    assert.equal(renderSyntaxTree(outcome.tree), recovered);
    assert.equal(text(outcome.tree), source);
  }
  observe(['nativeHclRejectsInvalidInput'], context.name);
});

// CI executes the upstream corpus; local checks select the focused tests.
test('native HCL matches the independent oracle on every pinned upstream corpus input', (context) => {
  const file = 'parity/fixtures/native-grammars/hcl-corpus.json';
  assert.equal(read(file), renderFixture(buildNativeGrammarCorpusSources('hcl')));
  const { cases } = JSON.parse(read(file));
  assert.ok(cases.length > 0);
  const failures = [];
  for (const { file: sourceFile, title, source } of cases) {
    const label = `${sourceFile}: ${title}`;
    try {
      const outcome = parser.parseTree(source);
      if (oracleRecovers(source, 'HCL')) assert.equal(outcome.ok, false, label);
      else {
        assert.equal(outcome.ok, true, label);
        assert.deepEqual(outcome.ambiguities, [], label);
        assert.deepEqual(nativeRows(outcome.tree, source, fixture), oracleRows(source, 'HCL'), label);
        assert.equal(text(outcome.tree), source, label);
      }
    } catch (error) { failures.push(`${label}: ${error.message}`); }
  }
  assert.deepEqual(failures, []);
  observe(['nativeHclUpstreamCorpusMatchesOracle'], context.name);
});
