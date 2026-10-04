// Prints, for the given js/src modules, the items the self-translation into Rust translates.
import { readFileSync } from 'node:fs';
import { selfTranslate } from '../js/src/self-translation.js';
for (const module of process.argv.slice(2)) {
  const t = selfTranslate(readFileSync(`js/src/${module}`, 'utf8'), 'JavaScript', 'Rust');
  const counts = {};
  for (const item of t.items) counts[`${item.status}:${item.reason ?? ''}`] = (counts[`${item.status}:${item.reason ?? ''}`] ?? 0) + 1;
  console.log(module, JSON.stringify(counts));
  if (process.env.SHOW) console.log(t.code);
}
