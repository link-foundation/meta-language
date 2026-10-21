// A finite metadata-only source view shared by two immutable historical readers.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {serializeProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
const root=new URL('../../',import.meta.url);
export const RAW_DESCRIPTOR_HISTORICAL_FILE='experiments/original-link-complete-table-projection/data/raw-consuming-current-native-definitions.lino';
export const RAW_DESCRIPTOR_HISTORICAL_OUTPUT='parity/self-translation/raw-descriptor-historical-cohort.lino';
export const RAW_DESCRIPTOR_HISTORICAL_REVISION='c33d6e0af666d3b679b5ca420917f5cafb347613';
export const RAW_DESCRIPTOR_HISTORICAL_ARCHIVE='parity/self-translation/library-relocation-receipt-history.lino';
export const RAW_DESCRIPTOR_HISTORICAL_ARCHIVE_PIN=Object.freeze({bytes:4537895,sha256:'a467a8ba8fcfeba0103e8280192d3e76f9947fde8765ae605bce6f47e8fb9e21'});
export const RAW_DESCRIPTOR_HISTORICAL_PREVIOUS=Object.freeze({bytes:1816560,sha256:'3f507a16b9c1c18110ae551273f053c552893b5b3ad3bff9fec50fc065369af4'});
export const RAW_DESCRIPTOR_HISTORICAL_CURRENT=Object.freeze({bytes:1816560,sha256:'bcfee09474c6d8922f59da87fb5a33d89c02e4bdb266095469396fec464fea77'});
export const RAW_DESCRIPTOR_HISTORICAL_RELOCATION=Object.freeze({bytes:1816560,sha256:'856a9b6139960222c160b98655e192af0097df2f8aa6e251887e29fe53d729bb'});
export const RAW_DESCRIPTOR_BORROWED_FILE='experiments/original-link-complete-table-projection/data/raw-borrowed-parameter-receipt.lino';
export const RAW_DESCRIPTOR_CONTINUATION_ARCHIVE='parity/self-translation/ci-39389240046-historical-inputs.lino';
export const RAW_DESCRIPTOR_CONTINUATION_ARCHIVE_PIN=Object.freeze({bytes:10759631,sha256:'05e17bf81c6304f36411073ee4853798041e2764164c999ca93888870fb61560'});
export const RAW_DESCRIPTOR_BORROWED_PREVIOUS=Object.freeze({bytes:688930,sha256:'9eba88d1db3eeff606b1eb6101a0e35d7bd2990ffd27009d8fd7a5c45715f4ca'});
export const RAW_DESCRIPTOR_BORROWED_CURRENT=Object.freeze({bytes:688930,sha256:'badf417fe1a838ce0b001dc89cdb21793713c1741ea63ccfdf08a9bf73e08903'});

const packetSha256='38721b3d7376f0b4b98337a1f7567caac8e41116d6aa45c9d02bfd59ae7e3a6c';
export const pinRawDescriptorHistoricalSource=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const exactKeys=(value,names)=>{assert.ok(value&&typeof value==='object'&&!Array.isArray(value));assert.deepEqual(Object.keys(value).sort(),[...names].sort());};
const scalarFacts=[
  [228,62588,62902],
  [230,'4a6f2db0a3c620d82cdea5048c013722070da9ebbe722d8c249b5197b63f1165','f87fa8dcb3362165869d8f4678fd014f13c057eee92243d1489297613c66972d'],
  [305,'0aabd771f2321b4a48f7f807754b33dbbb7ec7262ca98f3bfe812807bd9b4cfc','36dbd7ffcf25a403deab81f489eb9c3ff5e69cbf7f084ea8d9c34a279b2ae626'],
  [418,130416,138343],
  [420,'38d155ce765157039d2555c070e537e97a91f2c64587c70b6c6c578c5371ebe8','48442a9034ab8f603ac69ff0239b5bbb97a9477081798d620bf876bc2faef290'],
  [468,18923,19336],
  [470,'fbffe8ff099b43b8f23fd5c35a0dda83ab711db4735ef3dc1a8239a87d5e5110','d04c88f6e6116e255eb2f5ac924ea0d6a1d3dc3da25c53d732025bc3bf5c48b9'],
  [482,10830,11211],
  [484,'fb2636a04a45c38c5dda0d99c239eb6dc97f5d3d441245f6855fde22789e3691','5d224a630dbb3d55582648a888482144331e2bad31d67a2fe05d4cb34ca601da'],
];
const line=(id,value)=>'('+id+': (meta: (t: semantic) (n: 1) (term: '+(typeof value==='number'?'number':'string')+'%20value) (def: '+String(value)+') (lang: meta-language%20structured%20data)))';
let authority,borrowedAuthority,retained;

