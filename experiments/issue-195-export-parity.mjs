// Probe: how many public JavaScript exports resolve to a Rust public item by
// the naming convention (camelCase → snake_case, PascalCase and UPPER kept).
import { readFile, readdir } from 'node:fs/promises';
import path from 'node:path';
const root = path.resolve(import.meta.dirname, '..');
const js = await import(path.join(root, 'js/src/index.js'));
const items = new Map();
async function walk(dir) {
  for (const entry of await readdir(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) await walk(full);
    else if (entry.name.endsWith('.rs')) {
      const text = await readFile(full, 'utf8');
      for (const m of text.matchAll(/^\s*pub(?:\([^)]*\))? (?:const fn|async fn|fn|struct|enum|const|static|trait|type|mod) (\w+)/gm)) {
        if (!items.has(m[1])) items.set(m[1], path.relative(root, full));
      }
      for (const m of text.matchAll(/^\s*pub use [^;]*?(\w+);/gm)) if (!items.has(m[1])) items.set(m[1], path.relative(root, full));
    }
  }
}
await walk(path.join(root, 'rust/src'));
const snake = (n) => n.replace(/([a-z0-9])([A-Z])/g, '$1_$2').replace(/([A-Z]+)([A-Z][a-z])/g, '$1_$2').toLowerCase();
const found = [], missing = [];
for (const name of Object.keys(js).sort()) {
  const candidates = [name, snake(name), name.toUpperCase(), snake(name).toUpperCase()];
  const hit = candidates.find((c) => items.has(c));
  (hit ? found : missing).push(hit ? `${name} -> ${hit} (${items.get(hit)})` : `${name} [${typeof js[name]}]`);
}
console.log(`found ${found.length}, missing ${missing.length}`);
if (process.argv.includes('--verbose')) console.log(found.join('\n'));
console.log(missing.join('\n'));
