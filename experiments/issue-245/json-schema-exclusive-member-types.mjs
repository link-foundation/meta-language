import assert from 'node:assert/strict';
import { writeProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar, importJsonSchema, validateJson } from '../../js/src/index.js';

const booleanOrString = { type: ['string', 'boolean'] };
const nullOrString = { type: ['string', 'null'] };
const array = (items, constraints = {}) => ({ items, ...constraints });
const object = (detail, constraints = {}) => ({ properties: { detail }, additionalProperties: false, ...constraints });
const fixtures = {
  grammars: [
    [{ type: 'array', oneOf: [array(booleanOrString), array(nullOrString)] }, [
      ['[]', false], ['["shared"]', false], ['["shared","other"]', false],
      ['["shared",true]', true], ['[null,"shared"]', true], ['[true,null]', false],
      ['[true, true,"shared", false]', true], ['[123,"shared"]', false],
      ['["\\u0073hared",null]', true], ['[true,]', false],
    ]],
    [{ type: 'object', oneOf: [object(booleanOrString), object(nullOrString)] }, [
      ['{}', false], ['{"detail":"shared"}', false], ['{"detail":true}', true],
      ['{"detail":null}', true], ['{"detail":17}', false],
      ['{"det\\u0061il":false}', true], ['{"detail":null,"extra":0}', false],
    ]],
    [{ type: 'array', $defs: { input: { type: ['object', 'array', 'number', 'string'] } },
      items: { type: ['object', 'array', 'number', 'string'] }, oneOf: [
        array({ $ref: '#/$defs/input', type: ['array', 'number', 'boolean'] }, { minItems: 1 }),
        array({ $ref: '#/$defs/input', type: ['object', 'number', 'string'] }, { maxItems: 2 }),
      ] }, [
      ['[]', true], ['[1]', false], ['[1.0e+1,2.3]', false], ['[3e2,1,0]', true],
      ['[[-1.2,{"x":true}],1e3]', true], ['[{"x":[null,{}]},1e3]', true],
      ['[[],{}]', false], ['["other"]', true], ['[false]', false], ['[null]', false],
      ['[{"x":1},0,"other"]', false],
    ]],
    [{ type: 'object', required: ['detail'], properties: { detail: { type: ['boolean', 'number', 'null'] } },
      $defs: { detail: { type: ['boolean', 'number', 'string'] } }, oneOf: [
        object({ $ref: '#/$defs/detail' }), object({ type: ['null', 'number', 'string'] }),
      ] }, [
      ['{}', false], ['{"detail":false}', true], ['{"detail":null}', true],
      ['{"detail":1.0}', false], ['{"detail":"only string"}', false], ['{"detail":[]}', false],
    ]],
    [{ type: 'object', oneOf: [
      { properties: { detail: booleanOrString, state: nullOrString }, additionalProperties: false },
      { properties: { detail: nullOrString, state: booleanOrString }, additionalProperties: false },
    ] }, [
      ['{}', false], ['{"detail":"shared","state":"shared"}', false],
      ['{"detail":true,"state":false}', false], ['{"detail":null,"state":null}', false],
      ['{"detail":true,"state":null}', true], ['{"state":true,"detail":null}', true],
      ['{"detail":"shared","state":null}', true], ['{"detail":null}', true],
      ['{"state":"shared"}', false], ['{"detail":true,"extra":null}', false],
    ]],
  ],
  ungeneratable: [
    [{ type: 'array', oneOf: [array(booleanOrString), array({ type: ['boolean', 'string'] })] }, [
      ['[]', false], ['[true]', false], ['["text",false]', false], ['[null]', false],
    ]],
    [{ type: 'object', oneOf: [object(nullOrString), object({ type: ['null', 'string'] })] }, [
      ['{}', false], ['{"detail":null}', false], ['{"detail":"text"}', false], ['{"detail":true}', false],
    ]],
  ],
};

if (process.argv.includes('--write')) {
  writeProjectData(new URL('../../parity/fixtures/json-schema-exclusive-member-types.lino', import.meta.url), fixtures);
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
    for (const [source, accepted] of cases) assert.equal(validateJson(source, schema).valid, accepted, `validation of ${source}`);
    assert.throws(() => importJsonSchema(schema), /no generatable document/u);
  }
  console.log('Exclusive collection alternatives preserve matching JSON instance types.');
}
