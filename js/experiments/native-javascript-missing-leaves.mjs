// Prints the public CST lines and the native parse tree of JavaScript sources
// whose recovery inserts MISSING leaves, to compare the runtimes.
//   node experiments/native-javascript-missing-leaves.mjs [SOURCE...]
import { readFileSync } from 'node:fs';

import { LinkNetwork, compileGrammar, parseGrammarLinks } from '../src/index.js';
import { documentGrammarRoots, renderCstLines } from '../tests/support/cst-lines.js';

const sources = process.argv.length > 2 ? process.argv.slice(2) : ['\n/* a', '\n/*\n * @return {void}\n */\nfunc\nfunction foo() {}\nclass'];
const links = readFileSync(new URL('../../parity/grammars/native/javascript.lino', import.meta.url), 'utf8');
const parser = compileGrammar(parseGrammarLinks(links));
const show = (tree, depth = 0) => {
  const pad = '  '.repeat(depth);
  if (tree.type === 'node') {
    console.log(`${pad}${tree.kind ?? '(anon)'}${tree.missing ? ' MISSING' : ''}`);
    for (const child of tree.children) show(child, depth + 1);
  } else {
    console.log(`${pad}${tree.type} ${JSON.stringify(tree.kind ?? null)} ${JSON.stringify(tree.text)}${tree.missing ? ' MISSING' : ''}`);
  }
};
for (const source of sources) {
  console.log('==', JSON.stringify(source));
  console.log(renderCstLines(documentGrammarRoots(LinkNetwork.parse(source, 'JavaScript'), 'JavaScript'), 'JavaScript').text);
  show(parser.parseTree(source, { errorRecovery: true }).tree);
}
