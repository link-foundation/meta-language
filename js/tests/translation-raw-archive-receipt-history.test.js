import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import test from 'node:test';
import {readProjectData,serializeProjectData} from '../scripts/project-data.mjs';
import {readRawArchiveReceiptHistory,verifyRawArchiveReceiptHistory,restoreRawArchiveReceiptText} from '../scripts/raw-archive-receipt-history.mjs';
import {restoreCompleteDeclarationReceipt} from '../scripts/complete-declaration-receipts.mjs';
const root=new URL('../../',import.meta.url);
test('raw archive metadata succession preserves its whole certified snapshot and every unrelated field',()=>{
 const[major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25 required');
 const packet=readRawArchiveReceiptHistory();assert.equal(packet.rows.length,1);
 for(const row of packet.rows){
  const source=readFileSync(new URL(row.file,root),'utf8'),before=execFileSync('git',['--no-replace-objects','show',packet.revision+':'+row.file],{cwd:root,encoding:'utf8',maxBuffer:3*1024*1024});
  assert.equal(source,before,'physical snapshot bytes stay exact');
  assert.equal(restoreRawArchiveReceiptText(before,row.file),before);
  assert.equal(restoreRawArchiveReceiptText(source+'\n',row.file),source+'\n');
  assert.equal(restoreRawArchiveReceiptText(source,'unregistered.lino'),source);
  const value=readProjectData(new URL(row.file,root)),current=structuredClone(value);
  for(const change of row.changes){assert.deepEqual(value[change.path[0]][change.path[1]],change.before);current[change.path[0]][change.path[1]]=change.after;}
  const actual=serializeProjectData(current,row.file);
  assert.equal(restoreRawArchiveReceiptText(actual,row.file),before,'all complete unrelated fields remain exact');
  assert.equal(restoreRawArchiveReceiptText(actual+'\n',row.file),actual+'\n');
  assert.doesNotThrow(()=>restoreCompleteDeclarationReceipt(value,'raw consuming'));
 }
});
test('altered archive counts replays definitions receipts and coherent caller repins cannot acquire old authority',()=>{
 const packet=readRawArchiveReceiptHistory();
 for(const mutate of [value=>{value.rows[0].current.sha256='0'.repeat(64);},value=>{value.rows[0].changes.pop();},value=>{value.rows[0].changes[0].path=['owners','0'];},value=>{value.rows.push(structuredClone(value.rows[0]));},value=>{value.rows[0].certificate.path='unregistered.lino';}]){
  const changed=structuredClone(packet);mutate(changed);assert.throws(()=>verifyRawArchiveReceiptHistory(changed));
 }
 const row=packet.rows[0],value=readProjectData(new URL(row.file,root));
 for(const change of row.changes)value[change.path[0]][change.path[1]]=change.after;
 for(const mutate of [current=>{current.archive.recordCount++;},current=>{current.archive.textBytes++;},current=>{current.replays[0].emittedSha256='0'.repeat(64);},current=>{current.completeRust+=' ';}]){
  const changed=structuredClone(value);mutate(changed);const source=serializeProjectData(changed,row.file);
  assert.equal(restoreRawArchiveReceiptText(source,row.file),source);
  assert.throws(()=>restoreCompleteDeclarationReceipt(changed,'raw consuming'));
 }
 const detached=readRawArchiveReceiptHistory();detached.rows.pop();assert.equal(readRawArchiveReceiptHistory().rows.length,1);
});
