// Parses JavaScript sources that end in a bare `class` with the native grammar.
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';

const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL('../../parity/grammars/native/javascript.lino', import.meta.url), 'utf8')));
const show = (tree, depth = 0) => {
  const label = tree.type === 'node' ? tree.rule ?? tree.kind : `${tree.kind ?? ''} ${JSON.stringify(tree.text)}`;
  console.log(`${'  '.repeat(depth)}${label}${tree.scanned ? ' scanned' : ''}`);
  if (tree.type === 'node') for (const child of tree.children) show(child, depth + 1);
};
for (const source of process.argv.slice(2).length ? process.argv.slice(2) : ['class', 'x\nclass', '\nfunction foo() {}\nclass']) {
  const outcome = parser.parseTree(source);
  console.log(JSON.stringify(source), 'ok', outcome.ok, outcome.rejection ? JSON.stringify(outcome.rejection).slice(0, 200) : '');
  if (outcome.ok) show(outcome.tree);
}
