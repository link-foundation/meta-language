import { Parser, Language } from 'web-tree-sitter';
import { readFile, readdir } from 'node:fs/promises';
await Parser.init();
const dir = new URL('../src/vendor/grammars/', import.meta.url);
let t0 = performance.now();
for (const f of (await readdir(dir)).filter(f => f.endsWith('.wasm'))) {
  const s = performance.now();
  await Language.load(await readFile(new URL(f, dir)));
  console.log(f, (performance.now() - s).toFixed(0));
}
console.log('total', (performance.now() - t0).toFixed(0));
