import assert from 'node:assert/strict';
import fs from 'node:fs';
import { syncBuiltinESMExports } from 'node:module';
import test from 'node:test';
import { Parser } from 'links-notation';

import { CONCEPT_RECORDS, conceptRecordsForSourceName } from '../src/concept-records.js';
import { conceptCorrespondence, sourceMeanings } from '../src/concept-distinctions.js';
import { FOUNDATION_MODELS } from '../src/foundation-models.js';
import { nativeGrammarRuleConcepts, translateNativeConstruct } from '../src/grammar-concepts.js';

const alias = (source, name) => ({ source, name });

test('shipped alias lookup keeps register order, ambiguity and independent result arrays', () => {
  const expected = CONCEPT_RECORDS.filter(record => record.sourceAliases.some(candidate =>
    candidate.source === 'Python' && candidate.name === '='));
  assert.deepEqual(expected.map(record => record.id), ['binding', 'assignment']);
  const first = conceptRecordsForSourceName('Python', '=');
  assert.deepEqual(first, expected);
  first.reverse(); first.pop();
  assert.deepEqual(conceptRecordsForSourceName('Python', '='), expected);
  assert.deepEqual(sourceMeanings(alias('Python', '=')), ['binding', 'assignment']);
  assert.deepEqual(sourceMeanings(alias('Python', '='), { within: 'binding' }), ['binding']);
  assert.equal(conceptCorrespondence(alias('Python', '='), alias('Python', '=')).relation, 'ambiguous');
  assert.deepEqual(conceptRecordsForSourceName('absent source', 'absent name'), []);
});

test('caller supplied records remain live after array, alias and definition edits', () => {
  const records = [
    { id: 'example.first', definition: 'The first declared meaning.', sourceAliases: [alias('source', 'name'), alias('source', 'name')] },
    { id: 'example.second', definition: 'The second declared meaning.', sourceAliases: [alias('other', 'name')] },
  ];
  const options = { records, register: { models: [], distinctions: [], correspondences: [] } };
  assert.deepEqual(sourceMeanings(alias('source', 'name'), options), ['example.first']);
  records[1].sourceAliases.push(alias('source', 'name'));
  assert.deepEqual(sourceMeanings(alias('source', 'name'), options), ['example.first', 'example.second']);
  assert.equal(conceptCorrespondence(alias('source', 'name'), alias('other', 'name'), options).relation, 'ambiguous');
  records.shift();
  assert.equal(conceptCorrespondence(alias('source', 'name'), alias('other', 'name'), options).shared, 'example.second');
  records[0].definition = 'The revised declared meaning.';
  assert.equal(conceptCorrespondence(alias('source', 'name'), alias('other', 'name'), options).justification, records[0].definition);
  records[0].sourceAliases.pop();
  assert.deepEqual(sourceMeanings(alias('source', 'name'), options), []);
});

test('custom foundation aliases remain live while shipped concepts are indexed', () => {
  const register = structuredClone(FOUNDATION_MODELS);
  const model = register.models[0];
  model.sourceAliases.push(alias('custom source', 'first, second'));
  const options = { records: CONCEPT_RECORDS, register };
  assert.deepEqual(sourceMeanings(alias('custom source', 'first'), options), [model.id]);
  model.sourceAliases.at(-1).name = 'third';
  assert.deepEqual(sourceMeanings(alias('custom source', 'first'), options), []);
  assert.deepEqual(sourceMeanings(alias('custom source', 'third'), options), [model.id]);
});

test('native construct lookup observes mutable custom target aliases', () => {
  const record = { id: 'example.construct', sourceAliases: [alias('native:json', 'pair'), alias('native:diff', 'first')] };
  const options = { records: [record] };
  assert.deepEqual(translateNativeConstruct('native-json', 'pair', 'native-diff', options), { relation: 'translated', concept: record.id, rules: ['first'] });
  record.sourceAliases.at(-1).name = 'second';
  assert.deepEqual(translateNativeConstruct('native-json', 'pair', 'native-diff', options).rules, ['second']);
  record.sourceAliases.pop();
  assert.equal(translateNativeConstruct('native-json', 'pair', 'native-diff', options).relation, 'untranslatable');
});

test('native rule summaries isolate caller edits and invalidate changed grammar text', t => {
  const originalRead = fs.readFileSync;
  const original = originalRead(new URL('../src/data/native-grammars/json.lino', import.meta.url), 'utf8');
  let text = original;
  const reader = t.mock.method(fs, 'readFileSync', (file, ...args) => {
    if (String(file).endsWith('/native-grammars/json.lino')) return text;
    return originalRead(file, ...args);
  });
  const originalParse = Parser.prototype.parse;
  let parses = 0;
  t.mock.method(Parser.prototype, 'parse', function (...args) {
    parses += 1;
    return originalParse.apply(this, args);
  });
  syncBuiltinESMExports();
  t.after(() => { reader.mock.restore(); syncBuiltinESMExports(); });
  const expected = nativeGrammarRuleConcepts('native-json');
  const initialParses = parses;
  const returned = nativeGrammarRuleConcepts('native-json');
  returned[0].concept = 'invalid caller edit'; returned.pop();
  assert.deepEqual(nativeGrammarRuleConcepts('native-json'), expected);
  assert.equal(parses, initialParses, 'unchanged snapshots do not parse again');
  const originalConcept = expected.find(entry => entry.concept !== null).concept;
  const replacement = 'grammar.changed-cache-fixture';
  text = original.replace(`(concept ${originalConcept})`, `(concept ${replacement})`);
  assert.notEqual(text, original);
  const changed = nativeGrammarRuleConcepts('native-json');
  assert.ok(changed.some(entry => entry.concept === replacement));
  assert.equal(parses, initialParses + 1);
  text = original;
  assert.deepEqual(nativeGrammarRuleConcepts('native-json'), expected);
  assert.equal(parses, initialParses + 2);
});
