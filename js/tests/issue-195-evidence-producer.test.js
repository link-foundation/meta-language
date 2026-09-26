import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdtemp, readFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { buildIssue195Manifest, evaluateIssue195Acceptance } from '../scripts/issue-195-acceptance-lib.mjs';
import {
  buildEvidencePlan,
  evidenceGroupFor,
  observedEvidenceForCell,
} from '../scripts/issue-195-evidence-plan.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

test('evidence plan covers every non-gate pre-merge verification exactly once', async () => {
  const manifest = await buildIssue195Manifest(root);
  const expected = manifest.atomicRequirements.flatMap((requirement) =>
    requirement.verifications
      .filter(
        (cell) =>
          cell.checkpoint === 'pre-merge' &&
          requirement.area !== 'acceptance-gate-fault-injection',
      )
      .map((cell) => cell.testId),
  );
  const plan = buildEvidencePlan(manifest);
  const observed = plan.flatMap(({ cells }) => cells.map(({ cell }) => cell.testId));

  assert.deepEqual(observed.toSorted(), expected.toSorted());
  assert.equal(new Set(observed).size, observed.length);
  assert.ok(plan.every(({ cells }) => cells.length > 0));
});

test('evidence plan covers every published-artifact verification exactly once', async () => {
  const manifest = await buildIssue195Manifest(root);
  const expected = manifest.atomicRequirements.flatMap((requirement) =>
    requirement.verifications
      .filter((cell) => cell.checkpoint === 'release-delivery')
      .map((cell) => cell.testId),
  );
  const plan = buildEvidencePlan(manifest, 'release-delivery');
  const observed = plan.flatMap(({ cells }) => cells.map(({ cell }) => cell.testId));

  assert.deepEqual(observed.toSorted(), expected.toSorted());
  assert.equal(observed.length, 6);
  assert.equal(new Set(observed).size, observed.length);
  assert.deepEqual(
    plan.map(({ group }) => group),
    ['delivery:crate', 'delivery:npm', 'delivery:rml'],
  );
});

test('special evidence cannot be certified by a generic runtime suite', () => {
  assert.equal(evidenceGroupFor({ area: 'runtime-parity' }, { runtime: 'javascript' }), 'runtime-parity');
  assert.equal(evidenceGroupFor({ area: 'native-validation', scope: { language: 'Lean' } }, { runtime: 'rust' }), 'native:Lean:rust');
  assert.equal(evidenceGroupFor({ id: 'I195-DELIVERY-NPM-CANDIDATE', area: 'package-delivery' }, { runtime: 'javascript' }), 'delivery:npm');
  assert.equal(evidenceGroupFor({ id: 'I195-DELIVERY-CRATE-CANDIDATE', area: 'package-delivery' }, { runtime: 'rust' }), 'delivery:crate');
  assert.equal(evidenceGroupFor({ id: 'I195-DELIVERY-RML-CANDIDATE', area: 'package-delivery' }, { runtime: 'rust' }), 'delivery:rml');
  assert.equal(evidenceGroupFor({ area: 'default-cst' }, { runtime: 'javascript' }), 'suite:javascript');
  assert.equal(evidenceGroupFor({ area: 'grammar-importer' }, { runtime: 'rust' }), 'suite:rust');
});

test('producer leaves a green no-op command and missing callback uncertified', () => {
  const cell = {
    testId: 'i195-test', runtime: 'javascript',
    assertions: ['semanticPreservationChecked'], fixtureIds: ['fixture'],
  };
  const noOp = observedEvidenceForCell(cell, []);
  assert.equal(noOp.complete, false);
  assert.deepEqual(noOp.assertionsPassed, []);
  const wrongFixture = observedEvidenceForCell(cell, [{
    testId: cell.testId, assertionId: cell.assertions[0], fixtureId: 'other', outcome: 'passed',
  }]);
  assert.equal(wrongFixture.complete, false);
  const observed = observedEvidenceForCell(cell, [{
    testId: cell.testId, assertionId: cell.assertions[0], fixtureId: 'fixture', outcome: 'passed',
  }]);
  assert.equal(observed.complete, true);
});

