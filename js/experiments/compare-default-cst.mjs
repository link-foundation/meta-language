// Compares the JavaScript public network of every inventory source with the
// grammar-derived rows in parity/fixtures/default-cst-expected.json.
//   node experiments/compare-default-cst.mjs [language]
import { readFileSync } from 'node:fs';

import { LinkNetwork, LinkType } from '../src/index.js';

const expected = JSON.parse(readFileSync(new URL('../../parity/fixtures/default-cst-expected.json', import.meta.url)));
const inventory = JSON.parse(readFileSync(new URL('../../parity/language-grammar-inventory.json', import.meta.url)));
const only = process.argv[2];

const GAP_TERMS = new Set(['whitespace', 'hidden_text']);

function rows(network, language) {
  const links = network.links();
  const byId = new Map(links.map((link) => [String(link.id()), link]));
  const syntax = (link) => link?.metadata().linkType === LinkType.Syntax;
  const gap = (link) => !link.metadata().named && GAP_TERMS.has(link.metadata().term);
  const fields = new Map();
  const referenced = new Set();
  for (const link of links) {
    const m = link.metadata();
    if (m.linkType === LinkType.Field) {
      const [parent, child] = link.references().map(String);
      fields.set(`${parent}:${child}`, m.term);
    }
    if (syntax(link)) for (const ref of link.references()) referenced.add(String(ref));
  }
  const roots = links.filter((link) => syntax(link) && !gap(link) && !referenced.has(String(link.id())));
  const out = [];
  const walk = (link, depth, field) => {
    const m = link.metadata();
    const f = m.flags;
    out.push([depth, field ?? null, m.term, m.named ? 1 : 0, m.span?.byteRange.start, m.span?.byteRange.end,
      `${f.isError ? 'E' : ''}${f.isMissing ? 'M' : ''}${f.isExtra ? 'X' : ''}`]);
    for (const ref of link.references()) {
      const child = byId.get(String(ref));
      if (syntax(child) && !gap(child)) walk(child, depth + 1, fields.get(`${link.id()}:${ref}`));
    }
  };
  return { roots, out, walk };
}

let failures = 0;
for (const language of inventory.languages) {
  const want = expected.languages[language.name];
  if (!want || (only && only !== language.name)) continue;
  for (const [kind, source] of [['positive', language.source], ['recovery', language.recoverySource]]) {
    const network = LinkNetwork.parse(source, language.name);
    const { roots, out, walk } = rows(network, language.name);
    const hostRoots = roots.filter((link) => link.metadata().language === roots.at(-1).metadata().language || true);
    walk(roots.find((r) => r.metadata().language === network.links()[0].metadata().term) ?? roots[0], 0, null);
    const a = out.map((row) => JSON.stringify(row));
    const b = want[kind].map((row) => JSON.stringify(row));
    if (a.join('\n') !== b.join('\n')) {
      failures += 1;
      let i = 0;
      while (a[i] === b[i]) i += 1;
      console.log(`${language.name} ${kind}: first difference at row ${i} (actual ${a.length}, expected ${b.length}) roots=${roots.map((r) => r.metadata().term).join(',')}`);
      console.log(`  actual:   ${a.slice(i, i + 3).join(' | ')}`);
      console.log(`  expected: ${b.slice(i, i + 3).join(' | ')}`);
    }
  }
}
console.log(failures ? `${failures} differences` : 'all equal');
