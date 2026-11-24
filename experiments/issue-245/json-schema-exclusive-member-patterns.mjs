import assert from 'node:assert/strict';
import { writeProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar, importJsonSchema, validateJson } from '../../js/src/index.js';

const prefix = { type: 'string', pattern: '^a' };
const suffix = { type: 'string', pattern: 'z$' };
const array = (items, constraints = {}) => ({ items, ...constraints });
const object = detail => ({ properties: { detail }, additionalProperties: false });
const fixtures = {
  grammars: [
    [{ type: 'array', oneOf: [array(prefix), array(suffix)] }, [
      ['[]', false], ['["az"]', false], ['["az","abz"]', false],
      ['["a","az"]', true], ['["az","z"]', true], ['["a","z"]', false],
      ['["a","a","az","a"]', true], ['["a","b"]', false], ['["z",null]', false],
      ['["\\u0061z","a"]', true], ['["az","\\u007a"]', true], ['["a",]', false],
    ]],
    [{ type: 'object', required: ['detail'], properties: { detail: { pattern: '^[a-z]{2,5}$' } },
      $defs: { prefix }, oneOf: [object({ $ref: '#/$defs/prefix', pattern: 'b' }), object(suffix)] }, [
      ['{}', false], ['{"detail":"ab"}', true], ['{"detail":"az"}', true],
      ['{"detail":"abz"}', false], ['{"detail":"ax"}', false], ['{"detail":"ab123"}', false],
      ['{"detail":"z"}', false], ['{"det\\u0061il":"\\u0062z"}', true],
      ['{"detail":"axxxbz"}', false], ['{"detail":17}', false],
      ['{"detail":"ab","extra":0}', false],
    ]],
    [{ type: 'array', oneOf: [array({ const: 'az' }, { maxItems: 2 }), array(suffix, { minItems: 1 })] }, [
      ['[]', true], ['["az"]', false], ['["az","az"]', false],
      ['["az","az","az"]', true], ['["az","z"]', true], ['["z","z","az"]', true],
      ['["az","a"]', false], ['["\\u0061z"]', false], ['["x"]', false],
    ]],
    [{ type: 'object', oneOf: [object(prefix), object(suffix), object({ const: 'az' })] }, [
      ['{}', false], ['{"detail":"a"}', true], ['{"detail":"z"}', true],
      ['{"detail":"az"}', false], ['{"detail":"abz"}', false], ['{"detail":"b"}', false],
      ['{"detail":"Az"}', true], ['{"detail":true}', false],
    ]],
    [{ type: 'array', oneOf: [
      array({ ...prefix, type: ['string', 'boolean'] }), array({ ...suffix, type: ['string', 'null'] }),
    ] }, [
      ['[]', false], ['["az"]', false], ['[true,"az"]', true], ['["az",null]', true], ['[false,null]', false],
      ['["a",true,"az",false]', true], ['["z",true]', false], ['[0,"a"]', false],
    ]],
    [{ type: 'array', oneOf: [array({ pattern: '^a' }), array({ pattern: 'z$' })] }, [
      ['[1,{}]', false], ['[1.0e+1,{"x":[null]},"a"]', true], ['["z",false,[null]]', true],
      ['["a",1,"z"]', false], ['["az",[]]', false], ['["b"]', false],
    ]],
    [{ type: 'object', oneOf: [
      object({ type: 'string', format: 'date' }), object({ type: 'string', pattern: '^2024-' }),
    ] }, [
      ['{}', false], ['{"detail":"2024-02-29"}', false], ['{"detail":"2024-02-31"}', true],
      ['{"detail":"2030-02-28"}', true], ['{"detail":"2030-02-31"}', false],
      ['{"detail":"\\u0032024-02-31"}', true],
    ]],
    [{ type: 'object', oneOf: [
      { properties: { detail: prefix, state: suffix }, additionalProperties: false },
      { properties: { detail: suffix, state: prefix }, additionalProperties: false },
    ] }, [
      ['{}', false], ['{"detail":"az","state":"az"}', false], ['{"detail":"a","state":"a"}', false],
      ['{"detail":"a","state":"z"}', true], ['{"state":"a","detail":"z"}', true],
      ['{"detail":"az","state":"a"}', true], ['{"state":"az"}', false], ['{"state":"z"}', true],
    ]],
  ],
  ungeneratable: [
    [{ type: 'array', oneOf: [array(prefix), array({ type: 'string', pattern: '^[a]' })] }, [
      ['[]', false], ['["a"]', false], ['["ab","a"]', false], ['["z"]', false],
    ]],
  ],
};

if (process.argv.includes('--write')) {
  writeProjectData(new URL('../../parity/fixtures/json-schema-exclusive-member-patterns.lino', import.meta.url), fixtures);
} else {
  for (const [schema, cases] of fixtures.grammars) {
    for (const [text, expected] of cases) assert.equal(validateJson(text, schema).valid, expected, text);
  }
  for (const [schema, cases] of fixtures.ungeneratable) {
    for (const [text, expected] of cases) assert.equal(validateJson(text, schema).valid, expected, text);
  }
  for (const [schema, cases] of fixtures.grammars) {
    const grammar = importJsonSchema(schema);
    const parser = compileGrammar(grammar);
    for (const [text, expected] of cases) assert.equal(parser.parseTree(text).ok, expected, text);
    assert.deepEqual(grammar.schemaDiagnostics, []);
  }
  for (const [schema] of fixtures.ungeneratable) {
    assert.throws(() => importJsonSchema(schema), /no generatable document/u);
  }
  console.log('Exclusive collection patterns agree with document validation on all shared cases.');
}
