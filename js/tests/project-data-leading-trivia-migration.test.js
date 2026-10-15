import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, mkdtempSync, rmSync, unlinkSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { validateStructuredDataMigration } from '../scripts/validate-structured-data-migration.mjs';
import { structuredDataToLino } from '../src/structured-data-links.js';

const hash = value => createHash('sha256').update(value).digest('hex');

function synthetic(t, { alterHistorical = value => value, alterInventory = value => value } = {}) {
  const repository = mkdtempSync(join(tmpdir(), 'meta-leading-trivia-migration-'));
  t.after(() => rmSync(repository, { recursive: true, force: true }));
  const git = args => execFileSync('git', args, { cwd: repository, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] });
  const write = (path, value) => {
    mkdirSync(dirname(join(repository, path)), { recursive: true });
    writeFileSync(join(repository, path), value);
  };
  const commit = message => {
    git(['add', '--all']);
    git(['-c', 'user.name=Migration fixture', '-c', 'user.email=migration-fixture@example.invalid',
      'commit', '--quiet', '--no-gpg-sign', '--no-verify', '-m', message]);
    return git(['rev-parse', 'HEAD']).trim();
  };
  git(['init', '-q']);
  const logicalPath = 'parity/fixtures/native-grammars/demonstration.json';
  const original = { id: 'demonstration', grammar: 'parity/grammars/native/demonstration.lino',
    hidden: [], anonymous: ['separator'], extras: ['comment'], aliases: ['original alias'],
    matches: [{ source: '\noriginal source', rows: [[0, '', 'program', 1, 0, 16, '']] }],
    divergences: [], rejections: [{ source: 'invalid', reason: 'original refusal' }] };
  write(logicalPath, JSON.stringify(original));
  const originalGitRevision = commit('Original immutable fixture and expectations');
  const historical = alterHistorical(Object.fromEntries(Object.entries(original).flatMap(([name, value]) => name === 'matches'
    ? [['rootIncludesLeadingTrivia', true], [name, value]] : [[name, value]])));
  write(logicalPath.replace(/\.json$/u, '.lino'), structuredDataToLino(historical));
  unlinkSync(join(repository, logicalPath));
  const baseline = [{ logicalPath, decodedSha256: hash(JSON.stringify(historical)) }];
  write('baselines.lino', structuredDataToLino(baseline));
  const migrationRevision = commit('Historical conversion with additive root metadata');
  const inventoryFile = 'parity/language-grammar-inventory.lino';
  const inventory = structuredDataToLino({ nativeGrammars: { 'native-demonstration': alterInventory({
    grammar: original.grammar, rootIncludesLeadingTrivia: true, hidden: [], anonymous: ['separator'], extras: ['comment'],
  }) } });
  write(inventoryFile, inventory);
  const inventoryRevision = commit('Later independently committed native projection metadata');
  const correction = { logicalPath, kind: 'native grammar leading trivia metadata addition', nativeGrammar: 'native-demonstration',
    originalDecodedSha256: hash(JSON.stringify(original)), addedField: 'rootIncludesLeadingTrivia', addedValue: true,
    beforeField: 'matches', inventoryFile, inventoryRevision, inventorySha256: hash(inventory),
    temporalRelation: 'later committed native grammar inventory' };
  const entry = { logicalPath, originalGitRevision, migrationRevision, corrections: [correction] };
  const save = () => write('revisions.lino', structuredDataToLino([entry]));
  save();
  return { repository, original, historical, correction, entry, save,
    options: { repository, baselineFile: 'baselines.lino', migrationRevisionFile: 'revisions.lino' } };
}

test('leading trivia correction authenticates one ordered metadata addition and all original oracle fields', t => {
  const fixture = synthetic(t);
  const report = validateStructuredDataMigration(fixture.options);
  assert.equal(report.verifiedCount, 1);
  assert.equal(report.results[0].originalDecodedSha256, fixture.correction.originalDecodedSha256);
  assert.deepEqual(report.results[0].metadataCorrections, [{ nativeGrammar: 'native-demonstration',
    addedField: 'rootIncludesLeadingTrivia', addedValue: true, beforeField: 'matches',
    inventoryRevision: fixture.correction.inventoryRevision, inventorySha256: fixture.correction.inventorySha256,
    temporalRelation: 'later committed native grammar inventory' }]);
  const preserved = Object.fromEntries(Object.entries(fixture.historical).filter(([name]) => name !== 'rootIncludesLeadingTrivia'));
  assert.deepEqual(preserved, fixture.original);
  assert.deepEqual(Object.keys(preserved), Object.keys(fixture.original));
});

