// Prints the rejection offset and the repairs of the native parse of each
// source argument under a native grammar (`\n` escapes a line break).
//   node experiments/merged-lex-snippets.mjs <grammar-id> <source>...
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../js/src/index.js';

const [id, ...sources] = process.argv.slice(2);
const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL(`../parity/grammars/native/${id}.lino`, import.meta.url), 'utf8')));
const leaves = (tree) => (!tree ? [] : tree.type === 'node' ? tree.children.flatMap(leaves) : [tree]);
for (const raw of sources) {
  const source = raw.replaceAll('\\n', '\n');
  const outcome = parser.parseTree(source);
  const recovered = parser.parseTree(source, { errorRecovery: true, recovery: 'accept' }).tree;
  const repairs = leaves(recovered).filter(({ type }) => type !== 'token').map(({ type, start, end }) => `${type}@${start}-${end}`);
  const tokens = leaves(recovered).filter(({ type, text }) => type === 'token' && text.trim()).map(({ kind, text }) => kind ? `${kind}:${text}` : text);
  console.log(JSON.stringify(source), outcome.ok, outcome.rejection?.offset, repairs, tokens.join(' '));
}
