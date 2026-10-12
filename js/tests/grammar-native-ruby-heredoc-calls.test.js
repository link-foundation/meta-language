import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { grammarSourceOf, importSource, sourceText } from '../scripts/import-native-grammars.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';

const read = file => JSON.parse(readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8'));
const fixture = read('parity/fixtures/native-grammars/ruby-heredoc-call-context-ci.json');
const naming = read('parity/naming/grammar-name-expansions.json');
const entry = grammarSourceOf('native-ruby');
const imported = importSource(entry, new Map(naming.words.map(({ word, replacement }) => [word, replacement])), naming.grammars.ruby);
const parser = compileGrammar(parseGrammarLinks(imported.text));
const projection = read('parity/fixtures/native-grammars/ruby-profile.json');
const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;

test('ordinary call argument preference retains every pinned command and call production', () => {
  const original = JSON.parse(sourceText(entry));
  const transformed = transformNativeSource(original, fixture.sourceTransforms);
  for (const name of ['argument_list', 'command_argument_list', 'command_call', 'command_call_with_block', '_call']) {
    assert.deepEqual(transformed.rules[name], original.rules[name], `${name} retains all source alternatives`);
  }
  const unwrap = node => Array.isArray(node) ? node.map(unwrap)
    : node && typeof node === 'object' ? node.type === 'PREC_DYNAMIC' ? unwrap(node.content)
      : Object.fromEntries(Object.entries(node).map(([key, value]) => [key, unwrap(value)])) : node;
  assert.deepEqual(unwrap(transformed.rules.call), original.rules.call, 'the preference introduces no new or removed production');
  assert.throws(() => transformNativeSource(original, [{ ...fixture.sourceTransforms[0], symbol: 'absent_argument_list' }]), TypeError);
  assert.deepEqual(original, JSON.parse(sourceText(entry)), 'pinned source bytes remain untouched');
});

test('Ruby heredoc call continuations reproduce all four ambiguity failures with exact independent trees', () => {
  for (const decision of fixture.sourceTransforms) {
    assert.ok(naming.grammars.ruby.sourceTransforms.some(other => JSON.stringify(other) === JSON.stringify(decision)), 'preferences must be registered source data');
  }
  assert.equal(entry.id, fixture.grammarSource);
  assert.equal(entry.revision, fixture.revision);
  assert.equal(entry.sha256, fixture.sourceSha256);
  assert.deepEqual(entry.corpus, fixture.corpus);
  assert.equal(fixture.cases.length, 4);
  assert.equal(fixture.controls.length, 11);
  const corpus = read('parity/fixtures/native-grammars/ruby-profile-corpus.json').cases;
  for (const { file, title, source } of fixture.cases) {
    assert.equal(corpus.find(item => item.file === file && item.title === title)?.source, source, title);
  }
  for (const { title, source, rows } of [...fixture.cases, ...fixture.controls]) {
    const oracle = oracleSnapshot(source, fixture.language);
    assert.equal(oracle.recovers, false, title ?? source);
    assert.deepEqual(oracle.rows, rows, title ?? source);
    const actual = parser.parseTree(source);
    assert.equal(actual.ok, true, title ?? source);
    assert.deepEqual(actual.ambiguities, [], title ?? source);
    assert.deepEqual(nativeRows(actual.tree, source, projection), rows, title ?? source);
    assert.equal(text(actual.tree), source, title ?? source);
  }
});
