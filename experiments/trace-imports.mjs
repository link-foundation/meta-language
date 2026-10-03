// Prints the chain of relative imports from an entry module to a target module.
import fs from 'node:fs';
import path from 'node:path';
const [entry, target] = process.argv.slice(2).map((p) => path.resolve(p));
const seen = new Map([[entry, null]]);
const queue = [entry];
while (queue.length) {
  const file = queue.shift();
  if (file === target) break;
  const text = fs.readFileSync(file, 'utf8');
  for (const m of text.matchAll(/(?:import|export)[^'"]*?from\s+['"](\.[^'"]+)['"]|import\(\s*['"](\.[^'"]+)['"]\s*\)/g)) {
    const next = path.resolve(path.dirname(file), m[1] ?? m[2]);
    if (!seen.has(next) && fs.existsSync(next)) { seen.set(next, file); queue.push(next); }
  }
}
const chain = [];
for (let f = target; f; f = seen.get(f)) chain.unshift(path.relative(process.cwd(), f));
console.log(seen.has(target) ? chain.join('\n -> ') : 'not reachable');
