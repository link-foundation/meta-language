import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {rustPublicPureUnitCallFixture} from '../scripts/generate-rust-public-pure-unit-call-fixtures.mjs';
import {applyPureUnitContainerMutation,rustPureUnitCallFixture} from '../scripts/generate-rust-pure-unit-call-fixtures.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks,encodeTypedProgramLinks,emitTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {parseRust} from '../src/translation/rust.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {acceptRustPureUnitVisibility} from '../src/translation/rust-pure-unit-call-rules.js';
import {LinkNetwork} from '../src/network.js';
import {LinkMetadata} from '../src/primitives.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-public-pure-unit-calls.lino',import.meta.url));
const privateFixture=readProjectData(new URL('../../parity/fixtures/rust-pure-unit-calls.lino',import.meta.url));
const options={rustPureUnitCallModel:fixture.model};
const reload=entry=>({network:LinkNetwork.fromLino(entry.links),root:entry.root});
const label=name=>TYPED_PROGRAM_LINKS_SCHEMA.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
const field=(network,parent,name)=>network.fieldRelations().find(row=>row.parent.equals(parent)&&row.label===label(name));
function scalar(network,value){return network.insertLink([],LinkMetadata.new().withLanguage(TYPED_PROGRAM_LINKS_SCHEMA.language).withNamed(true).withTerm(TYPED_PROGRAM_LINKS_SCHEMA.scalars[value===null?'null':typeof value]).withDefinition(value===null?'':String(value)));}
function replaceField(network,parent,name,child){const relation=field(network,parent,name);if(relation){const refs=relation.link.references();refs[2]=child;relation.link.setReferences(refs);}else network.insertFieldRelation(parent,label(name),child);}
function execute(code){const output=[],context=vm.createContext({console:{log:(...args)=>output.push(args.join(' '))}});vm.runInContext(code,context,{timeout:1000});return {output:output.join('\n'),context};}
function observeCalls(packet){const program=decodeTypedProgramLinks(packet.network,packet.root),target=execute(emitTypedProgramLinks(packet.network,packet.root,'JavaScript').code),calls=[];for(const name of program.declarations.keys()){assert.equal(typeof target.context[name],'function');target.context[name]=()=>{calls.push(name);return null;};}vm.runInContext('main();',target.context,{timeout:1000});return calls;}

test('actual public keyword and complete owned Unit headers survive source-free reload while original call identities and order execute',()=>{
 assert.deepEqual(rustPublicPureUnitCallFixture(),fixture);assert.equal(fixture.cases.length,9);
 for(const entry of fixture.cases){
  const packet=reload(entry),program=decodeTypedProgramLinks(packet.network,packet.root);
  assert.deepEqual(program.sourcePureUnitCalls,{model:fixture.model,language:'Rust'});assert.deepEqual([...program.declarations.keys()],entry.names);assert.deepEqual(program.main.effects.map(effect=>effect.value.fn),entry.calls);
  for(const name of entry.names){const fn=program.declarations.get(name),proof=fn.sourcePureUnit;assert.deepEqual(fn.params,[]);assert.deepEqual(fn.ret,{kind:'unit'});assert.deepEqual(fn.body,{k:'unit',type:{kind:'unit'}});assert.equal(proof.visibility.form,'public');assert.equal(proof.visibility.span.start,fn.span.start);assert.equal(entry.source.slice(proof.visibility.span.start,proof.visibility.span.end),'pub');assert.equal(proof.visibility.span.end-proof.visibility.span.start,3);assert.equal(proof.headerSpan.start,fn.span.start);assert.equal(proof.bodySpan.end,fn.span.end);}
  assert.equal(execute(emitTypedProgramLinks(packet.network,packet.root,'JavaScript').code).output,entry.output);assert.deepEqual(observeCalls(packet),entry.calls);
  recheckLinkedTypedProgram(program,{wholeProgram:true});assert.ok(!packet.network.toLino().includes(encodeURIComponent(entry.source)),'no saved source envelope');
 }
});

