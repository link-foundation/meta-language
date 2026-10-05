// Prints the MISSING and ERROR leaves of the raw native executor tree of each
// file, before the public projection hides any kind: a probe for repairs a
// projection would otherwise mask (a MISSING `unnamed_token`).
//   node experiments/native-raw-missing.mjs LANGUAGE FILE...
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../js/src/index.js';
import { nativeGrammarText } from '../js/src/native-grammar-parser.js';

const [language, ...files] = process.argv.slice(2);
const parser = compileGrammar(parseGrammarLinks(nativeGrammarText(`native-${language}`)));
for (const file of files) {
  const source = readFileSync(file, 'utf8');
  const { tree } = parser.parseTree(source, { errorRecovery: true, recovery: 'accept' });
  const found = [];
  const walk = (node) => {
    if (node.type === 'missing' || node.type === 'error') found.push([node.type, node.kind ?? null, node.start, node.end]);
    for (const child of node.children ?? []) walk(child);
  };
  if (tree) walk(tree);
  console.log(JSON.stringify({ file, repairs: found }));
}
