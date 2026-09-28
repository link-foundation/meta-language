#!/usr/bin/env node
// Fails when the cache cleanup policy of docs/cache-cleanup.md is not wired:
//
// - the pre-commit hook exists, is executable and runs the cleanup, the
//   bootstrap installs it, and no build or package script installs hooks into
//   a consumer's git configuration;
// - .pre-commit-config.yaml runs the cleanup on every commit and wraps its
//   test hooks;
// - every required cache category has a class;
// - every CI job that runs cargo, npm or the acceptance scripts ends with an
//   always-run teardown, and its build, test, coverage, package and acceptance
//   commands go through scripts/with-cache-cleanup.mjs;
// - the Cargo dev profile keeps debug information and incremental state lean.
//
//   node scripts/check-cache-policy.mjs [--json]
import { execFileSync } from 'node:child_process';
import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

import { CACHE_CLASSES } from './lib/cache-classes.mjs';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');

/** Categories scripts/clean-caches.mjs must own, from the cache cleanup request. */
export const REQUIRED_CATEGORIES = Object.freeze([
  'rust-debug', 'rust-release', 'rust-test', 'rust-coverage', 'rust-incremental', 'rust-examples',
  'custom-target-directory', 'js-build-cache', 'js-test-coverage', 'js-package-consumer',
  'generated-parser-intermediates', 'generated-compiler-intermediates', 'lean-build', 'rocq-build',
  'acceptance-scratch', 'benchmark-scratch', 'container', 'buildkit', 'nested-temporary-clone',
  'nested-consumer-target',
]);

/**
 * Commands that produce build, test, coverage, benchmark or package caches.
 * The acceptance runners (js/scripts/run-issue-195-evidence.mjs and
 * -consumer.mjs) are not wrapped: the evidence runner holds its own lease and
 * cleans the worktree for the acceptance event at the end, which a wrapper's
 * lease would hold back. Their jobs still end with the teardown.
 */
export const CACHE_PRODUCING_COMMAND = /\b(?:cargo\s+(?:\+\S+\s+)?(?:test|build|bench|llvm-cov|package)\b|npm\s+(?:test|pack)\b|wasm-pack\s+build\b)/u;
/** Any command that writes a Cargo or npm cache, which obliges the job to tear down. */
const CACHE_TOUCHING_COMMAND = /\b(?:cargo|npm|npx|wasm-pack)\s|run-issue-195-(?:evidence|consumer)\.mjs/u;
const WRAPPER = /scripts\/with-cache-cleanup\.mjs\s+--event\s+\S+.*\s--\s/u;
const TEARDOWN = /scripts\/clean-caches\.mjs\b.*--event\s+ci-teardown/u;

