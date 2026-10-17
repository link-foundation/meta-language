import assert from 'node:assert/strict';
import test from 'node:test';
import { normalizeNativeConceptReuse } from '../scripts/normalize-native-concept-reuse.mjs';
import { readStructuredDataLino, structuredDataToLino } from '../src/structured-data-links.js';

function report() {
  return { generatedBy: 'js/scripts/build-native-grammar-concept-reuse.mjs', grammars: [
    { grammar: 'native-alpha', shared: [{ rule: 'expression', concept: 'grammar.expression', languages: ['alpha', 'beta'] }], specific: [] },
    { grammar: 'native-beta', shared: [{ rule: 'expression', concept: 'grammar.expression', languages: ['alpha', 'beta'] }], specific: [] },
  ], concepts: [{ concept: 'grammar.expression', languages: ['alpha', 'beta'], shared: true }] };
}

test('normalization restores canonical shared lists without changing any ordered report value', () => {
  const original = report();
  const normalized = normalizeNativeConceptReuse(original);
  assert.deepEqual(normalized, original);
  assert.notEqual(original.grammars[0].shared[0].languages, original.concepts[0].languages);
  const loaded = readStructuredDataLino(structuredDataToLino(normalized));
  assert.equal(loaded.grammars[0].shared[0].languages, loaded.concepts[0].languages);
  assert.equal(loaded.grammars[1].shared[0].languages, loaded.concepts[0].languages);
  assert.equal(structuredDataToLino(normalizeNativeConceptReuse(loaded)), structuredDataToLino(normalized));
});

test('normalization refuses changed lists, unknown concepts and duplicate identities', () => {
  const changed = report(); changed.grammars[0].shared[0].languages.reverse();
  assert.throws(() => normalizeNativeConceptReuse(changed), /language list differs/);
  const unknown = report(); unknown.grammars[0].shared[0].concept = 'grammar.unknown';
  assert.throws(() => normalizeNativeConceptReuse(unknown), /language list differs/);
  const duplicate = report(); duplicate.concepts.push(duplicate.concepts[0]);
  assert.throws(() => normalizeNativeConceptReuse(duplicate), /invalid canonical/);
});
