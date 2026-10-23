// Automatically capture both complete immutable host versions, never runtime edits.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {BORROWED_JAVASCRIPT_BODY_SOURCE,BORROWED_JAVASCRIPT_BODY_HISTORY,BORROWED_JAVASCRIPT_BODY_CHANGES,readBorrowedJavaScriptBodySourceIdentity,verifyBorrowedJavaScriptBodySourceHistory} from './borrowed-javascript-body-source-history.mjs';
export function generateBorrowedJavaScriptBodySourceHistory({check=false}={}){
 const specification=BORROWED_JAVASCRIPT_BODY_SOURCE;
 const capture=revision=>{const{blob,certificate}=captureGitArtifactCertificate(revision,specification.file);const source=blob.toString('utf8');assert.deepEqual(Buffer.from(source),blob);return{source,...readBorrowedJavaScriptBodySourceIdentity(source),certificate};};
 const packet={schemaVersion:1,model:'borrowed JavaScript function body native source 1',...specification,previous:capture(specification.previousRevision),current:capture(specification.currentRevision),changes:BORROWED_JAVASCRIPT_BODY_CHANGES,
  boundary:'Two complete independently certified native source epochs and the exact seven borrowing edits preserve every preceding source byte. Only existing scoped-iteration historical readers request this predecessor. The original scoped, required-field and mutable-array packets and every assertion remain exact. Current parsing, compilation and behavior require separate gates; no execution or semantic-equivalence evidence.'};
 verifyBorrowedJavaScriptBodySourceHistory(packet);
 assert.equal(readFileSync(new URL('../../'+specification.file,import.meta.url),'utf8'),packet.current.source,'unknown live native successors refuse');
 const output=new URL('../../'+BORROWED_JAVASCRIPT_BODY_HISTORY,import.meta.url);if(check)checkProjectData(output,packet);else writeProjectData(output,packet);
 return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){assert.ok(process.argv.slice(2).every(value=>value==='--check'));generateBorrowedJavaScriptBodySourceHistory({check:process.argv.includes('--check')});}
