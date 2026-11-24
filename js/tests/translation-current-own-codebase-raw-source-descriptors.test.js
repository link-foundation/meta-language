// New current-source controls supplement every retained historical control.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData,serializeProjectData} from '../scripts/project-data.mjs';
import {generateCurrentOwnCodebaseRawSourceDescriptors} from '../scripts/generate-current-own-codebase-raw-source-descriptors.mjs';
import {generateOwnCodebaseRawSourceDescriptors} from '../scripts/generate-own-codebase-raw-source-descriptors.mjs';
import {readOwnCodebaseRawDescriptorPacket} from '../scripts/own-codebase-raw-source-descriptors.mjs';
import {readCurrentOwnCodebaseRawDescriptorPolicy,readCurrentOwnCodebaseRawDescriptorPacket,
  verifyCurrentOwnCodebaseRawDescriptorPacket,deriveCurrentOwnCodebaseRawDescriptorSource,
  restoreCurrentOwnCodebaseRawSource,restoreCurrentOwnCodebaseRawText,restoreCurrentOwnCodebaseRawEdge} from '../scripts/current-own-codebase-raw-source-descriptors.mjs';
import {restoreCompleteDeclarationReceipt} from '../scripts/complete-declaration-receipts.mjs';
import {restoreTypedSourceDescriptorHostSource} from '../scripts/typed-source-descriptor-host-history.mjs';
const root=new URL('../../',import.meta.url);
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});

test('one current source edge retains every complete prior artifact receipt alias and emitted definition',()=>{
  const packet=generateCurrentOwnCodebaseRawSourceDescriptors({check:true}),policy=readCurrentOwnCodebaseRawDescriptorPolicy();
  assert.equal(packet.edge.changes.length,10);assert.equal(packet.edge.sources.length,3);
  const actual=readProjectData(new URL(policy.artifactFile,root)),register=readProjectData(new URL(policy.registerFile,root));
  const snapshot=serializeProjectData(actual,policy.artifactFile),restored=restoreCurrentOwnCodebaseRawEdge(actual,'raw consuming',register);
  assert.equal(serializeProjectData(restored.artifact,policy.artifactFile),packet.artifact.sourceText);
  assert.equal(serializeProjectData(restored.register,policy.registerFile),packet.register.sourceText);
  assert.equal(serializeProjectData(actual,policy.artifactFile),snapshot,'the caller graph is not changed');
  assert.equal(actual.definitionCount,106);assert.equal(actual.owners.length,106);
  for(const field of ['completeRust','owners','declarationNames','totals','assumptions','encodings','mappings','preludeHashes','replays','archive'])assert.deepEqual(actual[field],restored.artifact[field],field);
  const proof=actual.predecessor.proof.producerApplicability,oldProof=restored.artifact.predecessor.proof.producerApplicability;
  assert.equal(proof.sources.find(row=>row.file===policy.sourcePaths[1])===proof.sourceHistory.current,packet.inferenceDescriptorAlias);
  assert.equal(oldProof.sources.find(row=>row.file===policy.sourcePaths[1])===oldProof.sourceHistory.current,packet.inferenceDescriptorAlias);
  assert.deepEqual(proof.sourceHistory.current,actual.decisionSources[2]);
  for(const row of packet.sources){
    const source=readFileSync(new URL(row.file,root),'utf8');assert.equal(source,row.current.sourceText);assert.deepEqual(pin(source),{bytes:row.current.bytes,sha256:row.current.sha256});
    assert.equal(restoreCurrentOwnCodebaseRawSource(source,row.file),row.sourceText);
  }
  for(const row of packet.adapters)assert.equal(deriveCurrentOwnCodebaseRawDescriptorSource(row),restoreTypedSourceDescriptorHostSource(readFileSync(new URL(row.file,root),'utf8'),row.file));
  for(const row of[packet.artifact,packet.register,packet.loan])assert.equal(restoreCurrentOwnCodebaseRawText(readFileSync(new URL(row.file,root),'utf8'),row.file),row.sourceText);
  const loan=readProjectData(new URL(policy.loanFile,root));assert.deepEqual({bytes:loan.predecessor.bytes,sha256:loan.predecessor.sha256},packet.edge.current);
  assert.doesNotThrow(()=>restoreCompleteDeclarationReceipt(actual,'raw consuming',{register}));
});

