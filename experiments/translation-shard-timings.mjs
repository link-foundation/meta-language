#!/usr/bin/env node
// Compare retained serial GitHub TAP logs against the current translation files.
// Ambiguous titles stay unmapped. Timing samples are scheduling estimates.
import assert from 'node:assert/strict';
import { readFileSync, statSync } from 'node:fs';
import { basename } from 'node:path';
import { partitionGroupFiles, partitionTests, testFiles } from '../js/scripts/test-groups.mjs';

const files = partitionTests(testFiles()).groups.translation;
const titles = new Map();
const dynamicPrefixes = new Map();
for (const file of files) {
  const address = new URL(`../js/tests/${file}`, import.meta.url);
  let source = readFileSync(address, 'utf8');
  for (const match of source.matchAll(/^import ['"]([^'"\n]+\.test\.mjs)['"];?$/gmu)) {
    source += '\n' + readFileSync(new URL(match[1], address), 'utf8');
  }
  titles.set(`tests/${file}`, file);
  for (const match of source.matchAll(/\b(?:test|it)(?:\.skip|\.todo)?\(\s*(['"`])((?:\\.|(?!\1)[^\n])*)\1\s*,/gu)) {
    const title = match[2].replace(/\\(['"`\\])/gu, '$1');
    if (!titles.has(title)) titles.set(title, file);
    if (titles.get(title) !== file) titles.set(title, null);
  }
  for (const match of source.matchAll(/\btest\(\s*(['"`])((?:\\.|(?!\1)[^\n])*?)\s*(?:\1\s*\+|\$\{)/gu)) {
    if (match[2].length < 20) continue;
    if (!dynamicPrefixes.has(match[2])) dynamicPrefixes.set(match[2], file);
    if (dynamicPrefixes.get(match[2]) !== file) dynamicPrefixes.set(match[2], null);
  }
}

function sourceForTitle(title) {
  if (titles.has(title)) return titles.get(title);
  const candidates = new Set([...dynamicPrefixes].filter(([prefix]) => title.startsWith(prefix)).map(([, file]) => file));
  return candidates.size === 1 ? [...candidates][0] : undefined;
}

function logTime(line) {
  return Date.parse(`${line.slice(0, 23)}Z`);
}

const logs = process.argv.slice(2);
assert.ok(logs.length > 0 && logs.length <= 24, 'pass one to 24 retained translation job logs');
const measured = new Map();
const runs = [];
for (const log of logs) {
  const stat = statSync(log);
  assert.ok(stat.isFile() && stat.size < 1024 * 1024, `${log} exceeds the bounded log size or is not a file`);
  const rows = [];
  let title;
  let total;
  let start;
  for (const line of readFileSync(log, 'utf8').split('\n')) {
    if (line.endsWith('Z TAP version 13')) start = logTime(line);
    const test = /Z # Subtest: (.*)$/u.exec(line);
    if (test) title = test[1];
    const duration = /Z   duration_ms: ([\d.]+)$/u.exec(line);
    if (duration && title) {
      rows.push({ title, file: sourceForTitle(title), milliseconds: Number(duration[1]), end: logTime(line) });
      title = undefined;
    }
    const summary = /Z # duration_ms ([\d.]+)$/u.exec(line);
    if (summary) total = Number(summary[1]);
  }
  const samples = new Map();
  const bounds = new Map();
  let unmapped = 0;
  let previous = start;
  const unmatched = [];
  for (const { title, file, milliseconds, end } of rows) {
    if (!file) unmapped += milliseconds;
    else {
      samples.set(file, (samples.get(file) ?? 0) + milliseconds);
      if (previous && end >= previous) bounds.set(file, (bounds.get(file) ?? 0) + end - previous);
    }
    previous = end;
    if (!file && milliseconds >= 1000) unmatched.push({ title, milliseconds });
  }
  for (const [file, milliseconds] of samples) {
    const costs = measured.get(file) ?? [];
    costs.push({ log: basename(log), milliseconds: Math.round(milliseconds), elapsed: bounds.get(file) ?? 0 });
    measured.set(file, costs);
  }
  assert.ok(Number.isFinite(start) && rows.length > 0, `${log} is missing the test stream`);
  runs.push({ log: basename(log), total: total === undefined ? null : Math.round(total), files: samples.size,
    unmapped: Math.round(unmapped), mapped: Math.round([...samples.values()].reduce((sum, cost) => sum + cost, 0)), unmatched });
}

const costs = [...measured].map(([file, samples]) => ({ file, samples,
  maximum: Math.max(...samples.map(sample => sample.milliseconds)) }));
const partitions = Array.from({ length: 6 }, (_, index) => {
  const assigned = partitionGroupFiles(files, `${index + 1}/6`);
  return { partition: index + 1, files: assigned.length,
    maximum: costs.filter(row => assigned.includes(row.file)).reduce((sum, row) => sum + row.maximum, 0) };
});
console.log(JSON.stringify({ runs, partitions, missing: files.filter(file => !measured.has(file)),
  costs: costs.sort((left, right) => right.maximum - left.maximum) }, null, 2));
