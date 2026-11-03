// Deferred controls: source policy, primitive order, actual state writeback and
// public graph transports. No behavior or source authoring runs on import.
import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readRustExclusiveSourceCompoundOperation,readRustExclusiveSourceCompoundPlace,acceptRustExclusiveSourceCompoundPrimitive} from '../src/translation/rust-exclusive-source-compound-rules.js';
import {rustExclusiveSourceCompoundOriginals as originals,rustExclusiveSourceCompoundRefusals as refusals,rustExclusiveSourceCompoundVersions as versions} from './fixtures/rust-exclusive-source-compounds.mjs';
function output(code){const lines=[];vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
for(const row of originals)for(const sourceVersion of ['1.90.0','1.99.0'])test('exclusive compound actual source '+row.name+'/'+sourceVersion,()=>{
 const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered Node '+process.versions.node+'; '+versions.node);
 const checked=checkProgram(parseRust(row.source,{sourceVersion,exclusiveSourceModel:versions.model}));assert.equal(output(emitJavaScript(checked).text),row.stdout);
 const concrete=LinkNetwork.parse(row.source,'Rust');for(const graph of [concrete,LinkNetwork.fromLino(concrete.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(concrete))])assert.equal(graph.reconstructText(),row.source);
});
for(const row of refusals)test('exclusive compound retained source boundary '+row.name,()=>{assert.equal(typeof row.compilerAccepted,'boolean');assert.throws(()=>checkProgram(parseRust(row.source,{sourceVersion:'1.90.0',exclusiveSourceModel:versions.model})),error=>['type','unsupported'].includes(error.kind));});
test('compound source policy requires its genuine complete primitive owner',()=>{
 for(const [token,operation]of [['+=','add'],['-=','sub'],['*=','mul'],['/=','div'],['%=','rem']])assert.equal(readRustExclusiveSourceCompoundOperation(token),operation);
 for(const token of ['=','&=','|=','<<=','user'])assert.equal(readRustExclusiveSourceCompoundOperation(token),'');
 assert.equal(acceptRustExclusiveSourceCompoundPrimitive('fixed','8',false,'primitive','u8',0,true,0),true);assert.equal(acceptRustExclusiveSourceCompoundPrimitive('float','',false,'primitive','f64',0,true,0),true);
 for(const facts of [['fixed','8',false,'declaration','u8',0,true,0],['fixed','16',false,'primitive','u8',0,true,0],['fixed','8',true,'primitive','u8',0,true,0],['fixed','8',false,'primitive','u8',0,false,0],['fixed','8',false,'primitive','u8',0,true,1],['bool','',false,'primitive','bool',0,true,0]])assert.equal(acceptRustExclusiveSourceCompoundPrimitive(...facts),false);
});
test('compound place plans preserve prefix postfix and grouping order independently of source owners',()=>{
 const plans=[
  {forms:['name','}'],expected:[1,0]},
  {forms:['*','*','name','}'],expected:[3,2,-1,-1]},
  {forms:['*','name','.','name','}'],expected:[4,1,3,-1]},
  {forms:['(','*','name',')','.','name','}'],expected:[6,2,-1,5]},
  {forms:['*','(','(','*','name',')','.','name',')','}'],expected:[9,4,-1,7,-1]},
  {forms:['(','name','.','number',')','.','name','}'],expected:[7,1,3,6]},
 ];for(const row of plans)assert.deepEqual(readRustExclusiveSourceCompoundPlace(row.forms),row.expected);
 for(const forms of [[],['*','}'],['name','(',')','}'],['name','.','}'],['name','.','*','name','}'],['(','name','}'],['name',')','}'],['name','other','}'],[...Array(256).fill('*'),'name','}'],Array(1026).fill('name')])assert.deepEqual(readRustExclusiveSourceCompoundPlace(forms),[]);
 const maximum=readRustExclusiveSourceCompoundPlace([...Array(255).fill('*'),'name','}']);assert.equal(maximum[0],256);assert.equal(maximum[1],255);assert.deepEqual(maximum.slice(2),Array(255).fill(-1));
});
test('primitive RHS-before-place semantics survive typed Links and every emitter',()=>{
 const row=originals[4],packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:'1.90.0',rustExclusiveSourceModel:versions.model}),text=packet.network.toLino(),binary=encodeLinkNetworkBinary(packet.network),targets=['JavaScript','Rust','Lean','Rocq'],baseline=Object.fromEntries(targets.map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]));
 for(const graph of [packet.network,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)]){assert.equal(graph.toLino(),text);assert.deepEqual(encodeLinkNetworkBinary(graph),binary);const restored=decodeTypedProgramLinks(graph,packet.root);assert.ok(restored.items.length>0);for(const target of targets){const emitted=emitTypedProgramLinks(graph,packet.root,target);assert.deepEqual(emitted,baseline[target]);if(target==='JavaScript')assert.equal(output(emitted.code),row.stdout);}}
});
test('unselected source profile keeps its original primitive mutation boundary',()=>{assert.throws(()=>parseRust(originals[0].source,{sourceVersion:'1.90.0'}),error=>error.kind==='unsupported');});
