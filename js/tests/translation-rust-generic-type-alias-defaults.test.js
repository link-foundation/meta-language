// Deferred source controls. No execution until the complete source draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readRustGenericTypeAliasParameterDirectory,readRustGenericTypeAliasDefaultSubstitutionPlan,readRustGenericTypeAliasDefaultArgumentPlan,acceptRustGenericTypeAliasConcreteIdentifier} from '../src/translation/rust-generic-type-alias-rules.js';
import {genericAliasDefaultOriginals,genericAliasDefaultRefusals,genericAliasDefaultVersionRange,defaultParameterDirectories} from './fixtures/rust-generic-type-alias-defaults-draft.mjs';
for(const [index,row]of defaultParameterDirectories.entries())test('actual default parameter token directory '+index,()=>assert.deepEqual(readRustGenericTypeAliasParameterDirectory(row.kinds,row.values,row.raws,row.position),row.expected));
test('default substitution selects only earlier actual binding identities',()=>{
 assert.deepEqual(readRustGenericTypeAliasDefaultSubstitutionPlan(['owner/T','owner/U'],1,1,['owner/T']),['substitution','0']);
 assert.deepEqual(readRustGenericTypeAliasDefaultSubstitutionPlan(['owner/T'],0,0,[]),['substitution']);
 assert.equal(readRustGenericTypeAliasDefaultSubstitutionPlan(['owner/T','owner/U'],0,0,['owner/U'])[0],'error');
 assert.equal(readRustGenericTypeAliasDefaultSubstitutionPlan(['owner/T'],0,0,['caller/T'])[0],'error');
 assert.deepEqual(readRustGenericTypeAliasDefaultArgumentPlan([false,true],1,true),['arguments','1']);
 assert.deepEqual(readRustGenericTypeAliasDefaultArgumentPlan([true,true],0,false),['arguments','0','1']);
 assert.equal(readRustGenericTypeAliasDefaultArgumentPlan([false,true],0,false)[0],'error');
 assert.equal(readRustGenericTypeAliasDefaultArgumentPlan([true],2,true)[0],'error');
 assert.equal(acceptRustGenericTypeAliasConcreteIdentifier(['T'],'T',true),true);
 assert.equal(acceptRustGenericTypeAliasConcreteIdentifier(['T'],'T',false),false);
});
function observe(code){const lines=[];vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
for(const row of genericAliasDefaultOriginals)for(const sourceVersion of ['1.90.0','1.99.0'])test(row.name+' / '+sourceVersion,()=>{
 assert.equal(genericAliasDefaultVersionRange,'>=1.90.0 <1.100.0');
 const checked=checkProgram(parseRust(row.source,{sourceVersion}));assert.equal(observe(emitJavaScript(checked).text),row.stdout);
 const packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion});const text=packet.network.toLino(),binary=encodeLinkNetworkBinary(packet.network);
 for(const graph of [packet.network,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)]){
  const restored=decodeTypedProgramLinks(graph,packet.root);assert.deepEqual(restored.items,checked.items);assert.deepEqual(restored.main,checked.main);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(graph,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
  assert.equal(observe(emitTypedProgramLinks(graph,packet.root,'JavaScript').code),row.stdout);
 }
});
for(const row of genericAliasDefaultRefusals)test(row.name,()=>{
 for(const sourceVersion of ['1.90.0','1.99.0'])assert.throws(()=>checkProgram(parseRust(row.source,{sourceVersion})),error=>['unsupported','type','syntax'].includes(error.kind));
});
