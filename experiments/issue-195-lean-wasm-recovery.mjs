// Compares Lean error recovery in web-tree-sitter for string input and callback input.
//   node experiments/issue-195-lean-wasm-recovery.mjs [SOURCE]
import { createRequire } from 'node:module';
import { gunzipSync } from 'node:zlib';
import { readExternalGrammarAsset } from '../js/scripts/external-grammar-cache.mjs';

const require = createRequire(new URL('../js/package.json', import.meta.url));
const { Parser, Language } = require('web-tree-sitter');
const source = process.argv[2] ?? 'import';
await Parser.init();
const wasm = gunzipSync(readExternalGrammarAsset('js/oracles/grammars/lean.wasm.gz'));
const language = await Language.load(wasm);
for (const [label, input] of [
  ['string', source],
  ['callback', (index) => source.slice(index, index + 5119)],
]) {
  const parser = new Parser();
  parser.setLanguage(language);
  const tree = parser.parse(input);
  console.log(label.padEnd(8), tree.rootNode.toString());
  tree.delete();
  parser.delete();
}
