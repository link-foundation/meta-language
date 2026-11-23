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
const collectionDefinitions = {
  tuple: { type: 'array', prefixItems: [{ type: 'boolean' }], items: { type: 'string' }, maxItems: 4 },
  limited: { $ref: '#/$defs/tuple', minItems: 2, maxItems: 3 },
  record: { type: 'object', properties: { kind: { const: 'item' }, value: { type: 'string' } }, additionalProperties: false },
  identified: { $ref: '#/$defs/record', required: ['kind'] },
};
const closedArrayReference = {
  $ref: '#/$defs/prefix', items: false, $defs: { prefix: { type: 'array', prefixItems: [{ type: 'boolean' }] } },
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
  [
    { $defs: collectionDefinitions, $ref: '#/$defs/limited', minItems: 1, maxItems: 2 },
    [['[true,"a"]', true], ['[false, "b"]', true], ['[]', false], ['[true]', false], ['[true,"a","b"]', false], ['["a","b"]', false], ['[true,1]', false]],
    [],
  ],
  [
    { $defs: collectionDefinitions, $ref: '#/$defs/identified', required: ['value', 'kind'] },
    [['{"kind":"item","value":"a"}', true], ['{"value":"b","k\\u0069nd":"item"}', true], ['{}', false], ['{"kind":"item"}', false],
      ['{"value":"a"}', false], ['{"kind":"item","value":"a","extra":null}', false]],
    [],
  ],
  [
    { type: 'object', required: ['new'], $ref: '#/$defs/open', $defs: { open: { additionalProperties: { type: 'boolean' } } } },
    [['{"new":true}', true], ['{"x":false,"new":true}', true], ['{}', false], ['{"x":true}', false], ['{"new":1}', false]],
    [],
  ],
  [
    closedArrayReference,
    [['[]', true], ['[true]', false], ['[false,0]', false], ['[1]', false]],
    [],
  ],
  [
    { $ref: '#/$defs/prefix', prefixItems: [{ type: 'string' }], $defs: closedArrayReference.$defs },
    [['[]', true], ['[true]', true], ['[false,0]', true], ['[1]', false]],
    ['/$ref'],
    [['[]', true], ['[true]', false], ['[false,0]', false], ['[1]', false]],
  ],
  [
    { type: 'array', minItems: 2, maxItems: 2, $defs: collectionDefinitions, anyOf: [
      { $ref: '#/$defs/tuple' }, { type: 'array', items: { type: 'null' } }, false,
    ] },
    [['[true,"a"]', true], ['[null,null]', true], ['[]', false], ['[true]', false], ['[null]', false],
      ['[true,"a","b"]', false], ['[null,true]', false], ['null', false]],
    [],
  ],
  [
    { type: 'object', required: ['value'], $defs: { item: tag('item', 'string') }, oneOf: [
      { $ref: '#/$defs/item' },
      { type: 'object', required: ['kind'], additionalProperties: false,
        properties: { kind: { const: 'group' }, value: { type: 'boolean' } } },
    ] },
    [['{"kind":"item","value":"a"}', true], ['{"value":false,"kind":"group"}', true],
      ['{"kind":"group"}', false], ['{"kind":"item"}', false], ['{"kind":"group","value":"a"}', false], ['{}', false]],
    [],
  ],
  [
    { type: 'array', items: false, anyOf: [
      { prefixItems: [{ type: 'integer' }], items: { type: 'number' } },
      { items: { type: 'string' } },
    ] },
    [['[]', true], ['[ \t ]', true], ['[1]', false], ['[1,2]', false], ['["a"]', false], ['{}', false]],
    [],
  ],
  [
    { $ref: '#/$defs/prefix', minItems: 1, maxItems: 2,
      $defs: { prefix: { type: 'array', prefixItems: [{ type: 'boolean' }], items: false } } },
    [['[true]', true], ['[false]', true], ['[]', false], ['[true,false]', false], ['[1]', false]],
    [],
  ],
  [
    { $ref: '#/$defs/base', maxItems: 2, $defs: { base: { minItems: 1 } } },
    [['true', true], ['null', true], ['{}', true], ['"text"', true], ['[1]', true], ['[1,2]', true], ['[]', false], ['[1,2,3]', false]],
    [],
  ],
  [
    { $ref: '#/$defs/base', minItems: 1, pattern: 'z$', required: ['value'], maximum: 4,
      $defs: { base: { type: ['array', 'string', 'object', 'number', 'boolean'], maxItems: 2, pattern: '^a', minimum: 1 } } },
    [['true', true], ['null', false], ['"az"', true], ['"ab\\u007a"', true], ['"ax"', false], ['"bz"', false], ['[1]', true],
      ['[]', false], ['[1,2,3]', false], ['{"value":null}', true], ['{}', false], ['1.5', true], ['4', true], ['0', false], ['5', false]],
    ['/type'],
  ],
  [
    { minItems: 1, pattern: 'z$', anyOf: [
      { type: ['array', 'boolean'], maxItems: 2 }, { type: 'string', pattern: '^a' }, false,
    ] },
    [['true', true], ['null', false], ['[1]', true], ['[1,2]', true], ['[]', false], ['[1,2,3]', false],
      ['"az"', true], ['"ax"', false], ['"bz"', false]],
    [],
  ],
  [
    { minItems: 1, required: ['value'], $defs: { listOrFlag: { type: ['array', 'boolean'], maxItems: 2 } }, oneOf: [
      { $ref: '#/$defs/listOrFlag' }, { type: 'object', properties: { value: { type: 'string' } }, additionalProperties: false },
    ] },
    [['true', true], ['null', false], ['[1]', true], ['[]', false], ['[1,2,3]', false], ['{"value":"a"}', true],
      ['{}', false], ['{"value":1}', false], ['{"value":"a","extra":null}', false]],
    [],
  ],
];
const impossible = [
  { $defs: stringDefinitions, $ref: '#/$defs/address', pattern: '^b' },
  { $defs: numericDefinitions, $ref: '#/$defs/restricted', type: 'integer', maximum: -1 },
  { type: 'integer', minimum: 5, anyOf: [{ maximum: 3 }, { exclusiveMaximum: 5 }] },
  { $defs: collectionDefinitions, $ref: '#/$defs/limited', maxItems: 1 },
  { $defs: collectionDefinitions, $ref: '#/$defs/identified', required: ['extra'] },
  { type: 'array', minItems: 2, anyOf: [{ maxItems: 1 }, { maxItems: 0 }] },
  { ...closedArrayReference, minItems: 1 },
];

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) {
    const file = new URL('../../parity/fixtures/json-schema-and-recovery.lino', import.meta.url);
    const data = readProjectData(file);
    const corrected = data.grammars.findIndex(([schema]) => JSON.stringify(schema) === JSON.stringify(closedArrayReference));
    const replacement = cases.find(([schema]) => schema === closedArrayReference);
    const changed = corrected >= 0 && JSON.stringify(data.grammars[corrected]) !== JSON.stringify(replacement);
    if (changed) data.grammars[corrected] = replacement;
    const grammars = new Set(data.grammars.map(([schema]) => JSON.stringify(schema)));
    const empty = new Set(data.ungeneratableSchemas.map(schema => JSON.stringify(schema)));
    const newGrammars = cases.filter(([schema]) => !grammars.has(JSON.stringify(schema)));
    const newEmpty = impossible.filter(schema => !empty.has(JSON.stringify(schema)));
    if (changed || newGrammars.length || newEmpty.length) {
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
