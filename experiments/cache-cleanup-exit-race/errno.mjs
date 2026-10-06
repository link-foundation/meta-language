// Shows the error readlink reports for /proc/<pid>/cwd of a process of another
// user (EACCES) and of a pid that does not exist (ENOENT).
//   node experiments/cache-cleanup-exit-race/errno.mjs
import { readlinkSync } from 'node:fs';

for (const pid of [1, 2 ** 22 + 7]) {
  try {
    console.log(pid, readlinkSync(`/proc/${pid}/cwd`));
  } catch (error) {
    console.log(pid, error.code);
  }
}
