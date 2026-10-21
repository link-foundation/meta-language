// Historical source identity only; current emission keeps its physical inputs.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData} from './project-data.mjs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
const root=new URL('../../',import.meta.url);
export const RAW_CONSUMING_SNAPSHOT_REVISION='8d77e476d097448eb381ebe03e2aa173402bc03d';
export const RAW_CONSUMING_SNAPSHOT_OUTPUT='parity/self-translation/raw-consuming-snapshot-source-profile.lino';
export const RAW_CONSUMING_SNAPSHOT_HOST='experiments/original-link-complete-table-projection/raw-consuming-emission.mjs';
export const RAW_CONSUMING_SNAPSHOT_FILES=Object.freeze([
  Object.freeze({file:RAW_CONSUMING_SNAPSHOT_HOST,bytes:21092,sha256:'a4d64a98279329ec80451bdcf888cef6bcf6a7eedcd0d2c517727a1eb49e0db7'}),
  Object.freeze({file:'js/src/translation/check.js',bytes:131170,sha256:'5219d03742b1227486a0e0daa71225891586f5f7815a1a558c1f9c0228ff9bbd'}),
  Object.freeze({file:'js/src/translation/emit-rust.js',bytes:78257,sha256:'7d8f61a0de6af13f0061fcd7efe75af746909fbd050d9e281e0db12014754bd0'}),
  Object.freeze({file:'js/src/translation/emit-javascript.js',bytes:35562,sha256:'b9967bc6024d7967e1c429ffdc5171d32db30042354104164c64ef1ba155414d'}),
  Object.freeze({file:'js/src/translation/owned-array-append-rules.js',bytes:4830,sha256:'eb349133f29d2e5944ace3b96975d508a92d265231518ac2b99a604129c7b48a'}),
]);
export const RAW_CONSUMING_SNAPSHOT_BOUNDARY='Five complete source identities serve only the frozen raw consuming artifact and its historical consumers. Three original source certificates are projected automatically from existing archives; only the missing consuming host and checker are captured once. Complete stored source certificates make later freshness independent of those archives. The current producer retains its physical source metadata separately and must reproduce every complete original semantic artifact field and both codecs before snapshot comparison. Source conservation is not general semantic equivalence or native execution evidence.';
const packetSha256='ef289a9e8a67d0deccb22cd943831ae0860053ea3b5267aaba9d208dc12e0b4f';
export const pinRawConsumingSnapshotSource=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const keys=(value,names)=>assert.deepEqual(Object.keys(value).sort(),[...names].sort());
let cached;

function verifyPreviousSource(row,expected){
  keys(row,['file','source','bytes','sha256','certificate']);
  assert.equal(row.file,expected.file);assert.equal(typeof row.source,'string');
  assert.deepEqual(pinRawConsumingSnapshotSource(row.source),{bytes:expected.bytes,sha256:expected.sha256});
  assert.deepEqual({bytes:row.bytes,sha256:row.sha256},pinRawConsumingSnapshotSource(row.source));
  verifyGitArtifactCertificate(row.certificate,Buffer.from(row.source),{revision:RAW_CONSUMING_SNAPSHOT_REVISION,path:row.file});
  return row;
}

/** First capture only: project three existing independently certified source
 * rows. Their exact8d Git membership is authority, not archive freshness or an
 * assumption about current provider behavior. Later checks need no archives. */
export function readArchivedRawConsumingSnapshotSources(){
  const archive=file=>{const bytes=readFileSync(new URL(file,root));assert.ok(bytes.length<=16*1024*1024);return readBootstrapStructuredDataLino(bytes.toString('utf8'));};
  const providers=archive('parity/self-translation/reviewed-overlay-provider-succession.lino');
  const profiles=providers.profiles.filter(row=>row.sourceCommit===RAW_CONSUMING_SNAPSHOT_REVISION);
  assert.equal(profiles.length,1);
  const owned=archive('parity/self-translation/relative-array-owned-rule-source-history.lino');
  assert.equal(owned.revision,RAW_CONSUMING_SNAPSHOT_REVISION);
  return RAW_CONSUMING_SNAPSHOT_FILES.slice(2).map(expected=>{
    let source,certificate;
    if(expected.file===owned.source){({source,certificate}=owned.previous);}
    else{
      const entries=profiles[0].certificates.filter(row=>'js/src/'+row.module===expected.file);assert.equal(entries.length,1);
      ({source,certificate}=entries[0].current);
    }
    return verifyPreviousSource({file:expected.file,source,...pinRawConsumingSnapshotSource(source),certificate},expected);
  });
}

