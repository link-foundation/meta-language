// Refresh current provenance without changing a single emitted definition.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {serializeProjectData,writeProjectData,checkProjectData} from './project-data.mjs';
import {restoreOwnCodebaseRawDescriptorText} from './own-codebase-raw-source-descriptors.mjs';
import {restoreReviewedOverlayProviderModuleSource} from '../../experiments/rust-text-certificate-adapter/provider-succession.mjs';
const root=new URL('../../',import.meta.url),revision='90636f9fb52b1b9095defbac041c8089608b8576';
const artifactFile='experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino';
const registerFile='parity/self-translation/complete-declaration-receipts.lino';
const loanFile='experiments/original-link-complete-table-projection/data/raw-borrowed-parameter-receipt.lino';
const sourceFile='js/src/typed-program-checked-projection.js';
const output='parity/self-translation/program-string-source-descriptors.lino';
const helper='js/scripts/program-string-source-descriptors.mjs';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});

/** Append one exact current-source edge; all older fields and bytes remain. */
export function generateProgramStringSourceDescriptors({check=false}={}) {
  const original=file=>{const{blob,certificate}=captureGitArtifactCertificate(revision,file);return{file,commit:revision,...pin(blob),sourceText:blob.toString('utf8'),certificate};};
  const artifact=original(artifactFile),register=original(registerFile),loan=original(loanFile),sourceOwner={...original(sourceFile),current:pin(restoreReviewedOverlayProviderModuleSource(readFileSync(new URL(sourceFile,root),'utf8'),sourceFile,'f73a10b22f8d0bcfbf4707621bcc9edd6989533d'))};
  const value=readStructuredDataLino(artifact.sourceText),changes=[];
  const owner=value.decisionSources[11];assert.equal(owner.file,sourceFile);
  assert.deepEqual(owner,{file:sourceFile,bytes:sourceOwner.bytes,sha256:sourceOwner.sha256});
  for(const field of ['bytes','sha256']){assert.notEqual(owner[field],sourceOwner.current[field]);changes.push({path:['decisionSources','11',field],before:owner[field],after:sourceOwner.current[field]});owner[field]=sourceOwner.current[field];}
  const currentText=serializeProjectData(value,artifactFile),edge={identity:'raw consuming',sourceRevision:revision,
    previous:{bytes:artifact.bytes,sha256:artifact.sha256},current:pin(currentText),source:{file:sourceFile,previous:{bytes:sourceOwner.bytes,sha256:sourceOwner.sha256},current:sourceOwner.current},changes};
  const inverse=structuredClone(value);for(const change of changes)inverse.decisionSources[11][change.path.at(-1)]=change.before;
  assert.equal(serializeProjectData(inverse,artifactFile),artifact.sourceText,'every emitted owner archive mapping fixture and original artifact byte stays exact');
  const records=readStructuredDataLino(register.sourceText);assert.equal(Object.hasOwn(records,'programStringRepresentationSourceSuccessors'),false);
  records.programStringRepresentationSourceSuccessors=[edge];const registerText=serializeProjectData(records,registerFile);
  delete records.programStringRepresentationSourceSuccessors;assert.equal(serializeProjectData(records,registerFile),register.sourceText);
  const receipt=readStructuredDataLino(loan.sourceText),previous=structuredClone(receipt.predecessor);
  assert.deepEqual({bytes:receipt.predecessor.bytes,sha256:receipt.predecessor.sha256},edge.previous);
  Object.assign(receipt.predecessor,edge.current);const loanText=serializeProjectData(receipt,loanFile);receipt.predecessor=previous;assert.equal(serializeProjectData(receipt,loanFile),loan.sourceText);
  const packet={schemaVersion:1,revision,artifact,register:{...register,current:pin(registerText)},loan:{...loan,current:pin(loanText)},sourceOwner,edge,
    boundary:'Only two current checked-program projection descriptor fields change. Every complete native definition, owner, archive, fixture, input/result and older receipt remains exact. The new register entry and borrowed predecessor descriptor have complete original byte inverses. Actual new-source generation and native execution remain mandatory independent checks.'};
  const writes=[[artifactFile,artifact.sourceText,currentText],[registerFile,register.sourceText,registerText],[loanFile,loan.sourceText,loanText]];
  for(const[file,old,text]of writes){
    const path=new URL(file,root),observed=readFileSync(path,'utf8'),actual=check?restoreOwnCodebaseRawDescriptorText(observed,file):observed;assert.ok(actual===old||actual===text,'unknown provenance edits refuse: '+file);
    if(check)assert.equal(actual,text);
  }
  if(check)checkProjectData(new URL(output,root),packet);else writeProjectData(new URL(output,root),packet);
  if(!check)for(const[file,,text]of writes)writeFileSync(new URL(file,root),text);
  const helperPath=new URL(helper,root),source=readFileSync(helperPath,'utf8'),pattern=/const policySha256='(?:PENDING|[a-f0-9]{64})';/u;
  assert.equal(source.split(pattern).length,2);const expected=source.replace(pattern,"const policySha256='"+pin(readFileSync(new URL(output,root))).sha256+"';");
  if(check)assert.equal(source,expected);else writeFileSync(helperPath,expected);
  return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateProgramStringSourceDescriptors({check:process.argv.includes('--check')});
  console.log('Refreshed two actual projection source descriptor fields; complete original artifacts and registers remain independently recoverable.');
}
