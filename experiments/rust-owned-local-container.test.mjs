import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';
import {readProjectData} from '../js/scripts/project-data.mjs';
import {rustOwnedLocalContainerFixture,applyRustOwnedLocalContainerMutation,readRustOwnedLocalContainerOwner} from './generate-rust-owned-local-container-fixture.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks,encodeTypedProgramLinks,emitTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA} from '../js/src/typed-program-links.js';
import {LinkNetwork} from '../js/src/network.js';
import {LinkMetadata} from '../js/src/primitives.js';
import {recheckLinkedTypedProgram} from '../js/src/typed-program-checked-projection.js';
import {rustOwnedLocalStatePortableRules} from './generate-rust-owned-local-state-rules.mjs';
import {parseJavaScript} from '../js/src/translation/javascript.js';
import {parseRust} from '../js/src/translation/rust.js';
import {checkProgram} from '../js/src/translation/check.js';
import {emitRust} from '../js/src/translation/emit-rust.js';
import {emitJavaScript} from '../js/src/translation/emit-javascript.js';
import {checkRustOwnedLocalStateRecordShapes} from '../js/src/translation/rust-owned-local-state.js';
import {acceptRustOwnedLocalContainer,acceptRustOwnedLocalRecordValue,acceptRustOwnedLocalSourceSpan} from '../js/src/translation/rust-owned-local-state-rules.js';

const sources=readProjectData(new URL('./rust-owned-local-state-fixture.lino',import.meta.url));
const fixture=readProjectData(new URL('./rust-owned-local-container-fixture.lino',import.meta.url));
const loaded=(entry,selection)=>decodeTypedProgramLinks(LinkNetwork.fromLino(selection==='moves'?entry.links:entry.replayLinks),selection==='moves'?entry.root:entry.replayRoot);
const archived=program=>{const packet=encodeTypedProgramLinks(program);return{network:LinkNetwork.fromLino(packet.network.toLino()),root:packet.root};};
const label=field=>TYPED_PROGRAM_LINKS_SCHEMA.fields[field]??field.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
function scalar(network,value){const kind=value===null?'null':typeof value;return network.insertLink([],LinkMetadata.new().withLanguage(TYPED_PROGRAM_LINKS_SCHEMA.language).withNamed(true).withTerm(TYPED_PROGRAM_LINKS_SCHEMA.scalars[kind]).withDefinition(kind==='null'?undefined:String(value)));}
function field(network,parent,name){return network.fieldRelations().find(row=>row.parent.equals(parent)&&row.label===label(name));}
function replace(network,parent,name,value){const child=scalar(network,value),existing=field(network,parent,name);if(existing){const refs=existing.link.references();refs[2]=child;existing.link.setReferences(refs);}else network.insertFieldRelation(parent,label(name),child);}

test('separate linked mutation data and generated boundary decisions retain the declared truth tables',()=>{
 assert.deepEqual(rustOwnedLocalContainerFixture(),fixture);
 for(const entry of fixture.containerDecisions)assert.equal(acceptRustOwnedLocalContainer(entry.kind,entry.dense,entry.unrepresented),entry.expected);
 for(const entry of fixture.recordDecisions)assert.equal(acceptRustOwnedLocalRecordValue(entry.booleanRequired,entry.booleanValue),entry.expected);
 for(const entry of fixture.spanDecisions)assert.equal(acceptRustOwnedLocalSourceSpan(entry.present,entry.recordShape,entry.offsetShape),entry.expected);
});

test('both selected models refuse every actual host descriptor mutation without invoking getters',()=>{
 for(const selection of['moves','copies'])for(const mutation of fixture.containerMutations){
  if(!mutation.selections.includes(selection))continue;
  const program=loaded(sources.cases[0],selection);let invoked=0;
  applyRustOwnedLocalContainerMutation(program,mutation,()=>{invoked++;return false;});
  assert.throws(()=>encodeTypedProgramLinks(program),undefined,selection+' '+mutation.id);
  assert.equal(invoked,0,selection+' '+mutation.id+' never invokes a host accessor');
 }
});

test('known owned field and array-index getters refuse before the first value access',()=>{
 for(const selection of['moves','copies'])for(const mutation of fixture.fieldGetters){
  const program=loaded(sources.cases[0],selection),owner=readRustOwnedLocalContainerOwner(program,mutation.owner),value=owner[mutation.field];let invoked=0;
  Object.defineProperty(owner,mutation.field,{get(){invoked++;return value;},enumerable:true,configurable:true});
  assert.throws(()=>encodeTypedProgramLinks(program),undefined,selection+' '+mutation.owner+' '+mutation.field);
  assert.equal(invoked,0);
 }
});

test('contextual record edits refuse through the selected whole-program boundary',()=>{
 for(const selection of['moves','copies'])for(const mutation of fixture.recordMutations){
  const program=loaded(sources.cases[0],selection),owner=readRustOwnedLocalContainerOwner(program,mutation.owner);owner[mutation.field]=structuredClone(mutation.value);
  assert.throws(()=>encodeTypedProgramLinks(program),undefined,selection+' '+mutation.id);
  assert.throws(()=>recheckLinkedTypedProgram(program),undefined,selection+' direct recheck '+mutation.id);
 }
});

