// One finite additional write predecessor. No current semantics or native run.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {serializeProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {CURRENT_RAW_PREDECESSOR_REVISION,CURRENT_RAW_PREDECESSOR_PIN,readCurrentRawPredecessorPacket} from './current-own-codebase-raw-source-history.mjs';
import {readCurrentOwnCodebaseRawDescriptorPolicy,verifyCurrentOwnCodebaseRawDescriptorStructure,
  deriveCurrentOwnCodebaseRawDescriptorOutputs} from './current-own-codebase-raw-source-descriptors.mjs';
const root=new URL('../../',import.meta.url);
export const ADDITIONAL_CURRENT_RAW_PREDECESSOR_REVISION='899de70f9c6dd74a13e0214dcfdcf4711f83a9ed';
export const ADDITIONAL_CURRENT_RAW_PREDECESSOR_OUTPUT='parity/self-translation/current-raw-descriptor-cohort-899de70f.lino';
export const ADDITIONAL_CURRENT_RAW_PREDECESSOR_PIN=Object.freeze({bytes:7602190,sha256:'db6a903d036aced419f35c1c09652b0115f4fc650358f06a31f3c7b190b16d66'});
export const ADDITIONAL_CURRENT_RAW_PREDECESSOR_BOUNDARY='One independently committed899de raw source-descriptor cohort is reconstructed from the unchanged complete8d packet. Only current/interval fields of the three declared source rows are supplied; existing normal derivation restores every entire packet/output byte and alias, checked against seven899de Git memberships. No large native artifact, register, loan or source packet is copied. This grants a finite normal-author preimage only, not current source identity, source/semantic replacement, native execution or permission to overwrite unknown bytes. All8d/a8 data and strict refusal controls remain required.';
const cohortSha256='dcc61ed3d7b98f9670a43a352ef225900e01f1a75caf80bfbd3792e39b0ff00b';
export const pinAdditionalCurrentRawPredecessor=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const keys=(value,names)=>assert.deepEqual(Object.keys(value).sort(),[...names].sort());
let cached;

/** Replace only complete current source facts. Every originalA8 artifact,
 * adapter, source, certificate, boundary and alias comes from the exact8d
 * archive; the existing normal author derives edge and output descriptors. */
export function deriveAdditionalCurrentRawPredecessor(history){
  keys(history,['schemaVersion','model','revision','base','packet','sources','outputs','boundary']);
  assert.equal(history.schemaVersion,1);assert.equal(history.model,'additional complete current raw descriptor cohort 1');
  assert.equal(history.revision,ADDITIONAL_CURRENT_RAW_PREDECESSOR_REVISION);
  assert.equal(history.boundary,ADDITIONAL_CURRENT_RAW_PREDECESSOR_BOUNDARY);
  assert.deepEqual(history.base,{revision:CURRENT_RAW_PREDECESSOR_REVISION,...CURRENT_RAW_PREDECESSOR_PIN});
  const policy=readCurrentOwnCodebaseRawDescriptorPolicy();
  keys(history.packet,['file','bytes','sha256','certificate']);assert.equal(history.packet.file,policy.output);
  assert.deepEqual({bytes:history.packet.bytes,sha256:history.packet.sha256},ADDITIONAL_CURRENT_RAW_PREDECESSOR_PIN);
  assert.deepEqual(history.sources.map(row=>row.file),policy.sourcePaths);
  assert.deepEqual(history.outputs.map(row=>row.file),[policy.artifactFile,policy.registerFile,policy.loanFile]);
  const packet=readCurrentRawPredecessorPacket();
  for(const[index,row]of history.sources.entries()){
    keys(row,['file','current','edits','certificate']);keys(row.current,['bytes','sha256','sourceText']);
    assert.equal(typeof row.current.sourceText,'string');assert.ok(row.current.bytes<=512*1024);
    assert.deepEqual(pinAdditionalCurrentRawPredecessor(row.current.sourceText),{bytes:row.current.bytes,sha256:row.current.sha256});
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.current.sourceText),{revision:history.revision,path:row.file});
    packet.sources[index].current=structuredClone(row.current);
    packet.sources[index].edits=structuredClone(row.edits);
  }
  const derived=deriveCurrentOwnCodebaseRawDescriptorOutputs(packet);
  packet.edge=derived.edge;
  for(const row of[packet.artifact,packet.register,packet.loan])row.current=pinAdditionalCurrentRawPredecessor(derived.outputs.find(output=>output.file===row.file).source);
  verifyCurrentOwnCodebaseRawDescriptorStructure(packet);
  const source=serializeProjectData(packet,policy.output);
  assert.deepEqual(pinAdditionalCurrentRawPredecessor(source),ADDITIONAL_CURRENT_RAW_PREDECESSOR_PIN,'the entire899de packet, not merely descriptor fields, is independently fixed');
  verifyGitArtifactCertificate(history.packet.certificate,Buffer.from(source),{revision:history.revision,path:policy.output});
  for(const row of history.outputs){
    keys(row,['file','bytes','sha256','certificate']);
    const output=derived.outputs.find(output=>output.file===row.file);assert.ok(output);
    assert.deepEqual(pinAdditionalCurrentRawPredecessor(output.source),{bytes:row.bytes,sha256:row.sha256});
    verifyGitArtifactCertificate(row.certificate,Buffer.from(output.source),{revision:history.revision,path:row.file});
    assert.deepEqual([packet.artifact,packet.register,packet.loan].find(input=>input.file===row.file).current,{bytes:row.bytes,sha256:row.sha256});
  }
  return{packet,source,outputs:derived.outputs};
}

export function verifyAdditionalCurrentRawPredecessorHistory(history){deriveAdditionalCurrentRawPredecessor(history);return history;}
function load(){
  if(!cached){
    assert.match(cohortSha256,/^[a-f0-9]{64}$/u,'normal cohort capture is required first');
    const bytes=readFileSync(new URL(ADDITIONAL_CURRENT_RAW_PREDECESSOR_OUTPUT,root));
    assert.equal(pinAdditionalCurrentRawPredecessor(bytes).sha256,cohortSha256,'the complete source-only cohort stays independently pinned');
    const history=readBootstrapStructuredDataLino(bytes.toString('utf8'));
    cached={history,...deriveAdditionalCurrentRawPredecessor(history)};
  }
  return cached;
}
export function readAdditionalCurrentRawPredecessorHistory(){return structuredClone(load().history);}
export function readAdditionalCurrentRawPredecessorPacket(){return structuredClone(load().packet);}
export function readAdditionalCurrentRawPredecessorOutputs(){return structuredClone(load().outputs);}
export function acceptAdditionalCurrentRawPredecessorOutput(source,file){
  assert.equal(typeof source,'string');assert.equal(typeof file,'string');
  const policy=readCurrentOwnCodebaseRawDescriptorPolicy();
  if(![policy.artifactFile,policy.registerFile,policy.loanFile].includes(file))return false;
  return load().outputs.some(row=>row.file===file&&row.source===source);
}
export function acceptAdditionalCurrentRawPredecessorPacketSource(source){
  assert.equal(typeof source,'string');
  if(Buffer.byteLength(source)!==ADDITIONAL_CURRENT_RAW_PREDECESSOR_PIN.bytes||pinAdditionalCurrentRawPredecessor(source).sha256!==ADDITIONAL_CURRENT_RAW_PREDECESSOR_PIN.sha256)return false;
  return source===load().source;
}
