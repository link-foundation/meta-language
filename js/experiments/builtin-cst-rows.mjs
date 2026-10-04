// Prints the JavaScript public CST rows of the built-in grammar languages
// (those without vendored tree-sitter grammars) for their inventory sources.
import { readFile } from 'node:fs/promises';
import { LinkNetwork, LinkType } from '../src/index.js';

const read = async (p) => JSON.parse(await readFile(new URL(`../../parity/${p}`, import.meta.url), 'utf8'));
const inventory = await read('language-grammar-inventory.json');
const expected = await read('fixtures/default-cst-expected.json');
const only = process.argv[2];

function rows(network) {
  const links = network.links();
  const fields = new Map();
  for (const link of links) {
    if (link.metadata().linkType === LinkType.Field) {
      const [p, c] = link.references();
      fields.set(`${p.asU64()}:${c.asU64()}`, link.metadata().term);
    }
  }
  const syntax = links.filter((l) => l.metadata().linkType === LinkType.Syntax);
  const childIds = new Set(syntax.flatMap((l) => l.references().map((r) => r.asU64())));
  const root = syntax.find((l) => !childIds.has(l.id().asU64()));
  const out = [];
  const visit = (link, depth, field) => {
    const m = link.metadata();
    const f = m.flags;
    out.push([depth, field ?? null, m.term, m.named ? 1 : 0, m.span?.byteRange.start, m.span?.byteRange.end,
      `${f?.isError ? 'E' : ''}${f?.isMissing ? 'M' : ''}${f?.isExtra ? 'X' : ''}`]);
    for (const r of link.references()) {
      const child = network.link(r);
      if (child?.metadata().linkType === LinkType.Syntax) visit(child, depth + 1, fields.get(`${link.id().asU64()}:${r.asU64()}`));
    }
  };
  visit(root, 0);
  return out;
}

const extra = process.argv[3];
for (const language of extra ? [{ name: only, source: JSON.parse(extra) }] : inventory.languages) {
  if (!extra && expected.languages[language.name]) continue;
  if (only && language.name !== only) continue;
  console.log(`## ${language.name} ${JSON.stringify(language.source)}`);
  for (const row of rows(LinkNetwork.parse(language.source, language.name))) console.log(JSON.stringify(row));
}
