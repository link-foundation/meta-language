import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdtempSync, mkdirSync, readFileSync, realpathSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { test } from 'node:test';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';
import { captureTranslationTestFeedbackPartition, mergeTranslationTestFeedbackPartitions } from '../scripts/translation-test-feedback.mjs';
import { readProjectData, writeProjectData } from '../scripts/project-data.mjs';
import { partitionGroupFiles, partitionTests, testFiles, TEST_GROUPS } from '../scripts/test-groups.mjs';

const commit = 'a'.repeat(40);
const receiptName = 'translation-test-feedback-partition.lino';
const compilerFile = 'issue-195-artifacts/compiler-feedback/example/round-0.lino';
const nativeFile = 'issue-195-results/artifacts/decimal-format-draft-native.lino';

function fixture(parts = [{ [compilerFile]: '(compiler feedback)\n' }, { [nativeFile]: '(native observation)\n' }, {}]) {
  const root = realpathSync(mkdtempSync(path.join(tmpdir(), 'translation-feedback-')));
  mkdirSync(path.join(root, 'js/tests'), { recursive: true });
  for (const name of ['a', 'b', 'c', 'd', 'e', 'f']) writeFileSync(path.join(root, `js/tests/translation-${name}.test.js`), '');
  const inputDirectory = path.join(root, '.issue-195-work/parts');
  const outputDirectory = path.join(root, '.issue-195-work/merged');
  const directories = parts.map((payloads, index) => {
    for (const source of ['issue-195-artifacts/compiler-feedback', nativeFile]) rmSync(path.join(root, source), { recursive: true, force: true });
    for (const [relative, bytes] of Object.entries(payloads)) {
      mkdirSync(path.dirname(path.join(root, relative)), { recursive: true });
      writeFileSync(path.join(root, relative), bytes);
    }
    const directory = path.join(inputDirectory, `translation-test-feedback-${commit}-${index + 1}-of-3`);
    captureTranslationTestFeedbackPartition({ root, commit, shard: `${index + 1}/3`, outputDirectory: directory });
    return directory;
  });
  return { root, commit, inputDirectory, outputDirectory, directories };
}

function editReceipt(directory, change) {
  const file = path.join(directory, receiptName);
  const receipt = readProjectData(file);
  change(receipt);
  writeProjectData(file, receipt);
}

test('all three exact scopes merge original feedback paths and bytes while a valid empty shard remains represented', () => {
  const value = fixture();
  try {
    const empty = readProjectData(path.join(value.directories[2], receiptName));
    assert.deepEqual(empty.payloads, []);
    assert.equal(empty.files.length, 2);
    const merged = mergeTranslationTestFeedbackPartitions(value);
    assert.deepEqual(merged, { commit, partitions: 3, files: [compilerFile, nativeFile].sort() });
    assert.equal(readFileSync(path.join(value.outputDirectory, compilerFile), 'utf8'), '(compiler feedback)\n');
    assert.equal(readFileSync(path.join(value.outputDirectory, nativeFile), 'utf8'), '(native observation)\n');
    assert.throws(() => readFileSync(path.join(value.outputDirectory, receiptName)), /ENOENT/u);
  } finally { rmSync(value.root, { recursive: true, force: true }); }
});

test('identical shared payload bytes deduplicate without selecting a conflicting shard winner', () => {
  const value = fixture([{ [compilerFile]: 'same' }, { [compilerFile]: 'same' }, {}]);
  try {
    assert.deepEqual(mergeTranslationTestFeedbackPartitions(value).files, [compilerFile]);
    assert.equal(readFileSync(path.join(value.outputDirectory, compilerFile), 'utf8'), 'same');
  } finally { rmSync(value.root, { recursive: true, force: true }); }
});

test('missing, stale, foreign, duplicate, tampered or changed-scope feedback cannot publish the final artifact', async context => {
  for (const [name, mutate, expected] of [
    ['missing partition', value => rmSync(value.directories[1], { recursive: true }), /partitions must be present/u],
    ['foreign partition', value => mkdirSync(path.join(value.inputDirectory, 'foreign')), /partitions must be present/u],
    ['stale commit', value => editReceipt(value.directories[0], receipt => { receipt.commit = 'b'.repeat(40); }), /provenance/u],
    ['wrong group', value => editReceipt(value.directories[0], receipt => { receipt.group = 'grammar'; }), /provenance/u],
    ['duplicate shard identity', value => editReceipt(value.directories[1], receipt => { receipt.shard = '1/3'; }), /provenance/u],
    ['missing file scope', value => editReceipt(value.directories[0], receipt => { receipt.files.pop(); }), /file scope/u],
    ['foreign file scope', value => editReceipt(value.directories[0], receipt => { receipt.files[0] = 'foreign.test.js'; }), /file scope/u],
    ['tampered payload', value => writeFileSync(path.join(value.directories[0], compilerFile), 'different bytes'), /differs from its receipt/u],
    ['duplicate payload', value => editReceipt(value.directories[0], receipt => { receipt.payloads.push(receipt.payloads[0]); }), /duplicate feedback payload/u],
    ['unregistered payload', value => writeFileSync(path.join(value.directories[0], 'unexpected'), 'extra'), /unregistered payloads/u],
    ['path traversal', value => editReceipt(value.directories[0], receipt => { receipt.payloads[0].path = '../outside'; }), /unexpected feedback payload path/u],
    ['symbolic link', value => symlinkSync(path.join(value.root, 'js'), path.join(value.directories[0], 'foreign')), /symbolic link/u],
  ]) {
    await context.test(name, () => {
      const value = fixture();
      try {
        mutate(value);
        assert.throws(() => mergeTranslationTestFeedbackPartitions(value), expected);
        assert.throws(() => readFileSync(path.join(value.outputDirectory, compilerFile)), /ENOENT/u,
          'validation happens before any merged output is published');
      } finally { rmSync(value.root, { recursive: true, force: true }); }
    });
  }
});

