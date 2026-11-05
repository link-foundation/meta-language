// Source metadata only: no native compilation or execution.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {SHARED_ARRAY_NATIVE_FRONTEND_HISTORY as output,SHARED_ARRAY_NATIVE_FRONTEND_PREVIOUS as previousRevision,SHARED_ARRAY_NATIVE_FRONTEND_CURRENT as currentRevision,SHARED_ARRAY_NATIVE_FRONTEND_FILES as files,pinSharedArrayNativeFrontend as pin,deriveSharedArrayNativeFrontendEdit,verifySharedArrayNativeFrontendHistory} from './shared-array-native-frontend-history.mjs';
const root=new URL('../../',import.meta.url);
export function generateSharedArrayNativeFrontendHistory({check=false}={}){
 const capture=(revision,file)=>{const{blob,certificate}=captureGitArtifactCertificate(revision,file),source=blob.toString('utf8');assert.deepEqual(Buffer.from(source),blob);return{source,...pin(blob),certificate};};
 const rows=files.map(file=>{const previous=capture(previousRevision,file),current=capture(currentRevision,file);assert.equal(readFileSync(new URL(file,root),'utf8'),current.source);return{file,previous,current,edit:deriveSharedArrayNativeFrontendEdit(previous.source,current.source)};});
 const packet={schemaVersion:1,model:'complete shared array native frontend source succession 1',previousRevision,currentRevision,rows,boundary:'Only seven complete committed parser/checker/projection host modules receive an explicitly historical source view. Their exact current bytes, complete independently certified predecessors and finite inverses retain all old ASCII/literal-match, Boolean and checked projection host assertions. Actual current parsers, source profiles, original semantic fixtures and native execution remain separate mandatory CI checks. Unknown sources cannot inherit this identity; no runtime file is overwritten.'};
 verifySharedArrayNativeFrontendHistory(packet);const file=new URL(output,root);if(check)checkProjectData(file,packet);else writeProjectData(file,packet);
 const helper=new URL('js/scripts/shared-array-native-frontend-history.mjs',root),source=readFileSync(helper,'utf8'),pattern=/const packetSha256='(?:PENDING|[a-f0-9]{64})';/u;assert.equal(source.split(pattern).length,2);
 const expected=source.replace(pattern,"const packetSha256='"+pin(readFileSync(file)).sha256+"';");if(check)assert.equal(source,expected);else writeFileSync(helper,expected);return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){assert.ok(process.argv.slice(2).every(value=>value==='--check'));generateSharedArrayNativeFrontendHistory({check:process.argv.includes('--check')});console.log('Retained all complete certified native frontend predecessors and actual source inverses.');}
