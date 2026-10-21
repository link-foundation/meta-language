// Normal, finite source review author. All source identities are real commits.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync,readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from '../../js/scripts/capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from '../../js/scripts/git-object-certificate.mjs';
import {readStructuredDataLino} from '../../js/src/structured-data-links.js';
import {checkProjectData,writeProjectData,serializeProjectData} from '../../js/scripts/project-data.mjs';
import {OVERLAY_PROVIDER_APPEND_REVIEW,readReviewedOverlayProviderSuccessionHistory,
 verifyReviewedOverlayProviderSuccessionHistory,verifyCurrentReviewedOverlayProviderSources,
 deriveReviewedOverlayProviderPolicy,deriveReviewedOverlayProviderReceipts} from './provider-succession.mjs';
const root=new URL('../../',import.meta.url);
const previous='478fb5873ddfd6ab7e4c4b6849654e2c2cef3fa8';
const revisions=['8d77e476d097448eb381ebe03e2aa173402bc03d','8ef026a395ebbbc415f948777f7f6f7e082a572e'];
const modules=['typed-program-links.js','typed-program-links-validation.js','typed-program-checked-projection.js','translation/emit-javascript.js','translation/emit-rust.js'];
const currentHashes=['a0f46c4fdca3ed9bb729da7a701266bf8d7609ba142bc3009cd6eafe39983458','2552d5ef78b53aac005f3a07a8a0c28aede6cd40d2002f229f27fc0a431d2269','5d224a630dbb3d55582648a888482144331e2bad31d67a2fe05d4cb34ca601da','29fb06e7ab31ad36c4d7f33c9a612ab75350c1734cbc31184f7aee3cdcc82df5','7d8f61a0de6af13f0061fcd7efe75af746909fbd050d9e281e0db12014754bd0'];
const output='parity/self-translation/reviewed-overlay-provider-succession.lino';
const hostFile='experiments/rust-text-certificate-adapter/reviewed-providers.mjs';
const helperFile='experiments/rust-text-certificate-adapter/provider-succession.mjs';
const policyFile='experiments/rust-text-certificate-adapter/data/provider-revisions.lino';
const receipts=['experiments/rust-text-certificate-adapter/data/overlays-current.lino','experiments/rust-raw-binding-certificate-adapter/data/overlays-current.lino'];
const hash=value=>createHash('sha256').update(value).digest('hex');
const pin=value=>({bytes:Buffer.byteLength(value),sha256:hash(value)});
const hostEdits=[
  {from:"import {readProgramStringOverlayProviderRevision} from '../grammar-context-source-metadata/overlay-provider-revision.mjs';\n",to:"import {readProgramStringOverlayProviderRevision} from '../grammar-context-source-metadata/overlay-provider-revision.mjs';\nimport {readReviewedOverlayProviderSuccessors} from './provider-succession.mjs';\n"},
  {from:'reviewedOverlayProviderPolicy.revisions.push(readProgramStringOverlayProviderRevision(reviewedOverlayProviderPolicy.revisions.at(-1)));\n',to:'reviewedOverlayProviderPolicy.revisions.push(readProgramStringOverlayProviderRevision(reviewedOverlayProviderPolicy.revisions.at(-1)));\nreviewedOverlayProviderPolicy.revisions.push(...readReviewedOverlayProviderSuccessors(reviewedOverlayProviderPolicy.revisions.at(-1)));\n'},
];

