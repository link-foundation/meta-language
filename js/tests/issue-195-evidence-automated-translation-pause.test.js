import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';
import { runInNewContext } from 'node:vm';

import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';
import { restoreVerificationPauseSource } from '../../experiments/issue-245/verification-pause-history.mjs';
import { collectDependencies } from '../scripts/dependency-inventory.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { executePhysicalHarness, matchesLogicalFilters, nativeTestArguments, nativeTestMatrix, readPhysicalShardPolicy, readShardAdmission, verifyHarnessExecution } from '../scripts/rust-test-shards.mjs';
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

test('native CI uses the covered toolchain for test harnesses, grammar artifacts and frozen compiler profiles', () => {
  const experiment = rust['native-grammar-outcome-experiment'];
  for (const job of [rust.lint, rust.check, rust['fresh-merge'], rust.test, rust.coverage, rust['original-link-raw-observations'], experiment, ci['rust-suite']]) {
    assert.match(step(job, 'Setup Rust').text, /^\s*uses: dtolnay\/rust-toolchain@1\.99\.0$/mu, job.id);
  }
  assert.match(step(experiment, 'Compile and execute the bounded frozen native candidate').run,
    /native-transport\/runner\/run\.mjs --native$/mu);
});

test('the dependency audit retains all opt-in native runners from the matrix policy', () => {
  const runners = collectDependencies(fileURLToPath(root))
    .filter(row => row.category === 'runner' && row.declaredIn.includes('.github/workflows/rust.yml'))
    .map(row => row.name);
  assert.deepEqual(runners.sort(), [...readPhysicalShardPolicy().platforms].sort());
});

const observe = (scope, assertions, name) =>
  recordIssue195DirectiveObservation(`I195-AUTOMATED-TRANSLATION-PAUSE-${scope}`, assertions, name);

