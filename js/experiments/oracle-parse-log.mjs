// Prints the parse log of a pinned tree-sitter oracle (web-tree-sitter's
// setLogger: every lex, shift, reduce, merge and select of the GLR parse) for
// one source, to see how the oracle resolves a conflict the native grammar
// ranks otherwise.
//   node experiments/oracle-parse-log.mjs Lean '(do return x : T)'
import { readFileSync } from 'node:fs';
import { gunzipSync } from 'node:zlib';
import { Parser } from 'web-tree-sitter';

import { languageEntry } from '../src/index.js';
import { loadGrammarLanguage } from '../src/grammar-tiering.js';
import { grammarFile } from '../scripts/grammar-files.mjs';

const [language, source] = process.argv.slice(2);
const lock = JSON.parse(readFileSync(new URL('../src/vendor/grammars/grammar-lock.json', import.meta.url), 'utf8'));
const entry = languageEntry(language);
const id = (entry.oracleGrammars ?? entry.grammars)[0].id;
const file = new URL(`../../${grammarFile(lock.grammars[id], `${id}.wasm.gz`)}`, import.meta.url);
const parser = new Parser();
parser.setLanguage(loadGrammarLanguage(gunzipSync(readFileSync(file))));
parser.setLogger((message, params, type) => {
  const details = Object.entries(params ?? {}).map(([key, value]) => `${key}:${value}`).join(' ');
  console.log(`${type === 1 ? 'lex' : 'parse'} ${message}${details ? ` ${details}` : ''}`);
});
const tree = parser.parse(source);
console.log(tree.rootNode.toString());
tree.delete();
parser.delete();
