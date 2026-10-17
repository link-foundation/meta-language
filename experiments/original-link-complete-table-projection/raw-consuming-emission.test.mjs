// Complete source-compiler proofs only. No corpus, grammar parse or native process.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {structuredDataToLino} from '../../js/src/structured-data-links.js';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {ORIGINAL_LINK_RAW_EMISSION_BASELINE,ORIGINAL_LINK_RAW_CURRENT_ARTIFACT,ORIGINAL_LINK_RAW_HISTORICAL_ARTIFACT} from './raw-emission-profiles.mjs';
import {ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT,ORIGINAL_LINK_RAW_CONSUMING_HISTORICAL_ARTIFACT,
  ORIGINAL_LINK_RAW_CONSUMING_SUCCESSOR,generateOriginalLinkRawConsumingEmission,
  readOriginalLinkRawConsumingOwnerSites,readOriginalLinkRawConsumingProfile,
  verifyOriginalLinkRawConsumingConservation} from './raw-consuming-emission.mjs';

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


test('raw consuming successors preserve the complete original artifact through an exact provenance inverse',async()=>{
  const originalBytes=readFileSync(ORIGINAL_LINK_RAW_CONSUMING_HISTORICAL_ARTIFACT);
  assert.equal(originalBytes.length,1801946);
  assert.equal(sha256(originalBytes),'3c1793f7ce672a846e2fc480f20596e985b9528644748a8078cd170ef5ac81c8');
  const original=readProjectData(ORIGINAL_LINK_RAW_CONSUMING_HISTORICAL_ARTIFACT);
  const {artifact,profile}=await generation(), successor=ORIGINAL_LINK_RAW_CONSUMING_SUCCESSOR;
  assert.notEqual(String(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT),String(ORIGINAL_LINK_RAW_CONSUMING_HISTORICAL_ARTIFACT));
  assert.ok(String(ORIGINAL_LINK_RAW_CONSUMING_ARTIFACT).endsWith('/data/raw-consuming-current-native-definitions.lino'));
  assert.equal(profile.sourceArtifact,'data/raw-consuming-current-native-definitions.lino');
  assert.deepEqual(artifact.predecessorConservation,successor);
  assert.deepEqual(profile.proof.predecessorConservation,successor);
  assert.deepEqual(verifyOriginalLinkRawConsumingConservation(artifact),successor);
  const inverse=structuredClone(artifact), refresh=successor.receiptRefresh;
  delete inverse.predecessorConservation;
  assert.deepEqual(inverse.decisionSources.map(row=>row.file),original.decisionSources.map(row=>row.file));
  inverse.decisionSources=structuredClone(original.decisionSources);
  const applicability=inverse.predecessor.proof.producerApplicability;
  assert.deepEqual(applicability.receiptRefresh,refresh);delete applicability.receiptRefresh;
  const positions=applicability.sources.flatMap((row,index)=>row.file===refresh.current.file?[index]:[]);
  assert.equal(positions.length,1);assert.deepEqual(applicability.sources[positions[0]],refresh.current);
  applicability.sources[positions[0]]=structuredClone(refresh.previous);
  for(const change of successor.archiveChanges){assert.equal(inverse.archive[change.field],change.after);inverse.archive[change.field]=change.before;}
  assert.deepEqual(inverse,original);
  assert.deepEqual(Buffer.from(structuredDataToLino(inverse)),originalBytes);
  for(const field of ['completeRust','owners','declarationNames','totals','assumptions','encodings','mappings','preludeHashes','replays'])
    assert.deepEqual(artifact[field],original[field],'the entire original '+field+' is retained');
  assert.deepEqual(readFileSync(ORIGINAL_LINK_RAW_CONSUMING_HISTORICAL_ARTIFACT),originalBytes);
});

test('raw consuming provenance inverses refuse unknown changes repins duplicate descriptors and original byte tampering',async()=>{
  const {artifact}=await generation(), refresh=ORIGINAL_LINK_RAW_CONSUMING_SUCCESSOR.receiptRefresh;
  for(const mutate of [
    row=>{row.extra='unreviewed';},
    row=>{row.predecessorConservation.original.sha256='0'.repeat(64);},
    row=>{row.predecessorConservation.archiveChanges.push({field:'textSha256',before:row.archive.textSha256,after:'0'.repeat(64)});},
    row=>{row.decisionSources[0].sha256='0'.repeat(64);},
    row=>{row.decisionSources.reverse();},
    row=>{delete row.predecessor.proof.producerApplicability.receiptRefresh;},
    row=>{row.predecessor.proof.producerApplicability.receiptRefresh.revision+=' changed';},
    row=>{row.predecessor.proof.producerApplicability.sources.find(source=>source.file===refresh.current.file).sha256=refresh.previous.sha256;},
    row=>{row.predecessor.proof.producerApplicability.sources.push(structuredClone(refresh.current));},
    row=>{row.predecessor.proof.producerApplicability.sources[0].sha256='0'.repeat(64);},
    row=>{row.archive.textSha256='0'.repeat(64);},
    row=>{row.archive.binarySha256='0'.repeat(64);},
    row=>{row.archive.textBytes+=1;},
    row=>{row.archive.recordCount-=1;},
    row=>{row.owners[0].fullName+=' changed';},
    row=>{row.completeRust+='\n';},
    row=>{row.replays[0].emittedSha256='0'.repeat(64);},
  ]) {
    const changed=structuredClone(artifact);mutate(changed);
    assert.throws(()=>verifyOriginalLinkRawConsumingConservation(changed),assert.AssertionError);
  }
  const originalBytes=readFileSync(ORIGINAL_LINK_RAW_CONSUMING_HISTORICAL_ARTIFACT);
  for(const changed of [Buffer.concat([originalBytes,Buffer.from('\n')]),Buffer.from(originalBytes)]) {
    if(changed.length===originalBytes.length)changed[0]^=1;
    assert.throws(()=>verifyOriginalLinkRawConsumingConservation(artifact,file=>
      String(file)===String(ORIGINAL_LINK_RAW_CONSUMING_HISTORICAL_ARTIFACT)?changed:readFileSync(file)),
    /original consuming artifact/u);
  }
});
