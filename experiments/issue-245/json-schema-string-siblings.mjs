// A reference or alternative combines its own regular string language with
// its sibling assertions, including overlapping and unrestricted branches.
import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar, importJsonSchema, validateJson } from '../../js/src/index.js';

const schemas = [
  {
    schema: {
      $defs: { identity: { type: 'string', format: 'uuid' } },
      $ref: '#/$defs/identity',
      pattern: '^1234',
    },
    accepted: [
      '"12345678-1234-1234-1234-1234567890ab"',
      '"\\u00312345678-1234-1234-1234-1234567890a\\u0062"',
    ],
    rejected: [
      '"99995678-1234-1234-1234-1234567890ab"',
      '"12345678-1234-1234-1234-1234567890ag"',
      '"1234"',
      '1234',
    ],
  },
  {
    schema: { type: 'string', format: 'email', anyOf: [{ pattern: '^alice' }, { pattern: '@example\\.org$' }] },
    accepted: ['"alice@example.org"', '"alice@example.net"', '"eve@example.org"', '"eve\\u0040example.org"'],
    rejected: ['"alice"', '"org"', '"eve@example.net"', '"eve@other.org"', 'true'],
  },
  {
    schema: { type: 'string', $defs: { anything: true }, $ref: '#/$defs/anything', pattern: '^a+$' },
    accepted: ['"aaa"', '"a\\u0061"'],
    rejected: ['""', '"aba"', '"猫"', '42'],
  },
  {
    schema: { type: 'string', oneOf: [true, { pattern: '^a' }] },
    accepted: ['""', '"ba"', '"猫"', '"\\ud83d\\udc31"'],
    rejected: ['"a"', '"abc"', '"\\u0061bc"', 'false'],
  },
  {
    schema: { type: 'string', pattern: '^a', anyOf: [false, { pattern: 'b$' }] },
    accepted: ['"ab"', '"an\\u0062"'],
    rejected: ['"abc"', '"cb"', '"a"', 'false'],
  },
  {
    schema: { type: 'string', oneOf: [false, { pattern: '^a' }, { pattern: 'z$' }] },
    accepted: ['"abc"', '"bz"', '"\\u0061b"'],
    rejected: ['"az"', '"bb"', '"a\\u007a"', 'false'],
  },
];
const impossible = { $defs: { a: { type: 'string', pattern: '^a$' } }, $ref: '#/$defs/a', pattern: '^b$' };
const fallback = [
  { type: 'string', format: 'email', anyOf: [{ pattern: '^alice' }, { pattern: 'org$' }] },
  [['"alice"', true], ['"eve@example.org"', true], ['"eve@example.net"', false]],
  ['/anyOf'],
  [['"alice"', false], ['"eve@example.org"', true], ['"eve@example.net"', false]],
];
const examples = schemas.map(({ schema, accepted, rejected }) => [
  schema, [...accepted.map(text => [text, true]), ...rejected.map(text => [text, false])], [],
]);
examples.push(fallback);

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
    for (const [schema, cases, diagnostics, validation = cases] of examples) {
      const grammar = importJsonSchema(schema);
      const parser = compileGrammar(grammar);
      for (const [text, expected] of cases) {
        assert.equal(parser.parseTree(text).ok, expected, `${JSON.stringify(schema)} decoder: ${text}`);
      }
      for (const [text, expected] of validation) {
        assert.equal(validateJson(text, schema).valid, expected, `${JSON.stringify(schema)} validator: ${text}`);
      }
      assert.deepEqual(grammar.schemaDiagnostics.map(issue => issue.pointer), diagnostics);
    }
    assert.throws(() => importJsonSchema(impossible), /no generatable document/u);
  }
}
