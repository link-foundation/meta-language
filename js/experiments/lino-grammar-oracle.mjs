// Compares the built-in LiNo grammar CST with the official links-notation
// parser on the given cases (a JSON file of sources) or built-in samples.
import { readFileSync } from 'node:fs';
import { parse } from 'links-notation/src/parser-generated.js';
import { parseLinoCst } from '../src/lino-grammar.js';

const decode = (text) => {
  const quote = text[0];
  let count = 0;
  while (text[count] === quote) count += 1;
  const inner = text.slice(count, text.length - count);
  return inner.split(quote.repeat(count * 2)).join(quote.repeat(count));
};
const project = (text, node) => {
  if (node.term === 'reference') return { id: text.slice(node.start, node.end), values: [], children: [] };
  if (node.term === 'quoted_reference') return { id: decode(text.slice(node.start, node.end)), values: [], children: [] };
  const id = node.children.find((c) => c.field === 'id');
  return {
    id: id ? project(text, id).id : null,
    values: node.children.filter((c) => c.field === 'value').map((c) => project(text, c)),
    children: node.children.filter((c) => c.field === 'child').map((c) => project(text, c)),
  };
};
const norm = (l) => ({ id: l.id ?? null, values: (l.values ?? []).map(norm), children: (l.children ?? []).map(norm) });
const dump = (text, n, d = 0) => `${'  '.repeat(d)}${n.field ? `${n.field}: ` : ''}${n.term}${n.named ? '' : '*'} [${n.start},${n.end}]${n.children.length ? '' : ` ${JSON.stringify(text.slice(n.start, n.end))}`}\n${n.children.map((c) => dump(text, c, d + 1)).join('')}`;
const cases = process.argv[2] ? JSON.parse(readFileSync(process.argv[2], 'utf8')) : ['1 1 1\n'];
let mismatches = 0;
for (const source of cases) {
  let official;
  try { official = JSON.stringify(parse(source).map(norm)); } catch { official = 'ERR'; }
  const tree = parseLinoCst(source);
  const ours = tree.hasError ? 'ERR' : JSON.stringify(tree.children.filter((c) => c.term === 'link').map((c) => project(source, c)));
  const ok = ours === official;
  if (!ok) mismatches += 1;
  if (!ok || process.env.VERBOSE) console.log(ok ? 'OK  ' : 'DIFF', JSON.stringify(source), '\n  official', official, '\n  ours    ', ours, `\n${dump(source, tree)}`);
}
console.log(`${cases.length} cases, ${mismatches} mismatches`);
