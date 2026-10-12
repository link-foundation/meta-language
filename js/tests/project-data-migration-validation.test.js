import assert from 'node:assert/strict';
import { execFileSync, spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, unlinkSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';
import { convertStructuredData } from '../scripts/convert-structured-data.mjs';
import { createFixtureSourceLayout } from '../scripts/fixture-source-bytes.mjs';
import { compareStructuredDataMigrationValues, validateStructuredDataMigration } from '../scripts/validate-structured-data-migration.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { readStructuredDataLino, structuredDataToLino } from '../src/structured-data-links.js';

const hash = value => createHash('sha256').update(value).digest('hex');
const paths = ['fixtures/requirements.json', 'fixtures/values.json'];
const sources = [
  '{\r\n "requirements": [{"id":"requirement α","scope":{"language":"Rust","version":"1","construct":"match"},"fixtures":["fixture:one","fixture:two"],"assertions":["ordered bindings","same output"],"aliases":["original alias","another alias"],"reference":"record:two"}],\r\n "records": [{"id":"record:one","references":["record:two"]},{"id":"record:two","references":[]}],\r\n "source": "\\u03bb café 😀\\u0301", "number": 1e2\r\n}\r\n',
  '{"values":[-0,0,1.25,5e-324,1.7976931348623157e308,"\\ud800",null,false],"first":{"id":"equal"},"second":{"id":"equal"},"__proto__":{"owned":true}}\n',
];

function synthetic(t, { separateOriginals = false } = {}) {
  const directory = mkdtempSync(join(tmpdir(), 'meta-migration-validation-'));
  t.after(() => rmSync(directory, { recursive: true, force: true }));
  const git = arguments_ => execFileSync('git', arguments_, { cwd: directory, encoding: 'utf8', timeout: 10_000, stdio: ['ignore', 'pipe', 'pipe'] });
  git(['init', '-q']);
  const commit = message => {
    git(['add', '--all']);
    git(['-c', 'user.name=Migration fixture', '-c', 'user.email=migration-fixture@example.invalid',
      'commit', '--quiet', '--no-gpg-sign', '--no-verify', '-m', message]);
    return git(['rev-parse', 'HEAD']).trim();
  };
  const sourceRevisions = [];
  for (const [index, path] of paths.entries()) {
    mkdirSync(dirname(join(directory, path)), { recursive: true });
    writeFileSync(join(directory, path), sources[index]);
    if (separateOriginals && index === 0) sourceRevisions[index] = commit('Independent first original JSON fixture');
  }
  const originalGitRevision = commit('Original independent JSON fixtures');
  for (const index of sources.keys()) sourceRevisions[index] ??= originalGitRevision;
  const baselines = paths.map((logicalPath, index) => ({
    logicalPath, decodedSha256: hash(JSON.stringify(JSON.parse(sources[index]))),
    ...(index === 0 ? { sourceSha256: hash(sources[index]) } : {}),
  }));
  const layouts = { [paths[0]]: createFixtureSourceLayout(sources[0]) };
  for (const [index, path] of paths.entries()) {
    writeFileSync(join(directory, path.replace(/\.json$/u, '.lino')), convertStructuredData(sources[index]));
    unlinkSync(join(directory, path));
  }
  const saveBaselines = () => writeFileSync(join(directory, 'migration-baselines.lino'), structuredDataToLino(baselines));
  const saveLayouts = () => writeFileSync(join(directory, 'source-layouts.lino'), structuredDataToLino(layouts));
  saveBaselines(); saveLayouts();
  const migratedGitRevision = commit('Automatic Links storage migration');
  const replace = (index, value) => writeFileSync(join(directory, paths[index].replace(/\.json$/u, '.lino')), structuredDataToLino(value));
  const options = { repository: directory, baselineFile: 'migration-baselines.lino', layoutsFile: 'source-layouts.lino', originalGitRevision };
  return { directory, options, baselines, layouts, replace, saveBaselines, saveLayouts, migratedGitRevision, sourceRevisions };
}

test('independent original Git replay preserves scope, references, ordering, exact values and raw source bytes', t => {
  const fixture = synthetic(t);
  const report = validateStructuredDataMigration(fixture.options);
  assert.equal(report.verifiedCount, 2);
  assert.equal(report.baselineCount, 2);
  assert.deepEqual(report.failures, []);
  assert.deepEqual(report.results.map(row => row.logicalPath), paths);
  for (const [index, result] of report.results.entries()) {
    assert.equal(result.originalGitRevision, fixture.options.originalGitRevision);
    assert.equal(result.decodedSha256, fixture.baselines[index].decodedSha256);
    assert.equal(result.originalSourceSha256, hash(sources[index]));
    assert.equal(result.migratedLinoSha256, hash(readFileSync(join(fixture.directory, paths[index].replace(/\.json$/u, '.lino')))));
    assert.ok(result.collectionCount > 0);
    assert.ok(result.scalarCount > 0);
  }
  assert.equal(report.results[0].sourceSha256, fixture.baselines[0].sourceSha256);
  const values = readProjectData(join(fixture.directory, paths[1]));
  assert.ok(Object.is(values.values[0], -0));
  assert.equal(values.values[3], Number.MIN_VALUE);
  assert.equal(values.values[4], Number.MAX_VALUE);
  assert.equal(values.first === values.second, false, 'equal JSON records remain distinct collection occurrences');
});

test('migration refuses missing, moving, abbreviated or unavailable original Git provenance', t => {
  const fixture = synthetic(t);
  for (const originalGitRevision of [undefined, 'HEAD', fixture.options.originalGitRevision.slice(0, 8)]) {
    assert.throws(() => validateStructuredDataMigration({ ...fixture.options, originalGitRevision }), /missing immutable original Git provenance/u);
  }
  assert.throws(() => validateStructuredDataMigration({ ...fixture.options, originalGitRevision: '0'.repeat(40) }), /original Git revision is unavailable/u);
  assert.throws(() => validateStructuredDataMigration({ ...fixture.options, originalGitRevision: fixture.migratedGitRevision }), error => {
    assert.equal(error.report.verifiedCount, 0);
    assert.equal(error.report.failures.length, 2);
    assert.ok(error.report.failures.every(failure => /original JSON blob is unavailable/u.test(failure.reason)));
    return true;
  });
});

test('a recaptured current-data hash cannot authenticate itself as the original baseline', t => {
  const fixture = synthetic(t);
  const changed = JSON.parse(sources[0]);
  changed.requirements[0].scope.construct = 'a reduced construct';
  fixture.replace(0, changed);
  fixture.baselines[0].decodedSha256 = hash(JSON.stringify(changed));
  fixture.saveBaselines();
  assert.throws(() => validateStructuredDataMigration(fixture.options), error => {
    assert.equal(error.report.verifiedCount, 1, 'independent unaffected originals are still checked');
    assert.equal(error.report.failures.length, 1);
    assert.match(error.report.failures[0].reason, /baseline hash differs from the original Git value/u);
    return true;
  });
});

test('fixture identities, obligations, aliases and cross-record references cannot be reduced by migration', t => {
  const fixture = synthetic(t);
  const mutations = [
    value => { value.requirements[0].id = 'replacement requirement'; },
    value => { delete value.requirements[0].scope.version; },
    value => { value.requirements[0].fixtures.pop(); },
    value => { value.requirements[0].assertions.pop(); },
    value => { value.requirements[0].aliases.reverse(); },
    value => { value.records[0].references = []; },
    value => { value.records.reverse(); },
  ];
  for (const mutate of mutations) {
    const changed = JSON.parse(sources[0]); mutate(changed); fixture.replace(0, changed);
    assert.throws(() => validateStructuredDataMigration(fixture.options), error => {
      assert.equal(error.report.verifiedCount, 1);
      assert.match(error.report.failures[0].reason, /migrated values or semantic identity fields differ/u);
      return true;
    });
  }
});

test('collection field order is checked independently of deep value equality', t => {
  const fixture = synthetic(t);
  const original = JSON.parse(sources[0]);
  const reordered = { source: original.source, requirements: original.requirements, records: original.records, number: original.number };
  assert.deepEqual(reordered, original);
  fixture.replace(0, reordered);
  assert.throws(() => validateStructuredDataMigration(fixture.options), error => {
    assert.match(error.report.failures[0].reason, /collection order differs from the original/u);
    return true;
  });
});

test('source byte provenance must authenticate the original and reproduce its exact formatting', t => {
  const fixture = synthetic(t);
  fixture.baselines[0].sourceSha256 = hash(JSON.stringify(JSON.parse(sources[0])));
  fixture.saveBaselines();
  assert.throws(() => validateStructuredDataMigration(fixture.options), error => {
    assert.match(error.report.failures[0].reason, /source byte hash differs from the original Git blob/u);
    return true;
  });
  fixture.baselines[0].sourceSha256 = hash(sources[0]); fixture.saveBaselines();
  fixture.layouts[paths[0]].whitespace[0][1] = '\n ';
  fixture.saveLayouts();
  assert.throws(() => validateStructuredDataMigration(fixture.options), error => {
    assert.match(error.report.failures[0].reason, /does not reconstruct the exact original Git bytes/u);
    return true;
  });
});

test('each original may use its independently recorded full revision and duplicate identities are refused', t => {
  const fixture = synthetic(t, { separateOriginals: true });
  assert.notEqual(fixture.sourceRevisions[0], fixture.sourceRevisions[1]);
  for (const [index, row] of fixture.baselines.entries()) row.sourceRevision = fixture.sourceRevisions[index];
  fixture.saveBaselines();
  assert.equal(validateStructuredDataMigration({ ...fixture.options, originalGitRevision: undefined }).verifiedCount, 2);
  fixture.baselines[1].logicalPath = fixture.baselines[0].logicalPath;
  fixture.saveBaselines();
  assert.throws(() => validateStructuredDataMigration(fixture.options), /duplicate migration baseline identity/u);
});

test('CI command emits actual verified Links results and bulk function replay refuses a local environment', t => {
  const fixture = synthetic(t);
  const script = new URL('../scripts/validate-structured-data-migration.mjs', import.meta.url);
  const replay = spawnSync(process.execPath, [fileURLToPath(script), '--repository', fixture.directory,
    '--baseline', 'migration-baselines.lino', '--layouts', 'source-layouts.lino',
    '--original-revision', fixture.options.originalGitRevision], {
    encoding: 'utf8', timeout: 30_000, env: { ...process.env, CI: 'true' },
  });
  assert.equal(replay.status, 0, replay.stderr);
  assert.equal(readStructuredDataLino(replay.stdout).verifiedCount, 2);
  const localCommand = spawnSync(process.execPath, [fileURLToPath(script)], {
    encoding: 'utf8', timeout: 10_000, env: { ...process.env, CI: '' },
  });
  assert.notEqual(localCommand.status, 0);
  assert.match(localCommand.stderr, /command runs only in CI/u);
  while (fixture.baselines.length < 9) fixture.baselines.push({ ...fixture.baselines[0], logicalPath: `fixtures/not-read-${fixture.baselines.length}.json` });
  fixture.saveBaselines();
  const local = spawnSync(process.execPath, ['--input-type=module', '-e',
    `const {validateStructuredDataMigration}=await import(${JSON.stringify(script.href)});try{validateStructuredDataMigration(${JSON.stringify(fixture.options)});}catch(error){if(error.message==='bulk structured data migration replay runs only in CI'){process.stdout.write(error.message);process.exit(0);}throw error;}throw Error('local bulk replay accepted');`], {
    encoding: 'utf8', timeout: 10_000, env: { ...process.env, CI: '' },
  });
  assert.equal(local.status, 0, local.stderr);
  assert.match(local.stdout, /bulk structured data migration replay runs only in CI/u);
});

test('graph comparison preserves sharing and distinct identities beyond the JSON tree model', () => {
  const shared = { id: 'record', references: ['another record'] };
  const original = { first: shared, second: shared, distinct: { id: 'record', references: ['another record'] } };
  const reloaded = readStructuredDataLino(structuredDataToLino(original));
  assert.equal(compareStructuredDataMigrationValues(original, reloaded).collectionCount, 5);
  assert.equal(reloaded.first, reloaded.second);
  assert.notEqual(reloaded.first, reloaded.distinct);
  const split = readStructuredDataLino(structuredDataToLino(structuredClone({ first: { ...shared }, second: { ...shared }, distinct: original.distinct })));
  assert.throws(() => compareStructuredDataMigrationValues(original, split), /shared collection identity was split/u);
  const merged = readStructuredDataLino(structuredDataToLino({ first: shared, second: shared, distinct: shared }));
  assert.throws(() => compareStructuredDataMigrationValues(original, merged), /distinct collection identities were merged/u);
});