test('cross-shard conflicting bytes and a completely empty collection retain the original artifact failure', () => {
  for (const parts of [[{ [compilerFile]: 'first' }, { [compilerFile]: 'second' }, {}], [{}, {}, {}]]) {
    const value = fixture(parts);
    try { assert.throws(() => mergeTranslationTestFeedbackPartitions(value), /conflicting feedback payload|has no files/u); }
    finally { rmSync(value.root, { recursive: true, force: true }); }
  }
});

test('the real command-line merger preserves nested payload layout and rejects stale commit input', () => {
  const value = fixture();
  const script = fileURLToPath(new URL('../scripts/translation-test-feedback.mjs', import.meta.url));
  try {
    const options = ['merge', '--root', value.root, '--commit', commit, '--inputs', value.inputDirectory, '--out', value.outputDirectory];
    const merged = spawnSync(process.execPath, [script, ...options], { encoding: 'utf8' });
    assert.equal(merged.status, 0, merged.stderr);
    assert.match(merged.stdout, /merged 2 files from 3 exact-commit partitions/u);
    const stale = spawnSync(process.execPath, [script, ...options.map(value => value === commit ? 'b'.repeat(40) : value)], { encoding: 'utf8' });
    assert.equal(stale.status, 1);
    assert.match(stale.stderr, /partitions must be present/u);
  } finally { rmSync(value.root, { recursive: true, force: true }); }
});

