#!/usr/bin/env node
// Authenticate the exact generated-stage changes in a reviewed immutable batch.
// This reads fixture metadata only. It does not execute the translation matrix.
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { resolve } from 'node:path';
import { isDeepStrictEqual } from 'node:util';
import { fileURLToPath } from 'node:url';
import { readStructuredDataLino } from '../src/structured-data-links.js';
import { checkProjectData, readProjectData, writeProjectData } from './project-data.mjs';
import { normalizeMigrationTranslationStages } from './validate-structured-data-migration.mjs';

const root = fileURLToPath(new URL('../../', import.meta.url));
const originalRevision = '2b2a72a74e4ba74a2dda9bd55edc02086c50eb68';
const refreshRevision = 'a8a789afb0ed569340d9079279f534f0bf7c4e72';
const originalPath = 'parity/fixtures/translation-stages.json';
const refreshPath = 'parity/fixtures/translation-stages.lino';
const receiptPath = 'parity/fixtures/translation-stage-refresh-receipt.lino';
const historyPath = 'parity/fixtures/translation-stage-refresh-history.lino';
const destination = new URL('../../parity/fixtures/translation-stage-refresh-receipt.lino', import.meta.url);
const hash = bytes => createHash('sha256').update(bytes).digest('hex');
const blob = (revision, path) => execFileSync('git', ['show', `${revision}:${path}`], {
  cwd: root, maxBuffer: 16 * 1024 * 1024, timeout: 30000, stdio: ['ignore', 'pipe', 'pipe'],
});

// A null result means this exact path is absent from an otherwise valid commit.
// Other Git failures remain errors rather than being interpreted as missing history.
const optionalBlob = (revision, path) => {
  const listing = execFileSync('git', ['ls-tree', revision, '--', path], {
    cwd: root, maxBuffer: 1024 * 1024, timeout: 30000, stdio: ['ignore', 'pipe', 'pipe'],
  });
  return listing.length ? blob(revision, path) : null;
};

export function validateTranslationStageRefreshHistory(history, loadBlob = optionalBlob, snapshotRevision) {
  if (!history || history.schemaVersion !== 1 || !Array.isArray(history.receipts) || !history.receipts.length
      || !isDeepStrictEqual(Object.keys(history).sort(), ['receipts', 'schemaVersion'])) {
    throw new Error('stage refresh history requires a nonempty ordered receipt archive');
  }
  const identities = new Set();
  const receipts = [];
  for (let index = 0; index < history.receipts.length; index += 1) {
    const entry = history.receipts[index];
    if (!entry || !isDeepStrictEqual(Object.keys(entry).sort(), ['receipt', 'sourceCommit', 'sourcePath', 'sourceSha256'])
        || typeof entry.sourceCommit !== 'string' || !/^[0-9a-f]{40}$/u.test(entry.sourceCommit) || entry.sourcePath !== receiptPath
        || typeof entry.sourceSha256 !== 'string' || !/^[0-9a-f]{64}$/u.test(entry.sourceSha256) || identities.has(entry.sourceCommit)) {
      throw new Error('stage refresh history has an invalid or duplicate source identity');
    }
    identities.add(entry.sourceCommit);
    const receiptBytes = loadBlob(entry.sourceCommit, entry.sourcePath);
    if (!receiptBytes || hash(receiptBytes) !== entry.sourceSha256
        || !isDeepStrictEqual(readStructuredDataLino(receiptBytes.toString('utf8')), entry.receipt)) {
      throw new Error('stage refresh history receipt differs from its immutable Git bytes');
    }
    const previousBytes = loadBlob(entry.sourceCommit, historyPath);
    const previous = { schemaVersion: 1, receipts: history.receipts.slice(0, index) };
    if (previousBytes ? !isDeepStrictEqual(readStructuredDataLino(previousBytes.toString('utf8')), previous) : index !== 0) {
      throw new Error('stage refresh history does not preserve its complete preceding archive');
    }
    const receipt = entry.receipt;
    if (!receipt || receipt.schemaVersion !== 1 || receipt.originalRevision !== originalRevision || receipt.originalPath !== originalPath
        || typeof receipt.refreshRevision !== 'string' || !/^[0-9a-f]{40}$/u.test(receipt.refreshRevision) || receipt.refreshPath !== refreshPath) {
      throw new Error('stage refresh history changed the original source or snapshot identity');
    }
    const originalBytes = loadBlob(receipt.originalRevision, receipt.originalPath);
    const snapshotBytes = loadBlob(receipt.refreshRevision, receipt.refreshPath);
    if (!originalBytes || hash(originalBytes) !== receipt.originalSha256
        || !snapshotBytes || hash(snapshotBytes) !== receipt.refreshSha256) {
      throw new Error('stage refresh history snapshot differs from its immutable Git bytes');
    }
    const historical = JSON.parse(originalBytes.toString('utf8'));
    normalizeMigrationTranslationStages(historical, readStructuredDataLino(snapshotBytes.toString('utf8')), receipt.refreshes);
    receipts.push(receipt);
  }
  if (snapshotRevision !== undefined) {
    if (typeof snapshotRevision !== 'string' || !/^[0-9a-f]{40}$/u.test(snapshotRevision)) {
      throw new Error('stage refresh history needs an immutable snapshot revision');
    }
    const latest = history.receipts.at(-1);
    const priorReceiptBytes = loadBlob(snapshotRevision, receiptPath);
    if (!priorReceiptBytes || hash(priorReceiptBytes) !== latest.sourceSha256
        || !isDeepStrictEqual(readStructuredDataLino(priorReceiptBytes.toString('utf8')), latest.receipt)) {
      throw new Error('stage refresh history omitted or changed the snapshot revision\'s latest prior receipt');
    }
    const priorHistoryBytes = loadBlob(snapshotRevision, historyPath);
    const prefix = { schemaVersion: 1, receipts: history.receipts.slice(0, -1) };
    const priorHistory = priorHistoryBytes ? readStructuredDataLino(priorHistoryBytes.toString('utf8')) : null;
    if (priorHistory ? !isDeepStrictEqual(priorHistory, prefix) && !isDeepStrictEqual(priorHistory, history) : prefix.receipts.length !== 0) {
      throw new Error('stage refresh history omitted or changed the snapshot revision\'s committed archive');
    }
  }
  return receipts;
}

