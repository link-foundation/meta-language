import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData,serializeProjectData} from '../scripts/project-data.mjs';
import {reviewedOverlayProviderPolicy,readReviewedOverlayProviderSources,restoreReviewedOverlayProviderSources}
  from '../../experiments/rust-text-certificate-adapter/reviewed-providers.mjs';
import {readReviewedOverlayProviderSuccession,verifyReviewedOverlayProviderSuccession,
  restoreReviewedOverlayProviderPolicy,restoreReviewedOverlayProviderReceiptSource,
  OVERLAY_PROVIDER_ORIGINAL_PACKET,OVERLAY_PROVIDER_APPEND_REVIEW,OVERLAY_PROVIDER_REFERENCE_REVIEW,deriveReviewedOverlayProviderPolicy,
  verifyReviewedOverlayProviderSuccessionHistory,restoreReviewedOverlayProviderModuleSource}
  from '../../experiments/rust-text-certificate-adapter/provider-succession.mjs';
import {generateReviewedOverlayProviderSuccession} from '../../experiments/rust-text-certificate-adapter/generate-provider-succession.mjs';
import {rustTextCertificateOverlayRecords} from '../../experiments/rust-text-certificate-adapter/generate.mjs';
import {rawBindingCertificateOverlayRecords} from '../../experiments/rust-raw-binding-certificate-adapter/generate-overlay.mjs';
import {createRustTextCertificatePublicOverlay} from '../../experiments/rust-text-certificate-adapter/runtime.mjs';
import {createRustRawBindingCertificateOverlay} from '../../experiments/rust-raw-binding-certificate-adapter/runtime.mjs';
const root=new URL('../../',import.meta.url),read=file=>readFileSync(new URL(file,root));
const hash=value=>createHash('sha256').update(value).digest('hex');

test('reviewed overlay source assertions select covered active Node versions',()=>{
  const[major,minor]=process.versions.node.split('.').map(Number);
  assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 || >=24 <25 required');
});

test('actual five-provider profiles have one exact reviewed revision and complete committed inverses',()=>{
  const packet=readReviewedOverlayProviderSuccession(),current=reviewedOverlayProviderPolicy.revisions.at(-1),modules=current.providers.map(row=>row.module);
  assert.equal(packet.profiles[1].sourceCommit,'8ef026a395ebbbc415f948777f7f6f7e082a572e');
  assert.deepEqual(current,packet.profiles.at(-1));
  assert.deepEqual(packet.profiles.slice(0,2).map(row=>row.inverse.map(action=>action.module)),[['typed-program-links-validation.js'],['translation/emit-javascript.js']]);
  for(const experiment of ['rust-text-certificate-adapter','rust-raw-binding-certificate-adapter']){
    const actual=readReviewedOverlayProviderSources({experiment,modules});assert.equal(actual.revision,current);
    for(const row of current.providers)assert.equal(hash(read('js/src/'+row.module)),row.sourceSha256);
    for(const sourceCommit of [...packet.profiles.slice(0,-1).map(row=>row.sourceCommit),packet.previousSourceCommit]){
      const restored=restoreReviewedOverlayProviderSources({experiment,modules,sourceCommit});
      for(const row of restored.revision.providers)assert.equal(hash(restored.sources.get(row.module)),row.sourceSha256);
    }
    for(const changed of modules)assert.throws(()=>readReviewedOverlayProviderSources({experiment,modules,
      readSource:module=>actual.sources.get(module)+(module===changed?'\n':'')}),/no unique reviewed revision/u,changed);
    assert.throws(()=>restoreReviewedOverlayProviderSources({experiment,modules,sourceCommit:'0'.repeat(40)}),/one reviewed historical/u);
  }
  const original=restoreReviewedOverlayProviderPolicy(reviewedOverlayProviderPolicy);
  assert.equal(serializeProjectData(original,packet.historicalOutputs[0].file),packet.historicalOutputs[0].source);
});

test('both current overlay receipts retain every old decorator action and exact committed historical bytes',()=>{
  const packet=readReviewedOverlayProviderSuccession();
  for(const[index,author]of [rustTextCertificateOverlayRecords,rawBindingCertificateOverlayRecords].entries()){
    const row=packet.historicalOutputs[index+1],actual=read(row.file).toString('utf8');
    assert.equal(actual,serializeProjectData(author(),row.file));
    assert.equal(restoreReviewedOverlayProviderReceiptSource(actual,row.file),row.source);
    assert.equal(restoreReviewedOverlayProviderReceiptSource(row.source,row.file),row.source);
    assert.throws(()=>restoreReviewedOverlayProviderReceiptSource(actual+'\n',row.file),assert.AssertionError);
    const historical=author({sourceCommit:packet.previousSourceCommit});
    assert.equal(serializeProjectData(historical,row.file),row.source);
  }
});

