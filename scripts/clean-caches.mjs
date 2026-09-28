#!/usr/bin/env node
// Single entry point for cleaning the regenerable caches of this worktree.
// See docs/cache-cleanup.md for the classes, safety rules and events.
//
//   node scripts/clean-caches.mjs                  prune to the disk budget
//   node scripts/clean-caches.mjs --full           remove every cache class
//   node scripts/clean-caches.mjs --dry-run --json report.json
//
// Exit status: 0 when the cleanup ran, was busy or was skipped (for example
// outside a git worktree); 2 for invalid arguments; 1 for an internal error.
import { mkdirSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

import { CACHE_CLASSES } from './lib/cache-classes.mjs';
import { runCleanup, summarize, UsageError } from './lib/cache-cleanup.mjs';

const USAGE = `usage: node scripts/clean-caches.mjs [options]

  --mode prune|full     prune to the budget (default) or remove every class
  --full                same as --mode full
  --budget-mb N         aggregate cache budget in MiB
                        (default $META_LANGUAGE_CACHE_BUDGET_MB or 4096)
  --min-free-mb N       low-disk preflight floor; below it the run is full
                        (default $META_LANGUAGE_MIN_FREE_MB or 2048)
  --no-preflight        never escalate to a full clean on low disk
  --event NAME          event that triggered the run (default manual)
  --target-dir DIR      extra Cargo target directory (repeatable)
  --results-dir DIR     evidence directory whose work/ is scratch (repeatable)
  --protect DIR         extra directory that must never be removed (repeatable)
  --class ID            only clean this class (repeatable); see --list-classes
  --no-docker           skip project-labeled container resources
  --dry-run             report what would be removed without removing it
  --json FILE           also write the full report to FILE
  --quiet               print only the one-line summary
  --list-classes        print the cache classes and exit
`;

export function parseArguments(argv) {
  const options = { targetDirectories: [], resultsDirectories: [], protect: [], classes: [] };
  const output = { json: null, quiet: false, list: false, help: false };
  const value = (index, flag) => {
    const next = argv[index + 1];
    if (next === undefined || next.startsWith('--')) throw new UsageError(`${flag} needs a value`);
    return next;
  };
  for (let index = 0; index < argv.length; index += 1) {
    const flag = argv[index];
    switch (flag) {
      case '--mode': options.mode = value(index, flag); index += 1; break;
      case '--full': options.mode = 'full'; break;
      case '--budget-mb': options.budgetMb = Number(value(index, flag)); index += 1; break;
      case '--min-free-mb': options.minFreeMb = Number(value(index, flag)); index += 1; break;
      case '--no-preflight': options.preflight = false; break;
      case '--preflight': options.preflight = true; break;
      case '--event': options.event = value(index, flag); index += 1; break;
      case '--target-dir': options.targetDirectories.push(value(index, flag)); index += 1; break;
      case '--results-dir': options.resultsDirectories.push(value(index, flag)); index += 1; break;
      case '--protect': options.protect.push(value(index, flag)); index += 1; break;
      case '--class': options.classes.push(value(index, flag)); index += 1; break;
      case '--no-docker': options.docker = false; break;
      case '--dry-run': options.dryRun = true; break;
      case '--json': output.json = value(index, flag); index += 1; break;
      case '--quiet': output.quiet = true; break;
      case '--list-classes': output.list = true; break;
      case '--help': case '-h': output.help = true; break;
      default: throw new UsageError(`unknown option: ${flag}`);
    }
  }
  return { options, output };
}

function printReport(report) {
  console.log(summarize(report));
  for (const [id, entry] of Object.entries(report.classes ?? {})) {
    for (const removed of entry.removed ?? []) console.log(`  ${id}: removed ${removed.path} (${removed.note})`);
    for (const skipped of entry.skipped ?? []) console.log(`  ${id}: kept ${skipped.path}: ${skipped.reason}`);
    if (entry.status === 'skipped') console.log(`  ${id}: skipped: ${entry.reason}`);
    for (const command of entry.commands ?? []) {
      console.log(`  ${id}: docker ${command.args.join(' ')}${command.skipped ? ` (skipped: ${command.skipped})` : ` exited ${command.exitCode}`}`);
    }
  }
}

export function main(argv = process.argv.slice(2)) {
  let parsed;
  try {
    parsed = parseArguments(argv);
  } catch (error) {
    console.error(`clean-caches: ${error.message}\n\n${USAGE}`);
    return 2;
  }
  const { options, output } = parsed;
  if (output.help) {
    console.log(USAGE);
    return 0;
  }
  if (output.list) {
    for (const cacheClass of CACHE_CLASSES) console.log(`${cacheClass.id}\t${cacheClass.title}\t${cacheClass.covers.join(',')}`);
    return 0;
  }
  let report;
  try {
    report = runCleanup(options);
  } catch (error) {
    if (error instanceof UsageError) {
      console.error(`clean-caches: ${error.message}`);
      return 2;
    }
    console.error(`clean-caches: cleanup failed: ${error.stack ?? error}`);
    return 1;
  }
  if (output.json) {
    mkdirSync(path.dirname(path.resolve(output.json)), { recursive: true });
    writeFileSync(output.json, `${JSON.stringify(report, null, 2)}\n`);
  }
  if (output.quiet) console.log(summarize(report));
  else printReport(report);
  return 0;
}

if (process.argv[1] && pathToFileURL(path.resolve(process.argv[1])).href === import.meta.url) {
  process.exitCode = main();
}
