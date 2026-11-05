import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readSharedArrayNativeFrontendHistory,verifySharedArrayNativeFrontendHistory,restoreSharedArrayNativeFrontendSource} from '../scripts/shared-array-native-frontend-history.mjs';
test('the complete committed native frontend source view retains actual current bytes and each original host',()=>{
 const packet=readSharedArrayNativeFrontendHistory();assert.equal(packet.rows.length,7);
 for(const row of packet.rows){
  const actual=readFileSync(new URL('../../'+row.file,import.meta.url),'utf8');assert.equal(actual,row.current.source);
  assert.equal(restoreSharedArrayNativeFrontendSource(actual,row.file),row.previous.source);
  assert.equal(restoreSharedArrayNativeFrontendSource(row.previous.source,row.file),row.previous.source);
  assert.equal(restoreSharedArrayNativeFrontendSource(actual+'\n',row.file),actual+'\n');
  assert.equal(restoreSharedArrayNativeFrontendSource(actual,'unregistered.rs'),actual);
 }
 for(const mutate of[value=>{value.rows.pop();},value=>{value.rows[0].current.source+=' ';},value=>{value.rows[0].previous.certificate.path='unregistered.rs';},value=>{value.rows[0].edit.removed+=' ';},value=>{value.currentRevision='0'.repeat(40);},value=>{value.extra=true;}]){
  const changed=structuredClone(packet);mutate(changed);assert.throws(()=>verifySharedArrayNativeFrontendHistory(changed));
 }
});
