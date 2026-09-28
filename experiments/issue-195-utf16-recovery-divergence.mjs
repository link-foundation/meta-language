// Finds malformed inputs on which the stock web-tree-sitter runtime (UTF-16 input) recovers
// differently from the vendored UTF-8 runtime, which matches the native runtime. Inputs are
// prefixes of upstream corpus sources.
//   node experiments/issue-195-utf16-recovery-divergence.mjs CORPUS_DIR GRAMMAR_ID [LIMIT]
import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { gunzipSync } from 'node:zlib';
import { parseCorpus } from './issue-195-upstream-corpus.mjs';

const [directory, grammar, limit = '20'] = process.argv.slice(2);
const load = async (wasmBinary) => {
  const module = await import(`../js/node_modules/web-tree-sitter/tree-sitter.js?${wasmBinary ? 'utf8' : 'utf16'}`);
  await module.Parser.init(wasmBinary ? { wasmBinary } : undefined);
  const language = await module.Language.load(gunzipSync(readFileSync(`js/src/vendor/grammars/${grammar}.wasm.gz`)));
  return { module, language };
};
const stock = await load();
const utf8 = await load(gunzipSync(readFileSync('js/src/vendor/web-tree-sitter/tree-sitter.wasm.gz')));
const encoder = new TextEncoder();
const parse = ({ module, language }, input) => {
  const parser = new module.Parser();
  parser.setLanguage(language);
  const tree = parser.parse(input);
  parser.delete();
  return tree.rootNode.toString();
};
let found = 0;
for (const file of readdirSync(directory).filter((name) => name.endsWith('.txt')).sort()) {
  for (const test of parseCorpus(readFileSync(join(directory, file), 'utf8'))) {
    const words = [...test.source.matchAll(/\S+/gu)].map((match) => match.index + match[0].length);
    for (const end of words.slice(0, 6)) {
      const source = test.source.slice(0, end);
      const bytes = encoder.encode(source);
      const left = parse(stock, source);
      const right = parse(utf8, (index) => String.fromCharCode(...bytes.subarray(index, index + 4096)));
      if (left !== right) {
        console.log(JSON.stringify({ source, utf16: left, utf8: right }));
        if (++found >= Number(limit)) process.exit(0);
      }
    }
  }
}
