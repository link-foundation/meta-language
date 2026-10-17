import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { copyFile, mkdir, mkdtemp, readFile, realpath, rm, writeFile } from 'node:fs/promises';
import { readFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { test } from 'node:test';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';
import { BULK_TRANSLATION_RULE_CHECKS, runBulkTranslationRuleChecks } from '../scripts/check-bulk-translation-rules.mjs';
import { writeProjectData } from '../scripts/project-data.mjs';
import { grammarSelectionFixture } from './support/grammar-selection-cases.js';

const repository = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');

test('bulk translation freshness includes every new native family without repeating existing generation gates', () => {
  const previousChecks = BULK_TRANSLATION_RULE_CHECKS.slice(0, 16);
  const checks = new Map(previousChecks.map(check => [path.basename(check.script), check.arguments]));
  const complete = new Map(BULK_TRANSLATION_RULE_CHECKS.map(check => [path.basename(check.script), check.arguments]));
  const defaultNative = ['array-value', 'callable-source', 'optional', 'javascript-surface', 'primitive-method', 'surface-raw-metadata', 'surface-reference-analysis'];
  const explicitNative = ['record-value', 'record-emission', 'record-type-syntax', 'type-syntax', 'grammar-lexer', 'grammar-selection'];
  assert.equal(checks.size, previousChecks.length);
  assert.equal(checks.size, 16);
  assert.equal(complete.size, BULK_TRANSLATION_RULE_CHECKS.length);
  assert.deepEqual(previousChecks, [
    ...defaultNative.map(family => ({ script: `js/scripts/generate-${family}-rules.mjs`, arguments: ['--check'] })),
    ...explicitNative.map(family => ({ script: `js/scripts/generate-${family}-rules.mjs`, arguments: ['--native', '--check'] })),
    ...['generate-grammar-selection-fixtures.mjs', 'generate-private-output-test-model.mjs', 'extend-typed-program-schema-second-stage.mjs']
      .map(name => ({ script: `js/scripts/${name}`, arguments: ['--check'] })),
  ], 'retain the complete sixteen-command prefix and every original flag in order');
  assert.deepEqual(BULK_TRANSLATION_RULE_CHECKS.slice(16, 18), [
    { script: 'js/scripts/generate-owned-array-append-rules.mjs', arguments: ['--native', '--check'] },
    { script: 'js/scripts/generate-nullable-type-syntax-rules.mjs', arguments: ['--native', '--check'] },
  ], 'additional source packets append without replacing an existing freshness scope');
  assert.deepEqual(BULK_TRANSLATION_RULE_CHECKS.slice(18, 19), [
    { script: 'js/scripts/generate-callable-emission-rules.mjs', arguments: ['--native', '--check'] },
  ], 'callable emission appends after every retained eighteen-command scope');
  assert.deepEqual(BULK_TRANSLATION_RULE_CHECKS.slice(19, 20), [
    { script: 'js/scripts/generate-string-code-unit-template.mjs', arguments: ['--check'] },
  ], 'the complete code-unit template appends after every retained nineteen-command scope');
  assert.deepEqual(BULK_TRANSLATION_RULE_CHECKS.slice(20, 21), [
    { script: 'js/scripts/extend-typed-program-schema-fourth-stage.mjs', arguments: ['--check'] },
  ], 'the current schema successor authenticates strict equality and every retained predecessor scope');
  assert.deepEqual(BULK_TRANSLATION_RULE_CHECKS.slice(21, 23), [
    { script: 'js/scripts/generate-grammar-selection-mapping-rules.mjs', arguments: ['--native', '--check'] },
    { script: 'js/scripts/generate-grammar-selection-mapping-fixtures.mjs', arguments: ['--check'] },
  ], 'mapped selection appends source and fixture checks after every retained predecessor scope');
  assert.deepEqual(BULK_TRANSLATION_RULE_CHECKS.slice(23), [
    { script: 'js/scripts/generate-rust-character-rules.mjs', arguments: ['--native', '--check'] },
    { script: 'js/scripts/generate-javascript-tag-context-rules.mjs', arguments: ['--native', '--check'] },
    { script: 'js/scripts/generate-rust-character-literal-fixtures.mjs', arguments: ['--check'] },
  ], 'character and constructor-context checks append after the complete retained twenty-three-command prefix');
  assert.equal(BULK_TRANSLATION_RULE_CHECKS.length, 26, 'the complete source freshness gate retains exactly its reviewed scope');
  for (const family of defaultNative) assert.deepEqual(checks.get(`generate-${family}-rules.mjs`), ['--check']);
  for (const family of explicitNative) assert.deepEqual(checks.get(`generate-${family}-rules.mjs`), ['--native', '--check']);
  for (const name of ['generate-grammar-selection-fixtures.mjs', 'generate-private-output-test-model.mjs', 'extend-typed-program-schema-second-stage.mjs']) {
    assert.deepEqual(checks.get(name), ['--check']);
  }
  const previousWorkflow = readFileSync(path.join(repository, '.github/workflows/js.yml'), 'utf8');
  for (const name of ['generate-typed-program-link-rules', 'generate-link-network-binary-record-rules', 'generate-scalar-loop-rules',
    'generate-rust-product-rules', 'generate-product-source-rules', 'generate-primitive-value-rules', 'generate-rust-array-rules',
    'generate-rust-borrowed-call-interface-rules', 'generate-rust-empty-option-rules']) {
    assert.equal(checks.has(`${name}.mjs`), false, `${name} retains its existing owner`);
    assert.equal(previousWorkflow.includes(`${name}.mjs --check`), true, `${name} still runs in the prior workflow`);
  }
  assert.equal(previousWorkflow.includes('npm run check:frontend-rules'), true);
  for (const check of BULK_TRANSLATION_RULE_CHECKS) {
    const source = readFileSync(path.join(repository, check.script), 'utf8');
    assert.equal(source.includes('--check'), true, `${check.script} actually implements check mode`);
    if (check.script === 'js/scripts/generate-javascript-tag-context-rules.mjs') {
      assert.equal(source.includes('const args = process.argv.slice(2);'), true, check.script);
      assert.equal(source.includes("check: args.includes('--check'), native: args.includes('--native')"), true, check.script);
    } else if (check.arguments.includes('--native')) assert.equal(source.includes("process.argv.includes('--native')"), true, check.script);
  }
});

test('bulk translation freshness runs serially and reports every spawn and native-output failure', async () => {
  const checks = BULK_TRANSLATION_RULE_CHECKS.slice(0, 4), calls = [], messages = [];
  let active = 0, maximum = 0;
  const result = await runBulkTranslationRuleChecks({ checks, report: message => messages.push(message), invoke: async (command, args, options) => {
    const index = calls.length;
    calls.push({ command, args, options });
    active += 1; maximum = Math.max(maximum, active);
    await Promise.resolve(); active -= 1;
    if (index === 0) throw new Error('actual process could not start');
    if (index === 1) return { exitCode: 1, signal: null };
    if (index === 2) return { exitCode: null, signal: 'SIGTERM' };
    return { exitCode: 0, signal: null };
  } });
  assert.equal(maximum, 1);
  assert.equal(calls.length, checks.length);
  assert.equal(result.checks, checks.length);
  assert.deepEqual(result.failures.map(failure => failure.script), checks.slice(0, 3).map(check => check.script));
  assert.equal(messages.filter(message => message.startsWith('Translation rule freshness FAILED:')).length, 3);
  assert.match(messages.at(-1), /4 checks, 3 failures/u);
  for (let index = 0; index < calls.length; index += 1) {
    assert.equal(calls[index].command, process.execPath);
    assert.deepEqual(calls[index].args, [path.join(repository, checks[index].script), ...checks[index].arguments]);
  }
  const rejected = await runBulkTranslationRuleChecks({ checks: [{ script: 'js/scripts/generate-grammar-selection-fixtures.mjs', arguments: ['--native'] }],
    report: () => {}, invoke: () => { assert.fail('a command without check mode must never run'); } });
  assert.equal(rejected.failures.length, 1);
});

test('the actual unchanged fixture generator rejects missing and stale outputs without compiling Rust', async () => {
  // Node canonicalizes the entry module URL. macOS /var temporary paths can
  // otherwise differ from their /private/var argv spelling and bypass the
  // actual producer's existing direct-entry guard before any assertion runs.
  const root = await realpath(await mkdtemp(path.join(tmpdir(), 'meta-language-bulk-freshness-')));
  const producer = 'js/scripts/generate-grammar-selection-fixtures.mjs';
  const check = BULK_TRANSLATION_RULE_CHECKS.find(check => check.script === producer);
  const diagnostics = [], outputs = [];
  const invoke = (command, args, options) => {
    const actual = spawnSync(command, args, { ...options, encoding: 'utf8', maxBuffer: 256 * 1024, timeout: 10000 });
    if (actual.error) throw actual.error;
    diagnostics.push(actual.stderr);
    outputs.push(actual.stdout);
    return { exitCode: actual.status, signal: actual.signal };
  };
  try {
    await mkdir(path.join(root, 'js/scripts'), { recursive: true });
    await mkdir(path.join(root, 'js/tests/support'), { recursive: true });
    await copyFile(path.join(repository, producer), path.join(root, producer));
    assert.deepEqual(await readFile(path.join(root, producer)), await readFile(path.join(repository, producer)), 'the actual generator remains byte-for-byte unchanged');
    // The temporary Node package is an external module boundary. Its two
    // adapters import the actual authoring dependencies, without replacement
    // fixture or serialization semantics and without cloning the repository.
    await writeFile(path.join(root, 'js/package.json'), '{"type":"module"}\n');
    await writeFile(path.join(root, 'js/scripts/project-data.mjs'), `export * from ${JSON.stringify(new URL('../scripts/project-data.mjs', import.meta.url).href)};\n`);
    await writeFile(path.join(root, 'js/tests/support/grammar-selection-cases.js'), `export * from ${JSON.stringify(new URL('./support/grammar-selection-cases.js', import.meta.url).href)};\n`);
    const run = () => runBulkTranslationRuleChecks({ root, checks: [check], invoke, report: () => {} });
    const missing = await run();
    assert.equal(missing.failures.length, 1);
    assert.match(diagnostics.at(-1), /ENOENT|no such file/u);
    const file = path.join(root, 'parity/fixtures/grammar-recursive-selection.lino');
    const stale = grammarSelectionFixture(); stale.chain.source += ' changed';
    writeProjectData(file, stale);
    const staleBytes = await readFile(file);
    const changed = await run();
    assert.equal(changed.failures.length, 1);
    assert.match(diagnostics.at(-1), /authored recursive grammar selection fixtures are stale/u);
    assert.deepEqual(await readFile(file), staleBytes, 'check mode cannot repair or rewrite a stale fixture');
    writeProjectData(file, grammarSelectionFixture());
    const freshBytes = await readFile(file);
    assert.deepEqual(await run(), { checks: 1, failures: [] });
    assert.match(outputs.at(-1), /authored recursive grammar selection fixture matches its source definitions/u);
    assert.deepEqual(await readFile(file), freshBytes);
  } finally { await rm(root, { recursive: true, force: true }); }
});

test('Rust lint runs complete source freshness after tools and existing checks even when a prior check fails', () => {
  const workflow = readFileSync(path.join(repository, '.github/workflows/rust.yml'), 'utf8');
  const job = parseWorkflow(workflow).find(job => job.id === 'lint');
  const freshness = job.steps.find(step => step.name === 'Check the complete bulk translation rule source freshness');
  assert.ok(freshness);
  assert.equal(job.steps.filter(step => step.name === freshness.name).length, 1);
  assert.equal(freshness.run.trim(), 'node js/scripts/check-bulk-translation-rules.mjs');
  assert.match(freshness.text, /if: \$\{\{ !cancelled\(\) \}\}/u);
  assert.match(freshness.text, /working-directory: \$\{\{ github.workspace \}\}/u);
  for (const name of ['Setup Rust', 'Setup Node.js for independent grammar sources', 'Install the linked grammar manifest reader dependencies',
    'Check formatting', 'Reject tests in src', 'Run Clippy', 'Check docs.rs feature set']) {
    const index = job.steps.findIndex(step => step.name === name);
    assert.ok(index >= 0 && index < job.steps.indexOf(freshness), `${name} remains before the added freshness gate`);
  }
  assert.equal(job.steps.find(step => step.name === 'Check formatting').run.trim(), 'cargo fmt --all -- --check');
  assert.equal(job.steps.find(step => step.name === 'Run Clippy').run.trim(), 'cargo clippy --all-targets --all-features');
  for (const step of job.steps.slice(job.steps.indexOf(freshness) + 1)) assert.equal(step.afterFailure, true, step.name);
});

test('the complete freshness CLI refuses selection and update arguments before running any family', () => {
  const actual = spawnSync(process.execPath, [path.join(repository, 'js/scripts/check-bulk-translation-rules.mjs'), '--only', 'one'],
    { encoding: 'utf8', maxBuffer: 256 * 1024, timeout: 10000 });
  assert.equal(actual.error, undefined);
  assert.equal(actual.status, 1);
  assert.match(actual.stderr, /accepts no selection or update arguments/u);
  assert.doesNotMatch(actual.stderr, /Translation rule freshness: node/u);
});
