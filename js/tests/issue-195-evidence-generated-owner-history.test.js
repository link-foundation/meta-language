import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { restoreGeneratedPackageOwnerHistory } from './support/generated-package-owner-history.js';

test('actual generated scheduling preserves every check once under both original deadlines after a measured timeout', () => {
  const evidence = readProjectData(new URL('../../parity/fixtures/generated-package-owner-history.lino', import.meta.url));
  const current = restoreCiNodeRuntimeHistory(readFileSync(new URL('../../' + evidence.path, import.meta.url), 'utf8'));
  assert.equal(current, evidence.after.source);
  assert.equal(restoreGeneratedPackageOwnerHistory(current), evidence.before.source);
  const before = parseWorkflow(evidence.before.source).find(job => job.id === 'package-checks');
  const after = parseWorkflow(current).find(job => job.id === 'package-checks');
  assert.equal(after.header, before.header, 'every matrix cell, display name, cancellation key and deadline remains exact');
  assert.equal(after.steps.length, before.steps.length);
  assert.deepEqual(after.steps.map(step => step.name), before.steps.map(step => step.name));
  const expected = ['Run Rust owned local container checks', 'Check strict public declaration consumers',
    'Authenticate reviewed generated-stage migration refreshes'];
  assert.deepEqual(evidence.changes.map(change => change.name), expected);
  for (const [index, step] of after.steps.entries()) {
    const previous = before.steps[index];
    if (expected.includes(step.name)) assert.equal(step.text,
      previous.text.replace("matrix.generated_slice == 'second'", "matrix.generated_slice == 'first'"));
    else assert.equal(step.text, previous.text, 'every unrelated command and predicate remains exact');
  }
  const first = after.steps.filter(step => step.text.includes("matrix.generated_slice == 'first'")).map(step => step.name);
  assert.deepEqual(first, ['Check generated owned Rust literal value edit experiment rules',
    'Check complete generated owned Rust literal value edit fixtures', 'Run generated owned Rust literal value edit experiment', ...expected]);
  const second = after.steps.filter(step => step.text.includes("matrix.generated_slice == 'second'")).map(step => step.name);
  assert.equal(new Set([...first, ...second]).size, first.length + second.length);
  assert.equal(first.length + second.length, before.steps.filter(step => step.text.includes('matrix.generated_slice')).length);
  assert.deepEqual(second.slice(-3), ['Check generated frontend rule compilation and style',
    'Check generated global concept rule compilation and style', 'Retain exact generated sources and compiler feedback']);
  for (const response of Object.values(evidence.responses)) {
    assert.equal(Buffer.byteLength(response.raw), response.bytes);
    assert.equal(createHash('sha256').update(response.raw).digest('hex'), response.sha256);
    assert.deepEqual(JSON.parse(response.raw), response.data);
  }
  assert.equal(evidence.responses.first.data.id, 118143113860);
  assert.equal(evidence.responses.first.data.conclusion, 'success');
  assert.equal(evidence.responses.second.data.id, 118143114075);
  assert.equal(evidence.responses.second.data.conclusion, 'cancelled');
  assert.ok(evidence.responses.annotations.data.some(row => row.message === 'The job has exceeded the maximum execution time of 10m0s'));
  assert.equal(Buffer.byteLength(evidence.log.source), 253635);
  assert.equal(createHash('sha256').update(evidence.log.source).digest('hex'),
    'b9262aecfaba8c6bc0d9f3b1445eb406fc6ba77a107949b9f4a4140433b99b2e');
  for (const change of evidence.changes) for (const replacement of ['', change.before, change.after + change.after,
    change.after.replace('!cancelled()', 'always()'), change.after.includes('working-directory:')
      ? change.after.replace('working-directory:', 'removed-directory:')
      : change.after.replace('        run:', '        working-directory: wrong\n        run:'),
    change.after.replace('run:', 'removed-command:'), change.after.replace("generated_slice == 'first'", "generated_slice == 'first' || true")]) {
    assert.notEqual(replacement, change.after);
    assert.throws(() => restoreGeneratedPackageOwnerHistory(current.replace(change.after, replacement)), change.name);
  }
  assert.throws(() => restoreGeneratedPackageOwnerHistory(evidence.before.source));
});

