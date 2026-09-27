#!/usr/bin/env node
import { execFileSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  ISSUE_195_FIXTURE_FILES,
  issue195FixtureDigest,
  recordIssue195Observations,
} from '../tests/support/issue-195-observations.js';
import {
  PARITY_REQUIREMENTS,
  mismatchedSections,
  parityAssertions,
  stableJson,
} from './issue-195-parity-evidence.mjs';
import { runtimeObservation } from './issue-195-runtime-observation.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const rust = JSON.parse(execFileSync('cargo', [
  'run', '--quiet', '--manifest-path', path.join(root, 'rust/Cargo.toml'),
  '--example', 'issue_195_runtime_probe',
], { cwd: root, encoding: 'utf8', maxBuffer: 512 * 1024 * 1024 }));
const javascript = runtimeObservation();

const artifactsOption = process.argv.indexOf('--artifacts-dir');
if (artifactsOption !== -1) {
  const directory = path.resolve(root, process.argv[artifactsOption + 1]);
  await mkdir(directory, { recursive: true });
  await Promise.all([
    writeFile(path.join(directory, 'javascript.json'), stableJson(javascript)),
    writeFile(path.join(directory, 'rust.json'), stableJson(rust)),
  ]);
}

const mismatches = mismatchedSections(javascript, rust);
if (mismatches.length > 0) {
  throw new Error(`JavaScript/Rust runtime parity mismatch in ${mismatches.join(', ')}`);
}
recordParityEvidence();
console.log('issue-195 runtime parity: shared fixture observations agree; full language and translation coverage remains subject to the acceptance gate');

// Records, for each runtime, the assertions of the parity and shared-concept
// requirements that its compared observation shows.
function recordParityEvidence() {
  const fixtureFile = ISSUE_195_FIXTURE_FILES.fourLanguage;
  const pinnedDigest = issue195FixtureDigest(fixtureFile);
  for (const requirementId of Object.keys(PARITY_REQUIREMENTS)) {
    for (const runtime of ['javascript', 'rust']) {
      const { passed, failed } = parityAssertions({
        requirementId,
        runtime,
        observations: { javascript, rust },
        pinnedDigest,
      });
      for (const reason of Object.values(failed)) {
        console.log(`issue-195 runtime parity: ${requirementId} ${runtime}: ${reason}`);
      }
      recordIssue195Observations({
        requirementId,
        runtime,
        suffix: 'positive',
        fixtureId: `planned:cross-cutting:${requirementId}`,
        fixtureFile,
        assertions: passed,
        testName: `issue 195 runtime parity ${requirementId} (${runtime} observation)`,
      });
    }
  }
}
