// Lists processes repeatedly while churn.sh runs and counts the rustc entries
// seen, and those whose working directory could not be read, which the cleanup
// treats as "may use" every Cargo target.
//   bash experiments/cache-cleanup-exit-race/churn.sh 20 & node experiments/cache-cleanup-exit-race/stress.mjs
import { existsSync } from 'node:fs';
import { runningProcesses } from '../../scripts/lib/cache-cleanup.mjs';

let seen = 0;
let unknown = 0;
let vanished = 0;
const end = Date.now() + 15000;
while (Date.now() < end) {
  for (const entry of runningProcesses().processes) {
    if (!/rustc/u.test(entry.program) && !/rustc/u.test(entry.executable ?? '')) continue;
    seen += 1;
    if (entry.cwd === null) {
      unknown += 1;
      if (!existsSync(`/proc/${entry.pid}`)) vanished += 1;
    }
  }
}
console.log(`rustc entries: ${seen}; with an unreadable cwd: ${unknown}, of which already gone: ${vanished}`);
