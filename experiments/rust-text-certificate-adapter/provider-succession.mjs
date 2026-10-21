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
const packetSha256='73383f9c3f5803df5dea0cde16dfccc0037c6c29f8908478d699d8f3ca9317b6';
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

export function verifyReviewedOverlayProviderSuccession(packet){
  exactKeys(packet,['schemaVersion','model','previousSourceCommit','profiles','historicalOutputs','host','boundary']);
  assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'reviewed complete overlay provider succession 1');
  assert.equal(packet.previousSourceCommit,OVERLAY_PROVIDER_PREVIOUS_REVISION);
  assert.deepEqual(packet.profiles.map(row=>row.sourceCommit),OVERLAY_PROVIDER_SUCCESSOR_REVISIONS);
  let previous=OVERLAY_PROVIDER_PREVIOUS_REVISION;
  let modules;
  for(const[index,profile]of packet.profiles.entries()){
    exactKeys(profile,['sourceCommit','previousSourceCommit','review','providers','inverse','certificates']);
    assert.equal(profile.previousSourceCommit,previous);assert.ok(profile.review.length>0);
    const inventory=profile.providers.map(row=>row.module);
    if(modules===undefined)modules=inventory;else assert.deepEqual(inventory,modules);
    assert.deepEqual(inventory,['typed-program-links.js','typed-program-links-validation.js','typed-program-checked-projection.js','translation/emit-javascript.js','translation/emit-rust.js']);
    assert.equal(profile.certificates.length,5);
    for(const[owner,row]of profile.certificates.entries()){
      assert.equal(row.module,modules[owner]);
      for(const side of ['current','previous']){
        const source=row[side],revision=side==='current'?profile.sourceCommit:profile.previousSourceCommit;
        assert.equal(source.revision,revision);
        verifyGitArtifactCertificate(source.certificate,Buffer.from(source.source),{revision,path:'js/src/'+row.module});
      }
      assert.equal(hash(row.current.source),profile.providers[owner].sourceSha256);
      if(index>0)assert.equal(hash(row.previous.source),packet.profiles[index-1].providers[owner].sourceSha256);
      const actions=profile.inverse.filter(action=>action.module===row.module);
      assert.deepEqual(actions,row.current.source===row.previous.source?[]:[{module:row.module,from:row.current.source,to:row.previous.source}]);
      if(index===packet.profiles.length-1)assert.equal(row.current.source,readFileSync(new URL('js/src/'+row.module,root),'utf8'),'every actual complete current provider belongs to the real checkpoint');
    }
    assert.deepEqual(profile.inverse.map(row=>row.module),index===0?['typed-program-links-validation.js']:['translation/emit-javascript.js']);
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

export function readReviewedOverlayProviderSuccession(){
  if(!cached){const bytes=readFileSync(new URL(OVERLAY_PROVIDER_SUCCESSOR_OUTPUT,root));assert.equal(hash(bytes),packetSha256);cached=verifyReviewedOverlayProviderSuccession(readProjectData(new URL(OVERLAY_PROVIDER_SUCCESSOR_OUTPUT,root)));}
  return structuredClone(cached);
}

export function readReviewedOverlayProviderSuccessors(previous){
  const packet=readReviewedOverlayProviderSuccession();
  assert.equal(previous.sourceCommit,packet.previousSourceCommit);
  assert.deepEqual(previous.providers,packet.profiles[0].certificates.map(row=>({module:row.module,sourceSha256:hash(row.previous.source)})));
  return packet.profiles;
}

/** Remove only the two exact appended rows; all original policy fields remain. */
export function restoreReviewedOverlayProviderPolicy(policy){
  const packet=readReviewedOverlayProviderSuccession(),current=structuredClone(policy);
  assert.deepEqual(current.revisions.slice(-2),packet.profiles);
  current.revisions.splice(-2);
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
  const row=packet.profiles[1].certificates.find(row=>row.module===module);
  return row&&source===row.current.source?row.previous.source:source;
}
