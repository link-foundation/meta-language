// Measures the cache cleanup of the real worktree at the end of the issue #195
// evidence run (requirement I195-CACHE-CLEANUP-MEASURED): the bytes before,
// after and reclaimed, the budget, the tracked state and the evidence the gate
// reads.
import { execFileSync } from 'node:child_process';
import { readdirSync, statSync, writeFileSync } from 'node:fs';
import path from 'node:path';

import { runCleanup } from '../../scripts/lib/cache-cleanup.mjs';

function trackedState(root) {
  // Tracked modifications and untracked, non-ignored files: none of them is a cache.
  return execFileSync('git', ['status', '--porcelain=v1', '-z'], { cwd: root, encoding: 'utf8' });
}

/** Every evidence file in the results directory, with its size; `work` contributes only records and native evidence. */
export function evidenceInventory(resultsDirectory) {
  const files = new Map();
  const walk = (directory) => {
    let entries;
    try {
      entries = readdirSync(directory, { withFileTypes: true });
    } catch {
      return;
    }
    for (const entry of entries) {
      const full = path.join(directory, entry.name);
      const relative = path.relative(resultsDirectory, full).split(path.sep).join('/');
      if (relative.startsWith('work/') && !/^work\/(?:[^/]+\.(?:jsonl?|log)|native\/.*)$/u.test(relative) && relative !== 'work/native') continue;
      if (entry.isDirectory()) walk(full);
      else if (entry.isFile()) files.set(relative, statSync(full).size);
    }
  };
  walk(resultsDirectory);
  return files;
}

/**
 * Cleans the worktree for the `acceptance` event and returns the report with
 * the checks the measured requirement asserts. The caller holds the run's
 * lease, which the cleanup of the same process ignores.
 */
export function measureCacheCleanup({ root, resultsDirectory, reportPath, env = process.env }) {
  const stateBefore = trackedState(root);
  const evidenceBefore = evidenceInventory(resultsDirectory);
  const report = runCleanup({ cwd: root, env, event: 'acceptance', resultsDirectories: [resultsDirectory] });
  const stateAfter = trackedState(root);
  const evidenceAfter = evidenceInventory(resultsDirectory);
  const lostEvidence = [...evidenceBefore]
    .filter(([file, size]) => evidenceAfter.get(file) !== size)
    .map(([file]) => file);
  const checks = {
    beforeAfterReclaimedMeasured: report.status === 'ok' &&
      [report.beforeBytes, report.afterBytes, report.reclaimedBytes].every((bytes) => Number.isFinite(bytes) && bytes >= 0) &&
      report.afterBytes <= report.beforeBytes,
    withinBudgetAfterCleanup: report.withinBudget === true,
    trackedStateUnchanged: stateBefore === stateAfter,
    evidencePreservedAfterCleanup: evidenceBefore.size > 0 && lostEvidence.length === 0,
  };
  const measurement = {
    schemaVersion: 1,
    checks,
    evidenceFiles: evidenceBefore.size,
    lostEvidence,
    trackedStateEntries: stateBefore.split('\0').filter(Boolean).length,
    report,
  };
  writeFileSync(reportPath, `${JSON.stringify(measurement, null, 2)}\n`);
  return measurement;
}