function when(text, overrides = {}) {
  const inline = /^\s*if: \$\{\{ (.*) \}\}$/mu.exec(text)?.[1];
  const multiline = /^\s*if: \|\n((?: {6,}\S[^\n]*\n)+)/mu.exec(text)?.[1];
  assert.ok(inline || multiline, text);
  const expression = (inline ?? multiline).replace(/\bneeds\.([\w-]*-[\w-]+)/gu, 'needs["$1"]');
  const automatedTranslation = overrides.inputs?.automated_translation === true ||
    overrides.vars?.META_LANGUAGE_AUTOMATED_TRANSLATION === 'true';
  return runInNewContext(expression, {
    inputs: { automated_translation: false, external_reports: true },
    env: { META_LANGUAGE_AUTOMATED_TRANSLATION: String(automatedTranslation) },
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
  assert.match(workflow('ci'), /workflow_dispatch:\n    inputs:\n(?:[ ]{6,}[^\n]+\n)*?      automated_translation:[\s\S]*?default: false/u);
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
  const release = { github: { event_name: 'release' } };
  assert.equal(when(ci.delivery.header, release), false);
  assert.equal(when(ci.delivery.header, { ...release, ...repositoryOptIn }), true);
  for (const [job, name] of [
    [ci.consumers, 'Install and parse the exact artifacts in clean consumers'],
    [ci['formal-ai-workload-parts'], 'Run the link-assistant/formal-ai workloads on the exact artifacts'],
    [ci['formal-ai-workloads'], 'Merge the independently validated Formal AI workloads'],
  ]) {
    assert.match(step(job, name).text,
      /META_LANGUAGE_AUTOMATED_TRANSLATION: \$\{\{ inputs\.automated_translation \|\| vars\.META_LANGUAGE_AUTOMATED_TRANSLATION == 'true' \}\}/u);
  }
  assert.equal(when(ci.rust.header), true);
  assert.equal(when(ci.rust.header, { ...optIn, needs: {
    changes: { result: 'success', outputs: { rust: 'true' } },
    js: { result: 'success' }, 'self-translation-report-validation': { result: 'failure' },
  } }), false);
  assert.equal(when(ci.rust.header, { cancelled: () => true }), false);
  assert.equal(when(ci.rust.header, { needs: { changes: { result: 'success' }, js: { result: 'failure' } } }), false);
  assert.equal(when(ci.candidates.header), true);
  // Delivery candidates must admit these jobs even when an earlier report was skipped.
  for (const id of ['consumers', 'rml-workloads', 'formal-ai-workload-parts', 'formal-ai-workloads']) {
    const { header } = ci[id];
    assert.match(header, /\bcancelled\(\)/u, id);
    assert.equal(when(header), true, id);
    assert.equal(when(header, optIn), true, id);
    assert.equal(when(header, { cancelled: () => true }), false, id);
    for (const result of ['failure', 'skipped']) {
      assert.equal(when(header, { needs: { candidates: { result } } }), false, id);
    }
  }
  assert.equal(when(js.reports.header), false);
  assert.equal(when(js.reports.header, optIn), true);
  assert.match(workflow('js'), /META_LANGUAGE_AUTOMATED_TRANSLATION: \$\{\{ inputs\.automated_translation \|\| vars\.META_LANGUAGE_AUTOMATED_TRANSLATION == 'true' \}\}/u);
  assert.equal(when(js.publish.header, { github: { event_name: 'workflow_dispatch' } }), false);
  for (const [name, group, generated_slice] of [
    ['Run generated owned Rust module experiment', 'generated', 'second'],
    ['Run generated owned Rust literal value edit experiment', 'generated', 'first'],
    ['Run Boolean Rust const decision prototype checks', 'generated', 'second'],
    ['Run portable Rust text certificate feasibility checks', 'source-models'],
    ['Run complete original link table projection checks', 'source-model-tables'],
    ['Run complete original link raw projection checks', 'source-model-raw-generation'],
    ['Run complete original link raw native oracle contracts', 'source-model-raw'],
  ]) {
    const entry = step(js['package-checks'], name);
    const matrix = { group, generated_slice };
    assert.equal(when(entry.text, { matrix }), false, name);
    assert.equal(when(entry.text, { matrix, ...optIn }), true, name);
  }
  for (const [name, group, generated_slice] of [
    ['Check grammar-derived default CST expectations', 'grammar'],
    ['Check original link schema source closure', 'generated', 'second'],
    ['Run complete original link raw schema and consumer contracts', 'source-model-raw-schema'],
  ]) {
    const entry = step(js['package-checks'], name);
    assert.equal(when(entry.text, { matrix: { group, generated_slice } }), true, name);
  }

  const pausedGate = step(ci.acceptance, 'Require the active checks while automated translation is paused');
  const missingCandidates = { needs: { candidates: { result: 'skipped' } } };
  const candidateGate = step(ci.acceptance, 'Require delivery candidates');
  assert.equal(when(candidateGate.text, missingCandidates), true);
  assert.equal(when(pausedGate.text, missingCandidates), false);
  assert.equal(when(pausedGate.text), true);
  assert.equal(when(pausedGate.text, optIn), false);
  assert.equal(when(pausedGate.text, { cancelled: () => true }), false);
  const { run } = pausedGate;
  const independent = ['CHANGES', 'JAVASCRIPT', 'GRAMMAR', 'CONSUMERS', 'RELATIVE_META_LOGIC', 'FORMAL_AI'];
  const good = Object.fromEntries(independent.map((check) => [`${check}_RESULT`, 'success']));
  const execute = (overrides) => spawnSync('bash', ['-e', '-c', run.replace(/^\|\n/u, '')], {
    env: { ...process.env, ...good, GITHUB_EVENT_NAME: 'pull_request', GITHUB_STEP_SUMMARY: '/dev/null', RUST_NEEDED: 'true', RUST_RESULT: 'success',
      DELIVERY_EVIDENCE_RESULT: 'skipped', ...overrides },
    encoding: 'utf8',
  });
  assert.equal(execute({}).status, 0);
  assert.equal(execute({ RUST_NEEDED: 'false', RUST_RESULT: 'skipped' }).status, 0);
  assert.equal(execute({ RUST_NEEDED: '', RUST_RESULT: 'skipped' }).status, 1);
  assert.equal(execute({ RUST_RESULT: 'skipped' }).status, 1);
  assert.equal(execute({ DELIVERY_EVIDENCE_RESULT: 'success' }).status, 1);
  const releaseResults = { GITHUB_EVENT_NAME: 'release', CHANGES_RESULT: 'skipped', JAVASCRIPT_RESULT: 'skipped',
    GRAMMAR_RESULT: 'skipped', RUST_NEEDED: '', RUST_RESULT: 'skipped' };
  assert.equal(execute(releaseResults).status, 0);
  assert.equal(execute({ ...releaseResults, CONSUMERS_RESULT: 'failure' }).status, 1);
  for (const check of independent) {
    const result = execute({ [`${check}_RESULT`]: 'failure' });
    assert.equal(result.status, 1, check);
    assert.match(result.stdout, new RegExp(check, 'u'));
  }
  observe('DEFAULT', ['defaultPausesTranslation', 'explicitOptInRunsTranslation', 'independentFailuresRemainVisible'], context.name);
  recordIssue195DirectiveObservation('I195-AUTOMATED-TRANSLATION-PAUSE-ISSUE-245',
    ['translationWorkIsOptIn', 'scopedChecksContinue'], context.name);
});

test('strict commands and native selectors remain available without claiming paused evidence', (context) => {
  for (const name of ['ci', 'js', 'rust']) {
    const file = `.github/workflows/${name}.yml`;
    const current = workflow(name);
    const predecessor = restoreVerificationPauseSource(file, current);
    assert.notEqual(predecessor, current);
    assert.equal(restoreVerificationPauseSource(file, current + '\n'), current + '\n');
    for (const oldJob of parseWorkflow(predecessor)) {
      assert.ok(jobs(name)[oldJob.id], oldJob.id);
      const currentSteps = jobs(name)[oldJob.id].steps;
      for (const oldStep of oldJob.steps.filter((entry) => entry.run)) {
        if (oldStep.name === 'Validate atomic manifest and protected scope baseline') {
          assert.ok(currentSteps.some((entry) => entry.name === oldStep.name && entry.run.includes(oldStep.run.trim())));
        } else if (oldStep.name === 'Require every JavaScript test group') {
          const guarded = currentSteps.find((entry) => entry.name === oldStep.name).run;
          assert.equal(guarded.replaceAll('[ "$AUTOMATED_TRANSLATION" = true ] && ', ''), oldStep.run);
        } else if (name === 'rust' && oldJob.id === 'test' && oldStep.name === 'Run tests') {
          assert.ok(currentSteps.some(entry => entry.name === oldStep.name && entry.run.includes(oldStep.run.trim())));
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
  const translated = 'translation_arrays::maps';
  const protectedManifest = 'issue_195_acceptance::issue_195_manifest_covers_the_full_inventory_and_translation_matrix';
  const independent = 'issue_195_default_cst::has_nodes';
  const inventory = [translated, protectedManifest, independent].map(name => ({ name }));
  for (const profile of ['test', 'fresh-merge', 'evidence']) {
    assert.equal(nativeTestArguments(original, profile, 'true'), original);
    assert.equal(nativeTestArguments(original, profile, undefined), original);
    const selected = nativeTestArguments(original, profile, 'false', inventory);
    assert.equal(matchesLogicalFilters(translated, selected), profile === 'evidence');
    assert.equal(matchesLogicalFilters(protectedManifest, selected), true);
    assert.equal(matchesLogicalFilters(independent, selected), true);
  }
  const ordinary = nativeTestArguments([], 'test', 'false', inventory);
  assert.equal(matchesLogicalFilters('typed_program_links::javascript_and_rust_sources_emit_from_reloaded_source_free_graphs', ordinary), false);
  assert.equal(matchesLogicalFilters('typed_program_links::shared_graph_paths_have_a_bounded_semantic_walk', ordinary), true);
  assert.match(rust['fresh-merge'].header, /META_LANGUAGE_AUTOMATED_TRANSLATION: .*inputs\.automated_translation/u);
  assert.match(read('rust/scripts/simulate-fresh-merge.sh'), /rust-test-shards\.mjs --configure --shell/u);
  observe('RETAIN', ['originalCommandsRetained', 'pausedRunsCannotSupplyCompleteEvidence'], context.name);
});

test('the ordinary native harness selects translation modules from the actual inventory', async () => {
  const translated = 'translation_arrays::maps';
  const independent = 'issue_195_acceptance::issue_195_manifest_covers_the_full_inventory_and_translation_matrix';
  const complete = [translated, independent];
  const receipt = await executePhysicalHarness({
    binary: 'unit', arguments: [], identity: { profile: 'test' }, shard: 1, count: 1, automatedTranslation: 'false',
    invoke: async (_binary, arguments_, listing) => {
      const names = listing === 'all' ? complete : listing
        ? complete.filter(name => matchesLogicalFilters(name, arguments_.slice(0, -3)))
        : complete.filter(name => arguments_.includes(name));
      const stdout = names.map(name => listing ? `${name}: test\n` : `test ${name} ... ok\n`).join('');
      return { stdout, stderr: '', exitCode: 0, signal: null };
    },
  });
  verifyHarnessExecution(receipt);
  assert.deepEqual(receipt.allInventory.map(row => row.name), [independent, translated]);
  assert.deepEqual(receipt.outcomes, [{ name: independent, outcome: 'ok' }]);
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

test('Linux is the default and opting in restores every original platform cell', context => {
  const oldRust = restoreVerificationPauseSource('.github/workflows/rust.yml', workflow('rust'));
  const predecessor = parseWorkflow(oldRust).find(job => job.id === 'test').header;
  const originalPlatforms = /^        os: \[([^\]]+)\]$/mu.exec(predecessor)[1].split(', ');
  const originalSuites = [...predecessor.matchAll(/^          - suite: (?!physical)([^\n]+)\n            filters: ([^\n]+)/gmu)];
  const supplemental = [...predecessor.matchAll(/^          - suite: physical\n            os: ([^\n]+)\n            logical_suite: ([^\n]+)\n            physical_shard: (\d+)\n            filters: ([^\n]+)/gmu)];
  const original = [
    ...originalPlatforms.flatMap(os => originalSuites.map(([, suite, filters]) => ({os, suite, filters}))),
    ...supplemental.map(([, os, logical_suite, index, filters]) => ({os, suite: 'physical', logical_suite, physical_shard: Number(index), filters})),
  ];
  const matrix = nativeTestMatrix(readPhysicalShardPolicy(), 'true');
  const linux = nativeTestMatrix(readPhysicalShardPolicy());
  const keys = cells => cells.map(row => JSON.stringify(row)).sort();
  assert.deepEqual(keys(matrix.include), keys(original));
  assert.deepEqual(keys(linux.include), keys(original.filter(row => row.os === 'ubuntu-latest')));
  assert.ok(matrix.include.some(row => row.os === 'windows-latest'));
  assert.ok(matrix.include.some(row => row.os === 'macos-latest'));
  assert.match(rust.test.header, /matrix: \$\{\{ fromJSON\(needs.check.outputs.native_test_matrix\) \}\}/u);
  assert.match(step(rust.check, 'Select the active native test platforms').run, /rust-test-shards\.mjs --test-matrix/u);
  assert.match(ci.rust.header, /cross_platform_checks:.*inputs\.cross_platform_checks/u);

  const consumerMatrix = /^        os: \$\{\{ (.+) \}\}$/mu.exec(ci.consumers.header)[1];
  const consumerPlatforms = (inputs, vars) => Array.from(runInNewContext(consumerMatrix, { inputs, vars, fromJSON: JSON.parse }));
  assert.deepEqual(consumerPlatforms({}, {}), ['ubuntu-latest']);
  assert.deepEqual(consumerPlatforms({ cross_platform_checks: true }, {}), originalPlatforms);
  assert.deepEqual(consumerPlatforms({}, { META_LANGUAGE_CROSS_PLATFORM_CHECKS: 'true' }), originalPlatforms);
  const jobs = linux.include.map(row => ({name: `Rust / Test (${row.os}, ${row.logical_suite ?? row.suite})`, status: 'completed', conclusion: 'success'}));
  assert.deepEqual(readShardAdmission(jobs, readPhysicalShardPolicy(), 'false'), ['test']);
  assert.throws(() => readShardAdmission(jobs.slice(1), readPhysicalShardPolicy(), 'false'), /missing or duplicate/u);
  assert.throws(() => readShardAdmission(jobs, readPhysicalShardPolicy(), 'true'), /missing or duplicate/u);
  recordIssue195DirectiveObservation('I195-CROSS-PLATFORM-CHECK-PAUSE',
    ['linuxChecksContinue', 'otherPlatformChecksAreRetained', 'optInRestoresPlatformMatrix'], context.name);
});
