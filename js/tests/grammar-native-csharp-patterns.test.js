import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { grammarSourceOf, importSource, sourceText } from '../scripts/import-native-grammars.mjs';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';
import { applySourceContextPreference } from '../scripts/native-source-preferences.mjs';

const read = file => readProjectData(new URL(`../../${file}`, import.meta.url));
const fixture = read('parity/fixtures/native-grammars/csharp-pattern-ci.json');
const naming = read('parity/naming/grammar-name-expansions.json');
const entry = grammarSourceOf('native-csharp');
const imported = importSource(entry, new Map(naming.words.map(({ word, replacement }) => [word, replacement])), naming.grammars.csharp);
const parser = compileGrammar(parseGrammarLinks(imported.text));
const projection = { ...read('parity/fixtures/native-grammars/csharp-profile.json'), oracleKinds: nativeOracleKinds(imported.text) };
const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;

test('source context preferences preserve original productions and refuse stale selectors atomically', () => {
  const original = JSON.parse(sourceText(entry));
  const untouched = structuredClone(original);
  assert.equal(applySourceContextPreference(untouched, { family: 'unknown-context' }), false);
  assert.deepEqual(untouched, original);
  const copy = fixture.sourceTransforms.find(({ family }) => family === 'rule-context-copy');
  assert.equal(applySourceContextPreference(untouched, copy), true);
  assert.deepEqual(untouched.rules[copy.helper], original.rules[copy.rule]);
  untouched.rules[copy.helper].members.pop();
  assert.deepEqual(untouched.rules[copy.rule], original.rules[copy.rule], 'context copies cannot mutate their source production');
  const beforeDuplicate = structuredClone(untouched);
  assert.throws(() => applySourceContextPreference(untouched, copy), /unique hidden helper/u);
  assert.deepEqual(untouched, beforeDuplicate);
  for (let index = 0; index < fixture.sourceTransforms.length; index += 1) {
    const decision = fixture.sourceTransforms[index];
    if (decision.family !== 'symbol-complete-context') continue;
    const grammar = transformNativeSource(original, fixture.sourceTransforms.slice(0, index));
    const before = structuredClone(grammar);
    assert.throws(() => applySourceContextPreference(grammar, { ...decision, count: decision.count + 1 }), /exactly/u);
    assert.deepEqual(grammar, before, 'stale counts cannot install helpers or rewrite selected rules');
    assert.equal(applySourceContextPreference(grammar, decision), true);
    for (const rule of decision.rules) {
      const fallback = decision.wholeRule ? before.rules[rule] : { type: 'SYMBOL', name: decision.symbol };
      assert.ok(JSON.stringify(grammar.rules[rule]).includes(JSON.stringify(fallback)), `source fallback retained for ${rule}`);
    }
  }
  assert.deepEqual(original, JSON.parse(sourceText(entry)), 'pinned source grammar bytes are unchanged');
});

test('CSharp pattern preferences reproduce all five CI failures and the frozen neighboring contexts', () => {
  for (const decision of fixture.sourceTransforms) {
    assert.ok(naming.grammars.csharp.sourceTransforms.some(other => JSON.stringify(other) === JSON.stringify(decision)), 'all preferences must come from registered source data');
  }
  assert.equal(entry.id, fixture.grammarSource);
  assert.equal(entry.revision, fixture.revision);
  assert.equal(entry.sha256, fixture.sourceSha256);
  assert.deepEqual(entry.corpus, fixture.corpus);
  assert.deepEqual(fixture.cases.map(({ id }) => id), [3, 4, 5, 6, 7]);
  assert.equal(fixture.controls.length, 19);
  const corpus = read('parity/fixtures/native-grammars/csharp-profile-corpus.json').cases;
  for (const { file, title, source } of fixture.cases) {
    assert.equal(corpus.find(item => item.file === file && item.title === title)?.source, source, title);
  }
  for (const { title, source, rows } of [...fixture.cases, ...fixture.controls]) {
    const oracle = oracleSnapshot(source, fixture.language);
    assert.equal(oracle.recovers, false, title ?? source);
    assert.deepEqual(oracle.rows, rows, title ?? source);
    const result = parser.parseTree(source);
    assert.equal(result.ok, true, title ?? source);
    assert.deepEqual(result.ambiguities, [], title ?? source);
    assert.deepEqual(nativeRows(result.tree, source, projection), rows, title ?? source);
    assert.equal(text(result.tree), source, title ?? source);
  }
});
