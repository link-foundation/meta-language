import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { importSource } from '../scripts/import-native-grammars.mjs';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';

const read = path => readProjectData(new URL(`../../${path}`, import.meta.url));
const fixture = read('parity/fixtures/native-grammars/elixir-comment-block-boundary.json');

test('registered trivia continuation boundaries preserve productions and reject source drift', () => {
  const source = { extras: [{ type: 'SYMBOL', name: 'comment' }], rules: {
    _terminator: { type: 'PATTERN', value: fixture.decision.pattern },
    comment: { type: 'TOKEN', content: { type: 'STRING', value: '#' } },
  } };
  const snapshot = structuredClone(source);
  const decision = { ...fixture.decision, count: 1 };
  const result = transformNativeSource(source, [decision]);
  assert.deepEqual(source, snapshot);
  assert.deepEqual(result.rules._terminator.content, source.rules._terminator);
  assert.deepEqual(result.extras, source.extras);
  assert.equal(result.rules._terminator.immediate, true);
  for (const change of [{ count: 2 }, { count: 0 }, { triviaRules: ['missing'] }, { triviaRules: ['comment', 'comment'] }, { continuationLiteral: '' }]) {
    assert.throws(() => transformNativeSource(source, [{ ...decision, ...change }]), TypeError);
  }
});

test('Elixir original CI newline-before-do case retains its comment inside the call', () => {
  const naming = read('parity/naming/grammar-name-expansions.json');
  const entry = read('parity/grammars/sources.json').sources.find(source => source.id === fixture.source);
  assert.ok(entry);
  const profile = structuredClone(naming.grammars.elixir);
  profile.sourceTransforms = profile.sourceTransforms.filter(decision => decision.family !== fixture.decision.family);
  profile.sourceTransforms.push(fixture.decision);
  const listing = importSource(entry, new Map(naming.words.map(({ word, replacement }) => [word, replacement])), profile).text;
  const parser = compileGrammar(parseGrammarLinks(listing));
  const projection = { ...read('parity/fixtures/native-grammars/elixir-profile.json'), oracleKinds: nativeOracleKinds(listing) };
  const upstream = read('parity/fixtures/native-grammars/elixir-profile-corpus.json').cases.find(test => test.file === fixture.corpus.file && test.title === fixture.corpus.title);
  assert.ok(upstream, 'the complete original failing CI case remains independently pinned');
  const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;
  for (const source of [upstream.source, ...fixture.matches, 'fun x\n# comment\ndog\n', 'fun x\n# comment\ndo?\n']) {
    const oracle = oracleSnapshot(source, fixture.language);
    assert.equal(oracle.recovers, false, source);
    const native = parser.parseTree(source);
    assert.equal(native.ok, true, source);
    assert.deepEqual(native.ambiguities, [], source);
    assert.deepEqual(nativeRows(native.tree, source, projection), oracle.rows, source);
    assert.equal(text(native.tree), source);
  }
});
