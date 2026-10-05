// Prints the tree-sitter oracle's parse log (its forks, merges and the trees
// it selects) and tree for a source, to see how it settles an ambiguity.
//   node experiments/oracle-parse-log.mjs LANGUAGE SOURCE [FILTER]
// SOURCE reads `\n` as a line feed; FILTER keeps the log lines holding it.
import { readFileSync } from 'node:fs';
import { gunzipSync } from 'node:zlib';
import { Parser } from '../js/node_modules/web-tree-sitter/web-tree-sitter.js';

import { languageEntry } from '../js/src/index.js';
import { loadGrammarLanguage } from '../js/src/grammar-tiering.js';
import { grammarFile } from '../js/scripts/grammar-files.mjs';

const [language, raw, filter = ''] = process.argv.slice(2);
const source = raw.replace(/\\n/gu, '\n');
await Parser.init();
const lock = JSON.parse(readFileSync(new URL('../js/src/vendor/grammars/grammar-lock.json', import.meta.url), 'utf8'));
const entry = languageEntry(language);
const id = (entry.oracleGrammars ?? entry.grammars).map((grammar) => grammar.id ?? grammar).find((name) => lock.grammars[name]);
const parser = new Parser();
parser.setLanguage(loadGrammarLanguage(gunzipSync(readFileSync(new URL(`../${grammarFile(lock.grammars[id], `${id}.wasm.gz`)}`, import.meta.url)))));
parser.setLogger((message, type) => { if (message.includes(filter)) console.log(`${type}: ${message}`); });
const tree = parser.parse(source);
console.log(tree.rootNode.toString());
tree.delete();
