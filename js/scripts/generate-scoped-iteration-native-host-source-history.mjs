// Normal capture of both complete committed source epochs; no native execution.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {SCOPED_ITERATION_NATIVE_SOURCE,SCOPED_ITERATION_NATIVE_OUTPUT,pinScopedIterationNativeSource,
  deriveScopedIterationNativeHostSource,verifyScopedIterationNativeHostSourceHistory} from './scoped-iteration-native-host-source-history.mjs';
const root=new URL('../../',import.meta.url),helper='js/scripts/scoped-iteration-native-host-source-history.mjs';
export function generateScopedIterationNativeHostSourceHistory({check=false}={}){
  const specification=SCOPED_ITERATION_NATIVE_SOURCE;
  const capture=revision=>{
    const{blob,certificate}=captureGitArtifactCertificate(revision,specification.file);
    const source=blob.toString('utf8');assert.deepEqual(Buffer.from(source),blob);
    return{source,...pinScopedIterationNativeSource(blob),certificate};
  };
  const previous=capture(specification.previousRevision),current=capture(specification.revision);
  assert.equal(readFileSync(new URL(specification.file,root),'utf8'),current.source,'author refuses an unrecorded live native successor');
  const packet={schemaVersion:1,model:'scoped iteration native historical source 1',file:specification.file,
    previousRevision:specification.previousRevision,revision:specification.revision,coordinateUnit:'UTF-16 code units',
    previous,current,edits:deriveScopedIterationNativeHostSource(previous.source,current.source),
    boundary:'One actual parser-owned iteration-source transport argument is retained in current runtime bytes. Both complete committed source epochs and every old source byte are independently authenticated before the unchanged required-field historical guard. Unknown paths, readers and successors remain visible; no native execution or semantic-equivalence claim.'};
  verifyScopedIterationNativeHostSourceHistory(packet);
  const output=new URL(SCOPED_ITERATION_NATIVE_OUTPUT,root);
  if(check)checkProjectData(output,packet);else writeProjectData(output,packet);
  const file=new URL(helper,root),before=readFileSync(file,'utf8'),pattern=/^const packetSha256='(?:PENDING|[a-f0-9]{64})';$/mu;
  assert.equal(before.split(pattern).length,2);
  const after=before.replace(pattern,"const packetSha256='"+pinScopedIterationNativeSource(readFileSync(output)).sha256+"';");
  if(check)assert.equal(before,after);else writeFileSync(file,after);
  return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateScopedIterationNativeHostSourceHistory({check:process.argv.includes('--check')});
  console.log('Authenticated one complete scoped-iteration native source inverse; runtime files remain current.');
}
