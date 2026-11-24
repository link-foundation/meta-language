// One finite source-descriptor edge, before the unchanged historical chain.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData,serializeProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {restoreOwnCodebaseRawDescriptorEdge} from './own-codebase-raw-source-descriptors.mjs';
import {restoreReviewedOverlayProviderModuleSource} from '../../experiments/rust-text-certificate-adapter/provider-succession.mjs';
const root=new URL('../../',import.meta.url);
const policyFile='parity/self-translation/program-string-source-descriptors.lino';
const policySha256='d404eef323e602b5f31dc88b0588d4a8a4ff5b23af98f82e0ee62540b6b39741';
const revision='90636f9fb52b1b9095defbac041c8089608b8576';
const property='programStringRepresentationSourceSuccessors';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
let policy;

function readProgramStringSourceDescriptorPolicy() {
  if(!policy){const bytes=readFileSync(new URL(policyFile,root));assert.equal(pin(bytes).sha256,policySha256);policy=readProjectData(new URL(policyFile,root));assert.equal(policy.schemaVersion,1);assert.equal(policy.revision,revision);}
  return policy;
}

/** Remove precisely the appended edge and, for its actual new artifact, invert
 * only the two current source descriptor fields. Unknown artifacts stay visible
 * to the original strict gate rather than acquiring a historical identity. */
export function restoreProgramStringSourceDescriptorEdge(artifact,identity,register) {
  ({artifact,register}=restoreOwnCodebaseRawDescriptorEdge(artifact,identity,register));
  if(!Object.hasOwn(register,property))return{artifact,register};
  const selected=readProgramStringSourceDescriptorPolicy();assert.deepEqual(register[property],[selected.edge]);
  const bytes=Buffer.from(serializeProjectData(register,selected.register.file));assert.deepEqual(pin(bytes),selected.register.current);
  const previousRegister={...register};delete previousRegister[property];
  const originalRegister=Buffer.from(serializeProjectData(previousRegister,selected.register.file));
  assert.deepEqual(pin(originalRegister),{bytes:selected.register.bytes,sha256:selected.register.sha256});
  assert.equal(originalRegister.toString('utf8'),selected.register.sourceText);
  verifyGitArtifactCertificate(selected.register.certificate,originalRegister,{revision,path:selected.register.file});
  if(identity!=='raw consuming'||artifact===null)return{artifact,register:structuredClone(previousRegister)};
  const current=Buffer.from(serializeProjectData(artifact,selected.artifact.file));
  if(pin(current).sha256!==selected.edge.current.sha256)return{artifact,register:structuredClone(previousRegister)};
  assert.deepEqual(pin(current),selected.edge.current);
  const owner=selected.sourceOwner;
  const historicalOwner=restoreReviewedOverlayProviderModuleSource(readFileSync(new URL(owner.file,root),'utf8'),owner.file,'f73a10b22f8d0bcfbf4707621bcc9edd6989533d');
  assert.deepEqual(pin(historicalOwner),owner.current,'the actual checked-program projection source retains its complete certified historical view');
  verifyGitArtifactCertificate(owner.certificate,Buffer.from(owner.sourceText),{revision,path:owner.file});
  const restored=structuredClone(artifact);assert.equal(selected.edge.changes.length,2);
  const fields=new Set();
  for(const change of selected.edge.changes){
    assert.deepEqual(change.path.slice(0,-1),['decisionSources','11']);
    const field=change.path.at(-1);assert.ok(['bytes','sha256'].includes(field)&&!fields.has(field));fields.add(field);
    const descriptor=restored.decisionSources[11];assert.equal(descriptor.file,owner.file);
    assert.equal(descriptor[field],change.after);descriptor[field]=change.before;
  }
  const original=Buffer.from(serializeProjectData(restored,selected.artifact.file));
  assert.deepEqual(pin(original),{bytes:selected.artifact.bytes,sha256:selected.artifact.sha256});assert.equal(original.toString('utf8'),selected.artifact.sourceText);
  verifyGitArtifactCertificate(selected.artifact.certificate,original,{revision,path:selected.artifact.file});
  return{artifact:restored,register:structuredClone(previousRegister)};
}
