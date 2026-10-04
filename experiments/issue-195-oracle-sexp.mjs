// Prints the tree-sitter oracle s-expression of each source argument
// (`\n` escapes a line break) under the oracle grammar of a language.
//   node experiments/issue-195-oracle-sexp.mjs <language> <source>...
import { readFileSync } from 'node:fs';
import { gunzipSync } from 'node:zlib';
import { Parser } from '../js/node_modules/web-tree-sitter/web-tree-sitter.js';
import { languageEntry } from '../js/src/index.js';
import { loadGrammarLanguage } from '../js/src/grammar-tiering.js';
import { grammarFile } from '../js/scripts/grammar-files.mjs';

const [language, ...sources] = process.argv.slice(2);
await Parser.init();
const lock = JSON.parse(readFileSync(new URL('../js/src/vendor/grammars/grammar-lock.json', import.meta.url), 'utf8'));
const entry = languageEntry(language);
const id = (entry.oracleGrammars ?? entry.grammars)[0].id;
const parser = new Parser();
parser.setLanguage(loadGrammarLanguage(gunzipSync(readFileSync(new URL(`../${grammarFile(lock.grammars[id], `${id}.wasm.gz`)}`, import.meta.url)))));
for (const source of sources) console.log(parser.parse(source.replaceAll('\\n', '\n')).rootNode.toString());
