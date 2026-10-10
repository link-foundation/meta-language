import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { LinkNetwork, LinkType, compileGrammar, parseGrammarLinks, renderGrammarLinks, renderSyntaxTree } from '../src/index.js';
import { NATIVE_GRAMMARS, buildNativeGrammarCorpusSources, buildNativeGrammarFixture, fixturePath, renderFixture } from '../scripts/generate-native-grammar-fixtures.mjs';
import { nativeRows, oracleRows, oracleRecovers, nativeCorpusFailure } from '../scripts/native-grammar-rows.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const read = (file) => readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8');
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'html');
const fixture = JSON.parse(read(fixturePath(entry)));
const listing = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(listing));
const text = (tree) => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;
const observe = (assertions, testName) => recordIssue195Observations({
  requirementId: 'I195-GRAMMAR-NATIVE-HTML', suffix: 'behavior',
  fixtureId: 'planned:repository-directive:i195-grammar-native-html',
  fixtureFile: fixturePath(entry), assertions, testName,
});

test('native HTML grammar and scanners are canonical generated Links Notation', (context) => {
  assert.equal(renderGrammarLinks(parseGrammarLinks(listing)), listing);
  assert.deepEqual(fixture, buildNativeGrammarFixture(entry));
  observe(['nativeHtmlGrammarIsCanonicalLinks'], context.name);
});

test('native HTML focused trees match the independent oracle and preserve every byte', (context) => {
  for (const { source, rows } of fixture.matches) {
    assert.equal(oracleRecovers(source, 'HTML'), false, JSON.stringify(source));
    assert.deepEqual(oracleRows(source, 'HTML'), rows);
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true, JSON.stringify(source));
    assert.deepEqual(outcome.ambiguities, []);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, JSON.stringify(source));
    assert.equal(text(outcome.tree), source);
    const network = LinkNetwork.parse(source, entry.language);
    assert.equal(network.reconstructText(), source);
    assert.ok(network.parseGrammars().some(({ id }) => id === `native-${entry.id}`));
  }
  observe(['nativeHtmlTreesMatchOracle', 'nativeHtmlTreesLossless'], context.name);
});

test('native HTML rejects and losslessly recovers invalid focused sources', (context) => {
  for (const { source, recovered } of fixture.rejections) {
    assert.equal(oracleRecovers(source, 'HTML'), true, JSON.stringify(source));
    assert.equal(parser.parseTree(source).ok, false, JSON.stringify(source));
    const outcome = parser.parseTree(source, { errorRecovery: true });
    assert.equal(outcome.rejection.reason, 'recovered');
    assert.equal(renderSyntaxTree(outcome.tree), recovered);
    assert.equal(text(outcome.tree), source);
  }
  observe(['nativeHtmlRejectsInvalidInput'], context.name);
});

// CI executes the upstream corpus; local checks select the focused tests.
test('native HTML matches the independent oracle on every pinned upstream corpus input', (context) => {
  const file = 'parity/fixtures/native-grammars/html-corpus.json';
  assert.equal(read(file), renderFixture(buildNativeGrammarCorpusSources('html')));
  const { cases } = JSON.parse(read(file));
  assert.ok(cases.length > 0);
  const failures = [];
  for (const { file: sourceFile, title, source } of cases) {
    const label = `${sourceFile}: ${title}`;
    try {
      const outcome = parser.parseTree(source);
      if (oracleRecovers(source, 'HTML')) assert.equal(outcome.ok, false, label);
      else {
        assert.equal(outcome.ok, true, label);
        assert.deepEqual(outcome.ambiguities, [], label);
        assert.deepEqual(nativeRows(outcome.tree, source, fixture), oracleRows(source, 'HTML'), label);
        assert.equal(text(outcome.tree), source, label);
      }
    } catch (error) { failures.push(nativeCorpusFailure(label, error)); }
  }
  assert.deepEqual(failures, []);
  observe(['nativeHtmlUpstreamCorpusMatchesOracle'], context.name);
});

test('native HTML focused embedded raw text preserves leading whitespace', () => {
  for (const [tag, language, body] of [['script', 'JavaScript', '\n  let leaf=7;\n'], ['style', 'CSS', '\n  p {color:green}\n']]) {
    const source = `<${tag}>${body}</${tag}>`;
    const network = LinkNetwork.parse(source, 'HTML');
    assert.equal(network.reconstructText(), source);
    const embedded = network.links().filter(link => link.metadata().linkType === LinkType.SourceToken && link.metadata().language === language);
    assert.equal(embedded.map(link => link.metadata().term).join(''), body, language);
  }
});

test('native HTML focused default CST retains independent embedded boundaries', () => {
  const inventory = JSON.parse(read('parity/language-grammar-inventory.json'));
  const source = inventory.languages.find(entry => entry.name === 'HTML').source;
  const oracle = JSON.parse(read('parity/fixtures/default-cst-expected.json')).languages.HTML;
  const expected = JSON.parse(read('parity/fixtures/native-default-cst-expected.json')).languages.HTML;
  assert.deepEqual(expected.embedded, oracle.embedded);
  const network = LinkNetwork.parse(source, 'HTML');
  const regions = network.links().filter(link => link.metadata().linkType === LinkType.Region && link.metadata().language !== 'HTML');
  assert.deepEqual(regions.map(region => [region.metadata().language, region.metadata().span.byteRange.start, region.metadata().span.byteRange.end]), expected.embedded.map(region => [region.language, region.startByte, region.endByte]));
});
