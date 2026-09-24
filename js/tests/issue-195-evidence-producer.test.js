import assert from 'node:assert/strict';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { buildIssue195Manifest } from '../scripts/issue-195-acceptance-lib.mjs';
import {
  buildEvidencePlan,
  evidenceGroupFor,
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