const code = (text) => text.split('\n').filter((line) => !/^\s*#/u.test(line)).join('\n');

/** Splits a GitHub Actions workflow into jobs and steps (the repository's workflows use two-space indentation). */
export function parseWorkflow(text) {
  const lines = text.split('\n');
  const jobs = [];
  let inJobs = false;
  let job = null;
  let step = null;
  for (const line of lines) {
    if (/^jobs:\s*$/u.test(line)) {
      inJobs = true;
      continue;
    }
    if (!inJobs) continue;
    if (/^\S/u.test(line)) {
      inJobs = false;
      continue;
    }
    const jobHeader = /^ {2}([A-Za-z0-9_-]+):\s*$/u.exec(line);
    if (jobHeader) {
      job = { id: jobHeader[1], steps: [], header: '' };
      jobs.push(job);
      step = null;
      continue;
    }
    if (!job) continue;
    if (/^ {6}- /u.test(line)) {
      step = { text: `${line}\n` };
      job.steps.push(step);
      continue;
    }
    if (step && (/^ {7,}\S/u.test(line) || line.trim() === '')) step.text += `${line}\n`;
    else if (/^ {4}\S/u.test(line)) {
      step = null;
      job.header += `${line}\n`;
    } else if (!step) job.header += `${line}\n`;
  }
  for (const entry of jobs) {
    for (const each of entry.steps) {
      each.name = /name:\s*(.+)/u.exec(each.text)?.[1]?.trim() ?? each.text.split('\n')[0].trim();
      const run = /^ {6}[- ] run:\s*(.*)$/mu.exec(each.text);
      each.run = run ? code(each.text.slice(each.text.indexOf(run[0]) + run[0].length - run[1].length)) : '';
      each.always = /^\s+(?:-\s+)?if:\s*\$?\{?\{?\s*always\(\)/mu.test(each.text);
    }
  }
  return jobs;
}

export function checkWorkflow(file, text) {
  const problems = [];
  for (const job of parseWorkflow(text)) {
    const touching = job.steps.filter((step) => CACHE_TOUCHING_COMMAND.test(step.run));
    for (const step of job.steps) {
      if (CACHE_PRODUCING_COMMAND.test(step.run) && !WRAPPER.test(step.run)) {
        problems.push(`${file} job ${job.id} step "${step.name}" runs a cache-producing command without scripts/with-cache-cleanup.mjs`);
      }
    }
    if (touching.length === 0) continue;
    const last = job.steps.at(-1);
    if (!last || !TEARDOWN.test(last.run) || !last.always) {
      problems.push(`${file} job ${job.id} runs cargo or npm but does not end with an "if: always()" step running scripts/clean-caches.mjs --event ci-teardown`);
    }
  }
  return problems;
}

function preCommitHooks(text) {
  const hooks = [];
  let hook = null;
  for (const line of text.split('\n')) {
    const id = /^\s*- id:\s*(\S+)/u.exec(line);
    if (id) {
      hook = { id: id[1], text: '' };
      hooks.push(hook);
    }
    if (hook) hook.text += `${line}\n`;
  }
  return hooks;
}

export function checkPreCommitConfig(text) {
  const problems = [];
  const hooks = preCommitHooks(text);
  const cleanup = hooks.find(({ id }) => id === 'clean-caches');
  if (!cleanup) problems.push('.pre-commit-config.yaml has no clean-caches hook');
  else {
    if (!/entry:.*scripts\/clean-caches\.mjs/u.test(cleanup.text)) problems.push('.pre-commit-config.yaml clean-caches hook does not run scripts/clean-caches.mjs');
    if (!/always_run:\s*true/u.test(cleanup.text)) problems.push('.pre-commit-config.yaml clean-caches hook must set always_run: true so documentation-only commits clean too');
    if (/^\s*(?:files|types|types_or):/mu.test(cleanup.text)) problems.push('.pre-commit-config.yaml clean-caches hook must not be restricted to some files');
  }
  for (const hook of hooks) {
    const entry = /entry:\s*(.*)/u.exec(hook.text)?.[1] ?? '';
    if (CACHE_PRODUCING_COMMAND.test(entry) && !WRAPPER.test(entry)) {
      problems.push(`.pre-commit-config.yaml hook ${hook.id} runs a cache-producing command without scripts/with-cache-cleanup.mjs`);
    }
  }
  return problems;
}

export function checkHook(text, executable) {
  const problems = [];
  if (text === null) return ['.githooks/pre-commit is missing'];
  if (!executable) problems.push('.githooks/pre-commit is not executable');
  if (!/scripts\/clean-caches\.mjs/u.test(code(text))) problems.push('.githooks/pre-commit does not run scripts/clean-caches.mjs');
  if (!/--event\s+pre-commit/u.test(code(text))) problems.push('.githooks/pre-commit does not report the pre-commit event');
  return problems;
}

export function checkCategories(classes) {
  const covered = new Set(classes.flatMap((cacheClass) => cacheClass.covers ?? []));
  return REQUIRED_CATEGORIES.filter((category) => !covered.has(category))
    .map((category) => `cache category ${category} has no class in scripts/lib/cache-classes.mjs`);
}

function tomlSection(text, name) {
  const start = text.search(new RegExp(`^\\[${name.replaceAll('.', '\\.')}\\]\\s*$`, 'mu'));
  if (start < 0) return null;
  const rest = text.slice(start).split('\n').slice(1);
  const end = rest.findIndex((line) => /^\[/u.test(line));
  return (end < 0 ? rest : rest.slice(0, end)).join('\n');
}

export function checkCargoProfiles(text) {
  const problems = [];
  const dev = tomlSection(text, 'profile.dev');
  if (dev === null) return ['rust/Cargo.toml has no [profile.dev]'];
  if (!/^debug\s*=\s*(?:"line-tables-only"|"none"|false|0)\s*$/mu.test(dev)) {
    problems.push('rust/Cargo.toml [profile.dev] must set debug = "line-tables-only" (or less)');
  }
  if (!/^incremental\s*=\s*false\s*$/mu.test(dev)) problems.push('rust/Cargo.toml [profile.dev] must set incremental = false');
  const test = tomlSection(text, 'profile.test');
  if (test !== null && /^(?:debug\s*=\s*(?:true|2|"full")|incremental\s*=\s*true)\s*$/mu.test(test)) {
    problems.push('rust/Cargo.toml [profile.test] must not re-enable full debug information or incremental compilation');
  }
  return problems;
}

export function checkBootstrap({ contributing, packageJson, buildScript }) {
  const problems = [];
  if (!/node scripts\/install-dev-hooks\.mjs/u.test(contributing ?? '')) {
    problems.push('CONTRIBUTING.md Development Setup does not run node scripts/install-dev-hooks.mjs');
  }
  const scripts = JSON.parse(packageJson ?? '{}').scripts ?? {};
  for (const name of ['preinstall', 'install', 'postinstall', 'prepare', 'prepack', 'postpack']) {
    if (/hooksPath|install-dev-hooks/u.test(scripts[name] ?? '')) {
      problems.push(`js/package.json ${name} script changes git hooks; a consumer's git configuration must stay untouched`);
    }
  }
  if (/hooksPath|install-dev-hooks/u.test(buildScript ?? '')) {
    problems.push("rust/build.rs changes git hooks; a consumer's git configuration must stay untouched");
  }
  return problems;
}

function readOptional(root, relative) {
  try {
    return readFileSync(path.join(root, relative), 'utf8');
  } catch {
    return null;
  }
}

function hookExecutable(root) {
  try {
    const staged = execFileSync('git', ['ls-files', '--stage', '--', '.githooks/pre-commit'], { cwd: root, encoding: 'utf8' });
    if (staged.trim()) return staged.startsWith('100755');
  } catch {
    // Not a git checkout: fall back to the file mode.
  }
  try {
    return process.platform === 'win32' || (statSync(path.join(root, '.githooks/pre-commit')).mode & 0o111) !== 0;
  } catch {
    return false;
  }
}

/** Every policy problem of the checkout at `root`; `overrides` replaces inputs for tests. */
export function checkCachePolicy({ root = ROOT, overrides = {} } = {}) {
  const read = (relative) => (relative in (overrides.files ?? {}) ? overrides.files[relative] : readOptional(root, relative));
  const workflowDirectory = path.join(root, '.github/workflows');
  const workflowFiles = existsSync(workflowDirectory)
    ? readdirSync(workflowDirectory).filter((name) => /\.ya?ml$/u.test(name)).map((name) => `.github/workflows/${name}`)
    : [];
  return [
    ...checkHook(read('.githooks/pre-commit'), overrides.hookExecutable ?? hookExecutable(root)),
    ...checkBootstrap({
      contributing: read('CONTRIBUTING.md'),
      packageJson: read('js/package.json'),
      buildScript: read('rust/build.rs'),
    }),
    ...checkPreCommitConfig(read('.pre-commit-config.yaml') ?? ''),
    ...checkCategories(overrides.classes ?? CACHE_CLASSES),
    ...workflowFiles.flatMap((file) => checkWorkflow(file, read(file) ?? '')),
    ...checkCargoProfiles(read('rust/Cargo.toml') ?? ''),
  ];
}

export function main(argv = process.argv.slice(2)) {
  const problems = checkCachePolicy();
  if (argv.includes('--json')) console.log(JSON.stringify({ problems }, null, 2));
  else if (problems.length === 0) console.log('cache policy: hook, bootstrap, pre-commit, categories, CI teardown and profiles are wired');
  else for (const problem of problems) console.error(`cache policy: ${problem}`);
  return problems.length === 0 ? 0 : 1;
}

if (process.argv[1] && pathToFileURL(path.resolve(process.argv[1])).href === import.meta.url) {
  process.exitCode = main();
}
