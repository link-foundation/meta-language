// Prints the grammar CST (named nodes, fields, text) for inventory sources.
// Usage: node experiments/inventory-tree-dump.mjs [language] [source]
import { readFile } from 'node:fs/promises';
import { LinkNetwork, LinkType } from '../src/index.js';

const inventory = JSON.parse(await readFile(new URL('../../parity/language-grammar-inventory.json', import.meta.url), 'utf8'));
const [only, override] = process.argv.slice(2);
for (const { name, source } of inventory.languages) {
  if (only && only !== name) continue;
  const text = override ?? source;
  const network = LinkNetwork.parse(text, name);
  const links = network.links();
  const fields = new Map();
  for (const link of links.filter((l) => l.metadata().linkType === LinkType.Field)) {
    const [, child] = link.references();
    fields.set(child.asU64(), link.metadata().term);
  }
  const bytes = Buffer.from(text);
  const childIds = new Set();
  for (const link of links) if (link.metadata().linkType === LinkType.Syntax) for (const r of link.references()) childIds.add(r.asU64());
  const print = (link, depth) => {
    const m = link.metadata();
    if (m.linkType !== LinkType.Syntax) return;
    const s = m.span?.byteRange;
    const t = s ? bytes.subarray(s.start, s.end).toString() : '';
    const field = fields.get(link.id().asU64());
    const flags = [m.flags.isError && 'ERROR', m.flags.isMissing && 'MISSING', m.flags.isExtra && 'extra'].filter(Boolean).join(',');
    console.log(`${'  '.repeat(depth)}${field ? field + ': ' : ''}${m.named ? m.term : JSON.stringify(m.term)} ${flags} ${JSON.stringify(t)}`);
    for (const r of link.references()) { const c = network.link(r); if (c) print(c, depth + 1); }
  };
  console.log(`=== ${name}`);
  for (const link of links) if (link.metadata().linkType === LinkType.Syntax && !childIds.has(link.id().asU64())) print(link, 0);
}
