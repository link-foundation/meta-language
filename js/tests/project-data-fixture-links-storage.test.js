import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import test from 'node:test';
import { readProjectData, resolveProjectDataPath, serializeProjectData } from '../scripts/project-data.mjs';
import { verifyGitArtifactCertificate } from '../scripts/git-object-certificate.mjs';
import { createFixtureSourceLayout, readFixtureSourceBytes, renderFixtureSourceBytes } from '../scripts/fixture-source-bytes.mjs';
import { readStructuredDataLino } from '../src/structured-data-links.js';
import { normalizeMigrationProviderReceipts, normalizeMigrationTranslationStages, normalizeMigrationAntlrLexerDeclarations } from '../scripts/validate-structured-data-migration.mjs';
import { captureTranslationStageRefreshHistory, retainTranslationStageRefreshScopes, validateTranslationStageRefreshHistory } from '../scripts/generate-translation-stage-refresh-receipt.mjs';
import { structuredDataToLino } from '../src/structured-data-links.js';

const root = new URL('../../', import.meta.url);
const hash = value => createHash('sha256').update(value).digest('hex');

test('fixture Links data preserves every original ordered decoded baseline', () => {
  const baselines = readProjectData(new URL('../../parity/fixtures/structured-data-migration-baselines.lino', import.meta.url));
  assert.ok(baselines.length >= 76);
  assert.equal(new Set(baselines.map(entry => entry.logicalPath)).size, baselines.length);
  const revisions = readProjectData(new URL('../../parity/fixtures/structured-data-migration-revisions.lino', import.meta.url));
  const contract = readProjectData(new URL('../../parity/fixtures/project-data-tooling-contract.lino', import.meta.url));
  for (const { logicalPath, decodedSha256, sourceSha256 } of baselines) {
    const file = new URL(logicalPath, root);
    assert.equal(existsSync(file), false, `${logicalPath} has one authoritative Links representation`);
    assert.ok(resolveProjectDataPath(file).endsWith('.lino'), logicalPath);
    let protectedValue = readProjectData(file);
    if (logicalPath === 'parity/fixtures/typescript-erasure.json') {
      const history = readProjectData(new URL('../../parity/fixtures/typescript-named-import-expectation-history.lino', import.meta.url));
      assert.equal(history.path, logicalPath.replace(/\.json$/u, '.lino'));
      assert.equal(hash(readFileSync(resolveProjectDataPath(file))), history.currentSha256,
        'the complete current fixture must match its independently checked extension');
      assert.equal(history.originalCases.length, 13);
      assert.equal(history.appended.length, 8);
      assert.deepEqual(protectedValue.cases.map(row => row.id),
        [...history.originalCases.map(row => row.id), ...history.appended]);
      for (const original of history.originalCases) {
        assert.equal(protectedValue.cases.find(row => row.id === original.id)?.source, original.source);
      }
      protectedValue = { ...protectedValue, cases: protectedValue.cases.slice(0, 13)
        .map(row => row.id === history.previousCase.id ? history.previousCase : row) };
      verifyGitArtifactCertificate(history.certificate, Buffer.from(serializeProjectData(protectedValue, history.path)), {
        revision: '15745ffcd322c48bdd537ccb48a5f7c9deb96917', path: 'parity/fixtures/typescript-erasure.lino',
      });
    }
    const freshness = contract.currentFreshnessChecks.find(check => check.logicalPaths.includes(logicalPath));
    const refresh = freshness?.providerReceiptRefresh;
    if (refresh || freshness?.translationStageRefresh || freshness?.antlrLexerDeclarationRefresh) {
      const revision = revisions.find(entry => entry.logicalPath === logicalPath)?.migrationRevision;
      assert.match(revision, /^[0-9a-f]{40}$/u);
      const historical = readStructuredDataLino(execFileSync('git', ['show', `${revision}:${logicalPath.replace(/\.json$/u, '.lino')}`],
        { cwd: fileURLToPath(root), encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 }));
      assert.equal(hash(JSON.stringify(historical)), decodedSha256, `${logicalPath}: frozen original conversion`);
      for (const { language, provider } of refresh ?? []) {
        const catalog = readProjectData(new URL('../../js/src/data/language-catalog.lino', import.meta.url));
        const grammar = catalog.languages.find(entry => entry.name === language).grammars.find(entry => entry.id === provider);
        assert.equal(protectedValue.languages[language].grammars[provider].parserSha256, grammar.parserSha256);
        for (const runtime of ['js', 'rust']) assert.equal(hash(readFileSync(new URL(`../../${runtime}/src/data/${catalog.nativeGrammars[provider].file}`, import.meta.url))), grammar.parserSha256);
      }
      if (refresh) protectedValue = normalizeMigrationProviderReceipts(historical, protectedValue, refresh);
      if (freshness.translationStageRefresh) {
        assert.deepEqual(freshness.command, ['node', 'js/scripts/build-translation-stage-fixtures.mjs', '--check'],
          'the complete current pipeline remains a required CI freshness check');
        protectedValue = normalizeMigrationTranslationStages(historical, protectedValue, freshness.translationStageRefresh);
      }
      if (freshness.antlrLexerDeclarationRefresh) {
        assert.deepEqual(freshness.command, ['node', 'js/scripts/generate-antlr-reverse-fixture.mjs', '--check'],
          'the bounded complete ANTLR reverse fixture remains a required freshness check');
        protectedValue = normalizeMigrationAntlrLexerDeclarations(historical, protectedValue);
      }
    }
    assert.equal(hash(JSON.stringify(protectedValue)), decodedSha256, `${logicalPath}: values and ordered occurrences`);
    if (sourceSha256) assert.equal(hash(readFixtureSourceBytes(file)), sourceSha256, `${logicalPath}: unchanged provenance bytes`);
  }
});

