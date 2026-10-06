// Prints the embedded regions of the JavaScript public network next to the
// grammar-derived regions in parity/fixtures/default-cst-expected.json.
//   node experiments/compare-embedded-cst.mjs
import { readFileSync } from 'node:fs';

import { LinkNetwork, LinkType } from '../src/index.js';

const expected = JSON.parse(readFileSync(new URL('../../parity/fixtures/default-cst-expected.json', import.meta.url)));
const inventory = JSON.parse(readFileSync(new URL('../../parity/language-grammar-inventory.json', import.meta.url)));

for (const language of inventory.languages) {
  const want = expected.languages[language.name];
  if (!want) continue;
  const network = LinkNetwork.parse(language.source, language.name);
  const regions = network.links().filter((link) => link.metadata().linkType === LinkType.Region);
  const actual = regions.map((link) => {
    const m = link.metadata();
    const outer = link.references().slice(2).map((id) => network.link(id).metadata())
      .filter((meta) => meta.named || !['whitespace', 'hidden_text'].includes(meta.term));
    return `${m.language} ${m.span.byteRange.start}-${m.span.byteRange.end} root=${outer.map((o) => `${o.term}@${o.span.byteRange.start}-${o.span.byteRange.end}`).join(',')}`;
  });
  const wanted = want.embedded.map((r) => `${r.language} ${r.startByte}-${r.endByte} root=${r.rows[0][2]}@${r.startByte + r.rows[0][4]}-${r.startByte + r.rows[0][5]} (${r.path})`);
  if (actual.length || wanted.length) {
    console.log(language.name);
    console.log('  actual:   ', actual.join(' | '));
    console.log('  expected: ', wanted.join(' | '));
  }
}
