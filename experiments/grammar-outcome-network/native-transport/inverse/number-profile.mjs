// Bind representation selection to the exact compiler artifact already checked
// by the outcome runner, rather than the union of declared Cargo features.
import assert from 'node:assert/strict';
import {readOutcomeCompilerProfile} from '../runner/compiler-profile.mjs';
import {readOwnedOutcomeNativeNumberKind} from './number-kind-rules.mjs';

export function readOwnedOutcomeNativeNumberProfile(inputs) {
  const compilerProfile=readOutcomeCompilerProfile(inputs);
  const version='1.0.151'; // The compiler reader checks this exact locked package.
  assert.ok(compilerProfile.dependency.package_id.endsWith('#serde_json@'+version));
  const arbitraryPrecision=compilerProfile.dependency.features.includes('arbitrary_precision');
  assert.equal(readOwnedOutcomeNativeNumberKind(true,version,arbitraryPrecision,false,false,true),'floating point',
    'uncovered native JSON number representation; arbitrary_precision requires its own source model');
  return {schemaVersion:1,version,arbitraryPrecision,compilerProfile,
    boundary:'Selected actual compiler record model only. Caller-supplied compiler text is not an executed original number observation; payload and occurrence facts must still be captured before projection.'};
}
