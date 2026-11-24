// Deferred transport and actual frontend controls. Never run during drafting.
import test from 'node:test';
import assert from 'node:assert/strict';
import {RustSourceOrigins} from '../src/translation/rust-source-origins.js';
import {qualifyRustStandardSumSourceOrigin} from '../src/translation/rust-standard-sum-source.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {rustSourceOwnerQualificationOriginals as originals,rustSourceOwnerQualificationRefusals as refusals} from './fixtures/rust-source-owner-qualification.mjs';

test('TYPE canonicalization preserves actual runtime facts and the outer Box/Vec structure',()=>{
 const table=new RustSourceOrigins();
 for(const fullName of ['Later','domain.Later'])table.registerCatalogEntry(fullName,{k:'data',fullName});
 table.nominalCopyOwners.add('domain.Later');
 const declaration={kind:'declaration',path:['Later'],arguments:[]};
 const input={type:{kind:'array',element:{kind:'named',path:['Later']}},path:['domain'],wrappers:['box'],sourceOwner:{kind:'library',path:['std','boxed','Box'],arguments:[{kind:'library',path:['std','vec','Vec'],arguments:[declaration]}]},proven:false,copyEvidence:false,diverging:false,uncertain:true,fields:null,sourceBindingIdentity:'existing source binding',sourceValueBorrowBindings:['existing source loan']};
 const saved=structuredClone(input),result=qualifyRustStandardSumSourceOrigin(table,input);
 assert.deepEqual(input,saved);assert.ok(result);
 assert.deepEqual(result.type.element.path,['crate','domain','Later']);
 assert.deepEqual(result.sourceOwner.arguments[0].arguments[0].path,['domain','Later']);
 assert.deepEqual(result.sourceOwner.path,['std','boxed','Box']);assert.deepEqual(result.sourceOwner.arguments[0].path,['std','vec','Vec']);
 for(const key of ['path','wrappers','proven','copyEvidence','diverging','uncertain','sourceBindingIdentity','sourceValueBorrowBindings'])assert.deepEqual(result[key],input[key]);
});
test('tuple and function children use their actual retained declaration contexts',()=>{
 const table=new RustSourceOrigins();for(const fullName of ['left.Item','right.Item'])table.registerCatalogEntry(fullName,{k:'data',fullName});
 const child=path=>({type:{kind:'named',path:['Item']},path:[path],wrappers:[],proven:false,copyEvidence:false,fields:null,sourceOwner:{kind:'declaration',path:['Item'],arguments:[]}});
 const left=child('left'),right=child('right'),input={type:{kind:'function',params:[left.type],ret:right.type},path:[],wrappers:[],proven:false,copyEvidence:false,callableParameters:[left],callableResult:right,sourceOwner:{kind:'function',path:[],arguments:[left.sourceOwner,right.sourceOwner]}};
 const result=qualifyRustStandardSumSourceOrigin(table,input);assert.ok(result);
 assert.deepEqual(result.type.params[0].path,['crate','left','Item']);assert.deepEqual(result.type.ret.path,['crate','right','Item']);
 assert.deepEqual(result.sourceOwner.arguments.map(owner=>owner.path),[['left','Item'],['right','Item']]);
});
for(const row of originals)for(const sourceVersion of ['1.90.0','1.99.0'])test(row.name+'/'+sourceVersion,()=>{
 const checked=checkProgram(parseRust(row.source,{sourceVersion})),packet=createTypedProgramLinks(row.source,'Rust',{rustSourceVersion:sourceVersion});
 const text=packet.network.toLino(),binary=encodeLinkNetworkBinary(packet.network);
 for(const graph of [packet.network,LinkNetwork.fromLino(text),decodeLinkNetworkBinary(binary)]){
  assert.equal(graph.toLino(),text);assert.deepEqual(decodeTypedProgramLinks(graph,packet.root).items,checked.items);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(graph,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
});
for(const row of refusals)for(const sourceVersion of ['1.90.0','1.99.0'])test(row.name+'/'+sourceVersion,()=>assert.throws(()=>checkProgram(parseRust(row.source,{sourceVersion})),error=>{assert.ok(['syntax','type','unsupported'].includes(error.kind));return true;}));