test('the old own-codebase packet keeps its identity and its author cannot overwrite a current successor',()=>{
  const policy=readCurrentOwnCodebaseRawDescriptorPolicy(),old=readOwnCodebaseRawDescriptorPacket(),current=readCurrentOwnCodebaseRawDescriptorPacket();
  assert.equal(old.revision,'52132727d7ce158762a4499b7032a42a9f16f3c3');assert.equal(old.sourceOwner.current.bytes,133770);
  assert.equal(current.sources[0].bytes,133770);assert.deepEqual(pin(current.sources[0].sourceText),old.sourceOwner.current);
  assert.equal(restoreCurrentOwnCodebaseRawSource(readFileSync(new URL(policy.sourcePaths[0],root),'utf8'),policy.sourcePaths[0]),
    current.sources[0].sourceText);
  const files=[policy.artifactFile,policy.registerFile,policy.loanFile,'parity/self-translation/own-codebase-raw-source-descriptors.lino'];
  const before=files.map(file=>readFileSync(new URL(file,root),'utf8'));
  assert.throws(()=>generateOwnCodebaseRawSourceDescriptors(),/unknown complete source or artifact changes refuse/u);
  for(const[index,file]of files.entries())assert.equal(readFileSync(new URL(file,root),'utf8'),before[index]);
});

test('unknown complete sources aliases registers and semantic payloads cannot acquire predecessor authority',()=>{
  const packet=readCurrentOwnCodebaseRawDescriptorPacket(),policy=readCurrentOwnCodebaseRawDescriptorPolicy();
  const artifact=readProjectData(new URL(policy.artifactFile,root)),register=readProjectData(new URL(policy.registerFile,root));
  for(const mutate of [value=>{value.extra=true;},value=>{value[policy.property][0].changes.reverse();},
    value=>{value[policy.property][0].current.sha256='0'.repeat(64);},value=>{value.ownCodebaseRawSourceDescriptorSuccessors.reverse();value.ownCodebaseRawSourceDescriptorSuccessors.push({});}]){
    const changed=structuredClone(register);mutate(changed);assert.throws(()=>restoreCurrentOwnCodebaseRawEdge(artifact,'raw consuming',changed));
  }
  for(const mutate of [value=>{value.sources[0].current.sourceText+=' ';},value=>{value.sources[1].artifactPrevious.bytes+=1;},
    value=>{value.sources[0].sourceText+=' ';},value=>{value.adapters[1].edits[0].previousText+=' ';},
    value=>{value.artifact.sourceText+=' ';},value=>{value.inferenceDescriptorAlias=!value.inferenceDescriptorAlias;}]){
    const changed=structuredClone(packet);mutate(changed);assert.throws(()=>verifyCurrentOwnCodebaseRawDescriptorPacket(changed));
  }
  for(const mutate of [value=>{value.completeRust+=' ';},value=>{value.owners.reverse();},
    value=>{value.predecessor.proof.producerApplicability.sources[1].bytes+=1;},
    value=>{value.archive.textBytes+=1;},value=>{value.decisionSources[9].sha256='0'.repeat(64);}]){
    const changed=structuredClone(artifact);mutate(changed);
    assert.throws(()=>restoreCurrentOwnCodebaseRawText(serializeProjectData(changed,policy.artifactFile),policy.artifactFile));
    assert.throws(()=>restoreCompleteDeclarationReceipt(changed,'raw consuming',{register}));
  }
  for(const row of packet.sources)assert.throws(()=>restoreCurrentOwnCodebaseRawSource(row.current.sourceText+' ',row.file));
});
