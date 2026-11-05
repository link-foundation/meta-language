// Complete source-only bridge before existing historical native reader chains.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {replayRequiredFieldNativeHostIntervals} from './required-field-native-host-succession.mjs';
const root=new URL('../../',import.meta.url);
export const CURRENT_NATIVE_HOST_REVISION='8d77e476d097448eb381ebe03e2aa173402bc03d';
export const CURRENT_NATIVE_HOST_OUTPUT='parity/self-translation/ci-39539903548-native-host-source-history.lino';
export const CURRENT_NATIVE_HOST_FILES=Object.freeze(['rust/src/self_translation.rs','rust/src/translation/mod.rs','rust/tests/unit/mod.rs','rust/src/translation/javascript/expressions/values.rs']);
const packetSha256='f340d690099b12fd4544714752f9c63d662a908c545bd6efe785caddcbe319f1';
let cached;
export const pinCurrentNativeHostSource=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const exactKeys=(value,keys)=>{assert.ok(value&&typeof value==='object'&&!Array.isArray(value));assert.deepEqual(Object.keys(value).sort(),[...keys].sort());};

export function deriveCurrentNativeHostSourceIntervals(previous,current){
  assert.equal(Buffer.from(previous).toString('utf8'),previous);assert.equal(Buffer.from(current).toString('utf8'),current);
  const before=[...previous],after=[...current];let start=0,oldEnd=before.length,end=after.length;
  while(start<oldEnd&&start<end&&before[start]===after[start])start++;
  while(oldEnd>start&&end>start&&before[oldEnd-1]===after[end-1]){oldEnd--;end--;}
  const edits=[{start:before.slice(0,start).join('').length,removed:before.slice(start,oldEnd).join(''),inserted:after.slice(start,end).join('')}];
  assert.equal(replayRequiredFieldNativeHostIntervals(previous,edits),current);return edits;
}

export function verifyCurrentNativeHostSourceHistory(packet){
  exactKeys(packet,['schemaVersion','model','revision','sources','boundary']);
  assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'complete current native historical host sources 1');
  assert.equal(packet.revision,CURRENT_NATIVE_HOST_REVISION);
  assert.deepEqual(packet.sources.map(row=>row.file),CURRENT_NATIVE_HOST_FILES);
  for(const row of packet.sources){
    exactKeys(row,['file','previous','current','coordinateUnit','edits']);
    exactKeys(row.previous,['source','bytes','sha256','certificate']);exactKeys(row.current,['source','bytes','sha256']);
    assert.deepEqual(pinCurrentNativeHostSource(row.previous.source),{bytes:row.previous.bytes,sha256:row.previous.sha256});
    verifyGitArtifactCertificate(row.previous.certificate,Buffer.from(row.previous.source),{revision:CURRENT_NATIVE_HOST_REVISION,path:row.file});
    assert.deepEqual(pinCurrentNativeHostSource(row.current.source),{bytes:row.current.bytes,sha256:row.current.sha256});
    assert.equal(row.current.source,readFileSync(new URL(row.file,root),'utf8'),'the entire actual current native host supplies this view');
    assert.equal(row.coordinateUnit,'UTF-16 code units');
    assert.deepEqual(row.edits,deriveCurrentNativeHostSourceIntervals(row.previous.source,row.current.source));
    assert.equal(replayRequiredFieldNativeHostIntervals(row.previous.source,row.edits),row.current.source);
    let inverse=row.current.source;
    for(const edit of row.edits.toReversed()){
      assert.equal(inverse.slice(edit.start,edit.start+edit.inserted.length),edit.inserted);
      inverse=inverse.slice(0,edit.start)+edit.removed+inverse.slice(edit.start+edit.inserted.length);
    }
    assert.equal(inverse,row.previous.source);
  }
  return packet;
}

export function readCurrentNativeHostSourceHistory(){
  if(!cached){const bytes=readFileSync(new URL(CURRENT_NATIVE_HOST_OUTPUT,root));assert.equal(pinCurrentNativeHostSource(bytes).sha256,packetSha256);cached=verifyCurrentNativeHostSourceHistory(readBootstrapStructuredDataLino(bytes.toString('utf8')));}
  return structuredClone(cached);
}

/** Unknown sources remain unchanged for the original strict reader to refuse.
 * No unknown source is assigned a historical identity; runtime files stay current. */
export function restoreCurrentNativeHostSource(source,file){
  assert.equal(typeof source,'string');assert.equal(typeof file,'string');
  if(!CURRENT_NATIVE_HOST_FILES.includes(file))return source;
  const row=readCurrentNativeHostSourceHistory().sources.find(row=>row.file===file);
  return source===row.current.source?row.previous.source:source;
}
