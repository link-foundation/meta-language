// Usage: node experiments/runtime-parity-diff.mjs <artifacts-dir> [section]
// Reports per-entry differences from differences.ndjson written by
// scripts/check-issue-195-runtime-parity.mjs --artifacts-dir.
import { readFileSync } from 'node:fs';
import path from 'node:path';

const dir = process.argv[2];
const only = process.argv[3];
const differences = readFileSync(path.join(dir, 'differences.ndjson'), 'utf8')
  .split('\n').filter(Boolean).map((line) => JSON.parse(line));
for (const { section, index, javascript: entry, rust: other } of differences) {
  if (only && section !== only) continue;
  console.log(`== ${section}[${index}] ${entry?.language ?? ''}`);
  if (entry === null || typeof entry !== 'object' || other === null || typeof other !== 'object') {
    console.log(`  JS ${JSON.stringify(entry)?.slice(0, 200)} | RS ${JSON.stringify(other)?.slice(0, 200)}`);
    continue;
  }
  for (const key of Object.keys(entry)) {
    const a = entry[key];
    const b = other?.[key];
    if (JSON.stringify(a) === JSON.stringify(b)) continue;
    if (Array.isArray(a) && Array.isArray(b)) {
      const bs = new Set(b.map((v) => JSON.stringify(v)));
      const as = new Set(a.map((v) => JSON.stringify(v)));
      const onlyJs = a.filter((v) => !bs.has(JSON.stringify(v)));
      const onlyRs = b.filter((v) => !as.has(JSON.stringify(v)));
      console.log(`  ${key}: js-only ${onlyJs.length}, rust-only ${onlyRs.length}`);
      for (const v of onlyJs.slice(0, 4)) console.log(`    JS  ${typeof v === 'string' ? v : JSON.stringify(v)}`);
      for (const v of onlyRs.slice(0, 4)) console.log(`    RS  ${typeof v === 'string' ? v : JSON.stringify(v)}`);
    } else {
      console.log(`  ${key}: JS ${JSON.stringify(a)?.slice(0, 200)} | RS ${JSON.stringify(b)?.slice(0, 200)}`);
    }
  }
}
