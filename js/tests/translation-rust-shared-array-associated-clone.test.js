import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {rustSharedArrayHandleRulesPolicy} from '../scripts/generate-rust-shared-array-handle-rules.mjs';
import {acceptRustSharedArrayHandleSourceModel,acceptRustSharedArrayHandleSourceInterpretation,acceptRustSharedArrayHandleBorrowedClone} from '../src/translation/rust-shared-array-handle-rules.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {LinkNetwork} from '../src/network.js';
import {selfTranslate,selfTranslationSignatures} from '../src/self-translation.js';
const policy=rustSharedArrayHandleRulesPolicy(),model=policy.associatedCloneModel;
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+policy.assertionVersions.Node);
const context=source=>{const result={};vm.runInNewContext(source,result,{timeout:1000});return result;};

test('actual associated Rc clones preserve all scalar domains and source-free identity after both codecs',()=>{
 assert.equal(policy.associatedCloneCases.length,3);
 for(const row of policy.associatedCloneCases){
  const original=context(row.javascriptSource);assert.equal(original.probe(),row.expected,row.name);
  const surface=parseRust(row.source,{sharedArrayHandleSourceModel:model}),program=checkProgram(surface);
  assert.equal(row.source.slice(surface.items.find(item=>item.name==='identity').body.span.start,surface.items.find(item=>item.name==='identity').body.span.end),'std::rc::Rc::clone(&std::rc::Rc::clone(&value))');
  const packet=encodeTypedProgramLinks(program),networks=[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];
  for(const current of [program,...networks.map(network=>recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true}))]){
   const actual=context(emitJavaScript(current).text);assert.equal(actual.probe(),row.expected);
   const input=original.fresh(),before=[...input];assert.equal(actual.identity(input),input,'associated cloning preserves the exact input identity');assert.deepEqual([...input],before);
   const fresh=actual.fresh();assert.deepEqual([...fresh],before);assert.notStrictEqual(fresh,actual.fresh());
   if(before.some(value=>Object.is(value,-0)))assert.ok([...fresh].some(value=>Object.is(value,-0)));
  }
  assert.throws(()=>parseRust(row.source,{sharedArrayHandleSourceModel:policy.model}),/associated clone/u,'the earlier source model retains its original domain');
 }
});

test('whole associated-clone libraries publish checked signatures and restore every original source byte',()=>{
 for(const row of policy.associatedCloneCases){
  const options={rustSharedArrayHandleSourceModel:model};
  const publication=selfTranslate(row.source,'Rust','JavaScript',options);
  assert.ok(publication.items.every(item=>['translated','comment'].includes(item.status)),JSON.stringify(publication.items));
  assert.deepEqual(selfTranslationSignatures(row.source,'Rust',options).map(signature=>signature.name),['identity','probe']);
  assert.equal(selfTranslate(publication.code,'JavaScript','Rust').code,row.source);
 }
});

test('public graph and module entry points retain the selected associated-clone source model',async()=>{
 const api=await import('../src/index.js');
 for(const row of policy.associatedCloneCases){
  const packet=api.createTypedProgramLinks(row.source,'Rust',{rustSharedArrayHandleSourceModel:model});
  const network=api.LinkNetwork.fromLino(packet.network.toLino());
  const emitted=api.emitTypedProgramLinks(network,packet.root,'JavaScript');
  const actual=context(emitted.code),value=actual.fresh();assert.equal(actual.identity(value),value);assert.equal(actual.probe(),row.expected);
  const publication=api.selfTranslate(row.source,'Rust','JavaScript',{rustSharedArrayHandleSourceModel:model});
  assert.equal(api.selfTranslate(publication.code,'JavaScript','Rust').code,row.source);
 }
});

test('associated clones evaluate borrowed call operands once in original source order',()=>{
 const handle='std::rc::Rc<std::cell::RefCell<Vec<bool>>>',fresh='std::rc::Rc::new(std::cell::RefCell::new(vec![true]))';
 const source='fn left()->'+handle+'{'+fresh+'}fn right()->'+handle+'{'+fresh+'}pub fn probe()->bool{std::rc::Rc::ptr_eq(&std::rc::Rc::clone(&left()),&std::rc::Rc::clone(&right()))}';
 const program=checkProgram(parseRust(source,{sharedArrayHandleSourceModel:model})),packet=encodeTypedProgramLinks(program);
 for(const current of [program,...[LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))].map(network=>recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true}))]){
  const actual=context(emitJavaScript(current).text),calls=[];
  for(const name of ['left','right']){const original=actual[name];actual[name]=()=>{calls.push(name);return original();};}
  assert.equal(actual.probe(),false);assert.deepEqual(calls,['left','right']);
 }
});

test('associated clones refuse wrong owners arity payloads mutable borrows and escaping references',()=>{
 assert.equal(acceptRustSharedArrayHandleSourceModel(model),false,'the original declaration keeps its exact preceding domain');
 for(const admitted of ['',policy.model,model])assert.equal(acceptRustSharedArrayHandleSourceInterpretation(admitted),true);
 assert.equal(acceptRustSharedArrayHandleSourceInterpretation('standard Rust shared array identity handles 3'),false);
 assert.equal(acceptRustSharedArrayHandleBorrowedClone(model,1,true,true,true),true);
 for(const args of [[policy.model,1,true,true,true],[model,0,true,true,true],[model,2,true,true,true],[model,1,false,true,true],[model,1,true,false,true],[model,1,true,true,false]])assert.equal(acceptRustSharedArrayHandleBorrowedClone(...args),false);
 assert.equal(policy.associatedCloneRefusals.length,10);
 for(const row of policy.associatedCloneRefusals){
  assert.throws(()=>parseRust(row.source,{sharedArrayHandleSourceModel:model}),row.name);
  assert.throws(()=>selfTranslate(row.source,'Rust','JavaScript',{rustSharedArrayHandleSourceModel:model}),row.name);
  assert.throws(()=>selfTranslationSignatures(row.source,'Rust',{rustSharedArrayHandleSourceModel:model}),row.name);
 }
});
