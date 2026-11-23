import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar, importJsonSchema, validateJson } from '../../js/src/index.js';

const examples = [
  [{ type: 'string', oneOf: [{ pattern: '^a' }, { pattern: 'a$' }] }, [
    ['"ab"', true], ['"ba"', true], ['"\\u0061b"', true], ['"b\\u0061"', true],
    ['"a"', false], ['"abc\\u0061"', false], ['"bbb"', false], ['[]', false],
  ], []],
  [{ type: 'string', format: 'uuid', oneOf: [{ pattern: '^1' }, { pattern: '000$' }, { pattern: '^2' }] }, [
    ['"123e4567-e89b-12d3-a456-426614174001"', true],
    ['"223e4567-e89b-12d3-a456-426614174001"', true],
    ['"323e4567-e89b-12d3-a456-42661417400\\u0030"', true],
    ['"123e4567-e89b-12d3-a456-426614174000"', false],
    ['"223e4567-e89b-12d3-a456-426614174000"', false],
    ['"323e4567-e89b-12d3-a456-426614174001"', false],
    ['"1-not-a-uuid"', false],
  ], []],
  [{ oneOf: [
    { type: 'string', format: 'date', pattern: '^20' },
    { type: 'string', pattern: '^2024-02-' },
  ] }, [
    ['"2024-02-31"', true], ['"2024-03-01"', true], ['"2050-09-08"', true],
    ['"2024-02-29"', false], ['"\\u0032024-02-29"', false], ['"1999-04-10"', false],
  ], []],
  [{ type: 'string', oneOf: [{ pattern: '^a' }, { enum: ['a', 'b'] }] }, [
    ['"alpha"', true], ['"b"', true], ['"a"', false], ['"\\u0061"', false], ['"c"', false],
  ], []],
];
const impossible = { oneOf: [{ type: 'string', pattern: '^x' }, { type: 'string', pattern: '^x' }] };

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
    for (const [schema, cases, diagnostics] of examples) {
      const imported = importJsonSchema(schema);
      const parser = compileGrammar(imported);
      for (const [source, accepted] of cases) {
        assert.equal(validateJson(source, schema).valid, accepted, source);
        assert.equal(parser.parseTree(source).ok, accepted, source);
      }
      assert.deepEqual(imported.schemaDiagnostics.map(issue => issue.pointer), diagnostics);
    }
    assert.throws(() => importJsonSchema(impossible), /no generatable document/u);
  }
}
