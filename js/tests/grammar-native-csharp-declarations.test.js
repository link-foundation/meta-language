import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

import { applyDeclarationSourceTransform } from '../scripts/native-csharp-declarations.mjs';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';
import { grammarSourceOf, nativeName, ruleConcept, scannerText, sourceText } from '../scripts/import-native-grammars.mjs';
import { importTreeSitterNative, renderTreeSitterNative } from '../src/grammar-importers/tree-sitter-native.js';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';

const read = (file) => readFileSync(new URL(`../../${file}`, import.meta.url), 'utf8');
const fixture = JSON.parse(read('parity/fixtures/native-grammars/csharp-declarations-regressions.json'));
const naming = JSON.parse(read('parity/naming/grammar-name-expansions.json'));
const decisions = naming.grammars.csharp;
const entry = grammarSourceOf('native-csharp');
const words = new Map(naming.words.map(({ word, replacement }) => [word, replacement]));
const selector = ({ family, rule, rules, field, alias }) => JSON.stringify([family, rule, rules, field, alias]);
const configured = [...decisions.sourceTransforms];
for (const decision of fixture.decisions) {
  const existing = configured.find((candidate) => selector(candidate) === selector(decision));
  if (existing) {
    const { reason: _reason, ...actual } = existing;
    const { reason: _expectedReason, ...expected } = decision;
    assert.deepEqual(actual, expected, 'the registered preference matches its bounded regression policy');
  } else configured.push(decision);
}
const source = transformNativeSource(JSON.parse(sourceText(entry)), configured);
const imported = importTreeSitterNative(source, {
  nameOf: (name) => nativeName(name, words, decisions),
  wordRule: 'word_characters',
  scanners: scannerText(entry),
  immediate: entry.scanner?.immediate ?? [],
});
const rules = imported.rules.map((rule) => ({ ...rule, concept: ruleConcept(rule, decisions) }));
const links = renderTreeSitterNative({ ...imported, rules }, {
  annotate: (rule) => [
    `(concept ${rule.concept})`,
    ...(rule.sourceName !== null && rule.sourceName !== rule.name ? [`(source-names (tree-sitter ${rule.sourceName}))`] : []),
  ],
});
const parser = compileGrammar(parseGrammarLinks(links));
const profile = JSON.parse(read('parity/fixtures/native-grammars/csharp-profile.json'));
const text = (tree) => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;

function exactCase({ source, rows, title }) {
  const oracle = oracleSnapshot(source, fixture.language);
  assert.equal(oracle.recovers, false, title);
  assert.deepEqual(oracle.rows, rows, `${title}: independent oracle`);
  const native = parser.parseTree(source);
  assert.equal(native.ok, true, title);
  assert.deepEqual(native.ambiguities, [], title);
  assert.deepEqual(nativeRows(native.tree, source, profile), rows, title);
  assert.equal(text(native.tree), source, title);
}

test('the five declaration regressions retain their exact CI sources and pinned grammar', () => {
  assert.deepEqual(fixture.provenance.failureIds, [0, 2, 20, 24, 25]);
  assert.equal(fixture.provenance.sourceFile, entry.file);
  assert.equal(fixture.provenance.sourceSha256, entry.sha256);
  assert.equal(fixture.provenance.corpusFile, entry.corpus.file);
  assert.equal(fixture.provenance.corpusSha256, entry.corpus.sha256);
  const corpus = JSON.parse(read('parity/fixtures/native-grammars/csharp-profile-corpus.json'));
  for (const row of fixture.cases) {
    const original = corpus.cases.find(({ file, title }) => file === row.file && title === row.title);
    assert.equal(row.source, original.source, row.title);
    exactCase(row);
  }
});

test('positional attributes, equality expressions and contextual type names remain complete alternatives', () => {
  for (const row of fixture.controls) exactCase(row);
});

test('declaration preferences reject stale selectors and exact-count mismatches', () => {
  const original = JSON.parse(sourceText(entry));
  for (const decision of fixture.decisions) {
    assert.throws(() => applyDeclarationSourceTransform(structuredClone(original), { ...decision, count: decision.count + 1 }), /recorded declaration occurrences/u);
  }
  assert.throws(() => applyDeclarationSourceTransform(structuredClone(original), {
    ...fixture.decisions[1], alias: 'absent_modifier',
  }), /recorded declaration occurrences/u);
  assert.throws(() => applyDeclarationSourceTransform(structuredClone(original), {
    ...fixture.decisions[0], rule: 'absent_rule',
  }), /existing rule/u);
  const untouched = structuredClone(original);
  assert.equal(applyDeclarationSourceTransform(untouched, { family: 'another-source-family' }), false);
  assert.deepEqual(untouched, original);
});
