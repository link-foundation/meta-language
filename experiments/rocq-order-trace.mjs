// Prints the result pairs addResult compared on SOURCE where one side has a
// node of kind KIND and the other none, with the order it gave, to find why
// the native Rocq grammar keeps a `custom_operation` over `2 + 2 = 4`.
//   SOURCE='Check 2 + 2 = 4.' KIND=custom_operation [SPAN=5-15] node experiments/rocq-order-trace.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../js/src/index.js';

const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL('../parity/grammars/native/rocq.lino', import.meta.url), 'utf8')));
const source = process.env.SOURCE ?? 'Check 2 + 2 = 4.';
const kind = process.env.KIND ?? 'custom_operation';
const bytes = Buffer.from(source);
globalThis.__orderTrace = [];
parser.parseTree(source);
const text = (tree) => bytes.subarray(tree.start, tree.end).toString();
const show = (tree, depth = 0) => {
  if (tree.trivia) return [];
  const tags = ['precedence', 'tail', 'reduced'].filter((key) => tree[key]).map((key) => ` ${key}=${tree[key].level}${tree[key].name ?? ''}${tree[key].associativity?.[0]}`).join('');
  const own = tree.type === 'node' ? '' : ` ${JSON.stringify(text(tree))}`;
  return [`${'  '.repeat(depth)}${tree.type} ${tree.kind ?? ''} ${tree.start}-${tree.end}${own}${tags}`, ...(tree.type === 'node' ? tree.children.flatMap((child) => show(child, depth + 1)) : [])];
};
// SPAN=start-end narrows KIND to a node of that span.
const [low, high] = (process.env.SPAN ?? '').split('-').map(Number);
const has = (children) => children.some((tree) => tree.type === 'node'
  && ((tree.kind === kind && (!process.env.SPAN || (tree.start === low && tree.end === high))) || has(tree.children)));
let shown = 0;
for (const [result, existing, order] of globalThis.__orderTrace) {
  if (has(result.children) === has(existing.children)) continue;
  if (shown++ >= Number(process.env.LIMIT ?? 3)) break;
  console.log(`=== order ${order} end ${result.end}`);
  console.log('--- result');
  for (const child of result.children) console.log(show(child).join('\n'));
  console.log('--- existing');
  for (const child of existing.children) console.log(show(child).join('\n'));
}
