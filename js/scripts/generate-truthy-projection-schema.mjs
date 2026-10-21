// Normal author only; no semantic execution observation is recorded here.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {writeProjectData,checkProjectData,readProjectData} from './project-data.mjs';
import {inspectedTruthyPreimages,TRUTHY_PROJECTION_REVISION,TRUTHY_PROJECTION_OUTPUT,
 TRUTHY_PROJECTION_ENTRIES,pinTruthyProjectionSource,deriveTruthyProjectionSchema,
 deriveTruthyProjectionHostSource,verifyTruthyProjectionSchemaPacket,readTruthyProjectionSchemaPacket,
 TRUTHY_PROJECTION_HOST_SUCCESSORS,verifyTruthyProjectionHostSuccessors,restoreTruthyProjectionHostSource} from './truthy-projection-schema.mjs';

const root=new URL('../../',import.meta.url),helper='js/scripts/truthy-projection-schema.mjs';
export function generateTruthyProjectionSchema({check=false}={}) {
 const stored=check?readTruthyProjectionSchemaPacket():null;
 const sources=inspectedTruthyPreimages.map((expected,index)=>{
  const original=stored?{blob:Buffer.from(stored.sources[index].previous.source),certificate:stored.sources[index].certificate}
   :captureGitArtifactCertificate(TRUTHY_PROJECTION_REVISION,expected.file);
  assert.deepEqual(pinTruthyProjectionSource(original.blob),{bytes:expected.bytes,sha256:expected.sha256},
   'actual immutable committed predecessor must match: '+expected.file);
  const previous=original.blob.toString('utf8');
  const current=index<2?deriveTruthyProjectionSchema(previous):deriveTruthyProjectionHostSource(expected.file,previous);
  const physical=readFileSync(new URL(expected.file,root),'utf8');
  const actual=check?restoreTruthyProjectionHostSource(physical,expected.file):physical;
  assert.ok(actual===previous||actual===current,'unknown actual source refuses: '+expected.file);
  if(check)assert.equal(actual,current,'actual current source must be the complete normal successor');
  return{file:expected.file,certificate:original.certificate,
   previous:{source:previous,...pinTruthyProjectionSource(previous)},current:{source:current,...pinTruthyProjectionSource(current)}};
 });
 const packet={schemaVersion:1,revision:TRUTHY_PROJECTION_REVISION,extensions:TRUTHY_PROJECTION_ENTRIES,sources,
  boundary:'Two additive shared schema entries reconstruct the existing checked Boolean conversion exactly. Existing shared source-language/type Boolean policy owns semantics; complete original schema and host bytes retain independent committed membership and exact inverses. No source rewrites, old abort changes, weakened IR comparison or native execution evidence.'};
 verifyTruthyProjectionSchemaPacket(packet);
 const output=new URL(TRUTHY_PROJECTION_OUTPUT,root),helperFile=new URL(helper,root),before=readFileSync(helperFile,'utf8');
 const pattern=/^const recipeSha256='(?:PENDING|[a-f0-9]{64})';$/mu;assert.equal(before.split(pattern).length,2);
 if(check)checkProjectData(output,packet);else writeProjectData(output,packet);
 const after=before.replace(pattern,"const recipeSha256='"+pinTruthyProjectionSource(readFileSync(output)).sha256+"';");
 if(check)assert.equal(before,after);
 else{for(const row of sources)writeFileSync(new URL(row.file,root),row.current.source);writeFileSync(helperFile,after);}
 return structuredClone(packet);
}

export function generateTruthyProjectionHostSuccessors({check=false}={}) {
 const output=new URL(TRUTHY_PROJECTION_HOST_SUCCESSORS,root);
 if(check){const packet=verifyTruthyProjectionHostSuccessors(readProjectData(output));for(const row of packet.sources)assert.equal(readFileSync(new URL(row.file,root),'utf8'),row.current.source);checkProjectData(output,packet);return packet;}
 const previousRevision='6ffff3b2dc530a5f24c128dadf877f3fa106932b',revision='3fe3a9f9decfa6431b5b0718729a5d123b5f236b';
 const sources=['js/src/translation/check.js','rust/src/translation/surface/nodes.rs','rust/src/translation/check/expressions.rs'].map(file=>{
  const capture=commit=>{const {blob,certificate}=captureGitArtifactCertificate(commit,file);const source=blob.toString('utf8');assert.deepEqual(Buffer.from(source),blob);return {source,...pinTruthyProjectionSource(source),certificate};};
  const previous=capture(previousRevision),current=capture(revision);assert.equal(readFileSync(new URL(file,root),'utf8'),current.source);
  let start=0,end=0;while(previous.source[start]===current.source[start]&&start<Math.min(previous.source.length,current.source.length))start++;
  while(end<Math.min(previous.source.length,current.source.length)-start&&previous.source.at(-end-1)===current.source.at(-end-1))end++;
  return {file,previous,current,edit:{start,removed:previous.source.slice(start,previous.source.length-end),inserted:current.source.slice(start,current.source.length-end)}};
 });
 const packet={schemaVersion:1,model:'Boolean projection checker source succession 1',previousRevision,revision,sources,
  boundary:'Only complete certified checker sources receive the original Boolean projection source view. Runtime semantics and the immutable original projection packet remain unchanged; unknown source bytes refuse.'};
 verifyTruthyProjectionHostSuccessors(packet);writeProjectData(output,packet);return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const args=process.argv.slice(2);assert.ok(args.length<=2&&new Set(args).size===args.length&&args.every(arg=>['--check','--host-successors'].includes(arg)));
 (args.includes('--host-successors')?generateTruthyProjectionHostSuccessors:generateTruthyProjectionSchema)({check:args.includes('--check')});
 console.log('Checked Boolean conversion projection: complete predecessor/current sources and two-entry inverse authenticated; execution remains separate.');
}
