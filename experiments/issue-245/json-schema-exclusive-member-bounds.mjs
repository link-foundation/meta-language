import assert from 'node:assert/strict';
import { writeProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar, importJsonSchema, validateJson } from '../../js/src/index.js';

const left = { type: 'number', minimum: -1, exclusiveMaximum: 2 };
const right = { type: 'number', minimum: 1, maximum: 3 };
const array = (items, constraints = {}) => ({ items, ...constraints });
const object = amount => ({ properties: { amount }, additionalProperties: false });
const fixtures = {
  grammars: [
    [{ type: 'array', oneOf: [array(left), array(right)] }, [
      ['[]', false], ['[1]', false], ['[1.5]', false], ['[1,1.999]', false],
      ['[-1]', true], ['[-1.00]', true], ['[-1.01]', false], ['[-0]', true],
      ['[2]', true], ['[3]', true], ['[3.0001]', false], ['[0.5,1.5]', true],
      ['[1.5,2]', true], ['[0,2]', false], ['[0,1.5,1,1.9,0]', true],
      ['[null]', false], ['[1.5,]', false],
    ], ['/items/type'], [['[5e-1]', true]]],
    [{ type: 'object', required: ['amount'], properties: { amount: { minimum: 0, maximum: 2 } },
      $defs: { left }, oneOf: [object({ $ref: '#/$defs/left', minimum: 0.5 }), object(right)] }, [
      ['{}', false], ['{"amount":0.49}', false], ['{"amount":0.50}', true],
      ['{"amount":0.999}', true], ['{"amount":1}', false], ['{"amount":1.45}', false],
      ['{"amount":2}', true], ['{"amount":2.01}', false], ['{"am\\u006funt":2.0}', true],
      ['{"amount":2,"extra":0}', false],
    ], ['/properties/amount/type'], [['{"amount":20e-1}', true]]],
    [{ type: 'array', oneOf: [
      array({ type: ['number', 'boolean'], minimum: 0, maximum: 2 }),
      array({ type: ['integer', 'null'], minimum: 1, maximum: 3 }),
    ] }, [
      ['[]', false], ['[1]', false], ['[1.00]', false], ['[1.25]', true],
      ['[2.000]', false], ['[2.05]', false], ['[3.0]', true], ['[3.1]', false],
      ['[1.25,1]', true], ['[1.25,3]', false], ['[true,1]', true],
      ['[null,1.0]', true], ['[null,3,3.0]', true], ['[true,null]', false], ['["1"]', false],
    ], ['/items/type'], [['[30e-1]', true]]],
    [{ type: 'array', items: { maximum: 3 }, oneOf: [
      array({ type: 'integer', exclusiveMinimum: 0, maximum: 2 }, { maxItems: 2 }),
      array({ type: 'integer', minimum: 2, exclusiveMaximum: 4 }, { minItems: 1 }),
    ] }, [
      ['[]', true], ['[0]', false], ['[1.0]', true], ['[2.000]', false], ['[3]', true],
      ['[2,2]', false], ['[2,2,2]', true], ['[3,2,3]', true], ['[1,3]', false],
      ['[2.1]', false], ['[4]', false],
    ], ['/items/type'], [['[3e0]', true]]],
    [{ type: 'object', oneOf: [
      object({ type: 'number', minimum: -1, maximum: 0 }),
      object({ type: 'number', minimum: 0, exclusiveMaximum: 1 }),
      object({ type: 'number', minimum: 0.25, maximum: 0.75 }),
    ] }, [
      ['{}', false], ['{"amount":-1}', true], ['{"amount":-0}', false],
      ['{"amount":0.00}', false], ['{"amount":0.125}', true],
      ['{"amount":0.25}', false], ['{"amount":0.5}', false],
      ['{"amount":0.75}', false], ['{"amount":0.875}', true], ['{"amount":1}', false],
    ], ['/properties/amount/type'], [['{"amount":875e-3}', true]]],
  ],
  ungeneratable: [
    [{ type: 'array', oneOf: [array(left), array({ ...left, exclusiveMinimum: -2 })] }, [
      ['[]', false], ['[-1]', false], ['[1.9]', false], ['[2]', false],
    ]],
  ],
};

if (process.argv.includes('--write')) {
  writeProjectData(new URL('../../parity/fixtures/json-schema-exclusive-member-bounds.lino', import.meta.url), fixtures);
} else {
  for (const [schema, cases, , validationOnly] of fixtures.grammars) {
    for (const [source, accepted] of [...cases, ...validationOnly]) assert.equal(validateJson(source, schema).valid, accepted, source);
  }
  for (const [schema, cases] of fixtures.ungeneratable) {
    for (const [source, accepted] of cases) assert.equal(validateJson(source, schema).valid, accepted, source);
  }
  for (const [schema, cases, pointers] of fixtures.grammars) {
    const grammar = importJsonSchema(schema);
    const parser = compileGrammar(grammar);
    for (const [source, accepted] of cases) assert.equal(parser.parseTree(source).ok, accepted, source);
    assert.deepEqual(grammar.schemaDiagnostics.map(issue => issue.pointer), pointers, JSON.stringify(schema));
  }
  for (const [schema] of fixtures.ungeneratable) {
    assert.throws(() => importJsonSchema(schema), /no generatable document/u);
  }
  console.log('Exclusive collection member bounds agree with document validation on all shared decimal cases.');
}
