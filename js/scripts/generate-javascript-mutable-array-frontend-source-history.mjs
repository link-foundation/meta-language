// Normal capture of the two actual complete committed frontend source epochs.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {MUTABLE_ARRAY_FRONTEND_FILE,MUTABLE_ARRAY_FRONTEND_PREVIOUS_REVISION,MUTABLE_ARRAY_FRONTEND_CURRENT_REVISION,MUTABLE_ARRAY_FRONTEND_OUTPUT,MUTABLE_ARRAY_FRONTEND_HOST_PACKET,
  pinMutableArrayFrontendSource,deriveMutableArrayFrontendSourceEdit,verifyMutableArrayFrontendSourceHistory} from './javascript-mutable-array-frontend-source-history.mjs';
const root=new URL('../../',import.meta.url),helper='js/scripts/javascript-mutable-array-frontend-source-history.mjs';
export function generateJavaScriptMutableArrayFrontendSourceHistory({check=false,revision=MUTABLE_ARRAY_FRONTEND_CURRENT_REVISION}={}){
  assert.match(revision,/^[a-f0-9]{40}$/u,'first capture requires --revision=<full actual committed current source revision>');
  assert.ok(MUTABLE_ARRAY_FRONTEND_CURRENT_REVISION==='PENDING'||MUTABLE_ARRAY_FRONTEND_CURRENT_REVISION===revision,'a known current source certificate cannot silently change epochs');
  const capture=commit=>{const{blob,certificate}=captureGitArtifactCertificate(commit,MUTABLE_ARRAY_FRONTEND_FILE);const source=blob.toString('utf8');assert.deepEqual(Buffer.from(source),blob);return{source,...pinMutableArrayFrontendSource(blob),certificate};};
  const previous=capture(MUTABLE_ARRAY_FRONTEND_PREVIOUS_REVISION),current=capture(revision);
  assert.equal(current.source,readFileSync(new URL(MUTABLE_ARRAY_FRONTEND_FILE,root),'utf8'),'the complete actual physical frontend is the certified current source');
  const packet={schemaVersion:1,model:'mutable array frontend historical source 1',file:MUTABLE_ARRAY_FRONTEND_FILE,previousRevision:MUTABLE_ARRAY_FRONTEND_PREVIOUS_REVISION,revision,
    coordinateUnit:'UTF-16 code units',previous,current,edit:deriveMutableArrayFrontendSourceEdit(previous.source,current.source),hostPacket:{...MUTABLE_ARRAY_FRONTEND_HOST_PACKET},
    boundary:'Only the mutable-array historical source author receives this certified complete predecessor view. Actual optional-tuple semantics, generated frontend Rust, styles, native hosts and all other source readers stay current; unknown sources remain visible to their original guards.'};
  verifyMutableArrayFrontendSourceHistory(packet,revision);
  const output=new URL(MUTABLE_ARRAY_FRONTEND_OUTPUT,root);
  if(check)checkProjectData(output,packet);else writeProjectData(output,packet);
  const file=new URL(helper,root),before=readFileSync(file,'utf8');
  const patterns=[[/^export const MUTABLE_ARRAY_FRONTEND_CURRENT_REVISION='(?:PENDING|[a-f0-9]{40})';$/mu,"export const MUTABLE_ARRAY_FRONTEND_CURRENT_REVISION='"+revision+"';"],
    [/^const packetSha256='(?:PENDING|[a-f0-9]{64})';$/mu,"const packetSha256='"+pinMutableArrayFrontendSource(readFileSync(output)).sha256+"';"]];
  let after=before;for(const[pattern,value]of patterns){assert.equal(after.split(pattern).length,2);after=after.replace(pattern,()=>value);}
  if(check)assert.equal(before,after);else writeFileSync(file,after);
  return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2);assert.ok(args.every(value=>value==='--check'||/^--revision=[a-f0-9]{40}$/u.test(value)));assert.ok(args.filter(value=>value.startsWith('--revision=')).length<=1);
  generateJavaScriptMutableArrayFrontendSourceHistory({check:args.includes('--check'),revision:args.find(value=>value.startsWith('--revision='))?.slice(11)??MUTABLE_ARRAY_FRONTEND_CURRENT_REVISION});
  console.log('Authenticated the full old and current frontend source; every immutable mutable-array host byte remains unchanged.');
}
