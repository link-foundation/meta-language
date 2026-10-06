// Loads every vendored grammar with one web-tree-sitter loading method and
// reports resident memory, to compare synchronous and asynchronous compilation.
// Usage: node grammar-load-rss.mjs load|loadSync|compileThenLoadSync
import { readFileSync } from 'node:fs';
import { gunzipSync } from 'node:zlib';
import { Language, Parser } from '../../js/node_modules/web-tree-sitter/web-tree-sitter.js';

const js = new URL('../../js/src/', import.meta.url);
const lock = JSON.parse(readFileSync(new URL('vendor/grammars/grammar-lock.json', js), 'utf8'));
const mb = () => Math.round(process.memoryUsage().rss / 1048576);
await Parser.init({ wasmBinary: gunzipSync(readFileSync(new URL('vendor/web-tree-sitter/web-tree-sitter.wasm.gz', js))) });
console.log('init', mb());
const method = process.argv[2];
for (const id of Object.keys(lock.grammars)) {
  const binary = gunzipSync(readFileSync(new URL(`vendor/grammars/${id}.wasm.gz`, js)));
  if (method === 'load') await Language.load(binary);
  else if (method === 'loadSync') Language.loadSync(new WebAssembly.Module(binary));
  else Language.loadSync(await WebAssembly.compile(binary));
}
console.log(method, mb(), readFileSync('/proc/self/status', 'utf8').match(/VmHWM:\s*(\d+)/)[1] >> 10, 'MB peak');
