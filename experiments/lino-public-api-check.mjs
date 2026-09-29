import { LinkNetwork } from '../js/src/index.js';
const m = JSON.parse(await (await import('node:fs/promises')).readFile('../parity/fixtures/lino-compatibility-matrix.json','utf8'));
let bad=0;
for (const f of m.features) { const {network, links} = LinkNetwork.parseLinksNotation(f.source); const r = links.map(id=>network.linksNotationReading(id)); if (JSON.stringify(r)!==JSON.stringify(f.links)) {bad++; console.log(f.id, JSON.stringify(r), JSON.stringify(f.links));} }
console.log('bad', bad);
// Every feature's links encode to text the official parser and ours read back
// as the same links.
const { Parser } = await import('../js/node_modules/links-notation/src/index.js');
const canonical = (link) => (link.values.length === 0 && link.id !== null
  ? link.id : [link.id, ...link.values.map(canonical)]);
let badEncode = 0;
const tricky = ['a b', '"x', "'y\"", '`\'"z', '#h', '', 'p:q', 'a\nb', 'x""y'];
const extra = { id: 'tricky', source: `(t: ${tricky.map((n) => `'${n.replaceAll("'", "''")}'`).join(' ')})\n` };
for (const f of [...m.features, extra]) {
  const {network, links} = LinkNetwork.parseLinksNotation(f.source);
  const encoded = network.linksNotationText(links);
  const expected = links.map((id) => network.linksNotationReading(id));
  const official = new Parser().parse(encoded).map(canonical);
  const again = LinkNetwork.parseLinksNotation(encoded);
  const ours = again.links.map((id) => again.network.linksNotationReading(id));
  if (JSON.stringify(official) !== JSON.stringify(expected) || JSON.stringify(ours) !== JSON.stringify(expected)) {
    badEncode++; console.log('ENCODE', f.id, JSON.stringify(encoded), JSON.stringify(expected), JSON.stringify(official), JSON.stringify(ours));
  }
}
console.log('badEncode', badEncode);
