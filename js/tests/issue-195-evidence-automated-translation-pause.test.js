import assert from 'node:assert/strict';
import { execFileSync, spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';
import { runInNewContext } from 'node:vm';

import { captureDiagnosticDependencyProfile, diagnosticFixturePath } from '../../experiments/grammar-outcome-network/native-transport/diagnostic-order/generate.mjs';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';
import { restoreAutomatedTranslationPauseSource } from '../scripts/automated-translation-pause-source-history.mjs';
import { ciIntegrationParents, restoreCiIntegrationSource } from '../scripts/ci-integration-source-history.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { executePhysicalHarness, matchesLogicalFilters, nativeTestArguments, nativeTestMatrix, readPhysicalShardPolicy, readShardAdmission, verifyHarnessExecution, verifyWorkflowPlatforms } from '../scripts/rust-test-shards.mjs';
import { partitionGroupFiles, partitionTests, testFiles } from '../scripts/test-groups.mjs';
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
  for (const job of [rust.check, rust['fresh-merge'], rust.test, rust.coverage, rust['original-link-raw-observations'], experiment, ci['rust-suite']]) {
    assert.match(step(job, 'Setup Rust').text, /^\s*uses: dtolnay\/rust-toolchain@1\.99\.1$/mu, job.id);
  }
  assert.match(step(rust.lint, 'Setup Rust').text, /^\s*uses: dtolnay\/rust-toolchain@stable$/mu);
  assert.match(step(experiment, 'Compile and execute the bounded frozen native candidate').run,
    /native-transport\/runner\/run\.mjs --native$/mu);
});

test('the native diagnostic fixture authenticates the active manifest and lock', () => {
  const fixture = readProjectData(new URL(diagnosticFixturePath, root));
  assert.deepEqual(fixture.dependencyProfile, captureDiagnosticDependencyProfile(fileURLToPath(root)));
  assert.equal(fixture.cases.length, 2);
});

test('the two new tooling tests preserve all established shard assignments', () => {
  const files = partitionTests(testFiles()).groups.tooling;
  const pauseTests = [
    'issue-195-evidence-automated-translation-pause.test.js',
    'issue-195-evidence-automated-translation-rollout.test.js',
  ];
  const established = files.filter(file => !pauseTests.includes(file));
  const partitions = [1, 2, 3, 4].map(partition => partitionGroupFiles(files, partition + '/4'));
  for (const [index, partition] of partitions.entries()) {
    assert.deepEqual(partition.filter(file => !pauseTests.includes(file)),
      partitionGroupFiles(established, index + 1 + '/4'));
  }
  assert.deepEqual(partitions.flat().sort(), files);
  assert.ok(partitions[2].includes(pauseTests[0]));
  assert.ok(partitions[3].includes(pauseTests[1]));
  for (const file of ['project-data-fixture-links-storage.test.js', 'issue-195-acceptance-gate-mutations.test.js']) {
    assert.equal(partitions.filter(partition => partition.includes(file)).length, 1);
  }
  const subset = [...pauseTests, 'issue-195-evidence-tooling-partitions.test.js', 'project-data-fixture-links-storage.test.js'].sort();
  const subsetPartitions = [1, 2, 3, 4].map(index => partitionGroupFiles(subset, `${index}/4`));
  assert.ok(subsetPartitions.every(partition => partition.length === 1));
  assert.deepEqual(subsetPartitions.flat().sort(), subset);
});

