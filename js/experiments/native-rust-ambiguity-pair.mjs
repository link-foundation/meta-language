// Prints the two results the native Rust grammar's executor finds equal when
// it marks a result ambiguous on SOURCE, as indented trees, to see where the
// two parses part (the trace in addResult of src/grammar-runtime/executor.js).
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';

globalThis.__ambiguityPairs = [];
const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL('../../parity/grammars/native/rust.lino', import.meta.url), 'utf8')));
const source = process.env.SOURCE;
parser.parseTree(source);
const bytes = Buffer.from(source);
const show = (tree, depth = 0) => {
  if (tree.trivia) return [];
  const text = tree.type === 'node' ? '' : ` ${JSON.stringify(bytes.subarray(tree.start, tree.end).toString())}`;
  const tags = ['precedence', 'tail', 'reduced'].filter((key) => tree[key]).map((key) => ` ${key}=${tree[key].level}${tree[key].associativity[0]}`).join('');
  return [`${'  '.repeat(depth)}${tree.type} ${tree.kind ?? ''} ${tree.start}-${tree.end}${text}${tags}`, ...(tree.type === 'node' ? tree.children.flatMap((child) => show(child, depth + 1)) : [])];
};
for (const [result, existing] of globalThis.__ambiguityPairs.slice(0, Number(process.env.LIMIT ?? 2))) {
  console.log('--- result');
  for (const child of result.children) console.log(show(child).join('\n'));
  console.log('--- existing');
  for (const child of existing.children) console.log(show(child).join('\n'));
}
