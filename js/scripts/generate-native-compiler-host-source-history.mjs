// Normal automatic capture; never overwrites a runtime host or earlier recipe.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {NATIVE_COMPILER_HOST_PREVIOUS,NATIVE_COMPILER_HOST_CURRENT,NATIVE_COMPILER_HOST_HISTORY,NATIVE_COMPILER_HOST_SOURCES,readNativeCompilerHostSourceIdentity,verifyNativeCompilerHostSourceHistory} from './native-compiler-host-source-history.mjs';
import {CRATE_FIXTURE_PACKAGE_SOURCE,CRATE_FIXTURE_PACKAGE_CHANGE,verifyCrateFixturePackageSourceHistory} from './crate-fixture-package-source-history.mjs';
export function generateNativeCompilerHostSourceHistory({check=false}={}){
 const capture=(revision,file)=>{const{blob,certificate}=captureGitArtifactCertificate(revision,file);const source=blob.toString('utf8');assert.deepEqual(Buffer.from(source),blob);return{source,...readNativeCompilerHostSourceIdentity(source),certificate};};
 const sources=NATIVE_COMPILER_HOST_SOURCES.map(({file,changes})=>({file,previous:capture(NATIVE_COMPILER_HOST_PREVIOUS,file),current:capture(NATIVE_COMPILER_HOST_CURRENT,file),changes}));
 const packet={schemaVersion:1,model:'complete native compiler historical host sources 1',previousRevision:NATIVE_COMPILER_HOST_PREVIOUS,currentRevision:NATIVE_COMPILER_HOST_CURRENT,sources,
  boundary:'Complete independently certified old/current compiler-repair sources preserve every prior host and ABI artifact byte. The artifact retains every checked program, replay and interface field; only certified native source files differ. Exact source conservation is not semantic or native execution evidence. Current behavior keeps independent gates.'};
 verifyNativeCompilerHostSourceHistory(packet);
 for(const row of sources)assert.equal(readFileSync(new URL('../../'+row.file,import.meta.url),'utf8'),row.current.source,'unknown live compiler successor refuses');
 const output=new URL('../../'+NATIVE_COMPILER_HOST_HISTORY,import.meta.url);if(check)checkProjectData(output,packet);else writeProjectData(output,packet);return packet;
}
export function generateCrateFixturePackageSourceHistory(options){
 const check=options?.check??false;
 const {file,previousRevision,currentRevision,history}=CRATE_FIXTURE_PACKAGE_SOURCE;
 const capture=revision=>{const{blob,certificate}=captureGitArtifactCertificate(revision,file),source=blob.toString('utf8');assert.deepEqual(Buffer.from(source),blob);return{source,...readNativeCompilerHostSourceIdentity(source),certificate};};
 const packet={schemaVersion:1,model:'complete crate fixture package source 1',file,previousRevision,currentRevision,
  previous:capture(previousRevision),current:capture(currentRevision),change:CRATE_FIXTURE_PACKAGE_CHANGE,
  boundary:'Complete independently certified crate manifests and exact inverse retain every original package input and add only the two generated crate-local fixture entries. Historical readers select their complete original source; current packaging and runtime checks remain separate. Source conservation is not execution evidence.'};
 verifyCrateFixturePackageSourceHistory(packet);assert.equal(readFileSync(new URL('../../'+file,import.meta.url),'utf8'),packet.current.source);
 const output=new URL('../../'+history,import.meta.url);if(check)checkProjectData(output,packet);else writeProjectData(output,packet);return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){assert.ok(process.argv.slice(2).every(value=>value==='--check'));const options={check:process.argv.includes('--check')};generateNativeCompilerHostSourceHistory(options);generateCrateFixturePackageSourceHistory(options);}
