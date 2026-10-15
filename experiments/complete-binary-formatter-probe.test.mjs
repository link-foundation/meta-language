import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {readProjectData} from '../js/scripts/project-data.mjs';
import {completeBinaryFormatterProbe} from './complete-binary-formatter-probe.mjs';
import {withRustOwnedLocalAuthenticSource} from './rust-owned-local-authentic-source.mjs';
import {readSourceTextUnits,sliceSourceTextBytes} from './source-text-byte-adapter.mjs';
import {readSourceTextByteLength} from './source-text-byte-rules.mjs';

test('the exact complete pinned formatter remains refused without substituting a builtin or extracting functions',()=>{
 const recorded=readProjectData(new URL('./complete-binary-formatter-probe.lino',import.meta.url));
 assert.deepEqual(completeBinaryFormatterProbe(),recorded);
 const previous=readProjectData(new URL('./rust-owned-local-state-fixture.lino',import.meta.url));
 assert.equal(recorded.gitCommit,'26667280');
 assert.deepEqual(recorded.files,previous.authentic.files);
 assert.equal(recorded.combined.bytes,132002);assert.ok(recorded.combined.tokens>1000);
 assert.equal(recorded.byteSliceSites.length,4);assert.ok(recorded.byteSliceSites.every(site=>site.end>site.start&&site.originalText.includes('[')));
 assert.equal(recorded.attempts[0].originalToken,'return');assert.equal(recorded.attempts[1].originalToken,'mut');
 for(let index=0;index<2;index++){
  const {originalToken,...attempt}=recorded.attempts[index];
  assert.deepEqual(attempt,previous.authentic.diagnostics[index]);
 }
 assert.match(recorded.attempts[2].reason,/only direct standard String, i32 and Boolean interfaces/u);
 assert.match(recorded.attempts[4].reason,/independent composition proofs/u);
});

test('the byte kernel preserves the complete original source and exact recorded slice sites without source rewriting',()=>{
 const recorded=readProjectData(new URL('./complete-binary-formatter-probe.lino',import.meta.url));
 withRustOwnedLocalAuthenticSource(recorded.files.map(entry=>entry.file),paths=>{
  for(let index=0;index<paths.length;index++){
   const bytes=readFileSync(paths[index]),text=bytes.toString('utf8'),receipt=recorded.files[index];
   assert.deepEqual(Buffer.from(text,'utf8'),bytes);
   assert.equal(readSourceTextByteLength(readSourceTextUnits(text)),receipt.bytes);
   assert.equal(sliceSourceTextBytes(text,0,receipt.bytes),text);
   for(const site of recorded.byteSliceSites.filter(site=>site.file===receipt.file)){
    const start=Buffer.byteLength(text.slice(0,site.start),'utf8'),end=Buffer.byteLength(text.slice(0,site.end),'utf8');
    assert.equal(sliceSourceTextBytes(text,start,end),site.originalText);
   }
  }
 });
});