import {restoreCiNodeRuntimeHistory} from './support/ci-node-runtime-history.js';

test('a third required generated slice keeps every command and feedback producer under the ten-minute deadline', () => {
  const current = readFileSync(new URL('../../.github/workflows/js.yml', import.meta.url), 'utf8');
  const jobs = parseWorkflow(current);
  const checks = jobs.find(job => job.id === 'package-checks');
  const extraCell = '          - group: generated\n            generated_slice: third\n';
  const extraLabel = " || matrix.generated_slice == 'third' && ' / third'";
  assert.equal(checks.header.split(extraCell).length, 2, 'the overloaded generated slice gains one distinct required peer');
  assert.equal(checks.header.split(extraLabel).length, 2, 'the extra slice has its own check name');
  assert.match(checks.header, /timeout-minutes: 10\n/u);
  assert.match(checks.header, /-\$\{\{ matrix\.generated_slice \}\}\n/u);
  const moved = [
    'Check generated JavaScript Unit module experiment rules',
    'Check complete generated JavaScript Unit module experiment fixtures',
    'Run generated JavaScript Unit module experiment',
    'Check generated owned Rust module experiment rules',
    'Check complete generated owned Rust module experiment fixtures',
    'Run generated owned Rust module experiment',
    'Check generated Rust text value projection experiment rules',
    'Check complete Rust text value projection experiment fixtures',
    'Run Rust text value projection experiment',
    'Check shared owned occurrence position fixtures',
    'Run owned occurrence position checks',
    'Authenticate reviewed generated-stage migration refreshes',
  ];
  const thirdSlice = "matrix.generated_slice == 'third'";
  assert.deepEqual(checks.steps.filter(step => step.text.includes(thirdSlice)).map(step => step.name), moved);
  const previousHeader = checks.header.replace(extraCell, '').replace(extraLabel, '');
  const rawCheckoutSetup = [
    '          # Keep commit ancestry for fresh merge simulation and fetch old blobs on demand.',
    "          filter: ${{ matrix.group == 'source-model-raw' && 'blob:none' || '' }}",
    '',
  ].join('\n');
  const previousSteps = checks.steps.map(step => {
    if (step.name === '- uses: actions/checkout@v7') {
      assert.equal(step.text.split(rawCheckoutSetup).length, 2, 'only raw planning uses the blobless checkout');
      return step.text.replace(rawCheckoutSetup, '');
    }
    if (!moved.includes(step.name)) return step.text;
    assert.equal(step.text.split(thirdSlice).length, 2);
    assert.ok(step.afterFailure);
    const oldSlice = step.name === 'Authenticate reviewed generated-stage migration refreshes' ? 'first' : 'second';
    return step.text.replace(thirdSlice, `matrix.generated_slice == '${oldSlice}'`);
  });
  // The timed-out run used 897b1f6b2ff7df5e094ca5d420b9c6e0057ef030.
  assert.equal(createHash('sha256').update(previousHeader + previousSteps.join('')).digest('hex'),
    '715f120d38b81b51a9b6cad39f496d1f2a3cc67be4f19410e7c3519335b58de6',
    'the schedule preserves every original command, setup step, artifact and predicate');
  const aggregate = jobs.find(job => job.id === 'test');
  const gate = aggregate.steps.find(step => step.name === 'Require every JavaScript test group');
  assert.equal(createHash('sha256').update(aggregate.header + gate.text).digest('hex'),
    'eeb8ab8db81f053f173fe98e55e847a990e04fabd038907e2ea287c614e8c7a6',
    'every generated slice feeds the same required package gate');
});