test('the certified translation pause preserves its complete original modulo assignments', () => {
  const path = 'js/scripts/test-groups.mjs';
  const source = restoreCiIntegrationSource(read(path), path, 'translationPause');
  const start = source.indexOf('export function partitionGroupFiles');
  const end = source.indexOf('\nif (process.argv', start);
  assert.ok(start >= 0 && end > start);
  const partition = runInNewContext(`(${source.slice(start + 'export '.length, end)})`);
  const previousFiles = execFileSync('git', ['--no-replace-objects', 'ls-tree', '-r', '--name-only',
    ciIntegrationParents.translationPause, '--', 'js/tests'], { cwd: root, encoding: 'utf8' })
    .trim().split('\n').filter(file => /^js\/tests\/[^/]+\.test\.js$/u.test(file)).map(file => file.slice('js/tests/'.length));
  const { tooling } = partitionTests(previousFiles.sort()).groups;
  const pause = 'issue-195-evidence-automated-translation-pause.test.js';
  const rollout = 'issue-195-evidence-automated-translation-rollout.test.js';
  const established = tooling.filter(file => file !== pause && file !== rollout);
  const shards = [1, 2, 3, 4].map(index => Array.from(partition(tooling, `${index}/4`)));
  for (const [index, shard] of shards.entries()) {
    assert.deepEqual(shard.filter(file => file !== pause && file !== rollout), Array.from(partition(established, `${index + 1}/4`)));
  }
  assert.deepEqual(shards.flat().sort(), tooling);
  assert.ok(shards[2].includes(pause));
  assert.ok(shards[3].includes(rollout));
  assert.ok(shards[1].includes('project-data-fixture-links-storage.test.js'));
  assert.ok(shards[3].includes('issue-195-acceptance-gate-mutations.test.js'));
});
const observe = (scope, assertions, name) =>
  recordIssue195DirectiveObservation(`I195-AUTOMATED-TRANSLATION-PAUSE-${scope}`, assertions, name);

