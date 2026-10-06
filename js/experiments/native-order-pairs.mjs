// Prints, in one line each, every result pair the executor's addResult
// compared on SOURCE whose children start at START and end at END, with the
// order it gave (1 keeps the first, -1 the second, 0 neither). Usage:
// SOURCE='def f := fun x c s => c' START=13 END=18 GRAMMAR=lean node experiments/native-order-pairs.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const grammar = process.env.GRAMMAR ?? 'rust';
const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL(`../../parity/grammars/native/${grammar}.lino`, import.meta.url), 'utf8')));
const source = process.env.SOURCE;
const bytes = Buffer.from(source);
globalThis.__orderTrace = [];
parser.parseTree(source);
const text = (tree) => bytes.subarray(tree.start, tree.end).toString();
const tags = (tree) => ['precedence', 'tail', 'reduced', 'closes'].filter((key) => tree[key]?.level !== undefined).map((key) => `^${key[0]}${tree[key].level}${tree[key].associativity?.[0] ?? ''}`).join('');
const show = (tree) => {
  if (tree.trivia) return '';
  if (tree.type !== 'node') return `${JSON.stringify(text(tree))}${tags(tree)}${tree.alone ? '!' : ''}`;
  return `(${tree.kind}${tags(tree)} ${tree.children.map(show).filter(Boolean).join(' ')})`;
};
const span = (result) => {
  const real = result.children.filter((child) => !child.trivia);
  return [real[0]?.start, real.at(-1)?.end];
};
const [start, end] = [Number(process.env.START ?? 0), Number(process.env.END ?? bytes.length)];
for (const [result, existing, order] of globalThis.__orderTrace) {
  const [a, b] = span(result);
  if (a < start || b > end) continue;
  console.log(`order ${order}\n  + ${result.children.map(show).filter(Boolean).join(' ')}${tags(result)}\n  - ${existing.children.map(show).filter(Boolean).join(' ')}${tags(existing)}`);
}
