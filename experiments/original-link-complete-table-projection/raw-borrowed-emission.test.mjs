import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT, readOriginalLinkRawConsumingProfile} from './raw-consuming-emission.mjs';
import {prepareOriginalLinkRawBorrowedEmission, readOriginalLinkRawBorrowedProfile} from './raw-borrowed-emission.mjs';
import {planOriginalLinkRawBorrowedNative} from './raw-native-observations.mjs';

test('raw immutable parameter loans retain the complete predecessor and every checked definition', () => {
  const before=readFileSync(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT);
  const stored=readProjectData(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT);
  const predecessor=readOriginalLinkRawConsumingProfile(stored.predecessor);
  const {source,receipt,profile}=prepareOriginalLinkRawBorrowedEmission(predecessor);
  assert.equal(receipt.definitionCount,106);assert.deepEqual(receipt.declarationNames,stored.declarationNames);
  assert.deepEqual(profile.proof.predecessor,predecessor);
  assert.deepEqual(receipt.cloneCalls,{before:746,after:396});
  assert.equal(receipt.styles.length,59);
  assert.equal(receipt.publicEntryHeader,'pub fn check_original_link_raw_shape(root_high: f64, root_low: f64, columns: Vec<Vec<f64>>) -> Vec<f64> {');
  assert.equal(source.split(receipt.publicEntryHeader).length,2);
  assert.equal(receipt.nativeCompilation,false);assert.equal(receipt.nativeExecution,false);
  assert.deepEqual(readOriginalLinkRawBorrowedProfile(stored.predecessor),profile);
  assert.deepEqual(readFileSync(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT),before);
});

test('raw immutable parameter loans reject changed predecessor bytes and actual mechanical decision sources', () => {
  const stored=readProjectData(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT);
  const predecessor=readOriginalLinkRawConsumingProfile(stored.predecessor);
  for(const file of [ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT,new URL('../../js/scripts/portable-source-library-style.mjs',import.meta.url)]) {
    assert.throws(()=>prepareOriginalLinkRawBorrowedEmission(predecessor,{read:path=>{
      const bytes=readFileSync(path);return String(path)===String(file)?Buffer.concat([bytes,Buffer.from('\n')]):bytes;
    }}),/complete consuming predecessor bytes changed|project data is stale/u);
  }
  const changed=structuredClone(predecessor);changed.sourceSha256='0'.repeat(64);
  assert.throws(()=>prepareOriginalLinkRawBorrowedEmission(changed),assert.AssertionError);
});

test('raw loan native planning remains CI-only and cannot launch a detached checked plan', async () => {
  await assert.rejects(planOriginalLinkRawBorrowedNative({},{}),/CI/u);
  await assert.rejects(planOriginalLinkRawBorrowedNative({},{CI:'true',ORIGINAL_LINK_RAW_NATIVE:'1'}),/exact checked consuming plan/u);
});
