import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {rustPureUnitCallFixture,applyPureUnitContainerMutation} from '../scripts/generate-rust-pure-unit-call-fixtures.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks,encodeTypedProgramLinks,emitTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {LinkNetwork} from '../src/network.js';
import {LinkMetadata} from '../src/primitives.js';
import {acceptRustPureUnitCallModel,acceptRustPureUnitNameCharacter,acceptRustPureUnitHeaderForm,acceptRustPureUnitBodyForm,acceptRustPureUnitPathForm,acceptRustPureUnitContainer} from '../src/translation/rust-pure-unit-call-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-pure-unit-calls.lino',import.meta.url));
const options={rustPureUnitCallModel:fixture.model};
const reload=entry=>({network:LinkNetwork.fromLino(entry.links),root:entry.root});
const label=name=>TYPED_PROGRAM_LINKS_SCHEMA.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
const field=(network,parent,name)=>network.fieldRelations().find(row=>row.parent.equals(parent)&&row.label===label(name));
function scalar(network,value){return network.insertLink([],LinkMetadata.new().withLanguage(TYPED_PROGRAM_LINKS_SCHEMA.language).withNamed(true).withTerm(TYPED_PROGRAM_LINKS_SCHEMA.scalars[value===null?'null':typeof value]).withDefinition(value===null?'':String(value)));}
function replaceField(network,parent,name,child){const relation=field(network,parent,name);if(relation){const refs=relation.link.references();refs[2]=child;relation.link.setReferences(refs);}else network.insertFieldRelation(parent,label(name),child);}
function observe(packet){const output=[],code=emitTypedProgramLinks(packet.network,packet.root,'JavaScript').code;vm.runInNewContext(code,{console:{log:(...values)=>output.push(values.join(' '))}},{timeout:1000});return {code,output:output.join('\n')};}
// Instrument only the test observer after running the complete original target.
// This checks retained call execution/order without attributing effects to Unit.
function observeCalls(packet){const code=observe(packet).code,program=decodeTypedProgramLinks(packet.network,packet.root),context=vm.createContext({console:{log(){}}});vm.runInContext(code,context,{timeout:1000});const actual=[];for(const name of program.declarations.keys()){
 assert.equal(typeof context[name],'function');context[name]=()=>{actual.push(name);return null;};}vm.runInContext('main();',context,{timeout:1000});return actual;}