test('workflow scheduling uses Linux while preserving every original platform cell', (context) => {
  const rustFile = '.github/workflows/rust.yml';
  const previousWorkflow = restoreAutomatedTranslationPauseSource(workflow('rust'), rustFile);
  assert.notEqual(previousWorkflow, workflow('rust'));
  const original = parseWorkflow(previousWorkflow).find(({ id }) => id === 'test').header;
  const [, platforms] = /^        os: \[([^\]]+)\]$/mu.exec(original);
  const [, suites] = /^        suite: \[([^\]]+)\]$/mu.exec(original);
  const originalPlatforms = platforms.split(', ');
  const primary = originalPlatforms.flatMap(os => suites.split(', ').map(suite => {
    const [, filters] = new RegExp(`^          - suite: ${suite}\\n            filters: (.+)$`, 'mu').exec(original);
    return { os, suite, filters };
  }));
  const explicit = [...original.matchAll(/^          - suite: physical\n            os: (.+)\n            logical_suite: (.+)\n            physical_shard: (\d+)\n            filters: (.+)$/gmu)]
    .map(([, os, logical_suite, shard, filters]) => ({ os, suite: 'physical', logical_suite, physical_shard: Number(shard), filters }));
  const cells = [...primary, ...explicit];
  assert.equal(cells.length, 82);
  const encode = rows => rows.map(({ os, suite, logical_suite, physical_shard, filters }) =>
    JSON.stringify([os, suite, logical_suite, physical_shard, filters])).sort();
  const policy = readPhysicalShardPolicy();
  assert.deepEqual(policy.platforms, originalPlatforms);
  assert.deepEqual(encode(nativeTestMatrix(policy).include), encode(cells));
  const oldCi = restoreAutomatedTranslationPauseSource(workflow('ci'), '.github/workflows/ci.yml');
  const originalConsumers = parseWorkflow(oldCi).find(({ id }) => id === 'consumers').header;
  assert.deepEqual(/^        os: \[([^\]]+)\]$/mu.exec(originalConsumers)[1].split(', '), policy.platforms);

  const selection = step(rust.check, 'Select native test platforms before matrix expansion');
  const consumers = step(ci.candidates, 'Select active clean consumer platforms');
  assert.equal(selection.run.trim(), 'node js/scripts/rust-test-shards.mjs --workflow-matrix test');
  assert.equal(consumers.run.trim(), 'node js/scripts/rust-test-shards.mjs --workflow-matrix consumer');
  assert.match(rust.check.header, /test_matrix: \$\{\{ steps\.test-matrix\.outputs\.matrix \}\}/u);
  assert.match(rust.test.header, /matrix: \$\{\{ fromJSON\(needs\.check\.outputs\.test_matrix\) \}\}/u);
  assert.match(ci.candidates.header, /consumer-matrix: \$\{\{ steps\.consumer-matrix\.outputs\.matrix \}\}/u);
  assert.match(ci.consumers.header, /matrix: \$\{\{ fromJSON\(needs\.candidates\.outputs\.consumer-matrix\) \}\}/u);
  assert.doesNotMatch(selection.text + consumers.text, /automated_translation|META_LANGUAGE_AUTOMATED_TRANSLATION/u);

  const directory = mkdtempSync(path.join(tmpdir(), 'meta-language-workflow-platforms-'));
  context.after(() => rmSync(directory, { recursive: true }));
  const output = path.join(directory, 'output');
  const command = 'js/scripts/rust-test-shards.mjs';
  const linuxCells = cells.filter(({ os }) => os === 'ubuntu-latest');
  assert.equal(linuxCells.length, 30);
  for (const automated of ['false', 'true']) {
    for (const [job, expected] of [['test', { include: linuxCells }], ['consumer', { os: ['ubuntu-latest'] }]]) {
      writeFileSync(output, '');
      execFileSync(process.execPath, [command, '--workflow-matrix', job], {
        cwd: root, env: { ...process.env, GITHUB_OUTPUT: output, META_LANGUAGE_AUTOMATED_TRANSLATION: automated },
      });
      const result = /^matrix=(.*)\n$/u.exec(readFileSync(output, 'utf8'))[1];
      const actual = JSON.parse(result);
      if (job === 'test') assert.deepEqual(encode(actual.include), encode(expected.include));
      else assert.deepEqual(actual, expected);
    }
  }

  const verification = step(ci.acceptance, 'Verify Linux-only workflow scheduling');
  assert.match(verification.text, /if: \$\{\{ !cancelled\(\) && needs\.candidates\.result == 'success' \}\}/u);
  assert.match(verification.run, /actions\/runs\/\$GITHUB_RUN_ID\/attempts\/\$GITHUB_RUN_ATTEMPT\/jobs\?per_page=100" --paginate --slurp/u);
  assert.match(verification.run, /node js\/scripts\/rust-test-shards\.mjs --verify-workflow-platforms/u);
  const completed = (name, labels = ['ubuntu-latest'], conclusion = 'success') =>
    ({ name, labels, status: 'completed', conclusion });
  const jobs = [
    completed('Clean Consumer (ubuntu-latest)'),
    completed('Rust / Cargo Check'),
    ...linuxCells.map(row => completed(`Rust / Test (${row.os}, ${row.logical_suite ?? row.suite})`)),
    completed('Clean Consumer (windows-latest)', [], 'skipped'),
  ];
  assert.deepEqual(verifyWorkflowPlatforms(jobs, policy), { nativeTests: 30, cleanConsumers: 1 });
  assert.deepEqual(verifyWorkflowPlatforms([jobs[0]], policy), { nativeTests: 0, cleanConsumers: 1 });
  assert.throws(() => readShardAdmission(jobs, policy), /missing or duplicate scheduled physical jobs/u);
  assert.throws(() => verifyWorkflowPlatforms(jobs.slice(0, -2), policy), /missing or extra Rust \/ Test jobs/u);
  assert.throws(() => verifyWorkflowPlatforms([...jobs, completed('Clean Consumer (macos-latest)')], policy), /macOS runner work was admitted/u);
  assert.throws(() => verifyWorkflowPlatforms([...jobs.slice(0, -2), { ...jobs.at(-2), labels: ['windows-latest'] }], policy), /macOS runner work was admitted/u);
  assert.throws(() => verifyWorkflowPlatforms([...jobs.slice(0, -2), { ...jobs.at(-2), labels: ['self-hosted'] }], policy), /missing Linux runner labels/u);
  assert.throws(() => verifyWorkflowPlatforms([...jobs.slice(0, -2), { ...jobs.at(-2), conclusion: 'failure' }], policy), /unsuccessful Rust \/ Test jobs/u);
  const metadata = path.join(directory, 'jobs.json');
  writeFileSync(metadata, JSON.stringify([{ jobs: jobs.slice(0, 5) }, { jobs: jobs.slice(5) }]));
  assert.match(execFileSync(process.execPath, [command, '--verify-workflow-platforms', metadata], { cwd: root, encoding: 'utf8' }),
    /Verified 30 Linux native test jobs and 1 Linux clean consumer jobs/u);
  recordIssue195DirectiveObservation('I195-ISSUE-197-CI-PLATFORMS',
    ['workflowsScheduleOnlyLinux', 'policyRetainsOriginalPlatformCells', 'aggregateVerifiesExactPlatformJobs'],
    'workflow scheduling uses Linux while preserving every original platform cell');
});

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

