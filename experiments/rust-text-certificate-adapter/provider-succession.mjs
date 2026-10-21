// Append complete certified provider profiles; never replace a previous revision.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData,serializeProjectData} from '../../js/scripts/project-data.mjs';
import {verifyGitArtifactCertificate} from '../../js/scripts/git-object-certificate.mjs';
import {readStructuredDataLino} from '../../js/src/structured-data-links.js';
import {DecoratorSet} from '../../js/src/decorators.js';

const root=new URL('../../',import.meta.url);
export const OVERLAY_PROVIDER_PREVIOUS_REVISION='478fb5873ddfd6ab7e4c4b6849654e2c2cef3fa8';
export const OVERLAY_PROVIDER_SUCCESSOR_REVISIONS=Object.freeze([
  '8d77e476d097448eb381ebe03e2aa173402bc03d','8ef026a395ebbbc415f948777f7f6f7e082a572e',
]);
export const OVERLAY_PROVIDER_SUCCESSOR_OUTPUT='parity/self-translation/reviewed-overlay-provider-succession.lino';
export const OVERLAY_PROVIDER_HOST='experiments/rust-text-certificate-adapter/reviewed-providers.mjs';
export const OVERLAY_PROVIDER_HOST_EDITS=Object.freeze([
  Object.freeze({from:"import {readProgramStringOverlayProviderRevision} from '../grammar-context-source-metadata/overlay-provider-revision.mjs';\n",
    to:"import {readProgramStringOverlayProviderRevision} from '../grammar-context-source-metadata/overlay-provider-revision.mjs';\nimport {readReviewedOverlayProviderSuccessors} from './provider-succession.mjs';\n"}),
  Object.freeze({from:'reviewedOverlayProviderPolicy.revisions.push(readProgramStringOverlayProviderRevision(reviewedOverlayProviderPolicy.revisions.at(-1)));\n',
    to:'reviewedOverlayProviderPolicy.revisions.push(readProgramStringOverlayProviderRevision(reviewedOverlayProviderPolicy.revisions.at(-1)));\nreviewedOverlayProviderPolicy.revisions.push(...readReviewedOverlayProviderSuccessors(reviewedOverlayProviderPolicy.revisions.at(-1)));\n'}),
]);
export const OVERLAY_PROVIDER_ORIGINAL_PACKET=Object.freeze({bytes:5552382,sha256:'73383f9c3f5803df5dea0cde16dfccc0037c6c29f8908478d699d8f3ca9317b6'});
export const OVERLAY_PROVIDER_APPEND_REVIEW=Object.freeze({
 kind:'optional Rust definition ownership metadata 1',
 previous:'8ef026a395ebbbc415f948777f7f6f7e082a572e',
 module:'translation/emit-rust.js',previousSha256:'7d8f61a0de6af13f0061fcd7efe75af746909fbd050d9e281e0db12014754bd0',
 currentBytes:79595,currentSha256:'e4d5f0f5b4a5f51d24dd240ebb35ae0c0231c66ac7b7b3bc2b72f51906bf7bcb',
 review:'Reviewed the complete Rust emitter successor against8ef026a395ebbbc415f948777f7f6f7e082a572e: optional nonserialized definition-ownership capture records actual complete emitted blocks/source spans/mapping intervals/targets, and a thin external-target reader uses the actual normal emitter naming path. Default output, complete original function bodies, source mappings and exact decorator actions retain their existing independent tests. Only this complete79595-byte source is admitted by this review; its full source inverse restores every predecessor byte. This is source succession and review, not semantic equivalence, native compilation or execution evidence.',
});
const packetSha256='3808a81f940b0544466da91c084ac74842ff6bc4bfd4aae32fd61070904f43e2';
const hash=value=>createHash('sha256').update(value).digest('hex');
const pin=value=>({bytes:Buffer.byteLength(value),sha256:hash(value)});
const exactKeys=(value,keys)=>assert.deepEqual(Object.keys(value).sort(),[...keys].sort());
let cached;