test('five complete original JavaScript programs replay the actual full emitted definition set with canonical empty main',()=>{
 assert.equal(fixture.generated.length,5);
 for(const entry of fixture.generated){
  assert.equal(execute(entry.original).output,entry.output,entry.id+' complete original JavaScript execution');
  const original=createTypedProgramLinks(entry.original,'JavaScript'),emitted=emitRust(decodeTypedProgramLinks(original.network,original.root));
  assert.equal(emitted.definitions.join('\n\n'),entry.definitions);assert.equal(emitted.text,entry.fullEmittedRust);assert.equal(entry.source,entry.definitions+'\nfn main(){}');
  let packet=reload(entry);
  for(let cycle=0;cycle<3;cycle++){
   const program=decodeTypedProgramLinks(packet.network,packet.root);
   assert.deepEqual([...program.declarations.keys()],entry.names);assert.equal(program.main.effects.length,0);
   assert.equal(execute(emitTypedProgramLinks(packet.network,packet.root,'JavaScript').code).output,entry.output,entry.id+' cycle '+cycle);
   // Reimport the actual next complete definition set, never restored input.
   const definitions=emitRust(program).definitions.join('\n\n');
   packet=createTypedProgramLinks(definitions+'\nfn main(){}','Rust',options);packet={network:LinkNetwork.fromLino(packet.network.toLino()),root:packet.root};
  }
  assert.throws(()=>parseRust(entry.fullEmittedRust),/function without a result: ml_main/u);
  assert.throws(()=>parseRust(entry.fullEmittedRust,{pureUnitCallModel:fixture.model}),/attributes require a separate compilation context proof/u);
  const wrapper=entry.fullEmittedRust.slice(entry.fullEmittedRust.lastIndexOf('fn main()'));
  assert.throws(()=>parseRust(wrapper),/std::env::args: the standard library is outside the portable core/u);
  assert.throws(()=>parseRust(wrapper,{pureUnitCallModel:fixture.model}));
 }
});

test('public metadata is not inferred for private sources and all original private fixtures and refusals remain exact',()=>{
 assert.deepEqual(rustPureUnitCallFixture(),privateFixture);assert.equal(privateFixture.cases.length,12);assert.equal(privateFixture.invalid.length,38);
 for(const invalid of privateFixture.invalid)assert.throws(()=>createTypedProgramLinks(invalid.source,'Rust',{rustPureUnitCallModel:privateFixture.model}),error=>error.kind===invalid.kind&&error.reason===invalid.reason&&assert.deepEqual(error.span??null,invalid.span)===undefined,invalid.id);
 for(const entry of privateFixture.cases){const packet=reload(entry),program=decodeTypedProgramLinks(packet.network,packet.root);for(const fn of program.declarations.values())assert.equal(Object.hasOwn(fn.sourcePureUnit,'visibility'),false);}
 assert.throws(()=>createTypedProgramLinks(fixture.cases[0].source,'Rust',{rustPureUnitCallModel:privateFixture.model}),/only flat private pure Unit functions/u);
 assert.throws(()=>createTypedProgramLinks(privateFixture.cases[0].source,'Rust',options),/each helper requires its actual bare pub keyword/u);
 assert.throws(()=>createTypedProgramLinks(fixture.cases[0].source,'Rust',{...options,rustSourceSignatureModel:'Rust source function signature 1'}),/independent source profile/u);
 assert.throws(()=>createTypedProgramLinks(fixture.generated[0].original,'JavaScript',options),/requires Rust source/u);
});

test('22 complete public source refusals and the retained void-annotation input gap keep their actual diagnostics',()=>{
 assert.equal(fixture.invalid.length,22);for(const invalid of fixture.invalid)assert.throws(()=>createTypedProgramLinks(invalid.source,'Rust',options),error=>error.kind===invalid.kind&&error.reason===invalid.reason&&assert.deepEqual(error.span??null,invalid.span)===undefined,invalid.id);
 assert.equal(fixture.javaScriptInvalid.length,1);for(const invalid of fixture.javaScriptInvalid)assert.throws(()=>createTypedProgramLinks(invalid.source,'JavaScript'),error=>error.kind===invalid.kind&&error.reason===invalid.reason&&assert.deepEqual(error.span??null,invalid.span)===undefined,invalid.id);
 for(const model of [privateFixture.model,fixture.model,'','unknown'])for(const form of ['private','public','crate','unknown'])assert.equal(acceptRustPureUnitVisibility(model,form),model===privateFixture.model&&form==='private'||model===fixture.model&&form==='public');
});

