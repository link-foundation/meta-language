// Prints every result pair the executor's addResult compared on SOURCE whose
// two sides differ, with the order it gave (1 keeps the first, -1 the
// second, 0 neither), to find the comparison that picks a parse the oracle
// does not. Usage:
// SOURCE='#check foo 2 3' GRAMMAR=lean [LIMIT=10] node experiments/native-order-pairs.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const grammar = process.env.GRAMMAR ?? 'lean';
const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL(`../../parity/grammars/native/${grammar}.lino`, import.meta.url), 'utf8')));
const source = process.env.SOURCE;
const bytes = Buffer.from(source);
globalThis.__orderTrace = [];
parser.parseTree(source);
const text = (tree) => bytes.subarray(tree.start, tree.end).toString();
const show = (tree, depth = 0) => {
  if (tree.trivia) return [];
  const tags = ['precedence', 'tail', 'reduced', 'closes'].filter((key) => tree[key]).map((key) => ` ${key}=${tree[key].level}${tree[key].associativity[0]}`).join('');
  const to = tree.reducedTo ? ` to=${tree.reducedTo.join(',')}` : '';
  const own = tree.type === 'node' ? '' : ` ${JSON.stringify(text(tree))}`;
  return [`${'  '.repeat(depth)}${tree.type} ${tree.kind ?? ''}${tree.field ? `:${tree.field}` : ''} ${tree.start}-${tree.end}${own}${tags}${to}`, ...(tree.type === 'node' ? tree.children.flatMap((child) => show(child, depth + 1)) : [])];
};
let shown = 0;
for (const [result, existing, order] of globalThis.__orderTrace) {
  const [a, b] = [result, existing].map((side) => side.children.flatMap((child) => show(child)).join('\n'));
  if (a === b) continue;
  if (shown++ >= Number(process.env.LIMIT ?? 10)) break;
  console.log(`=== order ${order}`);
  console.log(`--- result\n${a}`);
  console.log(`--- existing\n${b}`);
}
