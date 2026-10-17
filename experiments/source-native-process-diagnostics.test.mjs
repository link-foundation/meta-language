// Output-writer injection only; no compiler or native process can run here.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {join} from 'node:path';
import test from 'node:test';
import {mirrorSourceNativeProcessDiagnostics} from './source-native-process-diagnostics.mjs';
const sha256=value=>createHash('sha256').update(value).digest('hex');

test('native diagnostic mirroring keeps exact bounded buffers and authentic stream delimiters without creating observations',()=>{
 const stdout=Buffer.from('partial protocol Ω\r\n\0last'),stderr=Buffer.concat([Buffer.from('compiler diagnostic 😀\n'),Buffer.alloc(1048576,65)]);
 const events=[],context={processMode:'native subprocesses',label:'original library compiler',directory:join('owned','diagnostics'),stdout,stderr};
 const result=mirrorSourceNativeProcessDiagnostics(context,bytes=>events.push(Buffer.from(bytes)));
 assert.equal(result,undefined);assert.equal(events.length,6);
 for(const[index,stream,captured]of[[0,'stdout',stdout],[3,'stderr',stderr]]){
  const expected=captured.subarray(0,1048576),begin='\nBEGIN SOURCE NATIVE PROCESS DIAGNOSTICS ',end='\nEND SOURCE NATIVE PROCESS DIAGNOSTICS ';
  assert.deepEqual(events[index+1],expected,'the captured process bytes are not decoded, escaped, trimmed or reordered');
  const header=events[index].toString('utf8');assert.ok(header.startsWith(begin));assert.equal(header.at(-1),'\n');
  const identity=JSON.parse(header.slice(begin.length,-1));
  assert.deepEqual(identity,{label:context.label,stream,file:join(context.directory,stream+'.log'),capturedBytes:captured.length,
   retainedBytes:expected.length,truncated:captured.length!==expected.length,sha256:sha256(expected)});
  assert.equal(events[index+2].toString('utf8'),end+JSON.stringify(identity)+'\n');
 }
 assert.equal(events[4].length,1048576);assert.equal(Object.hasOwn(context,'observations'),false);
});

test('injected and unknown process modes cannot mirror synthetic bytes as native diagnostics',()=>{
 for(const processMode of['injected process contract','',undefined,'native']){
  let writes=0;
  assert.equal(mirrorSourceNativeProcessDiagnostics({processMode,label:'synthetic',directory:'owned',stdout:Buffer.from('fake output'),stderr:Buffer.from('fake error')},()=>writes++),undefined);
  assert.equal(writes,0);
 }
});
