// Lists every node shape (discriminant plus field names) that the frontends
// and the checker produce for the given sources: the data model a port must cover.
// Usage: node experiments/translation-node-shapes.mjs <source>...
import { readFileSync } from 'node:fs';
import { parseLean } from '../src/translation/lean.js';
import { parseRocq } from '../src/translation/rocq.js';
import { parseRust } from '../src/translation/rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';

const frontends = { lean: parseLean, v: parseRocq, rs: parseRust, mjs: parseJavaScript, js: parseJavaScript };
const shapes = { surface: new Map(), checked: new Map() };
const visit = (table, value, where) => {
  if (Array.isArray(value)) return value.forEach((item) => visit(table, item, where));
  if (!value || typeof value !== 'object') return;
  const tag = ['k', 'p', 'kind'].find((key) => typeof value[key] === 'string');
  const key = tag ? `${tag}=${value[tag]}` : `${where}:{${Object.keys(value).sort().join(',')}}`;
  const fields = table.get(key) ?? new Set();
  for (const field of Object.keys(value)) fields.add(field);
  table.set(key, fields);
  for (const [field, child] of Object.entries(value)) {
    if (value instanceof Map) continue;
    visit(table, child, tag ? `${value[tag]}.${field}` : field);
  }
  if (value instanceof Map) for (const [name, child] of value) visit(table, child, `map`);
};
for (const file of process.argv.slice(2)) {
  const surface = frontends[file.split('.').pop()](readFileSync(file, 'utf8'));
  visit(shapes.surface, surface, 'root');
  visit(shapes.checked, checkProgram(surface), 'root');
}
for (const [name, table] of Object.entries(shapes)) {
  console.log(`# ${name}`);
  for (const [key, fields] of [...table].sort()) console.log(`${key} -> ${[...fields].join(',')}`);
}
