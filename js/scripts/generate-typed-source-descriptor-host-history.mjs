// Normal metadata author: no translation workload or native execution.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {TYPED_SOURCE_DESCRIPTOR_HOST_REVISION as revision,TYPED_SOURCE_DESCRIPTOR_HOST_OUTPUT as output,TYPED_SOURCE_DESCRIPTOR_HOST_FILES as files,pinTypedSourceDescriptorHost as pin,deriveTypedSourceDescriptorHostEdit,verifyTypedSourceDescriptorHostHistory} from './typed-source-descriptor-host-history.mjs';
const root=new URL('../../',import.meta.url);
export function generateTypedSourceDescriptorHostHistory({check=false}={}){
 const rows=files.map(file=>{const{blob,certificate}=captureGitArtifactCertificate(revision,file),source=blob.toString('utf8');assert.deepEqual(Buffer.from(source),blob);const current=readFileSync(new URL(file,root),'utf8');return{file,previous:{source,...pin(source),certificate},current:{source:current,...pin(current)},edit:deriveTypedSourceDescriptorHostEdit(source,current)};});
 const packet={schemaVersion:1,model:'complete typed source descriptor host history 1',revision,rows,
  boundary:'Six complete source descriptor hosts, historical callers and classifier modules retain their independently certified preceding definitions and finite whole-source inverses. Only explicit historical guards select these views. Every current provider, complete original artifact, receipt, definition, fixture and refusal remains mandatory. Unknown complete source edits retain their original refusal; no runtime module is overwritten and no semantic equivalence or native execution is claimed.'};
 verifyTypedSourceDescriptorHostHistory(packet);const file=new URL(output,root);if(check)checkProjectData(file,packet);else writeProjectData(file,packet);
 const helper=new URL('./typed-source-descriptor-host-history.mjs',import.meta.url),source=readFileSync(helper,'utf8'),pattern=/const packetSha256='(?:PENDING|[a-f0-9]{64})';/u;assert.equal(source.split(pattern).length,2);
 const expected=source.replace(pattern,"const packetSha256='"+pin(readFileSync(file)).sha256+"';");if(check)assert.equal(source,expected);else writeFileSync(helper,expected);return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){assert.ok(process.argv.slice(2).every(value=>value==='--check'));generateTypedSourceDescriptorHostHistory({check:process.argv.includes('--check')});console.log('Retained all six complete typed source descriptor host predecessors.');}
