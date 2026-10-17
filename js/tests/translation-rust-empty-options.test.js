import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {rustEmptyOptionFixture} from '../scripts/generate-rust-empty-option-fixtures.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks,encodeTypedProgramLinks,emitTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA} from '../src/typed-program-links.js';
import {parseRust} from '../src/translation/rust.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {LinkNetwork} from '../src/network.js';
import {LinkMetadata} from '../src/primitives.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-empty-options.lino',import.meta.url));
const options={rustEmptyOptionSourceModel:fixture.model};
const scope={javaScriptScopedBindingEmissionModel:'owned JavaScript scoped bindings 1'};
const reload=value=>({network:LinkNetwork.fromLino(value.links),root:value.root});
const field=(network,parent,name)=>network.fieldRelations().find(value=>value.parent.equals(parent)&&value.label===(TYPED_PROGRAM_LINKS_SCHEMA.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase())));
function scalar(network,value){const kind=value===null?'null':typeof value;return network.insertLink([],LinkMetadata.new().withLanguage(TYPED_PROGRAM_LINKS_SCHEMA.language).withNamed(true).withTerm(TYPED_PROGRAM_LINKS_SCHEMA.scalars[kind]).withDefinition(value===null?'':String(value)));}
function replaceField(network,parent,name,child){const old=field(network,parent,name);if(old){const refs=old.link.references();refs[2]=child;old.link.setReferences(refs);}else network.insertFieldRelation(parent,TYPED_PROGRAM_LINKS_SCHEMA.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase()),child);}

test('independent standard owners, explicit primitive type arguments and source sites survive source-free Links reload',()=>{
 assert.deepEqual(rustEmptyOptionFixture(),fixture);
 for(const entry of fixture.cases){
  const packet=reload(entry),program=decodeTypedProgramLinks(packet.network,packet.root),fn=program.declarations.get(entry.functionName);
  assert.deepEqual(fn.sourceEmptyOption,entry.expected,entry.id);
  assert.deepEqual(fn.ret,fixture.type);assert.deepEqual(fn.body.type,fixture.type);assert.equal(fn.body.k,'none');assert.equal(fn.body.absence,'undefined');
  assert.equal(vm.runInNewContext(emitTypedProgramLinks(packet.network,packet.root,'JavaScript').code+'\n'+entry.functionName+'();',{}, {timeout:1000}),undefined);
  const definitions=emitRust(program).definitions.join('\n\n');
  const replay=createTypedProgramLinks(definitions+'\nfn main(){}','Rust',options);
  assert.equal(vm.runInNewContext(emitTypedProgramLinks(replay.network,replay.root,'JavaScript').code+'\n'+entry.emittedFunctionName+'();',{}, {timeout:1000}),undefined);
  assert.ok(!packet.network.toLino().includes(encodeURIComponent(entry.source)),'the archive contains no cached source payload');
 }
});

test('actual JavaScript to emitted Rust definitions to linked Rust to complete canonical JavaScript replays execute',()=>{
 const original='function empty(){return undefined;}';
 assert.equal(vm.runInNewContext(original+'empty();',{}, {timeout:1000}),undefined);
 const packet=createTypedProgramLinks(original,'JavaScript');
 const actual=emitRust(decodeTypedProgramLinks(packet.network,packet.root)).definitions.join('\n\n');
 assert.equal(actual,'pub fn empty() -> Option<f64> {\n    None::<f64>\n}');
 assert.throws(()=>parseRust(actual),error=>error.kind==='unsupported'&&error.reason==='type Option<…>: generic types are outside the portable core'&&assert.deepEqual(error.span,{start:18,end:24})===undefined);
 assert.throws(()=>parseRust(actual,{emptyOptionSourceModel:fixture.model}),error=>error.kind==='syntax'&&error.reason==='a Rust program needs fn main');
 let linked=createTypedProgramLinks(actual+'\nfn main(){}','Rust',{...options,provenance:{caller:'actual-js-empty'}});
 for(let cycle=0;cycle<4;cycle++){
  const network=LinkNetwork.fromLino(linked.network.toLino());
  const code=emitTypedProgramLinks(network,linked.root,'JavaScript',scope).code;
  assert.equal(vm.runInNewContext(code+'\nempty();',{}, {timeout:1000}),undefined);
  // Each import reads the whole actual output, including its main scaffold.
  linked=createTypedProgramLinks(code,'JavaScript');
  assert.deepEqual(decodeTypedProgramLinks(linked.network,linked.root).declarations.get('empty').ret,fixture.type);
 }
});

