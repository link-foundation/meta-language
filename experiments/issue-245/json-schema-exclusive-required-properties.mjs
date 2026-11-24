import assert from 'node:assert/strict';
import { writeProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar, importJsonSchema, validateJson } from '../../js/src/index.js';

const contact = {
  type: 'object',
  properties: {
    email: { type: 'string', format: 'email' },
    phone: { type: 'string', pattern: '^[0-9]{3}$' },
    message: { type: 'string' },
  },
  additionalProperties: false,
  required: ['message'],
  $defs: { emailContact: { required: ['email'] } },
  oneOf: [{ $ref: '#/$defs/emailContact' }, { required: ['phone'] }],
};
const fixtures = {
  grammars: [
    [contact, [
      ['{"email":"a@example.org","phone":"123","message":"hello"}', false],
      ['{"message":"hello","e\\u006dail":"a@example.org"}', true],
      ['{"phone":"123","message":"hello"}', true],
      ['{"email":"invalid","message":"hello"}', false],
      ['{"phone":"12","message":"hello"}', false],
      ['{"email":"a@example.org","phone":"12","message":"hello"}', false],
      ['{"email":"a@example.org"}', false],
      ['{"message":"hello"}', false],
      ['{"phone":"123","message":"hello","extra":null}', false],
    ]],
    [{ type: 'object', oneOf: [{ required: ['__proto__'] }, { required: ['source'] }] }, [
      ['{"__proto__":null,"evidence":[false]}', true],
      ['{"source":{"quote":"hello"}}', true],
      ['{"__pr\\u006fto__":null,"source":null}', false],
      ['{}', false],
    ]],
    [{ type: 'object', oneOf: [
      { properties: { mode: { enum: ['recorded', 'shared'] }, source: { type: 'string' } }, required: ['mode'] },
      { properties: { mode: { enum: ['shared', 'planned'] }, source: { type: 'string' } }, required: ['source'] },
    ] }, [
      ['{"mode":"shared","source":"quote"}', false],
      ['{"mode":"shared"}', true],
      ['{"mode":"planned"}', false],
      ['{"source":"quote","mode":"planned"}', true],
      ['{"mode":"recorded","source":"quote"}', true],
      ['{"source":"quote"}', true],
      ['{"source":false}', false],
      ['{}', false],
    ]],
  ],
  ungeneratable: [[{ ...contact, required: ['message', 'email', 'phone'] }, [
    ['{"email":"a@example.org","phone":"123","message":"hello"}', false],
    ['{"email":"a@example.org","message":"hello"}', false],
    ['{"phone":"123","message":"hello"}', false],
  ]]],
};

if (process.argv.includes('--write')) {
  writeProjectData(new URL('../../parity/fixtures/json-schema-exclusive-required-properties.lino', import.meta.url), fixtures);
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
  console.log('Exclusive object alternatives count branches using distinct required property names.');
}
