// Probes how web-tree-sitter reports the kind of an aliased anonymous node
// (Scala's class parameter `(`) against the language's symbol table.
import { readFile } from 'node:fs/promises';
import { gunzipSync } from 'node:zlib';
import { Language, Parser } from '../js/node_modules/web-tree-sitter/web-tree-sitter.js';

const root = new URL('..', import.meta.url).pathname;
await Parser.init({ wasmBinary: gunzipSync(await readFile(`${root}js/src/vendor/web-tree-sitter/web-tree-sitter.wasm.gz`)) });
const language = await Language.load(gunzipSync(await readFile(`${root}js/src/vendor/grammars/scala.wasm.gz`)));
const parser = new Parser();
parser.setLanguage(language);
const tree = parser.parse(process.argv[2] ?? 'case class Point(x: Int)\n');
const walk = (node, depth) => {
  if (!node.isNamed) {
    console.log(`${'  '.repeat(depth)}${JSON.stringify(node.type)} typeId=${node.typeId} grammarId=${node.grammarId} grammarType=${JSON.stringify(node.grammarType)} name=${JSON.stringify(language.types[node.typeId])}`);
  }
  node.children.forEach((child) => walk(child, depth + 1));
};
walk(tree.rootNode, 0);
console.log('symbolCount', language.types.length, 'fields', language.fields.length);