/** The complete old payload and its certificate already belong to this archive. */
export function readRawDescriptorHistoricalAuthority(){
  if(!authority){
    const bytes=readFileSync(new URL(RAW_DESCRIPTOR_HISTORICAL_ARCHIVE,root));
    assert.deepEqual(pinRawDescriptorHistoricalSource(bytes),RAW_DESCRIPTOR_HISTORICAL_ARCHIVE_PIN);
    const archive=readBootstrapStructuredDataLino(bytes.toString('utf8'));
    assert.equal(archive.schemaVersion,1);assert.equal(archive.sourceRevision,'6f32c2924d6284e0a43130f225ed817450f1f2f8');
    const rows=archive.sources.filter(row=>row.identity==='raw consuming');assert.equal(rows.length,1);
    const row=rows[0];assert.equal(row.file,RAW_DESCRIPTOR_HISTORICAL_FILE);
    assert.deepEqual({bytes:row.bytes,sha256:row.sha256},RAW_DESCRIPTOR_HISTORICAL_PREVIOUS);
    assert.deepEqual(pinRawDescriptorHistoricalSource(row.sourceText),RAW_DESCRIPTOR_HISTORICAL_PREVIOUS);
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:row.file});
    assert.deepEqual(row.current,RAW_DESCRIPTOR_HISTORICAL_RELOCATION);
    assert.deepEqual(row.changes,[{path:['predecessor','proof','producerApplicability','sources','12','sha256'],before:scalarFacts[2][1],after:scalarFacts[2][2]}]);
    authority=row;
  }
  return structuredClone(authority);
}

/** Reuse the existing complete certified continuation row and its exact forward edit. */
export function readRawDescriptorBorrowedAuthority(){
  if(!borrowedAuthority){
    const bytes=readFileSync(new URL(RAW_DESCRIPTOR_CONTINUATION_ARCHIVE,root));
    assert.deepEqual(pinRawDescriptorHistoricalSource(bytes),RAW_DESCRIPTOR_CONTINUATION_ARCHIVE_PIN);
    const archive=readBootstrapStructuredDataLino(bytes.toString('utf8'));
    assert.equal(archive.schemaVersion,1);assert.equal(archive.sources.length,10);
    const rows=archive.sources.filter(row=>row.file===RAW_DESCRIPTOR_BORROWED_FILE&&row.profiles.includes('relative array'));assert.equal(rows.length,1);
    const row=rows[0];assert.deepEqual(pinRawDescriptorHistoricalSource(row.sourceText),{bytes:row.bytes,sha256:row.sha256});
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.sourceText),{revision:row.commit,path:row.file});
    assert.deepEqual(row.current,RAW_DESCRIPTOR_BORROWED_PREVIOUS);
    const edit=row.edit,old=Buffer.from(row.sourceText);assert.equal(edit.offsetUnit,'UTF-8 bytes');
    assert.ok(Number.isSafeInteger(edit.start)&&Number.isSafeInteger(edit.end)&&edit.start>=0&&edit.end>=edit.start&&edit.start<=old.length);
    assert.equal(old.subarray(edit.start,edit.start+Buffer.byteLength(edit.removed)).toString('utf8'),edit.removed);
    const expected=Buffer.concat([old.subarray(0,edit.start),Buffer.from(edit.inserted),old.subarray(edit.start+Buffer.byteLength(edit.removed))]);
    assert.equal(edit.end,edit.start+Buffer.byteLength(edit.inserted));
    assert.deepEqual(pinRawDescriptorHistoricalSource(expected),RAW_DESCRIPTOR_BORROWED_PREVIOUS);
    const inverse=Buffer.concat([expected.subarray(0,edit.start),Buffer.from(edit.removed),expected.subarray(edit.end)]);
    assert.ok(inverse.equals(old),'the entire existing continuation source and inverse remain exact');
    borrowedAuthority={row,source:expected.toString('utf8')};
  }
  return structuredClone(borrowedAuthority);
}