test('executed translation callbacks reach the evaluator; green no-op and missing coverage do not', async () => {
  const fullManifest = await buildIssue195Manifest(root);
  const requirement = fullManifest.atomicRequirements.find(({ id }) =>
    id === 'I195-TRANSLATE-rust-to-javascript');
  assert.ok(requirement);
  const fullCell = requirement.verifications.find(({ testId }) =>
    testId === 'i195-translate-rust-to-javascript-javascript-positive');
  assert.ok(fullCell);
  const cell = {
    ...fullCell,
    fixtureIds: ['planned:translation:Rust:JavaScript'],
    assertions: ['realTargetArtifact', 'nativeTargetValidation', 'semanticPreservationChecked'],
  };
  const manifest = {
    fixtureCatalog: Object.fromEntries(cell.fixtureIds.map((id) =>
      [id, fullManifest.fixtureCatalog[id]])),
    atomicRequirements: [{ ...requirement, verifications: [cell] }],
  };
  const directory = await mkdtemp(path.join(tmpdir(), 'issue-195-execution-'));
  const observations = path.join(directory, 'records.jsonl');
  const commit = 'observed-commit';
  const documentFromRecords = (records) => {
    const observed = observedEvidenceForCell(cell, records);
    return {
      schemaVersion: 1, issue: 195, commit,
      producer: 'js/scripts/run-issue-195-evidence.mjs',
      generatedAt: new Date().toISOString(),
      results: [{
        testId: cell.testId, kind: cell.kind,
        outcome: observed.complete ? 'passed' : 'missing',
        positiveEvidence: observed.complete,
        command: 'node --test tests/issue-195-translation-behavior.test.js',
        toolchainVersions: { node: process.version },
        grammarVersions: { manifest: 'test' },
        evidenceArtifacts: ['execution-records.jsonl'],
        failureLogs: [],
        assertionsPassed: observed.assertionsPassed,
        executionRecords: observed.executionRecords,
        fixtureDigests: Object.fromEntries(cell.fixtureIds.map((id) =>
          [id, manifest.fixtureCatalog[id].sha256])),
      }],
    };
  };
  const evaluate = (records, candidate = manifest) => evaluateIssue195Acceptance(
    candidate, [documentFromRecords(records)], { checkpoint: 'pre-merge', commit },
  );
  try {
    const childEnvironment = {
      ...process.env, ISSUE_195_OBSERVATION_FILE: observations, ISSUE_195_COMMIT: commit,
    };
    delete childEnvironment.NODE_TEST_CONTEXT;
    const testOutput = execFileSync(process.execPath, ['--test', 'tests/issue-195-translation-behavior.test.js'], {
      cwd: path.join(root, 'js'),
      env: childEnvironment,
      stdio: 'pipe',
    }).toString();
    assert.match(testOutput, /Rust function translation exports an executable JavaScript function/u);
    const records = (await readFile(observations, 'utf8')).trim().split('\n')
      .map(JSON.parse).filter((record) => record.testId === cell.testId);
    assert.equal(records.length, cell.assertions.length * cell.fixtureIds.length);
    assert.equal(evaluate(records).passed, true);
    const fullFixtureManifest = {
      ...fullManifest,
      atomicRequirements: [{ ...requirement, verifications: [fullCell] }],
    };
    assert.equal(evaluate(records, fullFixtureManifest).passed, false);

    execFileSync(process.execPath, ['-e', 'process.exit(0)'], { stdio: 'pipe' });
    assert.equal(evaluate([]).passed, false);
    assert.equal(evaluate(records.slice(1)).passed, false);
    assert.equal(evaluate(records.map((record) => ({ ...record, commit: 'stale' }))).passed, false);
    assert.equal(evaluate(records.map((record) => ({ ...record, fixtureId: 'wrong' }))).passed, false);
    const extraCell = { ...cell, assertions: [...cell.assertions, 'neverExecuted'] };
    const extraManifest = {
      ...manifest,
      atomicRequirements: [{ ...requirement, verifications: [extraCell] }],
    };
    assert.equal(evaluate(records, extraManifest).passed, false);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});

test('inventory malformed-input checks emit observed evidence for each JavaScript CST cell', async () => {
  const manifest = await buildIssue195Manifest(root);
  const directory = await mkdtemp(path.join(tmpdir(), 'issue-195-cst-negative-'));
  const observations = path.join(directory, 'records.jsonl');
  const commit = 'observed-cst-commit';
  try {
    const childEnvironment = {
      ...process.env, ISSUE_195_OBSERVATION_FILE: observations, ISSUE_195_COMMIT: commit,
    };
    delete childEnvironment.NODE_TEST_CONTEXT;
    execFileSync(process.execPath, [
      '--test', '--test-name-pattern=every JavaScript grammar inventory frontend retains',
      'tests/four-language-conformance.test.js',
    ], { cwd: path.join(root, 'js'), env: childEnvironment, stdio: 'pipe' });
    const records = (await readFile(observations, 'utf8')).trim().split('\n').map(JSON.parse);
    const cells = manifest.atomicRequirements
      .filter(({ area }) => area === 'default-cst')
      .flatMap(({ verifications }) => verifications)
      .filter(({ runtime, kind }) => runtime === 'javascript' && kind === 'negative');
    assert.equal(records.length, cells.length * 4);
    for (const cell of cells) {
      const observed = observedEvidenceForCell(cell, records);
      assert.equal(observed.complete, true, cell.testId);
      for (const record of observed.executionRecords) {
        assert.equal(record.commit, commit);
        assert.equal(record.runtime, 'javascript');
        assert.equal(record.fixtureDigest, manifest.fixtureCatalog[record.fixtureId].sha256);
      }
    }
    const requirement = manifest.atomicRequirements.find(({ id }) => id === 'I195-CST-javascript');
    const cell = requirement.verifications.find(({ runtime, kind }) =>
      runtime === 'javascript' && kind === 'negative');
    const candidate = { ...manifest, atomicRequirements: [{ ...requirement, verifications: [cell] }] };
    const evaluate = (observedRecords) => {
      const observed = observedEvidenceForCell(cell, observedRecords);
      const document = {
        schemaVersion: 1, issue: 195, commit,
        producer: 'js/scripts/run-issue-195-evidence.mjs',
        generatedAt: new Date().toISOString(),
        results: [{
          testId: cell.testId, kind: cell.kind,
          outcome: observed.complete ? 'passed' : 'missing',
          positiveEvidence: observed.complete,
          command: 'node --test tests/four-language-conformance.test.js',
          toolchainVersions: { node: process.version },
          grammarVersions: { inventory: 'schema-1' },
          evidenceArtifacts: ['execution-records.jsonl'],
          failureLogs: [],
          assertionsPassed: observed.assertionsPassed,
          executionRecords: observed.executionRecords,
          fixtureDigests: Object.fromEntries(cell.fixtureIds.map((id) =>
            [id, manifest.fixtureCatalog[id].sha256])),
        }],
      };
      return evaluateIssue195Acceptance(candidate, [document], { checkpoint: 'pre-merge', commit });
    };
    const observedRecords = records.filter(({ testId }) => testId === cell.testId);
    assert.equal(evaluate(observedRecords).passed, true);
    assert.equal(evaluate(observedRecords.slice(1)).passed, false);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});