test('default and preexisting source profiles retain their exact generic-type refusal and selection diagnostics',()=>{
 const source=fixture.cases[0].source;
 for(const selection of [{},{rustSourceSignatureModel:'Rust source function signature 1'},{rustPrimitiveOwnerModel:'standard primitive Rust binary64 owner 1'},{rustArrayResultModel:'immutable fresh scalar vector results 1'},{rustCompletionModel:'closed guarded function returns 1'},{rustScalarLoopModel:'closed declaration scalar loop 1'}])assert.throws(()=>createTypedProgramLinks(source,'Rust',selection),error=>error.kind==='unsupported'&&error.reason==='type Option<…>: generic types are outside the portable core'&&assert.deepEqual(error.span,{start:12,end:18})===undefined);
 assert.throws(()=>createTypedProgramLinks(source,'Rust',{rustEmptyOptionSourceModel:'unknown'}),error=>error.reason==='Rust empty option source model: select the legacy model or closed empty primitive Rust option 1'&&error.span==null);
 assert.throws(()=>createTypedProgramLinks('function empty(){return undefined;}','JavaScript',options),/requires Rust source/u);
 assert.throws(()=>createTypedProgramLinks(source,'Rust',{...options,rustSourceSignatureModel:'Rust source function signature 1'}),/independent source profile/u);
});

test('each complete source refusal retains its recorded original source, diagnostic and span',()=>{
 for(const invalid of fixture.invalid)assert.throws(()=>createTypedProgramLinks(invalid.source,'Rust',options),error=>error.kind===invalid.kind&&error.reason===invalid.reason&&assert.deepEqual(error.span??null,invalid.span)===undefined,invalid.id);
});

test('coherent source-free declaration rename changes the actual executable binding and replay without inventing a singleton value edit',()=>{
 const packet=reload(fixture.cases[0]),program=decodeTypedProgramLinks(packet.network,packet.root),entry=program.declarations.get('empty');
 entry.name='vacant';entry.fullName='vacant';entry.sourceEmptyOption.sourceName='vacant';entry.sourceEmptyOption.namespace.declarationNames=['vacant'];
 program.declarations.delete('empty');program.declarations.set('vacant',entry);
 const encoded=encodeTypedProgramLinks(program),network=LinkNetwork.fromLino(encoded.network.toLino());
 const changed=decodeTypedProgramLinks(network,encoded.root);assert.ok(changed.declarations.has('vacant'));assert.ok(!changed.declarations.has('empty'));assert.deepEqual(changed.declarations.get('vacant').ret,fixture.type);
 const code=emitTypedProgramLinks(network,encoded.root,'JavaScript',scope).code;
 assert.equal(vm.runInNewContext(code+'\nvacant();',{}, {timeout:1000}),undefined);assert.throws(()=>vm.runInNewContext(code+'\nempty();',{}, {timeout:1000}),/empty is not defined/u);
 const actual=emitRust(changed).definitions.join('\n\n');assert.ok(actual.includes('pub fn vacant()'));
 const reverse=createTypedProgramLinks(actual+'\nfn main(){}','Rust',options);
 assert.equal(vm.runInNewContext(emitTypedProgramLinks(reverse.network,reverse.root,'JavaScript').code+'\nvacant();',{}, {timeout:1000}),undefined);
 const changedSource=createTypedProgramLinks(code,'JavaScript');assert.ok(decodeTypedProgramLinks(changedSource.network,changedSource.root).declarations.has('vacant'));
});

