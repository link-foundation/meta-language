import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {tokenize} from '../src/translation/lexer.js';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {rustSharedArrayHandleRulesPolicy} from '../scripts/generate-rust-shared-array-handle-rules.mjs';
const root=fileURLToPath(new URL('../../',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');
function nativeContractScript(){
 const source=readFileSync(new URL('../../rust/tests/unit/translation_rust_shared_array_handles.rs',import.meta.url),'utf8');
 const tokens=tokenize(source,'Rust').tokens;
 const owners=tokens.flatMap((token,index)=>token.value==='fn'&&tokens[index+1]?.value==='contract_with_source_model'?[index]:[]);assert.equal(owners.length,1);
 let start=owners[0]+2;while(tokens[start].value!=='{')start++;
 let depth=1,end=start+1;
 while(end<tokens.length&&depth){if(tokens[end].kind==='punct'){if(tokens[end].value==='{')depth++;else if(tokens[end].value==='}')depth--;}end++;}
 assert.equal(depth,0);const scripts=[];
 for(let at=start+1;at<end;at++)if(tokens[at].value==='let'&&tokens[at+1]?.value==='script'&&tokens[at+2]?.value==='='&&tokens[at+3]?.kind==='string')scripts.push(tokens[at+3].value);
 assert.equal(scripts.length,1,'the complete actual native oracle owns exactly one JavaScript contract');return scripts[0];
}
const execute=(script,args=[])=>execFileSync(process.execPath,['--max-old-space-size=352','--max-semi-space-size=1','--input-type=module','-e',script,...args],{cwd:root,encoding:'utf8',timeout:20000,maxBuffer:2*1024*1024,stdio:['ignore','pipe','pipe']});
test('the actual CI native contract transports every original shared-array source and both complete emitted field sets',()=>{
 const script=nativeContractScript();
 for(const row of rustSharedArrayHandleRulesPolicy().cases){
  const contract=readStructuredDataLino(execute(script,[row.source,row.call]));
  assert.ok(contract.source.startsWith(contract.library));assert.match(contract.source,/fn main\(\)\{println!\("\{\}",/u);
  assert.equal(contract.surface.main.effects[0].k,'print');
  for(const target of ['javascript','rust'])for(const field of ['language','text','definitions','preludes','mappings','assumptions','encodings','theorems','entry']){
   assert.ok(Object.hasOwn(contract[target],field),target+' '+field+' remains in the native parity oracle');
  }
  assert.ok(contract.javascript.definitions.length>0);assert.ok(contract.rust.definitions.length>0);
  const original=execute(row.source+'\nconsole.log('+row.call+'());');
  assert.equal(original,String(row.expected)+'\n');assert.equal(execute(contract.javascript.text),original);
 }
});
test('native contract serialization refuses missing required values rather than dropping them with optional annotations',()=>{
 const script=nativeContractScript(),marker='console.log(structuredDataToLino(';assert.equal(script.split(marker).length,2);
 const corrupted=script.replace(marker,'surface.items[0].body.value=undefined;\n'+marker),row=rustSharedArrayHandleRulesPolicy().cases[0];
 assert.throws(()=>execute(corrupted,[row.source,row.call]),error=>error.status!==0&&String(error.stderr).includes('only an absent optional let type may be omitted'));
 for(const mutation of [
  'Object.defineProperty(surface,"hidden",{value:true});',
  'surface[Symbol("hidden")]=true;',
  'Object.defineProperty(surface,"accessor",{enumerable:true,get(){throw new Error("accessor must not execute");}});',
  'Object.setPrototypeOf(surface,{inherited:true});',
  'delete surface.items[0];',
  'surface.items.extra=true;',
 ]){
  const invalid=script.replace(marker,mutation+'\n'+marker);
  assert.throws(()=>execute(invalid,[row.source,row.call]),error=>error.status!==0&&String(error.stderr).includes('native surface contract requires'));
 }
});

test('the actual native associated-clone contract retains complete original Rust and every emitted field',()=>{
 const script=nativeContractScript(),policy=rustSharedArrayHandleRulesPolicy();
 for(const row of policy.associatedCloneCases){
  const contract=readStructuredDataLino(execute(script,[row.source,row.call,policy.associatedCloneModel,'Rust']));
  assert.equal(contract.library,row.source);assert.equal(contract.surface.main.effects[0].k,'print');
  for(const target of ['javascript','rust'])for(const field of ['language','text','definitions','preludes','mappings','assumptions','encodings','theorems','entry'])assert.ok(Object.hasOwn(contract[target],field),target+' '+field);
  const expected=execute(row.javascriptSource+'\nconsole.log('+row.call+'());');assert.equal(expected,String(row.expected)+'\n');
  assert.equal(execute(contract.javascript.text),expected);
 }
});
