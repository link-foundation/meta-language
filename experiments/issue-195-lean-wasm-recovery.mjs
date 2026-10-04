// Compares Lean error recovery in web-tree-sitter for string input and callback input.
//   node experiments/issue-195-lean-wasm-recovery.mjs [SOURCE]
import { readFile } from 'node:fs/promises';
import { gunzipSync } from 'node:zlib';
import { Parser, Language } from '../js/node_modules/web-tree-sitter/tree-sitter.js';

const source = process.argv[2] ?? 'import';
await Parser.init();
const wasm = gunzipSync(await readFile(new URL('../js/src/vendor/grammars/lean.wasm.gz', import.meta.url)));
const language = await Language.load(wasm);
for (const [label, input] of [
  ['string', source],
  ['callback', (index) => source.slice(index, index + 5119)],
]) {
  const parser = new Parser();
  parser.setLanguage(language);
  console.log(label.padEnd(8), parser.parse(input).rootNode.toString());
  parser.delete();
}