export function replayOverlayProviderHost(source){
  let current=source;
  for(const{from,to}of OVERLAY_PROVIDER_HOST_EDITS){assert.equal(current.split(from).length,2);current=current.replace(from,()=>to);}
  let inverse=current;
  for(const{from,to}of OVERLAY_PROVIDER_HOST_EDITS.toReversed()){assert.equal(inverse.split(to).length,2);inverse=inverse.replace(to,()=>from);}
  assert.equal(inverse,source);return current;
}

/** Replay the EXISTING recorded exact-source recipes, retaining every action.
 * The normal model authors independently regenerate these same current rows. */
export function deriveReviewedOverlayProviderReceipts(packet){
  const profile=packet.profiles.at(-1),sources=new Map(profile.certificates.map(row=>[row.module,row.current.source]));
  return packet.historicalOutputs.slice(1).map(original=>{
    const records=readStructuredDataLino(original.source);
    assert.deepEqual(records.map(row=>row.module),profile.providers.map(row=>row.module));
    const current=records.map(row=>{
      assert.equal(row.providerRevision,OVERLAY_PROVIDER_PREVIOUS_REVISION);
      const source=sources.get(row.module);assert.equal(typeof source,'string');
      const decorated=DecoratorSet.fromLino(row.decorators).decorate('translation-rule',{module:row.module,source}).source;
      return{...row,sourceSha256:hash(source),overlaySha256:hash(decorated),providerRevision:profile.sourceCommit};
    });
    const source=serializeProjectData(current,original.file);
    return{file:original.file,...pin(source),source};
  });
}

export function verifyReviewedOverlayProviderSuccessionHistory(packet){
  exactKeys(packet,['schemaVersion','model','previousSourceCommit','profiles','historicalOutputs','host','boundary']);
  assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'reviewed complete overlay provider succession 1');
  assert.equal(packet.previousSourceCommit,OVERLAY_PROVIDER_PREVIOUS_REVISION);
  assert.ok(packet.profiles.length>=OVERLAY_PROVIDER_SUCCESSOR_REVISIONS.length);
  assert.deepEqual(packet.profiles.slice(0,2).map(row=>row.sourceCommit),OVERLAY_PROVIDER_SUCCESSOR_REVISIONS);
  const original={...packet,profiles:packet.profiles.slice(0,2)};
  assert.deepEqual(pin(serializeProjectData(original,OVERLAY_PROVIDER_SUCCESSOR_OUTPUT)),OVERLAY_PROVIDER_ORIGINAL_PACKET,
    'every byte of the complete original two-profile packet remains reconstructible');
  let previous=OVERLAY_PROVIDER_PREVIOUS_REVISION;
  let modules;
  for(const[index,profile]of packet.profiles.entries()){
    exactKeys(profile,['sourceCommit','previousSourceCommit','review','providers','inverse','certificates',...(index>=2?['epoch']:[])]);
    assert.match(profile.sourceCommit,/^[a-f0-9]{40}$/u);
    assert.equal(packet.profiles.filter(row=>row.sourceCommit===profile.sourceCommit).length,1);
    assert.equal(profile.previousSourceCommit,previous);assert.ok(profile.review.length>0);
    const inventory=profile.providers.map(row=>row.module);
    if(modules===undefined)modules=inventory;else assert.deepEqual(inventory,modules);
    assert.deepEqual(inventory,['typed-program-links.js','typed-program-links-validation.js','typed-program-checked-projection.js','translation/emit-javascript.js','translation/emit-rust.js']);
    assert.equal(profile.certificates.length,5);
    for(const[owner,row]of profile.certificates.entries()){
      exactKeys(row,['module','current','previous']);exactKeys(profile.providers[owner],['module','sourceSha256']);
      assert.equal(row.module,modules[owner]);
      for(const side of ['current','previous']){
        const source=row[side],revision=side==='current'?profile.sourceCommit:profile.previousSourceCommit;
        exactKeys(source,['revision','source','certificate']);
        assert.equal(source.revision,revision);
        verifyGitArtifactCertificate(source.certificate,Buffer.from(source.source),{revision,path:'js/src/'+row.module});
      }
      assert.equal(hash(row.current.source),profile.providers[owner].sourceSha256);
      if(index>0)assert.equal(hash(row.previous.source),packet.profiles[index-1].providers[owner].sourceSha256);
      const actions=profile.inverse.filter(action=>action.module===row.module);
      assert.deepEqual(actions,row.current.source===row.previous.source?[]:[{module:row.module,from:row.current.source,to:row.previous.source}]);

    }
    if(index<2)assert.deepEqual(profile.inverse.map(row=>row.module),index===0?['typed-program-links-validation.js']:['translation/emit-javascript.js']);
    else verifyReviewedOverlayProviderEpoch(packet,index);
    previous=profile.sourceCommit;
  }
  assert.deepEqual(packet.historicalOutputs.map(row=>row.file),[
    'experiments/rust-text-certificate-adapter/data/provider-revisions.lino',
    'experiments/rust-text-certificate-adapter/data/overlays-current.lino',
    'experiments/rust-raw-binding-certificate-adapter/data/overlays-current.lino',
  ]);
  for(const row of [...packet.historicalOutputs,packet.host]){
    assert.equal(row.revision,OVERLAY_PROVIDER_SUCCESSOR_REVISIONS[0]);
    assert.deepEqual(pin(row.source),{bytes:row.bytes,sha256:row.sha256});
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.source),{revision:row.revision,path:row.file});
  }
  assert.equal(packet.host.file,OVERLAY_PROVIDER_HOST);
  assert.equal(readFileSync(new URL(OVERLAY_PROVIDER_HOST,root),'utf8'),replayOverlayProviderHost(packet.host.source));
  return packet;
}

