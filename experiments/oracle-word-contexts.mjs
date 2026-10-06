// Prints, for each file under a directory, every leaf whose text is one of the
// given words under the tree-sitter oracle of a language, with its parent kind
// and line: an oracle-only scan, cheap on whole files.
//   node experiments/oracle-word-contexts.mjs <language> <directory> <extension> <word>...
import { readFileSync, readdirSync, statSync } from 'node:fs';
import { join } from 'node:path';
import { gunzipSync } from 'node:zlib';
import { Parser } from '../js/node_modules/web-tree-sitter/web-tree-sitter.js';
import { languageEntry } from '../js/src/index.js';
import { loadGrammarLanguage } from '../js/src/grammar-tiering.js';
import { grammarFile } from '../js/scripts/grammar-files.mjs';

const [language, directory, extension, ...words] = process.argv.slice(2);
await Parser.init();
const lock = JSON.parse(readFileSync(new URL('../js/src/vendor/grammars/grammar-lock.json', import.meta.url), 'utf8'));
const entry = languageEntry(language);
const id = (entry.oracleGrammars ?? entry.grammars)[0].id;
const parser = new Parser();
parser.setLanguage(loadGrammarLanguage(gunzipSync(readFileSync(new URL(`../${grammarFile(lock.grammars[id], `${id}.wasm.gz`)}`, import.meta.url)))));
const parseBytes = (text) => {
  const bytes = Buffer.from(text, 'utf8');
  return parser.parse((index) => String.fromCharCode(...bytes.subarray(index, Math.min(bytes.length, index + 4096))));
};
const files = (dir) => readdirSync(dir).flatMap((name) => {
  const path = join(dir, name);
  return statSync(path).isDirectory() ? files(path) : path.endsWith(`.${extension}`) ? [path] : [];
});
const wanted = new Set(words);
for (const file of files(directory)) {
  const source = readFileSync(file, 'utf8');
  const tree = parseBytes(source);
  const walk = (node) => {
    if (node.childCount === 0 && wanted.has(node.text)) {
      const at = Buffer.from(source, 'utf8').subarray(0, node.startIndex).toString('utf8').length;
      const context = source.slice(Math.max(0, at - 50), at + 50).replaceAll('\n', '\\n');
      console.log(`${file}:${node.startPosition.row + 1} ${node.parent?.type} ${node.type} | ${context}`);
    }
    for (let index = 0; index < node.childCount; index += 1) walk(node.child(index));
  };
  walk(tree.rootNode);
  if (tree.rootNode.hasError) console.log(`${file}: oracle error`);
  tree.delete();
}
