// Prints the native recovery of a source after each number of repair rounds,
// to see which repair point each round adds and the tree it leaves.
//   node experiments/repair-rounds.mjs GRAMMAR 'source' [ROUNDS]
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const [grammar, source, rounds = '4'] = process.argv.slice(2);
const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL(`../../parity/grammars/native/${grammar}.lino`, import.meta.url), 'utf8')));
const render = (tree) => tree.type === 'node'
  ? `(${tree.kind ?? '_'}${tree.children.map((child) => ` ${render(child)}`).join('')})`
  : tree.type === 'token' ? (tree.kind ? `${tree.kind}:${JSON.stringify(source.slice(tree.start, tree.end))}` : JSON.stringify(source.slice(tree.start, tree.end)))
    : `${tree.type.toUpperCase()}:${tree.start}-${tree.end}${tree.kind ? `:${tree.kind}` : ''}`;
for (let maxRepairs = 1; maxRepairs <= Number(rounds); maxRepairs += 1) {
  const outcome = parser.parseTree(source, { errorRecovery: true, maxRepairs });
  console.log(maxRepairs, outcome.ok, outcome.tree ? render(outcome.tree).replace(/ "[ \n]+"/gu, '') : outcome.rejection);
}
