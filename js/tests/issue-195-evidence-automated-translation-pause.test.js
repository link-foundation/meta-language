import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdtempSync, readFileSync, readdirSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';
import { runInNewContext } from 'node:vm';

import { captureDiagnosticDependencyProfile, diagnosticFixturePath } from '../../experiments/grammar-outcome-network/native-transport/diagnostic-order/generate.mjs';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';
import { nativeLintCaptureSources, nativePlatformPausePredecessor, restoreAutomatedTranslationPauseSource } from '../scripts/automated-translation-pause-source-history.mjs';
import { collectDependencies } from '../scripts/dependency-inventory.mjs';
import { supportedPlatformCoverage } from '../scripts/issue-195-delivery.mjs';
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
    assert.match(step(job, 'Setup Rust').text, /^\s*uses: dtolnay\/rust-toolchain@1\.99\.0$/mu, job.id);
  }
  assert.match(step(rust.lint, 'Setup Rust').text, /^\s*uses: dtolnay\/rust-toolchain@stable$/mu);
  assert.match(step(experiment, 'Compile and execute the bounded frozen native candidate').run,
    /native-transport\/runner\/run\.mjs --native$/mu);
});

test('native job-scoped cache assertions cover the toolchain-specific target keys', () => {
  const oracle = read('rust/tests/unit/ci-cd/workflow_release.rs');
  const cacheTest = oracle.split('fn cargo_cache_keys_are_scoped_by_job() {')[1].split('\n#[test]')[0];
  for (const cache of [
    step(rust.check, 'Cache cargo registry'),
    step(rust.test, 'Cache cargo registry and target on Unix'),
  ]) {
    const key = cache.text.split('\n').find(line => line.trimStart().startsWith('key: ')).trim();
    assert.ok(key.includes('-1.99.0-'), cache.name);
    assert.ok(cacheTest.includes(`"${key}"`), cache.name);
    assert.ok(!cacheTest.includes(`"${key.replace('-1.99.0-', '-')}"`), cache.name);
  }
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
  assert.ok(partitions[1].includes('project-data-fixture-links-storage.test.js'));
  assert.ok(partitions[3].includes('issue-195-acceptance-gate-mutations.test.js'));
});
const observe = (scope, assertions, name) =>
  recordIssue195DirectiveObservation(`I195-AUTOMATED-TRANSLATION-PAUSE-${scope}`, assertions, name);

test('retained matrix runners keep their workflow dependency ownership during the pause', () => {
  const collected = collectDependencies(fileURLToPath(root)).filter(item => item.category === 'runner');
  const inventory = readProjectData(new URL('parity/dependency-inventory.lino', root));
  const fields = ({ id, declaredIn, pinned, compare, source }) => ({ id, declaredIn, pinned, compare, source });
  for (const platform of readPhysicalShardPolicy().platforms) {
    const stored = inventory.items.find(item => item.category === 'runner' && item.name === platform);
    const runner = collected.find(item => item.name === platform);
    assert.ok(runner, `${platform} remains a declared workflow runner`);
    assert.ok(stored, `${platform} retains its audited inventory entry`);
    assert.ok(runner.declaredIn.includes('.github/workflows/ci.yml'));
    assert.ok(runner.declaredIn.includes('.github/workflows/rust.yml'));
    assert.deepEqual(fields(runner), fields(stored));
  }
});

