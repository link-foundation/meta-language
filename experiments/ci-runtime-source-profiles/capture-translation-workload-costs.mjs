import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { partitionTests } from '../../js/scripts/test-groups.mjs';

const root = fileURLToPath(new URL('../../', import.meta.url));
const revision = '38b367159fbe5a20cbcc19a8d75741875113a4b5';
const run = 45042220137;
const jobIds = [137268972429, 137268972769, 137268972684, 137268972578, 137268972792, 137268972919];
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const git = (...args) => execFileSync('git', ['--no-replace-objects', ...args], { cwd: root });

const [directory, mode] = process.argv.slice(2);
assert.ok(directory && [undefined, '--check'].includes(mode));
const sourcePaths = git('ls-tree', '--name-only', `${revision}:js/tests`).toString().trimEnd().split('\n');
const { translation } = partitionTests(sourcePaths.filter(file => file.endsWith('.test.js'))).groups;
const entries = [], logs = [];
for (const [index, job] of jobIds.entries()) {
  // The recorded run used the original modulo owners. Each file contributes only
  // the literal test titles that can be assigned to it uniquely in its own job.
  const titles = new Map(), sources = new Map(), costs = new Map();
  for (const file of translation.filter((_, offset) => offset % jobIds.length === index)) {
    const path = `js/tests/${file}`;
    const source = git('show', `${revision}:${path}`);
    sources.set(path, source);
    for (const match of source.toString().matchAll(/\b(?:test|it)\(\s*(['"`])([^\n]*?)\1\s*,/gu)) {
      titles.set(match[2], titles.has(match[2]) ? null : path);
    }
  }
  const log = readFileSync(join(directory, `ci-${run}-job-${job}.log`));
  logs.push({ job, shard: `${index + 1}/${jobIds.length}`, bytes: log.length, sha256: digest(log) });
  let file;
  for (const line of log.toString().split('\n')) {
    const title = /# Subtest: (.*)$/u.exec(line);
    if (title) file = titles.get(title[1]);
    const duration = /\bduration_ms: ([0-9.]+)/u.exec(line);
    if (!duration) continue;
    if (file) costs.set(file, (costs.get(file) ?? 0) + Number(duration[1]));
    file = undefined;
  }
  assert.ok(costs.size >= 30, `expected test timing samples in ${job}`);
  for (const [file, milliseconds] of costs) {
    const source = sources.get(file);
    entries.push({ file, bytes: source.length, sha256: digest(source), milliseconds: Math.max(1, Math.round(milliseconds)) });
  }
}
entries.sort((left, right) => left.file.localeCompare(right.file));
const packet = { schemaVersion: 1, model: 'observed CI workload scheduling costs 1', run, revision, logs, entries,
  boundary: 'Advisory elapsed time of uniquely named literal tests. The two cancelled jobs contain partial samples. Both unmeasured and changed files retain an estimated cost and mandatory execution. Every full test file retains one owner under the existing six jobs and 15-minute deadline.' };
const output = new URL('../../parity/fixtures/ci-45042220137-translation-workload-costs.lino', import.meta.url);
if (mode === '--check') checkProjectData(output, packet);
else writeProjectData(output, packet);
console.log(`Captured costs for ${entries.length} translation files from ${logs.length} retained job logs.`);
