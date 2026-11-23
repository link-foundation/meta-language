// Compare elapsed output intervals with TAP durations in retained job logs.
// Pass the before-change --list output, then the corresponding job logs.
import assert from 'node:assert/strict';
import { readFileSync, statSync } from 'node:fs';
import { basename } from 'node:path';

const [listing, ...logs] = process.argv.slice(2);
assert.ok(listing && logs.length, 'pass a test file listing and at least one retained job log');
const root = new URL('../../js/', import.meta.url);
const files = readFileSync(listing, 'utf8').trim().split('\n');
const owners = new Map();
for (const file of files) {
  assert.match(file, /^tests\/[\w.-]+\.test\.js$/u);
  const source = readFileSync(new URL(file, root), 'utf8');
  for (const [, , title] of source.matchAll(/\btest\((?:\{[^\n]*?\}, *)? *(['"`])([^\n]*?)\1 *[,)]/gu)) {
    if (!owners.has(title)) owners.set(title, file);
    else if (owners.get(title) !== file) owners.set(title, null);
  }
}

for (const log of logs) {
  assert.ok(statSync(log).size <= 1024 * 1024, 'expected a bounded single-job log');
  const totals = new Map(files.map(file => [file, { elapsed: 0, reported: 0 }]));
  let previous;
  let owner;
  for (const line of readFileSync(log, 'utf8').replace(/^\uFEFF/u, '').split('\n')) {
    const timestamp = Date.parse(line.split(' ', 1)[0]);
    if (line.includes('TAP version 13')) previous = timestamp;
    const title = /# Subtest: (.+)$/u.exec(line)?.[1];
    if (title && Number.isFinite(previous) && Number.isFinite(timestamp)) {
      owner = owners.get(title);
      if (owner) totals.get(owner).elapsed += timestamp - previous;
      previous = timestamp;
    }
    const duration = /\bduration_ms: ([0-9.]+)/u.exec(line)?.[1];
    if (duration && owner) {
      totals.get(owner).reported += Number(duration);
      owner = undefined;
    }
  }
  console.log(basename(log));
  for (const [file, { elapsed, reported }] of [...totals].sort((left, right) => right[1].elapsed - left[1].elapsed)) {
    if (elapsed || reported) console.log(`${file}\t${(elapsed / 1000).toFixed(1)} seconds elapsed\t${(reported / 1000).toFixed(1)} seconds reported`);
  }
}
