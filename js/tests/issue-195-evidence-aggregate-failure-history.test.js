import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { restoreAggregateFailureReportHistory } from './support/aggregate-failure-history.js';

test('actual aggregate failures retain one gate error and raw metadata diagnostics without forged job names', () => {
  const evidence = readProjectData(new URL('../../parity/fixtures/aggregate-failure-history.lino', import.meta.url));
  const current = restoreWindowsLinkerLookupHistory(readFileSync(new URL('../../' + evidence.path, import.meta.url), 'utf8'));
  assert.equal(current, evidence.after.source);
  assert.equal(restoreAggregateFailureReportHistory(current), evidence.before.source);
  const before = parseWorkflow(evidence.before.source).find(job => job.id === 'acceptance');
  const after = parseWorkflow(current).find(job => job.id === 'acceptance');
  assert.equal(after.header, before.header);
  assert.equal(after.steps[0].text, evidence.stepAfter);
  assert.deepEqual(after.steps.slice(1), before.steps.slice(1), 'every downstream stage and failure gate remains exact');
  assert.match(after.steps[0].text, /if: \$\{\{ needs.candidates.result != 'success' \}\}/u);
  assert.equal(Buffer.byteLength(evidence.log.source), 4391);
  assert.equal(createHash('sha256').update(evidence.log.source).digest('hex'),
    '191ef8df39922f2d0c5092fa59768b986eaec880bf254454482005a4f0ba1f60');
  assert.ok(evidence.log.source.includes('"message": "Server Error"'));
  const script = after.steps[0].run.replace(/^\|\s*\n/u, '').replace(/^ {10}/gmu, '');
  assert.equal(spawnSync('bash', ['-n', '-c', script]).status, 0);
  const stub = `gh() {
    printf 'metadata request: %s\\n' "$*" >&2
    printf '%s' "$AGGREGATE_METADATA_OUTPUT"
    printf '%s' "$AGGREGATE_METADATA_DIAGNOSTIC" >&2
    return "$AGGREGATE_METADATA_EXIT"
  }\n`;
  const response = JSON.stringify({ message: 'Server Error' }, null, 2) + '\n';
  const cases = [
    {javascript: 'failure', reports: 'success', candidates: 'skipped', exit: '0',
      output: 'JavaScript / First Failed Group\nJavaScript / Second Failed Group\n', diagnostic: '', named: true},
    {javascript: 'cancelled', reports: 'success', candidates: 'skipped', exit: '1',
      output: response, diagnostic: 'gh: Server Error (HTTP 502)\n', unavailable: true},
    {javascript: 'success', reports: 'failure', candidates: 'skipped', exit: '1',
      output: '', diagnostic: 'gh: connection unavailable\n', unavailable: true},
    {javascript: 'success', reports: 'cancelled', candidates: 'skipped', exit: '0',
      output: '', diagnostic: ''},
    {javascript: 'success', reports: 'success', candidates: 'failure', exit: '1',
      output: response, diagnostic: 'this command must not run', noQuery: true},
    {javascript: 'success', reports: 'success', candidates: 'cancelled', exit: '1',
      output: response, diagnostic: 'this command must not run', noQuery: true},
  ];
  for (const flags of [['-e'], ['-e', '-o', 'pipefail']]) for (const row of cases) {
    const result = spawnSync('bash', [...flags, '-c', stub + script], {encoding: 'utf8', timeout: 5000,
      maxBuffer: 1024 * 1024, env: {...process.env,
        GITHUB_REPOSITORY: 'link-foundation/meta-language', GITHUB_RUN_ID: String(evidence.run), GITHUB_RUN_ATTEMPT: '1',
        JAVASCRIPT_RESULT: row.javascript, REPORTS_RESULT: row.reports, CANDIDATES_RESULT: row.candidates,
        AGGREGATE_METADATA_EXIT: row.exit, AGGREGATE_METADATA_OUTPUT: row.output, AGGREGATE_METADATA_DIAGNOSTIC: row.diagnostic}});
    assert.equal(result.status, 1, JSON.stringify([flags, row]));assert.equal(result.signal, null);
    assert.equal(result.stdout.split('\n').filter(line => line.startsWith('::error::')).length, 1);
    assert.equal(result.stdout.includes('Server Error'), false, 'error response text is never presented as failed job names');
    if (row.noQuery) {
      assert.equal(result.stderr, '');assert.match(result.stdout, /Delivery Candidates finished as/u);
    } else {
      assert.equal(result.stderr.split('\n').filter(line => line.startsWith('metadata request:')).length, 1);
      assert.ok(result.stderr.includes(`repos/link-foundation/meta-language/actions/runs/${evidence.run}/attempts/1/jobs`));
      if (row.named) assert.ok(result.stdout.includes('JavaScript / First Failed Group, JavaScript / Second Failed Group'));
      else assert.ok(result.stdout.includes('The JavaScript workflow and report assertions'));
      if (row.unavailable) {
        assert.ok(result.stderr.includes('GitHub failed-job metadata is unavailable'));
        assert.ok(result.stderr.includes(row.diagnostic));
        if (row.output) assert.ok(result.stderr.includes(row.output));
      }
    }
  }
  for (const change of [evidence.stepBefore, '', evidence.stepAfter + evidence.stepAfter,
    evidence.stepAfter.replace('exit 1', 'exit 0'), evidence.stepAfter.replace("failed=''", "failed='forged success'")]) {
    assert.throws(() => restoreAggregateFailureReportHistory(current.replace(evidence.stepAfter, change)));
  }
});

import {restoreWindowsLinkerLookupHistory} from './support/windows-linker-lookup-history.js';
