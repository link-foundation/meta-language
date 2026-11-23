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
const conflictingArrayReference = {
  $ref: '#/$defs/prefix', prefixItems: [{ type: 'string' }], $defs: closedArrayReference.$defs,
};
const layeredArrayReference = {
  type: 'array', $ref: '#/$defs/row', prefixItems: [true, { type: 'string' }], items: { type: 'null' }, minItems: 2,
  $defs: { row: { prefixItems: [{ type: 'boolean' }, { type: 'string' }], maxItems: 3 } },
};
const layeredPatternProperty = {
  type: 'object', $ref: '#/$defs/record', properties: { value: { pattern: 'z$' } }, $defs: { record: {
    properties: { value: { type: 'string', pattern: '^a' } }, required: ['value'],
  } },
};
const layeredPatternItem = {
  type: 'array', $ref: '#/$defs/row', prefixItems: [{ type: 'string', pattern: 'z$' }],
  $defs: { row: { prefixItems: [{ type: 'string', pattern: '^a' }] } },
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
    conflictingArrayReference,
    [['[]', true], ['[true]', false], ['[false,0]', false], ['[1]', false]],
    [],
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
  [
    { $ref: '#/$defs/record', properties: { active: { type: 'boolean' }, value: { type: 'string' } }, required: ['active'],
      $defs: { record: { type: 'object', properties: { kind: { const: 'record' }, value: { type: 'string' } }, required: ['kind'] } } },
    [['{"kind":"record","active":false}', true], ['{"active":true,"other":null,"kind":"record","value":"a"}', true],
      ['{"act\\u0069ve":true,"kind":"record"}', true], ['{"active":1,"kind":"record"}', false],
      ['{"active":true,"kind":"other"}', false], ['{"kind":"record"}', false], ['{"active":true}', false],
      ['{"kind":"record","active":true,"value":null}', false]],
    [],
  ],
  [
    { type: 'object', $ref: '#/$defs/closed', properties: { label: { type: 'string' }, ignored: true },
      additionalProperties: { type: 'boolean' }, $defs: { closed: {
        properties: { label: true, enabled: { type: 'boolean' } }, required: ['label'], additionalProperties: false,
      } } },
    [['{"label":"a"}', true], ['{"enabled":false,"label":"b"}', true], ['{"label":false}', false],
      ['{"label":"c","enabled":null}', false], ['{"label":"a","ignored":true}', false],
      ['{"label":"a","other":false}', false], ['{}', false]],
    [],
  ],
  [
    { type: 'object', required: ['active'], properties: { active: { type: 'boolean' } }, anyOf: [
      { properties: { label: { type: 'string' } }, required: ['label'] },
      { properties: { count: { type: 'number' } }, required: ['count'] },
    ] },
    [['{"active":true,"label":"a"}', true], ['{"count":3,"active":false}', true],
      ['{"label":"a","active":null}', false], ['{"count":true,"active":false}', false],
      ['{"active":true,"label":null,"count":3}', true], ['{"label":"a"}', false], ['{"active":true}', false]],
    [],
  ],
  [
    layeredPatternProperty,
    [['{"value":"az"}', true], ['{"value":"ax"}', false], ['{"value":"xz"}', false]],
    [],
    [['{"value":"az"}', true], ['{"value":"ax"}', false], ['{"value":"xz"}', false]],
  ],
  [
    layeredArrayReference,
    [['[true,"a"]', true], ['[false,"b",null]', true], ['[false,"b",true]', false], ['[false,3]', false],
      ['[true]', false], ['[1,"a"]', false], ['[true,"a",null,null]', false], ['[]', false], ['null', false]],
    [],
  ],
  [
    { type: 'array', prefixItems: [true, { type: 'null' }], anyOf: [
      { items: { type: 'string' } }, { prefixItems: [{ const: true }, true], items: { type: 'boolean' } },
    ] },
    [['[]', true], ['["a"]', true], ['[true]', true], ['[true,null]', true], ['[true,null,false]', true],
      ['[true,null,false,true]', true], ['["a","b"]', false], ['["a",null]', false], ['[false]', false],
      ['[true,0]', false], ['[true,null,null]', false], ['{}', false]],
    [],
  ],
  [
    { type: 'array', $ref: '#/$defs/row', prefixItems: [{ type: 'boolean' }, { type: 'string', pattern: 'x$' }],
      maxItems: 1, $defs: { row: { prefixItems: [{ type: 'boolean' }, { type: 'string', pattern: '^a' }], minItems: 1 } } },
    [['[true]', true], ['[false]', true], ['[]', false], ['[null]', false], ['[true,"ax"]', false]],
    [],
  ],
  [
    { type: 'array', $ref: '#/$defs/row', prefixItems: [false, { type: 'string', pattern: 'x$' }],
      $defs: { row: { prefixItems: [{ type: 'boolean' }, { type: 'string', pattern: '^a' }] } } },
    [['[]', true], ['[true]', false], ['[true,"ax"]', false], ['[null]', false]],
    [],
  ],
  [
    { type: 'array', $ref: '#/$defs/row', prefixItems: [{ type: 'null' }], items: { type: 'number' },
      $defs: { row: { prefixItems: [true], items: { type: 'boolean' }, minItems: 1 } } },
    [['[null]', true], ['[]', false], ['[true]', false], ['[null,1]', false], ['[null,false]', false]],
    [],
  ],
  [
    layeredPatternItem,
    [['[]', true], ['["az"]', true], ['["ax"]', false], ['["xz"]', false]],
    [],
    [['[]', true], ['["az"]', true], ['["ax"]', false], ['["xz"]', false]],
  ],
  [
    { ...layeredPatternItem, prefixItems: [{ anyOf: [{ type: 'string', pattern: 'z$' }, { type: 'number' }] }] },
    [['[]', true], ['["az"]', true], ['["ax"]', true], ['["xz"]', false]],
    ['/$ref'],
    [['[]', true], ['["az"]', true], ['["ax"]', false], ['["xz"]', false]],
  ],
  [
    { type: 'object', $ref: '#/$defs/record', properties: { value: { type: 'string' } },
      $defs: { record: { properties: { value: { type: 'boolean' } } } } },
    [['{}', true], ['{"other":true}', true], ['{"value":true}', false], ['{"value":"a"}', false],
      ['{"v\\u0061lue":true}', false]],
    [],
  ],
  [
    { type: 'array', $ref: '#/$defs/row', items: { maximum: 4 },
      $defs: { row: { prefixItems: [{ type: 'boolean' }], items: { type: 'integer', minimum: 2 } } } },
    [['[]', true], ['[false]', true], ['[false,2,4]', true], ['[true,1]', false], ['[true,5]', false],
      ['[true,3.5]', false], ['[true,3.0]', false], ['[2,false]', false]],
    ['/items/type'],
    [['[]', true], ['[false]', true], ['[false,2,4]', true], ['[true,1]', false], ['[true,5]', false],
      ['[true,3.5]', false], ['[true,3.0]', true], ['[2,false]', false]],
  ],
  [
    { type: 'object', $ref: '#/$defs/record', additionalProperties: { pattern: 'z$' },
      $defs: { record: { properties: { id: { type: 'string', pattern: '^a' } }, required: ['id'],
        additionalProperties: { type: 'string', pattern: '^b' } } } },
    [['{"id":"az"}', true], ['{"other":"bz","i\\u0064":"az"}', true], ['{"id":"ax"}', false],
      ['{"id":"az","other":"bx"}', false], ['{"id":"az","other":"az"}', false],
      ['{"id":"az","other":true}', false]],
    [],
    [['{"id":"az"}', true], ['{"other":"bz","i\\u0064":"az"}', true], ['{"id":"ax"}', false],
      ['{"id":"az","other":"bx"}', false], ['{"id":"az","other":"az"}', false],
      ['{"id":"az","other":true}', false]],
  ],
  [
    { type: 'object', $ref: '#/$defs/record', properties: { code: { $ref: '#/$defs/suffix' } }, $defs: {
      record: { properties: { code: { $ref: '#/$defs/prefix' } }, required: ['code'] },
      prefix: { type: 'string', pattern: '^a' }, suffix: { pattern: 'z$' },
    } },
    [['{"code":"az"}', true], ['{"co\\u0064e":"a\\u007a"}', true], ['{"code":"ax"}', false],
      ['{"code":"xz"}', false], ['{"code":false}', false], ['{}', false]],
    [],
    [['{"code":"az"}', true], ['{"co\\u0064e":"a\\u007a"}', true], ['{"code":"ax"}', false],
      ['{"code":"xz"}', false], ['{"code":false}', false], ['{}', false]],
  ],
  [
    { type: 'array', $ref: '#/$defs/row', items: { pattern: 'z$' }, $defs: { row: { items: { pattern: '^a' } } } },
    [['[]', true], ['["az",null,true,2,{},[]]', true], ['["ax"]', false], ['["xz"]', false], ['[true,"ax"]', false]],
    [],
    [['[]', true], ['["az",null,true,2,{},[]]', true], ['["ax"]', false], ['["xz"]', false], ['[true,"ax"]', false]],
  ],
  [
    { type: 'object', $ref: '#/$defs/record', properties: { flag: { $ref: '#/$defs/enabled' } }, $defs: {
      record: { required: ['flag'], properties: { flag: { enum: [true, false, null] } } },
      enabled: { oneOf: [{ const: true }, { const: false }, { enum: [false, null] }, { const: null }] },
    } },
    [['{"flag":true}', true], ['{"fl\\u0061g":true}', true], ['{"flag":false}', false],
      ['{"flag":null}', false], ['{"flag":"true"}', false], ['{}', false]],
    [],
  ],
  [
    { type: 'array', $ref: '#/$defs/row', items: { minimum: 2, maximum: 3 },
      $defs: { row: { prefixItems: [{ type: 'null' }], items: { enum: [1, 2, 3, 4, true] } } } },
    [['[]', true], ['[null]', true], ['[null,2,3,true]', true], ['[null,1]', false], ['[null,4]', false],
      ['[null,2.5]', false], ['[null,"2"]', false], ['[true,2]', false]],
    [],
  ],
  [
    { type: 'object', $ref: '#/$defs/record', properties: { value: { properties: { flag: { const: true } } } },
      $defs: { record: { required: ['value'], properties: { value: { enum: [
        { flag: false, label: 'a' }, { flag: true, label: 'a' }, { flag: true, label: 'b' },
      ] } } } } },
    [['{"value":{"flag":true,"label":"a"}}', true], ['{"value":{"label":"\\u0062","flag":true}}', true],
      ['{"value":{"label":"a","flag":false}}', false], ['{"value":{"flag":true,"label":"c"}}', false],
      ['{"value":{"label":"a"}}', false], ['{}', false]],
    [],
  ],
  [
    { type: 'array', $ref: '#/$defs/rows', items: { properties: { label: { type: 'string' } }, required: ['label'] },
      $defs: { rows: { items: { type: 'object', properties: { active: { type: 'boolean' } }, required: ['active'] } } } },
    [['[]', true], ['[{"active":true,"label":"a"}]', true],
      ['[{"label":"a","active":false},{"active":true,"l\\u0061bel":"b"}]', true],
      ['[{"active":true}]', false], ['[{"label":"a"}]', false], ['[{"active":true,"label":1}]', false], ['[null]', false]],
    [],
  ],
  [
    { type: 'object', $ref: '#/$defs/record', properties: { entries: { maxItems: 2, items: { type: 'boolean' } } },
      $defs: { record: { required: ['entries'], properties: { entries: { type: 'array', minItems: 1 } } } } },
    [['{"entries":[true]}', true], ['{"entries":[true,false]}', true], ['{"entries":[]}', false],
      ['{"entries":[true,false,true]}', false], ['{"entries":[null]}', false], ['{"entries":false}', false], ['{}', false]],
    [],
  ],
  [
    { type: 'array', $ref: '#/$defs/first', items: { $ref: '#/$defs/second' }, $defs: {
      first: { type: ['array', 'boolean'], items: { $ref: '#/$defs/first' } },
      second: { type: ['array', 'boolean'], items: { $ref: '#/$defs/second' }, maxItems: 1 },
    } },
    [['[]', true], ['[true,[false]]', true], ['[[true,false]]', true], ['[null]', false], ['true', false]],
    ['/$ref'],
    [['[]', true], ['[true,[false]]', true], ['[[true,false]]', false], ['[null]', false], ['true', false]],
  ],
  [
    { type: ['array', 'null'], items: { type: 'boolean' }, $defs: {
      short: { type: 'array', prefixItems: [true], items: false },
      long: { $ref: '#/$defs/minimum', maxItems: 3 },
      minimum: { type: 'array', minItems: 2 },
    }, oneOf: [{ $ref: '#/$defs/short' }, { $ref: '#/$defs/long' }, { type: 'null' }] },
    [['[]', true], ['[true]', true], ['[true,false]', true], ['[true,false,true]', true],
      ['[true,false,true,false]', false], ['["x"]', false], ['[false,null]', false], ['null', true], ['false', false]],
    [],
  ],
  [
    { type: 'array', items: { type: 'boolean' }, oneOf: [{ maxItems: 2 }, { minItems: 2, maxItems: 3 }] },
    [['[]', true], ['[true,false]', true], ['[true,false,true]', true], ['[null]', true], ['[true,false,true,false]', false]],
    ['/oneOf', '/oneOf'],
    [['[]', true], ['[true,false]', false], ['[true,false,true]', true], ['[null]', false], ['[true,false,true,false]', false]],
  ],
  [
    { type: 'array', prefixItems: [true], items: { type: 'boolean' }, $defs: {
      selected: { type: 'array', minItems: 2, prefixItems: [{ const: 'selected' }] },
      unselected: { type: 'array', prefixItems: [{ enum: ['pending', 'unknown'] }] },
    }, oneOf: [{ $ref: '#/$defs/selected' }, { $ref: '#/$defs/unselected', minItems: 2 }] },
    [['["selected",true]', true], ['["p\\u0065nding",false,true]', true], ['["unknown",false]', true],
      ['["selected",null]', false], ['["pending","x"]', false], ['["other",true]', false],
      ['["selected"]', false], ['[]', false]],
    [],
  ],
  [
    { type: 'array', oneOf: [{ prefixItems: [{ const: 'selected' }] }, { prefixItems: [{ const: 'pending' }] }] },
    [['[]', true], ['["selected"]', true], ['["pending"]', true], ['["other"]', false]],
    ['/oneOf'],
    [['[]', false], ['["selected"]', true], ['["pending"]', true], ['["other"]', false]],
  ],
  [
    { type: 'number', minimum: -0.75, maximum: 4, $defs: numericDefinitions, oneOf: [
      { $ref: '#/$defs/restricted', maximum: 2.5 }, { exclusiveMinimum: 1.25 }, false,
    ] },
    [['-0.75', true], ['-0.7501', false], ['-0', true], ['1.2500', true], ['1.2501', false], ['2.5', false],
      ['2.5001', true], ['4.00', true], ['4.001', false], ['-7.5e-1', false], ['null', false]],
    ['/oneOf'],
    [['-0.75', true], ['-0.7501', false], ['1.2500', true], ['1.2501', false], ['2.5', false], ['2.5001', true],
      ['4.00', true], ['4.001', false], ['-7.5e-1', true], ['null', false]],
  ],
  [
    { type: 'number', oneOf: [{ maximum: 1.25 }, { exclusiveMaximum: 1.25 }, { minimum: 1.25, exclusiveMaximum: 1.25 }] },
    [['1.25', true], ['1.25000', true], ['1.25001', false], ['1.24999', false], ['0', false], ['125e-2', false]],
    ['/oneOf'],
    [['1.25000', true], ['1.25001', false], ['1.24999', false], ['125e-2', true]],
  ],
  [
    { type: 'number', oneOf: [{ exclusiveMaximum: 0 }, { exclusiveMinimum: 0 }] },
    [['0', false], ['-0.00', false], ['0.0001', true], ['-0.0001', true], ['123456789.0', true], ['-123456789', true]],
    ['/oneOf'],
    [['0', false], ['-0.00', false], ['0.0001', true], ['-0.0001', true], ['123456789.0', true], ['-123456789', true]],
  ],
  [
    { type: 'number', oneOf: [{ exclusiveMaximum: 0 }, { minimum: 0 }] },
    [['0.0', true], ['-0.00', true], ['-1e-5', true], ['1.2e309', true], ['null', false]],
    [],
  ],
  [
    {
      type: 'number', minimum: -2.5, maximum: 3.5,
      $defs: { decimals: { type: 'number', minimum: -1.25 }, integers: { type: 'integer', maximum: 1 } },
      oneOf: [{ $ref: '#/$defs/decimals', exclusiveMaximum: 3.25 }, { $ref: '#/$defs/integers' }],
    },
    [['-1.00', false], ['-3', false], ['-2.00', true], ['-2.50', false], ['-1.2501', false], ['-1.25', true],
      ['-0.00', false], ['0.001', true], ['1', false], ['1.5', true], ['2.00', true], ['3.20', true],
      ['3.25', false], ['2e0', false], ['null', false]],
    ['/oneOf'],
    [['-1.00', false], ['-2.00', true], ['-1.2501', false], ['-1.25', true], ['-0.00', false], ['0.001', true],
      ['1', false], ['2.00', true], ['3.20', true], ['3.25', false], ['2e0', true]],
  ],
  [
    { oneOf: [{ type: 'number' }, { type: 'integer' }] },
    [['0.002', true], ['-13.20', true], ['123', false], ['-0.0', false], ['123.000', false], ['1e-1', false], ['true', false]],
    ['/oneOf'],
    [['0.002', true], ['-13.20', true], ['123', false], ['-0.0', false], ['123.000', false], ['1e-1', true], ['true', false]],
  ],
  [
    { type: 'number', oneOf: [{ maximum: 0 }, { minimum: 0 }, { type: 'integer', minimum: 0 }] },
    [['-1.00', true], ['-0.01', true], ['-0.0', false], ['0.000', false], ['0.01', true], ['1', false], ['1.1', true]],
    ['/oneOf'],
  ],
  [
    { oneOf: [{ type: 'number', minimum: -1 }, { type: 'integer', maximum: 1 }, { type: 'null' }] },
    [['null', true], ['-2', true], ['2', true], ['0', false], ['-1.00', false], ['0.25', true], ['"text"', false]],
    ['/oneOf'],
  ],
  [
    {
      type: ['number', 'string', 'boolean'], minimum: -3, pattern: '[az]',
      $defs: { ...stringDefinitions, decimals: { type: 'number', maximum: 5 }, integers: { type: 'integer', minimum: 0 } },
      oneOf: [{ $ref: '#/$defs/identifier' }, { type: 'string', pattern: 'z$' },
        { $ref: '#/$defs/decimals' }, { $ref: '#/$defs/integers' }, { type: 'boolean' }],
    },
    [['"ab"', true], ['"x\\u007a"', true], ['"az"', false], ['"bx"', false], ['-3', true], ['-4', false],
      ['0.00', false], ['3.5', true], ['6', true], ['6.5', false], ['true', true], ['false', true], ['null', false]],
    ['/oneOf'],
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
  { ...conflictingArrayReference, minItems: 1 },
  { ...layeredPatternProperty, properties: { value: { pattern: '^x' } } },
  { type: 'object', $ref: '#/$defs/record', properties: { flag: { const: false } },
    $defs: { record: { required: ['flag'], properties: { flag: { const: true } } } } },
  { $ref: '#/$defs/record', properties: { active: { type: 'boolean' } }, additionalProperties: false,
    $defs: { record: { type: 'object', properties: { label: { type: 'string' } }, required: ['label'] } } },
  { type: 'object', $ref: '#/$defs/record', properties: { entries: { items: false } }, $defs: { record: {
    required: ['entries'], properties: { entries: { type: 'array', minItems: 1 } },
  } } },
  { type: 'number', oneOf: [{ minimum: -1.25 }, { minimum: -1.25 }] },
  { type: 'number', oneOf: [{ maximum: 0 }, { maximum: 0 }, { type: 'integer', minimum: 0, maximum: 0 }] },
  { oneOf: [{ type: 'number' }, { type: 'number' }, { type: 'string' }, { type: 'string' }] },
];

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) {
    const file = new URL('../../parity/fixtures/json-schema-and-recovery.lino', import.meta.url);
    const data = readProjectData(file);
    let changed = false;
    for (const corrected of [closedArrayReference, conflictingArrayReference, layeredPatternProperty, layeredPatternItem]) {
      const existing = data.grammars.findIndex(([schema]) => JSON.stringify(schema) === JSON.stringify(corrected));
      const replacement = cases.find(([schema]) => schema === corrected);
      if (existing >= 0 && JSON.stringify(data.grammars[existing]) !== JSON.stringify(replacement)) {
        data.grammars[existing] = replacement;
        changed = true;
      }
    }
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
