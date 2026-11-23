import assert from 'node:assert/strict';
import { readProjectData } from '../../js/scripts/project-data.mjs';
import { compileGrammar, importJsonSchema, validateJson } from '../../js/src/index.js';

const fixture = new URL('../../parity/fixtures/json-schema-exclusive-array-lengths.lino', import.meta.url);
const { grammars, ungeneratable } = readProjectData(fixture);

for (const [schema, cases] of grammars) {
  const grammar = importJsonSchema(schema);
  const parser = compileGrammar(grammar);
  for (const [source, accepted] of cases) {
    assert.equal(validateJson(source, schema).valid, accepted, `validation of ${source}`);
    assert.equal(parser.parseTree(source).ok, accepted, `decoder for ${source}`);
  }
  assert.deepEqual(grammar.schemaDiagnostics, []);
}

for (const [schema, cases] of ungeneratable) {
  for (const [source, accepted] of cases) {
    assert.equal(validateJson(source, schema).valid, accepted, `validation of ${source}`);
  }
  assert.throws(() => importJsonSchema(schema), /no generatable document/u);
}

const inputs = grammars.reduce((total, [, cases]) => total + cases.length, 0);
console.log(`${inputs} array decoder inputs passed. Verified empty exclusive languages (${ungeneratable.length})`);