export async function generateReviewedOverlayProviderSuccession({check=false,revision}={}){
  if(existsSync(new URL(output,root)))return updateReviewedOverlayProviderSuccession({check,revision});
  assert.equal(revision,undefined,'an append requires the complete original normal-author packet');
  const cache=new Map();
  const capture=(revision,file)=>{
    const key=revision+':'+file;
    if(!cache.has(key)){
      const{blob,certificate}=captureGitArtifactCertificate(revision,file);
      assert.deepEqual(Buffer.from(blob.toString('utf8')),blob);
      verifyGitArtifactCertificate(certificate,blob,{revision,path:file});
      cache.set(key,{revision,file,...pin(blob),source:blob.toString('utf8'),certificate});
    }
    return cache.get(key);
  };
  let predecessor=previous;
  const reviews=[
    'Reviewed the complete five-provider diff at8d77e476 against478fb587. Only typed-program-links-validation.js changes: its existing closed expression-field rule is applied to every actually registered operation, retaining the original tuple-specific refusal text and adding a general semantic-field refusal. The exact133-byte source delta and complete whole-module inverse are certified; every other provider byte is unchanged. All original certificate/raw-owner/descriptor/saved-LinkId hooks and every historical overlay stay required. This is source review and admitted-domain validation, not semantic-equivalence or native execution evidence.',
    'Reviewed all five complete providers at8ef026a395ebbbc415f948777f7f6f7e082a572e against8d77e476. Only translation/emit-javascript.js changes: actual checked external owners reserve target names, the same shared normally generated binding/call decisions render those existing calls and function references, and ordinary declarations keep their old path. An exact964-byte diff and complete inverse restore every prior provider byte. Old overlays/decorator actions and all ownership/descriptor/certificate/LinkId gates remain unchanged. Current standalone/linked/native behavior still requires its independent tests; source review is no runtime proof.',
  ];
  const profiles=revisions.map((revision,index)=>{
    const providers=[],inverse=[],certificates=[];
    for(const[owner,module]of modules.entries()){
      const file='js/src/'+module,old=capture(predecessor,file),current=capture(revision,file);
      if(index===1){assert.equal(current.sha256,currentHashes[owner],'the complete reviewed current provider');assert.equal(readFileSync(new URL(file,root),'utf8'),current.source,'current bytes must belong to the real provider checkpoint');}
      providers.push({module,sourceSha256:current.sha256});
      certificates.push({module,current:{revision,source:current.source,certificate:current.certificate},previous:{revision:predecessor,source:old.source,certificate:old.certificate}});
      if(current.source!==old.source)inverse.push({module,from:current.source,to:old.source});
    }
    assert.deepEqual(inverse.map(row=>row.module),index===0?['typed-program-links-validation.js']:['translation/emit-javascript.js']);
    const profile={sourceCommit:revision,previousSourceCommit:predecessor,review:reviews[index],providers,inverse,certificates};
    predecessor=revision;return profile;
  });
  const historicalOutputs=[policyFile,...receipts].map(file=>capture(revisions[0],file));
  const host=capture(revisions[0],hostFile);
  assert.deepEqual(pin(host.source),{bytes:44535,sha256:'b7ba37681c9c057163884d5b73e51bb1a918bafe542bf7cf7450fdfb5d2aa23d'});
  const packet={schemaVersion:1,model:'reviewed complete overlay provider succession 1',previousSourceCommit:previous,profiles,historicalOutputs,host,
    boundary:'Append two independently committed complete source profiles and retain every original policy, overlay document, decorator action, source body and receipt byte. Current source identities and complete inverses establish source succession only; changed validation and external-emission behavior retain separate tests and CI execution requirements. Unknown providers remain refused by the unchanged exact five-hash selector.'};
  let registered=host.source;
  for(const{from,to}of hostEdits){assert.equal(registered.split(from).length,2);registered=registered.replace(from,()=>to);}
  let inverse=registered;
  for(const{from,to}of hostEdits.toReversed()){assert.equal(inverse.split(to).length,2);inverse=inverse.replace(to,()=>from);}
  assert.equal(inverse,host.source);
  const hostPath=new URL(hostFile,root),actualHost=readFileSync(hostPath,'utf8');
  assert.ok(actualHost===host.source||actualHost===registered,'unknown provider reader source is never overwritten');
  if(check)assert.equal(actualHost,registered);
  // Bootstrap the packet and hash before registering it. No producer imports
  // the reader until its complete certified input exists.
  const packetPath=new URL(output,root);
  if(check)checkProjectData(packetPath,packet);else writeProjectData(packetPath,packet);
  const helperPath=new URL(helperFile,root),helper=readFileSync(helperPath,'utf8'),pattern=/const packetSha256='(?:PENDING|[a-f0-9]{64})';/u;
  assert.equal(helper.split(pattern).length,2);
  const expectedHelper=helper.replace(pattern,"const packetSha256='"+hash(readFileSync(packetPath))+"';");
  if(check)assert.equal(helper,expectedHelper);else{writeFileSync(helperPath,expectedHelper);writeFileSync(hostPath,registered);}
  const{reviewedOverlayProviderPolicy}=await import('./reviewed-providers.mjs');
  const{verifyReviewedOverlayProviderSuccession}=await import('./provider-succession.mjs');
  verifyReviewedOverlayProviderSuccession(packet);
  const originalPolicy=readStructuredDataLino(historicalOutputs[0].source),expectedPolicy=structuredClone(originalPolicy);
  assert.equal(expectedPolicy.revisions.at(-1).sourceCommit,previous);
  expectedPolicy.revisions.push(...profiles);assert.deepEqual(reviewedOverlayProviderPolicy,expectedPolicy);
  const actualPolicy=readFileSync(new URL(policyFile,root),'utf8');
  assert.ok(actualPolicy===historicalOutputs[0].source||actualPolicy===serializeProjectData(expectedPolicy,policyFile),'unknown provider policy bytes are never overwritten');
  if(check)checkProjectData(new URL(policyFile,root),expectedPolicy);else writeProjectData(new URL(policyFile,root),expectedPolicy);
  const{rustTextCertificateOverlayRecords}=await import('./generate.mjs');
  const{rawBindingCertificateOverlayRecords}=await import('../rust-raw-binding-certificate-adapter/generate-overlay.mjs');
  for(const[index,author]of [rustTextCertificateOverlayRecords,rawBindingCertificateOverlayRecords].entries()){
    const file=receipts[index],old=readStructuredDataLino(historicalOutputs[index+1].source),current=author(),restored=structuredClone(current);
    assert.equal(current.length,5);assert.equal(old.length,5);
    for(const[owner,row]of current.entries()){
      const before=old[owner];assert.equal(row.module,before.module);
      assert.equal(before.providerRevision,previous);assert.equal(row.providerRevision,revisions[1]);
      assert.equal(row.decorators,before.decorators,'every original exact decorator action stays');assert.equal(row.changes,before.changes);
      for(const key of ['sourceSha256','overlaySha256','providerRevision'])restored[owner][key]=before[key];
    }
    assert.deepEqual(restored,old);assert.equal(serializeProjectData(restored,file),historicalOutputs[index+1].source);
    const actual=readFileSync(new URL(file,root),'utf8');
    assert.ok(actual===historicalOutputs[index+1].source||actual===serializeProjectData(current,file),'unknown current overlay bytes are never overwritten');
    if(check)checkProjectData(new URL(file,root),current);else writeProjectData(new URL(file,root),current);
  }
  return packet;
}

