#!/usr/bin/env node
// Authenticate the exact generated-stage changes in a reviewed immutable batch.
// This reads fixture metadata only. It does not execute the translation matrix.
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { isDeepStrictEqual } from 'node:util';
import { fileURLToPath } from 'node:url';
import { readStructuredDataLino } from '../src/structured-data-links.js';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { normalizeMigrationTranslationStages } from './validate-structured-data-migration.mjs';

const root = fileURLToPath(new URL('../../', import.meta.url));
const originalRevision = '2b2a72a74e4ba74a2dda9bd55edc02086c50eb68';
const refreshRevision = '0312243800e40953ae3395ca559857df21b03db3';
const originalPath = 'parity/fixtures/translation-stages.json';
const refreshPath = 'parity/fixtures/translation-stages.lino';
const destination = new URL('../../parity/fixtures/translation-stage-refresh-receipt.lino', import.meta.url);
const hash = bytes => createHash('sha256').update(bytes).digest('hex');
const blob = (revision, path) => execFileSync('git', ['show', `${revision}:${path}`], {
  cwd: root, maxBuffer: 16 * 1024 * 1024, timeout: 30000, stdio: ['ignore', 'pipe', 'pipe'],
});
const originalBytes = blob(originalRevision, originalPath);
const refreshBytes = blob(refreshRevision, refreshPath);
const historical = JSON.parse(originalBytes.toString('utf8'));
const current = readStructuredDataLino(refreshBytes.toString('utf8'));
if (!isDeepStrictEqual(historical.programs.map(program => program.name), current.programs.map(program => program.name))) {
  throw new Error('stage refresh cannot change program identities or occurrence order');
}
const refreshes = [];
for (let index = 0; index < historical.programs.length; index += 1) {
  const before = historical.programs[index];
  const after = current.programs[index];
  const stages = ['parse', 'check', 'emit'].filter(stage => !isDeepStrictEqual(before[stage], after[stage]));
  if (stages.length) refreshes.push({ program: before.name, stages });
}
// Every source, descriptor and unselected field is still independently frozen.
normalizeMigrationTranslationStages(historical, current, refreshes);
const receipt = { schemaVersion: 1, originalRevision, originalPath,
  originalSha256: hash(originalBytes), refreshRevision, refreshPath, refreshSha256: hash(refreshBytes), refreshes };
if (process.argv.slice(2).some(argument => argument !== '--check')) throw new Error('usage: generate-translation-stage-refresh-receipt.mjs [--check]');
if (process.argv.includes('--check')) checkProjectData(destination, receipt);
else writeProjectData(destination, receipt);
console.log(`authenticated ${refreshes.length} generated-stage refresh identities; source metadata unchanged`);