test('fixture provenance formatting retains Unicode spellings and whitespace as separate records', () => {
  const source = '{\n  "source": "\\ufeffhéllo\\u0301", "number": 1e2, "array": [ true, null ]\n}\n';
  const value = JSON.parse(source);
  const layout = createFixtureSourceLayout(source);
  assert.ok(layout.whitespace.length > 0);
  assert.ok(layout.spellings.length > 0);
  assert.equal(renderFixtureSourceBytes(value, layout).toString('utf8'), source);
  assert.throws(() => renderFixtureSourceBytes({ ...value, array: [] }, layout), /token count/);
  assert.throws(() => renderFixtureSourceBytes(value, { ...layout, whitespace: [[0, 'source code']] }), /whitespace/);
  assert.throws(() => renderFixtureSourceBytes(value, { ...layout, spellings: [[3, '"a different source"']] }), /spelling/);
});

test('the stage refresh receipt authenticates the current immutable snapshot and every selected change', () => {
  const receipt = readProjectData(new URL('../../parity/fixtures/translation-stage-refresh-receipt.lino', import.meta.url));
  const originalBytes = execFileSync('git', ['show', `${receipt.originalRevision}:${receipt.originalPath}`],
    { cwd: fileURLToPath(root), maxBuffer: 16 * 1024 * 1024 });
  const refreshBytes = execFileSync('git', ['show', `${receipt.refreshRevision}:${receipt.refreshPath}`],
    { cwd: fileURLToPath(root), maxBuffer: 16 * 1024 * 1024 });
  assert.equal(hash(originalBytes), receipt.originalSha256);
  assert.equal(hash(refreshBytes), receipt.refreshSha256);
  assert.ok(readFileSync(new URL(receipt.refreshPath, root)).equals(refreshBytes),
    'a later generated snapshot needs its own authenticated immutable receipt');

  const historical = JSON.parse(originalBytes.toString('utf8'));
  const current = readStructuredDataLino(refreshBytes.toString('utf8'));
  assert.deepEqual(normalizeMigrationTranslationStages(historical, current, receipt.refreshes), historical);
  const selectedName = 'frontend-rust/frontend-rust-stringEscapes';
  assert.deepEqual(receipt.refreshes.find(entry => entry.program === selectedName),
    { program: selectedName, stages: ['parse'] });
  assert.throws(() => normalizeMigrationTranslationStages(historical, current,
    receipt.refreshes.filter(entry => entry.program !== selectedName)),
  /protected translation source and stage fields/u);

  const historyPath = 'parity/fixtures/translation-stage-refresh-history.lino';
  const history = readProjectData(new URL(historyPath, root));
  const archivedBlob = (revision, path) => {
    const options = { cwd: fileURLToPath(root), maxBuffer: 16 * 1024 * 1024 };
    return execFileSync('git', ['ls-tree', revision, '--', path], options).length
      ? execFileSync('git', ['show', `${revision}:${path}`], options) : null;
  };
  const priorReceipts = validateTranslationStageRefreshHistory(history, archivedBlob, receipt.refreshRevision);
  assert.deepEqual(captureTranslationStageRefreshHistory(receipt.refreshRevision, archivedBlob), history,
    'the complete archive is derived deterministically from the committed predecessor inputs');
  const committedHistory = readStructuredDataLino(archivedBlob(receipt.refreshRevision, historyPath).toString('utf8'));
  assert.deepEqual(history.receipts.slice(0, -1), committedHistory.receipts,
    'every original ordered receipt entry stays exact before the single appended snapshot entry');
  assert.equal(history.receipts.at(-1).sourceCommit, receipt.refreshRevision);
  assert.throws(() => captureTranslationStageRefreshHistory(receipt.refreshRevision, (revision, path) =>
    revision === receipt.refreshRevision && path === historyPath ? null : archivedBlob(revision, path)),
  /complete committed history and receipt/u);
  assert.throws(() => captureTranslationStageRefreshHistory(receipt.refreshRevision, (revision, path) =>
    revision === priorReceipts[0].refreshRevision && path === priorReceipts[0].refreshPath
      ? Buffer.from('changed snapshot bytes') : archivedBlob(revision, path)),
  /snapshot differs from its immutable Git bytes/u);
  assert.equal(priorReceipts[0].refreshRevision, '62ff1b8b05ab049c51736b06780bab8274ab45cf',
    'the original immutable refresh receipt remains inspectable and authenticated');
  for (const prior of priorReceipts) {
    assert.equal(prior.originalRevision, receipt.originalRevision);
    assert.equal(prior.originalPath, receipt.originalPath);
    assert.equal(prior.originalSha256, receipt.originalSha256);
    for (const { program, stages } of prior.refreshes) {
      const retained = receipt.refreshes.find(entry => entry.program === program);
      assert.ok(retained, `${program}: a historical refresh cannot disappear`);
      for (const stage of stages) assert.ok(retained.stages.includes(stage), `${program}: historical ${stage} scope remains`);
    }
  }
  assert.deepEqual(retainTranslationStageRefreshScopes(priorReceipts, []),
    retainTranslationStageRefreshScopes(priorReceipts, priorReceipts.flatMap(prior => prior.refreshes)),
    'historical scopes remain even when a later snapshot no longer differs in that stage');

  const wrongReceiptHash = structuredClone(history);
  wrongReceiptHash.receipts[0].sourceSha256 = '0'.repeat(64);
  assert.throws(() => validateTranslationStageRefreshHistory(wrongReceiptHash), /immutable Git bytes/u);
  const wrongReceiptContent = structuredClone(history);
  wrongReceiptContent.receipts[0].receipt.refreshSha256 = '0'.repeat(64);
  assert.throws(() => validateTranslationStageRefreshHistory(wrongReceiptContent), /immutable Git bytes/u);
  assert.throws(() => validateTranslationStageRefreshHistory({ schemaVersion: 1, receipts: [] }), /nonempty ordered receipt archive/u);
  assert.throws(() => validateTranslationStageRefreshHistory({ ...history, receipts: [...history.receipts, history.receipts[0]] }),
    /duplicate source identity/u);

  const first = history.receipts[0];
  assert.throws(() => validateTranslationStageRefreshHistory(history, (revision, path) => {
    if (revision === first.sourceCommit && path === historyPath) {
      return Buffer.from(structuredDataToLino(history));
    }
    return execFileSync('git', ['show', `${revision}:${path}`], { cwd: fileURLToPath(root), maxBuffer: 16 * 1024 * 1024 });
  }), /complete preceding archive/u);
  assert.throws(() => validateTranslationStageRefreshHistory(history, (revision, path) => {
    if (revision === first.sourceCommit && path === historyPath) return null;
    if (revision === first.receipt.refreshRevision && path === first.receipt.refreshPath) return Buffer.from('changed snapshot bytes');
    return execFileSync('git', ['show', `${revision}:${path}`], { cwd: fileURLToPath(root), maxBuffer: 16 * 1024 * 1024 });
  }), /snapshot differs from its immutable Git bytes/u);
  // This immutable snapshot is also the source of the newest archived receipt.
  // The original corruption therefore reaches its source authentication first.
  const archivedSnapshot = history.receipts.some(entry => entry.sourceCommit === receipt.refreshRevision);
  const latestReceiptError = archivedSnapshot
    ? /history receipt differs from its immutable Git bytes/u : /latest prior receipt/u;
  assert.throws(() => validateTranslationStageRefreshHistory(history, (revision, path) => {
    if (revision === receipt.refreshRevision && path === first.sourcePath) return Buffer.from('changed prior receipt bytes');
    return archivedBlob(revision, path);
  }, receipt.refreshRevision), latestReceiptError);
  // Retain a separate actual late-lookup corruption after the same immutable
  // source was authenticated. Both ordered guards must still execute.
  let latestReceiptReads = 0;
  assert.throws(() => validateTranslationStageRefreshHistory(history, (revision, path) => {
    if (revision === receipt.refreshRevision && path === first.sourcePath && ++latestReceiptReads === 2) {
      return Buffer.from('changed prior receipt bytes');
    }
    return archivedBlob(revision, path);
  }, receipt.refreshRevision), /latest prior receipt/u);
  assert.equal(latestReceiptReads, 2);
  const latestArchiveError = archivedSnapshot
    ? /complete preceding archive/u : /committed archive/u;
  assert.throws(() => validateTranslationStageRefreshHistory(history, (revision, path) => {
    if (revision === receipt.refreshRevision && path === historyPath) return Buffer.from(structuredDataToLino({ schemaVersion: 2 }));
    return archivedBlob(revision, path);
  }, receipt.refreshRevision), latestArchiveError);
  let latestHistoryReads = 0;
  assert.throws(() => validateTranslationStageRefreshHistory(history, (revision, path) => {
    if (revision === receipt.refreshRevision && path === historyPath && ++latestHistoryReads === 2) {
      return Buffer.from(structuredDataToLino({ schemaVersion: 2 }));
    }
    return archivedBlob(revision, path);
  }, receipt.refreshRevision), /committed archive/u);
  assert.equal(latestHistoryReads, 2);
  assert.throws(() => validateTranslationStageRefreshHistory({ schemaVersion: 1, receipts: history.receipts.slice(0, -1) },
    archivedBlob, receipt.refreshRevision), /receipt archive|latest prior receipt/u);
});
