// Prints the ERROR and MISSING sites of the tree-sitter oracle's parse of each
// file, next to the native sites experiments/native-kind-sites.mjs prints.
//   node experiments/oracle-error-sites.mjs LANGUAGE FILE...
import { readFileSync } from 'node:fs';
import { gunzipSync } from 'node:zlib';
import { Parser } from '../js/node_modules/web-tree-sitter/web-tree-sitter.js';

import { languageEntry } from '../js/src/index.js';
import { loadGrammarLanguage } from '../js/src/grammar-tiering.js';
import { grammarFile } from '../js/scripts/grammar-files.mjs';

const [language, ...files] = process.argv.slice(2);
await Parser.init();
const lock = JSON.parse(readFileSync(new URL('../js/src/vendor/grammars/grammar-lock.json', import.meta.url), 'utf8'));
const entry = languageEntry(language);
const id = (entry.oracleGrammars ?? entry.grammars).map((grammar) => grammar.id ?? grammar).find((name) => lock.grammars[name]);
const parser = new Parser();
parser.setLanguage(loadGrammarLanguage(gunzipSync(readFileSync(new URL(`../${grammarFile(lock.grammars[id], `${id}.wasm.gz`)}`, import.meta.url)))));
for (const file of files) {
  const source = readFileSync(file, 'utf8');
  const tree = parser.parse(source);
  const sites = [];
  const walk = (node) => {
    if (node.isError || node.isMissing) sites.push([node.type, node.startIndex, node.endIndex, source.slice(Math.max(0, node.startIndex - 40), node.endIndex + 40)]);
    node.children.forEach(walk);
  };
  walk(tree.rootNode);
  console.log(JSON.stringify({ file, oracle: id, error: tree.rootNode.hasError, sites: sites.slice(0, 5) }));
  tree.delete();
}
