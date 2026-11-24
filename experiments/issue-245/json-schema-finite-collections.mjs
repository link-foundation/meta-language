import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar } from '../../js/src/grammar.js';
import { importJsonSchema } from '../../js/src/grammar-importers/json-schema.js';
import { validateJson } from '../../js/src/json-schema-validation.js';

const mode = values => ({ enum: values });
const boolean = { type: 'boolean' };
const selection = (modes, required) => ({
  type: 'object', properties: { mode: mode(modes), verified: boolean },
  required, additionalProperties: false,
});
const fixedArray = (...prefixItems) => ({ type: 'array', prefixItems, items: false });
const selectors = [
  fixedArray(boolean, { type: 'null' }),
  fixedArray({ enum: [true, null] }, { const: null }),
];
const cases = [
  [{
    type: 'object',
    $defs: { first: selection(['first', 'shared'], ['mode']) },
    oneOf: [{ $ref: '#/$defs/first' }, selection(['shared', 'second'], ['mode', 'verified'])],
  }, [
    ['{"mode":"first"}', true], ['{"verified":false,"mode":"first"}', true],
    ['{"mode":"shared"}', true], ['{"mode":"second","verified":true}', true],
    ['{"mode":"shared","verified":false}', false], ['{"mode":"second"}', false],
    ['{"mode":"first","extra":true}', false], ['{}', false],
  ]],
  [{
    type: 'array', minItems: 1, oneOf: selectors,
  }, [
    ['[]', false], ['[false]', true], ['[true]', false], ['[null]', true],
    ['[false,null]', true], ['[true,null]', false], ['[null,null]', true],
    ['[null,false]', false], ['[false,null,null]', false],
  ]],
  [{
    type: 'array', anyOf: selectors,
    oneOf: [{ ...fixedArray({ const: false }), minItems: 1 }, { type: 'array', minItems: 2 }],
  }, [
    ['[]', false], ['[false]', true], ['[true]', false], ['[null]', false],
    ['[false,null]', true], ['[true,null]', true], ['[null,null]', true],
    ['[null,false]', false], ['[false,null,null]', false],
  ]],
  [{
    type: 'object', additionalProperties: false, required: ['__proto__'],
    properties: { ['__proto__']: { type: 'array' } },
    oneOf: [
      { type: 'object', additionalProperties: false, properties: { ['__proto__']: selectors[0] } },
      { type: 'object', additionalProperties: false, properties: { ['__proto__']: selectors[1] } },
    ],
  }, [
    ['{"__proto__":[false]}', true], ['{"__proto__":[null,null]}', true],
    ['{"\\u005f_proto__":[false]}', true], ['{"__proto__":[true]}', false],
    ['{"__proto__":[]}', false], ['{}', false],
  ]],
];
const impossible = { oneOf: [selection(['same'], ['mode']), selection(['same'], ['mode'])] };

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) {
    const file = new URL('../../parity/fixtures/json-schema-and-recovery.lino', import.meta.url);
    const data = readProjectData(file);
    for (const [schema, inputs] of cases) {
      assert.ok(data.grammars.every(([existing]) => JSON.stringify(existing) !== JSON.stringify(schema)));
      data.grammars.push([schema, inputs, []]);
    }
    assert.ok(data.ungeneratableSchemas.every(schema => JSON.stringify(schema) !== JSON.stringify(impossible)));
    data.ungeneratableSchemas.push(impossible);
    writeProjectData(file, data);
  } else {
    for (const [index, [schema, inputs]] of cases.entries()) {
      const grammar = importJsonSchema(schema);
      const parser = compileGrammar(grammar);
      for (const [source, accepted] of inputs) {
        assert.equal(validateJson(source, schema).valid, accepted, `schema ${index} validation ${source}`);
        assert.equal(parser.parseTree(source).ok, accepted, `schema ${index} decoder ${source}`);
      }
      assert.deepEqual(grammar.schemaDiagnostics, []);
    }
    assert.throws(() => importJsonSchema(impossible), /no generatable document/u);
    console.log(`${cases.length} finite collection schemas and one empty exclusive language passed`);
  }
}
