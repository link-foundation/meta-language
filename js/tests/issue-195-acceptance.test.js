import assert from 'node:assert/strict';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import {
  ASSERTION_PROFILES,
  buildIssue195Manifest,
  compareScopeBaseline,
  evaluateIssue195Acceptance,
  runIssue195GateFaultInjections,
  validateIssue195Manifest,
} from '../scripts/issue-195-acceptance-lib.mjs';
import { translateProgram } from '../src/program-translation.js';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function clone(value) {
  return structuredClone(value);
}

function syntheticCase(assertions = ['observableBehavior']) {
  const testId = 'i195-synthetic-javascript-positive';
  const manifest = {
    fixtureCatalog: {
      fixture: { path: 'fixture.json', selector: '$', sha256: 'fixture-digest' },
    },
    atomicRequirements: [
      {
        id: 'I195-SYNTHETIC',
        area: 'synthetic',
        scope: { aliases: ['synthetic'], extensions: ['.synthetic'] },
        requiredRuntimes: ['javascript'],
        implementationEntryPoints: { javascript: ['implementation.js'] },
        verifications: [
          {
            testId,
            runtime: 'javascript',
            kind: 'positive',
            checkpoint: 'pre-merge',
            fixtureIds: ['fixture'],
            assertions,
          },
        ],
      },
    ],
  };
  const document = {
    schemaVersion: 1,
    issue: 195,
    commit: 'candidate-sha',
    producer: 'independent-test-runner',
    generatedAt: new Date(0).toISOString(),
    results: [
      {
        testId,
        outcome: 'passed',
        kind: 'positive',
        positiveEvidence: true,
        command: 'node independent-test.js',
        toolchainVersions: { node: process.version },
        grammarVersions: { synthetic: '1.0.0' },
        evidenceArtifacts: ['independent-result.json'],
        assertionsPassed: assertions,
        executionRecords: assertions.map((assertionId) => ({
          testId,
          assertionId,
          fixtureId: 'fixture',
          fixtureDigest: 'fixture-digest',
          runtime: 'javascript',
          commit: 'candidate-sha',
          outcome: 'passed',
          testName: 'independent observable behavior',
        })),
        fixtureDigests: { fixture: 'fixture-digest' },
      },
    ],
  };
  return { manifest, document };
}

function evaluate(manifest, documents) {
  return evaluateIssue195Acceptance(manifest, documents, {
    checkpoint: 'pre-merge',
    commit: 'candidate-sha',
  });
}

test('issue 195 generated manifest is atomic, traceable, and reconciled', async () => {
  const manifest = await buildIssue195Manifest(root);
  const errors = await validateIssue195Manifest(manifest, root);

  assert.deepEqual(errors, []);
  assert.ok(manifest.atomicRequirements.length >= 150);
  assert.equal(
    manifest.atomicRequirements.filter(({ area }) => area === 'directed-translation').length,
    12,
  );
  assert.ok(
    manifest.atomicRequirements.every(
      ({ verifications }) => verifications.length > 0 && verifications.every(({ testId }) => testId),
    ),
  );
});

test('acceptance evaluator accepts exact positive evidence', () => {
  const { manifest, document } = syntheticCase();
  const report = evaluate(manifest, [document]);

  assert.equal(report.passed, true);
  assert.equal(report.summary.passed, 1);
  assert.equal(
    report.requirements[0].cells[0].evidence.command,
    'node independent-test.js',
  );
});

test('a zero-exit suite cannot certify assertions without execution callbacks', () => {
  const { manifest, document } = syntheticCase();
  document.results[0].command = 'node -e "process.exit(0)"';
  document.results[0].executionRecords = [];
  const report = evaluate(manifest, [document]);
  assert.equal(report.passed, false);
  assert.match(JSON.stringify(report), /execution record/u);
});

