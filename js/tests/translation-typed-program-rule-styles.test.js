import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';

import * as decisions from '../src/typed-program-link-rules.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { parseRust } from '../src/translation/rust.js';
import { readProjectData } from '../scripts/project-data.mjs';

test('reviewed Rust styles preserve every shared schema decision through translation back to JavaScript', () => {
  const schema = readProjectData(new URL('../src/data/typed-program-links-schema.lino', import.meta.url));
  const generated = readFileSync(new URL('../../rust/src/translation/typed_program_link_rules.rs', import.meta.url), 'utf8');
  // These are library declarations. The empty executable entry point makes
  // their existing Rust -> semantic program -> JavaScript adapter available.
  const replay = {};
  runInNewContext(emitJavaScript(checkProgram(parseRust(`${generated}\nfn main() {}\n`))).text, replay, { timeout: 1_000 });
  let observations = 0;
  const check = (operation, ...values) => {
    const nativeName = operation.replace(/[A-Z]/gu, character => `_${character.toLowerCase()}`);
    assert.equal(replay[nativeName](...values), decisions[operation](...values), `${operation}(${values.map(value => JSON.stringify(value)).join(', ')})`);
    observations += 1;
  };
  const variants = values => [...new Set(values), 'unrepresented decision'];
  for (const [operation, table] of [
    ['readProgramContainerKind', schema.containers],
    ['readProgramScalarKind', schema.scalars],
    ['readProgramPatternBindingForm', schema.patternBindingForms],
    ['readProgramExpressionOperandFields', schema.expressionOperands],
  ]) for (const value of variants(operation.includes('Kind') ? Object.values(table) : Object.keys(table))) check(operation, value);
  const labels = variants([...Object.values(schema.references), ...Object.keys(schema.referenceAliases ?? {})]);
  for (const label of labels) {
    check('readProgramReferenceRole', label);
    check('readProgramReferenceTargetField', label);
    check('acceptProgramReferenceField', label);
  }
  const sourceFields = Object.values(schema.referenceSourceFields).flat();
  for (const label of labels) for (const construct of variants(sourceFields.map(row => row[0]))) {
    for (const type of variants(sourceFields.map(row => row[1]))) check('readProgramReferenceSourceField', label, construct, type);
  }
  for (const kind of variants(Object.keys(schema.containerFieldPolicies))) {
    for (const fields of [false, true]) check('acceptProgramContainerFields', kind, fields);
  }
  for (const kind of variants(Object.keys(schema.expressionFields))) {
    for (const field of variants(Object.values(schema.expressionFields).flat())) check('acceptProgramExpressionField', kind, field);
  }
  const contexts = variants([...Object.keys(schema.neutralFieldDefaults),
    ...Object.keys(schema.expressionProjections).map(kind => `expression:${kind.split(':')[0]}`)]);
  const fields = variants([...Object.values(schema.neutralFieldDefaults).flatMap(Object.keys), 'span']);
  for (const context of contexts) for (const field of fields) check('readProgramNeutralFieldDefault', context, field);
  for (const model of variants(schema.tupleProductModels)) {
    for (const frozen of [false, true]) check('acceptProgramTupleProductModel', model, frozen);
  }
  for (const kind of variants(Object.keys(schema.expressionProjections))) check('readProgramExpressionProjection', kind);
  for (const kind of variants(Object.keys(schema.expressionOperations))) {
    for (const operation of variants(Object.values(schema.expressionOperations).flat())) check('acceptProgramExpressionOperation', kind, operation);
  }
  for (const name of ['__proto__', 'constructor', 'prototype', 'safe field']) check('isUnsafeProgramField', name);
  for (const term of variants([schema.root])) for (const revision of variants([String(schema.schemaVersion)])) {
    for (const arity of variants(['1'])) check('acceptProgramSchemaRoot', term, revision, arity);
  }
  for (const width of variants(schema.machineIntegerWidths.map(String))) check('acceptProgramMachineIntegerWidth', width);
  for (const absence of variants(schema.optionalAbsences)) check('acceptProgramOptionalAbsence', absence);
  for (const rounding of variants(schema.divisionRoundingPolicies)) check('acceptProgramDivisionRounding', rounding);
  assert.ok(observations > 1_000, 'the replay covers interacting decisions and negative alternatives');
});
