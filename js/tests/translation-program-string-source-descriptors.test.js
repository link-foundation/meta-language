import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readProjectData,serializeProjectData} from '../scripts/project-data.mjs';
import {generateProgramStringSourceDescriptors} from '../scripts/generate-program-string-source-descriptors.mjs';
import {restoreProgramStringSourceDescriptorEdge} from '../scripts/program-string-source-descriptors.mjs';
import {verifyGitArtifactCertificate} from '../scripts/git-object-certificate.mjs';
import {restoreOwnCodebaseRawDescriptorEdge,restoreOwnCodebaseRawDescriptorText,restoreOwnCodebaseRawDescriptorData} from '../scripts/own-codebase-raw-source-descriptors.mjs';
const root=new URL('../../',import.meta.url);

test('the current program-string source descriptor edge retains every original artifact register and borrowed receipt byte',()=>{
  const packet=generateProgramStringSourceDescriptors({check:true});
  const {artifact,register}=restoreOwnCodebaseRawDescriptorEdge(readProjectData(new URL(packet.artifact.file,root)),'raw consuming',readProjectData(new URL(packet.register.file,root)));
  const restored=restoreProgramStringSourceDescriptorEdge(artifact,'raw consuming',register);
  assert.equal(serializeProjectData(restored.artifact,packet.artifact.file),packet.artifact.sourceText);
  assert.equal(serializeProjectData(restored.register,packet.register.file),packet.register.sourceText);
  for(const row of [packet.artifact,packet.register,packet.loan,packet.sourceOwner])verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:packet.revision,path:row.file});
  assert.equal(packet.edge.changes.length,2);assert.equal(artifact.definitionCount,106);
  for(const field of ['completeRust','owners','declarationNames','totals','assumptions','encodings','mappings','preludeHashes','replays','archive'])assert.deepEqual(artifact[field],restored.artifact[field],field);
  const loan=restoreOwnCodebaseRawDescriptorData(readProjectData(new URL(packet.loan.file,root)),packet.loan.file),original=readProjectData(new URL('parity/self-translation/program-string-source-descriptors.lino',root));
  assert.deepEqual({bytes:loan.predecessor.bytes,sha256:loan.predecessor.sha256},packet.edge.current);
  const old=JSON.parse(JSON.stringify(loan));Object.assign(old.predecessor,packet.edge.previous);
  assert.equal(serializeProjectData(old,packet.loan.file),original.loan.sourceText);
  assert.equal(serializeProjectData(artifact,packet.artifact.file),restoreOwnCodebaseRawDescriptorText(readFileSync(new URL(packet.artifact.file,root),'utf8'),packet.artifact.file));
});

test('changed descriptor ownership register fields edge order and caller mutations cannot enter the historical source chain',()=>{
  const packet=readProjectData(new URL('parity/self-translation/program-string-source-descriptors.lino',root));
  const {artifact,register}=restoreOwnCodebaseRawDescriptorEdge(readProjectData(new URL(packet.artifact.file,root)),'raw consuming',readProjectData(new URL(packet.register.file,root)));
  for(const mutate of [
    value=>{value.programStringRepresentationSourceSuccessors[0].changes.reverse();},
    value=>{value.programStringRepresentationSourceSuccessors[0].changes[0].path[1]='10';},
    value=>{value.programStringRepresentationSourceSuccessors[0].current.sha256='0'.repeat(64);},
    value=>{value.extra='unknown';},
  ]){const changed=structuredClone(register);mutate(changed);assert.throws(()=>restoreProgramStringSourceDescriptorEdge(artifact,'raw consuming',changed));}
  const before=serializeProjectData(register,packet.register.file),returned=restoreProgramStringSourceDescriptorEdge(artifact,'raw consuming',register);
  returned.register.receipts[0].identity='caller edit';returned.artifact.completeRust+=' caller edit';
  assert.equal(serializeProjectData(register,packet.register.file),before);
  assert.equal(serializeProjectData(restoreProgramStringSourceDescriptorEdge(artifact,'raw consuming',register).artifact,packet.artifact.file),packet.artifact.sourceText);
  const changed=structuredClone(artifact);changed.completeRust+=' unknown edit';
  assert.equal(restoreProgramStringSourceDescriptorEdge(changed,'raw consuming',register).artifact,changed,'unknown artifact bytes remain visible to the unchanged strict gate');
});
