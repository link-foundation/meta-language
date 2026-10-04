// Prints the result pairs the executor's addResult compared on SOURCE whose
// leaves include a token of text TEXT on one side only, with the order it
// gave (1 keeps the first, -1 the second, 0 neither), to find the comparison
// that drops a parse. Usage:
// SOURCE='fn f() { .. ..=.. ..; }' TEXT='..=' [GRAMMAR=rust] node experiments/native-order-trace.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const grammar = process.env.GRAMMAR ?? 'rust';
const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL(`../../parity/grammars/native/${grammar}.lino`, import.meta.url), 'utf8')));
const source = process.env.SOURCE;
const bytes = Buffer.from(source);
globalThis.__orderTrace = [];
parser.parseTree(source);
const leaves = (children) => children.flatMap((tree) => (tree.type === 'node' ? leaves(tree.children) : [tree]));
const text = (tree) => bytes.subarray(tree.start, tree.end).toString();
const show = (tree, depth = 0) => {
  if (tree.trivia) return [];
  const tags = ['precedence', 'tail', 'reduced'].filter((key) => tree[key]).map((key) => ` ${key}=${tree[key].level}${tree[key].associativity[0]}`).join('');
  const own = tree.type === 'node' ? '' : ` ${JSON.stringify(text(tree))}`;
  return [`${'  '.repeat(depth)}${tree.type} ${tree.kind ?? ''} ${tree.start}-${tree.end}${own}${tags}`, ...(tree.type === 'node' ? tree.children.flatMap((child) => show(child, depth + 1)) : [])];
};
const has = (children) => leaves(children).some((leaf) => text(leaf) === process.env.TEXT);
let shown = 0;
for (const [result, existing, order] of globalThis.__orderTrace) {
  if (has(result.children) === has(existing.children)) continue;
  if (shown++ >= Number(process.env.LIMIT ?? 3)) break;
  console.log(`=== order ${order}`);
  console.log('--- result');
  for (const child of result.children) console.log(show(child).join('\n'));
  console.log('--- existing');
  for (const child of existing.children) console.log(show(child).join('\n'));
}