test('selected actual implicit and explicit Unit helpers retain complete headers, body sites, declaration identities and ordered calls',()=>{
 assert.deepEqual(rustPureUnitCallFixture(),fixture);
 for(const entry of fixture.cases){
  const packet=reload(entry),program=decodeTypedProgramLinks(packet.network,packet.root);
  assert.deepEqual(program.sourcePureUnitCalls,{model:fixture.model,language:'Rust'});assert.deepEqual([...program.declarations.keys()],entry.names);assert.deepEqual(program.main.effects.map(effect=>effect.value.fn),entry.calls);
  for(const name of entry.names){const fn=program.declarations.get(name),proof=fn.sourcePureUnit;assert.deepEqual(fn.params,[]);assert.deepEqual(fn.ret,{kind:'unit'});assert.deepEqual(fn.body,{k:'unit',type:{kind:'unit'}});assert.equal(proof.headerSpan.start,fn.span.start);assert.equal(proof.bodySpan.end,fn.span.end);assert.match(entry.source.slice(proof.headerSpan.start,proof.headerSpan.end),new RegExp('^fn\\s+'+name+'\\s*\\('));assert.match(entry.source.slice(proof.bodySpan.start,proof.bodySpan.end),/^\{/u);}
  assert.deepEqual(program.main.effects.map(effect=>entry.source.slice(effect.span.start,effect.span.end).replace(/\s+/gu,'')),entry.calls.map((name,index)=>program.main.effects[index].sourcePureUnitCall.pathForm==='absolute crate path'?'crate::'+name+'();':name+'();'));
  assert.equal(observe(packet).output,entry.output,entry.id);assert.deepEqual(observeCalls(packet),entry.calls,entry.id+' actual target call execution');
  assert.equal(recheckLinkedTypedProgram(program,{wholeProgram:true}).sourcePureUnitCalls.model,fixture.model);
  assert.ok(!packet.network.toLino().includes(encodeURIComponent(entry.source)),entry.id+' contains no saved source payload');
 }
});

test('source-free normal declaration reference edits and ordered-container edits change the actual called identities and order',()=>{
 const entry=fixture.cases.find(entry=>entry.id==='ordered-repeated'),packet=reload(entry),network=packet.network;
 const refs=network.fieldRelations().filter(row=>row.label==='declaration reference');assert.equal(refs.length,3);
 const beta=refs[1].child;const changed=refs[0].link.references();changed[2]=beta;refs[0].link.setReferences(changed);
 const programId=network.link(packet.root).references()[0],main=field(network,programId,'main').child,effects=field(network,main,'effects').child;
 const ordered=network.link(effects).references();network.link(effects).setReferences([ordered[2],ordered[0],ordered[1]]);
 const raw={network:LinkNetwork.fromLino(network.toLino()),root:packet.root};
 assert.deepEqual(decodeTypedProgramLinks(raw.network,raw.root).main.effects.map(effect=>effect.value.fn),['alpha','beta','beta']);assert.deepEqual(observeCalls(raw),['alpha','beta','beta']);
 const rust=emitTypedProgramLinks(raw.network,raw.root,'Rust').code;assert.equal((rust.match(/= crate::beta\(\)/gu)??[]).length,2);assert.equal(observe(raw).output,'');
});

test('coherent source-free binding rename updates the real declaration and all actual call references without frozen name copies',()=>{
 const packet=reload(fixture.cases.find(entry=>entry.id==='ordered-repeated')),program=decodeTypedProgramLinks(packet.network,packet.root),fn=program.declarations.get('alpha');
 fn.name='renamed';fn.fullName='renamed';program.declarations.delete('alpha');program.declarations.set('renamed',fn);for(const effect of program.main.effects)if(effect.value.fn==='alpha')effect.value.fn='renamed';
 const encoded=encodeTypedProgramLinks(program),raw={network:LinkNetwork.fromLino(encoded.network.toLino()),root:encoded.root};assert.deepEqual(observeCalls(raw),['renamed','beta','renamed']);assert.ok(!decodeTypedProgramLinks(raw.network,raw.root).declarations.has('alpha'));
 assert.match(emitTypedProgramLinks(raw.network,raw.root,'Rust').code,/pub fn renamed\(\) -> \(\)/u);
});

test('38 complete source refusals preserve exact diagnostics and spans, including std ownership, effects, arguments and runtime wrappers',()=>{
 assert.equal(fixture.invalid.length,38);for(const invalid of fixture.invalid)assert.throws(()=>createTypedProgramLinks(invalid.source,'Rust',options),error=>error.kind===invalid.kind&&error.reason===invalid.reason&&assert.deepEqual(error.span??null,invalid.span)===undefined,invalid.id);
 assert.throws(()=>createTypedProgramLinks(fixture.cases[0].source,'Rust',{rustPureUnitCallModel:'unknown'}),/Rust pure Unit-call model/u);
 assert.throws(()=>createTypedProgramLinks(fixture.cases[0].source,'Rust',{...options,rustSourceSignatureModel:'Rust source function signature 1'}),/independent source profile/u);
 assert.throws(()=>createTypedProgramLinks('function empty(){}','JavaScript',options),/requires Rust source/u);
});

test('default Unit-helper and main-call refusals, full ml_main and std wrapper diagnostics stay unchanged',()=>{
 assert.throws(()=>parseRust('fn noop(){}fn main(){}'),error=>error.reason==='function without a result: noop returns (); only value-returning functions are portable'&&assert.deepEqual(error.span,{start:0,end:9})===undefined);
 assert.throws(()=>parseRust('fn noop()->(){()}fn main(){noop();}'),error=>error.reason==='main statement: main may only print, bind and assert');
 const full=emitRust(checkProgram(parseRust('fn main(){}'))).text;
 assert.throws(()=>parseRust(full),/function without a result: ml_main/u);assert.throws(()=>parseRust(full,{pureUnitCallModel:fixture.model}),/attributes require a separate compilation context proof/u);
 const wrapper=full.slice(full.lastIndexOf('fn main()'));
 assert.throws(()=>parseRust(wrapper),/std::env::args: the standard library is outside the portable core/u);assert.throws(()=>parseRust(wrapper,{pureUnitCallModel:fixture.model}));
});

test('malformed owned Unit facts, call targets, scopes, contexts and omitted facts cannot authorize target emission',()=>{
 const changes=[
  p=>{p.sourcePureUnitCalls=null;},p=>{p.sourcePureUnitCalls.model='unknown';},p=>{p.sourcePureUnitCalls.extra=false;},p=>{delete p.sourcePureUnitCalls;},
  p=>{p.declarations.get('alpha').sourcePureUnit=null;},p=>{delete p.declarations.get('alpha').sourcePureUnit;},p=>{p.declarations.get('alpha').sourcePureUnit.headerForm='borrowed unit';},p=>{p.declarations.get('alpha').sourcePureUnit.bodyForm='call';},p=>{p.declarations.get('alpha').sourcePureUnit.headerSpan.end=p.declarations.get('alpha').span.end;},
  p=>{p.declarations.get('alpha').params=[{name:'value',type:{kind:'unit'}}];},p=>{p.declarations.get('alpha').ret={kind:'unit',extra:false};},p=>{p.declarations.get('alpha').body={k:'unit',type:{kind:'unit'},extra:false};},p=>{p.declarations.get('alpha').generated=true;},p=>{p.declarations.get('alpha').fullName='other.alpha';},
  p=>{p.main.effects[0].value.fn='foreign';},p=>{p.main.effects[0].value.args=[{k:'unit',type:{kind:'unit'}}];},p=>{p.main.effects[0].sourcePureUnitCall=null;},p=>{delete p.main.effects[0].sourcePureUnitCall;},p=>{p.main.effects[0].sourcePureUnitCall.pathForm='standard-library alias';},p=>{p.main.effects[0].name='alpha';},p=>{p.main.effects[1].name=p.main.effects[0].name;},p=>{p.main.effects[0].value.span={...p.main.effects[0].value.span,end:p.main.effects[0].value.span.end-1};},p=>{p.main.effects[0].constant=false;},p=>{p.main.effects[0].value.type={kind:'bool'};},
  p=>{p.referenceSemantics=true;},p=>{p.imports=[{name:'foreign'}];},p=>{p.main.sequentialAsync=false;},p=>{p.main=null;},p=>{p.items.push(p.items[0]);},p=>{p.declarations.set('foreign',structuredClone(p.declarations.get('alpha')));},
 ];
 for(const [index,change] of changes.entries()){const packet=reload(fixture.cases.find(entry=>entry.id==='ordered-repeated')),program=decodeTypedProgramLinks(packet.network,packet.root);change(program);assert.throws(()=>encodeTypedProgramLinks(program),undefined,'malformed edit '+index);}
});

test('raw linked null and unknown facts remain visible before validation on both targets',()=>{
 const entry=fixture.cases.find(entry=>entry.id==='ordered-repeated');
 for(const [role,name,value] of [['program','sourcePureUnitCalls',null],['marker','language',null],['marker','unknown',false],['fn','sourcePureUnit',null],['proof','headerForm',null],['proof','bodySpan',null],['effect','sourcePureUnitCall',null],['statement','pathForm',null],['statement','unknown',false],['effect','name','alpha']]){
  const packet=reload(entry),network=packet.network,program=network.link(packet.root).references()[0],fn=network.fieldRelations().find(row=>row.label===label('sourcePureUnit')).parent,proof=field(network,fn,'sourcePureUnit').child,marker=field(network,program,'sourcePureUnitCalls').child,main=field(network,program,'main').child,effect=network.link(field(network,main,'effects').child).references()[0],statement=field(network,effect,'sourcePureUnitCall').child;
  replaceField(network,{program,marker,fn,proof,effect,statement}[role],name,scalar(network,value));const raw=LinkNetwork.fromLino(network.toLino());assert.throws(()=>decodeTypedProgramLinks(raw,packet.root),undefined,role+' '+name);for(const target of ['Rust','JavaScript'])assert.throws(()=>emitTypedProgramLinks(raw,packet.root,target));
 }
});

test('selected model provenance and automatically translated policy boundaries stay inspectable and strict',()=>{
 const packet=createTypedProgramLinks(fixture.cases[0].source,'Rust',{...options,provenance:{request:'independent'}}),network=LinkNetwork.fromLino(packet.network.toLino()),provenance=field(network,packet.root,'provenance').child,profile=field(network,provenance,'sourceProfile').child,user=field(network,provenance,'userProvenance').child;
 assert.equal(network.link(field(network,profile,'rustPureUnitCallModel').child).metadata().definition,fixture.model);assert.equal(network.link(field(network,user,'request').child).metadata().definition,'independent');
 assert.equal(acceptRustPureUnitCallModel(''),true);assert.equal(acceptRustPureUnitCallModel(fixture.model),true);assert.equal(acceptRustPureUnitCallModel('unknown'),false);
 for(let code=0;code<128;code++)for(const first of [false,true])assert.equal(acceptRustPureUnitNameCharacter(code,first),code===95||code>=65&&code<=90||code>=97&&code<=122||!first&&code>=48&&code<=57);
 for(const [accept,values] of [[acceptRustPureUnitHeaderForm,['implicit unit','explicit unit']],[acceptRustPureUnitBodyForm,['empty block','unit expression']],[acceptRustPureUnitPathForm,['unqualified name','absolute crate path']]]){for(const value of values)assert.equal(accept(value),true);assert.equal(accept('unknown'),false);}
 for(const kind of ['array','map','unknown'])for(const dense of [false,true])for(const unrepresented of [false,true])assert.equal(acceptRustPureUnitContainer(kind,dense,unrepresented),!unrepresented&&(kind==='array'&&dense||kind==='map'));
});

test('all selected ordered containers and the actual declaration Map refuse own metadata before the codec can erase it',()=>{
 assert.equal(fixture.containerMutations.length,52);
 for(const mutation of fixture.containerMutations){
  const packet=createTypedProgramLinks(fixture.mutationSource,'Rust',options),network=LinkNetwork.fromLino(packet.network.toLino()),program=decodeTypedProgramLinks(network,packet.root);
  applyPureUnitContainerMutation(program,mutation);
  assert.throws(()=>encodeTypedProgramLinks(program),error=>error.kind===mutation.kind&&error.reason===mutation.reason,mutation.id);
 }
 const packet=createTypedProgramLinks(fixture.mutationSource,'Rust',options),program=decodeTypedProgramLinks(packet.network,packet.root);
 program.imports=[];program.externals=[];
 const clean=encodeTypedProgramLinks(program),raw=LinkNetwork.fromLino(clean.network.toLino());
 assert.equal(observe({network:raw,root:clean.root}).output,'');assert.deepEqual(decodeTypedProgramLinks(raw,clean.root).main.effects.map(effect=>effect.value.fn),['empty']);
});

test('hidden record fields and callback-shaped ordered fields cannot be normalized as an absent ownership fact',()=>{
 for(const role of ['marker','function','proof','statement']){
  const packet=createTypedProgramLinks(fixture.mutationSource,'Rust',options),program=decodeTypedProgramLinks(packet.network,packet.root),fn=program.declarations.get('empty'),owner={marker:program.sourcePureUnitCalls,function:fn,proof:fn.sourcePureUnit,statement:program.main.effects[0].sourcePureUnitCall}[role];
  Object.defineProperty(owner,'borrowEscape',{value:true,enumerable:false});assert.throws(()=>encodeTypedProgramLinks(program),/unrepresented own fields/u,role);
 }
 const packet=createTypedProgramLinks(fixture.mutationSource,'Rust',options),program=decodeTypedProgramLinks(packet.network,packet.root);
 let executed=0;Object.defineProperty(program.main.effects,'0',{get(){executed++;throw Error('unrepresented callback was invoked');},enumerable:true});
 assert.throws(()=>encodeTypedProgramLinks(program),/unrepresented own fields or sparse positions/u);assert.equal(executed,0);
});