export function readReviewedOverlayProviderSuccessionHistory(){
  if(!cached){const bytes=readFileSync(new URL(OVERLAY_PROVIDER_SUCCESSOR_OUTPUT,root));assert.equal(hash(bytes),packetSha256);cached=verifyReviewedOverlayProviderSuccessionHistory(readProjectData(new URL(OVERLAY_PROVIDER_SUCCESSOR_OUTPUT,root)));}
  return structuredClone(cached);
}
export function verifyCurrentReviewedOverlayProviderSources(packet){
  for(const row of packet.profiles.at(-1).certificates)assert.equal(row.current.source,readFileSync(new URL('js/src/'+row.module,root),'utf8'),
    'every actual complete current provider belongs to the real checkpoint');
  return packet;
}
export function verifyReviewedOverlayProviderSuccession(packet){
  return verifyCurrentReviewedOverlayProviderSources(verifyReviewedOverlayProviderSuccessionHistory(packet));
}
export function readReviewedOverlayProviderSuccession(){
  return verifyCurrentReviewedOverlayProviderSources(readReviewedOverlayProviderSuccessionHistory());
}

export function readReviewedOverlayProviderSuccessors(previous){
  const packet=readReviewedOverlayProviderSuccession();
  assert.equal(previous.sourceCommit,packet.previousSourceCommit);
  assert.deepEqual(previous.providers,packet.profiles[0].certificates.map(row=>({module:row.module,sourceSha256:hash(row.previous.source)})));
  return packet.profiles;
}

/** Remove only the complete verified appended suffix; every old policy byte remains. */
export function restoreReviewedOverlayProviderPolicy(policy){
  const packet=readReviewedOverlayProviderSuccession(),current=structuredClone(policy);
  assert.deepEqual(current.revisions.slice(-packet.profiles.length),packet.profiles);
  current.revisions.splice(-packet.profiles.length);
  const original=packet.historicalOutputs[0];
  assert.equal(serializeProjectData(current,original.file),original.source,'every old revision and policy byte is retained');
  return current;
}

/** Exact current recipes authenticate receipt bytes before exposing the old
 * committed478 receipt to its original historical control. Unknown bytes refuse. */
