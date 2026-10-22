import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {readTypedSourceDescriptorHostHistory,verifyTypedSourceDescriptorHostHistory,restoreTypedSourceDescriptorHostSource} from '../scripts/typed-source-descriptor-host-history.mjs';
test('complete descriptor host predecessors retain certified definitions while unknown successors stay visible',()=>{
 const packet=readTypedSourceDescriptorHostHistory();assert.equal(packet.rows.length,6);
 for(const row of packet.rows){
  const actual=readFileSync(new URL('../../'+row.file,import.meta.url),'utf8');assert.equal(actual,row.current.source);
  assert.equal(restoreTypedSourceDescriptorHostSource(actual,row.file),row.previous.source);
  assert.equal(restoreTypedSourceDescriptorHostSource(row.previous.source,row.file),row.previous.source);
  assert.equal(restoreTypedSourceDescriptorHostSource(actual+'\n',row.file),actual+'\n');
  assert.equal(restoreTypedSourceDescriptorHostSource(actual,'unregistered.mjs'),actual);
 }
 for(const mutate of [value=>{value.rows.pop();},value=>{value.rows[0].current.source+=' ';},value=>{value.rows[0].previous.certificate.path='unregistered.mjs';},value=>{value.rows[0].edit.removed+=' ';},value=>{value.revision='0'.repeat(40);},value=>{value.extra=true;}]){
  const value=structuredClone(packet);mutate(value);assert.throws(()=>verifyTypedSourceDescriptorHostHistory(value));
 }
});
