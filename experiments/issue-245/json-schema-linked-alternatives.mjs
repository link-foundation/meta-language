import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar, importJsonSchema, validateJson } from '../../js/src/index.js';

const stringDefinitions = {
  identifier: { type: 'string', pattern: '^a' },
  address: { $ref: '#/$defs/identifier', pattern: 'z$' },
  narrow: { $ref: '#/$defs/address', pattern: '^ab' },
};
const tag = (kind, valueType) => ({
  type: 'object', required: ['kind', 'value'], additionalProperties: false,
  properties: { kind: { const: kind }, value: { type: valueType } },
});
const cases = [
  [
    { $defs: stringDefinitions, $ref: '#/$defs/address', pattern: '^ab' },
    [['"abz"', true], ['"ab\\u007a"', true], ['"axz"', false], ['"abx"', false], ['"xaz"', false], ['true', false]],
    [],
  ],
  [
    { $defs: stringDefinitions, oneOf: [{ $ref: '#/$defs/narrow' }, { type: 'string', pattern: 'z$' }] },
    [['"xxz"', true], ['"axz"', true], ['"az"', true], ['"abz"', false], ['"ab\\u007a"', false], ['"abx"', false], ['null', false]],
    [],
  ],
  [
    { $defs: stringDefinitions, pattern: 'b', anyOf: [{ $ref: '#/$defs/narrow' }, { type: 'string', pattern: '^x' }] },
    [['"abz"', true], ['"xb"', true], ['"xz"', false], ['"az"', false], ['"abx"', false], ['42', false]],
    [],
  ],
  [
    {
      $defs: { item: tag('item', 'string'), group: tag('group', 'boolean'), alias: { $ref: '#/$defs/group' } },
      oneOf: [{ $ref: '#/$defs/item' }, { $ref: '#/$defs/alias' }],
    },
    [
      ['{"kind":"item","value":"answer"}', true], ['{"value":false,"k\\u0069nd":"group"}', true],
      ['{"kind":"item","value":true}', false], ['{"kind":"group","value":"answer"}', false],
      ['{"kind":"other","value":false}', false], ['{"kind":"group"}', false],
    ],
    [],
  ],
  [
    { $defs: { item: tag('item', 'string'), alias: { $ref: '#/$defs/item' } }, oneOf: [{ $ref: '#/$defs/item' }, { $ref: '#/$defs/alias' }] },
    [['{"kind":"item","value":"answer"}', true], ['{"kind":"item","value":true}', false]],
    ['/oneOf'],
    [['{"kind":"item","value":"answer"}', false], ['{"kind":"item","value":true}', false]],
  ],
];
const impossible = { $defs: stringDefinitions, $ref: '#/$defs/address', pattern: '^b' };

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) {
    const file = new URL('../../parity/fixtures/json-schema-and-recovery.lino', import.meta.url);
    const data = readProjectData(file);
    if (!data.grammars.some(([schema]) => JSON.stringify(schema) === JSON.stringify(cases[0][0]))) {
      data.grammars.push(...cases);
      data.ungeneratableSchemas.push(impossible);
      writeProjectData(file, data);
    }
  } else {
    for (const [schema, expected, diagnostics, validation = expected] of cases) {
      const grammar = importJsonSchema(schema);
      const parser = compileGrammar(grammar);
      for (const [text, valid] of expected) assert.equal(parser.parseTree(text).ok, valid, 'decoder: ' + text);
      for (const [text, valid] of validation) assert.equal(validateJson(text, schema).valid, valid, 'validator: ' + text);
      assert.deepEqual(grammar.schemaDiagnostics.map(issue => issue.pointer), diagnostics);
    }
    assert.throws(() => importJsonSchema(impossible), /no generatable document/u);
  }
}
