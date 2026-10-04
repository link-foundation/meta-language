// Differential fuzz of the built-in LiNo grammar CST against the official
// links-notation parser: identical accept/reject, identical links, lossless leaves.
import { writeFileSync } from 'node:fs';
import { parse } from 'links-notation/src/parser-generated.js';
import { parseLinoCst } from '../src/lino-grammar.js';

const alphabet = ['a', 'b', 'c', ' ', ' ', ' ', '\t', '\n', '\n', '\r', '(', ')', ':', '"', "'", '`', 'é'];
let seed = Number(process.argv[3] ?? 1);
const random = () => ((seed = (seed * 1103515245 + 12345) % 2147483648) / 2147483648);
const count = Number(process.argv[2] ?? 20000);
const decode = (text) => {
  const quote = text[0];
  let n = 0;
  while (text[n] === quote) n += 1;
  return text.slice(n, text.length - n).split(quote.repeat(n * 2)).join(quote.repeat(n));
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
const leaves = (n) => (n.children.length === 0 && n.term !== 'lino_document' ? [n] : n.children.flatMap(leaves));
const failures = [];
let accepted = 0;
for (let i = 0; i < count; i += 1) {
  const length = Math.floor(random() * 24);
  let source = '';
  for (let j = 0; j < length; j += 1) source += alphabet[Math.floor(random() * alphabet.length)];
  let official;
  try { official = JSON.stringify(parse(source).map(norm)); accepted += 1; } catch { official = 'ERR'; }
  const tree = parseLinoCst(source);
  const ours = tree.hasError ? 'ERR' : JSON.stringify(tree.children.filter((c) => c.term === 'link').map((c) => project(source, c)));
  const text = leaves(tree).map((n) => source.slice(n.start, n.end)).join('');
  if (ours !== official || text !== source) failures.push({ source, official, ours, lossless: text === source });
}
writeFileSync('/tmp/lino-fuzz-failures.json', JSON.stringify(failures, null, 1));
console.log(`${count} cases, ${accepted} accepted by the official parser, ${failures.length} failures`);
