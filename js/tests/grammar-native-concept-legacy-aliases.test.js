import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { GENERATED, mergeConcepts } from '../scripts/import-native-grammars.mjs';
import { conceptRecord, conceptRecordsForSourceName, currentConceptId } from '../src/concept-records.js';

const current = 'grammar.fixture-call-without-parentheses';
const retired = 'grammar.fixture-unparenthesized-call';
const entry = { language: 'elixir' };
const imported = [{ entry, rules: [{ name: 'fixture_call_without_parentheses',
  sourceName: '_fixture_unparenthesized_call', concept: current }] }];
const register = () => ({ concepts: [{ id: current, phrase: 'fixture call without parentheses',
  role: 'concept', definition: 'A source call production without parentheses.',
  constraints: ['Only the native Elixir grammar defines this construct.', GENERATED],
  sourceAliases: [{ source: 'native:elixir', name: 'fixture_call_without_parentheses' }],
  formerNames: [] }] });
const decisions = () => ({ legacyAliases: { [current]: [{ source: 'native:elixir', name: 'fixture_unparenthesized_call' }] },
  formerNames: { [current]: [retired] } });

test('native regeneration preserves exact legacy source names and retired identities without duplication', () => {
  const input = register();
  const before = structuredClone(input);
  const first = mergeConcepts(input, imported, decisions());
  assert.deepEqual(input, before, 'source metadata is not mutated in place');
  assert.deepEqual(first.concepts[0].sourceAliases, [
    { source: 'native:elixir', name: 'fixture_call_without_parentheses' },
    { source: 'native:elixir', name: 'fixture_unparenthesized_call' },
  ]);
  assert.deepEqual(first.concepts[0].formerNames, [retired]);
  assert.equal(first.concepts[0].definition, before.concepts[0].definition);
  assert.deepEqual(mergeConcepts(first, imported, decisions()), first,
    'a second source generation retains the identical source record and alias order');
});

test('legacy decisions reject unknown concepts, conflicting identities and revival of retired names', () => {
  const invalid = [
    { legacyAliases: [] },
    { legacyAliases: { [current]: [{ source: 'unknown', name: 'old' }] } },
    { formerNames: { [current]: [current] } },
    { formerNames: { [current]: [retired, retired] } },
    { legacyAliases: { 'grammar.missing': [{ source: 'native:elixir', name: 'old' }] } },
  ];
  for (const decision of invalid) assert.throws(() => mergeConcepts(register(), imported, decision), TypeError);
  const conflicting = register();
  conflicting.concepts[0].formerNames = [retired];
  assert.throws(() => mergeConcepts(conflicting, imported, { formerNames: { 'grammar.other': [retired] } }),
    /cannot name different concepts/u);
  assert.throws(() => mergeConcepts(register(), [{ entry, rules: [{ name: 'old', concept: retired }] }], decisions()),
    /a former name/u, 'a source cannot silently restore a retired canonical identity');
});

test('the seven reviewed Elixir migrations resolve old identities and spellings through shipped concept records', () => {
  const naming = readProjectData(new URL('../../parity/naming/grammar-name-expansions.json', import.meta.url)).concepts;
  const migrations = [
    ['bare-argument-unparenthesized-call', 'bare-argument-call-without-parentheses'],
    ['bare-argument-local-unparenthesized-call', 'bare-argument-local-call-without-parentheses'],
    ['bare-argument-remote-unparenthesized-call', 'bare-argument-remote-call-without-parentheses'],
    ['postfix-unparenthesized-call', 'postfix-call-without-parentheses'],
    ['attribute-unparenthesized-call', 'attribute-call-without-parentheses'],
    ['attribute-local-unparenthesized-call', 'attribute-local-call-without-parentheses'],
    ['postfix-attribute-unparenthesized-call', 'postfix-attribute-call-without-parentheses'],
  ];
  for (const [old, current] of migrations) {
    const id = `grammar.${current}`;
    const former = `grammar.${old}`;
    const alias = { source: 'native:elixir', name: old.replaceAll('-', '_') };
    assert.ok(naming.formerNames[id].includes(former), former);
    assert.ok(naming.legacyAliases[id].some(({ source, name }) => source === alias.source && name === alias.name), alias.name);
    const record = conceptRecord(id);
    assert.ok(record, id);
    assert.equal(conceptRecord(former), record, former);
    assert.equal(currentConceptId(former), id, former);
    assert.equal(currentConceptId(id), id, id);
    assert.ok(conceptRecordsForSourceName(alias.source, alias.name).includes(record), alias.name);
  }
});

test('the reviewed CSharp type test name retains its generated identity and spelling', () => {
  const record = conceptRecord('grammar.null-capable-type-test-expression');
  assert.ok(record);
  assert.equal(conceptRecord('grammar.null-admitting-type-test-expression'), record);
  assert.equal(currentConceptId('grammar.null-admitting-type-test-expression'), record.id);
  assert.equal(currentConceptId(record.id), record.id);
  assert.ok(conceptRecordsForSourceName('native:csharp', 'null_admitting_type_test_expression').includes(record));
});

test('unknown concept identities remain unchanged', () => {
  const unknown = 'grammar.unregistered-concept-identity';
  assert.equal(conceptRecord(unknown), undefined);
  assert.equal(currentConceptId(unknown), unknown);
});
