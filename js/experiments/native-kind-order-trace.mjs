// Prints the result pairs the executor's addResult compared on SOURCE where a
// node of kind KIND is on one side only, with the order it gave (1 keeps the
// first, -1 the second, 0 neither). Usage:
// SOURCE='function f() { revert(x); }' KIND=parenthesized_expression GRAMMAR=solidity node experiments/native-kind-order-trace.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const grammar = process.env.GRAMMAR ?? 'rust';
const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL(`../../parity/grammars/native/${grammar}.lino`, import.meta.url), 'utf8')));
const source = process.env.SOURCE;
const bytes = Buffer.from(source);
globalThis.__orderTrace = [];
parser.parseTree(source);
const text = (tree) => bytes.subarray(tree.start, tree.end).toString();
const show = (tree, depth = 0) => {
  if (tree.trivia) return [];
  const tags = ['precedence', 'tail', 'reduced'].filter((key) => tree[key]).map((key) => ` ${key}=${tree[key].level}${tree[key].associativity[0]}`).join('');
  const own = tree.type === 'node' ? '' : ` ${JSON.stringify(text(tree))}`;
  return [`${'  '.repeat(depth)}${tree.type} ${tree.kind ?? ''} ${tree.start}-${tree.end}${own}${tags}`, ...(tree.type === 'node' ? tree.children.flatMap((child) => show(child, depth + 1)) : [])];
};
const has = (children) => children.some((tree) => tree.kind === process.env.KIND || (tree.type === 'node' && has(tree.children)));
let shown = 0;
for (const [result, existing, order] of globalThis.__orderTrace) {
  if (has(result.children) === has(existing.children)) continue;
  if (shown++ >= Number(process.env.LIMIT ?? 3)) break;
  console.log(`=== order ${order} precedence ${JSON.stringify(result.precedence)} / ${JSON.stringify(existing.precedence)}`);
  console.log('--- result');
  for (const child of result.children) console.log(show(child).join('\n'));
  console.log('--- existing');
  for (const child of existing.children) console.log(show(child).join('\n'));
}
