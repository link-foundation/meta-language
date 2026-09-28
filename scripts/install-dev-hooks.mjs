#!/usr/bin/env node
// Developer bootstrap: points this clone's git hooks at `.githooks`, whose
// pre-commit hook cleans the regenerable caches on every commit and still runs
// a hook installed in `.git/hooks` (docs/cache-cleanup.md).
//
//   node scripts/install-dev-hooks.mjs          install, then verify
//   node scripts/install-dev-hooks.mjs --check  only verify; exit 1 when missing
//   node scripts/install-dev-hooks.mjs --force  replace another core.hooksPath
//
// Only `git config --local` of this clone changes. Nothing runs this script
// implicitly: no build script, npm lifecycle script or package consumer
// touches any git configuration.
import { execFileSync } from 'node:child_process';
import { accessSync, chmodSync, constants, realpathSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

export const HOOKS_PATH = '.githooks';
const scriptRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');

function git(root, args) {
  return execFileSync('git', args, { cwd: root, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim();
}

function configuredHooksPath(root) {
  try {
    return git(root, ['config', '--local', '--get', 'core.hooksPath']);
  } catch {
    return '';
  }
}

/** Problems that keep the hook from running, or an empty list. */
export function verifyHooks(root) {
  const problems = [];
  const hook = path.join(root, HOOKS_PATH, 'pre-commit');
  try {
    accessSync(hook, constants.X_OK);
  } catch {
    problems.push(`${path.relative(root, hook)} is missing or not executable`);
  }
  const configured = configuredHooksPath(root);
  if (configured !== HOOKS_PATH) problems.push(`core.hooksPath is ${configured ? `'${configured}'` : 'unset'}, not '${HOOKS_PATH}'`);
  else {
    const effective = realpathSync(path.resolve(root, git(root, ['rev-parse', '--git-path', 'hooks'])));
    if (effective !== realpathSync(path.join(root, HOOKS_PATH))) problems.push(`git resolves hooks to ${effective}`);
  }
  return problems;
}

export function main(argv = process.argv.slice(2)) {
  const root = realpathSync(git(scriptRoot, ['rev-parse', '--show-toplevel']));
  if (argv.includes('--check')) {
    const problems = verifyHooks(root);
    for (const problem of problems) console.error(`install-dev-hooks: ${problem}`);
    if (problems.length === 0) console.log(`install-dev-hooks: ${HOOKS_PATH}/pre-commit is active`);
    return problems.length === 0 ? 0 : 1;
  }
  const configured = configuredHooksPath(root);
  if (configured && configured !== HOOKS_PATH && !argv.includes('--force')) {
    console.error(
      `install-dev-hooks: core.hooksPath is already '${configured}'. Call ` +
        '`node scripts/clean-caches.mjs --event pre-commit --quiet` from that pre-commit hook, ' +
        `or re-run with --force to use ${HOOKS_PATH}.`,
    );
    return 1;
  }
  chmodSync(path.join(root, HOOKS_PATH, 'pre-commit'), 0o755);
  git(root, ['config', '--local', 'core.hooksPath', HOOKS_PATH]);
  const problems = verifyHooks(root);
  for (const problem of problems) console.error(`install-dev-hooks: ${problem}`);
  if (problems.length > 0) return 1;
  console.log(`install-dev-hooks: ${HOOKS_PATH}/pre-commit now cleans caches on every commit`);
  return 0;
}

if (process.argv[1] && pathToFileURL(path.resolve(process.argv[1])).href === import.meta.url) {
  process.exitCode = main();
}
