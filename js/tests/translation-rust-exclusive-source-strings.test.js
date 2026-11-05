// Deferred String source controls; no originals execute on import.
import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {readRustExclusiveSourceStringMethodArity,acceptRustExclusiveSourceStringReceiver,acceptRustExclusiveSourceStringSuffix} from '../src/translation/rust-exclusive-source-string-rules.js';
import {rustExclusiveSourceStringOriginals as originals,rustExclusiveSourceStringRefusals as refusals,rustExclusiveSourceStringVersions as versions} from './fixtures/rust-exclusive-source-strings.mjs';
function output(code){const lines=[];vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}}, {timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
for(const row of originals)for(const sourceVersion of ['1.90.0','1.99.0'])test('exclusive String actual source '+row.name+'/'+sourceVersion,()=>{
 const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered Node '+process.versions.node+'; '+versions.node);
 const checked=checkProgram(parseRust(row.source,{sourceVersion,exclusiveSourceModel:versions.model}));assert.equal(output(emitJavaScript(checked).text),row.stdout);
 const concrete=LinkNetwork.parse(row.source,'Rust');for(const graph of [concrete,LinkNetwork.fromLino(concrete.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(concrete))])assert.equal(graph.reconstructText(),row.source);
});
for(const row of refusals)test('exclusive String retained source boundary '+row.name,()=>{assert.equal(typeof row.compilerAccepted,'boolean');assert.throws(()=>checkProgram(parseRust(row.source,{sourceVersion:'1.90.0',exclusiveSourceModel:versions.model})),error=>['type','unsupported'].includes(error.kind));});
test('String receiver and suffix rules preserve actual owned and shared source modes',()=>{
 assert.equal(readRustExclusiveSourceStringMethodArity('push_str'),1);assert.equal(readRustExclusiveSourceStringMethodArity('clear'),0);for(const method of ['push','truncate','insert_str','reserve','capacity','len'])assert.equal(readRustExclusiveSourceStringMethodArity(method),-1);
 assert.equal(acceptRustExclusiveSourceStringReceiver('string','owned string',0,true,'library','std::string::String',0),true);
 for(const row of [['string','owned string',1,true,'library','std::string::String',0],['string','owned string',0,true,'declaration','String',0],['string','string slice',0,true,'primitive','str',0],['string','owned string',0,false,'library','std::string::String',0]])assert.equal(acceptRustExclusiveSourceStringReceiver(...row),false);
 assert.equal(acceptRustExclusiveSourceStringSuffix('string','string slice',1,true,true,'primitive','str',0),true);assert.equal(acceptRustExclusiveSourceStringSuffix('string','owned string',1,true,true,'library','std::string::String',0),true);
 for(const row of [['string','owned string',0,true,true,'library','std::string::String',0],['string','string slice',2,true,true,'primitive','str',0],['string','string slice',1,true,false,'primitive','str',0],['string','string slice',1,true,true,'declaration','str',0]])assert.equal(acceptRustExclusiveSourceStringSuffix(...row),false);
});
test('Unicode String mutations survive typed Links text/binary and every emitter',()=>{
 const row=originals[5],packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:'1.90.0',rustExclusiveSourceModel:versions.model}),text=packet.network.toLino(),binary=encodeLinkNetworkBinary(packet.network),targets=['JavaScript','Rust','Lean','Rocq'],baseline=Object.fromEntries(targets.map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]));
 for(const graph of [packet.network,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)]){assert.equal(graph.toLino(),text);assert.deepEqual(encodeLinkNetworkBinary(graph),binary);assert.ok(decodeTypedProgramLinks(graph,packet.root).items.length>0);for(const target of targets){const emitted=emitTypedProgramLinks(graph,packet.root,target);assert.deepEqual(emitted,baseline[target]);if(target==='JavaScript')assert.equal(output(emitted.code),row.stdout);}}
});
test('unselected String source mode keeps its original mutation refusal',()=>{assert.throws(()=>parseRust(originals[0].source,{sourceVersion:'1.90.0'}),error=>error.kind==='unsupported');});
