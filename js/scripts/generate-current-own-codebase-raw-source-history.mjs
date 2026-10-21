// Normal immutable predecessor capture. Reads Git only, never executes source.
import assert from 'node:assert/strict';
import {existsSync,readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {readCurrentOwnCodebaseRawDescriptorPolicy} from './current-own-codebase-raw-source-descriptors.mjs';
import {checkProjectData,writeProjectData,serializeProjectData} from './project-data.mjs';
import {CURRENT_RAW_PREDECESSOR_REVISION,CURRENT_RAW_PREDECESSOR_OUTPUT,CURRENT_RAW_PREDECESSOR_ARCHIVE,
  CURRENT_RAW_PREDECESSOR_PIN,pinCurrentRawPredecessorSource,verifyCurrentRawPredecessorHistory} from './current-own-codebase-raw-source-history.mjs';
const root=new URL('../../',import.meta.url),helper='js/scripts/current-own-codebase-raw-source-history.mjs';

export function generateCurrentRawPredecessorHistory({check=false}={}){
  const policy=readCurrentOwnCodebaseRawDescriptorPolicy();
  const archive=captureGitArtifactCertificate(CURRENT_RAW_PREDECESSOR_REVISION,policy.output);
  assert.deepEqual(pinCurrentRawPredecessorSource(archive.blob),CURRENT_RAW_PREDECESSOR_PIN);
  const capture=file=>{const{blob,certificate}=captureGitArtifactCertificate(CURRENT_RAW_PREDECESSOR_REVISION,file);return{file,...pinCurrentRawPredecessorSource(blob),certificate};};
  const history={schemaVersion:1,model:'complete prior current raw source descriptors 1',revision:CURRENT_RAW_PREDECESSOR_REVISION,
    archive:{path:CURRENT_RAW_PREDECESSOR_ARCHIVE,file:policy.output,...CURRENT_RAW_PREDECESSOR_PIN,certificate:archive.certificate},
    outputs:[policy.artifactFile,policy.registerFile,policy.loanFile].map(capture),sources:policy.sourcePaths.map(capture),
    boundary:'The complete independently committed8d current packet is retained once as unchanged Links bytes. Every prior native106-definition artifact, source descriptor, receipt edge and alias is exactly reconstructible and independently checked against its8d Git object. This grants only a finite normal-author write preimage, never current source identity, semantic equality, native execution or permission to overwrite unknown bytes.'};
  verifyCurrentRawPredecessorHistory(history,archive.blob);
  const archivePath=new URL(CURRENT_RAW_PREDECESSOR_ARCHIVE,root);
  if(check)assert.deepEqual(readFileSync(archivePath),archive.blob);
  else if(existsSync(archivePath))assert.deepEqual(readFileSync(archivePath),archive.blob,'the immutable prior packet is never overwritten');
  else writeFileSync(archivePath,archive.blob);
  const output=new URL(CURRENT_RAW_PREDECESSOR_OUTPUT,root);
  if(existsSync(output))assert.equal(readFileSync(output,'utf8'),serializeProjectData(history,CURRENT_RAW_PREDECESSOR_OUTPUT),'an unknown predecessor manifest is never overwritten');
  if(check)checkProjectData(output,history);else writeProjectData(output,history);
  const path=new URL(helper,root),source=readFileSync(path,'utf8'),pattern=/const historySha256='(?:PENDING|[a-f0-9]{64})';/u;
  assert.equal(source.split(pattern).length,2);const expected=source.replace(pattern,"const historySha256='"+pinCurrentRawPredecessorSource(readFileSync(output)).sha256+"';");
  if(check)assert.equal(source,expected);else writeFileSync(path,expected);
  return history;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(argument=>argument==='--check'));
  const history=generateCurrentRawPredecessorHistory({check:process.argv.includes('--check')});
  console.log(JSON.stringify({revision:history.revision,archive:history.archive.path,outputs:history.outputs.length,nativeCompilation:false,nativeExecution:false}));
}
