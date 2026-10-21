// Capture the finite899de cohort once; check mode uses stored certificates.
import assert from 'node:assert/strict';
import {existsSync,readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {CURRENT_RAW_PREDECESSOR_REVISION,CURRENT_RAW_PREDECESSOR_PIN} from './current-own-codebase-raw-source-history.mjs';
import {readCurrentOwnCodebaseRawDescriptorPolicy} from './current-own-codebase-raw-source-descriptors.mjs';
import {checkProjectData,writeProjectData,serializeProjectData} from './project-data.mjs';
import {ADDITIONAL_CURRENT_RAW_PREDECESSOR_REVISION,ADDITIONAL_CURRENT_RAW_PREDECESSOR_OUTPUT,
  ADDITIONAL_CURRENT_RAW_PREDECESSOR_PIN,ADDITIONAL_CURRENT_RAW_PREDECESSOR_BOUNDARY,
  pinAdditionalCurrentRawPredecessor,deriveAdditionalCurrentRawPredecessor,
  readAdditionalCurrentRawPredecessorHistory} from './current-raw-descriptor-cohort.mjs';
const root=new URL('../../',import.meta.url),helper='js/scripts/current-raw-descriptor-cohort.mjs';

export function generateAdditionalCurrentRawPredecessorHistory({check=false}={}){
  const policy=readCurrentOwnCodebaseRawDescriptorPolicy();
  const output=new URL(ADDITIONAL_CURRENT_RAW_PREDECESSOR_OUTPUT,root);
  let history;
  if(check){history=readAdditionalCurrentRawPredecessorHistory();}
  else{
    const revision=ADDITIONAL_CURRENT_RAW_PREDECESSOR_REVISION;
    const original=captureGitArtifactCertificate(revision,policy.output);
    assert.deepEqual(pinAdditionalCurrentRawPredecessor(original.blob),ADDITIONAL_CURRENT_RAW_PREDECESSOR_PIN);
    assert.ok(Buffer.from(original.blob.toString('utf8')).equals(original.blob));
    const packet=readBootstrapStructuredDataLino(original.blob.toString('utf8'));
    const sources=policy.sourcePaths.map((file,index)=>{
      const{blob,certificate}=captureGitArtifactCertificate(revision,file),row=packet.sources[index];
      assert.equal(row.file,file);assert.ok(Buffer.from(row.current.sourceText).equals(blob),'the certified899 source is the complete packet current source');
      return{file,current:structuredClone(row.current),edits:structuredClone(row.edits),certificate};
    });
    const outputs=[policy.artifactFile,policy.registerFile,policy.loanFile].map(file=>{
      const{blob,certificate}=captureGitArtifactCertificate(revision,file);return{file,...pinAdditionalCurrentRawPredecessor(blob),certificate};
    });
    history={schemaVersion:1,model:'additional complete current raw descriptor cohort 1',revision,
      base:{revision:CURRENT_RAW_PREDECESSOR_REVISION,...CURRENT_RAW_PREDECESSOR_PIN},
      packet:{file:policy.output,...ADDITIONAL_CURRENT_RAW_PREDECESSOR_PIN,certificate:original.certificate},sources,outputs,
      boundary:ADDITIONAL_CURRENT_RAW_PREDECESSOR_BOUNDARY};
    const derived=deriveAdditionalCurrentRawPredecessor(history);
    assert.equal(derived.source,original.blob.toString('utf8'),'every complete independently committed899 packet byte is reconstructed');
  }
  // Validate all seven memberships and every complete inverse before any write.
  const path=new URL(helper,root),source=readFileSync(path,'utf8'),pattern=/^const cohortSha256='(?:PENDING|[a-f0-9]{64})';$/mu;
  assert.equal(source.split(pattern).length,2);
  if(existsSync(output))assert.equal(readFileSync(output,'utf8'),serializeProjectData(history,ADDITIONAL_CURRENT_RAW_PREDECESSOR_OUTPUT),'an unknown cohort is never overwritten');
  if(check)checkProjectData(output,history);else writeProjectData(output,history);
  const expected=source.replace(pattern,"const cohortSha256='"+pinAdditionalCurrentRawPredecessor(readFileSync(output)).sha256+"';");
  if(check)assert.equal(source,expected);else writeFileSync(path,expected);
  return history;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(argument=>argument==='--check'));
  const history=generateAdditionalCurrentRawPredecessorHistory({check:process.argv.includes('--check')});
  console.log(JSON.stringify({revision:history.revision,sourceRows:history.sources.length,outputs:history.outputs.length,largeArtifactsCopied:false,nativeExecution:false}));
}
