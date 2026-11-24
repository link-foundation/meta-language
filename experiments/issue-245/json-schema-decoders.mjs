import { readProjectData } from '../../js/scripts/project-data.mjs';
import { emitGbnf, emitLark, importJsonSchema } from '../../js/src/index.js';

const fixtures = [
  'json-schema-and-recovery.lino',
  'json-schema-exclusive-array-lengths.lino',
  'json-schema-exclusive-typed-array-items.lino',
  'json-schema-exclusive-typed-array-prefixes.lino',
  'json-schema-exclusive-array-items.lino',
  'json-schema-exclusive-object-property.lino',
  'json-schema-exclusive-object-siblings.lino',
  'json-schema-exclusive-object-fields.lino',
  'json-schema-exclusive-required-properties.lino',
  'json-schema-exclusive-declared-properties.lino',
];

const grammars = fixtures.flatMap(file => readProjectData(new URL(`../../parity/fixtures/${file}`, import.meta.url)).grammars);
const samples = grammars.map(([schema, cases]) => {
  const grammar = importJsonSchema(schema);
  return { gbnf: emitGbnf(grammar).source, lark: emitLark(grammar).source, cases };
});

process.stdout.write(JSON.stringify(samples));
