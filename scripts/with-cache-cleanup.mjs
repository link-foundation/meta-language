#!/usr/bin/env node
// Runs a build, test, coverage, benchmark, package or acceptance command and
// cleans the caches afterwards, whether the command passed, failed or was
// interrupted. The command's exit status is preserved.
//
//   node scripts/with-cache-cleanup.mjs --event test -- cargo test --all-features
//
// While the command runs, a lease marks the worktree as busy, so a concurrent
// cleanup (for example a pre-commit hook) keeps the build outputs it uses. A
// first SIGINT, SIGTERM or SIGHUP is forwarded to the command and the wrapper
// waits for it to exit; a second one kills it. Only then does the cleanup run.
// Unless already set, the command gets bounded parallelism (CARGO_BUILD_JOBS,
// RUST_TEST_THREADS), no incremental compilation (CARGO_INCREMENTAL=0), and a
// bounded compiler cache (SCCACHE_CACHE_SIZE).
import { spawn } from 'node:child_process';
import os from 'node:os';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

import { parseArguments } from './clean-caches.mjs';
import { acquireLease, runCleanup, summarize, UsageError } from './lib/cache-cleanup.mjs';

const SIGNAL_NUMBERS = { SIGHUP: 1, SIGINT: 2, SIGTERM: 15 };
const FORWARDED = process.platform === 'win32' ? ['SIGINT', 'SIGTERM'] : ['SIGINT', 'SIGTERM', 'SIGHUP'];

/** Parallelism the wrapper grants when the caller has not chosen one. */
export function boundedJobs(cpus = os.availableParallelism?.() ?? os.cpus().length) {
  return Math.max(1, Math.min(8, Math.ceil(cpus * 0.75)));
}

/** Environment for the wrapped command: the caller's, with bounded defaults filled in. */
export function boundedEnvironment(env = process.env) {
  const jobs = String(env.META_LANGUAGE_JOBS ?? boundedJobs());
  return {
    ...env,
    CARGO_BUILD_JOBS: env.CARGO_BUILD_JOBS ?? jobs,
    RUST_TEST_THREADS: env.RUST_TEST_THREADS ?? jobs,
    CARGO_INCREMENTAL: env.CARGO_INCREMENTAL ?? '0',
    SCCACHE_CACHE_SIZE: env.SCCACHE_CACHE_SIZE ?? '2G',
  };
}

function splitArguments(argv) {
  const separator = argv.indexOf('--');
  if (separator < 0 || separator === argv.length - 1) {
    throw new UsageError('usage: node scripts/with-cache-cleanup.mjs --event NAME [clean-caches options] -- command [args...]');
  }
  return { cleanupArguments: argv.slice(0, separator), command: argv.slice(separator + 1) };
}

function cleanupAfter(options, status) {
  try {
    const report = runCleanup({ ...options, event: options.event ?? 'build' });
    console.error(`with-cache-cleanup: ${summarize(report)} (command ${status})`);
  } catch (error) {
    // Cleanup never changes the command's status.
    console.error(`with-cache-cleanup: cleanup failed and was ignored: ${error.message}`);
  }
}

export async function main(argv = process.argv.slice(2)) {
  let cleanupOptions;
  let command;
  try {
    const split = splitArguments(argv);
    command = split.command;
    cleanupOptions = parseArguments(split.cleanupArguments).options;
  } catch (error) {
    console.error(`with-cache-cleanup: ${error.message}`);
    return 2;
  }
  const event = cleanupOptions.event ?? 'build';
  const lease = acquireLease({ label: `${event}: ${command.join(' ')}`.slice(0, 200) });
  let received = 0;
  let child;
  const handlers = new Map();
  const outcome = await new Promise((resolve) => {
    child = spawn(command[0], command.slice(1), { stdio: 'inherit', env: boundedEnvironment() });
    for (const signal of FORWARDED) {
      const handler = () => {
        received += 1;
        if (child.exitCode === null && child.signalCode === null) child.kill(received > 1 ? 'SIGKILL' : signal);
      };
      handlers.set(signal, handler);
      process.on(signal, handler);
    }
    child.on('error', (error) => resolve({ code: error.code === 'ENOENT' ? 127 : 126, description: error.message }));
    child.on('exit', (code, signal) => resolve(
      signal
        ? { code: 128 + (SIGNAL_NUMBERS[signal] ?? os.constants.signals[signal] ?? 0), description: `killed by ${signal}` }
        : { code, description: `exited ${code}` },
    ));
  });
  lease.release();
  cleanupAfter({ ...cleanupOptions, event }, outcome.description);
  for (const [signal, handler] of handlers) process.off(signal, handler);
  return outcome.code;
}

if (process.argv[1] && pathToFileURL(path.resolve(process.argv[1])).href === import.meta.url) {
  process.exitCode = await main();
}