test('malformed owned source facts, tags, body sites, namespaces and context cannot authorize emission',()=>{
 const changes=[
  p=>{p.declarations.get('empty').sourceEmptyOption=null;},p=>{p.declarations.get('empty').sourceEmptyOption.model='unknown';},
  p=>{p.declarations.get('empty').sourceEmptyOption.language='JavaScript';},p=>{p.declarations.get('empty').sourceEmptyOption.sourceName='other';},
  p=>{p.declarations.get('empty').sourceEmptyOption.unknown=false;},p=>{p.declarations.get('empty').sourceEmptyOption.parameters=[{type:{kind:'float'}}];},
  p=>{p.declarations.get('empty').sourceEmptyOption.result.owner.sourcePath=['custom','Option'];},p=>{p.declarations.get('empty').sourceEmptyOption.result.owner.resolvedPath=['std','option','Option'];},
  p=>{p.declarations.get('empty').sourceEmptyOption.result.owner.canonicalPath=['foreign','Option'];},p=>{p.declarations.get('empty').sourceEmptyOption.result.owner.arguments[0].sourcePath=['f32'];},
  p=>{p.declarations.get('empty').sourceEmptyOption.result.owner.arguments=[];},p=>{p.declarations.get('empty').sourceEmptyOption.result.wrappers=['reference'];},
  p=>{p.declarations.get('empty').sourceEmptyOption.construction.owner.arguments[0].type={kind:'bool'};},p=>{p.declarations.get('empty').sourceEmptyOption.construction.owner.sourcePath=['crate','None'];},
  p=>{p.declarations.get('empty').sourceEmptyOption.construction.path=['value'];},p=>{p.declarations.get('empty').sourceEmptyOption.construction.span.end--;},
  p=>{p.declarations.get('empty').sourceEmptyOption.namespace.imports=[{name:'None',target:['other']}];},p=>{p.declarations.get('empty').sourceEmptyOption.namespace.declarationNames=['counterfeit'];},
  p=>{p.declarations.get('empty').sourceEmptyOption.namespace.declaredTypeOwners=['Option'];},p=>{p.declarations.get('empty').sourceEmptyOption.namespace.externalOptionRoots=['std'];},
  p=>{p.declarations.get('empty').ret.element={kind:'bool'};},p=>{p.declarations.get('empty').body.absence='null';},
  p=>{p.declarations.get('empty').body.extra=false;},p=>{p.declarations.get('empty').generated=true;},p=>{p.main=null;},p=>{p.main.borrowEscape=true;},
  p=>{p.borrowEscape=false;},p=>{p.referenceSemantics=true;},p=>{p.declarations.set('foreign',structuredClone(p.declarations.get('empty')));},
 ];
 for(const [index,change] of changes.entries()){
  const packet=reload(fixture.cases[0]),program=decodeTypedProgramLinks(packet.network,packet.root);change(program);assert.throws(()=>encodeTypedProgramLinks(program),undefined,'malformed edit '+index);
 }
});

test('present raw null and unknown fields remain visible through the actual linked decoder and are refused',()=>{
 const edits=[['sourceEmptyOption',null,'fn'],['unknown',false,'proof'],['construction',null,'proof'],['parameters',null,'proof'],['arguments',null,'owner'],['sourcePath',null,'owner'],['unknown',false,'main'],['main',null,'program']];
 for(const [name,value,kind] of edits){
  const packet=reload(fixture.cases[0]),network=packet.network;
  const proof=network.fieldRelations().find(row=>row.label===TYPED_PROGRAM_LINKS_SCHEMA.fields.sourceEmptyOption);
  const programId=network.link(packet.root).references()[0];
  const targets={fn:proof.parent,proof:proof.child,owner:field(network,field(network,proof.child,'result').child,'owner').child,main:field(network,programId,'main').child,program:programId};
  replaceField(network,targets[kind],name,scalar(network,value));
  const raw=LinkNetwork.fromLino(network.toLino());assert.throws(()=>decodeTypedProgramLinks(raw,packet.root),undefined,name+' raw '+kind);
  for(const target of ['JavaScript','Rust'])assert.throws(()=>emitTypedProgramLinks(raw,packet.root,target));
 }
});

test('actual selected profile and independent caller provenance survive source-free archive reload',()=>{
 const packet=createTypedProgramLinks(fixture.cases[0].source,'Rust',{...options,provenance:{request:'owned-test'}}),network=LinkNetwork.fromLino(packet.network.toLino());
 const provenance=field(network,packet.root,'provenance').child;
 const sourceProfile=field(network,provenance,'sourceProfile').child,user=field(network,provenance,'userProvenance').child;
 assert.equal(network.link(field(network,sourceProfile,'rustEmptyOptionSourceModel').child).metadata().definition,fixture.model);
 assert.equal(network.link(field(network,user,'request').child).metadata().definition,'owned-test');
 assert.deepEqual(decodeTypedProgramLinks(network,packet.root).declarations.get('empty').sourceEmptyOption,fixture.cases[0].expected);
});
