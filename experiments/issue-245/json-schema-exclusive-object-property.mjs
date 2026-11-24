import assert from 'node:assert/strict';
import { writeProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar, importJsonSchema, validateJson } from '../../js/src/index.js';

function branch(mode, required = ['mode', 'evidence'], additionalProperties = false) {
  return {
    type: 'object',
    properties: { mode, evidence: { type: 'string' } },
    required,
    additionalProperties,
  };
}

const common = branch({ const: 'shared' });
const fixtures = {
  grammars: [
    [{ oneOf: [branch({ enum: ['first', 'shared'] }), branch({ enum: ['shared', 'second'] })] }, [
      ['{"mode":"first","evidence":"original text"}', true],
      ['{"evidence":"original text","mode":"second"}', true],
      ['{"mode":"shared","evidence":"original text"}', false],
      ['{"mo\\u0064e":"\\u0073econd","evidence":"caf\u00e9"}', true],
      ['{"mode":"\\u0073hared","evidence":"original text"}', false],
      ['{"mode":"first"}', false],
      ['{"mode":"second","evidence":true}', false],
      ['{"mode":"first","evidence":"text","extra":true}', false],
    ]],
    [{ oneOf: [
      branch({ type: 'boolean' }, ['evidence'], { type: 'number' }),
      branch({ enum: [false, null] }, ['evidence'], { type: 'number' }),
    ] }, [
      ['{"evidence":"text"}', false],
      ['{"evidence":"text","mode":false}', false],
      ['{"mode":true,"page":2,"evidence":"text"}', true],
      ['{"evidence":"text","mode":null}', true],
      ['{"mode":null,"evidence":"text","page":true}', false],
    ]],
    [{
      $defs: {
        first: branch({ $ref: '#/$defs/firstModes' }),
        firstModes: { enum: ['first', 'shared'], pattern: '^f' },
      },
      oneOf: [{ $ref: '#/$defs/first' }, branch({ enum: ['shared', 'second'] }, ['evidence', 'mode'])],
    }, [
      ['{"evidence":"text","mode":"first"}', true],
      ['{"evidence":"text","mode":"second"}', true],
      ['{"evidence":"text","mode":"shared"}', true],
      ['{"evidence":"text","mode":"third"}', false],
    ]],
  ],
  ungeneratable: [
    [{ oneOf: [common, { ...common, title: 'same data' }] }, [
      ['{"mode":"shared","evidence":"text"}', false],
      ['{"mode":"external","evidence":"text"}', false],
    ]],
    [{ oneOf: [common, branch({ enum: ['shared'] })] }, [
      ['{"mode":"shared","evidence":"text"}', false],
    ]],
  ],
};

if (process.argv.includes('--write')) {
  writeProjectData(new URL('../../parity/fixtures/json-schema-exclusive-object-property.lino', import.meta.url), fixtures);
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
  console.log('Exclusive object alternatives passed for shared, optional and referenced properties.');
}