export function deriveRawDescriptorBorrowedChange(previous,current){
  assert.deepEqual(pinRawDescriptorHistoricalSource(previous),RAW_DESCRIPTOR_BORROWED_PREVIOUS);
  assert.deepEqual(pinRawDescriptorHistoricalSource(current),RAW_DESCRIPTOR_BORROWED_CURRENT);
  const before=previous.split('\n'),after=current.split('\n');assert.equal(before.length,after.length);
  const indices=[];for(let index=0;index<before.length;index++)if(before[index]!==after[index])indices.push(index);
  assert.equal(indices.length,1);const index=indices[0];
  assert.equal(before[index],line(24,RAW_DESCRIPTOR_HISTORICAL_RELOCATION.sha256));
  assert.equal(after[index],line(24,RAW_DESCRIPTOR_HISTORICAL_CURRENT.sha256));
  return{line:index,link:24,before:before[index],after:after[index]};
}

/** All nine changes affect scalar facts, never definitions, references or links. */
export function deriveRawDescriptorHistoricalChanges(previous,current){
  assert.deepEqual(pinRawDescriptorHistoricalSource(previous),RAW_DESCRIPTOR_HISTORICAL_PREVIOUS);
  assert.deepEqual(pinRawDescriptorHistoricalSource(current),RAW_DESCRIPTOR_HISTORICAL_CURRENT);
  const before=previous.split('\n'),after=current.split('\n');assert.equal(before.length,after.length);
  const edits=[];
  for(let index=0;index<before.length;index++)if(before[index]!==after[index]){
    const specification=scalarFacts[edits.length];assert.ok(specification,'no additional scalar or structural changes');
    const[id,oldValue,newValue]=specification;
    assert.equal(before[index],line(id,oldValue));assert.equal(after[index],line(id,newValue));
    edits.push({line:index,link:id,before:before[index],after:after[index]});
  }
  assert.equal(edits.length,9);return edits;
}

function replayRawDescriptorHistoricalChanges(source,edits,forward){
  const lines=source.split('\n');let previous=-1;
  assert.equal(edits.length,9);
  for(let index=0;index<edits.length;index++){
    const edit=edits[index];exactKeys(edit,['line','link','before','after']);
    assert.ok(Number.isSafeInteger(edit.line)&&edit.line>previous&&edit.line<lines.length);previous=edit.line;
    const[id,before,after]=scalarFacts[index];assert.equal(edit.link,id);assert.equal(edit.before,line(id,before));assert.equal(edit.after,line(id,after));
    assert.equal(lines[edit.line],forward?edit.before:edit.after);lines[edit.line]=forward?edit.after:edit.before;
  }
  return lines.join('\n');
}