test('source-free public declaration rename and normal reference retargeting change the executable public binding and real calls',()=>{
 const entry=fixture.cases.find(entry=>entry.id==='public-ordered-repeated'),packet=reload(entry),program=decodeTypedProgramLinks(packet.network,packet.root),fn=program.declarations.get('alpha');
 fn.name='renamed';fn.fullName='renamed';program.declarations.delete('alpha');program.declarations.set('renamed',fn);for(const effect of program.main.effects)if(effect.value.fn==='alpha')effect.value.fn='renamed';
 const encoded=encodeTypedProgramLinks(program),raw={network:LinkNetwork.fromLino(encoded.network.toLino()),root:encoded.root};
 assert.deepEqual(observeCalls(raw),['renamed','beta','renamed']);const result=execute(emitTypedProgramLinks(raw.network,raw.root,'JavaScript').code);assert.equal(typeof result.context.renamed,'function');assert.equal(typeof result.context.alpha,'undefined');
 const actual=emitRust(decodeTypedProgramLinks(raw.network,raw.root)).definitions.join('\n\n');assert.match(actual,/pub fn renamed\(\) -> \(\)/u);
 const reverse=createTypedProgramLinks(actual+'\nfn main(){}','Rust',options);assert.ok(decodeTypedProgramLinks(reverse.network,reverse.root).declarations.has('renamed'));
 const referenceRows=raw.network.fieldRelations().filter(row=>row.label==='declaration reference'),refs=referenceRows[0].link.references();refs[2]=referenceRows[1].child;referenceRows[0].link.setReferences(refs);
 const changed={network:LinkNetwork.fromLino(raw.network.toLino()),root:raw.root};assert.deepEqual(observeCalls(changed),['beta','beta','renamed']);
});

test('malformed actual visibility facts and raw null or unknown linked fields cannot authorize either target',()=>{
 const changes=[
  p=>{delete p.declarations.get('empty').sourcePureUnit.visibility;},p=>{p.declarations.get('empty').sourcePureUnit.visibility=null;},p=>{p.declarations.get('empty').sourcePureUnit.visibility.form='private';},p=>{p.declarations.get('empty').sourcePureUnit.visibility.form='crate';},
  p=>{p.declarations.get('empty').sourcePureUnit.visibility.span=null;},p=>{p.declarations.get('empty').sourcePureUnit.visibility.span.end++;},p=>{p.declarations.get('empty').sourcePureUnit.visibility.span.start++;},p=>{p.declarations.get('empty').sourcePureUnit.visibility.unsafe=false;},
  p=>{p.declarations.get('empty').sourcePureUnit.model=privateFixture.model;},p=>{p.sourcePureUnitCalls.model=privateFixture.model;},p=>{p.declarations.get('empty').params=[{name:'value',type:{kind:'unit'}}];},p=>{p.declarations.get('empty').borrowEscape=false;},
 ];
 for(const [index,change] of changes.entries()){const packet=reload(fixture.cases[1]),program=decodeTypedProgramLinks(packet.network,packet.root);change(program);assert.throws(()=>encodeTypedProgramLinks(program),undefined,'malformed visibility edit '+index);}
 for(const [role,name,value] of [['proof','visibility',null],['visibility','form',null],['visibility','span',null],['visibility','unsafe',false],['span','end',2],['span','start',1]]){
  const packet=reload(fixture.cases[1]),network=packet.network,proof=network.fieldRelations().find(row=>row.label===label('sourcePureUnit')).child,visibility=field(network,proof,'visibility').child,span=field(network,visibility,'span').child;
  replaceField(network,{proof,visibility,span}[role],name,scalar(network,value));const raw=LinkNetwork.fromLino(network.toLino());assert.throws(()=>decodeTypedProgramLinks(raw,packet.root),undefined,role+' '+name);for(const target of ['Rust','JavaScript'])assert.throws(()=>emitTypedProgramLinks(raw,packet.root,target));
 }
});

test('all 52 own-container mutations remain refused under the separate public model after source-free reload',()=>{
 for(const mutation of privateFixture.containerMutations){
  const packet=createTypedProgramLinks('pub fn empty(){}fn main(){empty();}','Rust',options),program=decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()),packet.root);applyPureUnitContainerMutation(program,mutation);
  assert.throws(()=>encodeTypedProgramLinks(program),error=>error.kind===mutation.kind&&error.reason===mutation.reason,mutation.id);
 }
 for(const [role,name] of [['program','main'],['program','declarations'],['main','effects']]){
  const packet=reload(fixture.cases[1]),program=decodeTypedProgramLinks(packet.network,packet.root),owner=role==='program'?program:program.main,value=owner[name];let invoked=0;
  Object.defineProperty(owner,name,{get(){invoked++;return value;},enumerable:true});assert.throws(()=>encodeTypedProgramLinks(program),/unrepresented own fields/u);assert.equal(invoked,0,role+' '+name);
 }
});
