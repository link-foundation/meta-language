import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {parseWorkflow} from '../../scripts/check-cache-policy.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {readRustSourceItems} from '../scripts/generate-private-output-test-model.mjs';

test('native coverage queue correction retains all original coverage assertions and complete physical coordinates',()=>{
  const history=readProjectData(new URL('../../parity/naming/native-coverage-queue-source-history.lino',import.meta.url));
  assert.equal(history.sources.length,1);const before=history.sources[0];
  for(const row of [before,history.after]){
    assert.equal(Buffer.byteLength(row.sourceText),row.bytes);
    assert.equal(createHash('sha256').update(row.sourceText).digest('hex'),row.sha256);
  }
  assert.equal(history.capture.job,118178406550);
  assert.equal(Buffer.byteLength(history.capture.log.source),167646);
  assert.equal(createHash('sha256').update(history.capture.log.source).digest('hex'),
    '7382442c2a8a74ad4a29a0c06874f287f2f81f215e1712dd3af16c31a1aa14b1');
  assert.ok(history.capture.log.source.includes('coverage shards should not cancel each other'));
  const expected="group: rust-${{ github.event_name == 'pull_request' && format('pull-{0}', github.event.pull_request.number) || github.run_id }}-coverage-${{ matrix.suite }}-${{ matrix.logical_suite || matrix.suite }}-${{ matrix.physical_shard || 1 }}";
  assert.equal(history.previous,'group: rust-${{ github.ref }}-coverage-${{ matrix.suite }}');
  assert.equal(history.current,expected);
  const current=restoreNativeTestProfileHistory(readFileSync(new URL('../../'+before.file,import.meta.url),'utf8'));
  assert.equal(current,history.after.sourceText);
  assert.equal(before.sourceText.split(history.previous).length,2);
  assert.equal(current,before.sourceText.replace(history.previous,expected));
  const parsed=readRustSourceItems(Buffer.from(current));
  assert.equal(parsed.items.filter(item=>item.kind==='function_item'&&item.name==='coverage_job_enforces_recorded_line_coverage_floor').length,1);
  const source=readFileSync(new URL('../../.github/workflows/rust.yml',import.meta.url),'utf8');
  const jobs=parseWorkflow(source),coverage=jobs.find(job=>job.id==='coverage'),report=jobs.find(job=>job.id==='coverage-report');
  assert.ok(coverage.header.includes(expected));
  assert.ok(coverage.header.includes('cancel-in-progress: true'));
  assert.ok(coverage.header.includes('suite: [grammar, inference, generative, translation, remaining]'));
  assert.ok(coverage.steps.some(step=>step.run.includes('cargo llvm-cov --all-features --lcov --output-path lcov.info --no-fail-fast -- $SUITE_FILTERS')));
  assert.ok(report.header.includes('needs: [coverage]'));
  assert.ok(report.steps.some(step=>step.run.includes('node scripts/merge-lcov.mjs --output rust/lcov.info --fail-under-lines 84.30')));
});

import {restoreNativeTestProfileHistory} from './support/native-test-profile-history.js';
