// Complete independently certified8d source/version archive; no native execution.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {acceptAdditionalCurrentRawPredecessorOutput,acceptAdditionalCurrentRawPredecessorPacketSource} from './current-raw-descriptor-cohort.mjs';
import {readCurrentOwnCodebaseRawDescriptorPolicy,verifyCurrentOwnCodebaseRawDescriptorStructure,
  deriveCurrentOwnCodebaseRawDescriptorOutputs} from './current-own-codebase-raw-source-descriptors.mjs';
const root=new URL('../../',import.meta.url);
export const CURRENT_RAW_PREDECESSOR_REVISION='8d77e476d097448eb381ebe03e2aa173402bc03d';
export const CURRENT_RAW_PREDECESSOR_OUTPUT='parity/self-translation/current-own-codebase-raw-source-history.lino';
export const CURRENT_RAW_PREDECESSOR_ARCHIVE='parity/self-translation/current-own-codebase-raw-source-descriptors-8d77e476.lino';
export const CURRENT_RAW_PREDECESSOR_PIN=Object.freeze({bytes:7580712,sha256:'c70826340739bdda2eef7e306271109c017efaf624a2a85239a4f57897cd55d5'});
const historySha256='09edc0f02df13c03e2ef603e2dc4b56af0e06fb348ad9c1e80efd38dc006ef13';
const exactKeys=(value,keys)=>{assert.ok(value&&typeof value==='object'&&!Array.isArray(value));assert.deepEqual(Object.keys(value).sort(),[...keys].sort());};
export const pinCurrentRawPredecessorSource=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
let cached;

/** The large archived document is retained verbatim exactly once. Its physical
 * outputs are reconstructed from it and checked against independent8d Git blobs. */
export function verifyCurrentRawPredecessorHistory(history,archiveBytes=readFileSync(new URL(CURRENT_RAW_PREDECESSOR_ARCHIVE,root))){
  exactKeys(history,['schemaVersion','model','revision','archive','outputs','sources','boundary']);
  assert.equal(history.schemaVersion,1);assert.equal(history.model,'complete prior current raw source descriptors 1');
  assert.equal(history.revision,CURRENT_RAW_PREDECESSOR_REVISION);
  const policy=readCurrentOwnCodebaseRawDescriptorPolicy();
  exactKeys(history.archive,['path','file','bytes','sha256','certificate']);
  assert.equal(history.archive.path,CURRENT_RAW_PREDECESSOR_ARCHIVE);assert.equal(history.archive.file,policy.output);
  assert.deepEqual({bytes:history.archive.bytes,sha256:history.archive.sha256},CURRENT_RAW_PREDECESSOR_PIN);
  assert.deepEqual(pinCurrentRawPredecessorSource(archiveBytes),CURRENT_RAW_PREDECESSOR_PIN);
  verifyGitArtifactCertificate(history.archive.certificate,archiveBytes,{revision:CURRENT_RAW_PREDECESSOR_REVISION,path:policy.output});
  const packet=readBootstrapStructuredDataLino(archiveBytes.toString('utf8'));
  verifyCurrentOwnCodebaseRawDescriptorStructure(packet);
  const derived=deriveCurrentOwnCodebaseRawDescriptorOutputs(packet);assert.deepEqual(packet.edge,derived.edge);
  assert.equal(packet.inferenceDescriptorAlias,derived.edge.inferenceDescriptorAlias);
  assert.deepEqual(history.outputs.map(row=>row.file),[policy.artifactFile,policy.registerFile,policy.loanFile]);
  assert.deepEqual(history.sources.map(row=>row.file),policy.sourcePaths);
  for(const row of [...history.outputs,...history.sources]){
    exactKeys(row,['file','bytes','sha256','certificate']);
    const source=history.outputs.includes(row)?derived.outputs.find(output=>output.file===row.file).source
      :packet.sources.find(source=>source.file===row.file).current.sourceText;
    assert.deepEqual(pinCurrentRawPredecessorSource(source),{bytes:row.bytes,sha256:row.sha256});
    verifyGitArtifactCertificate(row.certificate,Buffer.from(source),{revision:CURRENT_RAW_PREDECESSOR_REVISION,path:row.file});
  }
  for(const row of history.outputs){
    const original=[packet.artifact,packet.register,packet.loan].find(input=>input.file===row.file);
    assert.deepEqual(original.current,{bytes:row.bytes,sha256:row.sha256});
  }
  return{packet,outputs:derived.outputs};
}

function load(){
  if(!cached){
    const bytes=readFileSync(new URL(CURRENT_RAW_PREDECESSOR_OUTPUT,root));
    assert.equal(pinCurrentRawPredecessorSource(bytes).sha256,historySha256);
    const history=readBootstrapStructuredDataLino(bytes.toString('utf8'));
    cached={history,...verifyCurrentRawPredecessorHistory(history)};
  }
  return cached;
}

export function readCurrentRawPredecessorHistory(){return structuredClone(load().history);}
export function readCurrentRawPredecessorPacket(){return structuredClone(load().packet);}
export function readCurrentRawPredecessorOutputs(){return structuredClone(load().outputs);}

/** An author may replace only the three exact certified prior current outputs.
 * No other path or even one unreviewed byte obtains this admission. */
export function acceptCurrentRawPredecessorOutput(source,file){
  assert.equal(typeof source,'string');assert.equal(typeof file,'string');
  return load().outputs.some(row=>row.file===file&&row.source===source)||acceptAdditionalCurrentRawPredecessorOutput(source,file);
}

export function acceptCurrentRawPredecessorPacketSource(source){
  assert.equal(typeof source,'string');load();
  const bytes=Buffer.from(source);return bytes.length===CURRENT_RAW_PREDECESSOR_PIN.bytes
    &&pinCurrentRawPredecessorSource(bytes).sha256===CURRENT_RAW_PREDECESSOR_PIN.sha256
    ||acceptAdditionalCurrentRawPredecessorPacketSource(source);
}
