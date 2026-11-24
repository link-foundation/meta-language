import assert from 'node:assert/strict';
import { writeProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar, importJsonSchema, validateJson } from '../../js/src/index.js';

const colors = {
  type: 'array', oneOf: [
    { items: { enum: ['red', 'blue'] } },
    { items: { enum: ['blue', 'green'] } },
  ],
};
const fixtures = {
  grammars: [
    [colors, [
      ['["blue"]', false],
      ['[]', false],
      ['["blue","blue"]', false],
      ['["red","green"]', false],
      ['["green","red"]', false],
      ['["red"]', true],
      ['["blue","green"]', true],
      ['["red", "bl\\u0075e", "red", "blue", "blue"]', true],
      ['["red",false]', false],
      ['["blue","green",]', false],
    ]],
    [{ type: 'array', oneOf: [
      { prefixItems: [{ enum: ['red', 'blue'] }], items: { type: 'string' }, minItems: 2 },
      { prefixItems: [{ enum: ['blue', 'green'] }], items: { type: 'string' }, maxItems: 3 },
    ] }, [
      ['[]', true],
      ['["blue"]', true],
      ['["red"]', false],
      ['["red","text"]', true],
      ['["blue","text"]', false],
      ['["blue","text","text"]', false],
      ['["blue","text","text","text"]', true],
      ['["green","text","text","text"]', false],
      ['["blue","text",false,"text"]', false],
      ['["black","text"]', false],
    ]],
    [{ type: 'array', minItems: 1, items: { type: 'string', pattern: '^(red|blue|green)$' }, $defs: {
      warm: { items: { enum: ['red', 'blue', 'orange'] } },
      cool: { items: { enum: ['green', 'blue'] } },
    }, oneOf: [
      { $ref: '#/$defs/warm' },
      { $ref: '#/$defs/cool' },
      { items: { enum: ['red', 'green'] }, maxItems: 2 },
    ] }, [
      ['[]', false],
      ['["red"]', false],
      ['["blue"]', false],
      ['["green","green"]', false],
      ['["orange","blue"]', false],
      ['["blue","green"]', true],
      ['["red","green"]', true],
      ['["red","green","red"]', false],
      ['["green","green","green"]', true],
      ['["blue","blue","\\u0072ed","red"]', true],
    ]],
    [{ type: 'array', oneOf: [
      { prefixItems: [{ enum: ['red', 'blue'] }], items: false },
      { prefixItems: [{ enum: ['blue', 'green'] }, false], items: { type: 'boolean' } },
      { prefixItems: [{ enum: ['blue', 'green'] }], items: { type: 'boolean' }, minItems: 2 },
    ] }, [
      ['[]', false],
      ['["blue"]', false],
      ['["red"]', true],
      ['["green"]', true],
      ['["blue",false,true]', true],
      ['["green",true]', true],
      ['["green",null]', false],
      ['["red",true]', false],
    ]],
    [{ type: 'array', items: { enum: ['a', 'ab', 'b'] }, oneOf: [
      { items: { type: 'string', pattern: '^a' } },
      { items: { type: 'string', pattern: 'b$' } },
    ] }, [
      ['[]', false],
      ['["ab"]', false],
      ['["a","b"]', false],
      ['["a"]', true],
      ['["b"]', true],
      ['["ab","b","ab"]', true],
      ['["a","ax"]', false],
    ]],
  ],
  ungeneratable: [[{ type: 'array', oneOf: [
    { items: { enum: ['red', 'blue'] } },
    { items: { enum: ['blue', 'red'] } },
  ] }, [
    ['[]', false],
    ['["red","blue"]', false],
    ['["blue"]', false],
    ['["green"]', false],
  ]]],
};

if (process.argv.includes('--write')) {
  writeProjectData(new URL('../../parity/fixtures/json-schema-exclusive-array-items.lino', import.meta.url), fixtures);
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
    for (const [source, accepted] of cases) {
      assert.equal(validateJson(source, schema).valid, accepted, `validation of ${source}`);
    }
    assert.throws(() => importJsonSchema(schema), /no generatable document/u);
  }
  console.log('Exclusive array alternatives count matching branches across member values and lengths.');
}
