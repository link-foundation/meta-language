// Compare elapsed output intervals with TAP durations in retained job logs.
// Pass the before-change --list output, then the corresponding job logs.
import assert from 'node:assert/strict';
import { readFileSync, statSync } from 'node:fs';
import { basename, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = new URL('../../js/', import.meta.url);
const dynamicTitles = new Map([
  ['tests/issue-195-binding-rename.test.js', /^issue 195 .+ binding rename follows symbol identity and rejects capture$/u],
  ['tests/issue-195-structured-transformations.test.js', /^issue 195 .+ structured .+ runs on a reloaded program and reparses$/u],
  ['tests/translation-pure-unit-public-declaration-contracts.test.js', /^the exact (?:(?:private|public|complete JavaScript) Unit consumer|unregistered .* Unit model) /u],
]);

export function testFileWallCosts(files, bytes, readSource = url => readFileSync(url, 'utf8')) {
  assert.ok(bytes.length <= 1024 * 1024, 'expected a bounded single-job log');
  const owners = new Map();
  for (const file of files) {
    assert.match(file, /^tests\/[\w.-]+\.test\.js$/u);
    const sourceUrl = new URL(file, root);
    const source = readSource(sourceUrl);
    const imports = [...source.matchAll(/^import ['"](\.\.\/\.\.\/experiments\/[^'"\n]+\.test\.mjs)['"]/gmu)];
    const sources = [source, ...imports.map(([, link]) => readSource(new URL(link, sourceUrl)))];
    for (const text of sources) {
      for (const [, , title] of text.matchAll(/\btest\((?:\{[^\n]*?\}, *)? *(['"`])([^\n]*?)\1 *[,)]/gu)) {
        if (!owners.has(title)) owners.set(title, file);
        else if (owners.get(title) !== file) owners.set(title, null);
      }
    }
  }

  const totals = new Map(files.map(file => [file, { elapsed: 0, reported: 0 }]));
  const matchingDynamicTitles = [...dynamicTitles].filter(([file]) => totals.has(file));
  let previous;
  let owner;
  for (const line of bytes.toString('utf8').replace(/^\uFEFF/u, '').split('\n')) {
    const timestamp = Date.parse(line.split(' ', 1)[0]);
    if (line.includes('TAP version 13')) previous = timestamp;
    const title = /# Subtest: (.+)$/u.exec(line)?.[1];
    if (title && Number.isFinite(previous) && Number.isFinite(timestamp)) {
      owner = owners.has(title) ? owners.get(title) : matchingDynamicTitles.find(([, pattern]) => pattern.test(title))?.[0];
      if (owner) totals.get(owner).elapsed += timestamp - previous;
      previous = timestamp;
    }
    const duration = /\bduration_ms: ([0-9.]+)/u.exec(line)?.[1];
    if (duration && owner) {
      totals.get(owner).reported += Number(duration);
      owner = undefined;
    }
  }
  return totals;
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const [listing, ...logs] = process.argv.slice(2);
  assert.ok(listing && logs.length, 'pass a test file listing and at least one retained job log');
  const files = readFileSync(listing, 'utf8').trim().split('\n');
  for (const log of logs) {
    assert.ok(statSync(log).size <= 1024 * 1024, 'expected a bounded single-job log');
    const totals = testFileWallCosts(files, readFileSync(log));
    console.log(basename(log));
    for (const [file, { elapsed, reported }] of [...totals].sort((left, right) => right[1].elapsed - left[1].elapsed)) {
      if (elapsed || reported) console.log(`${file}\t${(elapsed / 1000).toFixed(1)} seconds elapsed\t${(reported / 1000).toFixed(1)} seconds reported`);
    }
  }
}