test('duplicate, stale, and mismatched execution callbacks fail closed', async (context) => {
  for (const [name, mutate, expected] of [
    ['duplicate', (result) => result.executionRecords.push({ ...result.executionRecords[0] }), /duplicate execution record/u],
    ['stale', (result) => { result.executionRecords[0].commit = 'old-sha'; }, /execution record commit mismatch/u],
    ['wrong fixture', (result) => { result.executionRecords[0].fixtureId = 'other'; }, /unexpected execution fixture/u],
    ['skipped', (result) => { result.executionRecords[0].outcome = 'skipped'; }, /execution record outcome is skipped/u],
  ]) {
    await context.test(name, () => {
      const { manifest, document } = syntheticCase();
      mutate(document.results[0]);
      const report = evaluate(manifest, [document]);
      assert.equal(report.passed, false);
      assert.match(JSON.stringify(report), expected);
    });
  }
});

test('scope baseline detects removed rows, aliases, tests, assertions, and fixtures', async (context) => {
  const baseline = await buildIssue195Manifest(root);

  await context.test('required row', () => {
    const candidate = clone(baseline);
    candidate.atomicRequirements.shift();
    assert.match(compareScopeBaseline(baseline, candidate).join('\n'), /scope row removed/);
  });

  await context.test('stable test ID', () => {
    const candidate = clone(baseline);
    candidate.atomicRequirements[0].verifications.shift();
    assert.match(compareScopeBaseline(baseline, candidate).join('\n'), /verification removed/);
  });

  await context.test('required alias', () => {
    const candidate = clone(baseline);
    const entry = candidate.atomicRequirements.find(({ scope }) => scope.aliases.length > 0);
    entry.scope.aliases.shift();
    assert.match(compareScopeBaseline(baseline, candidate).join('\n'), /alias removed/);
  });

  await context.test('observable assertion', () => {
    const candidate = clone(baseline);
    candidate.atomicRequirements[0].verifications[0].assertions.shift();
    assert.match(compareScopeBaseline(baseline, candidate).join('\n'), /assertion removed/);
  });

  await context.test('pinned fixture', () => {
    const candidate = clone(baseline);
    candidate.atomicRequirements[0].verifications[0].fixtureIds.shift();
    assert.match(compareScopeBaseline(baseline, candidate).join('\n'), /fixture removed/);
  });
});

test('known parser shortcuts cannot satisfy the aggregate gate', async (context) => {
  for (const [name, missingAssertion] of [
    ['plain-text fallback', 'realGrammarNodes'],
    ['lexical fallback', 'hierarchy'],
    ['dropped fields', 'namedFields'],
    ['dropped trivia', 'commentsAndTrivia'],
    ['dropped spans', 'exactUtf8Spans'],
  ]) {
    await context.test(name, () => {
      const assertions = ASSERTION_PROFILES.cstPositive;
      const { manifest, document } = syntheticCase(assertions);
      document.results[0].assertionsPassed = assertions.filter(
        (assertion) => assertion !== missingAssertion,
      );

      const report = evaluate(manifest, [document]);
      assert.equal(report.passed, false);
      assert.match(JSON.stringify(report), new RegExp(missingAssertion));
    });
  }
});

test('semantic and edit stubs cannot satisfy the aggregate gate', async (context) => {
  for (const [name, assertions, missingAssertion] of [
    ['binding resolution stub', ASSERTION_PROFILES.bindingRename, 'symbolIdentity'],
    ['rename ignores capture', ASSERTION_PROFILES.bindingRename, 'captureAvoidance'],
    ['edit leaves stale structure', ASSERTION_PROFILES.transformPositive, 'treeIntegrity'],
  ]) {
    await context.test(name, () => {
      const { manifest, document } = syntheticCase(assertions);
      document.results[0].assertionsPassed = assertions.filter(
        (assertion) => assertion !== missingAssertion,
      );

      const report = evaluate(manifest, [document]);
      assert.equal(report.passed, false);
      assert.match(JSON.stringify(report), new RegExp(missingAssertion));
    });
  }
});

