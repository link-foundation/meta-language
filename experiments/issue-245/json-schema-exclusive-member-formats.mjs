import assert from 'node:assert/strict';
import { writeProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar, importJsonSchema, validateJson } from '../../js/src/index.js';

const object = amount => ({ properties: { amount }, additionalProperties: false });
const property = spelling => `{"amount":${spelling}}`;
const label = 'b'.repeat(63);
const fixtures = {
  grammars: [
    [{
      type: 'object',
      required: ['amount'],
      properties: { amount: { maximum: 18, pattern: 'z$' } },
      $defs: { left: { type: ['number', 'string', 'boolean'], minimum: 0 } },
      oneOf: [
        object({ $ref: '#/$defs/left', exclusiveMaximum: 16, pattern: '^a' }),
        object({ type: ['integer', 'string', 'null'], minimum: 3, maximum: 20, format: 'email' }),
      ],
    }, [
      ['0', true], ['2.9', true], ['3', false], ['3.0', false], ['3.25', true], ['15', false],
      ['16', true], ['18', true], ['19', false], ['18.1', false], ['-1', false],
      ['"abz"', true], ['"a@b.cz"', false], ['"b@b.cz"', true], ['"a@b.no"', false],
      ['"zzz"', false], ['"a\\u0062z"', true], ['"b\\u0040b.c\\u007a"', true],
      ['"a😀z"', true], ['"a\\ud83d\\ude00z"', true], ['"😀z"', false],
      ['"\\u0022b\\u0022@b.cz"', true], ['"b..c@b.cz"', false],
      [`"b@${label}.cz"`, true], [`"b@${label}b.cz"`, false],
      ['true', true], ['null', true],
    ].map(([spelling, accepted]) => [property(spelling), accepted]).concat([
      ['{}', false], ['{"amount":null,"extra":0}', false],
    ]), ['/properties/amount/type'], [[property('18e0'), true]]],
    [{
      type: 'array',
      oneOf: [
        { items: { type: 'string', pattern: '^b' } },
        { items: { type: 'string', format: 'email' } },
      ],
    }, [
      ['[]', false], ['["b"]', true], ['["b@x.org"]', false], ['["a@x.org"]', true],
      ['["b@x.org","b"]', true], ['["b@x.org","a@x.org"]', true],
      ['["b","a@x.org"]', false], ['["a..b@x.org"]', false], ['["b\\u0040x.org"]', false],
    ]],
  ],
  ungeneratable: [],
};

if (process.argv.includes('--write')) {
  writeProjectData(new URL('../../parity/fixtures/json-schema-exclusive-member-formats.lino', import.meta.url), fixtures);
} else {
  for (const [schema, cases, pointers = [], validationOnly = []] of fixtures.grammars) {
    for (const [source, accepted] of [...cases, ...validationOnly]) assert.equal(validateJson(source, schema).valid, accepted, source);
    const grammar = importJsonSchema(schema);
    const parser = compileGrammar(grammar);
    for (const [source, accepted] of cases) assert.equal(parser.parseTree(source).ok, accepted, source);
    assert.deepEqual(grammar.schemaDiagnostics.map(issue => issue.pointer), pointers);
  }
  console.log('Exclusive collection formats and patterns agree with validation across all shared cases.');
}
