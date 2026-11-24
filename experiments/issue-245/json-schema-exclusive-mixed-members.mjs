import assert from 'node:assert/strict';
import { writeProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar, importJsonSchema, validateJson } from '../../js/src/index.js';

const array = items => ({ items });
const object = amount => ({ properties: { amount }, additionalProperties: false });
const fixtures = {
  grammars: [
    [{
      type: 'array',
      oneOf: [
        array({ type: ['number', 'string', 'boolean'], minimum: 0, pattern: '^a' }),
        array({ type: ['number', 'string', 'null'], maximum: 10, pattern: 'z$' }),
      ],
    }, [
      ['[]', false], ['[5]', false], ['["az"]', false], ['[15]', true], ['[-1]', true],
      ['["alpha"]', true], ['["z"]', true], ['[0,"z"]', true], ['[15,"z"]', false],
      ['[true,0]', true], ['[null,10]', true], ['[true,null]', false],
      ['["\\u0061z"]', false], ['["\\u0061lpha",0]', true], ['["az",0]', false],
      ['[0,11]', true], ['[-1,"alpha"]', false], ['[11,"alpha","az",true]', true],
      ['[[],"z"]', false], ['[0.02]', false], ['[-0.1]', true],
    ], ['/items/type'], [['[2e1]', true]]],
    [{
      type: 'object',
      required: ['amount'],
      properties: { amount: { maximum: 18, pattern: '^202' } },
      $defs: { left: { type: ['number', 'string', 'boolean'], minimum: 0 } },
      oneOf: [
        object({ $ref: '#/$defs/left', exclusiveMaximum: 16, pattern: '02$' }),
        object({ type: ['integer', 'string', 'null'], minimum: 3, maximum: 20, format: 'date' }),
      ],
    }, [
      ['{"amount":0}', true], ['{"amount":2.9}', true], ['{"amount":3}', false],
      ['{"amount":3.0}', false], ['{"amount":3.25}', true], ['{"amount":15}', false],
      ['{"amount":16}', true], ['{"amount":18}', true], ['{"amount":19}', false],
      ['{"amount":18.1}', false], ['{"amount":-1}', false], ['{"amount":"202abc02"}', true],
      ['{"amount":"2026-01-02"}', false], ['{"amount":"2026-01-03"}', true], ['{"amount":"1999-01-02"}', false],
      ['{"amount":"202z"}', false], ['{"amount":"202\\u0061bc02"}', true],
      ['{"amount":true}', true], ['{"amount":null}', true], ['{}', false], ['{"amount":null,"other":0}', false],
    ], ['/properties/amount/type'], [['{"amount":18e0}', true]]],
  ],
  ungeneratable: [
    [{
      type: 'array',
      oneOf: [
        array({ type: ['number', 'string'], minimum: 0, pattern: '^a' }),
        array({ type: ['number', 'string'], minimum: 0, pattern: '^[a]' }),
      ],
    }, [['[]', false], ['[0]', false], ['[-1]', false], ['["alpha"]', false], ['["z"]', false]]],
  ],
};

if (process.argv.includes('--write')) {
  writeProjectData(new URL('../../parity/fixtures/json-schema-exclusive-mixed-members.lino', import.meta.url), fixtures);
} else {
  for (const [schema, cases, , validationOnly] of fixtures.grammars) {
    for (const [source, accepted] of [...cases, ...validationOnly]) assert.equal(validateJson(source, schema).valid, accepted, source);
  }
  for (const [schema, cases, pointers] of fixtures.grammars) {
    const grammar = importJsonSchema(schema);
    const parser = compileGrammar(grammar);
    console.log(JSON.stringify({kind: schema.type, diagnostics: grammar.schemaDiagnostics, rules: grammar.rules.length}));
    for (const [source, accepted] of cases) assert.equal(parser.parseTree(source).ok, accepted, source);
    assert.deepEqual(grammar.schemaDiagnostics.map(issue => issue.pointer), pointers, JSON.stringify(schema));
  }
  for (const [schema, cases] of fixtures.ungeneratable) {
    for (const [source, accepted] of cases) assert.equal(validateJson(source, schema).valid, accepted, source);
    assert.throws(() => importJsonSchema(schema), /no generatable document/u);
  }
  console.log('Mixed collection scalar alternatives select exactly one branch across all shared cases.');
}
