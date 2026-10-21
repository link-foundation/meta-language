// Normal author only; no semantic execution observation is recorded here.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {writeProjectData,checkProjectData} from './project-data.mjs';
import {inspectedTruthyPreimages,TRUTHY_PROJECTION_REVISION,TRUTHY_PROJECTION_OUTPUT,
 TRUTHY_PROJECTION_ENTRIES,pinTruthyProjectionSource,deriveTruthyProjectionSchema,
 deriveTruthyProjectionHostSource,verifyTruthyProjectionSchemaPacket,readTruthyProjectionSchemaPacket} from './truthy-projection-schema.mjs';

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
  const actual=readFileSync(new URL(expected.file,root),'utf8');
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
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const args=process.argv.slice(2);assert.ok(args.length<=1&&args.every(arg=>arg==='--check'));
 generateTruthyProjectionSchema({check:args.includes('--check')});
 console.log('Checked Boolean conversion projection: complete predecessor/current sources and two-entry inverse authenticated; execution remains separate.');
}
