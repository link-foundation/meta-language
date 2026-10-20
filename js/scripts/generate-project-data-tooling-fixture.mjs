#!/usr/bin/env node
// Immutable independent inputs for tooling evidence, not successful results.
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';
import { checkProjectData, readProjectData, writeProjectData } from './project-data.mjs';

const stageRefreshReceiptPath = 'parity/fixtures/translation-stage-refresh-receipt.lino';
const stageRefreshReceipt = readProjectData(new URL('../../' + stageRefreshReceiptPath, import.meta.url));
const priorStageRefreshes = [
  { program: 'check/rust-match-tuple', stages: ['parse'] },
  { program: 'frontend-rust/frontend-rust-typeTuple', stages: ['parse', 'check', 'emit'] },
  { program: 'frontend-rust/frontend-rust-letTuple', stages: ['parse', 'check'] },
  { program: 'frontend-rust/frontend-rust-methodNotIdentifier', stages: ['parse', 'check'] },
  { program: 'frontend-rust/frontend-rust-tuple', stages: ['parse', 'check'] },
  { program: 'frontend-rust/frontend-rust-patternTuple', stages: ['parse'] },
];
const stageRefreshes = new Map(priorStageRefreshes.map(({program, stages}) => [program, [...stages]]));
for (const {program, stages} of stageRefreshReceipt.refreshes) {
  const selected = new Set([...(stageRefreshes.get(program) ?? []), ...stages]);
  stageRefreshes.set(program, ['parse', 'check', 'emit'].filter(stage => selected.has(stage)));
}

export const projectDataToolingContract = Object.freeze({
  schemaVersion: 1,
  originalGitRevision: '33ef0fd779316f56084652cde036872f26bda0c5',
  migrationBaselineFile: 'parity/fixtures/structured-data-migration-baselines.lino',
  migrationRevisionFile: 'parity/fixtures/structured-data-migration-revisions.lino',
  migrationBaselineCount: 167,
  currentFreshnessChecks: [
    { logicalPaths: ['parity/fixtures/native-default-cst-expected.json'],
      providerReceiptRefresh: [{ language: 'R', provider: 'native-r' }, { language: 'Python', provider: 'native-python' },
        { language: 'Agda', provider: 'native-agda' }],
      command: ['node', 'js/scripts/generate-native-grammar-fixtures.mjs', '--language', 'r', '--refresh-provenance', '--check', '--language', 'python', '--language', 'agda'] },
    { logicalPaths: ['parity/fixtures/native-grammar-concept-reuse.json'],
      command: ['node', 'js/scripts/build-native-grammar-concept-reuse.mjs', '--check'] },
    { logicalPaths: ['parity/fixtures/translation-stages.json'],
      translationStageRefreshReceipt: stageRefreshReceiptPath,
      translationStageRefresh: [...stageRefreshes].map(([program, stages]) => ({program, stages})),
      command: ['node', 'js/scripts/build-translation-stage-fixtures.mjs', '--check'] },
    { logicalPaths: ['parity/fixtures/grammar-importers.json'],
      antlrLexerDeclarationRefresh: true,
      command: ['node', 'js/scripts/generate-antlr-reverse-fixture.mjs', '--check'] },
    { logicalPaths: ['parity/fixtures/typescript-erasure.json'],
      command: ['node', 'js/scripts/generate-typescript-erasure-fixtures.mjs', '--check'] },
  ],
  protectedScopeBaselines: ['950efbee452da0853e272d841445eb85caa2d81f',
    '33ef0fd779316f56084652cde036872f26bda0c5',
    '4dd3243b1e503dd7603a48c20da23ecbca222f1a'],
  exceptionRegistry: 'parity/project-data-json-exceptions.lino',
  assetManifest: 'parity/grammars/external-assets.lino',
  nativeInventory: 'parity/language-grammar-inventory.lino',
  trackedProvenance: ['parity/grammars/external-assets.lino',
    'parity/grammars/sources.lino', 'parity/grammars-v4-sources.lino',
    'js/src/vendor/grammars/grammar-lock.lino', 'rust/src/data/grammar-lock.lino'],
  agdaCorpus: 'parity/fixtures/native-grammars/agda-corpus.lino',
  assetCacheDirectory: '.external-grammars',
});

export const projectDataToolingFixture = new URL('../../parity/fixtures/project-data-tooling-contract.lino', import.meta.url);

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.slice(2).some(value => value !== '--check')) throw new Error('usage: generate-project-data-tooling-fixture.mjs [--check]');
  if (process.argv.includes('--check')) checkProjectData(projectDataToolingFixture, projectDataToolingContract);
  else writeProjectData(projectDataToolingFixture, projectDataToolingContract);
}
