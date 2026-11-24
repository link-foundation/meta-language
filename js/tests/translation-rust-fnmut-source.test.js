// Deferred source/checked/Links controls. No compilation or original execution
// is performed by the source drafting workflow.
import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {checkRustFnMutSourceSpecialization} from '../src/translation/rust-fnmut-source-signatures.js';
import {readRustFnMutSourceModelError,readRustFnMutSourceDeclarationBudgetError} from '../src/translation/rust-fnmut-source-declaration-rules.js';
import {rustFnMutSourceOriginals as originals,rustFnMutSourceRefusals as refusals,rustFnMutSourceVersions as versions} from './fixtures/rust-fnmut-source.mjs';
const options={sourceVersion:'1.90.0',exclusiveSourceModel:versions.exclusiveModel,fnMutSourceModel:versions.model};
function nodes(root){const result=[],pending=[root],seen=new Set();while(pending.length){const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;assert.ok(seen.size<1000000);seen.add(value);result.push(value);pending.push(...(value instanceof Map?[...value.values()]:Object.values(value)));}return result;}
function output(text){const lines=[];vm.runInNewContext(text,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}},{timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
function activeNode(){const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered Node '+process.versions.node+'; '+versions.node);}
for(const row of originals)for(const sourceVersion of ['1.90.0','1.99.0'])test('actual FnMut original '+row.name+'/'+sourceVersion,()=>{
 activeNode();const surface=parseRust(row.source,{...options,sourceVersion}),checked=checkProgram(surface),all=nodes(checked),facets=all.filter(value=>value.sourceFnMutSpecialization);
 assert.ok(facets.length>0,'actual checked selected generic body exists');assert.equal(output(emitJavaScript(checked).text),row.stdout);
 assert.ok(!all.some(value=>value.sourceFnMutTemplate||String(value.op??'').startsWith('rustExclusiveSource')||String(value.op??'').startsWith('rustOwnedRecord')),'all source-only markers must be interpreted before public checked output');
 for(const fn of facets){const facet=fn.sourceFnMutSpecialization;assert.equal(facet.model,versions.model);assert.equal(facet.sourceVersion,sourceVersion);assert.equal(fn.sourceSignature,undefined);assert.equal(fn.sourceExclusiveSignature,undefined);assert.ok(facet.declaration.range.end<=row.source.length);assert.ok(row.source.slice(facet.declaration.range.start,facet.declaration.bodyRange.start).includes('FnMut'));assert.equal(facet.packet.length+1,fn.ret.elements.length);}
 const concrete=LinkNetwork.parse(row.source,'Rust');for(const graph of [concrete,LinkNetwork.fromLino(concrete.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(concrete))])assert.equal(graph.reconstructText(),row.source);
});
for(const row of refusals)test('retained FnMut source boundary '+row.name,()=>{assert.equal(typeof row.compilerAccepted,'boolean');assert.throws(()=>checkProgram(parseRust(row.source,options)),error=>['type','unsupported'].includes(error.kind));});
test('closed FnMut remains explicit and bounded by actual source versions',()=>{
 assert.equal(readRustFnMutSourceModelError(versions.model,90),'');assert.equal(readRustFnMutSourceModelError(versions.model,99),'');for(const minor of [0,89,100,NaN])assert.notEqual(readRustFnMutSourceModelError(versions.model,minor),'');assert.notEqual(readRustFnMutSourceModelError('',90),'');
 assert.equal(readRustFnMutSourceDeclarationBudgetError(100,2,1,1),'');for(const args of [[1000001,2,1,1],[1,257,1,1],[1,2,33,1],[1,2,1,0],[1,2,1,2]])assert.notEqual(readRustFnMutSourceDeclarationBudgetError(...args),'');
 for(const sourceVersion of ['1.89.0','1.100.0','2.90.0'])assert.throws(()=>parseRust(originals[0].source,{...options,sourceVersion}));
 assert.throws(()=>parseRust(originals[0].source,{sourceVersion:'1.90.0',exclusiveSourceModel:versions.exclusiveModel}));assert.throws(()=>parseRust(originals[0].source,{...options,exclusiveSourceModel:''}));
});
test('checked source captures retain actual binder identity and ordered writebacks',()=>{
 const checked=checkProgram(parseRust(originals[3].source,options)),fn=nodes(checked).find(value=>value.sourceFnMutSpecialization),facet=fn.sourceFnMutSpecialization;
 assert.equal(facet.callback.captures.length,2);assert.deepEqual(facet.callback.captures.map(value=>value.binder.name),['first','second']);assert.equal(new Set(facet.callback.captures.map(value=>value.binder.identity)).size,2);
 for(const capture of facet.callback.captures)assert.equal(originals[3].source.slice(capture.binder.start,capture.binder.end),capture.binder.name);
 for(const change of [value=>value.packet.reverse(),value=>value.callback.captures[1].binder.identity=value.callback.captures[0].binder.identity,value=>value.callback.captures[0].parameter=0,value=>value.declaration.callback.mutable=false,value=>value.model='Rust source function signature 2']){const forged=structuredClone(facet);change(forged);assert.throws(()=>checkRustFnMutSourceSpecialization(forged,fn.params.map(value=>value.type),fn.ret,'Rust'),error=>error.kind==='type');}
});
test('fallible traversal lowers real callback source into a first-error fold',()=>{
 const checked=checkProgram(parseRust(originals[11].source,options)),all=nodes(checked);assert.ok(all.some(value=>value.kind==='arrayReduce'||value.op==='reduce'||value.k==='method'&&value.op==='reduce'));assert.equal(output(emitJavaScript(checked).text),'true\n');
});
test('closed source FnMut survives typed Links storage and all emitters',()=>{
 const row=originals[2],packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:options.sourceVersion,rustExclusiveSourceModel:versions.exclusiveModel,rustFnMutSourceModel:versions.model}),text=packet.network.toLino(),bytes=encodeLinkNetworkBinary(packet.network),targets=['JavaScript','Rust','Lean','Rocq'],baseline=Object.fromEntries(targets.map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]));
 for(const graph of [packet.network,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(bytes)]){assert.equal(graph.toLino(),text);assert.deepEqual(encodeLinkNetworkBinary(graph),bytes);assert.ok(nodes(decodeTypedProgramLinks(graph,packet.root)).some(value=>value.sourceFnMutSpecialization));for(const target of targets){const emitted=emitTypedProgramLinks(graph,packet.root,target);assert.deepEqual(emitted,baseline[target]);if(target==='JavaScript')assert.equal(output(emitted.code),row.stdout);}}
});
test('same original generic body specializes distinct concrete errors without cache pollution',()=>{
 const source='fn apply<E>(value: u8, mut map: impl FnMut(u8) -> Result<u8, E>) -> Result<u8, E> { map(value) } fn main() { let first: Result<u8, u8> = apply::<u8>(7, |value| Ok(value)); let second: Result<u8, String> = apply::<String>(7, |_value| Err(String::from("error"))); println!("{}", (match first { Ok(value) => value == 7, Err(_) => false }) && (match second { Err(value) => value == "error", Ok(_) => false })); }';
 const checked=checkProgram(parseRust(source,options)),facets=nodes(checked).filter(value=>value.sourceFnMutSpecialization).map(value=>value.sourceFnMutSpecialization);assert.equal(facets.length,2);assert.equal(new Set(facets.map(value=>JSON.stringify(value.typeArguments))).size,2);assert.equal(output(emitJavaScript(checked).text),'true\n');
});
test('whole moved Vec root can be assigned a genuine new owner without reviving moved fields',()=>{
 const source='fn main() { let mut values: Vec<u8> = vec![7]; for _value in values { break; } values = vec![9]; println!("{}", values.len() == 1 && values[0] == 9); }';assert.equal(output(emitJavaScript(checkProgram(parseRust(source,options))).text),'true\n');
 const invalid='struct Pair { first: String, second: String } fn main() { let mut pair = Pair { first: String::from("a"), second: String::from("b") }; let moved = pair; pair.first = String::from("c"); }';assert.throws(()=>checkProgram(parseRust(invalid,options)),error=>['type','unsupported'].includes(error.kind));
});
