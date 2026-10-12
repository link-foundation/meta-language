import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { grammarSourceOf, importSource, sourceText } from '../scripts/import-native-grammars.mjs';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';

const read = file => JSON.parse(readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8'));
const fixture = read('parity/fixtures/native-grammars/ruby-expression-contexts.json');
const naming = read('parity/naming/grammar-name-expansions.json');
const entry = grammarSourceOf('native-ruby');
const imported = importSource(entry, new Map(naming.words.map(({ word, replacement }) => [word, replacement])), naming.grammars.ruby);
const parser = compileGrammar(parseGrammarLinks(imported.text));
const projection = { ...read('parity/fixtures/native-grammars/ruby-profile.json'), oracleKinds: nativeOracleKinds(imported.text) };
const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;

test('recorded Ruby expression decisions preserve receiver and declaration source productions', () => {
  const original = JSON.parse(sourceText(entry));
  const transformed = transformNativeSource(original, fixture.decisions);
  assert.deepEqual(transformed.rules._call, original.rules._call, 'receiver method spellings remain valid');
  assert.deepEqual(transformed.rules._chained_command_call, original.rules._chained_command_call, 'chained receiver methods remain valid');
  assert.deepEqual(transformed.rules._method_name, original.rules._method_name, 'method declarations retain all original names');
  assert.deepEqual(transformed.rules._function_identifier, original.rules._function_identifier, 'the underlying identifier productions are retained');
  const count = node => Array.isArray(node) ? node.reduce((total, child) => total + count(child), 0)
    : node && typeof node === 'object' ? Number(node.type === 'NATIVE_PREFIX_EXCLUSION') + Object.values(node).reduce((total, child) => total + count(child), 0) : 0;
  for (const decision of fixture.decisions.filter(({ family }) => family === 'symbol-pattern-exclusion')) {
    assert.equal(count(transformed.rules[decision.rule]), decision.count);
    assert.throws(() => transformNativeSource(original, [{ ...decision, count: decision.count + 1 }]), /exactly/u);
  }
  assert.equal(transformed.rules.command_binary.type, 'PREC_LEFT');
  assert.equal(transformed.rules.command_binary.value, -1);
  assert.deepEqual(transformed.rules.command_binary.content, original.rules.command_binary.content);
  assert.deepEqual(original, JSON.parse(sourceText(entry)));
});

test('Ruby operator and call contexts match the pinned independent oracle without larger budgets', () => {
  for (const decision of fixture.decisions) {
    assert.deepEqual(naming.grammars.ruby.sourceTransforms.find(other => other.family === decision.family && other.rule === decision.rule
      && other.pattern === decision.pattern), decision, `missing registered source decision for ${decision.rule}`);
  }
  assert.equal(entry.id, fixture.source);
  assert.equal(entry.revision, fixture.revision);
  assert.equal(entry.sha256, fixture.sourceSha256);
  assert.equal(entry.corpus.sha256, fixture.corpusSha256);
  assert.equal(fixture.matches.length, 16);
  for (const { source, rows } of fixture.matches) {
    const oracle = oracleSnapshot(source, fixture.language);
    assert.equal(oracle.recovers, false, source);
    assert.deepEqual(oracle.rows, rows, source);
    const actual = parser.parseTree(source);
    assert.equal(actual.ok, true, `${source}: ${JSON.stringify(actual.rejection)}`);
    assert.deepEqual(actual.ambiguities, [], source);
    assert.deepEqual(nativeRows(actual.tree, source, projection), rows, source);
    assert.equal(text(actual.tree), source);
  }
});
