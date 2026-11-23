import assert from 'node:assert/strict';
import { writeProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar, importJsonSchema, validateJson } from '../../js/src/index.js';

function branch(modes, additionalProperties = true) {
  return {
    type: 'object',
    properties: { mode: { enum: modes }, evidence: { type: 'string' } },
    additionalProperties,
  };
}

const alternatives = [branch(['first', 'first-only', 'shared']), branch(['shared', 'second'])];
const fixtures = {
  grammars: [
    [{
      $defs: { first: alternatives[0] },
      type: 'object',
      oneOf: [{ $ref: '#/$defs/first' }, alternatives[1]],
      properties: {
        mode: { enum: ['first', 'second', 'shared'] },
        evidence: { type: 'string', pattern: '^quote:' },
        source: { const: 'record' },
      },
      required: ['evidence', 'source'],
      additionalProperties: false,
    }, [
      ['{"source":"record","mode":"first","evidence":"quote: original text"}', true],
      ['{"evidence":"quote: text","source":"record","mo\\u0064e":"\\u0073econd"}', true],
      ['{"source":"record","mode":"shared","evidence":"quote: text"}', false],
      ['{"source":"record","mode":"first-only","evidence":"quote: text"}', false],
      ['{"source":"record","evidence":"quote: text"}', false],
      ['{"source":"record","mode":"second","evidence":"misquoted"}', false],
      ['{"source":"other","mode":"second","evidence":"quote: text"}', false],
      ['{"mode":"second","evidence":"quote: text"}', false],
      ['{"source":"record","mode":"second","evidence":"quote: text","extra":true}', false],
    ]],
    [{
      oneOf: [branch(['allowed', 'shared', 'blocked'], { type: 'string' }), branch(['shared', 'second'], { type: 'string' })],
      properties: { evidence: { type: 'string' } },
      required: ['evidence'],
      additionalProperties: { type: 'string', pattern: '^[as]' },
    }, [
      ['{"mode":"allowed","agent":"assigned","evidence":"text"}', true],
      ['{"evidence":"text","mode":"second"}', true],
      ['{"evidence":"text","mode":"blocked"}', false],
      ['{"evidence":"text","mode":"shared"}', false],
      ['{"evidence":"text","mode":"allowed","agent":"denied"}', false],
      ['{"mode":"allowed","agent":"assigned"}', false],
    ]],
    [{
      oneOf: [branch([1, 'first']), branch([1, 'second'])],
      properties: { mode: { enum: [1, 'first', 'second'] } },
    }, [
      ['{"mode":"first","evidence":"text"}', true],
      ['{"mode":"second","evidence":"text"}', true],
      ['{"mode":1,"evidence":"text"}', false],
      ['{"mode":1.0,"evidence":"text"}', false],
    ]],
  ],
  ungeneratable: [
    [{ oneOf: alternatives, properties: { mode: { const: 'shared' } } }, [
      ['{"mode":"shared","evidence":"text"}', false],
      ['{"mode":"first","evidence":"text"}', false],
      ['{"evidence":"text"}', false],
    ]],
    [{ oneOf: alternatives, properties: { evidence: true }, additionalProperties: false }, [
      ['{"mode":"second","evidence":"text"}', false],
      ['{"evidence":"text"}', false],
    ]],
  ],
};

if (process.argv.includes('--write')) {
  writeProjectData(new URL('../../parity/fixtures/json-schema-exclusive-object-siblings.lino', import.meta.url), fixtures);
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
  console.log('Exclusive object alternatives applied parent property, required-name and additional-property assertions.');
}
