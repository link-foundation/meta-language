// Automatically capture independently committed whole inputs and append a finite provenance edge.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {readOwnCodebaseRawDescriptorPolicy,deriveOwnCodebaseRawDescriptorSource,deriveOwnCodebaseRawDescriptorOutputs,verifyOwnCodebaseRawDescriptorPacket} from './own-codebase-raw-source-descriptors.mjs';
const root=new URL('../../',import.meta.url);
const output='parity/self-translation/own-codebase-raw-source-descriptors.lino';
const helper='js/scripts/own-codebase-raw-source-descriptors.mjs';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});

/** Preserve all stored semantics; actual current translation and native replay run independently in CI. */
export function generateOwnCodebaseRawSourceDescriptors({check=false}={}){
  const policy=readOwnCodebaseRawDescriptorPolicy();
  const original=(file,current)=>{const{blob,certificate}=captureGitArtifactCertificate(policy.revision,file);return{file,commit:policy.revision,...pin(blob),sourceText:blob.toString('utf8'),certificate,current};};
  const [artifact,register,loan]=policy.anchors.map(row=>original(row.file,null));
  const sourceOwner={...original(policy.sourceFile,policy.sourceOwner.current),edits:policy.sourceOwner.edits};
  const adapters=policy.adapters.map(row=>({...original(row.file,row.current),edits:row.edits}));
  const packet={schemaVersion:policy.schemaVersion,model:policy.model,revision:policy.revision,artifact,register,loan,sourceOwner,adapters,edge:null,boundary:policy.boundary};
  const derived=deriveOwnCodebaseRawDescriptorOutputs(packet);packet.edge=derived.edge;
  for(const row of [artifact,register,loan])row.current=pin(derived.outputs.find(output=>output.file===row.file).source);
  verifyOwnCodebaseRawDescriptorPacket(packet);
  assert.deepEqual(pin(readFileSync(new URL(policy.sourceFile,root))),policy.sourceOwner.current,'the actual reviewed frontend source, not an archived read view, is used');
  const writes=derived.outputs.map(row=>({...row,previous:[artifact,register,loan].find(capture=>capture.file===row.file).sourceText}));
  for(const [index,row]of adapters.entries())writes.push({file:row.file,previous:row.sourceText,source:deriveOwnCodebaseRawDescriptorSource(policy.adapters[index],row.sourceText)});
  for(const row of writes){const observed=readFileSync(new URL(row.file,root),'utf8');assert.ok(observed===row.previous||observed===row.source,'unknown complete source or artifact changes refuse: '+row.file);if(check)assert.equal(observed,row.source);}
  const helperPath=new URL(helper,root),source=readFileSync(helperPath,'utf8'),pattern=/^const policySha256='(?:PENDING|[a-f0-9]{64})';$/mu;
  assert.equal(source.split(pattern).length,2);
  if(check)checkProjectData(new URL(output,root),packet);else writeProjectData(new URL(output,root),packet);
  const expected=source.replace(pattern,"const policySha256='"+pin(readFileSync(new URL(output,root))).sha256+"';");
  if(check)assert.equal(source,expected);else{for(const row of writes)writeFileSync(new URL(row.file,root),row.source);writeFileSync(helperPath,expected);}
  return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2);assert.ok(args.length<=1&&args.every(arg=>arg==='--check'));
  generateOwnCodebaseRawSourceDescriptors({check:args.includes('--check')});
  console.log('Current frontend descriptors and borrowed predecessor pins updated; every certified original byte retained, without translation or native execution.');
}
