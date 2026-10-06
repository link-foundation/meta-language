#!/usr/bin/env node
// Merges the lcov reports of coverage shards into one report: the shards run
// disjoint sets of tests over the same instrumented build, so a line, function
// or branch of a source file counts the hits of every shard, and the merged
// report covers what one run of all the tests covers. With
// --fail-under-lines PERCENT it fails when the merged line coverage is lower,
// as `cargo llvm-cov --fail-under-lines` does on one run.
//   node scripts/merge-lcov.mjs --output lcov.info [--fail-under-lines 84.30] shard.info...
import { readFileSync, writeFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';

const add = (map, key, count) => map.set(key, (map.get(key) ?? 0) + count);
const hits = (text) => (text === '-' ? 0 : Number(text));

/** Merges lcov texts into a map from source file to its line, function and branch hits. */
export function mergeLcov(texts) {
  const files = new Map();
  for (const text of texts) {
    let file = null;
    for (const line of text.split(/\r?\n/u)) {
      const colon = line.indexOf(':');
      const [tag, value] = colon < 0 ? [line, ''] : [line.slice(0, colon), line.slice(colon + 1)];
      if (tag === 'SF') {
        if (!files.has(value)) files.set(value, { functions: new Map(), functionHits: new Map(), branches: new Map(), lines: new Map() });
        file = files.get(value);
      } else if (tag === 'end_of_record') {
        file = null;
      } else if (file === null) {
        continue;
      } else if (tag === 'FN') {
        const comma = value.indexOf(',');
        file.functions.set(value.slice(comma + 1), value.slice(0, comma));
      } else if (tag === 'FNDA') {
        const comma = value.indexOf(',');
        add(file.functionHits, value.slice(comma + 1), Number(value.slice(0, comma)));
      } else if (tag === 'BRDA') {
        const parts = value.split(',');
        add(file.branches, parts.slice(0, 3).join(','), hits(parts[3]));
      } else if (tag === 'DA') {
        const [number, count] = value.split(',');
        add(file.lines, Number(number), Number(count));
      }
    }
  }
  return files;
}

/** The lcov text of merged files, with the totals of each file recomputed. */
export function renderLcov(files) {
  const out = [];
  for (const [path, file] of files) {
    out.push('TN:', `SF:${path}`);
    for (const [name, line] of file.functions) out.push(`FN:${line},${name}`);
    for (const [name] of file.functions) out.push(`FNDA:${file.functionHits.get(name) ?? 0},${name}`);
    out.push(`FNF:${file.functions.size}`, `FNH:${[...file.functions.keys()].filter((name) => (file.functionHits.get(name) ?? 0) > 0).length}`);
    for (const [key, count] of file.branches) out.push(`BRDA:${key},${count}`);
    out.push(`BRF:${file.branches.size}`, `BRH:${[...file.branches.values()].filter((count) => count > 0).length}`);
    for (const [number, count] of [...file.lines].sort(([a], [b]) => a - b)) out.push(`DA:${number},${count}`);
    out.push(`LF:${file.lines.size}`, `LH:${[...file.lines.values()].filter((count) => count > 0).length}`, 'end_of_record');
  }
  return `${out.join('\n')}\n`;
}

/** The covered and the total lines of merged files, and the line coverage in percent. */
export function lineCoverage(files) {
  let covered = 0;
  let total = 0;
  for (const file of files.values()) {
    total += file.lines.size;
    for (const count of file.lines.values()) if (count > 0) covered += 1;
  }
  return { covered, total, percent: total === 0 ? 0 : (100 * covered) / total };
}

function main(args) {
  let output = null;
  let floor = null;
  const inputs = [];
  for (let index = 0; index < args.length; index += 1) {
    if (args[index] === '--output') output = args[(index += 1)];
    else if (args[index] === '--fail-under-lines') floor = Number(args[(index += 1)]);
    else inputs.push(args[index]);
  }
  if (output === null || inputs.length === 0 || Number.isNaN(floor)) {
    console.error('usage: node scripts/merge-lcov.mjs --output lcov.info [--fail-under-lines PERCENT] shard.info...');
    return 2;
  }
  const files = mergeLcov(inputs.map((input) => readFileSync(input, 'utf8')));
  writeFileSync(output, renderLcov(files));
  const { covered, total, percent } = lineCoverage(files);
  console.log(`merge-lcov: ${inputs.length} reports, ${files.size} files, ${covered} of ${total} lines covered (${percent.toFixed(2)}%)`);
  if (floor !== null && percent < floor) {
    console.error(`merge-lcov: line coverage ${percent.toFixed(2)}% is under the floor of ${floor.toFixed(2)}%`);
    return 1;
  }
  return 0;
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) process.exitCode = main(process.argv.slice(2));
