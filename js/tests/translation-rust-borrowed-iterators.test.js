// Authored future assertions. This file has not been run during source drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import * as rules from '../src/translation/rust-borrowed-iterator-rules.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {checkRustBorrowedVectorSourceRegions} from '../src/translation/rust-borrowed-source-regions.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {rustBorrowedIteratorDecisions as decisions,rustBorrowedIteratorPendingOriginals as originals,rustTypeChildrenPendingOriginal as children,rustBorrowedIteratorRefusals as refusals} from './fixtures/rust-borrowed-iterators.mjs';
for(const[index,row]of decisions.entries())test('independent readonly iterator decision '+index,()=>assert.deepEqual(rules[row.operation](...row.arguments),row.expected));
function observe(code){const lines=[];vm.runInNewContext(code,{console:{log:(...values)=>lines.push(values.map(String).join(' '))},process:{argv:[]}},{timeout:1000});return lines.length?lines.join('\n')+'\n':'';}
function selected(row,sourceVersion){return checkProgram(parseRust(row.source,{sourceVersion,sourceSignatureModel:'Rust source function signature 3'}));}
for(const row of [...originals,children])for(const sourceVersion of ['1.90.0','1.99.0'])test('deferred selected readonly original '+row.name+'/'+sourceVersion,()=>{
 assert.equal(row.sourceVersionRange,'>=1.90.0 <1.100.0');const program=selected(row,sourceVersion),packet=encodeTypedProgramLinks(program),targets=['JavaScript','Rust','Lean','Rocq'],expected=new Map(targets.map(target=>[target,emitTypedProgramLinks(packet.network,packet.root,target)]));
 assert.equal(observe(expected.get('JavaScript').code),row.stdout);
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){const reloaded=decodeTypedProgramLinks(network,packet.root);assert.deepEqual(reloaded,program);assert.equal(checkRustBorrowedVectorSourceRegions(reloaded),reloaded);for(const target of targets)assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected.get(target));}
});
for(const row of refusals)for(const sourceVersion of ['1.90.0','1.99.0'])test('deferred readonly refusal '+row.name+'/'+sourceVersion,()=>assert.throws(()=>selected(row,sourceVersion),error=>row.originalCompilerOutcome==='accept'?error.kind==='unsupported':['unsupported','type','syntax'].includes(error.kind)));
test('reloaded projection lambda cannot forge a static reference result',()=>{
 const program=selected(originals[1],'1.90.0'),packet=encodeTypedProgramLinks(program),reloaded=decodeTypedProgramLinks(packet.network,packet.root),pending=[...reloaded.declarations.values()].filter(entry=>entry.k==='fn').map(entry=>entry.body);let body;
 while(pending.length){const node=pending.pop();if(node?.k==='lambda'){body=node.body;break;}for(const[key,value]of Object.entries(node??{}))if(!['type','sourcePosition'].includes(key)){if(Array.isArray(value))pending.push(...value);else if(value&&typeof value==='object')pending.push(value);}}
 assert.equal(body?.sourcePosition?.position.wrappers[0],'reference');body.sourcePosition.position.lifetimes[0]='static';assert.throws(()=>checkRustBorrowedVectorSourceRegions(reloaded),/lifetime|static/);
});
test('the original scalar callback gates still refuse nominal reference domains',async()=>{
 const old=await import('../src/translation/rust-copy-closure-parameter-rules.js');assert.equal(old.acceptRustCopyClosureParameterSource(true,false,1,true,false),false);
});
