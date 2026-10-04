// Prints where the tree-sitter-typescript oracle reports ERROR or MISSING
// nodes in one file, with the surrounding source.
//   node experiments/issue-195-oracle-error-sites.mjs <file> [grammar id]
import { readFileSync } from 'node:fs';
import { gunzipSync } from 'node:zlib';
import { Parser } from '../js/node_modules/web-tree-sitter/web-tree-sitter.js';
import { languageEntry } from '../js/src/index.js';
import { loadGrammarLanguage } from '../js/src/grammar-tiering.js';
import { grammarFile } from '../js/scripts/grammar-files.mjs';

const [file, language = 'typescript'] = process.argv.slice(2);
await Parser.init();
const lock = JSON.parse(readFileSync(new URL('../js/src/vendor/grammars/grammar-lock.json', import.meta.url), 'utf8'));
const entry = languageEntry(language);
const id = (entry.oracleGrammars ?? entry.grammars)[0].id;
const parser = new Parser();
parser.setLanguage(loadGrammarLanguage(gunzipSync(readFileSync(new URL(`../${grammarFile(lock.grammars[id], `${id}.wasm.gz`)}`, import.meta.url)))));
const source = readFileSync(file, 'utf8');
const tree = parser.parse(source);
const walk = (node) => {
  if (node.type === 'ERROR' || node.isMissing) {
    console.log(JSON.stringify({ type: node.type, missing: node.isMissing, start: node.startIndex, text: source.slice(Math.max(0, node.startIndex - 60), node.startIndex + 60) }));
    return;
  }
  node.children.forEach(walk);
};
walk(tree.rootNode);
