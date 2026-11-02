// Deferred controls: actual Rust source -> checked graph -> emitted stdout.
import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {readRustOwnedVectorSourceIteration} from '../src/translation/rust-owned-vector-source-iteration.js';
import {testRustStandardCollectionSourceLoopNode} from '../src/translation/rust-standard-collection-source-loops.js';
import {rustOwnedVectorLoopOriginals as originals,rustOwnedVectorLoopRefusals as refusals,rustOwnedVectorLoopVersions as versions} from './fixtures/rust-owned-vector-source-loops.mjs';
const options=sourceVersion=>({sourceVersion,exclusiveSourceModel:'Rust exclusive source loans 1'});
function nodes(root){const pending=[root],seen=new Set();while(pending.length){const node=pending.pop();if(!node||typeof node!=='object'||seen.has(node))continue;assert.ok(seen.size<1000000);seen.add(node);pending.push(...(node instanceof Map?[...node.values()]:Object.values(node)));}return [...seen];}
function stdout(text){const lines=[];vm.runInNewContext(text,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}},{timeout:10000});return lines.length?lines.join('\n')+'\n':'';}
test('each genuine owned Vec original checks and emits its independent stdout without a collection library',()=>{
 const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===24||major===22&&minor>=15,'uncovered Node '+process.versions.node+'; '+versions.node);
 for(const sourceVersion of ['1.90.0','1.99.0'])for(const row of originals){const surface=parseRust(row.source,options(sourceVersion));assert.ok(!nodes(surface).some(testRustStandardCollectionSourceLoopNode),row.name);const checked=checkProgram(surface);assert.equal(stdout(emitJavaScript(checked).text),row.stdout,row.name+'/'+sourceVersion);}
});
test('every ownership and iterator-publication refusal rejects its retained source input',()=>{
 for(const row of refusals)assert.throws(()=>checkProgram(parseRust(row.source,options('1.90.0'))),error=>['type','unsupported','syntax'].includes(error.kind),row.name);
});
test('private IntoIterator authority cannot be forged from a source origin flag',()=>{
 const node={k:'name',path:['values']},parser={origins:{origin:()=>({sourceOwnedVectorIterator:true,arraySourceModel:null,wrappers:[],type:{kind:'array',element:{kind:'bool'}},sourceOwner:{kind:'library',path:['std','vec','IntoIter'],arguments:[]}})}};
 assert.equal(readRustOwnedVectorSourceIteration(parser,node),null);
});
test('the opt-in source profile and supported language versions remain explicit',()=>{
 assert.throws(()=>parseRust(originals[0].source,{sourceVersion:'1.90.0'}));for(const sourceVersion of ['1.89.0','1.100.0'])assert.throws(()=>parseRust(originals[0].source,options(sourceVersion)));
 assert.equal(originals.length,24);assert.equal(refusals.length,16);for(const row of [...originals,...refusals]){assert.equal(row.sourceVersionRange,versions.rust);assert.match(row.source,/fn main/);}for(const row of originals)assert.equal(typeof row.stdout,'string');
});
