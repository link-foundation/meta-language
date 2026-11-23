import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';

const schema = {
  oneOf: [
    false,
    { type: 'null' },
    {
      type: 'object', additionalProperties: false, required: ['kind', 'value'],
      properties: { kind: { enum: ['text', 'text/🌍'] }, value: { type: 'string' } },
    },
    {
      type: ['object'], additionalProperties: false, required: ['kind', 'value'],
      properties: { kind: { const: 'enabled' }, value: { type: 'boolean' } },
    },
  ],
};
const cases = [
  ['null', true],
  ['{"kind":"text","value":"example"}', true],
  ['{"value":"example","kind":"text/🌍"}', true],
  ['{"value":"example","k\\u0069nd":"text\\/\\ud83c\\udf0d"}', true],
  ['{"value":false,"kind":"enabled"}', true],
  ['{"kind":"text","value":true}', false],
  ['{"kind":"enabled","value":"example"}', false],
  ['{"kind":"enabled","value":false,"extra":null}', false],
  ['{"kind":"unknown","value":"example"}', false],
  ['{"value":"example"}', false],
  ['[]', false],
];

const record = (required, values) => ({
  type: 'object', required,
  properties: Object.fromEntries(Object.entries(values).map(([key, value]) => [key, { const: value }])),
});
const pairwise = {
  oneOf: [record(['a', 'b'], { a: 'first', b: 'first' }),
    record(['a', 'c'], { a: 'second', c: 'second' }),
    record(['b', 'c'], { b: 'third', c: 'third' })],
};
const pairwiseCases = [
  ['{"a":"first","b":"first","c":10}', true],
  ['{"c":"second","a":"second","b":"third"}', true],
  ['{"b":"third","c":"third","a":"first"}', true],
  ['{"a":"first","b":"third","c":"second"}', false],
  ['{"a":"first"}', false],
];
const optionalTag = { oneOf: [record([], { kind: 'first' }), record([], { kind: 'second' })] };
const overlappingTag = { oneOf: [
  record(['kind'], { kind: 'shared' }),
  { type: 'object', required: ['kind'], properties: { kind: { enum: ['shared', 'second'] } } },
] };
const examples = [
  [schema, cases, []],
  [pairwise, pairwiseCases, []],
  [optionalTag, [['{}', false], ['{"kind":"first"}', true], ['{"kind":"second"}', true]], [], [['{}', false]]],
  [overlappingTag, [['{"kind":"shared"}', false], ['{"kind":"second"}', true]], [], [['{"kind":"shared"}', false]]],
];

const file = new URL('../../parity/fixtures/json-schema-and-recovery.lino', import.meta.url);
const data = readProjectData(file);
let changed = false;
for (const example of examples) {
  const [schema] = example;
  const index = data.grammars.findIndex(([existing]) => JSON.stringify(existing) === JSON.stringify(schema));
  if (index < 0) {
    data.grammars.push(example);
    changed = true;
  } else if (JSON.stringify(data.grammars[index]) !== JSON.stringify(example)) {
    data.grammars[index] = example;
    changed = true;
  }
}
if (changed) writeProjectData(file, data);