// The existing two-profile bootstrap above stays intact. Normal successor
// authoring consumes its pinned complete history and appends a real Git epoch;
// --check reads only stored certificates and never invokes Git.
export async function updateReviewedOverlayProviderSuccession({check=false,revision}={}){
  assert.ok(!check||revision===undefined,'check-only mode cannot append a revision');
  const original=readReviewedOverlayProviderSuccessionHistory();
  const previousBytes=readFileSync(new URL(output,root));
  assert.equal(previousBytes.toString('utf8'),serializeProjectData(original,output),'the complete input packet is canonical');
  let packet=original;
  const outputs=profile=>[{file:policyFile,source:serializeProjectData(deriveReviewedOverlayProviderPolicy(profile),policyFile)},
    ...deriveReviewedOverlayProviderReceipts(profile)];
  if(revision!==undefined){
    assert.match(revision,/^[a-f0-9]{40}$/u,'supply one complete actual committed source epoch');
    assert.equal(original.profiles.some(row=>row.sourceCommit===revision),false,'a committed epoch is appended once');
    const review=OVERLAY_PROVIDER_APPEND_REVIEW,last=original.profiles.at(-1);
    assert.equal(last.sourceCommit,review.previous,'this review applies to the exact preceding provider profile');
    const capture=file=>{
      const{blob,certificate}=captureGitArtifactCertificate(revision,file);
      assert.equal(Buffer.from(blob.toString('utf8')).equals(blob),true);
      verifyGitArtifactCertificate(certificate,blob,{revision,path:file});
      assert.equal(readFileSync(new URL(file,root)).equals(blob),true,'every captured actual file belongs to the real epoch: '+file);
      return{revision,file,...pin(blob),source:blob.toString('utf8'),certificate};
    };
    const preceding=capture(output);assert.equal(preceding.source,previousBytes.toString('utf8'));
    const priorOutputs=outputs(original).map(row=>{const actual=capture(row.file);assert.equal(actual.source,row.source);return actual;});
    const providers=[],certificates=[],inverse=[];
    for(const module of modules){
      const current=capture('js/src/'+module),previous=last.certificates.find(row=>row.module===module).current;
      providers.push({module,sourceSha256:current.sha256});
      certificates.push({module,previous:structuredClone(previous),current:{revision,source:current.source,certificate:current.certificate}});
      if(current.source!==previous.source)inverse.push({module,from:current.source,to:previous.source});
    }
    assert.deepEqual(inverse.map(row=>row.module),[review.module],'only the complete reviewed optional-emission-metadata provider changes');
    const metadata=row=>({revision:row.revision,file:row.file,bytes:row.bytes,sha256:row.sha256,certificate:row.certificate});
    const profile={sourceCommit:revision,previousSourceCommit:last.sourceCommit,review:review.review,providers,inverse,certificates,
      epoch:{reviewKind:review.kind,predecessorPacket:metadata(preceding),predecessorOutputs:priorOutputs.map(metadata)}};
    packet={...original,profiles:[...original.profiles,profile]};
    verifyReviewedOverlayProviderSuccessionHistory(packet);verifyCurrentReviewedOverlayProviderSources(packet);
    // The prior complete packet/three receipts are independently certified at
    // this real commit but never duplicated as a new multi-megabyte snapshot.
    const currentOutputs=outputs(packet),helperPath=new URL(helperFile,root),helper=readFileSync(helperPath,'utf8');
    const pattern=/const packetSha256='[a-f0-9]{64}';/u;assert.equal(helper.split(pattern).length,2);
    const source=serializeProjectData(packet,output),updated=helper.replace(pattern,"const packetSha256='"+hash(source)+"';");
    writeProjectData(new URL(output,root),packet);assert.equal(readFileSync(new URL(output,root),'utf8'),source);
    writeFileSync(helperPath,updated);
    for(const row of currentOutputs)writeFileSync(new URL(row.file,root),row.source);
    // Do not import the current reader in this author process: its already
    // loaded module correctly retains the preceding in-memory source epoch.
    // Fresh --check/normal model writers verify the new physical profile.
    return packet;
  }
  verifyCurrentReviewedOverlayProviderSources(packet);
  const currentOutputs=outputs(packet);
  for(const row of currentOutputs){
    const actual=readFileSync(new URL(row.file,root),'utf8');
    const known=[packet.historicalOutputs.find(old=>old.file===row.file).source];
    for(let length=1;length<=packet.profiles.length;length++)known.push(outputs({...packet,profiles:packet.profiles.slice(0,length)}).find(value=>value.file===row.file).source);
    assert.ok(known.includes(actual),'unknown complete current policy/receipt bytes refuse normal authoring: '+row.file);
    if(check)assert.equal(actual,row.source);else writeFileSync(new URL(row.file,root),row.source);
  }
  const{rustTextCertificateOverlayRecords}=await import('./generate.mjs');
  const{rawBindingCertificateOverlayRecords}=await import('../rust-raw-binding-certificate-adapter/generate-overlay.mjs');
  for(const[index,author]of [rustTextCertificateOverlayRecords,rawBindingCertificateOverlayRecords].entries())
    assert.equal(serializeProjectData(author(),receipts[index]),currentOutputs[index+1].source,'independent normal recipe author agrees with the actual current provider');
  return packet;
}

if(process.argv[1]===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2);let check=false,revision;
  for(const argument of args){
    if(argument==='--check'){assert.equal(check,false);check=true;}
    else if(argument.startsWith('--revision=')){assert.equal(revision,undefined);revision=argument.slice('--revision='.length);}
    else assert.fail('use --check or --revision=<actual committed source epoch>');
  }
  assert.ok(!check||revision===undefined);
  const packet=await generateReviewedOverlayProviderSuccession({check,revision});
  console.log(JSON.stringify({revisions:packet.profiles.map(row=>row.sourceCommit),providers:5,currentReceipts:2,nativeCompilation:false,nativeExecution:false}));
}
