// Deferred contracts: source-authored assertions, no execution observations.
import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import * as rules from '../src/translation/rust-copy-closure-rules.js';
import {assignRustCopyClosureLexicalBinding,readRustCopyClosureValueBorrows,validateRustCopyClosureValueBorrows,carryRustCopyClosureValueUnion,projectRustCopyClosureValueField} from '../src/translation/rust-copy-closure-lifetimes.js';
import {cloneRustCallableTypeOrigin} from '../src/translation/rust-function-pointers.js';
import {rustCopyClosureValueDecisions as decisions,rustCopyClosureValueOriginals as originals,rustCopyClosureValueRefusals as refusals} from './fixtures/rust-copy-closure-values.mjs';
const versions=['1.90.0','1.99.0'],targets=['JavaScript','Rust','Lean','Rocq'];
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered Copy callable value Node '+process.versions.node);
function stdout(code){const lines=[];vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
for(const [index,row]of decisions.entries())test('actual Copy callable value decision '+index+' '+row.operation,()=>assert.deepEqual(rules[row.operation](...row.arguments),row.expected));
test('all independent lexical callable inputs stay represented',()=>{assert.equal(decisions.length,20);assert.equal(originals.length,17);assert.equal(refusals.length,11);});
test('lexical declarations have distinct identities even when values are copied',()=>{
 const table={collecting:false,nextSourceBindingIdentity:0,sourceBindingStates:new Map()};
 const first=assignRustCopyClosureLexicalBinding(table,{}),second=assignRustCopyClosureLexicalBinding(table,{...first});
 assert.notEqual(first.sourceLexicalBindingIdentity,second.sourceLexicalBindingIdentity);
 const copiedScope=new Map([['first',first]]);assert.equal(copiedScope.get('first').sourceLexicalBindingIdentity,first.sourceLexicalBindingIdentity);
 assert.equal(table.sourceBindingStates.get(first.sourceLexicalBindingIdentity),'available');
});
test('contained callable borrows survive unions and expire with actual lexical states',()=>{
 const table={collecting:false,nextSourceBindingIdentity:0,sourceBindingStates:new Map()};
 const binding=assignRustCopyClosureLexicalBinding(table,{}),id=binding.sourceLexicalBindingIdentity;
 const callable={type:{kind:'function',params:[],ret:{kind:'unit'}},wrappers:[],sourceValueBorrowBindings:[id]};
 const wrapped=carryRustCopyClosureValueUnion({type:{kind:'named',path:['UnknownContainer']},wrappers:[]},[callable]);
 assert.deepEqual(readRustCopyClosureValueBorrows(wrapped),[id]);validateRustCopyClosureValueBorrows(table,wrapped);
 table.sourceBindingStates.set(id,'dropped');assert.throws(()=>validateRustCopyClosureValueBorrows(table,wrapped),/does not outlive/);
 const scalar=projectRustCopyClosureValueField({sourceValueFieldBorrowBindings:[[id],[]]},{type:{kind:'bool'},wrappers:[]},1);
 assert.deepEqual(readRustCopyClosureValueBorrows(scalar),[]);
});
test('stored callable TYPE contracts never retain actual lexical loans',()=>{
 const actual={type:{kind:'function',params:[],ret:{kind:'unit'}},path:[],wrappers:[],proven:true,copyEvidence:true,sourceLexicalBindingIdentity:'lexical outer',sourceValueBorrowBindings:['lexical capture'],sourceValueFieldBorrowBindings:[['lexical nested']],copyClosureSpan:{start:1,end:2},copyClosureCaptureCount:1};
 const contract=cloneRustCallableTypeOrigin(actual);
 assert.deepEqual(readRustCopyClosureValueBorrows(contract),[]);assert.ok(!contract.sourceLexicalBindingIdentity);
 assert.deepEqual(contract.copyClosureSpan,actual.copyClosureSpan);assert.equal(contract.copyClosureCaptureCount,1);
 assert.deepEqual(actual.sourceValueBorrowBindings,['lexical capture']);
});
for(const row of originals)for(const sourceVersion of versions)test('actual lexical callable source '+row.name+'/'+sourceVersion,()=>{
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
for(const row of refusals)for(const sourceVersion of versions)test('actual lexical callable refusal '+row.name+'/'+sourceVersion,()=>{
 assert.ok(['accept','reject'].includes(row.originalCompilerOutcome));
 for(const read of [()=>checkProgram(parseRust(row.source,{sourceVersion})),()=>createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion})])assert.throws(read,error=>{assert.ok(['syntax','type','unsupported'].includes(error.kind));assert.ok(error.reason.length>0);return true;});
});