test('unsupported or relabelled translations cannot satisfy positive translation evidence', async (context) => {
  await context.test('unsupported descriptor', () => {
    const { manifest, document } = syntheticCase(ASSERTION_PROFILES.translationPositive);
    document.results[0].outcome = 'unsupported';

    const report = evaluate(manifest, [document]);
    assert.equal(report.passed, false);
    assert.match(JSON.stringify(report), /unsupported/);
  });

  await context.test('relabeled source', () => {
    const assertions = ASSERTION_PROFILES.translationPositive;
    const { manifest, document } = syntheticCase(assertions);
    document.results[0].assertionsPassed = assertions.filter(
      (assertion) => assertion !== 'noSourceRelabelling',
    );

    const report = evaluate(manifest, [document]);
    assert.equal(report.passed, false);
    assert.match(JSON.stringify(report), /noSourceRelabelling/);
  });
});

test('a valid source envelope cannot certify semantic translation', async () => {
  const translation = translateProgram('pub fn answer(x: u32) -> u32 { x }', 'Rust', 'JavaScript');
  const targetModule = await import(`data:text/javascript,${encodeURIComponent(translation.code)}`);
  assert.equal(typeof targetModule.answer, 'undefined');
  assert.equal(translation.contract.support, 'portable-encoding');

  const { manifest, document } = syntheticCase(ASSERTION_PROFILES.translationPositive);
  document.results[0].executionRecords = document.results[0].executionRecords.filter(
    ({ assertionId }) => assertionId !== 'semanticPreservationChecked',
  );
  const report = evaluate(manifest, [document]);
  assert.equal(report.passed, false);
  assert.match(JSON.stringify(report), /semanticPreservationChecked/u);
});

test('skips and capability declarations cannot replace exact test evidence', async (context) => {
  await context.test('missing result simulates a skipped test or job', () => {
    const { manifest } = syntheticCase();
    const report = evaluate(manifest, []);

    assert.equal(report.passed, false);
    assert.match(JSON.stringify(report), /required result is missing/);
  });

  await context.test('xfail is not accepted', () => {
    const { manifest, document } = syntheticCase();
    document.results[0].outcome = 'xfail';

    const report = evaluate(manifest, [document]);
    assert.equal(report.passed, false);
    assert.match(JSON.stringify(report), /xfail/);
  });

  await context.test('a falsified status declaration is ignored', () => {
    const { manifest } = syntheticCase();
    manifest.atomicRequirements[0].status = 'complete';
    const report = evaluate(manifest, []);

    assert.equal(report.passed, false);
    assert.match(JSON.stringify(report), /required result is missing/);
  });

  await context.test('self-reported pass without grammar versions is rejected', () => {
    const { manifest, document } = syntheticCase();
    delete document.results[0].grammarVersions;
    const report = evaluate(manifest, [document]);

    assert.equal(report.passed, false);
    assert.match(JSON.stringify(report), /grammar\/component versions are missing/);
  });
});

test('all declared aggregate shortcut mutations are detected and produce exact evidence', async () => {
  const manifest = await buildIssue195Manifest(root);
  const document = runIssue195GateFaultInjections(manifest, 'candidate-sha');
  assert.equal(document.results.length, 11);
  assert.ok(document.results.every(({ outcome }) => outcome === 'passed'));
  assert.ok(document.results.every(({ failureLogs }) => failureLogs.length > 0));

  const report = evaluateIssue195Acceptance(manifest, [document], {
    checkpoint: 'pre-merge',
    commit: 'candidate-sha',
  });
  const gateRequirements = report.requirements.filter(
    ({ area }) => area === 'acceptance-gate-fault-injection',
  );
  assert.equal(gateRequirements.length, 11);
  assert.ok(gateRequirements.every(({ passed }) => passed));
  assert.equal(report.passed, false, 'unfinished non-gate requirements must still fail closed');
});