test('the native raw observation oracle checks the exact opt-in and independent prerequisites', () => {
  const oracle = read('rust/tests/unit/ci-cd/workflow_release.rs');
  const expected = /let prerequisites = r"([\s\S]+?)";/u.exec(oracle)?.[1];
  const { header } = rust['original-link-raw-observations'];
  assert.ok(expected && header.includes(expected));
  const needs = {
    'cargo-lock': { result: 'success' }, check: { result: 'success' }, changelog: { result: 'success' },
    'detect-changes': { outputs: {} },
  };
  const context = { needs, github: { event_name: 'push' } };
  const optIn = { ...context, inputs: { automated_translation: true } };
  assert.equal(when(header, context), false);
  assert.equal(when(header, optIn), true);
  assert.equal(when(header, { ...optIn, cancelled: () => true }), false);
  assert.equal(when(header, { ...optIn, needs: { ...needs, check: { result: 'failure' } } }), false);
});

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
    ['Run generated owned Rust module experiment', 'generated', 'third'],
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
});

test('strict commands and native selectors remain available without claiming paused evidence', (context) => {
  const dependencyCommands = {
    consumers: ['Install and parse the exact artifacts in clean consumers'],
    'formal-ai-workload-parts': ['Identify the exact Formal AI build cache'],
    'native-translations': ['Setup Lean'],
    'javascript-suite-independent': ['Setup Lean'],
    'javascript-suite': ['Setup Lean'],
    'rust-suite': ['Setup Lean'],
    acceptance: ['Provision the authenticated grammar input for manifest validation'],
    'translation-test-feedback': ['Merge exact-commit translation feedback without overwriting conflicts'],
    'native-grammar-outcome-experiment': ['Compile and execute the bounded frozen native candidate'],
  };
  for (const name of ['ci', 'js', 'rust']) {
    const file = `.github/workflows/${name}.yml`;
    const current = workflow(name);
    const predecessor = restoreAutomatedTranslationPauseSource(current, file);
    const dependencyWorkflow = restoreCiIntegrationSource(current, file, 'dependencies');
    const dependencyJobs = parseWorkflow(dependencyWorkflow);
    assert.notEqual(dependencyWorkflow, current);
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
        } else if (dependencyCommands[oldJob.id]?.includes(oldStep.name)) {
          const previous = step(dependencyJobs.find(job => job.id === oldJob.id), oldStep.name);
          const currentRun = step(jobs(name)[oldJob.id], oldStep.name).run.trim();
          if (oldStep.name === 'Identify the exact Formal AI build cache') {
            const newestInRange = "grep -Fx 'release: 1.99.1'";
            assert.equal(currentRun.split(newestInRange).length, 2);
            assert.equal(currentRun.replace(newestInRange, "grep -Fx 'release: 1.100.0'"), previous.run.trim());
          } else {
            assert.equal(currentRun, previous.run.trim());
          }
        } else if (oldStep.name === 'Check that every dependency, toolchain, action and image is inventoried at its current stable release or records why not') {
          assert.equal(step(js['package-checks'], 'Check direct dependency freshness and the recorded dependency audit').run.trim(), oldStep.run.trim());
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
