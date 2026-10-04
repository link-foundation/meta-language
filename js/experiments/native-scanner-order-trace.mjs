// Prints the result pairs the executor's addResult compared on SOURCE where
// one side has more leaves of the token kind KIND (a scanner token, which may
// be zero-width, so a text filter cannot find it), with the order it gave.
// Usage:
// SOURCE=$'return\nx' KIND=automatic_semicolon GRAMMAR=javascript node experiments/native-scanner-order-trace.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const grammar = process.env.GRAMMAR ?? 'javascript';
const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL(`../../parity/grammars/native/${grammar}.lino`, import.meta.url), 'utf8')));
const source = process.env.SOURCE;
globalThis.__orderTrace = [];
parser.parseTree(source);
const leaves = (children) => children.flatMap((tree) => (tree.type === 'node' ? leaves(tree.children) : [tree]));
const show = (tree, depth = 0) => (tree.trivia ? [] : [
  `${'  '.repeat(depth)}${tree.type} ${tree.kind ?? ''} ${tree.start}-${tree.end}`,
  ...(tree.type === 'node' ? tree.children.flatMap((child) => show(child, depth + 1)) : []),
]);
const count = (children) => leaves(children).filter((leaf) => leaf.kind === process.env.KIND).length;
let shown = 0;
for (const [result, existing, order] of globalThis.__orderTrace) {
  if (count(result.children) === count(existing.children)) continue;
  if (shown++ >= Number(process.env.LIMIT ?? 3)) break;
  console.log(`=== order ${order} end ${result.end}`);
  console.log('--- result');
  console.log(result.children.flatMap((child) => show(child)).join('\n'));
  console.log('--- existing');
  console.log(existing.children.flatMap((child) => show(child)).join('\n'));
}
console.log(`${shown} pairs shown of ${globalThis.__orderTrace.length} compared`);
