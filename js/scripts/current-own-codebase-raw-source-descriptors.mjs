// One finite provenance edge. No compiler, translator or native process runs.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {serializeProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {readCurrentRawPredecessorHistory,readCurrentRawPredecessorPacket} from './current-own-codebase-raw-source-history.mjs';
import {restoreTypedSourceDescriptorHostSource} from './typed-source-descriptor-host-history.mjs';
const root=new URL('../../',import.meta.url);
const policySha256='26646438286157ed8250c4a8e631521ceb39281d95c8766700235d6cc62b6553';
const revision='a8a789afb0ed569340d9079279f534f0bf7c4e72';
const property='currentOwnCodebaseRawSourceDescriptorSuccessors';
const output='parity/self-translation/current-own-codebase-raw-source-descriptors.lino';
const artifactFile='experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino';
const registerFile='parity/self-translation/complete-declaration-receipts.lino';
const loanFile='experiments/original-link-complete-table-projection/data/raw-borrowed-parameter-receipt.lino';
const sourcePaths=['js/src/translation/javascript.js','js/src/translation/javascript-infer.js','js/src/typed-program-link-rules.js'];
const adapterPaths=['js/scripts/own-codebase-raw-source-descriptors.mjs','js/scripts/generate-own-codebase-raw-source-descriptors.mjs','js/tests/translation-own-codebase-raw-source-descriptors.test.js'];
const decisionIndices=[1,2,9];
const semanticFields=['completeRust','owners','declarationNames','totals','assumptions','encodings','mappings','preludeHashes','replays','archive'];
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const exactKeys=(value,keys)=>assert.deepEqual(Object.keys(value).sort(),keys.slice().sort());
const endpoint=(value,path)=>{let owner=value;for(const key of path.slice(0,-1)){assert.ok(Object.hasOwn(owner,key));owner=owner[key];}return[owner,path.at(-1)];};
let cached;

export function readCurrentOwnCodebaseRawDescriptorPolicy(){return{schemaVersion:1,model:'current own codebase raw source descriptor successor 1',revision,property,output,artifactFile,registerFile,loanFile,sourcePaths:[...sourcePaths],adapterPaths:[...adapterPaths],decisionIndices:[...decisionIndices]};}

/** Ordered byte intervals are checked forward and backward against whole files. */
export function deriveCurrentOwnCodebaseRawDescriptorSource(row){
  assert.equal(typeof row.sourceText,'string');assert.equal(typeof row.current.sourceText,'string');
  assert.deepEqual(pin(row.sourceText),{bytes:row.bytes,sha256:row.sha256});
  let bytes=Buffer.from(row.sourceText),limit=bytes.length;
  for(const edit of row.edits.toReversed()){
    assert.ok(Number.isSafeInteger(edit.start)&&Number.isSafeInteger(edit.end)&&edit.start>=0&&edit.start<=edit.end&&edit.end<=limit);
    assert.equal(bytes.subarray(edit.start,edit.end).toString('utf8'),edit.previousText);
    bytes=Buffer.concat([bytes.subarray(0,edit.start),Buffer.from(edit.currentText),bytes.subarray(edit.end)]);limit=edit.start;
  }
  assert.equal(bytes.toString('utf8'),row.current.sourceText);assert.deepEqual(pin(bytes),{bytes:row.current.bytes,sha256:row.current.sha256});
  limit=bytes.length;
  for(const edit of row.edits.toReversed()){
    assert.ok(Number.isSafeInteger(edit.currentStart)&&Number.isSafeInteger(edit.currentEnd)&&edit.currentStart>=0&&edit.currentStart<=edit.currentEnd&&edit.currentEnd<=limit);
    assert.equal(bytes.subarray(edit.currentStart,edit.currentEnd).toString('utf8'),edit.currentText);
    bytes=Buffer.concat([bytes.subarray(0,edit.currentStart),Buffer.from(edit.previousText),bytes.subarray(edit.currentEnd)]);limit=edit.currentStart;
  }
  assert.equal(bytes.toString('utf8'),row.sourceText,'the entire certified source predecessor survives');
  return row.current.sourceText;
}