export function retainTranslationStageRefreshScopes(receipts, refreshes) {
  const retained = new Map();
  for (const receipt of [...receipts, { refreshes }]) {
    for (const { program, stages } of receipt.refreshes) {
      if (!retained.has(program)) retained.set(program, []);
      for (const stage of stages) if (!retained.get(program).includes(stage)) retained.get(program).push(stage);
    }
  }
  return [...retained].map(([program, stages]) => ({ program, stages }));
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.slice(2).some(argument => argument !== '--check')) throw new Error('usage: generate-translation-stage-refresh-receipt.mjs [--check]');
  const history = readProjectData(new URL('../../' + historyPath, import.meta.url));
  const receipts = validateTranslationStageRefreshHistory(history, optionalBlob, refreshRevision);
  const originalBytes = blob(originalRevision, originalPath);
  const refreshBytes = blob(refreshRevision, refreshPath);
  const historical = JSON.parse(originalBytes.toString('utf8'));
  const current = readStructuredDataLino(refreshBytes.toString('utf8'));
  if (!isDeepStrictEqual(historical.programs.map(program => program.name), current.programs.map(program => program.name))) {
    throw new Error('stage refresh cannot change program identities or occurrence order');
  }
  const changes = [];
  for (let index = 0; index < historical.programs.length; index += 1) {
    const before = historical.programs[index];
    const after = current.programs[index];
    const stages = ['parse', 'check', 'emit'].filter(stage => !isDeepStrictEqual(before[stage], after[stage]));
    if (stages.length) changes.push({ program: before.name, stages });
  }
  const refreshes = retainTranslationStageRefreshScopes(receipts, changes);
  // Every source, descriptor and unselected field is still independently frozen.
  normalizeMigrationTranslationStages(historical, current, refreshes);
  const receipt = { schemaVersion: 1, originalRevision, originalPath,
    originalSha256: hash(originalBytes), refreshRevision, refreshPath, refreshSha256: hash(refreshBytes), refreshes };
  if (process.argv.includes('--check')) checkProjectData(destination, receipt);
  else writeProjectData(destination, receipt);
  console.log(`authenticated ${refreshes.length} generated-stage refresh identities; source metadata unchanged`);
}