function assertInitialTranslationShards(text) {
  const jobs = new Map(parseWorkflow(text).map(job => [job.id, job]));
  const tests = jobs.get('tests');
  const groups = /group: \[([^\]]+)\]/u.exec(tests.header)[1].split(', ');
  assert.deepEqual(groups, Object.keys(TEST_GROUPS).filter(group => group !== 'report-artifacts'));
  assert.match(tests.header, /partition: \[1\]/u);
  assert.doesNotMatch(tests.header, /exclude:/u);
  const include = [...tests.header.matchAll(/\{group: ([a-z-]+), partition: (\d+), shards: (\d+)\}/gu)]
    .map(([, group, partition, shards]) => ({ group, partition: Number(partition), shards: Number(shards) }));
  assert.deepEqual(include, [['grammar', 3], ['grammar-native', 3], ['conformance', 2], ['translation', 3]].flatMap(([group, shards]) =>
    Array.from({ length: shards }, (_, index) => ({ group, partition: index + 1, shards }))));
  const rows = new Map(groups.map(group => [`${group}/1`, { group, partition: 1, shards: 1 }]));
  for (const row of include) rows.set(`${row.group}/${row.partition}`, row);
  const actualGroups = partitionTests(testFiles()).groups;
  const executed = [...rows.values()].flatMap(({ group, partition, shards }) => partitionGroupFiles(actualGroups[group], `${partition}/${shards}`));
  assert.equal(new Set(executed).size, executed.length);
  assert.deepEqual([...executed].sort(), groups.flatMap(group => actualGroups[group]).sort());
  assert.match(tests.header, /timeout-minutes: 15\n/u);
  assert.match(tests.header, /fail-fast: false/u);
  const run = tests.steps.find(step => step.name === 'Run the test group');
  assert.ok(run.afterFailure);
  assert.match(run.text, /TEST_GROUP_SHARD: \$\{\{ matrix.partition \}\}\/\$\{\{ matrix.shards \|\| 1 \}\}/u);
  for (const flag of ['TRANSLATION_CHECK_RUST', 'SELF_TRANSLATION_CHECK_RUST']) {
    assert.ok(run.text.includes(`${flag}: \${{ matrix.group == 'translation' && '1' || '0' }}`));
  }
  assert.equal(run.run, 'node ../scripts/with-cache-cleanup.mjs --event test -- node scripts/test-groups.mjs --run "$TEST_GROUP"\n        working-directory: js\n\n');
  const capture = tests.steps.findIndex(step => step.name === 'Capture translation compiler feedback partition');
  const upload = tests.steps.findIndex(step => step.name === 'Retain translation compiler feedback and native observations');
  assert.ok(capture > tests.steps.indexOf(run) && upload > capture);
  assert.ok(tests.steps[capture].afterFailure && tests.steps[upload].afterFailure);
  assert.match(tests.steps[capture].run, /--commit "\$GITHUB_SHA" --shard "\$TRANSLATION_SHARD"/u);
  assert.match(tests.steps[upload].text, /name: translation-test-feedback-\$\{\{ github.sha \}\}-\$\{\{ matrix.partition \}\}-of-\$\{\{ matrix.shards \}\}/u);
  assert.match(tests.steps[upload].text, /translation-test-feedback-partition\.lino/u);
  for (const original of ['issue-195-artifacts/compiler-feedback', nativeFile]) assert.ok(tests.steps[upload].text.includes(original));
  assert.match(tests.steps[upload].text, /if-no-files-found: error/u);
  assert.match(tests.steps[upload].text, /retention-days: 7/u);

  const merge = jobs.get('translation-test-feedback');
  assert.match(merge.header, /needs: tests\n/u);
  assert.match(merge.header, /if: \$\{\{ !cancelled\(\) \}\}/u);
  assert.match(merge.header, /timeout-minutes: 5/u);
  const download = merge.steps.find(step => step.name === 'Download every translation feedback partition separately');
  assert.match(download.text, /pattern: translation-test-feedback-\$\{\{ github.sha \}\}-\*-of-3/u);
  assert.match(download.text, /merge-multiple: false/u);
  const combine = merge.steps.find(step => /translation-test-feedback\.mjs merge/u.test(step.run));
  assert.ok(combine.afterFailure);
  assert.match(combine.text, /shell: bash/u);
  assert.match(combine.run, /2>&1 \| tee .*merge\.log/u);
  const final = merge.steps.find(step => step.name === 'Retain the complete original translation feedback artifact');
  assert.match(final.text, /name: translation-test-feedback-\$\{\{ github.sha \}\}\n/u);
  assert.match(final.text, /path: \.issue-195-work\/translation-test-feedback\/merged\//u);
  assert.match(final.text, /if-no-files-found: error/u);
  assert.match(final.text, /retention-days: 7/u);
  const diagnostics = merge.steps.find(step => step.name === 'Retain translation feedback merge diagnostics');
  assert.ok(diagnostics.afterFailure);
  assert.match(diagnostics.text, /merge\.log/u);
  assert.ok(merge.steps.at(-1).afterFailure);
  assert.match(merge.steps.at(-1).run, /clean-caches\.mjs --event ci-teardown/u);
  const gate = jobs.get('test');
  assert.match(gate.header, /needs: \[tests, report-assertions, package-checks, translation-test-feedback\]/u);
  const require = gate.steps.find(step => step.name === 'Require every JavaScript test group');
  assert.match(require.text, /TRANSLATION_FEEDBACK_RESULT: \$\{\{ needs.translation-test-feedback.result \}\}/u);
  assert.match(require.run, /if \[ "\$TRANSLATION_FEEDBACK_RESULT" != "success" \]; then/u);
}

test('initial translation shards retain every file, native flag, original deadline and final feedback gate', () => {
  assertInitialTranslationShards(readFileSync(new URL('../../.github/workflows/js.yml', import.meta.url), 'utf8'));
  const acceptance = new Map(parseWorkflow(readFileSync(new URL('../../.github/workflows/ci.yml', import.meta.url), 'utf8')).map(job => [job.id, job]));
  assert.match(acceptance.get('javascript-suite-independent').header, /group: translation\n\s+shard: 1\/1\n\s+partition: 1-of-1/u);
  assert.match(acceptance.get('javascript-suite-independent').header, /timeout-minutes: 60/u);
});

test('omitted or duplicate initial shards, flattened feedback and a weakened final gate fail conservation', () => {
  const workflow = readFileSync(new URL('../../.github/workflows/js.yml', import.meta.url), 'utf8');
  const row = '          - {group: translation, partition: 2, shards: 3}\n';
  const grammarRow = '          - {group: grammar, partition: 2, shards: 3}\n';
  for (const mutate of [
    text => text.replace(row, ''), text => text.replace(row, row + row),
    text => text.replace(grammarRow, ''), text => text.replace(grammarRow, grammarRow + grammarRow),
    text => text.replace('{group: translation, partition: 3, shards: 3}', '{group: translation, partition: 3, shards: 2}'),
    text => text.replace('merge-multiple: false', 'merge-multiple: true'),
    text => text.replace('needs: [tests, report-assertions, package-checks, translation-test-feedback]', 'needs: [tests, report-assertions, package-checks]'),
    text => text.replace('if [ "$TRANSLATION_FEEDBACK_RESULT" != "success" ]; then', 'if false; then'),
  ]) {
    const changed = mutate(workflow);
    assert.notEqual(changed, workflow);
    assert.throws(() => assertInitialTranslationShards(changed), { name: 'AssertionError' });
  }
});
