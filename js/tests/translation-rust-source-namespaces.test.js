// Deferred actual frontend and retained source-namespace controls. Not executed.
import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {RustSourceOrigins} from '../src/translation/rust-source-origins.js';
import * as rules from '../src/translation/rust-source-namespace-rules.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {sourceNamespaceOriginals,sourceNamespaceRefusals,sourceNamespaceVersionRange} from './fixtures/rust-source-namespaces-draft.mjs';

test('shared namespace candidates preserve independent roles and lexical blocking',()=>{
 assert.equal(rules.readRustSourceNamespaceRole('fn'),'value');
 assert.equal(rules.readRustSourceNamespaceRole('module'),'type');
 assert.equal(rules.readRustSourceNamespaceCandidate('fn','type',['data']),'continue');
 assert.equal(rules.readRustSourceNamespaceCandidate('module','type',['data']),'stop');
 assert.equal(rules.readRustSourceNamespaceCandidate('data','type',['data']),'select');
 assert.equal(rules.readRustSourceNamespacePathRole(1,true),'value');
 assert.equal(rules.readRustSourceNamespacePathRole(2,true),'type');
 assert.equal(rules.acceptRustSourceNamespaceOuterCandidate(2,'module'),false);
 assert.equal(rules.acceptRustSourceNamespaceOuterCandidate(2,'fn'),true);
 assert.equal(rules.acceptRustSourceNamespaceDeclarationPair('externalCrate','fn'),true);
 assert.equal(rules.acceptRustSourceNamespaceDeclarationPair('externalCrate','module'),false);
});
test('homograph index survives passes in either declaration order',()=>{
 for(const kinds of [['module','fn'],['fn','module']]){
  const first=new RustSourceOrigins();
  for(const k of kinds)first.declaration('std',[],k==='fn'?{k,parameters:[],result:first.unknownOrigin([])}:{k});
  const next=new RustSourceOrigins(first.catalog);
  assert.equal(next.lookupTypeOwner(['std'],[]).k,'module');
  assert.equal(next.lookupValueOwner(['std'],[]).k,'fn');
 }
});
test('nearer type item blocks filtered and qualified outer candidates',()=>{
 const origins=new RustSourceOrigins(new Map([['X',{k:'data'}],['X.C',{k:'ctor'}],['inner.X',{k:'module'}],['inner.value',{k:'fn'}]]));
 assert.equal(origins.lookup(['X'],['inner'],['data']),null);
 assert.equal(origins.lookupValueOwner(['X','C'],['inner']),null);
 assert.equal(origins.lookup(['X'],['unrelated'],['data']).fullName,'X');
});
function observe(code){const lines=[];vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
for(const row of sourceNamespaceOriginals)for(const sourceVersion of ['1.90.0','1.99.0'])test(row.name+' / '+sourceVersion,()=>{
 assert.equal(sourceNamespaceVersionRange,'>=1.90.0 <1.100.0');
 const checked=checkProgram(parseRust(row.source,{sourceVersion}));assert.equal(observe(emitJavaScript(checked).text),row.stdout);
 const packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion});
 const text=packet.network.toLino(),binary=encodeLinkNetworkBinary(packet.network);
 for(const graph of [packet.network,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)]){
  const restored=decodeTypedProgramLinks(graph,packet.root);assert.deepEqual(restored.items,checked.items);assert.deepEqual(restored.main,checked.main);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(graph,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
  assert.equal(observe(emitTypedProgramLinks(graph,packet.root,'JavaScript').code),row.stdout);
 }
});
for(const row of sourceNamespaceRefusals)test(row.name,()=>{
 for(const sourceVersion of ['1.90.0','1.99.0'])assert.throws(()=>checkProgram(parseRust(row.source,{sourceVersion})),error=>['unsupported','type','syntax'].includes(error.kind));
});
