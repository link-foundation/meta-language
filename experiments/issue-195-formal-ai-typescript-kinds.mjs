// Compares the named node kinds of formal-ai's ts/ corpus under the native
// TypeScript grammar and the tree-sitter-typescript oracle: the inventory
// formal-ai's grammar_projection_corpus_ratchet measures.
//   node experiments/issue-195-formal-ai-typescript-kinds.mjs <formal-ai checkout> [max bytes]
import { readFileSync, readdirSync, statSync } from 'node:fs';
import { join } from 'node:path';
import { gunzipSync } from 'node:zlib';
import { Parser } from '../js/node_modules/web-tree-sitter/web-tree-sitter.js';

import { languageEntry } from '../js/src/index.js';
import { parseNative } from '../js/src/native-grammar-parser.js';
import { loadGrammarLanguage } from '../js/src/grammar-tiering.js';
import { grammarFile } from '../js/scripts/grammar-files.mjs';

const [checkout, maxBytes = '1000000000'] = process.argv.slice(2);
const files = [];
const walk = (dir) => readdirSync(dir).forEach((name) => {
  const path = join(dir, name);
  if (statSync(path).isDirectory()) walk(path);
  else if (path.endsWith('.ts') && statSync(path).size <= Number(maxBytes)) files.push(path);
});
walk(join(checkout, 'ts'));
await Parser.init();
const lock = JSON.parse(readFileSync(new URL('../js/src/vendor/grammars/grammar-lock.json', import.meta.url), 'utf8'));
const entry = languageEntry('typescript');
const id = (entry.oracleGrammars ?? entry.grammars)[0].id;
const parser = new Parser();
parser.setLanguage(loadGrammarLanguage(gunzipSync(readFileSync(new URL(`../${grammarFile(lock.grammars[id], `${id}.wasm.gz`)}`, import.meta.url)))));
const nativeId = Object.keys(JSON.parse(readFileSync(new URL('../parity/language-grammar-inventory.json', import.meta.url), 'utf8')).nativeGrammars).find((key) => key === 'native-typescript');

const count = (map, kind) => map.set(kind, (map.get(kind) ?? 0) + 1);
for (const file of files.sort()) {
  const source = readFileSync(file, 'utf8');
  const oracle = new Map();
  const tree = parser.parse(source);
  const walkOracle = (node) => { if (node.isNamed) count(oracle, node.type); node.children.forEach(walkOracle); };
  walkOracle(tree.rootNode);
  const oracleError = tree.rootNode.hasError;
  tree.delete();
  const native = new Map();
  const started = Date.now();
  const root = parseNative(nativeId, source);
  const walkNative = (node) => { if (node.named) count(native, node.term); node.children.forEach(({ node: child }) => walkNative(child)); };
  walkNative(root);
  const only = (a, b) => [...a.keys()].filter((kind) => !b.has(kind));
  console.log(JSON.stringify({
    file: file.slice(checkout.length + 1), bytes: source.length, ms: Date.now() - started,
    oracleError, nativeError: root.hasError, nativeOnly: only(native, oracle), oracleOnly: only(oracle, native),
  }));
}