test('coherent profile source mutations forged Git memberships inverse deletion and duplicate profiles refuse',()=>{
  const mutations=[
    packet=>{packet.profiles.reverse();},packet=>{packet.profiles[0].sourceCommit='0'.repeat(40);},
    packet=>{packet.profiles[1].previousSourceCommit=packet.previousSourceCommit;},
    packet=>{packet.profiles[0].certificates[1].current.certificate.path='unregistered.js';},
    packet=>{packet.profiles[1].certificates[3].current.source+='\n';packet.profiles[1].providers[3].sourceSha256=hash(packet.profiles[1].certificates[3].current.source);},
    packet=>{packet.profiles[0].inverse=[];},packet=>{packet.profiles[1].inverse[0].to+='\n';},
    packet=>{packet.profiles.push(structuredClone(packet.profiles[1]));},
    packet=>{packet.historicalOutputs[1].source+='\n';packet.historicalOutputs[1].bytes+=1;packet.historicalOutputs[1].sha256=hash(packet.historicalOutputs[1].source);},
    packet=>{packet.host.source+='\n';packet.host.bytes+=1;packet.host.sha256=hash(packet.host.source);},
  ];
  for(const[index,mutate]of mutations.entries()){
    const packet=readReviewedOverlayProviderSuccession();mutate(packet);
    assert.throws(()=>verifyReviewedOverlayProviderSuccession(packet),assert.AssertionError,'tamper '+index);
  }
});

test('both actual isolated model overlays load using the updated exact current provider receipts',async()=>{
  for(const create of [createRustTextCertificatePublicOverlay,createRustRawBindingCertificateOverlay]){
    const overlay=await create();
    try{
      assert.equal(overlay.receipts.length,5);
      assert.equal(typeof overlay.api.createTypedProgramLinks,'function');
      assert.equal(typeof overlay.javascript.emitJavaScript,'function');
      assert.equal(typeof overlay.validation.validateLinkedTypedProgram,'function');
    }finally{overlay.dispose();}
  }
});

test('normal source review refreshes only current receipts and retains both immutable original overlays',async()=>{
  const paths=reviewedOverlayProviderPolicy.historicalOverlays.map(row=>'experiments/'+row.experiment+'/data/overlays.lino');
  const before=paths.map(read);
  const packet=await generateReviewedOverlayProviderSuccession({check:true});
  assert.deepEqual(packet,readReviewedOverlayProviderSuccession());
  paths.forEach((path,index)=>assert.deepEqual(read(path),before[index]));
  assert.deepEqual(readProjectData(new URL('experiments/rust-text-certificate-adapter/data/provider-revisions.lino',root)),reviewedOverlayProviderPolicy);
});


