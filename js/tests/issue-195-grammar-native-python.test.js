import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { pythonScanner, pythonScannerDescriptors } from '../experiments/build-python-scanner.mjs';
import { scannerFamilies } from '../scripts/scanner-families.mjs';
import { LinkNetwork, compileGrammar, parseGrammarLinks, renderGrammarLinks, renderSyntaxTree } from '../src/index.js';
import { NATIVE_GRAMMARS, buildNativeGrammarCorpusSources, buildNativeGrammarFixture, fixturePath, renderFixture } from '../scripts/generate-native-grammar-fixtures.mjs';
import { nativeRows, oracleRows, oracleRecovers, nativeCorpusFailure } from '../scripts/native-grammar-rows.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const read = (file) => readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8');
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'python');
const fixture = JSON.parse(read(fixturePath(entry)));
const listing = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(listing));
const text = (tree) => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;
const observe = (assertions, testName) => recordIssue195Observations({
  requirementId: 'I195-GRAMMAR-NATIVE-PYTHON', suffix: 'behavior',
  fixtureId: 'planned:repository-directive:i195-grammar-native-python',
  fixtureFile: fixturePath(entry), assertions, testName,
});

test('native Python grammar and scanners are canonical generated Links Notation', (context) => {
  const manifest = read('rust/Cargo.toml');
  const production = manifest.split('[dependencies]')[1].split('[dev-dependencies]')[0];
  const development = manifest.split('[dev-dependencies]')[1].split('[build-dependencies]')[0];
  assert.doesNotMatch(production, /^tree-sitter-python\s*=/mu);
  assert.match(development, /^tree-sitter-python\s*=\s*"=0\.25\.0"/mu);
  assert.equal(renderGrammarLinks(parseGrammarLinks(listing)), listing);
  assert.equal(scannerFamilies(pythonScannerDescriptors), pythonScanner);
  assert.equal(read('parity/grammars/scanners/python.lino'), pythonScanner);
  assert.deepEqual(fixture, buildNativeGrammarFixture(entry));
  observe(['nativePythonGrammarIsCanonicalLinks'], context.name);
});

test('native Python focused trees match the independent oracle and preserve every byte', (context) => {
  for (const { source, rows } of fixture.matches) {
    assert.equal(oracleRecovers(source, 'Python'), false, JSON.stringify(source));
    assert.deepEqual(oracleRows(source, 'Python'), rows);
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true, JSON.stringify(source));
    assert.deepEqual(outcome.ambiguities, []);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, JSON.stringify(source));
    assert.equal(text(outcome.tree), source);
    const network = LinkNetwork.parse(source, entry.language);
    assert.equal(network.reconstructText(), source);
    assert.ok(network.parseGrammars().some(({ id }) => id === `native-${entry.id}`));
  }
  observe(['nativePythonTreesMatchOracle', 'nativePythonTreesLossless'], context.name);
});

test('native Python rejects and losslessly recovers invalid focused sources', (context) => {
  for (const { source, recovered } of fixture.rejections) {
    assert.equal(oracleRecovers(source, 'Python'), true, JSON.stringify(source));
    assert.equal(parser.parseTree(source).ok, false, JSON.stringify(source));
    const outcome = parser.parseTree(source, { errorRecovery: true });
    assert.equal(outcome.rejection.reason, 'recovered');
    assert.equal(renderSyntaxTree(outcome.tree), recovered);
    assert.equal(text(outcome.tree), source);
  }
  observe(['nativePythonRejectsInvalidInput'], context.name);
});

// CI executes the upstream corpus; local checks select the focused tests.
test('native Python matches the independent oracle on every pinned upstream corpus input', (context) => {
  const file = 'parity/fixtures/native-grammars/python-corpus.json';
  assert.equal(read(file), renderFixture(buildNativeGrammarCorpusSources('python')));
  const { cases } = JSON.parse(read(file));
  assert.ok(cases.length > 0);
  const failures = [];
  for (const { file: sourceFile, title, source } of cases) {
    const label = `${sourceFile}: ${title}`;
    try {
      const outcome = parser.parseTree(source);
      if (oracleRecovers(source, 'Python')) assert.equal(outcome.ok, false, label);
      else {
        assert.equal(outcome.ok, true, label);
        assert.deepEqual(outcome.ambiguities, [], label);
        assert.deepEqual(nativeRows(outcome.tree, source, fixture), oracleRows(source, 'Python'), label);
        assert.equal(text(outcome.tree), source, label);
      }
    } catch (error) { failures.push(nativeCorpusFailure(label, error)); }
  }
  assert.deepEqual(failures, []);
  observe(['nativePythonUpstreamCorpusMatchesOracle'], context.name);
});

test('prefixed quote descriptors reject conflicting flags and delimiters', () => {
  const descriptor = pythonScannerDescriptors[1];
  for (const change of [
    { endToken: descriptor.contentToken }, { startToken: ') fail (' },
    { quotes: [] }, { quotes: [{ text: 'ab', triple: true }] },
    { quotes: [{ text: '"', triple: 1 }] },
    { rawPrefixes: ['r'], bytesPrefixes: ['r'] },
    { interpolationCharacters: ['ab'] }, { interpolationVariable: ') fail (' },
  ]) assert.throws(() => scannerFamilies([{ ...descriptor, ...change }]), TypeError);
});
