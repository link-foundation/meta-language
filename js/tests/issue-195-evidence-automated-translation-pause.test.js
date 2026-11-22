import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';

import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';
import { restoreAutomatedTranslationPauseSource } from '../scripts/automated-translation-pause-source-history.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { matchesLogicalFilters, nativeTestArguments } from '../scripts/rust-test-shards.mjs';
import { recordIssue195DirectiveObservation } from './support/issue-195-observations.js';

const root = new URL('../../', import.meta.url);
const read = (file) => readFileSync(new URL(file, root), 'utf8');
const workflow = (name) => read(`.github/workflows/${name}.yml`);
const jobs = (name) => Object.fromEntries(parseWorkflow(workflow(name)).map((job) => [job.id, job]));
const ci = jobs('ci');
const js = jobs('js');
const rust = jobs('rust');
const step = (job, name) => {
  const value = job.steps.find((entry) => entry.name === name);
  assert.ok(value, `${job.id}: ${name}`);
  return value;
};
const observe = (scope, assertions, name) =>
  recordIssue195DirectiveObservation(`I195-AUTOMATED-TRANSLATION-PAUSE-${scope}`, assertions, name);

function when(text, overrides = {}) {
  const inline = /^\s*if: \$\{\{ (.*) \}\}$/mu.exec(text)?.[1];
  const multiline = /^\s*if: \|\n((?: {6,}\S[^\n]*\n)+)/mu.exec(text)?.[1];
  assert.ok(inline || multiline, text);
  const expression = (inline ?? multiline).replace(/\bneeds\.([\w-]*-[\w-]+)/gu, 'needs["$1"]');
  return runInNewContext(expression, {
    inputs: { automated_translation: false, external_reports: true },
    vars: {}, github: { event_name: 'pull_request' },
    needs: {
      changes: { result: 'success', outputs: { rust: 'true' } },
      js: { result: 'success' },
      candidates: { result: 'success' },
      test: { result: 'success' },
      'self-translation-report-validation': { result: 'skipped' },
    },
    matrix: { group: 'translation' }, cancelled: () => false, always: () => true,
    ...overrides,
  });
}

test('ordinary workflows pause automated translation and fail on independent check failures', (context) => {
  assert.match(workflow('ci'), /workflow_dispatch:\n    inputs:\n      automated_translation:[\s\S]*?default: false/u);
  for (const name of ['ci', 'rust']) {
    assert.match(ci[name === 'ci' ? 'js' : 'rust'].header, /automated_translation:.*inputs\.automated_translation/u);
  }
  for (const id of ['self-translation-report', 'self-translation-report-validation', 'runtime-parity',
    'native-javascript', 'native-rust', 'native-translations', 'javascript-suite-independent', 'javascript-suite', 'rust-suite', 'delivery']) {
    assert.equal(when(ci[id].header), false, id);
  }
  const optIn = { inputs: { automated_translation: true } };
  const repositoryOptIn = { vars: { META_LANGUAGE_AUTOMATED_TRANSLATION: 'true' } };
  for (const profile of [optIn, repositoryOptIn]) {
    assert.equal(when(ci['self-translation-report'].header, profile), true);
    assert.equal(when(ci['self-translation-report-validation'].header, profile), true);
  }
  assert.equal(when(ci.rust.header), true);
  assert.equal(when(ci.rust.header, { ...optIn, needs: {
    changes: { result: 'success', outputs: { rust: 'true' } },
    js: { result: 'success' }, 'self-translation-report-validation': { result: 'failure' },
  } }), false);
  assert.equal(when(ci.rust.header, { cancelled: () => true }), false);
  assert.equal(when(ci.rust.header, { needs: { changes: { result: 'success' }, js: { result: 'failure' } } }), false);
  assert.equal(when(ci.candidates.header), true);
  assert.equal(when(js.reports.header), false);
  assert.equal(when(js.reports.header, optIn), true);
  assert.equal(when(js.publish.header, { github: { event_name: 'workflow_dispatch' } }), false);

  const run = step(ci.acceptance, 'Require the active checks while automated translation is paused').run;
  const independent = ['CHANGES', 'JAVASCRIPT', 'GRAMMAR', 'CONSUMERS', 'RELATIVE_META_LOGIC', 'FORMAL_AI'];
  const good = Object.fromEntries(independent.map((check) => [`${check}_RESULT`, 'success']));
  const execute = (overrides) => spawnSync('bash', ['-e', '-c', run.replace(/^\|\n/u, '')], {
    env: { ...process.env, ...good, GITHUB_STEP_SUMMARY: '/dev/null', RUST_NEEDED: 'true', RUST_RESULT: 'success',
      DELIVERY_EVIDENCE_RESULT: 'skipped', ...overrides },
    encoding: 'utf8',
  });
  assert.equal(execute({}).status, 0);
  assert.equal(execute({ RUST_NEEDED: 'false', RUST_RESULT: 'skipped' }).status, 0);
  assert.equal(execute({ RUST_NEEDED: '', RUST_RESULT: 'skipped' }).status, 1);
  assert.equal(execute({ RUST_RESULT: 'skipped' }).status, 1);
  assert.equal(execute({ DELIVERY_EVIDENCE_RESULT: 'success' }).status, 1);
  for (const check of independent) {
    const result = execute({ [`${check}_RESULT`]: 'failure' });
    assert.equal(result.status, 1, check);
    assert.match(result.stdout, new RegExp(check, 'u'));
  }
  observe('DEFAULT', ['defaultPausesTranslation', 'explicitOptInRunsTranslation', 'independentFailuresRemainVisible'], context.name);
});

