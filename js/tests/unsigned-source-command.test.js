// Deferred command controls. Never import while the full code draft is active.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {runUnsignedSourceCommand} from '../src/unsigned-source-cli.js';
import {runCommandLine} from '../src/cli.js';
import {planUnsignedSourceCommand,acceptUnsignedSourceCommandIdentity,readUnsignedSourceCommandUsage} from '../src/unsigned-source-command-rules.js';
import {UnsignedLinkNetwork,decodeUnsignedLinkNetworkBinary} from '../src/unsigned-link-network.js';
import {createTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {emitUnsignedTypedProgramLinks} from '../src/unsigned-typed-program-links.js';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const data=readProjectData(new URL('../../parity/fixtures/unsigned-source-command.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+data.versions.JavaScript);
for(const row of data.identities)test('exact command identity '+row.value,()=>assert.equal(acceptUnsignedSourceCommandIdentity(row.value),row.expected));
for(const [index,row]of data.commands.entries())test('exact command plan '+index,()=>{
 assert.deepEqual(planUnsignedSourceCommand(row.args),row.expected);
 if(row.expected.length===1){let reads=0;const result=runUnsignedSourceCommand(row.args,{readFile:()=>{reads+=1;throw new Error('must not read');}});assert.equal(reads,0);assert.equal(result.exitCode,row.expected[0]==='help'?0:2);assert.equal(result.stdout,row.expected[0]==='help'?readUnsignedSourceCommandUsage():'');assert.equal(result.stderr,row.expected[0]==='help'?'':'error: invalid exact source arguments\n'+readUnsignedSourceCommandUsage());}
});
for(const row of data.io)test(row.name,()=>{
 let reads=0;const readFile=path=>{reads+=1;assert.equal(path,'input.source');return row.source;},result=runUnsignedSourceCommand(row.args,{readFile});assert.equal(reads,1);assert.equal(result.exitCode,0,result.stderr);assert.equal(result.stderr,'');assert.deepEqual(runCommandLine(['exact-source',...row.args],{readFile}),result);assert.equal(reads,2);
 const plan=planUnsignedSourceCommand(row.args);
 if(row.kind==='emission'){
  const packet=plan[0]==='Rust source owners'?createRustSourceOwnerProgramLinks(row.source,{sourceVersion:plan[5]}):createTypedProgramLinks(row.source,plan[2]);const expected=emitTypedProgramLinks(packet.network,packet.root,row.target);assert.equal(result.stdout,expected.code);
  if(row.expected!==undefined){const name=expected.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target;assert.equal(String(runInNewContext(result.stdout.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000})),row.expected);}
 }else{
  const graph=row.binary?decodeUnsignedLinkNetworkBinary(result.stdout):UnsignedLinkNetwork.fromLino(result.stdout);assert.equal(graph.links()[0].id().toString(),plan[3]);
  if(row.kind==='concrete')assert.equal(graph.reconstructText(),row.source);else{const roots=graph.links().filter(link=>link.metadata().language==='meta-language typed program'&&link.metadata().term==='typed program'&&link.metadata().definition==='1');assert.equal(roots.length,1);const original=plan[0]==='Rust source owners'?createRustSourceOwnerProgramLinks(row.source,{sourceVersion:plan[5]}):createTypedProgramLinks(row.source,plan[2]);for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitUnsignedTypedProgramLinks(graph,roots[0].id(),target),emitTypedProgramLinks(original.network,original.root,target));}
 }
});
test('exact command file errors retain a failing process result',()=>{let reads=0;const result=runUnsignedSourceCommand(['input.source','--language','Rust'],{readFile:()=>{reads+=1;throw new Error('fixture read failure');}});assert.equal(reads,1);assert.deepEqual(result,{exitCode:1,stdout:'',stderr:'error: fixture read failure\n'});});
test('command source definitions keep both codecs and all four emissions',()=>{const packet=createSourceDefinitionProgramLinks('meta-language/source-program/unsigned-source-command-program');for(const graph of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(graph,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));});
