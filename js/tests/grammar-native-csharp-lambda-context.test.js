import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { isDeepStrictEqual } from 'node:util';

import { grammarSourceOf, nativeName, ruleConcept, scannerText, sourceText } from '../scripts/import-native-grammars.mjs';
import { transformNativeSource } from '../scripts/native-grammar-transforms.mjs';
import { nativeRows, oracleSnapshot } from '../scripts/native-grammar-rows.mjs';
import { importTreeSitterNative, renderTreeSitterNative } from '../src/grammar-importers/tree-sitter-native.js';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const read = (file) => readProjectData(new URL(`../../${file}`, import.meta.url));
const fixture = read('parity/fixtures/native-grammars/csharp-lambda-ref-context.json');
const declarations = read('parity/fixtures/native-grammars/csharp-declarations-regressions.json');
const naming = read('parity/naming/grammar-name-expansions.json');
const decisions = naming.grammars.csharp;
const configured = decisions.sourceTransforms;
const withoutReason = ({ reason: _reason, ...decision }) => decision;
const sameDecision = (actual, expected) => isDeepStrictEqual(withoutReason(actual), withoutReason(expected));
let previousIndex = -1;
for (const expected of fixture.decisions) {
  const indices = configured.flatMap((actual, index) => sameDecision(actual, expected) ? [index] : []);
  assert.equal(indices.length, 1, 'each lambda context preference is registered exactly once');
  assert.ok(indices[0] > previousIndex, 'lambda context preferences retain their recorded order');
  previousIndex = indices[0];
}
const baseline = configured.filter((actual) => !fixture.decisions.some((expected) => sameDecision(actual, expected)));
const entry = grammarSourceOf('native-csharp');
const originalSource = JSON.parse(sourceText(entry));
const words = new Map(naming.words.map(({ word, replacement }) => [word, replacement]));
const projection = read('parity/fixtures/native-grammars/csharp-profile.json');
const source = transformNativeSource(originalSource, configured);

function parserOf(grammar) {
  const imported = importTreeSitterNative(grammar, {
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
  return compileGrammar(parseGrammarLinks(links));
}
const parser = parserOf(source);
const text = (tree) => tree.type === 'node' ? tree.children.map(text).join('') : tree.text;

function exactCase({ source: input, rows, title }) {
  const oracle = oracleSnapshot(input, fixture.language);
  assert.equal(oracle.recovers, false, title);
  assert.deepEqual(oracle.rows, rows, `${title}: independent oracle`);
  const native = parser.parseTree(input);
  assert.equal(native.ok, true, title);
  assert.deepEqual(native.ambiguities, [], title);
  assert.deepEqual(nativeRows(native.tree, input, projection), rows, title);
  assert.equal(text(native.tree), input, title);
}

test('the exact CI plain-reference lambda retains the pinned type and source span', () => {
  assert.equal(fixture.provenance.runId, 38227158183);
  assert.equal(fixture.provenance.jobId, 114737371616);
  assert.equal(fixture.provenance.sourceFile, entry.file);
  assert.equal(fixture.provenance.sourceSha256, entry.sha256);
  assert.equal(fixture.provenance.corpusFile, entry.corpus.file);
  assert.equal(fixture.provenance.corpusSha256, entry.corpus.sha256);
  const corpus = read('parity/fixtures/native-grammars/csharp-profile-corpus.json');
  assert.equal(fixture.cases.length, 1);
  const row = fixture.cases[0];
  const original = corpus.cases.find(({ file, title }) => file === row.file && title === row.title);
  assert.ok(original);
  assert.equal(row.source, original.source);
  const difference = fixture.provenance.firstDifference;
  assert.deepEqual(row.rows[difference.index], difference.expected);
  assert.equal(row.rows.length, difference.expectedLength);
  exactCase(row);

  const previous = parserOf(transformNativeSource(originalSource, baseline)).parseTree(row.source);
  assert.equal(previous.ok, true);
  const previousRows = nativeRows(previous.tree, row.source, projection);
  assert.equal(previousRows.length, difference.actualLength);
  assert.deepEqual(previousRows[difference.index], difference.actual);
});

test('readonly and scoped modifiers, mixed lists, contextual identifiers, comments and attributes retain their exact trees', () => {
  assert.equal(fixture.controls.length, 12);
  for (const row of fixture.controls) exactCase(row);
});

test('lambda context preferences preserve all five previous declaration regressions and their controls', () => {
  assert.deepEqual(declarations.provenance.failureIds, [0, 2, 20, 24, 25]);
  for (const expected of declarations.decisions) {
    assert.equal(configured.filter((actual) => sameDecision(actual, expected)).length, 1);
  }
  for (const row of [...declarations.cases, ...declarations.controls]) exactCase(row);
  const before = transformNativeSource(originalSource, baseline);
  for (const name of ['parameter_list', 'parameter', '_parameter_type_with_modifiers', 'type']) {
    assert.deepEqual(source.rules[name], before.rules[name], `${name}: original declaration production remains unchanged`);
  }
  assert.notDeepEqual(source.rules._lambda_parameters, before.rules._lambda_parameters);
});

test('lambda context decisions reject stale source rules, duplicate helpers and exact-count changes', () => {
  const prefix = fixture.decisions.slice(0, -1);
  const context = fixture.decisions.at(-1);
  assert.throws(() => transformNativeSource(originalSource, [
    ...baseline, ...prefix, { ...context, count: context.count + 1 },
  ]), /exactly their recorded source occurrences/u);
  assert.throws(() => transformNativeSource(originalSource, [
    ...baseline, { ...fixture.decisions[0], rules: ['absent_lambda_rule'] },
  ]), /context variant rule.*absent/u);
  assert.throws(() => transformNativeSource(originalSource, [
    ...baseline, fixture.decisions[0], fixture.decisions[0],
  ]), /context helper must have a unique name/u);
});
