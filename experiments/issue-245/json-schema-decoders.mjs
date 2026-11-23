import { readProjectData } from '../../js/scripts/project-data.mjs';
import { emitGbnf, emitLark, importJsonSchema } from '../../js/src/index.js';

const fixture = new URL('../../parity/fixtures/json-schema-and-recovery.lino', import.meta.url);
const { grammars } = readProjectData(fixture);
const exclusiveArrays = new URL('../../parity/fixtures/json-schema-exclusive-array-lengths.lino', import.meta.url);

const samples = [...grammars, ...readProjectData(exclusiveArrays).grammars].map(([schema, cases]) => {
  const grammar = importJsonSchema(schema);
  return { gbnf: emitGbnf(grammar).source, lark: emitLark(grammar).source, cases };
});

process.stdout.write(JSON.stringify(samples));