/** Reconstruct and authenticate both complete epochs without duplicating bodies. */
export function verifyRawDescriptorHistoricalCohort(packet){
  exactKeys(packet,['schemaVersion','model','file','revision','archive','previous','current','edits','borrowed','boundary']);
  assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'raw descriptor historical cohort 1');
  assert.equal(packet.file,RAW_DESCRIPTOR_HISTORICAL_FILE);assert.equal(packet.revision,RAW_DESCRIPTOR_HISTORICAL_REVISION);
  assert.deepEqual(packet.archive,{file:RAW_DESCRIPTOR_HISTORICAL_ARCHIVE,...RAW_DESCRIPTOR_HISTORICAL_ARCHIVE_PIN,identity:'raw consuming'});
  const old=readRawDescriptorHistoricalAuthority();assert.deepEqual(packet.previous,{revision:old.commit,...RAW_DESCRIPTOR_HISTORICAL_PREVIOUS});
  exactKeys(packet.current,['bytes','sha256','certificate']);assert.deepEqual({bytes:packet.current.bytes,sha256:packet.current.sha256},RAW_DESCRIPTOR_HISTORICAL_CURRENT);
  assert.equal(packet.boundary,'Complete committed descriptor metadata only; unchanged definitions, source identities and historical gates. This is not native execution or semantic equivalence.');
  const current=replayRawDescriptorHistoricalChanges(old.sourceText,packet.edits,true);
  assert.deepEqual(pinRawDescriptorHistoricalSource(current),RAW_DESCRIPTOR_HISTORICAL_CURRENT);
  verifyGitArtifactCertificate(packet.current.certificate,Buffer.from(current),{revision:packet.revision,path:packet.file});
  assert.equal(replayRawDescriptorHistoricalChanges(current,packet.edits,false),old.sourceText,'every complete old byte survives');
  assert.deepEqual(packet.edits,deriveRawDescriptorHistoricalChanges(old.sourceText,current));
  // Actual normal graph decoding preserves ordered owners and the aliased source
  // history record. Scalar-link replacement changes every use of the same fact.
  const previous=readBootstrapStructuredDataLino(old.sourceText),actual=readBootstrapStructuredDataLino(current);
  for(const value of[previous,actual]){
    assert.equal(value.predecessor.proof.producerApplicability.sourceHistory.current,value.predecessor.proof.producerApplicability.sources[1]);
    assert.equal(serializeProjectData(value,packet.file),value===previous?old.sourceText:current);
  }
  const logicalFacts=[
    [['predecessor','proof','producerApplicability','sources','1','bytes'],0],
    [['predecessor','proof','producerApplicability','sources','1','sha256'],1],
    [['predecessor','proof','producerApplicability','sources','12','sha256'],2],
    [['predecessor','proof','producerApplicability','sourceHistory','current','bytes'],0],
    [['predecessor','proof','producerApplicability','sourceHistory','current','sha256'],1],
    [['decisionSources','1','bytes'],3],[['decisionSources','1','sha256'],4],
    [['decisionSources','2','bytes'],0],[['decisionSources','2','sha256'],1],
    [['decisionSources','9','bytes'],5],[['decisionSources','9','sha256'],6],
    [['decisionSources','11','bytes'],7],[['decisionSources','11','sha256'],8],
  ];
  for(const[path,index]of logicalFacts){
    let before=previous,after=actual;for(const key of path){assert.ok(Object.hasOwn(before,key)&&Object.hasOwn(after,key));before=before[key];after=after[key];}
    assert.equal(before,scalarFacts[index][1]);assert.equal(after,scalarFacts[index][2]);
  }
  const relocation=old.sourceText.replace(line(305,scalarFacts[2][1]),line(305,scalarFacts[2][2]));
  assert.deepEqual(pinRawDescriptorHistoricalSource(relocation),RAW_DESCRIPTOR_HISTORICAL_RELOCATION);
  // The earlier packaging edge independently names the same sole changed path.
  const packaged=readBootstrapStructuredDataLino(relocation),change=old.changes[0];
  let owner=packaged;for(const key of change.path.slice(0,-1))owner=owner[key];
  assert.equal(owner[change.path.at(-1)],change.after);owner[change.path.at(-1)]=change.before;
  assert.equal(serializeProjectData(packaged,packet.file),old.sourceText);
  exactKeys(packet.borrowed,['file','archive','previous','current','edit']);
  const borrowed=packet.borrowed;assert.equal(borrowed.file,RAW_DESCRIPTOR_BORROWED_FILE);
  assert.deepEqual(borrowed.archive,{file:RAW_DESCRIPTOR_CONTINUATION_ARCHIVE,...RAW_DESCRIPTOR_CONTINUATION_ARCHIVE_PIN,source:RAW_DESCRIPTOR_BORROWED_FILE,profile:'relative array'});
  assert.deepEqual(borrowed.previous,RAW_DESCRIPTOR_BORROWED_PREVIOUS);
  exactKeys(borrowed.current,['bytes','sha256','certificate']);assert.deepEqual({bytes:borrowed.current.bytes,sha256:borrowed.current.sha256},RAW_DESCRIPTOR_BORROWED_CURRENT);
  const borrowedPrevious=readRawDescriptorBorrowedAuthority().source,parts=borrowedPrevious.split('\n'),edit=borrowed.edit;
  exactKeys(edit,['line','link','before','after']);assert.ok(Number.isSafeInteger(edit.line)&&edit.line>=0&&edit.line<parts.length);
  assert.equal(edit.link,24);assert.equal(edit.before,line(24,RAW_DESCRIPTOR_HISTORICAL_RELOCATION.sha256));assert.equal(edit.after,line(24,RAW_DESCRIPTOR_HISTORICAL_CURRENT.sha256));
  assert.equal(parts[edit.line],edit.before);parts[edit.line]=edit.after;const borrowedCurrent=parts.join('\n');
  assert.deepEqual(pinRawDescriptorHistoricalSource(borrowedCurrent),RAW_DESCRIPTOR_BORROWED_CURRENT);
  verifyGitArtifactCertificate(borrowed.current.certificate,Buffer.from(borrowedCurrent),{revision:packet.revision,path:borrowed.file});
  assert.deepEqual(edit,deriveRawDescriptorBorrowedChange(borrowedPrevious,borrowedCurrent));
  parts[edit.line]=edit.before;assert.equal(parts.join('\n'),borrowedPrevious,'the entire original borrowed receipt successor survives');
  for(const[source,expected]of [[borrowedPrevious,RAW_DESCRIPTOR_HISTORICAL_RELOCATION.sha256],[borrowedCurrent,RAW_DESCRIPTOR_HISTORICAL_CURRENT.sha256]])
    assert.equal(readBootstrapStructuredDataLino(source).predecessor.sha256,expected,'the sole changed scalar is the actual complete predecessor descriptor identity');
  return{packet,current,previous:old.sourceText,relocation,borrowedPrevious,borrowedCurrent};
}