/** A finite automatically discovered complete-source interval. */
export function deriveRawConsumingSnapshotSourceInterval(previous,current){
  const before=[...previous],after=[...current];let start=0,oldEnd=before.length,end=after.length;
  while(start<oldEnd&&start<end&&before[start]===after[start])start+=1;
  while(oldEnd>start&&end>start&&before[oldEnd-1]===after[end-1]){oldEnd-=1;end-=1;}
  return{start:before.slice(0,start).join('').length,removed:before.slice(start,oldEnd).join(''),inserted:after.slice(start,end).join('')};
}

/** Structure/certificates do not grant current packet identity. */
export function verifyRawConsumingSnapshotSourceStructure(packet,{bindCurrent=true}={}){
  keys(packet,['schemaVersion','model','revision','previous','sources','boundary']);
  assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'raw consuming snapshot source profile 1');
  assert.equal(packet.revision,RAW_CONSUMING_SNAPSHOT_REVISION);assert.equal(packet.boundary,RAW_CONSUMING_SNAPSHOT_BOUNDARY);
  assert.deepEqual(packet.sources.map(row=>row.file),RAW_CONSUMING_SNAPSHOT_FILES.map(row=>row.file));
  assert.deepEqual(packet.previous.map(row=>row.file),RAW_CONSUMING_SNAPSHOT_FILES.map(row=>row.file));
  const previous=packet.previous.map((row,index)=>verifyPreviousSource(row,RAW_CONSUMING_SNAPSHOT_FILES[index]));
  for(const[index,row]of packet.sources.entries()){
    keys(row,['file','source','bytes','sha256','coordinateUnit','edit']);
    assert.equal(typeof row.source,'string');assert.ok(Buffer.byteLength(row.source)<=512*1024);
    assert.deepEqual(pinRawConsumingSnapshotSource(row.source),{bytes:row.bytes,sha256:row.sha256});
    assert.equal(row.coordinateUnit,'UTF-16 code units');keys(row.edit,['start','removed','inserted']);
    const original=previous[index].source,edit=row.edit;
    assert.deepEqual(edit,deriveRawConsumingSnapshotSourceInterval(original,row.source));
    assert.equal(original.slice(edit.start,edit.start+edit.removed.length),edit.removed);
    assert.equal(original.slice(0,edit.start)+edit.inserted+original.slice(edit.start+edit.removed.length),row.source);
    assert.equal(row.source.slice(edit.start,edit.start+edit.inserted.length),edit.inserted);
    assert.equal(row.source.slice(0,edit.start)+edit.removed+row.source.slice(edit.start+edit.inserted.length),original,'every certified historical source byte survives');
    if(bindCurrent)assert.equal(readFileSync(new URL(row.file,root),'utf8'),row.source,'actual final-use decision sources changed: '+row.file);
  }
  return packet;
}

function load(bindCurrent){
  if(!cached){
    assert.match(packetSha256,/^[a-f0-9]{64}$/u,'the normal source author must capture this profile first');
    const file=new URL(RAW_CONSUMING_SNAPSHOT_OUTPUT,root),bytes=readFileSync(file);
    assert.equal(pinRawConsumingSnapshotSource(bytes).sha256,packetSha256,'the complete independently pinned source profile stays exact');
    cached=verifyRawConsumingSnapshotSourceStructure(readProjectData(file),{bindCurrent:false});
  }
  if(bindCurrent)for(const row of cached.sources)assert.equal(readFileSync(new URL(row.file,root),'utf8'),row.source,'actual final-use decision sources changed: '+row.file);
  return cached;
}
export function readRawConsumingSnapshotSourceProfile(){return structuredClone(load(true));}
export function readCertifiedRawConsumingSnapshotSources(){return structuredClone(load(false).previous);}

/** Only these five exact current files acquire their recorded snapshot source;
 * all other paths are identities, and even one unknown selected byte refuses. */
export function restoreRawConsumingSnapshotSource(bytes,file,currentOnly=false){
  const index=RAW_CONSUMING_SNAPSHOT_FILES.findIndex(row=>row.file===file);
  if(index<0)return bytes;
  const packet=load(true),row=packet.sources[index];
  assert.ok(Buffer.isBuffer(bytes),'complete current source bytes are required');
  if(currentOnly&&bytes.toString('utf8')!==row.source)return bytes;
  assert.equal(bytes.toString('utf8'),row.source,'actual final-use decision sources changed: '+file);
  assert.deepEqual(pinRawConsumingSnapshotSource(bytes),{bytes:row.bytes,sha256:row.sha256});
  return Buffer.from(packet.previous[index].source);
}
export function rawConsumingSnapshotSourceReader(read){
  const selected=new Map(RAW_CONSUMING_SNAPSHOT_FILES.map(row=>[String(new URL(row.file,root)),row.file]));
  return file=>{const bytes=read(file),path=selected.get(String(file));return path?restoreRawConsumingSnapshotSource(bytes,path):bytes;};
}
