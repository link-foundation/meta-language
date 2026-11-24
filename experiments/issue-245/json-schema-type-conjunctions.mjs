import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar, importJsonSchema, validateJson } from '../../js/src/index.js';

const examples = [
  [{
    type: 'object', additionalProperties: false, required: ['text', 'either', 'whole'],
    $defs: { scalar: { type: ['string', 'number'] } },
    properties: {
      text: { $ref: '#/$defs/scalar', type: 'string' },
      either: { $ref: '#/$defs/scalar' },
      whole: { $ref: '#/$defs/scalar', type: 'integer' },
    },
  }, [
    ['{"text":"é","either":1.5,"whole":-2}', true],
    ['{"whole":0,"either":"open","text":"x"}', true],
    ['{"text":2,"either":"open","whole":1}', false],
    ['{"text":"x","either":null,"whole":1}', false],
    ['{"text":"x","either":"open","whole":"1"}', false],
    ['{"text":"x","either":"open","whole":1.2}', false],
  ], ['/$defs/scalar/type'], [['{"text":"x","either":2,"whole":2.0}', true]]],
  [{
    $defs: { branch: { anyOf: [{ type: 'string' }, {
      type: 'object', additionalProperties: false, required: ['next'],
      properties: { next: { $ref: '#/$defs/branch' } },
    }] } },
    $ref: '#/$defs/branch', type: 'object',
  }, [
    ['{"next":"leaf"}', true], ['{"next":{"next":"leaf"}}', true],
    ['"leaf"', false], ['{"next":1}', false], ['{}', false],
  ], []],
  [{
    type: ['array', 'null'],
    anyOf: [{ type: 'string' }, { type: 'array', items: { type: 'boolean' } }, true],
  }, [['[]', true], ['[false,2]', true], ['null', true], ['"leaf"', false], ['{}', false]], []],
  [{
    type: 'array',
    oneOf: [{ type: 'string', pattern: '^a' }, { type: 'string', pattern: 'a$' }, { type: 'array', items: { type: 'boolean' } }],
  }, [['[true,false]', true], ['[]', true], ['[null]', false], ['"aba"', false], ['"abc"', false]], []],
  [{ type: 'string', anyOf: [{ type: 'string' }, { const: false }, { const: 'yes' }] },
    [['"hi"', true], ['"yes"', true], ['false', false]], []],
];
const impossible = { $defs: { text: { type: 'string' } }, $ref: '#/$defs/text', type: 'integer' };

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) {
    const file = new URL('../../parity/fixtures/json-schema-and-recovery.lino', import.meta.url);
    const data = readProjectData(file);
    if (!data.grammars.some(([schema]) => JSON.stringify(schema) === JSON.stringify(examples[0][0]))) {
      data.grammars.push(...examples);
      data.ungeneratableSchemas.push(impossible);
      writeProjectData(file, data);
    }
  } else {
    for (const [schema, cases, diagnostics, validation] of examples) {
      const imported = importJsonSchema(schema);
      const parser = compileGrammar(imported);
      for (const [source, accepted] of cases) {
        assert.equal(validateJson(source, schema).valid, accepted, source);
        assert.equal(parser.parseTree(source).ok, accepted, source);
      }
      for (const [source, accepted] of validation ?? []) assert.equal(validateJson(source, schema).valid, accepted, source);
      assert.deepEqual(imported.schemaDiagnostics.map(issue => issue.pointer), diagnostics);
    }
    assert.throws(() => importJsonSchema(impossible), /no generatable document/u);
  }
}
