// Process ownership for persisted leases, locks and temporary directories.
// /proc/self names the process in the mounted procfs, even when process.pid
// belongs to an inner PID namespace. Start time and boot identity prevent a
// recycled numeric PID from inheriting ownership of an exited process's files.
import { readFileSync } from 'node:fs';

function statProcess(name) {
  const text = readFileSync(`/proc/${name}/stat`, 'utf8');
  const end = text.lastIndexOf(')');
  const fields = text.slice(end + 2).trim().split(/\s+/u);
  return { pid: Number(text.slice(0, text.indexOf(' '))), state: fields[0], startTime: fields[19] };
}

export function processIdentity(pid = process.pid) {
  if (process.platform === 'linux' && pid === process.pid) {
    try {
      const { pid: procPid, startTime } = statProcess('self');
      return { pid: procPid, startTime, bootId: readFileSync('/proc/sys/kernel/random/boot_id', 'utf8').trim() };
    } catch {
      // Systems without procfs use the conservative numeric-PID check.
    }
  }
  return { pid };
}

export function processIdentityAlive(owner) {
  if (!Number.isInteger(owner?.pid) || owner.pid <= 0) return false;
  if (owner.startTime && owner.bootId) {
    try {
      const process = statProcess(owner.pid);
      return !['Z', 'X'].includes(process.state) && process.startTime === owner.startTime
        && readFileSync('/proc/sys/kernel/random/boot_id', 'utf8').trim() === owner.bootId;
    } catch (error) {
      // Absence proves exit. Permission failures cannot prove that it exited.
      return !['ENOENT', 'ESRCH'].includes(error.code);
    }
  }
  try {
    process.kill(owner.pid, 0);
    return true;
  } catch (error) {
    return error.code === 'EPERM';
  }
}

export function sameProcess(left, right) {
  return left?.pid === right?.pid && left?.startTime === right?.startTime && left?.bootId === right?.bootId;
}