export function restoreReviewedOverlayProviderReceiptSource(source,file){
  const packet=readReviewedOverlayProviderSuccession(),row=packet.historicalOutputs.find(row=>row.file===file);
  if(!row||file.endsWith('/provider-revisions.lino'))return source;
  if(source===row.source)return source;
  const current=deriveReviewedOverlayProviderReceipts(packet).find(row=>row.file===file);assert.ok(current);
  assert.equal(source,current.source,'an unknown current overlay receipt cannot acquire historical identity');
  return row.source;
}

export function restoreReviewedOverlayProviderHostSource(source,file){
  if(file!==OVERLAY_PROVIDER_HOST)return source;
  const row=readReviewedOverlayProviderSuccession().host;
  return source===replayOverlayProviderHost(row.source)?row.source:source;
}

/** Only an explicitly historical caller uses this exact checkpoint→8d view. */
export function restoreReviewedOverlayProviderModuleSource(source,file){
  if(!file.startsWith('js/src/'))return source;
  const module=file.slice('js/src/'.length),packet=readReviewedOverlayProviderSuccession();
  let historical=source;
  for(const profile of packet.profiles.slice(1).toReversed()){
    const row=profile.certificates.find(row=>row.module===module);
    if(row&&historical===row.current.source)historical=row.previous.source;
  }
  return historical;
}


export function deriveReviewedOverlayProviderPolicy(packet){
  const original=packet.historicalOutputs[0],policy=readStructuredDataLino(original.source);
  assert.equal(policy.revisions.at(-1).sourceCommit,packet.previousSourceCommit);
  policy.revisions.push(...packet.profiles);return policy;
}

function verifyReviewedOverlayProviderEpoch(packet,index){
  const profile=packet.profiles[index],epoch=profile.epoch,review=OVERLAY_PROVIDER_APPEND_REVIEW;
  exactKeys(epoch,['reviewKind','predecessorPacket','predecessorOutputs']);
  assert.equal(epoch.reviewKind,review.kind);assert.equal(profile.previousSourceCommit,review.previous);
  assert.equal(profile.review,review.review);assert.deepEqual(profile.inverse.map(row=>row.module),[review.module]);
  const changed=profile.certificates.find(row=>row.module===review.module);assert.ok(changed);
  assert.equal(hash(changed.previous.source),review.previousSha256);
  assert.deepEqual(pin(changed.current.source),{bytes:review.currentBytes,sha256:review.currentSha256});
  const prefix={...packet,profiles:packet.profiles.slice(0,index)};
  const oldPacket=serializeProjectData(prefix,OVERLAY_PROVIDER_SUCCESSOR_OUTPUT);
  const before=epoch.predecessorPacket;
  exactKeys(before,['revision','file','bytes','sha256','certificate']);
  assert.equal(before.revision,profile.sourceCommit);assert.equal(before.file,OVERLAY_PROVIDER_SUCCESSOR_OUTPUT);
  assert.deepEqual(pin(oldPacket),{bytes:before.bytes,sha256:before.sha256});
  verifyGitArtifactCertificate(before.certificate,Buffer.from(oldPacket),{revision:profile.sourceCommit,path:before.file});
  const outputs=[{file:packet.historicalOutputs[0].file,source:serializeProjectData(deriveReviewedOverlayProviderPolicy(prefix),packet.historicalOutputs[0].file)},
    ...deriveReviewedOverlayProviderReceipts(prefix)];
  assert.deepEqual(epoch.predecessorOutputs.map(row=>row.file),outputs.map(row=>row.file));
  for(const[owner,row]of epoch.predecessorOutputs.entries()){
    exactKeys(row,['revision','file','bytes','sha256','certificate']);assert.equal(row.revision,profile.sourceCommit);
    assert.deepEqual(pin(outputs[owner].source),{bytes:row.bytes,sha256:row.sha256});
    verifyGitArtifactCertificate(row.certificate,Buffer.from(outputs[owner].source),{revision:profile.sourceCommit,path:row.file});
  }
}
