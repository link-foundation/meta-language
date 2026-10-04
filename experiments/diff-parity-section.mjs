// Prints the first differing paths between the JavaScript and Rust runtime parity outputs.
import { readFileSync } from 'node:fs';
const [dir, section] = process.argv.slice(2);
const js = JSON.parse(readFileSync(`${dir}/javascript.json`, 'utf8'));
const rs = JSON.parse(readFileSync(`${dir}/rust.json`, 'utf8'));
const out = [];
function walk(a, b, p) {
  if (out.length > 20) return;
  if (JSON.stringify(a) === JSON.stringify(b)) return;
  if (a && b && typeof a === 'object' && typeof b === 'object') {
    for (const k of new Set([...Object.keys(a), ...Object.keys(b)])) walk(a[k], b[k], `${p}.${k}`);
  } else out.push(`${p}\n  js: ${JSON.stringify(a)?.slice(0, 300)}\n  rs: ${JSON.stringify(b)?.slice(0, 300)}`);
}
walk(section ? js[section] : js, section ? rs[section] : rs, section ?? '');
console.log(out.join('\n') || 'identical');