test('leading trivia correction refuses missing, counterfeit, unrelated and repeated provenance', t => {
  const fixture = synthetic(t);
  const mutations = [
    [],
    [{ ...fixture.correction, originalDecodedSha256: '0'.repeat(64) }],
    [{ ...fixture.correction, inventorySha256: '0'.repeat(64) }],
    [{ ...fixture.correction, inventoryRevision: fixture.entry.migrationRevision }],
    [{ ...fixture.correction, inventoryRevision: fixture.entry.originalGitRevision }],
    [{ ...fixture.correction, nativeGrammar: 'native-another' }],
    [{ ...fixture.correction, addedField: 'matches' }],
    [{ ...fixture.correction, addedValue: false }],
    [{ ...fixture.correction, beforeField: 'rejections' }],
    [fixture.correction, fixture.correction],
  ];
  for (const corrections of mutations) {
    fixture.entry.corrections = corrections; fixture.save();
    assert.throws(() => validateStructuredDataMigration(fixture.options), error => {
      assert.equal(error.report.verifiedCount, 0);
      assert.equal(error.report.failures.length, 1);
      return true;
    });
  }
});

test('leading trivia metadata cannot authorize changed sources, golden rows, aliases or assertion order', t => {
  for (const alterHistorical of [
    value => ({ ...value, aliases: ['replacement alias'] }),
    value => ({ ...value, matches: [{ ...value.matches[0], source: 'replacement source' }] }),
    value => ({ ...value, matches: [{ ...value.matches[0], rows: [] }] }),
    value => Object.fromEntries(Object.entries(value).reverse()),
  ]) {
    const fixture = synthetic(t, { alterHistorical });
    assert.throws(() => validateStructuredDataMigration(fixture.options), error => {
      assert.match(error.report.failures[0].reason, /baseline hash differs from the original Git value/u);
      return true;
    });
  }
});

test('a pinned inventory must independently prove the same grammar, projection metadata and true flag', t => {
  for (const alterInventory of [
    value => ({ ...value, rootIncludesLeadingTrivia: false }),
    value => ({ ...value, grammar: 'parity/grammars/native/another.lino' }),
    value => ({ ...value, hidden: ['new hidden kind'] }),
    value => ({ ...value, anonymous: [] }),
    value => ({ ...value, extras: ['different extra'] }),
  ]) {
    const fixture = synthetic(t, { alterInventory });
    assert.throws(() => validateStructuredDataMigration(fixture.options), error => {
      assert.match(error.report.failures[0].reason, /does not authenticate the original native grammar projection/u);
      return true;
    });
  }
});

test('actual Pascal migration replays its immutable original with frozen hashes and unchanged golden rows', t => {
  const repository = fileURLToPath(new URL('../../', import.meta.url));
  const logicalPath = 'parity/fixtures/native-grammars/pascal.json';
  const baselines = readProjectData(join(repository, 'parity/fixtures/structured-data-migration-baselines.lino'));
  const revisions = readProjectData(join(repository, 'parity/fixtures/structured-data-migration-revisions.lino'));
  assert.equal(baselines.length, 167);
  assert.deepEqual(revisions.map(entry => entry.logicalPath), baselines.map(entry => entry.logicalPath));
  const baseline = baselines.find(entry => entry.logicalPath === logicalPath);
  const revision = revisions.find(entry => entry.logicalPath === logicalPath);
  assert.equal(baseline.decodedSha256, 'efadbdd3ccd82a83a909d98cbc5fa9fe236387f6892c23a8eb482ce0fc75605e');
  const directory = mkdtempSync(join(tmpdir(), 'meta-pascal-migration-replay-'));
  t.after(() => rmSync(directory, { recursive: true, force: true }));
  const baselineFile = join(directory, 'baseline.lino');
  const migrationRevisionFile = join(directory, 'revisions.lino');
  writeFileSync(baselineFile, structuredDataToLino([baseline]));
  writeFileSync(migrationRevisionFile, structuredDataToLino([revision]));
  const report = validateStructuredDataMigration({ repository, baselineFile, migrationRevisionFile });
  assert.equal(report.baselineCount, 1);
  assert.equal(report.verifiedCount, 1);
  assert.deepEqual(report.failures, []);
  assert.equal(report.results[0].originalDecodedSha256, '43ca12d91a51980ccd02f7db784df3968b719bc0160bf8cb1681d7ced6164156');
  assert.equal(report.results[0].decodedSha256, baseline.decodedSha256);
  assert.equal(report.results[0].historicalLinoSha256, report.results[0].migratedLinoSha256);
});
