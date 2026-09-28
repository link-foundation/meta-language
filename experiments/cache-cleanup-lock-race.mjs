// Stress test for the cleanup lock: many cleanups of one fixture worktree at
// once, repeated. Every run must exit 0 with status ok or busy.
//   node experiments/cache-cleanup-lock-race.mjs [clean-caches.mjs] [rounds] [parallel]
import { spawn } from 'node:child_process';
import path from 'node:path';

import { CLEAN_CACHES, makeBase, makeRepository, populateCaches } from '../js/tests/support/cache-fixtures.js';

const script = path.resolve(process.argv[2] ?? CLEAN_CACHES);
const rounds = Number(process.argv[3] ?? 20);
const parallel = Number(process.argv[4] ?? 8);
const cleanups = [];
const base = makeBase({ after: (fn) => cleanups.push(fn) });
const failures = [];
for (let round = 0; round < rounds; round += 1) {
  const fixture = makeRepository(base, `round ${round}`);
  populateCaches(fixture);
  const codes = await Promise.all(Array.from({ length: parallel }, () => new Promise((resolve) => {
    let stderr = '';
    const child = spawn(process.execPath, [script, '--full', '--no-docker', '--quiet'], {
      cwd: fixture.root,
      env: { ...process.env, TMPDIR: fixture.tmpRoot },
      stdio: ['ignore', 'ignore', 'pipe'],
    });
    child.stderr.on('data', (chunk) => { stderr += chunk; });
    child.on('exit', (code) => resolve({ code, stderr }));
  })));
  for (const { code, stderr } of codes) if (code !== 0) failures.push({ round, code, stderr: stderr.trim().split('\n').slice(0, 3).join(' | ') });
}
for (const fn of cleanups) fn();
console.log(JSON.stringify({ script, rounds, parallel, failures: failures.length, examples: failures.slice(0, 3) }, null, 1));
process.exitCode = failures.length === 0 ? 0 : 1;
