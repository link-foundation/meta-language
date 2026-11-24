// Deferred actual frontend/transport controls; never execute while drafting.
import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {RustSourceOrigins} from '../src/translation/rust-source-origins.js';
import {STRING,rustFixedType} from '../src/translation/types.js';
import {selectsRustOwnedStandardSumCallback} from '../src/translation/rust-owned-standard-sum-callback-selection.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import * as rules from '../src/translation/rust-owned-standard-sum-callback-rules.js';
import {rustStandardSumCopyOwnedCallbackDecisions as decisions,rustStandardSumCopyOwnedCallbackOriginals as originals,rustStandardSumCopyOwnedCallbackRefusals as refusals} from './fixtures/rust-standard-sum-copy-owned-callbacks.mjs';
const versions=['1.90.0','1.99.0'],targets=['JavaScript','Rust','Lean','Rocq'];
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered Copy-to-owned callback Node '+process.versions.node);
function stdout(code){const lines=[];vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
for(const [index,row]of decisions.entries())test('Copy-to-owned selection decision '+index+' '+row.operation,()=>assert.deepEqual(rules[row.operation](...row.arguments),row.expected));
test('all independently authored Copy-to-owned inputs remain represented',()=>{assert.equal(decisions.length,16);assert.equal(originals.length,14);assert.equal(refusals.length,5);});
// Transport premise controls deliberately construct tiny actual catalog rows;
// selection is not type/callback admission, which the complete source tests do.
function context(values){const origins=new RustSourceOrigins(new Map());return {origins,modulePath:[],sourceTypeAliases:new Map(),sourceTypeAliasCache:new Map(),cursor:{index:0,tokens:values.map((value,index)=>({value,kind:/^[a-zA-Z_][a-zA-Z_0-9]*$/u.test(value)?'identifier':'punctuation',start:index,end:index+1}))},resolve:names=>names,ordinaryCompletionContext:()=>true,type:()=>{throw new Error('lookahead must not parse a type');},expr:()=>{throw new Error('lookahead must not parse an expression');}};}
test('actual named result roles select without changing cursor or allocating a type',()=>{
 const parser=context(['(','mapper',')']),parameter=parser.origins.typeOrigin(rustFixedType('u8'),[]),result=parser.origins.typeOrigin(STRING,[]);
 parser.origins.registerCatalogEntry('mapper',{k:'fn',parameters:[parameter],result});assert.equal(selectsRustOwnedStandardSumCallback(parser,[]),true);assert.equal(parser.cursor.index,0);
 parser.origins.registerCatalogEntry('mapper',{k:'fn',parameters:[parameter],result:parameter});assert.equal(selectsRustOwnedStandardSumCallback(parser,[]),false);assert.equal(parser.cursor.index,0);
 parser.origins.scope.set('mapper',parameter);assert.equal(selectsRustOwnedStandardSumCallback(parser,[]),false);assert.equal(parser.cursor.index,0);
});
test('actual alias and namespace shadows prevent a written library-result selection',()=>{
 const values=['(','|','value',':','u8','|','->','String','{'];
 for(const mutation of [parser=>parser.sourceTypeAliases.set('String',{}),parser=>parser.origins.registerCatalogEntry('String',{k:'data'})]){const parser=context(values);mutation(parser);assert.equal(selectsRustOwnedStandardSumCallback(parser,[]),false);assert.equal(parser.cursor.index,0);}
 const parser=context(['(','||','->','std','::','string','::','String','{']);parser.origins.registerCatalogEntry('std',{k:'module'});assert.equal(selectsRustOwnedStandardSumCallback(parser,[]),false);assert.equal(parser.cursor.index,0);
});
for(const row of originals)for(const sourceVersion of versions)test('actual Copy-to-owned source '+row.name+'/'+sourceVersion,()=>{
 assert.equal(row.sourceVersionRange,'>=1.90.0 <1.100.0');const concrete=LinkNetwork.parse(row.source,'Rust'),text=concrete.toLino(),binary=encodeLinkNetworkBinary(concrete);
 for(const graph of [concrete,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)])assert.equal(graph.reconstructText(),row.source);
 const checked=checkProgram(parseRust(row.source,{sourceVersion}));assert.equal(stdout(emitJavaScript(checked).text),row.stdout);
 const packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion,provenance:{fixture:row.name}}),typedText=packet.network.toLino(),typedBinary=encodeLinkNetworkBinary(packet.network);
 const baseline=Object.fromEntries(targets.map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]));
 for(const graph of [packet.network,LinkNetwork.fromLino(typedText),decodeLinkNetworkBinary(typedBinary)]){
  assert.equal(graph.toLino(),typedText);const restored=decodeTypedProgramLinks(graph,packet.root);assert.deepEqual(restored.items,checked.items);assert.deepEqual(restored.main,checked.main);
  for(const target of targets){const emitted=emitTypedProgramLinks(graph,packet.root,target);assert.deepEqual(emitted,baseline[target]);assert.ok(emitted.code.length>0);if(target==='JavaScript')assert.equal(stdout(emitted.code),row.stdout);}
  assert.deepEqual(encodeLinkNetworkBinary(graph),typedBinary);
 }
});
for(const row of refusals)for(const sourceVersion of versions)test('actual Copy-to-owned callback refusal '+row.name+'/'+sourceVersion,()=>{
 assert.ok(['accept','reject'].includes(row.originalCompilerOutcome));
 for(const read of [()=>checkProgram(parseRust(row.source,{sourceVersion})),()=>createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion})])assert.throws(read,error=>{assert.ok(['syntax','type','unsupported'].includes(error.kind));assert.ok(error.reason.length>0);return true;});
});
