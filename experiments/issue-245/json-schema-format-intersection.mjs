import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar } from '../../js/src/grammar.js';
import { importJsonSchema } from '../../js/src/grammar-importers/json-schema.js';

const cases = [
  [{ type: 'string', format: 'date', pattern: '^20[0-9]{2}-02-' }, [
    ['"2024-02-29"', true], ['"\\u0032024-02-29"', true],
    ['"2024-02-31"', false], ['"2024-03-29"', false], ['"1996-02-29"', false],
  ]],
  [{ type: 'string', format: 'email', pattern: '@example\\.(?:org|net)$' }, [
    ['"first.last@example.org"', true], ['"a\\u0040example.net"', true],
    ['"a..b@example.org"', false], ['"first.last@elsewhere.org"', false],
    ['"a@example.org.evil"', false],
  ]],
  [{ type: 'string', format: 'email', pattern: '^(?:a*|b)@example\\.org$' }, [
    ['"aaa@example.org"', true], ['"b@example.org"', true],
    ['"aaab@example.org"', false], ['"@example.org"', false],
  ]],
  [{ type: 'string', format: 'uuid', pattern: '^[0-9a-f-]+$' }, [
    ['"123e4567-e89b-12d3-a456-426614174000"', true],
    ['"123E4567-e89b-12d3-a456-426614174000"', false], ['"deadbeef"', false],
  ]],
  [{ type: 'string', format: 'date-time', pattern: '[Zz]$' }, [
    ['"2024-02-29T23:59:60Z"', true], ['"2024-02-29t23:59:59\\u007a"', true],
    ['"2024-02-29T24:00:00Z"', false], ['"2024-02-29T00:00:00+00:00"', false],
  ]],
];
const ungeneratable = [{ type: 'string', format: 'date', pattern: '^never$' }];

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) {
    const file = new URL('../../parity/fixtures/json-schema-and-recovery.lino', import.meta.url);
    const data = readProjectData(file);
    const missing = cases.filter(([schema]) => !data.grammars.some(([existing]) => JSON.stringify(existing) === JSON.stringify(schema)));
    if (missing.length || !data.ungeneratableSchemas) {
      data.grammars.push(...missing.map(([schema, inputs]) => [schema, inputs, []]));
      data.ungeneratableSchemas ??= ungeneratable;
      writeProjectData(file, data);
    }
  } else {
    for (const [schema, inputs] of cases) {
      const grammar = importJsonSchema(schema);
      assert.deepEqual(grammar.schemaDiagnostics, [], schema.format);
      const parser = compileGrammar(grammar);
      for (const [input, accepted] of inputs) assert.equal(parser.parseTree(input).ok, accepted, input);
      console.log(schema.format, grammar.rules.size, inputs.length);
    }
    for (const schema of ungeneratable) assert.throws(() => importJsonSchema(schema), /no generatable document/u);
  }
}
