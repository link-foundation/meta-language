// Finite inventory probe. Bound the JavaScript heap and WASM memory when running:
// node --max-old-space-size=512 --wasm-max-mem-pages=8192 --expose-gc experiments/issue-195-inventory-memory.mjs
import { readFile } from 'node:fs/promises';

function report(stage) {
  if (process.memoryUsage().rss > 1200 * 1024 * 1024) {
    throw new Error(`bounded probe stopped before ${stage}: RSS exceeded 1200 MB`);
  }
  const memory = Object.fromEntries(Object.entries(process.memoryUsage())
    .map(([name, bytes]) => [name, Math.round(bytes / 1024 / 1024)]));
  process.stderr.write(`${JSON.stringify({ stage, megabytes: memory })}\n`);
}

report('before import');
const { LinkNetwork } = await import('../js/src/index.js');
report('after import');
const inventory = JSON.parse(await readFile(
  new URL('../parity/language-grammar-inventory.json', import.meta.url), 'utf8'));
for (const { name, source } of inventory.languages) {
  report(`before ${name}`);
  const network = LinkNetwork.parse(source, name);
  if (network.reconstructText() !== source) throw new Error(`${name}: reconstruction changed`);
  report(`after ${name}`);
  globalThis.gc?.();
}
