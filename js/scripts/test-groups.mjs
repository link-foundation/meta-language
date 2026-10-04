#!/usr/bin/env node
// Partitions the JavaScript test files into the groups CI runs as separate
// jobs, so no single job runs the whole suite in one process.
//
//   node scripts/test-groups.mjs --check        every test file is in exactly one group
//   node scripts/test-groups.mjs --list GROUP   print the files of GROUP
//   node scripts/test-groups.mjs --run GROUP    run GROUP with node --test
//
// A new test file that no group claims fails --check (and every --run), so a
// file can never silently drop out of CI.
import { spawnSync } from 'node:child_process';
import { readdirSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const packageRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');

/** Test groups, each a pattern over the test file name without `.test.js`. */
export const TEST_GROUPS = Object.freeze({
  grammar:
    /^(?:grammar-|lino-|links-notation$|pdf-grammar$|default-cst-|tree-sitter-node-kind$|error-recovery-|parse-scaling$|parser-memory$|unicode-input-|language-|natural-language$|regions$|issue-195-grammar-|issue-195-interchange-)/u,
  translation:
    /^(?:translation|issue-195-translation-|issue-195-binding-rename$|issue-195-structured-transformations$|issue-195-faithful-behavior$|issue-195-semantics-proof-preservation$|issue-195-project-semantics$)/u,
  conformance:
    /^(?:four-language-conformance$|issue-195-conformance$|issue-195-generative$|parity$|issue-195-runtime-parity-evidence$|core$|decorators$|access$|query-|graphql-adapter$|concept-distinctions$|foundation-models$|issue-195-concept-records$|issue-195-naming$)/u,
  downstream:
    /^(?:downstream-|issue-195-downstream-|issue-195-rml-|issue-195-formal-ai-|issue-195-delivery$|issue-195-dependency-current-stable-delivery$|dependency-delivery$|crate-package-include$|package-release$|prepare-npm-auth$)/u,
  tooling:
    /^(?:cache-|issue-195-acceptance|issue-195-evidence|issue-195-sources$|issue-195-documentation$|issue-195-vision$|issue-195-merge-enforcement$|issue-195-merge-quality-evidence$|dependency-inventory$|dependency-upgrades$|test-groups$|ci-workflow-structure$|export-parity$)/u,
});

export function testFiles(root = packageRoot) {
  return readdirSync(path.join(root, 'tests'))
    .filter((name) => name.endsWith('.test.js'))
    .sort();
}

/** Returns `{ groups, unassigned, ambiguous }` for the given test files. */
export function partitionTests(files) {
  const groups = Object.fromEntries(Object.keys(TEST_GROUPS).map((group) => [group, []]));
  const unassigned = [];
  const ambiguous = [];
  for (const file of files) {
    const stem = file.replace(/\.test\.js$/u, '');
    const matches = Object.entries(TEST_GROUPS).filter(([, pattern]) => pattern.test(stem)).map(([group]) => group);
    if (matches.length === 0) unassigned.push(file);
    else if (matches.length > 1) ambiguous.push(`${file} (${matches.join(', ')})`);
    else groups[matches[0]].push(file);
  }
  return { groups, unassigned, ambiguous };
}

export function partitionProblems({ groups, unassigned, ambiguous }) {
  return [
    ...unassigned.map((file) => `tests/${file} is in no test group of scripts/test-groups.mjs`),
    ...ambiguous.map((file) => `tests/${file} is in more than one test group`),
    ...Object.entries(groups).filter(([, files]) => files.length === 0).map(([group]) => `test group ${group} has no test files`),
  ];
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const [mode, group] = process.argv.slice(2);
  const partition = partitionTests(testFiles());
  const problems = partitionProblems(partition);
  if (problems.length > 0) {
    console.error(`JavaScript test groups:\n- ${problems.join('\n- ')}`);
    process.exit(1);
  }
  if (mode === '--check') {
    const counts = Object.entries(partition.groups).map(([name, files]) => `${name} ${files.length}`);
    console.log(`JavaScript test groups cover every test file: ${counts.join(', ')}`);
  } else if ((mode === '--list' || mode === '--run') && group in partition.groups) {
    const files = partition.groups[group].map((file) => path.join('tests', file));
    if (mode === '--list') {
      console.log(files.join('\n'));
    } else {
      const { status, signal } = spawnSync(process.execPath, ['--test', '--test-concurrency=1', ...files], {
        cwd: packageRoot,
        stdio: 'inherit',
      });
      process.exit(status ?? (signal ? 1 : 0));
    }
  } else {
    console.error(`usage: test-groups.mjs --check | --list GROUP | --run GROUP (groups: ${Object.keys(TEST_GROUPS).join(', ')})`);
    process.exit(2);
  }
}
