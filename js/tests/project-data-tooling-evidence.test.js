import assert from 'node:assert/strict';
import { execFileSync, spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { gzipSync } from 'node:zlib';
import test from 'node:test';
import { CRITICAL_CONVERSATION_REQUIREMENTS } from '../scripts/issue-195-conversation-requirements.mjs';
import { readProjectData, writeProjectData } from '../scripts/project-data.mjs';
import { projectDataToolingContract, projectDataToolingFixture } from '../scripts/generate-project-data-tooling-fixture.mjs';
import { auditExternalGrammarStorage, verifyOrdinaryDownloadStaging, verifyTemporaryGrammarLifeCycle,
  verifyPreparedNativeSnapshots } from '../scripts/project-data-tooling-evidence.mjs';
import { validateProjectDataStorage } from '../scripts/project-data-storage.mjs';
import { validateStructuredDataMigration } from '../scripts/validate-structured-data-migration.mjs';
import { readStructuredDataLino, structuredDataToLino } from '../src/structured-data-links.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const root = fileURLToPath(new URL('../../', import.meta.url));
const git = (directory, args) => execFileSync('git', args, { cwd: directory, encoding: 'utf8' });
const put = (directory, file, text) => {
  mkdirSync(dirname(join(directory, file)), { recursive: true });
  writeFileSync(join(directory, file), text);
};
const complete = (id, testName) => {
  const row = CRITICAL_CONVERSATION_REQUIREMENTS.find(row => row.id === id);
  assert.ok(row);
  for (const fixture of [{ id: row.fixtureId, path: row.fixture }, ...(row.additionalFixtures ?? [])]) {
    recordIssue195Observations({ requirementId: id, suffix: 'behavior', runtime: 'tooling',
      fixtureId: fixture.id, fixtureFile: fixture.path, assertions: row.assertions, testName });
  }
};
const settings = contract => ({
  manifest: readProjectData(join(root, contract.assetManifest)),
  inventory: readProjectData(join(root, contract.nativeInventory)),
  provenanceFiles: contract.trackedProvenance, cacheDirectory: contract.assetCacheDirectory,
});

test('tiny Git inventories exercise ignored downloads and ordinary staging without source deletion', () => {
  const base = mkdtempSync(join(tmpdir(), 'meta-tooling-paths-'));
  const repository = join(base, 'source');
  try {
    mkdirSync(repository);
    git(repository, ['init', '--quiet']);
    put(repository, '.gitignore', '/vendor/\n/.external-grammars/\n');
    put(repository, 'native/grammar.lino', '(grammar: native)\n');
    put(repository, 'vendor/receipt.lino', '(receipt: pinned)\n');
    git(repository, ['add', '--force', '--', '.gitignore', 'native/grammar.lino', 'vendor/receipt.lino']);
    const options = { manifest: { schemaVersion: 1, assets: { 'vendor/source.c': {} } },
      inventory: { nativeGrammars: { native: { grammar: 'native/grammar.lino' } } },
      provenanceFiles: ['vendor/receipt.lino'], cacheDirectory: '.external-grammars' };
    const audit = auditExternalGrammarStorage(repository, options);
    assert.deepEqual(verifyOrdinaryDownloadStaging(repository, join(base, 'staging'), audit),
      { excludedPaths: 2, retainedFiles: 2 });
    assert.equal(readFileSync(join(repository, 'vendor/receipt.lino'), 'utf8'), '(receipt: pinned)\n');
    put(repository, 'vendor/source.c', 'foreign source');
    git(repository, ['add', '--force', '--', 'vendor/source.c']);
    assert.throws(() => auditExternalGrammarStorage(repository, options), /upstream input is untracked/u);
    assert.throws(() => verifyOrdinaryDownloadStaging(repository, join(base, 'unsafe'),
      { ignoredPaths: ['../escape'], retainedFiles: ['native/grammar.lino'] }), /safe repository path/u);
  } finally { rmSync(base, { recursive: true, force: true }); }
});

test('tiny linked records exercise shared identities, exact scalars and inspectable reconstruction', () => {
  const shared = { id: 'original', aliases: ['α', 'alias'], reference: 'original' };
  const source = { first: shared, second: shared, values: [-0, Number.MIN_VALUE, Number.MAX_VALUE, '\ud800', 'λ\u0000'] };
  const lino = structuredDataToLino(source);
  const result = readStructuredDataLino(lino);
  assert.deepEqual(result, source);
  assert.equal(result.first, result.second);
  assert.ok(Object.is(result.values[0], -0));
  assert.deepEqual(Object.keys(result), Object.keys(source));
  assert.match(lino, /key%20value%20pair/u);
  assert.doesNotMatch(lino, /JSON/u);
});

test('tiny authenticated lifecycle reuses checked plain and gzip pins and cleans both temporary copies', async () => {
  const base = mkdtempSync(join(tmpdir(), 'meta-tooling-lifecycle-'));
  const repository = join(base, 'source');
  const plain = Buffer.from('grammar λ\n');
  const compressed = gzipSync(plain);
  const hash = value => createHash('sha256').update(value).digest('hex');
  try {
    mkdirSync(repository);
    git(repository, ['init', '--quiet']);
    put(repository, '.gitignore', '/vendor/\n/.external-grammars/\n');
    const assets = {
      'vendor/source.txt': { bytes: plain.length, sha256: hash(plain), encoding: 'identity' },
      'vendor/source.gz': { bytes: compressed.length, sha256: hash(compressed), encoding: 'gzip',
        decodedBytes: plain.length, decodedSha256: hash(plain) },
    };
    put(repository, '.external-grammars/vendor/source.txt', plain);
    put(repository, '.external-grammars/vendor/source.gz', compressed);
    const options = { manifest: { schemaVersion: 1, assets }, cacheDirectory: '.external-grammars' };
    assert.deepEqual(await verifyTemporaryGrammarLifeCycle(repository, join(base, 'lifecycle'), options),
      { verifiedAssets: 2, materializedAssets: 2, cleanedAssets: 2 });
    assert.deepEqual(readFileSync(join(repository, '.external-grammars/vendor/source.txt')), plain);
    assets['vendor/source.gz'].decodedSha256 = '0'.repeat(64);
    await assert.rejects(verifyTemporaryGrammarLifeCycle(repository, join(base, 'counterfeit'), options), /decoded external grammar asset checksum mismatch/u);
  } finally { rmSync(base, { recursive: true, force: true }); }
});

test('tiny prepared snapshot executes public parsing while upstream files and network are denied', () => {
  const report = verifyPreparedNativeSnapshots(root, fileURLToPath(projectDataToolingFixture), { grammarIds: ['native-json'] });
  assert.equal(report.scope, 'explicit selected grammars');
  assert.deepEqual(report.snapshots.map(entry => entry.id), ['native-json']);
  assert.equal(report.externalAssetsDenied, true);
  assert.equal(report.networkDenied, true);
});

test('source JSON exceptions have exact external boundaries and cannot exempt project-owned data', async context => {
  const contract = readProjectData(projectDataToolingFixture);
  assert.deepEqual(contract, projectDataToolingContract);
  const registry = readProjectData(join(root, contract.exceptionRegistry));
  const audit = await validateProjectDataStorage(root, registry);
  assert.deepEqual(audit.problems, []);
  assert.deepEqual(audit.files, registry.exceptions.map(entry => entry.path).sort());
  const unregistered = await validateProjectDataStorage(root, { ...registry, exceptions: [] });
  assert.equal(unregistered.problems.filter(problem => problem.startsWith('unregistered')).length, audit.files.length);
  const convenience = structuredClone(registry);
  convenience.exceptions[0].boundary = 'existing JSON is convenient';
  assert.ok((await validateProjectDataStorage(root, convenience)).problems.some(problem => problem.includes('unknown boundary')));
  complete('I195-ASSOCIATIVE-DATA-JSON-EXCEPTIONS', context.name);
});

test('full tooling inventory validates original migration, Git isolation and asset-free native execution in CI', {
  skip: process.env.META_LANGUAGE_TOOLING_FULL_SCOPE !== '1' ? 'full167 migration and native inventory execution run only in CI' : false,
}, async context => {
  assert.equal(process.env.CI, 'true', 'the complete scope is CI work');
  const contract = readProjectData(projectDataToolingFixture);
  assert.deepEqual(contract, projectDataToolingContract);
  const options = settings(contract);
  const audit = auditExternalGrammarStorage(root, options);
  const base = mkdtempSync(join(tmpdir(), 'meta-tooling-full-'));
  try {
    const staging = verifyOrdinaryDownloadStaging(root, join(base, 'staging'), audit);
    complete('I195-EXTERNAL-GRAMMAR-IGNORED-DIRECTORY', context.name);
    const lifecycle = await verifyTemporaryGrammarLifeCycle(root, join(base, 'lifecycle'), options);
    assert.equal(lifecycle.verifiedAssets, audit.assetPaths.length);
    assert.equal(lifecycle.cleanedAssets, audit.assetPaths.length);
    const snapshots = verifyPreparedNativeSnapshots(root, fileURLToPath(projectDataToolingFixture));
    assert.equal(snapshots.scope, 'complete registered native inventory');
    assert.equal(snapshots.snapshots.length, Object.keys(options.inventory.nativeGrammars).length);
    const migration = validateStructuredDataMigration({ repository: root,
      baselineFile: join(root, contract.migrationBaselineFile), originalGitRevision: contract.originalGitRevision,
      migrationRevisionFile: join(root, contract.migrationRevisionFile), currentFreshnessChecks: contract.currentFreshnessChecks });
    assert.equal(migration.baselineCount, contract.migrationBaselineCount);
    assert.equal(migration.verifiedCount, contract.migrationBaselineCount);
    const agda = readProjectData(join(root, contract.agdaCorpus));
    assert.equal(agda.cases.length, 75);
    assert.ok(agda.cases.every(example => typeof example.source === 'string'));
    const storage = await validateProjectDataStorage(root, readProjectData(join(root, contract.exceptionRegistry)));
    assert.deepEqual(storage.problems, []);
    // These checks establish storage through the public associative algorithms.
    // Conversion's BOTH-package behavior and inherited offline delivery still
    // require their native/clean-consumer witnesses; no whole cell is recorded
    // for those three broader requirements from this JavaScript-only probe.
    complete('I195-ASSOCIATIVE-DATA-STORAGE', context.name);
    const scopeComparisons = [];
    for (const baseline of contract.protectedScopeBaselines) {
      const scope = spawnSync(process.execPath, [join(root, 'js/scripts/check-issue-195-acceptance.mjs'),
        '--scope-baseline', baseline], { cwd: root, encoding: 'utf8', maxBuffer: 8 * 1024 * 1024,
        env: { ...process.env, ISSUE_195_SCOPE_CHANGE_APPROVED: 'false' } });
      assert.equal(scope.status, 0, scope.stderr || scope.stdout);
      scopeComparisons.push({ baseline, output: scope.stdout });
    }
    writeProjectData(join(root, 'issue-195-results/artifacts/project-data-tooling-evidence.lino'), {
      schemaVersion: 1, commit: process.env.ISSUE_195_COMMIT ?? git(root, ['rev-parse', 'HEAD']).trim(),
      staging, lifecycle, snapshots, migration, scopeComparisons,
      jsonBoundaryFiles: storage.files, agdaCorpusCases: agda.cases.length,
    }, { format: 'lino' });
    context.diagnostic(`verified ${migration.verifiedCount} immutable Git originals, ${lifecycle.verifiedAssets} authenticated assets, `
      + `${snapshots.snapshots.length} asset-free native snapshots, ${storage.files.length} JSON boundaries and ${scopeComparisons.length} protected scope baselines`);
  } finally { rmSync(base, { recursive: true, force: true }); }
});
