import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData,serializeProjectData} from '../scripts/project-data.mjs';
import {generateOwnCodebaseRawSourceDescriptors} from '../scripts/generate-own-codebase-raw-source-descriptors.mjs';
import {readOwnCodebaseRawDescriptorPacket,readOwnCodebaseRawDescriptorPolicy,deriveOwnCodebaseRawDescriptorSource,verifyOwnCodebaseRawDescriptorPacket,restoreOwnCodebaseRawDescriptorEdge,restoreOwnCodebaseRawDescriptorText,restoreOwnCodebaseRawDescriptorData} from '../scripts/own-codebase-raw-source-descriptors.mjs';
import {restoreCompleteDeclarationReceipt} from '../scripts/complete-declaration-receipts.mjs';
import {restoreCurrentOwnCodebaseRawSource,restoreCurrentOwnCodebaseRawData} from '../scripts/current-own-codebase-raw-source-descriptors.mjs';
const root=new URL('../../',import.meta.url);
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});

test('current frontend provenance keeps every original artifact source archive and historical assertion byte',()=>{
  const packet=generateOwnCodebaseRawSourceDescriptors({check:true}),policy=readOwnCodebaseRawDescriptorPolicy();
  assert.equal(packet.sourceOwner.bytes,133676);assert.equal(packet.sourceOwner.current.bytes,133770);
  assert.equal(packet.edge.source.artifactPrevious.bytes,130416);
  assert.deepEqual(packet.edge.source.checkedPrevious,{revision:packet.revision,...policy.sourceOwner.predecessor});
  const source=deriveOwnCodebaseRawDescriptorSource(policy.sourceOwner,packet.sourceOwner.sourceText);
  assert.equal(source,restoreCurrentOwnCodebaseRawSource(readFileSync(new URL(policy.sourceFile,root),'utf8'),policy.sourceFile));assert.deepEqual(pin(source),policy.sourceOwner.current);
  for(const [index,row]of packet.adapters.entries())assert.equal(deriveOwnCodebaseRawDescriptorSource(policy.adapters[index],row.sourceText),readFileSync(new URL(row.file,root),'utf8'));
  const artifact=readProjectData(new URL(packet.artifact.file,root)),register=readProjectData(new URL(packet.register.file,root));
  const before=serializeProjectData(artifact,packet.artifact.file),restored=restoreOwnCodebaseRawDescriptorEdge(artifact,'raw consuming',register);
  assert.equal(serializeProjectData(restored.artifact,packet.artifact.file),packet.artifact.sourceText);
  assert.equal(serializeProjectData(restored.register,packet.register.file),packet.register.sourceText);
  assert.equal(serializeProjectData(artifact,packet.artifact.file),before,'the caller graph is unchanged');
  for(const field of ['completeRust','owners','declarationNames','totals','assumptions','encodings','mappings','preludeHashes','replays','archive'])assert.deepEqual(artifact[field],restored.artifact[field],field);
  const loan=restoreCurrentOwnCodebaseRawData(readProjectData(new URL(packet.loan.file,root)),packet.loan.file);assert.deepEqual({bytes:loan.predecessor.bytes,sha256:loan.predecessor.sha256},packet.edge.current);
  assert.equal(serializeProjectData(restoreOwnCodebaseRawDescriptorData(loan,packet.loan.file),packet.loan.file),packet.loan.sourceText);
  for(const row of [packet.artifact,packet.register,packet.loan])assert.equal(restoreOwnCodebaseRawDescriptorText(readFileSync(new URL(row.file,root),'utf8'),row.file),row.sourceText);
  assert.doesNotThrow(()=>restoreCompleteDeclarationReceipt(artifact,'raw consuming',{register}));
});

test('changed descriptor owners current source pins registers and complete artifacts cannot acquire historical authority',()=>{
  const packet=readOwnCodebaseRawDescriptorPacket(),artifact=readProjectData(new URL(packet.artifact.file,root)),register=readProjectData(new URL(packet.register.file,root));
  for(const mutate of [value=>{value.extra='unknown';},value=>{value.ownCodebaseRawSourceDescriptorSuccessors[0].changes.reverse();},value=>{value.ownCodebaseRawSourceDescriptorSuccessors[0].current.sha256='0'.repeat(64);}]){
    const changed=structuredClone(register);mutate(changed);assert.throws(()=>restoreOwnCodebaseRawDescriptorEdge(artifact,'raw consuming',changed));
  }
  for(const mutate of [value=>{value.sourceOwner.sourceText+=' ';},value=>{value.sourceOwner.current.sha256='0'.repeat(64);},value=>{value.artifact.sourceText+=' ';},value=>{value.adapters[0].edits[0].previousText+=' ';}]){
    const changed=structuredClone(packet);mutate(changed);assert.throws(()=>verifyOwnCodebaseRawDescriptorPacket(changed));
  }
  const changed=structuredClone(artifact);changed.completeRust+=' unknown';
  assert.equal(restoreOwnCodebaseRawDescriptorEdge(changed,'raw consuming',register).artifact,changed,'unknown payload stays visible to the strict original gate');
  assert.throws(()=>restoreCompleteDeclarationReceipt(changed,'raw consuming',{register}));
  assert.throws(()=>restoreOwnCodebaseRawDescriptorText(readFileSync(new URL(packet.artifact.file,root),'utf8')+' ',packet.artifact.file));
});