test('Linux scheduling preserves every native platform definition and all strict evidence requirements', (context) => {
  const previousJobs = name => {
    const file = `.github/workflows/${name}.yml`;
    const previous = restoreAutomatedTranslationPauseSource(workflow(name), file, nativePlatformPausePredecessor);
    assert.notEqual(previous, workflow(name));
    return Object.fromEntries(parseWorkflow(previous).map(job => [job.id, job]));
  };
  const savedRust = previousJobs('rust').test;
  const savedConsumer = previousJobs('ci').consumers;
  const platforms = /^ {8}os: \[([^\]]+)\]$/mu;
  const originalPlatforms = platforms.exec(savedRust.header)[1].split(', ');
  assert.deepEqual(originalPlatforms, platforms.exec(savedConsumer.header)[1].split(', '));
  const suites = [...savedRust.header.matchAll(/^ {10}- suite: ((?!physical$)[\w-]+)\n {12}filters: (.+)$/gmu)];
  const baseCells = originalPlatforms.flatMap(os => suites.map(([, suite, filters]) => ({ os, suite, filters })));
  const extraCells = [...savedRust.header.matchAll(/^ {10}- suite: physical\n {12}os: (.+)\n {12}logical_suite: (.+)\n {12}physical_shard: (\d+)\n {12}filters: (.+)$/gmu)]
    .map(([, os, logical_suite, shard, filters]) => ({ os, suite: 'physical', logical_suite, physical_shard: Number(shard), filters }));
  assert.equal(baseCells.length, 15);
  assert.equal(extraCells.length, 67);
  const policy = readPhysicalShardPolicy();
  const retained = nativeTestMatrix(policy).include;
  const sorted = cells => cells.map(cell => JSON.stringify(cell)).sort();
  assert.deepEqual(sorted(retained), sorted([...baseCells, ...extraCells]));
  assert.deepEqual(rust.test.steps, savedRust.steps);
  assert.deepEqual(ci.consumers.steps, savedConsumer.steps);

  const selector = 'node js/scripts/rust-test-shards.mjs --workflow-matrix ';
  assert.equal(step(rust.check, 'Select active native test platforms').run.trim(), selector + 'test');
  assert.equal(step(ci.candidates, 'Select active clean consumer platforms').run.trim(), selector + 'consumer');
  assert.match(rust.check.header, /test-matrix: \$\{\{ steps\.test-matrix\.outputs\.matrix \}\}/u);
  assert.match(ci.candidates.header, /consumer-matrix: \$\{\{ steps\.consumer-matrix\.outputs\.matrix \}\}/u);
  assert.match(rust.test.header, /matrix: \$\{\{ fromJSON\(needs\.check\.outputs\.test-matrix\) \}\}/u);
  assert.match(ci.consumers.header, /matrix: \$\{\{ fromJSON\(needs\.candidates\.outputs\.consumer-matrix\) \}\}/u);
  const workflowFiles = readdirSync(new URL('.github/workflows/', root)).filter(file => /\.ya?ml$/u.test(file)).sort();
  const dynamicRunners = workflowFiles.flatMap(file => parseWorkflow(read(`.github/workflows/${file}`)).flatMap(job => {
    const runner = /^ {4}runs-on: (.+)$/mu.exec(job.header)?.[1];
    return runner && runner !== 'ubuntu-latest' ? [[file, job.id, runner]] : [];
  }));
  assert.deepEqual(dynamicRunners, [['ci.yml', 'consumers', '${{ matrix.os }}'], ['rust.yml', 'test', '${{ matrix.os }}']]);
  const linuxCells = retained.filter(cell => cell.os === 'ubuntu-latest');
  assert.equal(linuxCells.length, 30);
  const directory = mkdtempSync(path.join(tmpdir(), 'meta-language-ci-platforms-'));
  try {
    for (const translationMode of ['false', 'true']) {
      for (const [job, matrix] of [['test', { include: linuxCells }], ['consumer', { os: ['ubuntu-latest'] }]]) {
        const output = path.join(directory, `${job}-${translationMode}`);
        const run = spawnSync(process.execPath, ['js/scripts/rust-test-shards.mjs', '--workflow-matrix', job], {
          cwd: fileURLToPath(root), encoding: 'utf8',
          env: { ...process.env, GITHUB_OUTPUT: output, META_LANGUAGE_AUTOMATED_TRANSLATION: translationMode },
        });
        assert.equal(run.status, 0, run.stderr);
        assert.equal(readFileSync(output, 'utf8'), `matrix=${JSON.stringify(matrix)}\n`);
      }
    }
  } finally {
    rmSync(directory, { recursive: true });
  }

  const linuxJobs = linuxCells.map(({ os, suite, logical_suite }) => ({
    name: `Rust / Test (${os}, ${logical_suite ?? suite})`, status: 'completed', conclusion: 'success',
  }));
  const settled = (name, conclusion = 'success', labels = ['ubuntu-latest']) => ({ name, status: 'completed', conclusion, labels });
  const consumer = settled('Clean Consumer (ubuntu-latest)');
  const selectedJobs = [consumer, settled('Rust / Cargo Check'), ...linuxJobs];
  assert.deepEqual(verifyWorkflowPlatforms(selectedJobs, policy), { nativeTests: 30, cleanConsumers: 1 });
  assert.deepEqual(verifyWorkflowPlatforms([consumer, settled('Rust / Cargo Check', 'skipped')], policy), { nativeTests: 0, cleanConsumers: 1 });
  assert.throws(() => verifyWorkflowPlatforms(selectedJobs.slice(1), policy), /missing or extra Clean Consumer/u);
  assert.throws(() => verifyWorkflowPlatforms(selectedJobs.slice(0, -1), policy), /missing or extra.*Test/u);
  assert.throws(() => verifyWorkflowPlatforms([...selectedJobs, selectedJobs.at(-1)], policy), /missing or extra.*Test/u);
  assert.throws(() => verifyWorkflowPlatforms([settled(consumer.name, 'failure'), ...selectedJobs.slice(1)], policy), /unsuccessful.*Consumer/u);
  assert.throws(() => verifyWorkflowPlatforms([...selectedJobs, settled('Clean Consumer (macos-latest)')], policy), /runner work was admitted/u);
  assert.throws(() => verifyWorkflowPlatforms([...selectedJobs, settled('Opaque job label', 'success', ['windows-2025'])], policy), /runner work was admitted/u);
  assert.match(step(ci.acceptance, 'Verify Linux-only workflow scheduling').run, /attempts\/\$GITHUB_RUN_ATTEMPT\/jobs\?per_page=100.*\n.*--verify-workflow-platforms/u);
  assert.throws(() => readShardAdmission(linuxJobs, policy), /missing or duplicate scheduled physical jobs/u);
  const corpus = readProjectData(new URL('parity/fixtures/issue-195-evidence.json', root));
  const linuxReport = { platform: 'linux-x64', corpusSha256: 'fixture', agreement: { mismatches: [] } };
  const coverage = supportedPlatformCoverage([linuxReport], [], {}, { corpus, corpusSha256: 'fixture' });
  assert.deepEqual(coverage, { observed: false, failures: [
    { platform: 'darwin-arm64', reason: 'no consumer report' }, { platform: 'win32-x64', reason: 'no consumer report' },
  ] });
  const assertions = [
    'workflowsScheduleOnlyLinuxRunners', 'originalPlatformMatrixAndCommandsRetained', 'strictPlatformEvidenceStillRequired',
  ];
  for (const requirementId of ['I195-CI-NATIVE-PLATFORM-PAUSE', 'I195-CI-NON-LINUX-PAUSE']) {
    recordIssue195DirectiveObservation(requirementId, assertions, context.name);
  }
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
});

