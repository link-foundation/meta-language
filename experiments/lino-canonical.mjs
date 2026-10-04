// Prints the official links-notation 0.22 reading of each matrix feature in
// the matrix's canonical form: a reference is its string, a link is
// [id or null, ...values].
import { readFileSync } from 'node:fs';
import { Parser } from '../js/node_modules/links-notation/src/index.js';
const canonical = (link) => (link.values.length === 0 && link.id !== null
  ? link.id : [link.id, ...link.values.map(canonical)]);
const matrix = JSON.parse(readFileSync(new URL('../parity/fixtures/lino-compatibility-matrix.json', import.meta.url)));
for (const feature of matrix.features) {
  const official = new Parser().parse(feature.source).map(canonical);
  const same = JSON.stringify(official) === JSON.stringify(feature.links);
  console.log(feature.id, same ? 'agrees' : `DIFFERS ${JSON.stringify(official)}`);
}
