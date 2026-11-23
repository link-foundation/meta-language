import assert from 'node:assert/strict';
import { writeProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar, importJsonSchema, validateJson } from '../../js/src/index.js';

function branch(modes, regions, required = []) {
  return {
    type: 'object',
    properties: {
      mode: { enum: modes },
      region: { enum: regions },
      evidence: { type: 'string' },
    },
    required,
  };
}

const first = branch(['recorded', 'shared'], ['north', 'central']);
const second = branch(['shared', 'planned'], ['central', 'south']);
const fixtures = {
  grammars: [
    [{ type: 'object', oneOf: [first, second] }, [
      ['{"mode":"shared","region":"central","evidence":"quote"}', false],
      ['{"region":"central","mode":"recorded","evidence":"quote"}', true],
      ['{"evidence":"quote","mode":"shared","region":"north"}', true],
      ['{"evidence":"quote","region":"south","mode":"shared"}', true],
      ['{"mode":"recorded","region":"south","evidence":"quote"}', false],
      ['{"mode":"shared","evidence":"quote"}', false],
      ['{"mode":"recorded","evidence":"quote"}', true],
      ['{"region":"south","evidence":"quote"}', true],
      ['{"region":"central","evidence":"quote"}', false],
      ['{"evidence":"quote"}', false],
      ['{"region":"north","mo\\u0064e":"\\u0073hared","evidence":"quote","extra":1}', true],
      ['{"mode":"recorded","evidence":false}', false],
    ]],
    [{
      $defs: { first: branch(['recorded', 'shared'], ['north', 'central'], ['mode']) },
      oneOf: [{ $ref: '#/$defs/first' }, branch(['shared', 'planned'], ['central', 'south'], ['mode'])],
      properties: {
        mode: { enum: ['shared', 'recorded'] },
        region: { enum: ['central', 'south'] },
        evidence: { type: 'string', pattern: '^quote:' },
      },
      required: ['region', 'evidence'],
      additionalProperties: false,
    }, [
      ['{"mode":"shared","region":"south","evidence":"quote: text"}', true],
      ['{"region":"central","evidence":"quote: text","mode":"recorded"}', true],
      ['{"mode":"shared","region":"central","evidence":"quote: text"}', false],
      ['{"mode":"shared","region":"north","evidence":"quote: text"}', false],
      ['{"mode":"planned","region":"south","evidence":"quote: text"}', false],
      ['{"mode":"recorded","region":"central","evidence":"text"}', false],
      ['{"region":"south","evidence":"quote: text"}', false],
      ['{"mode":"recorded","evidence":"quote: text"}', false],
      ['{"mode":"shared","region":"south","evidence":"quote: text","extra":1}', false],
    ]],
    [{ type: 'object', oneOf: [first, second, branch(['shared', 'archived'], ['north', 'south'])] }, [
      ['{"mode":"recorded","region":"north","evidence":"quote"}', true],
      ['{"region":"south","mode":"archived","evidence":"quote"}', true],
      ['{"mode":"shared","region":"north","evidence":"quote"}', false],
      ['{"mode":"shared","region":"central","evidence":"quote"}', false],
      ['{"mode":"shared","region":"south","evidence":"quote"}', false],
      ['{"mode":"shared","evidence":"quote"}', false],
      ['{"mode":"archived","evidence":"quote"}', true],
    ]],
  ],
  ungeneratable: [
    [{
      type: 'object', oneOf: [first, second],
      properties: { mode: { const: 'shared' }, region: { const: 'central' } },
      required: ['mode', 'region'],
    }, [
      ['{"mode":"shared","region":"central","evidence":"quote"}', false],
      ['{"mode":"recorded","region":"central","evidence":"quote"}', false],
      ['{"mode":"shared","region":"south","evidence":"quote"}', false],
      ['{"mode":"shared","evidence":"quote"}', false],
    ]],
  ],
};

if (process.argv.includes('--write')) {
  writeProjectData(new URL('../../parity/fixtures/json-schema-exclusive-object-fields.lino', import.meta.url), fixtures);
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
  console.log('Exclusive object alternatives compare branch membership across each finite declared field.');
}