export function readRawDescriptorHistoricalCohort(){
  if(!retained){
    const bytes=readFileSync(new URL(RAW_DESCRIPTOR_HISTORICAL_OUTPUT,root));
    assert.match(packetSha256,/^[a-f0-9]{64}$/u);assert.equal(pinRawDescriptorHistoricalSource(bytes).sha256,packetSha256);
    retained=verifyRawDescriptorHistoricalCohort(readBootstrapStructuredDataLino(bytes.toString('utf8')));
  }
  return structuredClone(retained.packet);
}

/** Exact known physical bytes are selected before older intermediary views. */
export function restoreRawDescriptorHistoricalSource(source,file,profile){
  assert.equal(typeof source,'string');assert.equal(typeof file,'string');assert.equal(typeof profile,'string');
  if(file===RAW_DESCRIPTOR_BORROWED_FILE&&profile==='continuation historical input'){
    if(pinRawDescriptorHistoricalSource(source).sha256!==RAW_DESCRIPTOR_BORROWED_CURRENT.sha256)return source;
    assert.deepEqual(pinRawDescriptorHistoricalSource(source),RAW_DESCRIPTOR_BORROWED_CURRENT);
    readRawDescriptorHistoricalCohort();assert.equal(source,retained.borrowedCurrent);return retained.borrowedPrevious;
  }
  if(file!==RAW_DESCRIPTOR_HISTORICAL_FILE||!['relative array','native library relocation','continuation historical input'].includes(profile))return source;
  if(pinRawDescriptorHistoricalSource(source).sha256!==RAW_DESCRIPTOR_HISTORICAL_CURRENT.sha256)return source;
  assert.deepEqual(pinRawDescriptorHistoricalSource(source),RAW_DESCRIPTOR_HISTORICAL_CURRENT);
  readRawDescriptorHistoricalCohort();assert.equal(source,retained.current);
  return profile==='relative array'?retained.previous:retained.relocation;
}