test('strict commands and native selectors remain available without claiming paused evidence', (context) => {
  for (const name of ['ci', 'js', 'rust']) {
    const file = `.github/workflows/${name}.yml`;
    const current = workflow(name);
    const predecessor = restoreAutomatedTranslationPauseSource(current, file);
    assert.notEqual(predecessor, current);
    assert.equal(restoreAutomatedTranslationPauseSource(current + '\n', file), current + '\n');
    for (const oldJob of parseWorkflow(predecessor)) {
      assert.ok(jobs(name)[oldJob.id], oldJob.id);
      const currentSteps = jobs(name)[oldJob.id].steps;
      for (const oldStep of oldJob.steps.filter((entry) => entry.run)) {
        if (oldStep.name === 'Validate atomic manifest and protected scope baseline') {
          assert.ok(currentSteps.some((entry) => entry.name === oldStep.name && entry.run.includes(oldStep.run.trim())));
        } else if (oldStep.name === 'Require every JavaScript test group') {
          const guarded = currentSteps.find((entry) => entry.name === oldStep.name).run;
          assert.equal(guarded.replaceAll('[ "$AUTOMATED_TRANSLATION" = true ] && ', ''), oldStep.run);
        } else {
          assert.ok(currentSteps.some((entry) => entry.name === oldStep.name && entry.run.trim() === oldStep.run.trim()),
            `${name}/${oldJob.id}/${oldStep.name}`);
        }
      }
    }
  }
  for (const name of ['Identify admitted Rust physical profiles', 'Require the complete authenticated Rust test union',
    'Download the evidence stage outputs', 'Merge the evidence stages', 'Evaluate every required verification cell',
    'Publish per-requirement evidence report']) {
    const entry = step(ci.acceptance, name);
    assert.equal(when(entry.text, { needs: { candidates: { result: 'success' }, rust: { result: 'success' }, 'rust-suite': { result: 'success' } } }), false, name);
  }
  assert.match(rust.test.header, /META_LANGUAGE_AUTOMATED_TRANSLATION: .*inputs\.automated_translation/u);
  const original = ['issue_195_', 'translation_', '--skip', 'grammar_'];
  const translated = 'unit::translation_arrays::maps';
  for (const profile of ['test', 'fresh-merge', 'evidence']) {
    assert.equal(nativeTestArguments(original, profile, 'true'), original);
    assert.equal(nativeTestArguments(original, profile, undefined), original);
    const selected = nativeTestArguments(original, profile, 'false');
    assert.equal(matchesLogicalFilters(translated, selected), profile === 'evidence');
    assert.equal(matchesLogicalFilters('unit::issue_195_default_cst::has_nodes', selected), true);
  }
  assert.match(rust['fresh-merge'].header, /META_LANGUAGE_AUTOMATED_TRANSLATION: .*inputs\.automated_translation/u);
  assert.match(read('rust/scripts/simulate-fresh-merge.sh'), /rust-test-shards\.mjs --configure --shell/u);
  observe('RETAIN', ['originalCommandsRetained', 'pausedRunsCannotSupplyCompleteEvidence'], context.name);
});

test('the vision and contributor instructions allow manual ports with shared parity', (context) => {
  assert.match(read('docs/vision.md'), /contributors may hand-port JavaScript-first batches between the supported\nlanguages/u);
  assert.match(read('AGENTS.md'), /manual ports between supported languages\n  are allowed/u);
  assert.match(read('CONTRIBUTING.md'), /port changes by hand where needed and verify the\nsame behavior in each affected runtime/u);
  assert.match(read('docs/self-translation.md'), /manual\nports for supported languages and verify behavior with shared fixtures/u);
  assert.match(read('docs/vision.md'), /manual port follows the same observable behavior contract/u);
  observe('MANUAL', ['manualPortsPermitted', 'sharedParityPreserved'], context.name);
});

test('the case study captures exact issue input, baseline, alternatives and rollout', (context) => {
  const folder = 'docs/case-studies/issue-248/';
  const evidence = readProjectData(new URL(folder + 'evidence.lino', root));
  const issue = evidence.issue;
  const register = readProjectData(new URL('parity/issue-195-sources.lino', root));
  const source = register.requirementSources.find(({ id }) => id === 248);
  assert.equal(issue.url, source.url);
  assert.equal(createHash('sha256').update(issue.body).digest('hex'), source.sha256);
  assert.equal(Buffer.byteLength(issue.body), source.bytes);
  assert.ok(evidence.baseline.observations.every(({ job, url, excerpts }) =>
    url.endsWith('/job/' + job) && excerpts.every(({ line, text }) => line > 0 && text.length > 0)));
  const study = read(folder + 'README.md');
  for (const requirement of ['Pause required automated translation', 'Keep the code, checks, tests',
    'Disable failing automated translation', 'Permit manual translation', 'Apply the pause to all open pull requests',
    'Capture data in the requested folder', 'Deliver all changes in one pull request']) {
    assert.ok(study.includes('| ' + requirement), requirement);
  }
  for (const { number, url, headRefName } of evidence.openPullRequests) {
    assert.ok(study.includes(`| [#${number}](${url}) | \`${headRefName}\` |`));
  }
  assert.ok(study.includes('Alternatives considered'));
  assert.ok(study.includes('https://docs.github.com/en/actions/how-tos/reuse-automations/reuse-workflows'));
  assert.ok(study.includes('https://github.com/rhysd/actionlint'));
  observe('CASE-STUDY', ['capturedSourceReconstructsIssue', 'caseStudyMapsEveryRequirement', 'caseStudyRecordsResearchAndAlternatives'], context.name);
});