test('native host replays require the exact current lint-corrected source', () => {
  for (const file of nativeLintCaptureSources) {
    const current = read(file);
    const predecessor = restoreAutomatedTranslationPauseSource(current, file);
    assert.notEqual(predecessor, current, file);
    for (const altered of [current + '\n', current.replace('fn ', '// Unreviewed edit\nfn ')]) {
      assert.notEqual(altered, current, file);
      assert.equal(restoreAutomatedTranslationPauseSource(altered, file), altered, file);
    }
    assert.equal(restoreAutomatedTranslationPauseSource(current, 'rust/tests/unit/unrelated.rs'), current);
    assert.equal(restoreAutomatedTranslationPauseSource(predecessor, file), predecessor);
  }
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
  const platformInstruction = evidence.platformInstruction;
  const platformSource = register.requirementSources.find(({ id }) => id === platformInstruction.id);
  assert.equal(platformInstruction.url, platformSource.url);
  assert.equal(createHash('sha256').update(platformInstruction.body).digest('hex'), platformSource.sha256);
  assert.equal(Buffer.byteLength(platformInstruction.body), platformSource.bytes);
  assert.ok(evidence.baseline.observations.every(({ job, url, excerpts }) =>
    url.endsWith('/job/' + job) && excerpts.every(({ line, text }) => line > 0 && text.length > 0)));
  const study = read(folder + 'README.md');
  for (const requirement of ['Pause required automated translation', 'Keep the code, checks, tests',
    'Disable failing automated translation', 'Permit manual translation', 'Apply the pause to all open pull requests',
    'Pause Windows and macOS CI execution', 'Capture data in the requested folder', 'Deliver all changes in one pull request']) {
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