/** These are the only admitted descriptor locations; two may alias one object. */
function descriptorTargets(value){
  const proof=value.predecessor.proof.producerApplicability;
  const index=proof.sources.findIndex(row=>row.file===sourcePaths[1]);
  assert.ok(index>=0);assert.equal(proof.sources.filter(row=>row.file===sourcePaths[1]).length,1);
  assert.equal(proof.sourceHistory.revision,'81107a5945651f17c962e4bb81fb560f96c65ccd');
  assert.equal(proof.sourceHistory.current.file,sourcePaths[1]);
  assert.deepEqual(proof.sources[index],proof.sourceHistory.current);
  return{alias:proof.sources[index]===proof.sourceHistory.current,paths:[
    ...decisionIndices.map(index=>['decisionSources',String(index)]),
    ['predecessor','proof','producerApplicability','sources',String(index)],
    ['predecessor','proof','producerApplicability','sourceHistory','current'],
  ]};
}

/** Mutate each actual property once, while verifying every logical alias path. */
function apply(value,changes,direction){
  const seen=new WeakMap();
  for(const change of changes){
    const[owner,key]=endpoint(value,change.path),from=change[direction==='forward'?'before':'after'],to=change[direction==='forward'?'after':'before'];
    let fields=seen.get(owner);if(!fields){fields=new Set();seen.set(owner,fields);}
    if(fields.has(key)){assert.deepEqual(owner[key],to,'a shared physical record was already changed exactly once');continue;}
    assert.deepEqual(owner[key],from);owner[key]=to;fields.add(key);
  }
}

/** Derive one additive edge from whole certified inputs, without emitting code. */
export function deriveCurrentOwnCodebaseRawDescriptorOutputs(packet){
  const artifact=readStructuredDataLino(packet.artifact.sourceText),before=structuredClone(artifact);
  assert.equal(artifact.definitionCount,106);assert.equal(artifact.owners.length,106);
  const targets=descriptorTargets(artifact),changes=[];
  assert.equal(targets.alias,packet.inferenceDescriptorAlias);
  for(const[index,path]of targets.paths.entries()){
    const ownerIndex=index<3?index:1,row=packet.sources[ownerIndex];
    const[owner,key]=endpoint(artifact,path),descriptor=owner[key];
    exactKeys(descriptor,['file','bytes','sha256']);assert.equal(descriptor.file,row.file);
    if(index<3)assert.deepEqual(descriptor,row.artifactPrevious);
    for(const field of ['bytes','sha256'])changes.push({path:[...path,field],before:descriptor[field],after:row.current[field]});
  }
  apply(artifact,changes,'forward');assert.equal(descriptorTargets(artifact).alias,targets.alias);
  for(const field of semanticFields)assert.deepEqual(artifact[field],before[field],field);
  const artifactText=serializeProjectData(artifact,artifactFile),inverse=structuredClone(artifact);
  apply(inverse,changes,'inverse');assert.equal(descriptorTargets(inverse).alias,targets.alias);
  assert.equal(serializeProjectData(inverse,artifactFile),packet.artifact.sourceText,'every prior artifact byte survives');
  const edge={identity:'raw consuming',sourceRevision:revision,previous:{bytes:packet.artifact.bytes,sha256:packet.artifact.sha256},current:pin(artifactText),
    sources:packet.sources.map(row=>({file:row.file,artifactPrevious:row.artifactPrevious,checkedPrevious:{revision,bytes:row.bytes,sha256:row.sha256},current:{bytes:row.current.bytes,sha256:row.current.sha256}})),
    inferenceDescriptorAlias:targets.alias,changes};
  const register=readStructuredDataLino(packet.register.sourceText);assert.equal(Object.hasOwn(register,property),false);
  register[property]=[edge];const registerText=serializeProjectData(register,registerFile);
  delete register[property];assert.equal(serializeProjectData(register,registerFile),packet.register.sourceText,'all older register edges survive');
  const loan=readStructuredDataLino(packet.loan.sourceText),previous=structuredClone(loan.predecessor);
  assert.deepEqual({bytes:previous.bytes,sha256:previous.sha256},edge.previous);
  Object.assign(loan.predecessor,edge.current);const loanText=serializeProjectData(loan,loanFile);
  loan.predecessor=previous;assert.equal(serializeProjectData(loan,loanFile),packet.loan.sourceText,'the entire borrowed receipt survives');
  return{edge,outputs:[{file:artifactFile,source:artifactText},{file:registerFile,source:registerText},{file:loanFile,source:loanText}]};
}

