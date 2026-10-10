#!/usr/bin/env node
// Compares the JavaScript and Rust runtime observations of the shared fixtures.
// The Rust probe streams its observation as NDJSON to a file, so no process
// holds it as one string; with --artifacts-dir the check keeps one digest per
// section entry, the full entries only where the runtimes differ, and each
// runtime's translations for the native validation stages.
import { spawnSync } from 'node:child_process';
import { createReadStream } from 'node:fs';
import { mkdir, mkdtemp, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { createInterface } from 'node:readline';
import { fileURLToPath } from 'node:url';

import {
  ISSUE_195_FIXTURE_FILES,
  issue195FixtureDigest,
  recordIssue195Observations,
} from '../tests/support/issue-195-observations.js';
import {
  PARITY_ARTIFACT_FILES,
  PARITY_REQUIREMENTS,
  mismatchedSections,
  observationFromRecords,
  parityAssertions,
  writeParityDigestArtifacts,
  stableJson,
} from './issue-195-parity-evidence.mjs';
import { runtimeObservation } from './issue-195-runtime-observation.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const rust = await rustObservation();
const javascript = runtimeObservation();

const artifactsOption = process.argv.indexOf('--artifacts-dir');
if (artifactsOption !== -1) {
  const directory = path.resolve(root, process.argv[artifactsOption + 1]);
  await mkdir(directory, { recursive: true });
  const observations = { javascript, rust };
  await writeParityDigestArtifacts(directory, observations);
  for (const [runtime, observation] of Object.entries(observations)) {
    await writeFile(
      path.join(directory, PARITY_ARTIFACT_FILES.translations(runtime)),
      stableJson({ translations: observation.translations }),
    );
  }
}

const mismatches = mismatchedSections(javascript, rust);
if (mismatches.length > 0) {
  throw new Error(`JavaScript/Rust runtime parity mismatch in ${mismatches.join(', ')}`);
}
recordParityEvidence();
console.log('issue-195 runtime parity: shared fixture observations agree; full language and translation coverage remains subject to the acceptance gate');

// Runs the Rust probe with an NDJSON output file and rebuilds its observation
// record by record.
async function rustObservation() {
  const directory = await mkdtemp(path.join(tmpdir(), 'issue-195-runtime-probe-'));
  try {
    const output = path.join(directory, 'rust.ndjson');
    const { status, signal, error } = spawnSync('cargo', [
      'run', '--quiet', '--manifest-path', path.join(root, 'rust/Cargo.toml'),
      '--example', 'issue_195_runtime_probe', '--', output,
    ], { cwd: root, stdio: ['ignore', 'inherit', 'inherit'] });
    if (error) throw error;
    if (status !== 0) throw new Error(`the Rust runtime probe failed (${signal ?? `exit ${status}`})`);
    const records = [];
    for await (const line of createInterface({ input: createReadStream(output), crlfDelay: Infinity })) {
      if (line) records.push(JSON.parse(line));
    }
    return observationFromRecords(records);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
}

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
