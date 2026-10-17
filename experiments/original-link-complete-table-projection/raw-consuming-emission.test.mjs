// Complete source-compiler proofs only. No corpus, grammar parse or native process.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {ORIGINAL_LINK_RAW_EMISSION_BASELINE,ORIGINAL_LINK_RAW_CURRENT_ARTIFACT,ORIGINAL_LINK_RAW_HISTORICAL_ARTIFACT} from './raw-emission-profiles.mjs';
import {ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT,generateOriginalLinkRawConsumingEmission,
  readOriginalLinkRawConsumingOwnerSites,readOriginalLinkRawConsumingProfile} from './raw-consuming-emission.mjs';

const sha256=value=>createHash('sha256').update(value).digest('hex');
let consumingGeneration;
const generation=()=>consumingGeneration??=generateOriginalLinkRawConsumingEmission(true);

test('raw consuming token inventories ignore text and retain actual complete owner interfaces',()=>{
  const source='pub fn selected(values:Vec<f64>)->Vec<f64>{let text="values.clone() crate::ml_array::append(";values.clone()}';
  const sites=readOriginalLinkRawConsumingOwnerSites(source);
  assert.equal(sites.target,'selected');assert.equal(sites.kind,'fn');
  assert.equal(sites.cloneSites.length,1);assert.equal(sites.appendSites.length,0);
  assert.equal(source.slice(sites.cloneSites[0],sites.cloneSites[0]+5),'clone');
  assert.deepEqual(sites.header,['pub','fn','selected','(','values',':','Vec','<','f64','>',')','->','Vec','<','f64','>']);
  assert.throws(()=>readOriginalLinkRawConsumingOwnerSites(source+'\n'+source),/one complete/u);
});

test('raw consuming emission keeps all106owners and both complete typed archives without changing either predecessor',async()=>{
  const historical=readFileSync(ORIGINAL_LINK_RAW_HISTORICAL_ARTIFACT), current=readFileSync(ORIGINAL_LINK_RAW_CURRENT_ARTIFACT);
  const {artifact,profile}=await generation();
  assert.deepEqual(readFileSync(ORIGINAL_LINK_RAW_HISTORICAL_ARTIFACT),historical);
  assert.deepEqual(readFileSync(ORIGINAL_LINK_RAW_CURRENT_ARTIFACT),current);
  assert.equal(sha256(historical),ORIGINAL_LINK_RAW_EMISSION_BASELINE.artifactSha256);
  assert.equal(sha256(readProjectData(ORIGINAL_LINK_RAW_CURRENT_ARTIFACT).completeRust),ORIGINAL_LINK_RAW_EMISSION_BASELINE.currentSha256);
  assert.equal(artifact.definitionCount,106);assert.equal(artifact.owners.length,106);
  assert.deepEqual(artifact.owners.map(owner=>owner.fullName),artifact.declarationNames);
  assert.deepEqual(artifact.declarationNames,readProjectData(ORIGINAL_LINK_RAW_CURRENT_ARTIFACT).declarationNames);
  assert.deepEqual(artifact.options,{arrayConsumption:true});
  assert.deepEqual(artifact.totals,{copy:{cloneCalls:1004,appendCalls:31,consumingAppendBindings:0},
    consuming:{cloneCalls:742,appendCalls:31,consumingAppendBindings:25}});
  assert.equal(artifact.sourceBytes,351559);
  assert.equal(artifact.sourceSha256,'b0820a709afb3c66fc88321ceaa84e00baa795b0f07290896db6d00c740d9c7c');
  for(const owner of artifact.owners)assert.deepEqual(owner.consuming.header,owner.copy.header);
  assert.deepEqual(artifact.replays.map(row=>row.format),['text','binary']);
  for(const replay of artifact.replays){assert.equal(replay.ownerCount,106);assert.equal(replay.emittedSha256,artifact.sourceSha256);assert.equal(replay.emittedBytes,artifact.sourceBytes);}
  assert.equal(artifact.archive.binaryLimits.recordLimit,artifact.archive.recordCount);
  assert.ok(artifact.archive.recordCount>0&&artifact.archive.recordCount<=1_000_000);
  assert.equal(artifact.archive.binaryLimits.byteLimit,64*1024*1024);
  assert.equal(artifact.nativeCompilation,false);assert.equal(artifact.nativeExecution,false);
  assert.deepEqual(readOriginalLinkRawConsumingProfile(artifact.predecessor),profile);
  assert.equal(profile.proof.artifactSha256,sha256(readFileSync(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT)));
});

test('raw consuming stored profiles refuse altered source decisions owners sites options and archive obligations',async()=>{
  const {artifact}=await generation();
  const read=changed=>file=>String(file)===String(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT)
    ?Buffer.from(structuredDataToLino(changed)):readFileSync(file);
  for(const mutate of [
    row=>{row.options.arrayConsumption=false;},
    row=>{row.owners.reverse();},
    row=>{row.owners[1]=structuredClone(row.owners[0]);},
    row=>{row.owners[0].copy.header.push('changed');},
    row=>{row.totals.consuming.cloneCalls-=1;},
    row=>{row.replays.pop();},
    row=>{row.replays[1].emittedSha256='0'.repeat(64);},
    row=>{row.archive.binaryLimits.recordLimit-=1;},
    row=>{row.completeRust+='\n';},
    row=>{row.nativeExecution=true;},
  ]) {
    const changed=structuredClone(artifact);mutate(changed);
    assert.throws(()=>readOriginalLinkRawConsumingProfile(artifact.predecessor,read(changed)),assert.AssertionError);
  }
  assert.throws(()=>readOriginalLinkRawConsumingProfile(artifact.predecessor,file=>{
    const bytes=readFileSync(file);
    return String(file).endsWith('/owned-array-moves.js')?Buffer.concat([bytes,Buffer.from('\n')]):bytes;
  }),/actual final-use decision sources changed/u);
});
