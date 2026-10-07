import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { compileGrammar, parseGrammarLinks, renderGrammarLinks, renderSyntaxTree } from '../src/index.js';
import { LUA_SCANNERS } from '../experiments/build-lua-scanner.mjs';
import { scannerFamilies } from '../scripts/scanner-families.mjs';
import { NATIVE_GRAMMARS, buildNativeGrammarCorpusSources, buildNativeGrammarFixture, fixturePath, renderFixture } from '../scripts/generate-native-grammar-fixtures.mjs';
import { nativeRows, oracleRows, oracleRecovers } from '../scripts/native-grammar-rows.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const read = (file) => readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8');
const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'lua');
const fixture = JSON.parse(read(fixturePath(entry)));
const listing = read(entry.grammar);
const parser = compileGrammar(parseGrammarLinks(listing));
const text = (tree) => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;
const observe = (assertions, testName) => recordIssue195Observations({
  requirementId: 'I195-GRAMMAR-NATIVE-LUA', suffix: 'behavior',
  fixtureId: 'planned:repository-directive:i195-grammar-native-lua',
  fixtureFile: fixturePath(entry), assertions, testName,
});

test('native Lua grammar and scanners are canonical generated Links Notation', (context) => {
  assert.equal(renderGrammarLinks(parseGrammarLinks(listing)), listing);
  assert.equal(scannerFamilies(LUA_SCANNERS), read('parity/grammars/scanners/lua.lino'));
  assert.deepEqual(fixture, buildNativeGrammarFixture(entry));
  observe(['nativeLuaGrammarIsCanonicalLinks'], context.name);
});

test('native Lua focused trees match the independent oracle and preserve every byte', (context) => {
  for (const { source, rows } of fixture.matches) {
    assert.equal(oracleRecovers(source, 'Lua'), false, JSON.stringify(source));
    assert.deepEqual(oracleRows(source, 'Lua'), rows);
    const outcome = parser.parseTree(source);
    assert.equal(outcome.ok, true, JSON.stringify(source));
    assert.deepEqual(outcome.ambiguities, []);
    assert.deepEqual(nativeRows(outcome.tree, source, fixture), rows, JSON.stringify(source));
    assert.equal(text(outcome.tree), source);
  }
  observe(['nativeLuaTreesMatchOracle', 'nativeLuaTreesLossless'], context.name);
});

test('native Lua rejects and losslessly recovers invalid focused sources', (context) => {
  for (const { source, recovered } of fixture.rejections) {
    assert.equal(oracleRecovers(source, 'Lua'), true, JSON.stringify(source));
    assert.equal(parser.parseTree(source).ok, false, JSON.stringify(source));
    const outcome = parser.parseTree(source, { errorRecovery: true });
    assert.equal(outcome.rejection.reason, 'recovered');
    assert.equal(renderSyntaxTree(outcome.tree), recovered);
    assert.equal(text(outcome.tree), source);
  }
  observe(['nativeLuaRejectsInvalidInput'], context.name);
});

// CI executes the upstream corpus; local checks select the focused tests.
test('native Lua matches the independent oracle on every pinned upstream corpus input', (context) => {
  const file = 'parity/fixtures/native-grammars/lua-corpus.json';
  assert.equal(read(file), renderFixture(buildNativeGrammarCorpusSources('lua')));
  const { cases } = JSON.parse(read(file));
  assert.ok(cases.length > 0);
  for (const { file: sourceFile, title, source } of cases) {
    const label = `${sourceFile}: ${title}`;
    const outcome = parser.parseTree(source);
    if (oracleRecovers(source, 'Lua')) assert.equal(outcome.ok, false, label);
    else {
      assert.equal(outcome.ok, true, label);
      assert.deepEqual(outcome.ambiguities, [], label);
      assert.deepEqual(nativeRows(outcome.tree, source, fixture), oracleRows(source, 'Lua'), label);
      assert.equal(text(outcome.tree), source, label);
    }
  }
  observe(['nativeLuaUpstreamCorpusMatchesOracle'], context.name);
});