export function verifyCurrentOwnCodebaseRawDescriptorPacket(packet){
  // Only the initial normal author has no successor packet yet. Installed
  // readers never admit PENDING (load independently checks the whole digest).
  // A later caller cannot coherently replace current pins or source intervals.
  if(policySha256!=='PENDING')assert.equal(pin(serializeProjectData(packet,output)).sha256,policySha256,'the entire reviewed current packet stays exact');
  const result=verifyCurrentOwnCodebaseRawDescriptorStructure(packet);
  readCurrentRawPredecessorHistory();
  return result;
}

/** Certificate/structure proof only. This does not grant current-packet identity. */
export function verifyCurrentOwnCodebaseRawDescriptorStructure(packet){
  exactKeys(packet,['schemaVersion','model','revision','artifact','register','loan','sources','adapters','inferenceDescriptorAlias','edge','boundary']);
  const policy=readCurrentOwnCodebaseRawDescriptorPolicy();for(const key of ['schemaVersion','model','revision'])assert.equal(packet[key],policy[key]);
  assert.deepEqual(packet.sources.map(row=>row.file),sourcePaths);assert.deepEqual(packet.adapters.map(row=>row.file),adapterPaths);
  const rows=[packet.artifact,packet.register,packet.loan,...packet.sources,...packet.adapters];
  assert.deepEqual(rows.slice(0,3).map(row=>row.file),[artifactFile,registerFile,loanFile]);
  for(const row of rows){
    assert.equal(row.commit,revision);assert.equal(typeof row.sourceText,'string');
    assert.deepEqual(pin(row.sourceText),{bytes:row.bytes,sha256:row.sha256});
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision,path:row.file});
  }
  for(const row of [...packet.sources,...packet.adapters])deriveCurrentOwnCodebaseRawDescriptorSource(row);
  const result=deriveCurrentOwnCodebaseRawDescriptorOutputs(packet);assert.deepEqual(packet.edge,result.edge);
  for(const row of result.outputs){const capture=rows.find(input=>input.file===row.file);assert.deepEqual(capture.current,pin(row.source));}
  return packet;
}

/** Normal author admission for one new current view, retaining all old versions.
 * Installed readers still require their independent whole packet digest above. */
export function verifyCurrentOwnCodebaseRawDescriptorRefresh(packet){
  verifyCurrentOwnCodebaseRawDescriptorStructure(packet);
  const previous=readCurrentRawPredecessorPacket();
  for(const field of ['schemaVersion','model','revision','inferenceDescriptorAlias','boundary'])assert.deepEqual(packet[field],previous[field]);
  for(const role of ['artifact','register','loan']){
    const current={...packet[role]},old={...previous[role]};delete current.current;delete old.current;
    assert.deepEqual(current,old,'every complete original source and certificate stays exact: '+role);
  }
  for(const[index,row]of packet.sources.entries()){
    const current={...row},old={...previous.sources[index]};delete current.current;delete current.edits;delete old.current;delete old.edits;
    assert.deepEqual(current,old,'each complete A8 source and artifact descriptor remains unchanged');
    exactKeys(row.current,['bytes','sha256','sourceText']);
    for(const interval of row.edits)exactKeys(interval,['start','end','previousText','currentStart','currentEnd','currentText']);
    assert.equal(readFileSync(new URL(row.file,root),'utf8'),row.current.sourceText,'normal refresh uses exact actual current source bytes');
  }
  assert.deepEqual(packet.adapters,previous.adapters,'the complete three original adapter recipes remain exact');
  for(const row of packet.adapters)assert.equal(restoreTypedSourceDescriptorHostSource(readFileSync(new URL(row.file,root),'utf8'),row.file),deriveCurrentOwnCodebaseRawDescriptorSource(row));
  assert.equal(packet.edge.changes.length,previous.edge.changes.length,'all ten logical descriptor fields remain represented');
  return packet;
}

