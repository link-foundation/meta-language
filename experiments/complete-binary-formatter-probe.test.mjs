import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {readProjectData} from '../js/scripts/project-data.mjs';
import {readStructuredDataLino} from '../js/src/structured-data-links.js';
import {tokenize} from '../js/src/translation/lexer.js';
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
 const historicalBytes=execFileSync('git',['show','98d3599160997f0094598acf7be326f28142c536:experiments/complete-binary-formatter-probe.lino'],
  {cwd:fileURLToPath(new URL('../',import.meta.url)),timeout:10000,maxBuffer:64*1024});
 assert.equal(historicalBytes.length,20581);
 assert.equal(createHash('sha256').update(historicalBytes).digest('hex'),'691d5968ff7b07d5b7387eb68c1b0993310ed128f0744af9b57a4f918c859fde');
 const historical=readStructuredDataLino(historicalBytes.toString('utf8'));
 assert.equal(historical.combined.tokens,14722,'the exact archived report retains its original lexical observation');
 assert.equal(recorded.combined.tokens,14684,'the current lexer groups all thirteen original character literals');
 assert.deepEqual({...recorded,combined:{...recorded.combined,tokens:historical.combined.tokens}},historical,
  'every source, refusal, diagnostic and byte-slice field remains identical to the authenticated historical report');
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
  const source=paths.map(path=>readFileSync(path,'utf8')).join('\n');
  assert.equal(Buffer.byteLength(source),132002);assert.equal(source.length,132001);
  assert.equal(createHash('sha256').update(source).digest('hex'),'d4c229031becaed66c005ee8c2ae59155e7c084510a2df61785466b4b6ffad1c');
  const literals=[
   ['char','53',"'5'",3],['byteChar','48',"b'0'",4],['byteChar','48',"b'0'",4],
   ['char','48',"'0'",3],['char','45',"'-'",3],['char','43',"'+'",3],
   ['byteChar','57',"b'9'",4],['byteChar','48',"b'0'",4],
   ['char','101',"'e'",3],['char','46',"'.'",3],['char','48',"'0'",3],
   ['char','133',String.raw`'\u{85}'`,7],['char','65279',String.raw`'\u{feff}'`,7],
  ];
  const tokens=tokenize(source,'Rust').tokens;
  assert.deepEqual(tokens.filter(token=>['char','byteChar'].includes(token.kind)).map(token=>[token.kind,token.value,token.raw]),
   literals.map(entry=>entry.slice(0,3)),'all original character and byte-character sites retain their exact order, value and spelling');
  // Previous lexical pieces: apostrophe/value/apostrophe; b/apostrophe/value/apostrophe;
  // apostrophe/backslash/u/open-brace/value/close-brace/apostrophe.
  assert.deepEqual([3,4,7].map(count=>literals.filter(entry=>entry[3]===count).length),[7,4,2]);
  assert.equal(literals.reduce((count,entry)=>count+entry[3]-1,0),38);
  assert.equal(tokens.length,14722-38);assert.equal(tokens.length,recorded.combined.tokens);
  for(const token of tokens)assert.equal(source.slice(token.start,token.end),token.raw);
  assert.equal(tokens.at(-1).end,source.length,'the token inventory reaches the end of all four complete input files');
 });
});
