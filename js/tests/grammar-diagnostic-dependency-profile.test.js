import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import {
  captureDiagnosticDependencyProfile,
  diagnosticFixturePath,
} from '../../experiments/grammar-outcome-network/native-transport/diagnostic-order/generate.mjs';
import { checkProjectData, readProjectData } from '../scripts/project-data.mjs';

const repository = new URL('../../', import.meta.url);
const currentManifestProfile = new URL(
  'parity/fixtures/grammar-selection-diagnostic-dependency-profile.lino', repository,
);

test('the additional packaged identifier model preserves the exact diagnostic dependency profile', () => {
  const historical = readProjectData(new URL(diagnosticFixturePath, repository)).dependencyProfile;
  const current = captureDiagnosticDependencyProfile();
  const { manifest: historicalManifest, ...historicalDependency } = historical;
  const { manifest: currentManifest, ...currentDependency } = current;
  assert.deepEqual(currentDependency, historicalDependency);
  assert.equal(currentManifest.selected.source, historicalManifest.selected.source);
  assert.equal(currentManifest.selected.sha256, historicalManifest.selected.sha256);

  const source = readFileSync(new URL(currentManifest.path, repository), 'utf8');
  const addedAsset = '    "src/data/javascript-identifier-models.lino",\n';
  assert.equal(source.split(addedAsset).length, 2);
  const predecessor = source.replace(addedAsset, '');
  assert.equal(Buffer.byteLength(predecessor), historicalManifest.bytes);
  assert.equal(createHash('sha256').update(predecessor).digest('hex'), historicalManifest.sha256);
  checkProjectData(currentManifestProfile, currentManifest);
});
