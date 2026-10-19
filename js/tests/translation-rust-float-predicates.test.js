import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { rustFloatPredicateFixture } from '../scripts/generate-rust-float-predicate-fixtures.mjs';
import { parseRust } from '../src/translation/rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { LinkNetwork } from '../src/network.js';
import { LinkId } from '../src/primitives.js';
import { emitTypedProgramLinks, decodeTypedProgramLinks } from '../src/typed-program-links.js';
import { acceptRustFloatPredicateMethod, acceptRustFloatPredicateReceiver } from '../src/translation/rust-float-predicate-rules.js';
import { RUST_FLOAT_PREDICATE_POLICY, createRustFloatPredicateExpression } from '../src/translation/rust-float-predicates.js';

const fixture=readProjectData(new URL('../../parity/fixtures/rust-float-predicates.lino',import.meta.url));
function output(source, extra={}) {
 const lines=[];
 vm.runInNewContext(source,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},...extra},{timeout:1000});
 return lines;
}
function expressions(value, predicate, result=[]) {
 if (!value || typeof value !== 'object') return result;
 if (predicate(value)) result.push(value);
 for (const child of Object.values(value)) expressions(child,predicate,result);
 return result;
}

test('independent Rust f64 and JavaScript predicate sources preserve all floating classes',()=>{
 assert.deepEqual(rustFloatPredicateFixture(),fixture);
 for(const entry of fixture.cases){
  assert.deepEqual(output(entry.originalJavaScript),entry.output,entry.id);
  assert.deepEqual(output(emitJavaScript(checkProgram(parseJavaScript(entry.originalJavaScript))).text),entry.output,entry.id);
  assert.deepEqual(output(entry.javascript),entry.output,entry.id);
  assert.deepEqual(JSON.parse(JSON.stringify(parseRust(entry.source))),entry.surface,entry.id);
  const network=LinkNetwork.fromLino(entry.archive),root=LinkId.from(entry.root);
  assert.deepEqual(output(emitTypedProgramLinks(network,root,'JavaScript').code),entry.output,entry.id);
  assert.equal(emitTypedProgramLinks(network,root,'Rust').code,entry.rust,entry.id);
  assert.ok(network.links().every(link=>!link.metadata().definition?.includes(entry.source)),entry.id);
 }
});

test('wrong arity, custom and shadowed owners, f32 and unproven numeric receivers refuse exactly',()=>{
 for(const entry of fixture.invalid){
  assert.throws(()=>checkProgram(parseRust(entry.source)),error=>{
   assert.equal(error.kind,entry.kind,entry.id);
   assert.equal(error.reason,entry.reason,entry.id);
   assert.deepEqual(error.span??null,entry.span,entry.id);
   assert.ok(entry.span && entry.span.end>entry.span.start,entry.id);
   return true;
  });
 }
});

test('linked template infinity predicate evaluates its actual function receiver once',()=>{
 const zero=fixture.cases.find(value=>value.id==='typed-negative-zero');
 const zeroProgram=decodeTypedProgramLinks(LinkNetwork.fromLino(zero.archive),zero.root);
 assert.equal(expressions(zeroProgram,node=>node.k==='lit'&&node.type?.kind==='float'&&node.value==='-0').length,1);
 const entry=fixture.cases.find(value=>value.id==='receiver-call-once');
 const network=LinkNetwork.fromLino(entry.archive),root=LinkId.from(entry.root);
 const checked=decodeTypedProgramLinks(network,root);
 const target=checked.items.find(item=>item.name==='test');
 assert.equal(expressions(target.body,node=>node.k==='call'&&node.fn==='provider').length,1);
 const state={receiverCalls:0};
 const code=emitTypedProgramLinks(network,root,'JavaScript').code.replace('function provider() {','function provider() { globalThis.counter.receiverCalls += 1;');
 assert.deepEqual(output(code,{counter:state}),entry.output);
 assert.equal(state.receiverCalls,1);
 for(const [method,template] of Object.entries(RUST_FLOAT_PREDICATE_POLICY.templates)){
  assert.equal(expressions(template,node=>node.slot==='receiver').length,1,method);
  const receiver={k:'name',path:['actual'],span:{start:5,end:11}};
  const result=createRustFloatPredicateExpression(method,receiver,{start:11,end:21});
  assert.equal(expressions(result,node=>node===receiver).length,1,method);
  assert.deepEqual(result.span,{start:11,end:21});
  assert.deepEqual(receiver.span,{start:5,end:11});
 }
});

test('shared predicate source policy requires actual type/model/provenance evidence',()=>{
 for(const method of ['is_nan','is_finite','is_infinite']) assert.equal(acceptRustFloatPredicateMethod(method),true);
 for(const method of ['nan','isNaN','is_normal','toString']) assert.equal(acceptRustFloatPredicateMethod(method),false);
 assert.equal(acceptRustFloatPredicateReceiver('float','binary64 scalar',true,false,false,'0'),true);
 for(const [kind,model,proven,indirected,shadowed,arity] of [
  ['fixed','binary64 scalar',true,false,false,'0'],['float','',true,false,false,'0'],['float','binary64 scalar',false,false,false,'0'],
  ['float','binary64 scalar',true,true,false,'0'],['float','binary64 scalar',true,false,true,'0'],['float','binary64 scalar',true,false,false,'1'],
 ])assert.equal(acceptRustFloatPredicateReceiver(kind,model,proven,indirected,shadowed,arity),false);
});

test('source-free operation edits change predicate meaning through checked math links',()=>{
 const entry=fixture.cases.find(value=>value.id==='explicit-f64-literal');
 const initial=LinkNetwork.fromLino(entry.archive);
 const operation=initial.links().find(link=>link.metadata().definition==='isFinite');
 assert.ok(operation);
 const changed=LinkNetwork.fromLino(initial.toLino().split('\n').map(line=>line.startsWith('('+operation.id()+': ')?line.replace('(def: isFinite)','(def: isNaN)'):line).join('\n'));
 assert.deepEqual(output(emitTypedProgramLinks(changed,LinkId.from(entry.root),'JavaScript').code),['false']);
 assert.match(emitTypedProgramLinks(changed,LinkId.from(entry.root),'Rust').code,/\.is_nan\(\)/u);
});
