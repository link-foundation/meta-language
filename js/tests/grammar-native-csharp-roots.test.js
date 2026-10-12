import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { grammarSourceOf, importSource, sourceText } from '../scripts/import-native-grammars.mjs';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { applyRootSourceTransform } from '../scripts/native-csharp-roots.mjs';

const read = file => readProjectData(new URL(`../../${file}`, import.meta.url));
const fixture = read('parity/fixtures/native-grammars/csharp-root-contexts.json');
const naming = read('parity/naming/grammar-name-expansions.json');
const entry = grammarSourceOf('native-csharp');
const imported = importSource(entry, new Map(naming.words.map(({ word, replacement }) => [word, replacement])), naming.grammars.csharp);
const parser = compileGrammar(parseGrammarLinks(imported.text));
const projection = { ...read('parity/fixtures/native-grammars/csharp-profile.json'), oracleKinds: nativeOracleKinds(imported.text) };
const text = tree => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;

test('recorded root contexts retain the full source alternatives and require exact selectors', () => {
  const original = JSON.parse(sourceText(entry));
  const grammar = structuredClone(original);
  assert.equal(applyRootSourceTransform(grammar, { family: 'unknown-context' }), false);
  assert.deepEqual(grammar, original);
  for (const decision of fixture.decisions) {
    const before = structuredClone(grammar);
    assert.throws(() => applyRootSourceTransform(grammar, { ...decision, count: 2 }), /exactly/u);
    assert.deepEqual(grammar, before, 'a stale context count cannot partially rewrite the source');
    assert.equal(applyRootSourceTransform(grammar, decision), true);
  }
  const initializer = grammar.rules._class_declaration_initializer;
  const repeated = initializer.members.find(node => node.type === 'REPEAT' && node.content.type === 'CHOICE');
  assert.deepEqual(repeated.content.members.map(node => node.type === 'PREC_DYNAMIC' ? node.content.name : node.name), ['type_parameter_list', 'parameter_list', 'base_list']);
  const find = node => node?.type === 'NATIVE_COMPLETE_CONTEXT_VARIANT' ? node
    : Array.isArray(node) ? node.map(find).find(Boolean)
      : node && typeof node === 'object' ? Object.values(node).map(find).find(Boolean) : null;
  const preference = find(grammar.rules.switch_section);
  assert.equal(preference.boundary, ':');
  assert.equal(preference.preferred.members[0].name, 'pattern');
  assert.deepEqual(preference.content.members.map(node => node.type === 'SEQ' ? node.members[0].name : node.name), ['expression', 'pattern']);
  assert.deepEqual(original, JSON.parse(sourceText(entry)), 'source pins remain untouched');
});

test('CSharp class and switch root contexts reproduce every bounded CI failure exactly', () => {
  for (const decision of fixture.decisions) {
    assert.deepEqual(naming.grammars.csharp.sourceTransforms.find(other => other.family === decision.family && other.rule === decision.rule), decision,
      `missing registered source decision: ${decision.family}`);
  }
  assert.equal(entry.id, fixture.source);
  assert.equal(entry.revision, fixture.revision);
  assert.equal(entry.sha256, fixture.sourceSha256);
  assert.equal(entry.corpus.sha256, fixture.corpusSha256);
  assert.equal(fixture.matches.length, 19);
  assert.equal(fixture.matches.filter(({ file }) => file).length, 4);
  for (const { title, source, rows } of fixture.matches) {
    const oracle = oracleSnapshot(source, fixture.language);
    assert.equal(oracle.recovers, false, title);
    assert.deepEqual(oracle.rows, rows, title);
    const actual = parser.parseTree(source);
    assert.equal(actual.ok, true, `${title}: ${source}`);
    assert.deepEqual(actual.ambiguities, [], `${title}: ${source}`);
    assert.deepEqual(nativeRows(actual.tree, source, projection), rows, `${title}: ${source}`);
    assert.equal(text(actual.tree), source, title);
  }
});
