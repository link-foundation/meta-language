import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { validateJson } from '../../js/src/json-schema-validation.js';

const numericSource = '[1.00000000000000000001,0.99999999999999999999,-1.00000000000000000001,1e-400,100e-2]';
const constantSource = '{"café":{"data":[0.300000000000000000001]}}';
const enumerationSource = '{"café":1.00000000000000000001}';
const cases = [
  [numericSource, { type: 'array', prefixItems: [
    { type: 'integer', maximum: 1 }, { minimum: 1 }, { minimum: -1 },
    { type: 'integer', exclusiveMinimum: 0 }, { type: 'integer', minimum: 1, maximum: 1 },
  ] }, [
    ['type', '/0', 1, 23], ['maximum', '/0', 1, 23], ['minimum', '/1', 24, 46],
    ['minimum', '/2', 47, 70], ['type', '/3', 71, 77],
  ]],
  [constantSource, { const: { café: { data: [0.3] } } }, [['const', '', 0, Buffer.byteLength(constantSource)]]],
  [enumerationSource, { properties: { café: { enum: [1, 0] } } }, [['enum', '/café', 9, 31]]],
  ['{"café":{"data":[30e-2,-0.0e+100]}}', { const: { café: { data: [0.3, 0] } } }, []],
  ['[1e-400,-1e-400,0.0E-400]', { type: 'array', prefixItems: [
    { exclusiveMinimum: 0 }, { exclusiveMaximum: 0 }, { type: 'integer', const: 0 },
  ] }, []],
  ['[1e-400,-1e-400]', { type: 'array', prefixItems: [
    { exclusiveMaximum: 0 }, { exclusiveMinimum: 0 },
  ] }, [['exclusiveMaximum', '/0', 1, 7], ['exclusiveMinimum', '/1', 8, 15]]],
];

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) {
    const file = new URL('../../parity/fixtures/json-schema-and-recovery.lino', import.meta.url);
    const data = readProjectData(file);
    data.numericValidation = cases;
    writeProjectData(file, data);
  } else {
    for (const [source, schema, expected] of cases) {
      const result = validateJson(source, schema);
      assert.deepEqual(result.schemaIssues, []);
      assert.deepEqual(result.errors.map(issue => [issue.keyword, issue.pointer, issue.span.byteRange.start, issue.span.byteRange.end]), expected);
    }
  }
}