test('actual malformed contextual linked scalars refuse after LiNo reload for both targets',()=>{
 for(const selection of['moves','copies'])for(const [ownerName,name,value]of[['main','span',false],['main','sequentialAsync',null],['program','imports',null],['program','opaqueBinding',false],['main','opaqueBinding',false],['function','generated',null],['body','span',false]]){
  const entry=sources.cases[0],network=LinkNetwork.fromLino(selection==='moves'?entry.links:entry.replayLinks),root=selection==='moves'?entry.root:entry.replayRoot,programId=network.link(root).references()[0];
  const main=field(network,programId,'main').child,fn=network.links().find(link=>link.metadata().term===TYPED_PROGRAM_LINKS_SCHEMA.containers.record&&field(network,link.id(),'sourceLocalState'))?.id();
  assert.ok(fn);const body=field(network,fn,'body').child;
  replace(network,{program:programId,main,function:fn,body}[ownerName],name,value);
  const raw=LinkNetwork.fromLino(network.toLino());
  assert.throws(()=>decodeTypedProgramLinks(raw,root));
  for(const target of['Rust','JavaScript'])assert.throws(()=>emitTypedProgramLinks(raw,root,target));
 }
});

test('five original programs and actual emitted canonical copies keep their complete fixture bytes and value observations',()=>{
 for(const entry of sources.cases)for(const [selection,source,model,key,links,root]of[['moves',entry.source,sources.models.moves,'javascript','links','root'],['copies',entry.replaySource,sources.models.copies,'replayJavaScript','replayLinks','replayRoot']]){
  const packet=createTypedProgramLinks(source,'Rust',{rustOwnedLocalStateModel:model}),network=LinkNetwork.fromLino(packet.network.toLino());
  const canonical=encodeTypedProgramLinks(decodeTypedProgramLinks(network,packet.root));
  assert.equal(canonical.network.toLino(),entry[links]);assert.equal(canonical.root.asU64(),entry[root]);
  const emitted=emitTypedProgramLinks(network,packet.root,'JavaScript');assert.equal(emitted.code,entry[key]);
  const context={};vm.runInNewContext(emitted.code,context,{timeout:1000});for(const[text,count,flag,expected]of entry.observations)assert.equal(context[entry.name](text,BigInt(count),flag),expected);
  const program=decodeTypedProgramLinks(network,packet.root);program.main.span=undefined;
  const absent=archived(program);assert.equal(emitTypedProgramLinks(absent.network,absent.root,'JavaScript').code,entry[key]);
 }
});

test('actual automatic emitter definitions replay every boundary decision while native const syntax retains its parser gap',()=>{
 const source=readFileSync(new URL('../rust/src/translation/rust/owned_local_state_rules.rs',import.meta.url),'utf8');
 const emitted=emitRust(checkProgram(parseJavaScript(rustOwnedLocalStatePortableRules())));
 for(const [name,alias,rows,args]of[
  ['container','accept_rust_owned_local_container',fixture.containerDecisions,['kind','dense','unrepresented']],
  ['record_value','accept_rust_owned_local_record_value',fixture.recordDecisions,['booleanRequired','booleanValue']],
  ['source_span','accept_rust_owned_local_source_span',fixture.spanDecisions,['present','recordShape','offsetShape']],
 ]){
  const match=source.match(new RegExp('pub (?:const )?fn accept_rust_owned_local_'+name+'\\([^]*?\\n\\}', 'u'));assert.ok(match,name);
  const definition=emitted.definitions.find(value=>value.startsWith('pub fn '+alias+'('));assert.ok(definition,alias+' actual complete definition');
  const program=checkProgram(parseRust(definition+'\nfn main(){}')),runtime={};vm.runInNewContext(emitJavaScript(program).text,runtime,{timeout:1000});
  for(const row of rows)assert.equal(runtime[alias](...args.map(key=>row[key])),row.expected,name+' '+JSON.stringify(row));
  if(match[0].startsWith('pub const fn'))assert.throws(()=>parseRust(match[0]+'\nfn main(){}'),/expected : in const/u);
  else{
   const styled={};vm.runInNewContext(emitJavaScript(checkProgram(parseRust(match[0]+'\nfn main(){}'))).text,styled,{timeout:1000});
   for(const row of rows)assert.equal(styled[alias](...args.map(key=>row[key])),row.expected);
  }
 }
});

test('the new selected checks leave ordinary data, main and expression defaults unchanged',()=>{
 const packet=createTypedProgramLinks('enum Color{Red}fn keep(value:Color)->Color{value}fn main(){}','Rust');
 for(const [owner,fieldName,value]of[['data','opaqueBinding',false],['main','opaqueBinding',false],['expression','borrowEscape',false],['program','imports',null],['main','span',false]]){
  const program=decodeTypedProgramLinks(packet.network,packet.root),target={data:program.declarations.get('Color'),main:program.main,expression:program.declarations.get('keep').body,program}[owner];target[fieldName]=value;
  assert.equal(checkRustOwnedLocalStateRecordShapes(program),false);
  const archive=archived(program);for(const language of['Rust','JavaScript'])assert.ok(emitTypedProgramLinks(archive.network,archive.root,language).code);
 }
});
