// Deferred complete multi-file execution, source islands, receipts and codecs.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../src/project-data-files.js';
import {composeRustSourceProject,parseRustSourceProject,createRustSourceProjectProgramLinks,readRustSourceProjectConcreteGraphs} from '../src/rust-source-project.js';
import {emitTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA} from '../src/typed-program-links.js';
import {runRustSourceProjectCommand} from '../src/rust-source-project-cli.js';
import {structuredDataToLino} from '../src/structured-data-links.js';
import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';
import {LinkNetwork} from '../src/network.js';
import {LinkType} from '../src/primitives.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-source-project.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
for(const row of fixture.cases)test('complete original Rust source project: '+row.name,()=>{
 const composition=composeRustSourceProject(row.inputs,row.entry,row.options),packet=createRustSourceProjectProgramLinks(row.inputs,row.entry,row.options),emitted=emitTypedProgramLinks(packet.network,packet.root,'JavaScript'),lines=[];
 let code=emitted.code;if(row.options.library){const name=emitted.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target;code+='\nconsole.log('+name+'());';assert.equal(parseRustSourceProject(row.inputs,row.entry,row.options).entryNormalization,'empty frontend entry for a source library');}
 runInNewContext(code,{console:{log:value=>lines.push(String(value))}},{timeout:1000});assert.deepEqual(lines,[row.value]);
 const sourceBytes=Buffer.from(composition.source);let position=0;
 for(const mapping of composition.mappings){assert.equal(mapping.outputStart,position);position=mapping.outputEnd;if(mapping.kind==='source'){const source=Buffer.from(row.inputs.find(file=>file.path===mapping.path).source);assert.deepEqual(sourceBytes.subarray(mapping.outputStart,mapping.outputEnd),source.subarray(mapping.start,mapping.end));}}
 assert.equal(position,sourceBytes.length);
 for(const mapping of composition.mappings.filter(row=>row.kind==='interpreted attribute')){
  assert.equal(mapping.outputStart,mapping.outputEnd);
  const original=row.inputs.find(file=>file.path===mapping.path).source;
  const text=Buffer.from(original).subarray(mapping.start,mapping.end).toString('utf8');
  assert.ok(text.startsWith('#[')&&text.endsWith(']'));assert.match(text,/path\s*=/u);
  assert.equal(original.slice(mapping.startUnit,mapping.endUnit),text);
  assert.ok(composition.modules.some(module=>module.module===mapping.module));
 }
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  const originals=readRustSourceProjectConcreteGraphs(network,packet.root);assert.deepEqual(originals.map(({path,source})=>({path,source})),[...row.inputs].sort((a,b)=>a.path<b.path?-1:a.path>b.path?1:0));
  for(const original of originals){assert.ok(original.network.verifyFullMatch().isClean());assert.equal(original.network.reconstructText(),original.source);}
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
 // Corrupting actual concrete input cannot be hidden by a saved source string.
 const changed=LinkNetwork.fromLino(packet.network.toLino()),token=changed.links().find(link=>link.metadata().linkType===LinkType.SourceToken&&link.metadata().term);
 assert.ok(token);token.metadata().term+='x';assert.throws(()=>readRustSourceProjectConcreteGraphs(changed,packet.root));
});
for(const row of fixture.refusals)test('explicit source project refusal: '+row.name,()=>assert.throws(()=>composeRustSourceProject(row.inputs,row.entry,row.options),error=>error.message.includes(row.message)));
for(const row of fixture.planning)test('actual source module planning: '+row.name,()=>{const value=composeRustSourceProject(row.inputs,row.entry);assert.equal(value.modules.length,row.modules);assert.equal(value.source,row.inputs[0].source);});
test('complete module composition and abrupt result definitions preserve all target codecs',()=>{
 for(const name of ['rust-project-source-program','abrupt-result-program']){const packet=createSourceDefinitionProgramLinks('meta-language/source-program/'+name);for(const target of ['JavaScript','Rust','Lean','Rocq']){const expected=emitTypedProgramLinks(packet.network,packet.root,target);for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);}}
});
test('installed source-project command retains default CST graphs and optional source or binary views',()=>{
 const row=fixture.cases[0],input=structuredDataToLino({inputs:row.inputs,options:row.options}),io={readFile:path=>{assert.equal(path,'inventory.lino');return input;}};
 for(const flags of [[],['--binary']]){const output=runRustSourceProjectCommand(['inventory.lino','--entry',row.entry,...flags],io);assert.equal(output.exitCode,0,output.stderr);const network=flags.length?decodeLinkNetworkBinary(output.stdout):LinkNetwork.fromLino(output.stdout),root=network.links().find(link=>link.metadata().term===TYPED_PROGRAM_LINKS_SCHEMA.root).id();assert.equal(readRustSourceProjectConcreteGraphs(network,root).length,row.inputs.length);}
 for(const target of ['JavaScript','Rust','Lean','Rocq']){const output=runRustSourceProjectCommand(['inventory.lino','--entry',row.entry,'--to',target],io);assert.equal(output.exitCode,0,output.stderr);assert.equal(typeof output.stdout,'string');assert.ok(output.stdout.length>0);}
 for(const args of [['--entry'],['inventory.lino'],['inventory.lino','--entry','src/main.rs','--entry','src/main.rs'],['inventory.lino','--entry','src/main.rs','--binary','--to','Rust']])assert.equal(runRustSourceProjectCommand(args,io).exitCode,2);
 const help=runRustSourceProjectCommand(['help'],{readFile:()=>assert.fail('help cannot read a source')});assert.equal(help.exitCode,0);assert.match(help.stdout,/Default output retains/);
});
