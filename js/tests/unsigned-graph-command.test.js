// Deferred saved graph, exact root, metadata edit and process controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import * as rules from '../src/unsigned-graph-command-rules.js';
import {runUnsignedGraphCommand,readUnsignedGraphSnapshot,selectUnsignedTypedProgramRoot} from '../src/unsigned-graph-cli.js';
import {runCommandLine} from '../src/cli.js';
import {UnsignedLinkNetwork,UnsignedLinkId,encodeUnsignedLinkNetworkBinary,rebaseUnsignedLinkNetwork} from '../src/unsigned-link-network.js';
import {emitUnsignedTypedProgramLinks} from '../src/unsigned-typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {LinkMetadata} from '../src/primitives.js';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {emitTypedProgramLinks} from '../src/typed-program-links.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {createUnsignedGraphFixtureInput} from '../../experiments/unsigned-graph/fixture-input.mjs';
const fixture=readProjectData(new URL('../../parity/fixtures/unsigned-graph-command.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.roots)test(row.name,()=>assert.deepEqual(rules.planUnsignedTypedGraphRoot(row.ids,row.types,row.languages,row.terms,row.definitions,row.requested,row.limit),row.expected));
for(const [index,row]of fixture.bounds.entries())test('saved graph bounds '+index,()=>assert.equal(rules.readUnsignedGraphReadBoundsError(...row.args),row.expected));
for(const [index,row]of fixture.surface.entries())test('identified graph surface '+index,()=>{assert.equal(rules.readUnsignedGraphTextSurfaceError(row.value),row.expected);if(row.expected!=='')assert.throws(()=>readUnsignedGraphSnapshot(row.value,'text'));});
for(const [index,row]of fixture.commands.entries())test('graph command plan '+index,()=>{
 assert.deepEqual(rules.planUnsignedGraphCommand(row.args),row.expected);
 if(row.expected.length===1){let reads=0;const read=()=>{reads+=1;throw new Error('must not read');},result=runUnsignedGraphCommand(row.args,{readFile:read,readBytes:read});assert.equal(reads,0);assert.equal(result.exitCode,row.expected[0]==='help'?0:2);assert.equal(result.stdout,row.expected[0]==='help'?rules.readUnsignedGraphCommandUsage():'');assert.equal(result.stderr,row.expected[0]==='help'?'':'error: invalid exact graph arguments\n'+rules.readUnsignedGraphCommandUsage());}
});
for(const row of fixture.io)test(row.name,()=>{
 const input=createUnsignedGraphFixtureInput(row,fixture);let reads=0,byteReads=0;const io={readFile:path=>{reads+=1;assert.equal(path,'saved.graph');return input.text;},readBytes:path=>{byteReads+=1;assert.equal(path,'saved.graph');return input.bytes;}};
 const before=encodeUnsignedLinkNetworkBinary(input.network),result=runUnsignedGraphCommand(input.args,io),plan=rules.planUnsignedGraphCommand(input.args);
 assert.equal(reads,plan[1]==='text'?1:0);assert.equal(byteReads,plan[1]==='binary'?1:0);assert.equal(result.exitCode,row.exitCode,result.stderr);assert.deepEqual(runCommandLine(['exact-graph',...input.args],io),result);assert.deepEqual(encodeUnsignedLinkNetworkBinary(input.network),before);
 if(row.exitCode!==0){assert.equal(result.stdout,'');assert.equal(result.stderr,'error: '+row.error+'\n');return;}
 assert.equal(result.stderr,'');
 if(row.kind==='emission'){
  const root=row.multiple?input.second:input.root;assert.equal(result.stdout,emitUnsignedTypedProgramLinks(input.network,root,row.target).code);
  if(row.expected!==null){const emission=emitUnsignedTypedProgramLinks(input.network,root,'JavaScript'),name=emission.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target;assert.equal(String(runInNewContext(result.stdout.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000})),row.expected);}
 }else{
  const graph=readUnsignedGraphSnapshot(result.stdout,plan[2]);const expected=plan[5]===''?input.network:rebaseUnsignedLinkNetwork(input.network,plan[5]).network;assert.equal(graph.toLino(),expected.toLino());if(plan[2]==='binary')assert.deepEqual(encodeUnsignedLinkNetworkBinary(graph),encodeUnsignedLinkNetworkBinary(expected));
 }
});
test('saved graph emission uses metadata edits without the ordinary source-document parser',()=>{
 const row=fixture.io.find(row=>row.edit==='9'&&!row.args.includes('--input-binary')),input=createUnsignedGraphFixtureInput(row,fixture),original=LinkNetwork.parse;
 LinkNetwork.parse=()=>{throw new Error('ordinary source-document parser must not run');};
 try{const result=runUnsignedGraphCommand(input.args,{readFile:()=>input.text});assert.equal(result.exitCode,0,result.stderr);const graph=readUnsignedGraphSnapshot(input.text),root=selectUnsignedTypedProgramRoot(graph);const emitted=emitUnsignedTypedProgramLinks(graph,root,'JavaScript'),name=emitted.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target;assert.equal(String(runInNewContext(result.stdout.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000})),'9');}finally{LinkNetwork.parse=original;}
});
test('metadata edits preserve exact coordinates, explicit registrations, caches and absent identities',()=>{
 for(const graph of [new LinkNetwork(),new UnsignedLinkNetwork()]){
  const id=graph.insertPoint('meaning'),beforeId=String(id),references=graph.link(id).references().map(String),before=graph.toLino();assert.equal(graph.setLinkMetadata(999,LinkMetadata.new().withTerm('unused')),false);assert.equal(graph.toLino(),before);
  assert.equal(graph.setLinkMetadata(id,graph.link(id).metadata().clone().withDefinition('new meaning')),true);assert.equal(String(id),beforeId);assert.deepEqual(graph.link(id).references().map(String),references);assert.equal(String(graph.findTerm('meaning')),beforeId);assert.equal(graph.link(id).metadata().definition,'new meaning');
 }
});
test('selected actual roots and snapshot bounds reject counterfeit or absent entries',()=>{
 const row=fixture.io.find(row=>row.multiple),input=createUnsignedGraphFixtureInput(row,fixture);assert.throws(()=>selectUnsignedTypedProgramRoot(input.network));for(const root of [0,'0','',Number.MAX_SAFE_INTEGER+1])assert.throws(()=>selectUnsignedTypedProgramRoot(input.network,root),error=>error.message==='a typed graph root needs an exact positive unsigned identity');assert.equal(selectUnsignedTypedProgramRoot(input.network,input.second).toString(),String(input.second));assert.throws(()=>selectUnsignedTypedProgramRoot(input.network,UnsignedLinkId.from('18446744073709551615')));assert.throws(()=>readUnsignedGraphSnapshot(input.text,'text',{recordLimit:1}));assert.throws(()=>readUnsignedGraphSnapshot(input.text,'text',{byteLimit:33}));assert.throws(()=>readUnsignedGraphSnapshot(input.bytes,'binary',{byteLimit:33}));
});
test('saved graph definition retains both codecs and four target emissions',()=>{const packet=createSourceDefinitionProgramLinks('meta-language/source-program/unsigned-graph-command-program');for(const graph of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(graph,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));});
