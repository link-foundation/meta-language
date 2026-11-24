import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar } from '../../js/src/grammar.js';
import { importJsonSchema } from '../../js/src/grammar-importers/json-schema.js';
import { validateJson } from '../../js/src/json-schema-validation.js';

const cases = [
  [{ const: 1 }, [
    ['1', true], ['1.0', true], ['1E-000', true], ['10e-1', true],
    ['0.1e+01', true], ['0.01E2', true], ['1.00e1', false],
    ['1.01', false], ['01e-0', false], ['1.', false], ['1e+', false],
  ], ['/const']],
  [{ const: 0 }, [
    ['0', true], ['-0', true], ['-0.000', true], ['0.0E-00123', true],
    ['-0E+9999', true], ['0.00001e-9999', false], ['00.00', false], ['-0e', false],
  ], []],
  [{ enum: [-12.34, 1e-18, 1e21] }, [
    ['-12.3400', true], ['-123.4e-1', true], ['-0.1234E2', true], ['-1.234e+01', true],
    ['1e-18', true], ['10E-019', true], ['0.000000000000000001000', true],
    ['1000000000000000000000.00', true], ['1.0E+021', true],
    ['-12.339999', false], ['1.0e+22', false], ['1e-019', false],
  ], ['/enum']],
  [{ enum: [{ score: 1.25, factors: [0, -2] }] }, [
    ['{"score":1.250e-0,"factors":[-0.0, -2.00]}', true],
    ['{"factors":[0e777,-20e-1],"score":125e-2}', true],
    ['{"score":1.251,"factors":[-0, -2]}', false],
    ['{"factors":[0.0001,-2],"score":1.25}', false],
    ['{"score":1.25,"factors":[-2,-0]}', false],
  ], ['/enum']],
];
const outsideMantissaFamily = '10000000000e-10';

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) {
    const file = new URL('../../parity/fixtures/json-schema-and-recovery.lino', import.meta.url);
    const data = readProjectData(file);
    const joinedSchema = {
      type: 'array',
      $ref: '#/$defs/row',
      items: { minimum: 2, maximum: 3 },
      $defs: { row: { prefixItems: [{ type: 'null' }], items: { enum: [1, 2, 3, 4, true] } } },
    };
    const joined = data.grammars.find(([schema]) => JSON.stringify(schema) === JSON.stringify(joinedSchema));
    assert.deepEqual(joined.slice(2), [[]]);
    joined[2] = ['/items/enum'];
    joined[3] = [...joined[1]];
    for (const [schema, inputs, diagnostics] of cases) {
      assert.ok(data.grammars.every(([existing]) => JSON.stringify(existing) !== JSON.stringify(schema)));
      const decoderCases = [...inputs];
      const validationCases = [...inputs];
      if (schema.const === 1) {
        decoderCases.push([outsideMantissaFamily, false]);
        validationCases.push([outsideMantissaFamily, true]);
      }
      data.grammars.push(diagnostics.length
        ? [schema, decoderCases, diagnostics, validationCases]
        : [schema, decoderCases, diagnostics]);
    }
    writeProjectData(file, data);
    process.exit(0);
  }
  for (const [schema, inputs, diagnostics] of cases) {
    const sourceGrammar = importJsonSchema(schema);
    const decoder = compileGrammar(sourceGrammar);
    for (const [source, accepted] of inputs) {
      assert.equal(validateJson(source, schema).valid, accepted, `validation ${source}`);
      assert.equal(decoder.parseTree(source).ok, accepted, `decoder ${source}`);
    }
    assert.deepEqual(sourceGrammar.schemaDiagnostics.map(issue => issue.pointer), diagnostics);
    if (schema.const === 1) {
      assert.equal(decoder.parseTree(outsideMantissaFamily).ok, false);
      assert.equal(validateJson(outsideMantissaFamily, schema).valid, true);
    }
  }
  console.log(`${cases.length} constant and enum schemas passed`);
}