test('a real appended source epoch preserves the complete original packet policy receipts and exact only-emitter source inverse',async()=>{
  const packet=readReviewedOverlayProviderSuccession();assert.equal(packet.profiles.slice(0,3).length,3);
  const current=packet.profiles[2],review=OVERLAY_PROVIDER_APPEND_REVIEW;
  assert.equal(current.previousSourceCommit,packet.profiles[1].sourceCommit);
  assert.equal(current.epoch.reviewKind,review.kind);assert.equal(current.review,review.review);
  assert.deepEqual(current.inverse.map(row=>row.module),['translation/emit-rust.js']);
  const prefix={...packet,profiles:packet.profiles.slice(0,2)},source=serializeProjectData(prefix,'parity/self-translation/reviewed-overlay-provider-succession.lino');
  assert.deepEqual({bytes:Buffer.byteLength(source),sha256:hash(source)},OVERLAY_PROVIDER_ORIGINAL_PACKET,
    'dropping only the verified suffix reconstructs every original packet byte');
  assert.equal(Object.hasOwn(current.epoch.predecessorPacket,'source'),false,'no duplicate whole packet snapshot is stored');
  assert.deepEqual(deriveReviewedOverlayProviderPolicy(prefix).revisions.slice(-2),prefix.profiles);
  verifyReviewedOverlayProviderSuccessionHistory(prefix);
  for(const row of current.certificates){
    assert.equal(restoreReviewedOverlayProviderModuleSource(read('js/src/'+row.module).toString('utf8'),'js/src/'+row.module,current.sourceCommit),row.current.source);
    const restored=restoreReviewedOverlayProviderModuleSource(row.current.source,'js/src/'+row.module);
    const original=packet.profiles[0].certificates.find(value=>value.module===row.module);
    assert.equal(restored,original.current.source,'the explicit historical module view reaches exact8d');
    assert.equal(restoreReviewedOverlayProviderModuleSource(row.current.source+'\n','js/src/'+row.module),row.current.source+'\n',
      'unknown source remains visible to its original historical guard');
  }
  const changed=current.certificates.find(row=>row.module===review.module);
  assert.equal(hash(changed.previous.source),review.previousSha256);
  assert.equal(hash(changed.current.source),review.currentSha256);assert.equal(Buffer.byteLength(changed.current.source),review.currentBytes);
  for(const mutate of [
    value=>{value.profiles[2].epoch.predecessorPacket.sha256='0'.repeat(64);},
    value=>{value.profiles[2].epoch.predecessorPacket.certificate.path='unregistered.lino';},
    value=>{value.profiles[2].epoch.predecessorPacket.source='invented copied history';},
    value=>{value.profiles[2].epoch.predecessorOutputs[1].sha256='0'.repeat(64);},
    value=>{value.profiles[2].epoch.predecessorOutputs.reverse();},
    value=>{value.profiles[2].epoch.reviewKind='arbitrary provider changes';},
    value=>{value.profiles[2].review+=' unreviewed';},
    value=>{value.profiles[2].inverse[0].to+='\n';},
    value=>{value.profiles[2].certificates[0].current.source+='\n';},
    value=>{value.profiles.push(structuredClone(value.profiles[2]));},
  ]){const altered=structuredClone(packet);mutate(altered);assert.throws(()=>verifyReviewedOverlayProviderSuccession(altered),assert.AssertionError);}
  await assert.rejects(()=>generateReviewedOverlayProviderSuccession({check:true,revision:current.sourceCommit}),assert.AssertionError,
    'check-only authoring cannot append or invoke source capture');
});


test('the reference and identifier provider epoch retains every preceding packet and all three complete source inverses',()=>{
  const packet=readReviewedOverlayProviderSuccession();assert.equal(packet.profiles.length,4);
  const profile=packet.profiles[3],review=OVERLAY_PROVIDER_REFERENCE_REVIEW;
  assert.equal(profile.sourceCommit,'8bc0ef9725388c7927acf1854870bf46f5e365be');
  assert.equal(profile.previousSourceCommit,packet.profiles[2].sourceCommit);
  assert.equal(profile.epoch.reviewKind,review.kind);assert.equal(profile.review,review.review);
  assert.deepEqual(profile.inverse.map(row=>row.module),review.modules.map(row=>row.module));
  const before={...packet,profiles:packet.profiles.slice(0,3)};
  assert.equal(hash(serializeProjectData(before,'parity/self-translation/reviewed-overlay-provider-succession.lino')),profile.epoch.predecessorPacket.sha256);
  verifyReviewedOverlayProviderSuccessionHistory(before);
  for(const owner of review.modules){
    const row=profile.certificates.find(row=>row.module===owner.module);assert.ok(row);
    assert.equal(hash(row.previous.source),owner.previousSha256);assert.equal(hash(row.current.source),owner.currentSha256);
    assert.equal(Buffer.byteLength(row.current.source),owner.currentBytes);
    assert.equal(read('js/src/'+row.module).toString('utf8'),row.current.source);
    assert.equal(restoreReviewedOverlayProviderModuleSource(row.current.source,'js/src/'+row.module,profile.previousSourceCommit),row.previous.source);
  }
  for(const change of [
    value=>{value.profiles[3].inverse.pop();},
    value=>{value.profiles[3].epoch.predecessorPacket.sha256='0'.repeat(64);},
    value=>{value.profiles[3].epoch.predecessorOutputs[0].certificate.path='unregistered.lino';},
    value=>{value.profiles[3].certificates[1].current.source+=' ';},
    value=>{value.profiles[3].review+=' changed';},
  ]){const changed=structuredClone(packet);change(changed);assert.throws(()=>verifyReviewedOverlayProviderSuccession(changed));}
  assert.throws(()=>restoreReviewedOverlayProviderModuleSource(profile.certificates[0].current.source,'js/src/typed-program-links.js','0'.repeat(40)));
});
