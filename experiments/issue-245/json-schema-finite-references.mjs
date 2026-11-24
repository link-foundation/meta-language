import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar } from '../../js/src/grammar.js';
import { importJsonSchema } from '../../js/src/grammar-importers/json-schema.js';

const cases = [
  [{
    $defs: {
      addresses: { enum: ['a@example.org', 'b@example.org', 'a+tag@example.org', 'a@example.net', 'a..b@example.org'] },
      selected: { $ref: '#/$defs/addresses', format: 'email', pattern: '^a' },
    },
    $ref: '#/$defs/selected',
    pattern: '@example\\.org$',
  }, [
    ['"a@example.org"', true], ['"a+tag\\u0040example.org"', true],
    ['"b@example.org"', false], ['"a@example.net"', false], ['"a..b@example.org"', false],
  ]],
  [{
    $defs: { colors: { enum: ['red', 'green'] }, violet: { const: 'violet' } },
    oneOf: [
      { $ref: '#/$defs/colors' },
      { anyOf: [{ const: 'red' }, { $ref: '#/$defs/violet' }] },
    ],
  }, [
    ['"green"', true], ['"violet"', true], ['"\\u0076iolet"', true],
    ['"red"', false], ['"yellow"', false],
  ]],
  [{
    $defs: { labels: { enum: ['answer', 'question', false] } },
    anyOf: [{ $ref: '#/$defs/labels' }, { const: 'another' }, false],
    type: 'string',
    pattern: '^an',
  }, [
    ['"answer"', true], ['"another"', true], ['false', false], ['"question"', false],
  ]],
];

const impossible = {
  $defs: { labels: { enum: ['answer', 'question'] } },
  $ref: '#/$defs/labels',
  type: 'boolean',
};

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) {
    const file = new URL('../../parity/fixtures/json-schema-and-recovery.lino', import.meta.url);
    const data = readProjectData(file);
    const missing = cases.filter(([schema]) => !data.grammars.some(([existing]) => JSON.stringify(existing) === JSON.stringify(schema)));
    const haveImpossible = data.ungeneratableSchemas.some(schema => JSON.stringify(schema) === JSON.stringify(impossible));
    if (missing.length || !haveImpossible) {
      data.grammars.push(...missing.map(([schema, inputs]) => [schema, inputs, []]));
      if (!haveImpossible) data.ungeneratableSchemas.push(impossible);
      writeProjectData(file, data);
    }
  } else {
    for (const [schema, inputs] of cases) {
      const grammar = importJsonSchema(schema);
      assert.deepEqual(grammar.schemaDiagnostics, []);
      const parser = compileGrammar(grammar);
      for (const [input, accepted] of inputs) assert.equal(parser.parseTree(input).ok, accepted, input);
    }
    assert.throws(() => importJsonSchema(impossible), /no generatable document/u);
  }
}
