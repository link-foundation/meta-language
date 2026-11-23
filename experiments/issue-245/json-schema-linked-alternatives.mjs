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
const numericDefinitions = {
  number: { type: 'number', minimum: -3, maximum: 9 },
  restricted: { $ref: '#/$defs/number', exclusiveMinimum: -1, maximum: 5 },
};
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
  [
    { $defs: numericDefinitions, $ref: '#/$defs/restricted', type: 'integer', maximum: 3, exclusiveMaximum: 4 },
    [['0', true], ['-0', true], ['3', true], ['-1', false], ['4', false], ['6', false], ['1.0', false], ['3e0', false]],
    ['/type'],
    [['0', true], ['3', true], ['-1', false], ['4', false], ['6', false], ['1.0', true], ['3e0', true]],
  ],
  [
    { $defs: numericDefinitions, $ref: '#/$defs/restricted', exclusiveMaximum: 2.5 },
    [['-0.9', true], ['0', true], ['2.4999', true], ['-1', false], ['2.5', false], ['5', false], ['1e0', false]],
    ['/type'],
    [['-0.9', true], ['2.4999', true], ['-1', false], ['2.5', false], ['5', false], ['1e0', true]],
  ],
  [
    { $defs: numericDefinitions, type: 'number', exclusiveMinimum: 0, maximum: 4, anyOf: [
      { $ref: '#/$defs/restricted', maximum: 2 }, { type: 'number', exclusiveMinimum: 3 }, false,
    ] },
    [['1.5', true], ['2', true], ['3.5', true], ['4', true], ['-0', false], ['2.5', false], ['3', false], ['4.1', false]],
    ['/anyOf/0/type', '/anyOf/1/type'],
    [['1.5', true], ['2', true], ['3.5', true], ['4', true], ['-0', false], ['2.5', false], ['3', false], ['4.1', false]],
  ],
  [
    { $defs: numericDefinitions, type: 'integer', maximum: 6, oneOf: [
      { $ref: '#/$defs/restricted' }, { type: 'number', minimum: 3, maximum: 9 }, false,
    ] },
    [['0', true], ['2', true], ['6', true], ['-1', false], ['3', false], ['5', false], ['7', false], ['6.0', false]],
    ['/oneOf'],
    [['0', true], ['2', true], ['6', true], ['-1', false], ['3', false], ['5', false], ['7', false], ['6.0', true]],
  ],
];
const impossible = [
  { $defs: stringDefinitions, $ref: '#/$defs/address', pattern: '^b' },
  { $defs: numericDefinitions, $ref: '#/$defs/restricted', type: 'integer', maximum: -1 },
  { type: 'integer', minimum: 5, anyOf: [{ maximum: 3 }, { exclusiveMaximum: 5 }] },
];

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) {
    const file = new URL('../../parity/fixtures/json-schema-and-recovery.lino', import.meta.url);
    const data = readProjectData(file);
    const grammars = new Set(data.grammars.map(([schema]) => JSON.stringify(schema)));
    const empty = new Set(data.ungeneratableSchemas.map(schema => JSON.stringify(schema)));
    const newGrammars = cases.filter(([schema]) => !grammars.has(JSON.stringify(schema)));
    const newEmpty = impossible.filter(schema => !empty.has(JSON.stringify(schema)));
    if (newGrammars.length || newEmpty.length) {
      data.grammars.push(...newGrammars);
      data.ungeneratableSchemas.push(...newEmpty);
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
    for (const schema of impossible) assert.throws(() => importJsonSchema(schema), /no generatable document/u);
  }
}