function load(){
  if(!cached){const bytes=readFileSync(new URL(output,root));assert.equal(pin(bytes).sha256,policySha256,'the entire finite reviewed packet is pinned');cached=verifyCurrentOwnCodebaseRawDescriptorPacket(readBootstrapStructuredDataLino(bytes.toString('utf8')));}
  return cached;
}
export function readCurrentOwnCodebaseRawDescriptorPacket(){return structuredClone(load());}

/** Historical assertions get an exact whole source view, never arbitrary input. */
export function restoreCurrentOwnCodebaseRawSource(source,file,currentOnly=false){
  assert.equal(typeof source,'string');const row=load().sources.find(row=>row.file===file);assert.ok(row,'one exact source path');
  if(source===row.sourceText)return source;
  if(currentOnly&&pin(source).sha256!==row.current.sha256)return source;
  assert.deepEqual(pin(source),{bytes:row.current.bytes,sha256:row.current.sha256});assert.equal(source,row.current.sourceText);
  return row.sourceText;
}

function restore(source,file,packet){
  assert.equal(typeof source,'string');const row=[packet.artifact,packet.register,packet.loan].find(row=>row.file===file);assert.ok(row,'one exact artifact path');
  if(source===row.sourceText)return source;
  assert.deepEqual(pin(source),row.current,'an unknown whole artifact cannot acquire historical authority');
  const value=readStructuredDataLino(source);
  if(file===artifactFile){assert.equal(descriptorTargets(value).alias,packet.inferenceDescriptorAlias);apply(value,packet.edge.changes,'inverse');}
  else if(file===registerFile){assert.equal(Object.keys(value).at(-1),property);assert.deepEqual(value[property],[packet.edge]);delete value[property];}
  else{assert.deepEqual({bytes:value.predecessor.bytes,sha256:value.predecessor.sha256},packet.edge.current);Object.assign(value.predecessor,packet.edge.previous);}
  const previous=serializeProjectData(value,file);assert.equal(previous,row.sourceText,'the exact whole certified predecessor is recovered');return previous;
}
export function restoreCurrentOwnCodebaseRawText(source,file){return restore(source,file,load());}
export function restoreCurrentOwnCodebaseRawData(value,file){return readStructuredDataLino(restore(serializeProjectData(value,file),file,load()));}

/** One outer boundary; old APIs, policies and packets retain their identities. */
export function restoreCurrentOwnCodebaseRawEdge(artifact,identity,register){
  if(!Object.hasOwn(register,property))return{artifact,register};
  const packet=load(),previousRegister=readStructuredDataLino(restore(serializeProjectData(register,registerFile),registerFile,packet));
  if(identity!=='raw consuming'||artifact===null)return{artifact,register:previousRegister};
  const source=serializeProjectData(artifact,artifactFile);
  if(pin(source).sha256!==packet.edge.current.sha256)return{artifact,register:previousRegister};
  for(const row of packet.sources)assert.deepEqual(pin(readFileSync(new URL(row.file,root))),{bytes:row.current.bytes,sha256:row.current.sha256},'actual current source remains independently pinned: '+row.file);
  return{artifact:readStructuredDataLino(restore(source,artifactFile,packet)),register:previousRegister};
}
