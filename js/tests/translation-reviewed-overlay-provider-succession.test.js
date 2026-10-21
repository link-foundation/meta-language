import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData,serializeProjectData} from '../scripts/project-data.mjs';
import {reviewedOverlayProviderPolicy,readReviewedOverlayProviderSources,restoreReviewedOverlayProviderSources}
  from '../../experiments/rust-text-certificate-adapter/reviewed-providers.mjs';
import {readReviewedOverlayProviderSuccession,verifyReviewedOverlayProviderSuccession,
  restoreReviewedOverlayProviderPolicy,restoreReviewedOverlayProviderReceiptSource}
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
  assert.equal(current.sourceCommit,'8ef026a395ebbbc415f948777f7f6f7e082a572e');
  assert.deepEqual(packet.profiles.map(row=>row.inverse.map(action=>action.module)),[['typed-program-links-validation.js'],['translation/emit-javascript.js']]);
  for(const experiment of ['rust-text-certificate-adapter','rust-raw-binding-certificate-adapter']){
    const actual=readReviewedOverlayProviderSources({experiment,modules});assert.equal(actual.revision,current);
    for(const row of current.providers)assert.equal(hash(read('js/src/'+row.module)),row.sourceSha256);
    for(const sourceCommit of ['8d77e476d097448eb381ebe03e2aa173402bc03d',packet.previousSourceCommit]){
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
