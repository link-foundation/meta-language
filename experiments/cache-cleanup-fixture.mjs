// Builds a cache cleanup fixture and prints the engine's report for a mode.
//   node experiments/cache-cleanup-fixture.mjs [prune|full] [budgetMb]
import { makeBase, makeRepository, populateCaches } from '../js/tests/support/cache-fixtures.js';
import { runCleanup, summarize } from '../scripts/lib/cache-cleanup.mjs';
const cleanups = [];
const base = makeBase({ after: (fn) => cleanups.push(fn) });
const fixture = makeRepository(base);
populateCaches(fixture);
const report = runCleanup({ cwd: fixture.root, tmpRoot: fixture.tmpRoot, docker: false, mode: process.argv[2] ?? 'full', budgetMb: Number(process.argv[3] ?? 4096), preflight: false });
console.log(summarize(report));
for (const [id, entry] of Object.entries(report.classes)) {
  for (const r of entry.removed) console.log(`${id} removed ${r.path} ${r.tier} ${r.bytes}`);
  for (const s of entry.skipped) console.log(`${id} kept ${s.path}: ${s.reason}`);
}
console.log(JSON.stringify({ before: report.beforeBytes, after: report.afterBytes, reclaimed: report.reclaimedBytes }));
for (const fn of cleanups) fn();
