// Provenance changes preserve complete emitted definitions and prior receipts.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from './project-data.mjs';
import {structuredDataToLino} from '../src/structured-data-links.js';
import {restoreNativeLibraryReceipt} from './native-library-receipt-history.mjs';
import {restoreProgramStringSourceDescriptorEdge} from './program-string-source-descriptors.mjs';

const policyFile=new URL('../../parity/self-translation/current-translation-source-descriptors.lino',import.meta.url);
const registerFile=new URL('../../parity/self-translation/complete-declaration-receipts.lino',import.meta.url);
const policyHash='d3176c78b86a441a8f7d8cb3b33125c58066698f6054194ec2c8fe1db8cd1a36';
const hash=bytes=>createHash('sha256').update(bytes).digest('hex');
let policy;

function readSourceDescriptorPolicy(){
  if(!policy){
    const bytes=readFileSync(policyFile);
    assert.equal(hash(bytes),policyHash,'the complete independently reviewed source descriptor policy stays exact');
    policy=readProjectData(policyFile);assert.equal(policy.schemaVersion,1);
  }
  return policy;
}

export function restoreCurrentTranslationSourceDescriptors(artifact,identity,register){
  ({artifact,register}=restoreProgramStringSourceDescriptorEdge(artifact,identity,register));
  if(!Object.hasOwn(register,'sourceDescriptorSuccessors'))return{artifact,register};
  const selected=readSourceDescriptorPolicy();
  assert.equal(Object.keys(register).at(-1),'sourceDescriptorSuccessors');
  const {sourceDescriptorSuccessors,...previousRegister}=register;
  assert.deepEqual(sourceDescriptorSuccessors,[selected.edge],'only the exact reviewed source descriptor successor is admitted');
  const registerBytes=Buffer.from(structuredDataToLino(previousRegister));
  assert.equal(registerBytes.length,selected.previousRegister.bytes);
  assert.equal(hash(registerBytes),selected.previousRegister.sha256,'every complete previous receipt and field remains unchanged');
  const bytes=Buffer.from(structuredDataToLino(artifact));
  if(identity!==selected.edge.identity||hash(bytes)!==selected.edge.current.sha256){
    return{artifact,register:previousRegister};
  }
  assert.equal(bytes.length,selected.edge.current.bytes);
  const inverse=structuredClone(artifact);
  for(const change of selected.edge.changes){
    assert.ok(change.path.length>1&&change.path.every(key=>typeof key==='string'&&!['__proto__','prototype','constructor'].includes(key)));
    let owner=inverse;
    for(const key of change.path.slice(0,-1)){assert.ok(Object.hasOwn(owner,key));owner=owner[key];}
    const key=change.path.at(-1);
    assert.deepEqual(owner[key],change.after);
    if(change.kind==='source history'){
      assert.equal(key,'sourceHistory');assert.equal(change.before,'absent');assert.equal(delete owner[key],true);
    }else{
      assert.equal(change.kind,'source descriptor');assert.ok(['bytes','sha256'].includes(key));owner[key]=change.before;
    }
  }
  const restored=Buffer.from(structuredDataToLino(inverse));
  assert.equal(restored.length,selected.edge.previous.bytes);
  assert.equal(hash(restored),selected.edge.previous.sha256,'only recorded provenance changes may differ in the entire previous artifact');
  return{artifact:inverse,register:previousRegister};
}

export function readHistoricalTranslationSourceDescriptorRegister(){
  return restoreCurrentTranslationSourceDescriptors(null,'historical register',readProjectData(registerFile)).register;
}

export function projectHistoricalTranslationSourceDescriptorArtifact(artifact){
  let register=readProjectData(registerFile);
  ({artifact,register}=restoreProgramStringSourceDescriptorEdge(artifact,'raw consuming',register));
  artifact=restoreNativeLibraryReceipt(artifact,'raw consuming');
  const selected=readSourceDescriptorPolicy(),bytes=Buffer.from(structuredDataToLino(artifact));
  assert.ok([selected.edge.current.sha256,selected.edge.previous.sha256].includes(hash(bytes)),
    'a historical descriptor view requires one complete registered artifact');
  return restoreCurrentTranslationSourceDescriptors(artifact,selected.edge.identity,register).artifact;
}
