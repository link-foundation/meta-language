import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { restoreGeneratedPackageOwnerHistory } from './support/generated-package-owner-history.js';

test('actual generated scheduling preserves every check once under both original deadlines after a measured timeout', () => {
  const evidence = readProjectData(new URL('../../parity/fixtures/generated-package-owner-history.lino', import.meta.url));
  const current = readFileSync(new URL('../../' + evidence.path, import.meta.url), 'utf8');
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
