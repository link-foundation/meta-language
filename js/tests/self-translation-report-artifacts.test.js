import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const FIXTURE = 'parity/self-translation/cases.lino';

test('the report measures each translated module against its hand-written Rust', () => {
  const directory = process.env.ISSUE_195_SELF_TRANSLATION_REPORT_DIRECTORY;
  assert.ok(directory, 'the report assertion consumes complete published reports');
  const expectedCommit = process.env.GITHUB_SHA ?? execFileSync('git', ['rev-parse', 'HEAD'], { cwd: root, encoding: 'utf8' }).trim();
  const collect = location => readdirSync(location, { withFileTypes: true }).flatMap(entry => {
    const file = path.join(location, entry.name);
    return entry.isDirectory() ? collect(file) : entry.name === 'self-translation-report.json' ? [file] : [];
  });
  const files = collect(directory);
  assert.ok(files.length > 0, 'the published reports contain measurements');
  execFileSync(process.execPath, [
    path.join(root, 'js/scripts/generate-self-translation-report.mjs'),
    '--verify-reports', directory, '--commit', process.env.ISSUE_195_COMMIT ?? expectedCommit,
  ], { encoding: 'utf8' });
  const reports = files.map(file => JSON.parse(readFileSync(file, 'utf8')));
  const { schemaVersion, commit, decorators } = reports[0];
  const selected = ['js/src/language-support.js', 'js/src/self-translation.js'];
  const modules = reports.flatMap(report => report.modules)
    .filter(row => selected.includes(row.module))
    .sort((left, right) => selected.indexOf(left.module) - selected.indexOf(right.module));
  const failures = reports.flatMap(report => report.failures);
  assert.equal(schemaVersion, 1);
  assert.equal(commit, expectedCommit);
  assert.deepEqual(failures, []);
  assert.deepEqual(decorators, ['borrowed-name', 'bare-sum', 'bare-comparison']);
  assert.deepEqual(modules.map(({ module, rust }) => [module, rust]), [
    ['js/src/language-support.js', 'rust/src/language_support.rs'],
    ['js/src/self-translation.js', 'rust/src/self_translation.rs'],
  ]);
  for (const row of modules) {
    assert.equal(row.coverage.sourceBytes, readFileSync(path.join(root, row.module)).length, row.module);
    assert.equal(row.coverage.itemBytes + row.coverage.layoutBytes, row.coverage.sourceBytes, row.module);
    assert.equal(row.coverage.unrepresentedBytes, 0, row.module);
    assert.deepEqual(row.decorated.coverage, row.coverage, row.module);
    for (const [file, field] of [[row.module, 'sourceSha256'], [row.rust, 'rustSha256']]) {
      assert.equal(row[field], createHash('sha256').update(readFileSync(path.join(root, file))).digest('hex'));
    }
    assert.ok(row.items.translated > 0 && row.handWrittenLines > 0, row.module);
    assert.ok(row.sharedLines <= row.codeLines && row.identical <= row.matched && row.matched <= row.functions, row.module);
    assert.ok(row.decorated.sharedLines <= row.decorated.codeLines && row.decorated.identical <= row.decorated.matched, row.module);
  }
  const report = files.map(file => readFileSync(path.join(path.dirname(file), 'self-translation-report.md'), 'utf8')).join('\n');
  for (const { module } of modules) assert.match(report, new RegExp(`\\| ${module} \\| `, 'u'));
  assert.match(report, /\| identical, decorated \|/u);
  recordIssue195Observations({
    requirementId: 'I195-SELF-TRANSLATION-SHARED-CORPUS', suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-self-translation-shared-corpus', fixtureFile: FIXTURE,
    assertions: ['differencePerModulePublished'],
    testName: 'the report measures each translated module against its hand-written Rust',
  });
});
