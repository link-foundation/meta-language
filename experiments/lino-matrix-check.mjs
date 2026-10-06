// Compares meta-language's LiNo reading of every matrix feature with the
// links the matrix expects.
import { readFileSync } from 'node:fs';
import { LinkNetwork } from '../js/src/index.js';
import { insertLinoSemantics, linoReading } from '../js/src/lino-semantics.js';
const matrix = JSON.parse(readFileSync(new URL('../parity/fixtures/lino-compatibility-matrix.json', import.meta.url)));
for (const feature of matrix.features) {
  const network = new LinkNetwork();
  const reading = insertLinoSemantics(network, feature.source, 'LiNo').map((id) => linoReading(network, id));
  const same = JSON.stringify(reading) === JSON.stringify(feature.links);
  console.log(feature.id, same ? 'agrees' : `DIFFERS ${JSON.stringify(reading)}`);
}
