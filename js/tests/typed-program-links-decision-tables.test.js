import assert from 'node:assert/strict';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import * as rules from '../src/typed-program-link-rules.js';
import { readRustConditionalCompletion, readRustCompletionKind } from '../src/translation/rust-completion-rules.js';

const schema = readProjectData(new URL('../src/data/typed-program-links-schema.lino', import.meta.url));
const completion = readProjectData(new URL('../src/translation/data/rust-completions.lino', import.meta.url));

test('generated typed decisions retain every declared selector and off-diagonal refusal', () => {
  for (const [table, reader] of [[schema.containers, rules.readProgramContainerKind], [schema.scalars, rules.readProgramScalarKind]]) {
    for (const [kind, term] of Object.entries(table)) assert.equal(reader(term), kind, term);
    assert.equal(reader('unregistered field'), '');
  }
  for (const [table, reader, render] of [
    [schema.patternBindingForms, rules.readProgramPatternBindingForm, value => value],
    [schema.expressionOperands, rules.readProgramExpressionOperandFields, value => value.join(' ')],
  ]) {
    for (const [kind, value] of Object.entries(table)) assert.equal(reader(kind), render(value), kind);
    assert.equal(reader('unregistered kind'), '');
  }
  for (const [kind, policy] of Object.entries(schema.containerFieldPolicies)) for (const hasFields of [false, true]) {
    assert.equal(rules.acceptProgramContainerFields(kind, hasFields), policy === 'named' || !hasFields);
  }
  assert.equal(rules.acceptProgramContainerFields('unregistered kind', false), false);
  const roles = new Map(Object.entries(schema.references).map(([role, label]) => [label, role]));
  for (const [label, role] of Object.entries(schema.referenceAliases)) roles.set(label, role);
  const constructs = new Set(['unregistered kind', '']);
  const types = new Set(['unregistered type', '']);
  for (const cases of Object.values(schema.referenceSourceFields)) for (const [construct, type] of cases) { constructs.add(construct); types.add(type); }
  for (const [label, role] of [...roles, ['unregistered reference', null]]) for (const construct of constructs) for (const type of types) {
    const expected = role && schema.referenceSourceFields[role].find(([candidate, candidateType]) => candidate === construct && candidateType === type)?.[2] || '';
    assert.equal(rules.readProgramReferenceSourceField(label, construct, type), expected, `${label}/${construct}/${type}`);
  }
  for (const [label, role] of roles) {
    assert.equal(rules.readProgramReferenceRole(label), role);
    assert.equal(rules.readProgramReferenceTargetField(label), schema.referenceTargetFields[role]);
    assert.equal(rules.acceptProgramReferenceField(label), true);
  }
  assert.equal(rules.readProgramReferenceRole('unregistered reference'), '');
  assert.equal(rules.readProgramReferenceTargetField('unregistered reference'), '');
  assert.equal(rules.acceptProgramReferenceField('unregistered reference'), false);
  for (const [table, reader] of [[schema.expressionFields, rules.acceptProgramExpressionField], [schema.expressionOperations, rules.acceptProgramExpressionOperation]]) {
    const values = new Set(['unregistered field', ...Object.values(table).flat()]);
    for (const kind of [...Object.keys(table), 'unregistered kind']) for (const value of values) assert.equal(reader(kind, value), table[kind]?.includes(value) ?? false, `${kind}/${value}`);
  }
  for (const [context, fields] of Object.entries(schema.neutralFieldDefaults)) for (const [field, value] of Object.entries(fields)) assert.equal(rules.readProgramNeutralFieldDefault(context, field), value);
  assert.equal(rules.readProgramNeutralFieldDefault('unregistered context', 'unregistered field'), '');
});

test('grouped completion decisions retain complete original table priority and unknown states', () => {
  const kinds = ['', 'value', 'returned', 'aborted', 'unknown'];
  for (const then of kinds) for (const otherwise of kinds) for (const hasElse of [false, true]) for (const continuation of [false, true]) {
    const row = completion.actions.find(row => row.then === then && row.otherwise === otherwise && row.hasElse === hasElse && row.continuation === continuation);
    assert.equal(readRustConditionalCompletion(then, otherwise, hasElse, continuation), row?.action ?? 'unsupported completion');
  }
  for (const action of new Set(['unknown action', ...completion.resultKinds.map(row => row.action)])) for (const then of kinds) for (const otherwise of kinds) {
    const row = completion.resultKinds.find(row => row.action === action && row.then === then && row.otherwise === otherwise);
    assert.equal(readRustCompletionKind(action, then, otherwise), row?.result ?? 'unsupported completion');
  }
});
