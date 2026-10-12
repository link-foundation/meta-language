import { execFileSync } from 'node:child_process';

/** Retain child test diagnostics when Node's test reporter formats the failure. */
export function runTestCommand(command, args, options = {}) {
  try {
    return execFileSync(command, args, { ...options, encoding: 'utf8' });
  } catch (error) {
    throw new Error(`${error.message}\nChild stdout:\n${String(error.stdout ?? '') || '(empty)'}\nChild stderr:\n${String(error.stderr ?? '') || '(empty)'}`, { cause: error });
  }
}
