#!/usr/bin/env node
// Immutable independent inputs for tooling evidence, not successful results.
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';
import { checkProjectData, writeProjectData } from './project-data.mjs';

export const projectDataToolingContract = Object.freeze({
  schemaVersion: 1,
  originalGitRevision: '33ef0fd779316f56084652cde036872f26bda0c5',
  migrationBaselineFile: 'parity/fixtures/structured-data-migration-baselines.lino',
  migrationBaselineCount: 167,
  protectedScopeBaselines: ['950efbee452da0853e272d841445eb85caa2d81f',
    '33ef0fd779316f56084652cde036872f26bda0c5'],
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
