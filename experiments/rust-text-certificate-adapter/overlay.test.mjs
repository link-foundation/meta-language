import assert from 'node:assert/strict';
import test from 'node:test';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../../js/src/typed-program-links.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
import {emitJavaScript} from '../../js/src/translation/emit-javascript.js';
import {rustTextCertificateOverlayRecords} from './generate.mjs';
import {createCheckedModuleOverlay} from './overlay.mjs';
import {createRustTextCertificatePublicOverlay} from './runtime.mjs';
import {captureOwnedRustTextCertificate,readOwnedRustTextCertificate} from './model.mjs';
const hash=value=>createHash('sha256').update(value).digest('hex');

test('exact decorator receipts refuse changed providers and preserve all original source bytes',async()=>{
 const records=rustTextCertificateOverlayRecords(),root=new URL('../../js/src/',import.meta.url);for(const row of records)assert.equal(hash(readFileSync(new URL(row.module,root))),row.sourceSha256);
 const changed=records.map((row,index)=>index?row:{...row,sourceSha256:'0'.repeat(64)});await assert.rejects(createCheckedModuleOverlay({records:changed}),/exact overlay provider changed/u);
 const overlay=await createRustTextCertificatePublicOverlay();try{assert.deepEqual(overlay.receipts,records.map(({module,sourceSha256,overlaySha256})=>({module,sourceSha256,overlaySha256})));}finally{overlay.dispose();}
 for(const row of records)assert.equal(hash(readFileSync(new URL(row.module,root))),row.sourceSha256);
});

test('actual public defaults and nonsemantic provenance remain byte-identical in the isolated overlay',async()=>{
 const overlay=await createRustTextCertificatePublicOverlay();try{
  for(const provenance of[undefined,{sourceTextValueProjection:false,user:'metadata'}]){
   const source='fn keep(value:i32)->i32{value}fn main(){}',options=provenance===undefined?{}:{provenance},original=createTypedProgramLinks(source,'Rust',options),candidate=overlay.api.createTypedProgramLinks(source,'Rust',options);
   assert.equal(candidate.network.toLino(),original.network.toLino());assert.equal(candidate.root.asU64(),original.root.asU64());
   for(const target of['JavaScript','Rust'])assert.deepEqual(overlay.api.emitTypedProgramLinks(candidate.network,candidate.root,target),emitTypedProgramLinks(original.network,original.root,target));
  }
  const plain=createTypedProgramLinks('fn keep(value:i32)->i32{value}fn main(){}','Rust'),cached=decodeTypedProgramLinks(plain.network,plain.root),expected=[emitRust(cached),emitJavaScript(cached)];
  Object.defineProperty(cached,'inferredExpressionTypes',{value:new WeakMap()});
  const nullRoot=Object.assign(Object.create(null),cached);Object.defineProperty(nullRoot,'inferredExpressionTypes',{value:new WeakMap()});
  for(const program of[cached,nullRoot]){assert.equal(readOwnedRustTextCertificate(program),null);assert.deepEqual([overlay.rust.emitRust(program),overlay.javascript.emitJavaScript(program)],expected);}
  const code='pub fn choose(text:&String)->String{text.clone()} fn main(){}',program=captureOwnedRustTextCertificate(code),packet=overlay.api.encodeTypedProgramLinks(program),restored=overlay.api.decodeTypedProgramLinks(packet.network,packet.root);
  assert.equal(restored.sourceTextValueProjection.worker,restored.main);assert.equal(overlay.rust.emitRust(restored).text,code);assert.equal(overlay.api.emitTypedProgramLinks(packet.network,packet.root,'Rust').code,code);
 }finally{overlay.dispose();}
});
