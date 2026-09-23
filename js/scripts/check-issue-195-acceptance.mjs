#!/usr/bin/env node
import { execFileSync } from 'node:child_process';
import { mkdir, readdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  buildIssue195Manifest,
  compareScopeBaseline,
  evaluateIssue195Acceptance,
  renderIssue195Markdown,
  runIssue195GateFaultInjections,
  validateIssue195Manifest,
} from './issue-195-acceptance-lib.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const manifestPath = path.join(root, 'parity', 'issue-195-requirements.json');

function option(name, fallback = undefined) {
  const index = process.argv.indexOf(name);
  return index === -1 ? fallback : process.argv[index + 1];
}

function stableJson(value) {
  return `${JSON.stringify(value, null, 2)}\n`;
}

async function readManifest() {
  return JSON.parse(await readFile(manifestPath, 'utf8'));
}

async function readResults(directory) {
  try {
    const names = (await readdir(directory)).filter((name) => name.endsWith('.json')).sort();
    return await Promise.all(
      names.map(async (name) => JSON.parse(await readFile(path.join(directory, name), 'utf8'))),
    );
  } catch (error) {
    if (error.code === 'ENOENT') return [];
    throw error;
  }
}

function baselineManifest(reference) {
  if (!/^[A-Za-z0-9._/-]+$/.test(reference)) {
    throw new Error(`unsafe baseline reference: ${reference}`);
  }
  try {
    return JSON.parse(
      execFileSync('git', ['show', `${reference}:parity/issue-195-requirements.json`], {
        cwd: root,
        encoding: 'utf8',
        stdio: ['ignore', 'pipe', 'pipe'],
      }),
    );
  } catch (error) {
    if (error.status === 128) return null;
    throw error;
  }
}

function reportErrors(prefix, errors) {
  for (const error of errors) console.error(`${prefix}: ${error}`);
}

const generatedManifest = await buildIssue195Manifest(root);

if (process.argv.includes('--refresh-manifest')) {
  await writeFile(manifestPath, stableJson(generatedManifest));
  console.log(
    `issue-195: wrote ${generatedManifest.atomicRequirements.length} atomic requirements to ${path.relative(root, manifestPath)}`,
  );
  process.exit(0);
}

const manifest = await readManifest();
const validationErrors = await validateIssue195Manifest(manifest, root);
if (stableJson(manifest) !== stableJson(generatedManifest)) {
  validationErrors.push(
    'committed manifest is stale; run `npm run acceptance:issue-195:refresh` and review the scope change',
  );
}
if (validationErrors.length > 0) {
  reportErrors('issue-195 manifest', validationErrors);
  process.exit(1);
}
console.log(
  `issue-195: validated ${manifest.atomicRequirements.length} atomic requirements and ${Object.keys(manifest.fixtureCatalog).length} fixture identities`,
);

const baselineReference = option('--scope-baseline');
if (baselineReference) {
  const baseline = baselineManifest(baselineReference);
  if (baseline === null) {
    console.log(`issue-195: ${baselineReference} has no manifest; accepting the initial scope baseline`);
  } else {
    const scopeErrors = compareScopeBaseline(baseline, manifest);
    const approved = process.env.ISSUE_195_SCOPE_CHANGE_APPROVED === 'true';
    if (scopeErrors.length > 0 && !approved) {
      reportErrors('issue-195 unapproved scope change', scopeErrors);
      console.error(
        'issue-195: a maintainer must apply the issue-195-scope-change-approved label for an intentional scope reduction',
      );
      process.exit(1);
    }
    if (scopeErrors.length > 0) {
      reportErrors('issue-195 maintainer-approved scope change', scopeErrors);
    } else {
      console.log(`issue-195: required scope is not reduced relative to ${baselineReference}`);
    }
  }
}

const commit = option('--commit', process.env.GITHUB_SHA || 'WORKTREE');
const gateResultsPath = option('--produce-gate-results');
if (gateResultsPath) {
  const resolved = path.resolve(root, gateResultsPath);
  await mkdir(path.dirname(resolved), { recursive: true });
  await writeFile(resolved, stableJson(runIssue195GateFaultInjections(manifest, commit)));
  console.log(`issue-195: wrote gate fault-injection evidence to ${path.relative(root, resolved)}`);
}

if (
  !process.argv.includes('--evaluate') &&
  !process.argv.includes('--write-ledger') &&
  !gateResultsPath
) {
  process.exit(0);
}

if (!process.argv.includes('--evaluate') && !process.argv.includes('--write-ledger')) process.exit(0);

const resultsDirectory = path.resolve(root, option('--results-dir', 'issue-195-results'));
const resultDocuments = await readResults(resultsDirectory);
const checkpoint = option('--checkpoint', 'all');
if (!['all', 'pre-merge', 'release-delivery'].includes(checkpoint)) {
  console.error(`issue-195: invalid checkpoint ${checkpoint}`);
  process.exit(2);
}
const report = evaluateIssue195Acceptance(manifest, resultDocuments, { checkpoint, commit });

const jsonReportPath = option('--json-report');
if (jsonReportPath) {
  const resolved = path.resolve(root, jsonReportPath);
  await mkdir(path.dirname(resolved), { recursive: true });
  await writeFile(resolved, stableJson(report));
}
const markdown = renderIssue195Markdown(manifest, report);
const markdownReportPath = option('--markdown-report');
if (markdownReportPath) {
  const resolved = path.resolve(root, markdownReportPath);
  await mkdir(path.dirname(resolved), { recursive: true });
  await writeFile(resolved, `${markdown}\n`);
}
if (process.argv.includes('--write-ledger')) {
  await writeFile(path.join(root, 'docs', 'issue-195-requirement-ledger.md'), `${markdown}\n`);
}

console.log(
  `issue-195: ${report.summary.passed}/${report.summary.requirements} requirements passed (${report.summary.verificationCells} verification cells, ${report.summary.gateErrors} gate errors)`,
);
if (process.argv.includes('--evaluate') && !report.passed) process.exit(1);
