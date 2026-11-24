import assert from 'node:assert/strict';
import { writeProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar, importJsonSchema, validateJson } from '../../js/src/index.js';

const contactAlternatives = [
  { properties: { email: { type: 'string', format: 'email' } }, additionalProperties: false },
  { properties: { phone: { type: 'string', pattern: '^[0-9]{3}$' } }, additionalProperties: false },
];
const contact = { type: 'object', oneOf: contactAlternatives };
const fixtures = {
  grammars: [
    [contact, [
      ['{}', false],
      ['{"email":"a@example.org"}', true],
      ['{"ph\\u006fne":"123"}', true],
      ['{"email":"a@example.org","phone":"123"}', false],
      ['{"email":"invalid"}', false],
      ['{"phone":"12"}', false],
      ['{"email":"a@example.org","message":"hello"}', false],
    ]],
    [{ ...contact, properties: { email: { pattern: '^a' } }, required: ['email'] }, [
      ['{"email":"a@example.org"}', true],
      ['{"email":"b@example.org"}', false],
      ['{"email":"invalid"}', false],
      ['{"phone":"123"}', false],
      ['{}', false],
    ]],
    [{ type: 'object', oneOf: [
      { properties: { mode: { enum: ['recorded', 'shared'] }, source: { type: 'string' } }, additionalProperties: false },
      { properties: { mode: { enum: ['shared', 'planned'] }, note: { type: 'string' } }, additionalProperties: false },
    ] }, [
      ['{"mode":"shared"}', false],
      ['{"mode":"shared","source":"quote"}', true],
      ['{"note":"hello","mode":"shared"}', true],
      ['{"source":"quote","note":"hello"}', false],
      ['{"source":"quote","mode":"planned"}', false],
      ['{"mode":"recorded"}', true],
      ['{"mode":"planned"}', true],
      ['{"note":false}', false],
      ['{}', false],
    ]],
    [{ type: 'object', oneOf: [
      { properties: { value: { const: 'declared' } }, additionalProperties: { type: 'null' } },
      { properties: { note: { const: 'planned' } }, additionalProperties: { type: 'null' } },
    ] }, [
      ['{"source":null}', false],
      ['{"value":"declared","note":null,"source":null}', true],
      ['{"value":null,"note":"planned"}', true],
      ['{"value":"declared","note":"planned"}', false],
      ['{"value":null,"note":null}', false],
      ['{"note":"planned","source":false}', false],
    ]],
    [{ type: 'object', oneOf: [
      { properties: { message: { type: 'string' } }, additionalProperties: false },
      { additionalProperties: false },
    ] }, [
      ['{"message":"hello"}', true],
      ['{"message":false}', false],
      ['{"source":"hello"}', false],
      ['{}', false],
    ]],
  ],
  ungeneratable: [[{ ...contact, required: ['email', 'phone'] }, [
    ['{"email":"a@example.org","phone":"123"}', false],
    ['{"email":"a@example.org"}', false],
    ['{"phone":"123"}', false],
  ]]],
};

if (process.argv.includes('--write')) {
  writeProjectData(new URL('../../parity/fixtures/json-schema-exclusive-declared-properties.lino', import.meta.url), fixtures);
} else {
  for (const [schema, cases] of fixtures.grammars) {
    const grammar = importJsonSchema(schema);
    const parser = compileGrammar(grammar);
    for (const [source, accepted] of cases) {
      assert.equal(validateJson(source, schema).valid, accepted, `validation of ${source}`);
      assert.equal(parser.parseTree(source).ok, accepted, `decoder for ${source}`);
    }
    assert.deepEqual(grammar.schemaDiagnostics, []);
  }
  for (const [schema, cases] of fixtures.ungeneratable) {
    for (const [source, accepted] of cases) assert.equal(validateJson(source, schema).valid, accepted, source);
    assert.throws(() => importJsonSchema(schema), /no generatable document/u);
  }
  console.log('Exclusive object alternatives count branches using different declared properties.');
}
