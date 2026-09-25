// Compares JavaScript public CST rows with parity/fixtures/default-cst-expected.json.
import { readFile } from 'node:fs/promises';
import { LinkNetwork, LinkType } from '../src/index.js';

const read = async (p) => JSON.parse(await readFile(new URL(`../../parity/${p}`, import.meta.url), 'utf8'));
const inventory = await read('language-grammar-inventory.json');
const expected = await read('fixtures/default-cst-expected.json');

function rows(network) {
  const links = network.links();
  const fields = new Map();
  for (const link of links) {
    const m = link.metadata();
    if (m.linkType === LinkType.Field && link.references().length === 2) {
      const [p, c] = link.references();
      fields.set(`${p.asU64()}:${c.asU64()}`, m.term);
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
      if (process.env.SKIP_GAPS && ['whitespace', 'hidden_text'].includes(child?.metadata().term) && !child.metadata().named) continue;
      if (child?.metadata().linkType === LinkType.Syntax) visit(child, depth + 1, fields.get(`${link.id().asU64()}:${r.asU64()}`));
    }
  };
  visit(root, 0);
  return out;
}

for (const language of inventory.languages) {
  const want = expected.languages[language.name];
  if (!want) continue;
  for (const [kind, key] of [['positive', 'source'], ['recovery', 'recoverySource']]) {
    const got = rows(LinkNetwork.parse(language[key], language.name));
    const exp = publicRows(language.name, language[key], want[kind]);
    const i = got.findIndex((row, index) => JSON.stringify(row) !== JSON.stringify(exp[index]));
    if (i !== -1 || got.length !== exp.length) {
      console.log(language.name, kind, got.length, exp.length, i, JSON.stringify(got.slice(i, i + 2)), JSON.stringify(exp.slice(i, i + 2)));
    }
  }
}

function publicRows(language, source, rows) {
  const bytes = new TextEncoder().encode(source);
  if (language === 'Lean') {
    return [[0, null, 'file', 1, 0, bytes.length, rows[0]?.[6] ?? ''], ...rows.map((row) => [row[0] + 1, ...row.slice(1)])];
  }
  if (language === 'Rocq') {
    return rows.flatMap((row) => {
      if (row[2] !== 'ident') return [row];
      const text = new TextDecoder().decode(bytes.subarray(row[4], row[5]));
      const term = ['bool', 'nat', 'Prop', 'Set', 'SProp', 'Type', 'Z'].includes(text) ? 'primitive_type' : 'identifier';
      return [row, [row[0] + 1, null, term, 1, row[4], row[5], '']];
    });
  }
  return rows;
}
